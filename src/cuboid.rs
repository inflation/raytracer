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

        sides.add(AARect::from_corner(p0, p1, Plane::Xy, p1.z(), ptr.clone()));
        sides.add(AARect::from_corner(p0, p1, Plane::Xy, p0.z(), ptr.clone()));

        sides.add(AARect::from_corner(p0, p1, Plane::Xz, p1.y(), ptr.clone()));
        sides.add(AARect::from_corner(p0, p1, Plane::Xz, p0.y(), ptr.clone()));

        sides.add(AARect::from_corner(p0, p1, Plane::Yz, p1.x(), ptr.clone()));
        sides.add(AARect::from_corner(p0, p1, Plane::Yz, p0.x(), ptr.clone()));

        Arc::new(Self {
            box_min: p0,
            box_max: p1,
            sides,
        })
    }
}

impl Hittable for Cuboid {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }

    fn pdf_value(&self, _o: Point3, _v: Vec3) -> f32 {
        0.0
    }
}
