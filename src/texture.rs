use crate::{util::Perlin, vec3::*};

use std::sync::Arc;
pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub struct SolidColor {
    pub color_value: Color,
}

impl SolidColor {
    // pub fn new(r: f64, g: f64, b: f64) -> Self {
    //     Self {
    //         color_value: Color::new(r, g, b),
    //     }
    // }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn from_color(c1: Color, c2: Color) -> Self {
        Self {
            even: Arc::new(SolidColor { color_value: c1 }),
            odd: Arc::new(SolidColor { color_value: c2 }),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(self.scale * p, 7)).sin())
    }
}
