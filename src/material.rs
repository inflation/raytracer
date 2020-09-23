use crate::prelude::*;
use crate::texture::*;

use std::{fmt::Debug, sync::Arc};

pub trait Material: Sync + Send + Debug {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color, PDF)> {
        None
    }
    fn scatter2(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> PDF {
        0.0
    }
    fn emitted(&self, rec: &HitRecord) -> Color {
        Color::default()
    }
}

// Lambertian
#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}
impl IntoArc for Lambertian {}

impl Lambertian {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self::from_color(Color::new(r, g, b))
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor { color_value: color }),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color, PDF)> {
        let uvw = ONB::from_w(rec.normal);
        let direction = uvw.local_vec(random_cosine_direction());
        let scattered = Ray::new(rec.p, unit_vector(direction), r_in.time());
        let alb = self.albedo.value(rec.u, rec.v, rec.p);
        let pdf = dot(uvw.w(), scattered.direction()) / PI;

        Some((scattered, alb, pdf))
    }

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> PDF {
        let cos = dot(rec.normal, unit_vector(scattered.direction()));
        (cos / PI).max(0.0)
    }
}

// Metal
#[derive(Debug)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}
impl IntoArc for Metal {}

impl Metal {
    pub fn new(r: f64, g: f64, b: f64, fuzz: f64) -> Self {
        Self {
            albedo: Color::new(r, g, b),
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter2(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
            r_in.time(),
        );
        let attenuation = self.albedo;

        if dot(scattered.direction(), rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

// Dielectric
#[derive(Debug)]
pub struct Dielectric {
    pub ref_idx: f64,
}
impl IntoArc for Dielectric {}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter2(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = unit_vector(r_in.direction());

        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
            let scattered = Ray::new(rec.p, reflected, r_in.time());

            return Some((scattered, attenuation));
        }
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if rand::random::<f64>() < reflect_prob {
            let reflected = reflect(unit_direction, rec.normal);
            let scattered = Ray::new(rec.p, reflected, r_in.time());

            return Some((scattered, attenuation));
        }

        let refracted = refract(unit_direction, rec.normal, etai_over_etat);
        let scattered = Ray::new(rec.p, refracted, r_in.time());

        Some((scattered, attenuation))
    }
}

// Diffuse light
#[derive(Debug)]
pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}
impl IntoArc for DiffuseLight {}

impl DiffuseLight {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self::from_color(Color::new(r, g, b))
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor { color_value: color }),
        }
    }

    pub fn white(s: f64) -> Self {
        Self::new(s, s, s)
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, rec: &HitRecord) -> Color {
        if rec.front_face {
            self.emit.value(rec.u, rec.v, rec.p)
        } else {
            Color::default()
        }
    }
}

// Isotropic
#[derive(Debug)]
pub struct Isotropic {
    pub albedo: Arc<dyn Texture>,
}
impl IntoArc for Isotropic {}

impl Isotropic {
    pub fn from_color(color: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor { color_value: color }),
        }
    }
}

impl Material for Isotropic {
    fn scatter2(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scattered = Ray::new(rec.p, random_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);

        Some((scattered, attenuation))
    }
}
