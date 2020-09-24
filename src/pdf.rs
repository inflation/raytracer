use crate::prelude::*;

use std::{fmt::Debug, sync::Arc};

pub trait PDF: Debug {
    fn value(&self, direction: Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

#[derive(Debug)]

pub struct CosinePDF {
    uvw: ONB,
}

impl IntoArc for CosinePDF {}

impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: ONB::from_w(w),
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vec3) -> f64 {
        let cos = dot(unit_vector(direction), self.uvw.w());
        (cos / PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local(random_cosine_direction())
    }
}

#[derive(Debug)]
pub struct HittablePDF {
    o: Point3,
    ptr: Arc<dyn Hittable>,
}

impl HittablePDF {
    pub fn new(ptr: Arc<dyn Hittable>, o: Point3) -> Self {
        Self { o, ptr }
    }
}
impl IntoArc for HittablePDF {}

impl PDF for HittablePDF {
    fn value(&self, direction: Vec3) -> f64 {
        self.ptr.pdf_value(self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(self.o)
    }
}

#[derive(Debug)]
pub struct MixturePDF {
    p: [Arc<dyn PDF>; 2],
}

impl MixturePDF {
    pub fn new(p1: Arc<dyn PDF>, p2: Arc<dyn PDF>) -> Self {
        Self { p: [p1, p2] }
    }
}
impl IntoArc for MixturePDF {}

impl PDF for MixturePDF {
    fn value(&self, direction: Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
    fn generate(&self) -> Vec3 {
        if rand::random::<f32>() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
