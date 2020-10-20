use crate::prelude::*;

use std::{fmt::Debug, sync::Arc};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        r: &Ray,
        outward_normal: Vec3,
        p: Point3,
        t: f64,
        u: f64,
        v: f64,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
        let front_face = dot(r.direction(), outward_normal) < 0.0;
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
        self.front_face = dot(r.direction(), normal) < 0.0;
        self.normal = if self.front_face { normal } else { -normal };
    }
}

pub trait Hittable: Sync + Send + Debug {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;

    fn pdf_value(&self, _o: Point3, _v: Vec3) -> f64 {
        0.0
    }
    fn random(&self, _o: Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());
        if let Some(mut rec) = self.inner.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, rec.normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if let Some(output_box) = self.inner.bounding_box(t0, t1) {
            Some(AABB::new(
                output_box.min() + self.offset,
                output_box.max() + self.offset,
            ))
        } else {
            None
        }
    }
}
#[derive(Debug)]
pub struct RotateY {
    inner: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
}

impl RotateY {
    pub fn new(inner: Arc<dyn Hittable>, angle: f64) -> Arc<Self> {
        let radian = angle.to_radians();
        let sin_theta = radian.sin();
        let cos_theta = radian.cos();
        let bbox = inner.bounding_box(0.0, 1.0).map(|bbox| {
            let mut min = Vec3 {
                e: [f64::INFINITY; 3],
            };
            let mut max = Vec3 {
                e: [f64::NEG_INFINITY; 3],
            };

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                        let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                        let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(newx, y, newz);
                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }

            AABB::new(min, max)
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        // Ray inversely rotated
        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_r = Ray::new(origin, direction, r.time());

        if let Some(mut rec) = self.inner.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.p = p;
            rec.set_face_normal(&rotated_r, normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        self.bbox
    }
}

// Flip face
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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.inner.hit(r, t_min, t_max).map(|rec| HitRecord {
            front_face: !rec.front_face,
            ..rec
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.inner.bounding_box(t0, t1)
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        self.inner.pdf_value(o, v)
    }
}
