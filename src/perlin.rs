use crate::vec3::*;

use itertools::Itertools;
use rand::Rng;
use std::mem::MaybeUninit;
#[derive(Debug)]
pub struct Perlin {
    ran_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let ran_vec = rand::thread_rng()
            .sample_iter(rand::distributions::Uniform::new(-1.0, 1.0))
            .tuples()
            .map(|(e0, e1, e2)| Vec3::new(e0, e1, e2))
            .take(Self::POINT_COUNT)
            .collect();

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ran_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let c = {
            let mut c: [[[MaybeUninit<Vec3>; 2]; 2]; 2] =
                unsafe { MaybeUninit::uninit().assume_init() };

            for di in 0..2 {
                for dj in 0..2 {
                    for dk in 0..2 {
                        c[di][dj][dk] = MaybeUninit::new(
                            self.ran_vec[self.perm_x[((i + di as i32) & 255) as usize]
                                ^ self.perm_y[((j + dj as i32) & 255) as usize]
                                ^ self.perm_z[((k + dk as i32) & 255) as usize]],
                        );
                    }
                }
            }

            unsafe { std::mem::transmute::<_, [[[Vec3; 2]; 2]; 2]>(c) }
        };

        Self::perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: Point3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

impl Perlin {
    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = Vec::with_capacity(Self::POINT_COUNT);
        for i in 0..Self::POINT_COUNT {
            p.push(i);
        }
        Self::permute(&mut p);

        p
    }

    fn permute(p: &mut Vec<usize>) {
        let mut rng = rand::thread_rng();
        for i in (1..Self::POINT_COUNT).rev() {
            let target = rng.gen_range(0, i);
            p.swap(i, target);
        }
    }

    #[inline]
    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * dot(c[i][j][k], weight_v);
                }
            }
        }

        accum
    }
}
