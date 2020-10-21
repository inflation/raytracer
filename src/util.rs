use std::ops::Index;

use crate::prelude::*;

macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {
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
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
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
        let w = n.unit_vector();
        let a = if w.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = w.cross(a).unit_vector();
        let u = w.cross(v);

        Self { axis: [u, v, w] }
    }
}

impl Index<usize> for ONB {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.axis[index]
    }
}
