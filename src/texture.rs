use crate::vec3::*;
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
