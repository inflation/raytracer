use crate::prelude::*;

pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f32,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, tm: f32) -> Self {
        Self { orig, dir, tm }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn time(&self) -> f32 {
        self.tm
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}
