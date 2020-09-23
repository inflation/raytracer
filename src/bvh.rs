use rand::Rng;

use crate::hittable_list::*;
use crate::prelude::*;

use std::sync::Arc;

#[derive(Debug)]
pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(
        objects: &mut Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let axis = rand::thread_rng().gen_range(0, 2);
        let object_span = end - start;
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        let bbox;

        match object_span {
            0 => panic!("No Hittable provided"),
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => match Self::compare(axis, &objects[start], &objects[start + 1]) {
                std::cmp::Ordering::Less => {
                    left = objects[start].clone();
                    right = objects[start + 1].clone();
                }
                _ => {
                    left = objects[start + 1].clone();
                    right = objects[start].clone();
                }
            },
            _ => {
                objects[start..end].sort_unstable_by(|x, y| Self::compare(axis, x, y));
                let mid = start + object_span / 2;
                left = Arc::new(BVHNode::new(objects, start, mid, time0, time1));
                right = Arc::new(BVHNode::new(objects, mid, end, time0, time1));
            }
        }

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);
        box_left
            .and(box_right)
            .expect("No bounding box in BVHNode constructor");

        bbox = surrounding_box(box_left.unwrap(), box_right.unwrap());

        Self { left, right, bbox }
    }

    pub fn new_with_list(list: &mut HittableList, time0: f64, time1: f64) -> Self {
        let length = list.objects.len();
        Self::new(&mut list.objects, 0, length, time0, time1)
    }

    fn compare(axis: usize, a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        let box_a = a.bounding_box(0.0, 0.0);
        let box_b = b.bounding_box(0.0, 0.0);
        box_a
            .and(box_b)
            .expect("No bounding box in BVHNode constructor");

        box_a.unwrap().min().e[axis]
            .partial_cmp(&box_b.unwrap().min().e[axis])
            .unwrap()
    }
}

impl Hittable for BVHNode {
    // TODO: Double check for correctness
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min, t_max) {
            None
        } else {
            if let Some(rec_l) = self.left.hit(r, t_min, t_max) {
                if let Some(rec_r) = self.right.hit(r, t_min, rec_l.t) {
                    Some(rec_r)
                } else {
                    Some(rec_l)
                }
            } else {
                self.right.hit(r, t_min, t_max)
            }
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}
