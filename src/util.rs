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
