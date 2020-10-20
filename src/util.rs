use std::ops::Index;

use crate::vec3::*;

macro_rules! point {
    ($x:literal, $y:literal, $z:literal) => {
        Vec3::new($x, $y, $z)
    };
}
macro_rules! rgb {
    ($x:literal, $y:literal, $z:literal) => {
        Vec3::new($x, $y, $z)
    };
}
macro_rules! vec3 {
    ($x:literal, $y:literal, $z:literal) => {
        Vec3::new($x, $y, $z)
    };
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    x.max(min).min(max)
}

#[derive(Debug)]
pub struct ONB {
    axis: [Vec3; 3],
}

impl ONB {
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn local(&self, a: Vec3) -> Vec3 {
        a.x() * self.u() + a.y() * self.v() + a.z() * self.w()
    }

    pub fn from_w(n: Vec3) -> Self {
        let w = unit_vector(n);
        let a = if w.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = unit_vector(cross(w, a));
        let u = cross(w, v);

        Self { axis: [u, v, w] }
    }
}

impl Index<usize> for ONB {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.axis[index]
    }
}
