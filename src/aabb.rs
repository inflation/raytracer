use crate::vec3::*;

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
            min: Point3::default(),
            max: Point3::default(),
        }
    }
}

impl AABB {
    #[inline]
    pub fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.min[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin()[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            let tmin = t0.max(t_min);
            let tmax = t1.min(t_max);

            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let small = Vec3::new(
        box0.min().x().min(box1.min().x()),
        box0.min().y().min(box1.min().y()),
        box0.min().z().min(box1.min().z()),
    );
    let big = Vec3::new(
        box0.max().x().max(box1.max().x()),
        box0.max().y().max(box1.max().y()),
        box0.max().z().max(box1.max().z()),
    );

    AABB::new(small, big)
}
