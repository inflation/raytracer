use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

pub use std::f32::consts::PI;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// From unstable std::arch
#[inline]
const fn mm_shuffle(z: u32, y: u32, x: u32, w: u32) -> i32 {
    ((z << 6) | (y << 4) | (x << 2) | w) as i32
}

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    e: __m128,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

// Initialize
impl Vec3 {
    #[inline]
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self {
            e: unsafe { _mm_set_ps(0.0, e2, e1, e0) },
        }
    }
    #[inline]
    pub fn from_scalar(e: f32) -> Self {
        Self {
            e: unsafe { _mm_set1_ps(e) },
        }
    }
    #[inline]
    pub fn from_array(e: [f32; 3]) -> Self {
        Self::new(e[0], e[1], e[2])
    }
}

// Attribute
impl Vec3 {
    #[inline]
    pub fn x(&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.e, self.e, mm_shuffle(0, 0, 0, 0))) }
    }
    #[inline]
    pub fn y(&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.e, self.e, mm_shuffle(1, 1, 1, 1))) }
    }
    #[inline]
    pub fn z(&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.e, self.e, mm_shuffle(2, 2, 2, 2))) }
    }

    pub fn to_array(&self) -> [f32; 3] {
        let mut array = [0.0; 4];
        unsafe { _mm_storeu_ps(array.as_mut_ptr(), self.e) }
        [array[0], array[1], array[2]]
    }
}

// Random
impl Vec3 {
    #[inline]
    pub fn random(rng: &mut impl rand::Rng) -> Self {
        Self::new(rng.gen(), rng.gen(), rng.gen())
    }
    #[inline]
    pub fn random_with_bound(rng: &mut impl rand::Rng, min: f32, max: f32) -> Self {
        Self::new(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }
}

// Constants
impl Vec3 {
    pub fn origin() -> Self {
        Self::from_scalar(0.0)
    }
    pub fn black() -> Self {
        Self::origin()
    }
}

// Methods
impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Self::Output {
        Self {
            e: unsafe { _mm_xor_ps(self.e, _mm_set1_ps(-0.0)) },
        }
    }
}
impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: unsafe { _mm_add_ps(self.e, rhs.e) },
        }
    }
}
impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.e = unsafe { _mm_add_ps(self.e, rhs.e) };
    }
}
impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: unsafe { _mm_sub_ps(self.e, rhs.e) },
        }
    }
}
impl Mul for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: unsafe { _mm_mul_ps(self.e, rhs.e) },
        }
    }
}
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        unsafe {
            let scalar = _mm_set1_ps(rhs);
            Self {
                e: _mm_mul_ps(self.e, scalar),
            }
        }
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        unsafe {
            let scalar = _mm_set1_ps(rhs);
            self.e = _mm_mul_ps(self.e, scalar);
        }
    }
}
impl Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        (1.0 / rhs) * self
    }
}
impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}
impl Div<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn div(self, rhs: Vec3) -> Self::Output {
        unsafe {
            let scalar = _mm_set1_ps(self);
            Vec3 {
                e: _mm_div_ps(scalar, rhs.e),
            }
        }
    }
}
impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { _mm_movemask_ps(_mm_cmpeq_ps(self.e, other.e)) == 0xF }
    }
}
impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unsafe {
            match _mm_movemask_ps(_mm_cmplt_ps(self.e, other.e)) & 0b0111 {
                0b0111 => Some(std::cmp::Ordering::Less),
                _ => None,
            }
            .or_else(|| {
                match _mm_movemask_ps(_mm_cmpgt_ps(self.e, other.e)) & 0b0111 {
                    0b0111 => Some(std::cmp::Ordering::Greater),
                    _ => None,
                }
            })
        }
    }
}

// Other methods
impl Vec3 {
    #[inline]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    #[inline]
    pub fn length_squared(&self) -> f32 {
        self.dot(*self)
    }
    #[inline]
    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    #[inline]
    pub fn dot(&self, v: Vec3) -> f32 {
        unsafe {
            let mut sum = _mm_mul_ps(self.e, v.e);
            sum = _mm_hadd_ps(sum, sum); // [x + y, z + w, x + z, y + w]
            sum = _mm_hadd_ps(sum, sum); // [x + y + z + w; 4]
            _mm_cvtss_f32(sum)
        }
    }

    #[inline]
    pub fn cross(&self, v: Vec3) -> Vec3 {
        unsafe {
            let tmp0 = _mm_shuffle_ps(v.e, v.e, mm_shuffle(3, 0, 2, 1));
            let mut tmp1 = _mm_shuffle_ps(self.e, self.e, mm_shuffle(3, 0, 2, 1));
            tmp1 = _mm_mul_ps(tmp1, v.e);
            let tmp2 = _mm_fmsub_ps(tmp0, self.e, tmp1);

            Vec3 {
                e: _mm_shuffle_ps(tmp2, tmp2, mm_shuffle(3, 0, 2, 1)),
            }
        }
    }

    #[inline]
    pub fn min(&self, v: Vec3) -> Vec3 {
        Self {
            e: unsafe { _mm_min_ps(self.e, v.e) },
        }
    }
    #[inline]
    pub fn max(&self, v: Vec3) -> Vec3 {
        Self {
            e: unsafe { _mm_max_ps(self.e, v.e) },
        }
    }
    #[inline]
    pub fn select_lt(&self, v: Vec3, mask: Vec3) -> Vec3 {
        Self {
            e: unsafe { _mm_blendv_ps(self.e, v.e, _mm_cmplt_ps(mask.e, Vec3::origin().e)) },
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x(), self.y(), self.z())
    }
}

pub fn random_in_unit_sphere<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec3 {
    loop {
        let p = Vec3::new(
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
            rng.gen_range(-1.0, 1.0),
        );
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_unit_disk<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec3 {
    loop {
        let p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_cosine_direction<R: rand::Rng + ?Sized>(rng: &mut R) -> Vec3 {
    let r1: f32 = rng.gen();
    let r2: f32 = rng.gen();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}

pub fn random_to_sphere<R: rand::Rng + ?Sized>(
    rng: &mut R,
    radius: f32,
    distance_squared: f32,
) -> Vec3 {
    let r1: f32 = rng.gen();
    let r2: f32 = rng.gen();
    let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    Vec3::new(x, y, z)
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = -uv.dot(n);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attr() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a.x(), 1.0);
        assert_eq!(a.y(), 2.0);
        assert_eq!(a.z(), 3.0);
        assert_eq!(a.to_array(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn neg() {
        let a = (-Vec3::from_scalar(1.0)).to_array();
        let b = Vec3::from_scalar(-1.0).to_array();
        assert_eq!(a, b);
    }

    #[test]
    fn add() {
        let a = Vec3::from_array([1.0, 1.0, 1.0]);
        let mut b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!((a + b).to_array(), [2.0, 3.0, 4.0]);

        b += a;
        assert_eq!(b.to_array(), [2.0, 3.0, 4.0]);
    }

    #[test]
    fn sub() {
        let a = Vec3::from_scalar(2.0);
        let b = Vec3::from_scalar(1.0);
        assert_eq!((a - b).to_array(), [1.0; 3]);
    }

    #[test]
    fn mul() {
        let mut a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!((a * a).to_array(), [1.0, 4.0, 9.0]);
        assert_eq!((9.0 * a).to_array(), [9.0, 18.0, 27.0]);
        assert_eq!((a * 9.0).to_array(), [9.0, 18.0, 27.0]);

        a *= 9.0;
        assert_eq!(a.to_array(), [9.0, 18.0, 27.0]);
    }

    #[test]
    fn div() {
        let mut a = Vec3::new(2.0, 4.0, 8.0);
        assert_eq!((a / 2.0).to_array(), [1.0, 2.0, 4.0]);
        assert_eq!((2.0 / a).to_array(), [1.0, 0.5, 0.25]);

        a /= 2.0;
        assert_eq!(a.to_array(), [1.0, 2.0, 4.0]);
    }

    #[test]
    fn compare() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0 * a;
        let c = Vec3::new(2.0, 0.0, 5.0);
        assert!(a == a);
        assert!(a < b);
        assert!(b > a);
        assert_eq!(a.partial_cmp(&c), None);
    }

    #[test]
    fn dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a.dot(a), 14.0);
    }

    #[test]
    fn cross() {
        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        assert_eq!(a.cross(b).to_array(), [-3.0, 6.0, -3.0]);
    }

    #[test]
    fn min_max() {
        let a = Vec3::new(3.0, 1.0, 2.0);
        let b = Vec3::new(1.0, 3.0, 0.0);

        assert_eq!(a.min(b).to_array(), [1.0, 1.0, 0.0]);
        assert_eq!(a.max(b).to_array(), [3.0, 3.0, 2.0]);
        assert_eq!(
            a.select_lt(b, Vec3::new(-1.0, 1.0, -1.0)).to_array(),
            [1.0, 1.0, 0.0]
        );
    }
}
