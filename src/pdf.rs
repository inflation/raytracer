use crate::prelude::*;

use crate::{hittable_list::*, onb::*};

use std::fmt::Debug;

pub trait PDF: Debug {
    fn value(&self, direction: Vec3) -> f32;
    fn generate(&self, rng: &mut dyn rand::RngCore) -> Vec3;
}

#[derive(Debug)]

pub struct CosinePDF {
    uvw: OrthonormalBasis,
}

impl CosinePDF {
    pub fn new(w: Vec3) -> Box<Self> {
        Box::new(Self {
            uvw: OrthonormalBasis::from_w(w),
        })
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vec3) -> f32 {
        let cos = direction.unit_vector().dot(self.uvw.w());
        (cos / PI).max(0.0)
    }

    fn generate(&self, rng: &mut dyn rand::RngCore) -> Vec3 {
        self.uvw.local(random_cosine_direction(rng))
    }
}

#[derive(Debug)]
pub struct HittablePDF<'a> {
    o: Point3,
    ptr: &'a HittableList,
}

impl<'a> HittablePDF<'a> {
    pub fn new(ptr: &'a HittableList, o: Point3) -> Box<Self> {
        Box::new(Self { o, ptr })
    }
}

impl PDF for HittablePDF<'_> {
    fn value(&self, direction: Vec3) -> f32 {
        self.ptr.pdf_value(self.o, direction)
    }
    fn generate(&self, rng: &mut dyn rand::RngCore) -> Vec3 {
        self.ptr.random(rng, self.o)
    }
}

#[derive(Debug)]
pub struct MixturePDF<'a> {
    p: [Box<dyn PDF + 'a>; 2],
}

impl<'a> MixturePDF<'a> {
    pub fn new(p1: Box<dyn PDF + 'a>, p2: Box<dyn PDF + 'a>) -> Self {
        Self { p: [p1, p2] }
    }
}

impl PDF for MixturePDF<'_> {
    fn value(&self, direction: Vec3) -> f32 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
    fn generate(&self, rng: &mut dyn rand::RngCore) -> Vec3 {
        if rand::random::<f32>() < 0.5 {
            self.p[0].generate(rng)
        } else {
            self.p[1].generate(rng)
        }
    }
}
