use crate::prelude::*;

use crate::{aarect::*, hittable_list::*};

use std::sync::Arc;
#[derive(Debug)]
pub struct Cuboid {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl Cuboid {
    pub fn new(p0: Point3, p1: Point3, ptr: Arc<dyn Material>) -> Arc<Self> {
        let mut sides = HittableList::new();

        sides.add(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            ptr.clone(),
        ));
        sides.add(XYRect::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            ptr.clone(),
        ));

        sides.add(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            ptr.clone(),
        ));
        sides.add(XZRect::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            ptr.clone(),
        ));

        sides.add(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            ptr.clone(),
        ));
        sides.add(YZRect::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            ptr.clone(),
        ));

        Arc::new(Self {
            box_min: p0,
            box_max: p1,
            sides,
        })
    }
}

impl Hittable for Cuboid {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
}
