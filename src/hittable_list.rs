use crate::prelude::*;

use rand::Rng;
use std::sync::Arc;
#[derive(Debug)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let first = self.objects.first().and_then(|x| x.bounding_box(t0, t1))?;

        self.objects.iter().skip(1).fold(Some(first), |acc, x| {
            if let Some(bounding_box) = x.bounding_box(t0, t1) {
                Some(surrounding_box(acc.unwrap(), bounding_box))
            } else {
                None
            }
        })
    }

    fn pdf_value(&self, o: Point3, v: Vec3) -> f32 {
        let weight = 1.0 / self.objects.len() as f32;
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(o, v);
        }

        sum
    }

    fn random(&self, rng: &mut dyn rand::RngCore, o: Vec3) -> Vec3 {
        let size = self.objects.len();

        if size == 0 {
            return Point3::origin();
        }

        let index = rand::thread_rng().gen_range(0, size);
        self.objects[index].random(rng, o)
    }
}
