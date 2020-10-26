use crate::prelude::*;

use crate::{aabb::*, hittable::*, material::*, ray::*};

use rand::Rng;
use std::sync::Arc;

#[derive(Debug)]
pub struct AARect {
    mat_ptr: Arc<dyn Material>,
    a0: f32,
    a1: f32,
    b0: f32,
    b1: f32,
    k: f32,
    axis: Plane,
}

impl AARect {
    pub fn new(
        p0: Point3,
        p1: Point3,
        axis: Plane,
        k: f32,
        mat_ptr: Arc<dyn Material>,
    ) -> Arc<Self> {
        let (a0, a1, b0, b1) = (p0.x(), p1.x(), p0.y(), p1.y());
        Arc::new(Self {
            mat_ptr,
            a0,
            a1,
            b0,
            b1,
            k,
            axis,
        })
    }

    pub fn from_corner(
        p0: Point3,
        p1: Point3,
        axis: Plane,
        k: f32,
        mat_ptr: Arc<dyn Material>,
    ) -> Arc<Self> {
        let (a0, a1, b0, b1) = match axis {
            Plane::Xy => (p0.x(), p1.x(), p0.y(), p1.y()),
            Plane::Xz => (p0.x(), p1.x(), p0.z(), p1.z()),
            Plane::Yz => (p0.y(), p1.y(), p0.z(), p1.z()),
        };
        Arc::new(Self {
            mat_ptr,
            a0,
            a1,
            b0,
            b1,
            k,
            axis,
        })
    }
}

impl Hittable for AARect {
    fn hit(&self, r: &Ray, t0: f32, t1: f32) -> Option<HitRecord> {
        let tt = (Vec3::from_scalar(self.k) - r.origin()) / r.direction();
        let t = match self.axis {
            Plane::Xy => tt.z(),
            Plane::Xz => tt.y(),
            Plane::Yz => tt.x(),
        };

        if t < t0 || t > t1 {
            return None;
        }

        let rr = r.origin() + t * r.direction();
        let (a, b) = match self.axis {
            Plane::Xy => (rr.x(), rr.y()),
            Plane::Xz => (rr.x(), rr.z()),
            Plane::Yz => (rr.y(), rr.z()),
        };
        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            return None;
        }

        let u = (a - self.a0) / (self.a1 - self.a0);
        let v = (b - self.b0) / (self.b1 - self.b0);
        let p = r.at(t);
        let outward_normal = match self.axis {
            Plane::Xy => vec3!(0.0, 0.0, 1.0),
            Plane::Xz => vec3!(0.0, 1.0, 0.0),
            Plane::Yz => vec3!(1.0, 0.0, 0.0),
        };

        Some(HitRecord::new(
            r,
            outward_normal,
            p,
            t,
            u,
            v,
            self.mat_ptr.clone(),
        ))
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(
            point!(self.k - 0.0001, self.a0, self.b0),
            point!(self.k + 0.0001, self.a1, self.b1),
        ))
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f32 {
        if let Some(rec) = self.hit(&Ray::new(o, v, 0.0), 0.001, f32::INFINITY) {
            let area = (self.a1 - self.a0) * (self.b1 - self.b0);
            let distance_squared = rec.t * rec.t * v.length_squared();
            let cos = (v.dot(rec.normal) / v.length()).abs();

            distance_squared / (cos * area)
        } else {
            0.0
        }
    }

    fn random(&self, rng: &mut dyn rand::RngCore, o: Vec3) -> Vec3 {
        let random_point = point!(
            rng.gen_range(self.a0, self.a1),
            self.k,
            rng.gen_range(self.b0, self.b1)
        );
        random_point - o
    }
}
