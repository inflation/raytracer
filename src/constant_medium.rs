use crate::prelude::*;

use std::sync::Arc;
#[derive(Debug)]
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, d: f32, c: Color) -> Arc<Self> {
        Arc::new(Self {
            boundary,
            phase_function: Arc::new(Isotropic::from_color(c)),
            neg_inv_density: -1.0 / d,
        })
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(r, f32::NEG_INFINITY, f32::INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, f32::INFINITY) {
                rec1.t = rec1.t.max(t_min);
                rec2.t = rec2.t.min(t_max);

                if rec1.t >= rec2.t {
                    return None;
                }

                rec1.t = rec1.t.max(0.0);

                let ray_length = r.direction().length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rand::random::<f32>().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;
                let p = r.at(t);
                let normal = Vec3::new(1.0, 0.0, 0.0);
                return Some(HitRecord::new(
                    r,
                    normal,
                    p,
                    t,
                    0.0,
                    0.0,
                    self.phase_function.clone(),
                ));
            }
        }

        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }

    fn pdf_value(&self, _o: Point3, _v: Vec3) -> f32 {
        0.0
    }
}
