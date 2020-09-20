use crate::vec3::*;
pub type Color = Vec3;

pub fn write_color(f: &mut dyn std::io::Write, pixel_color: Color) -> Result<(), std::io::Error> {
    writeln!(
        f,
        "{} {} {}",
        (255.999 * pixel_color.x()) as u32,
        (255.999 * pixel_color.y()) as u32,
        (255.999 * pixel_color.z()) as u32
    )
}
