pub use crate::hittable::*;

use std::sync::Arc;

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut final_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                final_rec = Some(rec);
            }
        }

        final_rec
    }
}
