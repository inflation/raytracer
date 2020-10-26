use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct AABB {
    min: Point3,
    max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> Point3 {
        self.min
    }
    pub fn max(&self) -> Point3 {
        self.max
    }
}

impl std::default::Default for AABB {
    fn default() -> Self {
        Self {
            min: Point3::origin(),
            max: Point3::origin(),
        }
    }
}

impl AABB {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let inv_d = 1.0 / r.direction();
        let t0 = (self.min - r.origin()) * inv_d;
        let t1 = (self.max - r.origin()) * inv_d;
        let tt0 = t0.select_lt_0(t1, inv_d);
        let tt1 = t1.select_lt_0(t0, inv_d);

        let tmin = tt0.max(Vec3::from_scalar(t_min));
        let tmax = tt1.min(Vec3::from_scalar(t_max));

        tmax > tmin
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = box0.min().min(box1.min());
    let big = box0.max().max(box1.max());

    AABB::new(small, big)
}
