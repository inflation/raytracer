use crate::prelude::*;

use std::sync::Arc;
#[derive(Debug)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Arc<dyn Material>) -> Arc<Self> {
        Arc::new(Self {
            center,
            radius,
            mat_ptr,
        })
    }

    pub fn get_uv(p: Vec3) -> (f64, f64) {
        let phi = p.z().atan2(p.x());
        let theta = p.y().asin();
        (1.0 - (phi + PI) / (2.0 * PI), (theta + PI / 2.0) / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
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
                let outward_normal = (p - self.center) / self.radius;
                let (u, v) = Self::get_uv((p - self.center) / self.radius);
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
                let outward_normal = (p - self.center) / self.radius;
                let (u, v) = Self::get_uv((p - self.center) / self.radius);
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

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let radius = self.radius;
        let radius_vec = Vec3::new(radius, radius, radius);
        Some(AABB::new(
            self.center - radius_vec,
            self.center + radius_vec,
        ))
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        if self
            .hit(&Ray::new(o, v, 0.0), 0.001, f64::INFINITY)
            .is_some()
        {
            let cos_theta_max =
                (1.0 - self.radius * self.radius / (self.center - o).length_squared()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let direction = self.center - o;
        let distance_squared = direction.length_squared();
        let uvw = ONB::from_w(direction);

        uvw.local(random_to_sphere(self.radius, distance_squared))
    }
}
