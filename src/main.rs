use prelude::*;

use camera::*;
use color::*;
use hittable_list::*;
use pdf::*;
use worlds::*;

use std::sync::Arc;

use indicatif::ParallelProgressIterator;
use rand::Rng;
use rayon::prelude::*;

#[macro_use]
mod util;

mod aabb;
mod aarect;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod cuboid;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod pdf;
mod perlin;
mod prelude;
mod ray;
mod sphere;
mod texture;
mod vec3;
mod worlds;

#[global_allocator]
static GLOBAL: mimallocator::Mimalloc = mimallocator::Mimalloc;

fn ray_color(
    r: &Ray,
    background: Color,
    world: &HittableList,
    lights: Arc<dyn Hittable>,
    depth: u32,
) -> Color {
    if depth == 0 {
        return Color::BLACK;
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let emitted = rec.mat_ptr.emitted(&rec);
        if let Some(ScatterRecord {
            specular_ray,
            attenuation,
            pdf_ptr,
        }) = rec.mat_ptr.scatter(r, &rec)
        {
            if let Some(specular) = specular_ray {
                return attenuation * ray_color(&specular, background, world, lights, depth - 1);
            }
            let light_pdf = HittablePDF::new(lights.clone(), rec.p);
            let p = MixturePDF::new(light_pdf, pdf_ptr.unwrap());

            let scattered = Ray::new(rec.p, p.generate(), r.time());
            let pdf_val = p.value(scattered.direction());

            emitted
                + attenuation
                    * rec.mat_ptr.scattering_pdf(r, &rec, &scattered)
                    * ray_color(&scattered, background, world, lights, depth - 1)
                    / pdf_val
        } else {
            emitted
        }
    } else {
        background
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_height: u32 = 300;
    const MAX_DEPTH: u32 = 50;
    let mut samples_per_pixel = 100;

    // World
    let scene = 6;
    let world;
    let mut lights: Arc<dyn Hittable> = Arc::new(HittableList::new());

    // Camera
    let mut look_from = Point3::new(13.0, 2.0, 3.0);
    let mut look_at = Point3::ORIGIN;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let mut aperture = 0.0;
    let mut vfov = 20.0;
    let mut background = Color::new(0.70, 0.80, 1.00);

    match scene {
        1 => {
            world = random_scene();
            aperture = 0.1;
        }
        2 => {
            world = two_spheres();
        }
        3 => {
            world = two_perlin_spheres();
        }
        4 => {
            world = earth();
        }
        5 => {
            let (w, l) = simple_light();
            world = w;
            lights = l;

            background = Color::BLACK;
            look_from = point!(26.0, 3.0, 6.0);
            look_at = point!(0.0, 2.0, 0.0);
            samples_per_pixel = 400;
        }
        6 => {
            let (w, l) = cornell_box();
            world = w;
            lights = l;

            aspect_ratio = 1.0;
            image_height = 600;
            samples_per_pixel = 1000;
            background = Color::BLACK;
            look_from = point!(278.0, 278.0, -800.0);
            look_at = point!(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            let (w, l) = cornell_smoke();
            world = w;
            lights = l;

            aspect_ratio = 1.0;
            image_height = 600;
            samples_per_pixel = 1000;
            background = Color::BLACK;
            look_from = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            let (w, l) = final_scene();
            world = w;
            lights = l;

            aspect_ratio = 1.0;
            image_height = 800;
            // samples_per_pixel = 10_000;
            samples_per_pixel = 10;
            background = Color::BLACK;
            look_from = Point3::new(478.0, 278.0, -600.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    };

    let image_width = (image_height as f64 * aspect_ratio) as u32;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        0.0,
        1.0,
    );

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    let result: Vec<String> = (0..image_height)
        .into_par_iter()
        .rev()
        .progress_count(image_height.into())
        .map(|j| {
            (0..image_width)
                .into_par_iter()
                .map(|i| {
                    let mut pixel_color = Color::BLACK;
                    let mut rng = rand::thread_rng();

                    for _ in 0..samples_per_pixel {
                        let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                        let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                        let r = cam.get_ray(u, v);
                        pixel_color += ray_color(&r, background, &world, lights.clone(), MAX_DEPTH);
                    }

                    let mut buffer = String::new();
                    write_color(&mut buffer, pixel_color, samples_per_pixel).unwrap();
                    buffer
                })
                .collect()
        })
        .collect();
    print!("{}", result.join(""));

    Ok(())
}
