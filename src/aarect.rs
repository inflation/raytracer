use crate::{aabb::*, hittable::*, material::*, ray::*, vec3::*};

use std::sync::Arc;

#[derive(Debug)]
pub struct XYRect {
    mat_ptr: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            mat_ptr,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t0 || t > t1 {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let p = r.at(t);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);

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

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

#[derive(Debug)]
pub struct XZRect {
    mat_ptr: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            mat_ptr,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XZRect {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t0 || t > t1 {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let p = r.at(t);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);

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

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}
#[derive(Debug)]
pub struct YZRect {
    mat_ptr: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat_ptr: Arc<dyn Material>) -> Self {
        Self {
            mat_ptr,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for YZRect {
    fn hit(&self, r: &Ray, t0: f64, t1: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t0 || t > t1 {
            return None;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let p = r.at(t);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);

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

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(AABB::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
