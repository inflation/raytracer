use crate::prelude::*;

use std::ops::Index;

#[derive(Debug)]
pub struct OrthonormalBasis {
    axis: [Vec3; 3],
}

impl OrthonormalBasis {
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
        let any_vec = if w.x().abs() > 0.9 {
            vec3!(0.0, 1.0, 0.0)
        } else {
            vec3!(1.0, 0.0, 0.0)
        };
        let v = w.cross(any_vec).unit_vector();
        let u = w.cross(v);

        Self { axis: [u, v, w] }
    }
}

impl Index<usize> for OrthonormalBasis {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        &self.axis[index]
    }
}
