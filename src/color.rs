use crate::util::*;
use crate::vec3::*;

pub fn write_color(
    f: &mut dyn std::fmt::Write,
    mut pixel_color: Color,
    samples_per_pixel: u32,
) -> Result<(), std::fmt::Error> {
    let scale = 1.0 / samples_per_pixel as f32;
    pixel_color *= scale;
    let (mut r, mut g, mut b) = (pixel_color.x(), pixel_color.y(), pixel_color.z());

    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }

    // Gamma-correct for gamma = 2.0
    r = r.sqrt();
    g = g.sqrt();
    b = b.sqrt();

    // Truncate at [0,255]
    writeln!(
        f,
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as u32,
        (256.0 * clamp(g, 0.0, 0.999)) as u32,
        (256.0 * clamp(b, 0.0, 0.999)) as u32,
    )
}
