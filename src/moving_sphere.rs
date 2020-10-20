use crate::{aabb::*, hittable::*, material::Material, ray::Ray, sphere::Sphere, vec3::*};

use std::sync::Arc;
#[derive(Debug)]
pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat_ptr: Arc<dyn Material>,
    ) -> Arc<Self> {
        Arc::new(Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            mat_ptr,
        })
    }

    #[inline]
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let current_center = self.center(r.time());
        let oc = r.origin() - current_center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();

            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.at(t);
                let outward_normal = (p - current_center) / self.radius;
                let (u, v) = Sphere::get_uv((p - current_center) / self.radius);
                return Some(HitRecord::new(
                    &r,
                    outward_normal,
                    p,
                    t,
                    u,
                    v,
                    self.mat_ptr.clone(),
                ));
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.at(t);
                let outward_normal = (p - current_center) / self.radius;
                let (u, v) = Sphere::get_uv((p - current_center) / self.radius);
                return Some(HitRecord::new(
                    &r,
                    outward_normal,
                    p,
                    t,
                    u,
                    v,
                    self.mat_ptr.clone(),
                ));
            }
        }

        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let radius = self.radius;
        let radius_vec = Vec3::new(radius, radius, radius);
        let center0 = self.center(t0);
        let center1 = self.center(t1);

        let box0 = AABB::new(center0 - radius_vec, center1 + radius_vec);
        let box1 = AABB::new(center1 - radius_vec, center1 + radius_vec);

        Some(surrounding_box(box0, box1))
    }
}
