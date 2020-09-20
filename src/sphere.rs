use crate::{hittable::*, material::*, ray::*, vec3::*};

use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = f64::sqrt(discriminant);

            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.at(t);
                let outward_normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(
                    &r,
                    outward_normal,
                    p,
                    t,
                    self.mat_ptr.clone(),
                ));
            }

            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.at(t);
                let outward_normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(
                    &r,
                    outward_normal,
                    p,
                    t,
                    self.mat_ptr.clone(),
                ));
            }
        }

        None
    }
}
