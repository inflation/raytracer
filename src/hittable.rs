use crate::{material::*, ray::*, vec3::*};

use std::sync::Arc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        r: &Ray,
        outward_normal: Vec3,
        p: Point3,
        t: f64,
        mat_ptr: Arc<dyn Material>,
    ) -> Self {
        let front_face = dot(&r.direction(), &outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            t,
            front_face,
            mat_ptr,
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
