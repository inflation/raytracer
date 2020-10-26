pub use crate::{aabb::*, hittable::*, material::*, ray::*, texture::*, util::*, vec3::*};

#[derive(Copy, Clone, Debug)]
pub enum Plane {
    Xy,
    Xz,
    Yz,
}
