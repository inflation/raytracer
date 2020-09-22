use crate::{hittable::*, ray::*, texture::*, vec3::*};

use std::sync::Arc;

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::default()
    }
}

// Lambertian
pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_direction = rec.normal + random_unit_vector();
        let scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, rec.p);

        Some((scattered, attenuation))
    }
}

// Metal
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(r: f64, g: f64, b: f64, fuzz: f64) -> Self {
        Self {
            albedo: Color::new(r, g, b),
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
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
pub struct Dielectric {
    pub ref_idx: f64,
}

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
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

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self::from_color(Color::new(r, g, b))
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor { color_value: color }),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
