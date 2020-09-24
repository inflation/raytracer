use crate::prelude::*;

use crate::pdf::*;

use std::{fmt::Debug, sync::Arc};

pub trait Material: Sync + Send + Debug {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
    fn emitted(&self, _rec: &HitRecord) -> Color {
        Color::default()
    }
}

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>,
    pub attenuation: Color,
    pub pdf_ptr: Option<Arc<dyn PDF>>,
}

#[derive(Debug)]
pub struct NoMaterial {}
impl IntoArc for NoMaterial {}
impl Material for NoMaterial {}

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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord {
            specular_ray: None,
            attenuation: self.albedo.value(rec.u, rec.v, rec.p),
            pdf_ptr: Some(CosinePDF::new(rec.normal).into_arc()),
        })
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        Some(ScatterRecord {
            specular_ray: Some(Ray::new(
                rec.p,
                reflected + self.fuzz * random_in_unit_sphere(),
                0.0,
            )),
            attenuation: self.albedo,
            pdf_ptr: None,
        })
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let ray_direction;
        let scattered;

        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = unit_vector(r_in.direction());

        let cos_theta = dot(-unit_direction, rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let reflect_prob = schlick(cos_theta, etai_over_etat);

        if etai_over_etat * sin_theta > 1.0 || rand::random::<f64>() < reflect_prob {
            ray_direction = reflect(unit_direction, rec.normal);
        } else {
            ray_direction = refract(unit_direction, rec.normal, etai_over_etat);
        }
        scattered = Ray::new(rec.p, ray_direction, r_in.time());

        Some(ScatterRecord {
            specular_ray: Some(scattered),
            attenuation,
            pdf_ptr: None,
        })
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scattered = Ray::new(rec.p, random_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);

        Some(ScatterRecord {
            specular_ray: Some(scattered),
            attenuation,
            pdf_ptr: None,
        })
    }
}
