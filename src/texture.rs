use crate::prelude::*;

use crate::perlin::*;

use image::{DynamicImage, GenericImageView, Pixel};
use std::{fmt::Debug, sync::Arc};

pub trait Texture: Sync + Send + Debug {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

#[derive(Debug)]
pub struct SolidColor {
    pub color_value: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        self.color_value
    }
}
impl IntoArc for SolidColor {}

#[derive(Debug)]
pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> Self {
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

impl IntoArc for CheckerTexture {}

#[derive(Debug)]
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
            * (1.0 + (self.scale * p.z() + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

impl IntoArc for NoiseTexture {}

pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn new(filename: impl AsRef<str>) -> Self {
        let filename = filename.as_ref();

        let image = image::open(filename)
            .expect(format!("ERROR: Could not load texture image file: {}", filename).as_str());

        Self { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        let (width, height) = self.image.dimensions();
        let u = u.max(0.0).min(1.0);
        let v = 1.0 - v.max(0.0).min(1.0);

        let mut i = (u * width as f64) as u32;
        let mut j = (v * height as f64) as u32;

        if i >= width {
            i = width - 1;
        }
        if j >= height {
            j = height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let pixel: Vec<f64> = self
            .image
            .get_pixel(i, j)
            .channels()
            .into_iter()
            .map(|&x| color_scale * x as f64)
            .collect();

        Color::new(pixel[0], pixel[1], pixel[2])
    }
}

impl Debug for ImageTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Image")
    }
}

impl IntoArc for ImageTexture {}
