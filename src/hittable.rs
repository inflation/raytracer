use crate::prelude::*;

use std::{fmt::Debug, sync::Arc};

use rand::distributions::Uniform;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        r: &Ray,
        outward_normal: Vec3,
        p: Point3,
        t: f32,
        u: f32,
        v: f32,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
        let front_face = r.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            t,
            u,
            v,
            front_face,
            mat_ptr,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, normal: Vec3) {
        self.front_face = r.direction().dot(normal) < 0.0;
        self.normal = if self.front_face { normal } else { -normal };
    }
}

pub trait Hittable: Sync + Send + Debug {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
    fn pdf_value(&self, o: Point3, v: Vec3) -> f32;

    fn random(&self, _rng: &mut dyn rand::RngCore, _dist: &Uniform<f32>, _o: Vec3) -> Vec3 {
        vec3!(1.0, 0.0, 0.0)
    }
}

#[derive(Debug)]
pub struct Translate {
    inner: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(inner: Arc<dyn Hittable>, offset: Vec3) -> Arc<Self> {
        Arc::new(Self { inner, offset })
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if let Some(mut rec) = self.inner.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, rec.normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        if let Some(output_box) = self.inner.bounding_box(t0, t1) {
            Some(AABB::new(
                output_box.min() + self.offset,
                output_box.max() + self.offset,
            ))
        } else {
            None
        }
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f32 {
        if self
            .hit(&Ray::new(o, v, 0.0), 0.001, f32::INFINITY)
            .is_some()
        {
            self.inner.pdf_value(o, v)
        } else {
            0.0
        }
    }
}

#[derive(Debug)]
pub struct RotateY {
    inner: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(inner: Arc<dyn Hittable>, angle: f32) -> Arc<Self> {
        let radian = angle.to_radians();
        let sin_theta = radian.sin();
        let cos_theta = radian.cos();
        let bbox = inner.bounding_box(0.0, 1.0).map(|bbox| {
            let mut min = [f32::INFINITY; 3];
            let mut max = [f32::NEG_INFINITY; 3];

            // TODO: SIMD
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f32 * bbox.max().x() + (1 - i) as f32 * bbox.min().x();
                        let y = j as f32 * bbox.max().y() + (1 - j) as f32 * bbox.min().y();
                        let z = k as f32 * bbox.max().z() + (1 - k) as f32 * bbox.min().z();

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = [newx, y, newz];
                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }

            AABB::new(Vec3::from_array(min), Vec3::from_array(max))
        });

        Arc::new(Self {
            inner,
            sin_theta,
            cos_theta,
            bbox,
        })
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = r.origin().to_array();
        let mut direction = r.direction().to_array();

        // Ray inversely rotated
        origin[0] = self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z();
        origin[2] = self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z();

        direction[0] = self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z();
        direction[2] = self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z();

        let rotated_r = Ray::new(
            Vec3::from_array(origin),
            Vec3::from_array(direction),
            r.time(),
        );

        if let Some(mut rec) = self.inner.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p.to_array();
            let mut normal = rec.normal.to_array();

            p[0] = self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z();
            p[2] = -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z();

            normal[0] = self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
            normal[2] = -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();

            rec.p = Vec3::from_array(p);
            rec.set_face_normal(&rotated_r, Vec3::from_array(normal));

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        self.bbox
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f32 {
        if self
            .hit(&Ray::new(o, v, 0.0), 0.001, f32::INFINITY)
            .is_some()
        {
            self.inner.pdf_value(o, v)
        } else {
            0.0
        }
    }
}

// Flip Face
#[derive(Debug)]
pub struct FlipFace {
    inner: Arc<dyn Hittable>,
}

impl FlipFace {
    pub fn new(inner: Arc<dyn Hittable>) -> Arc<Self> {
        Arc::new(Self { inner })
    }
}

impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.inner.hit(r, t_min, t_max).map(|rec| HitRecord {
            front_face: !rec.front_face,
            ..rec
        })
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.inner.bounding_box(t0, t1)
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f32 {
        self.inner.pdf_value(o, v)
    }

    fn random(&self, rng: &mut dyn rand::RngCore, dist: &Uniform<f32>, o: Vec3) -> Vec3 {
        self.inner.random(rng, dist, o)
    }
}
