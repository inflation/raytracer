use prelude::*;

use camera::*;
use color::*;
use pdf::*;
use worlds::*;

use indicatif::ParallelProgressIterator;
use rand::Rng;
use rayon::prelude::*;
use std::sync::Arc;

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
    rng: &mut impl Rng,
    r: &Ray,
    background: Color,
    world: Arc<World>,
    depth: u32,
) -> Color {
    if depth == 0 {
        return Color::black();
    }

    if let Some(rec) = world.world().hit(r, 0.001, f32::INFINITY) {
        let emitted = rec.mat_ptr.emitted(&rec);
        if let Some(ScatterRecord {
            specular_ray,
            attenuation,
            pdf_ptr,
        }) = rec.mat_ptr.scatter(rng, r, &rec)
        {
            if let Some(specular) = specular_ray {
                return attenuation * ray_color(rng, &specular, background, world, depth - 1);
            }
            // let light_pdf = HittablePDF::new(world.lights(), rec.p);
            // let p = MixturePDF::new(light_pdf, pdf_ptr.unwrap());
            let p = pdf_ptr.unwrap();

            let scattered = Ray::new(rec.p, p.generate(rng), r.time());
            let pdf_val = p.value(scattered.direction());
            // let pdf_val = 0.1;

            emitted
                + attenuation
                    * rec.mat_ptr.scattering_pdf(r, &rec, &scattered)
                    * ray_color(rng, &scattered, background, world.clone(), depth - 1)
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
    let scene = std::env::var("SCENE")
        .ok()
        .and_then(|x| x.parse::<i32>().ok())
        .unwrap_or(2);
    let world;

    // Camera
    let mut look_from = point!(13.0, 2.0, 3.0);
    let mut look_at = Point3::origin();

    let vup = vec3!(0.0, 1.0, 0.0);
    let focus_dist = 10.0;
    let mut aperture = 0.0;
    let mut vfov = 20.0;
    let mut background = rgb!(0.70, 0.80, 1.00);

    match scene {
        1 => {
            world = World::random_scene();

            aperture = 0.1;
        }
        2 => {
            world = World::two_spheres();
        }
        3 => {
            world = World::two_perlin_spheres();
        }
        4 => {
            world = World::earth();
        }
        5 => {
            world = World::simple_light();

            background = Color::black();
            look_from = point!(26.0, 3.0, 6.0);
            look_at = point!(0.0, 2.0, 0.0);
            // samples_per_pixel = 400;
        }
        6 => {
            world = World::cornell_box();

            aspect_ratio = 1.0;
            image_height = 600;
            samples_per_pixel = 10;
            background = Color::black();
            look_from = point!(278.0, 278.0, -800.0);
            look_at = point!(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            world = World::cornell_smoke();

            aspect_ratio = 1.0;
            image_height = 600;
            // samples_per_pixel = 1000;
            background = Color::black();
            look_from = point!(278.0, 278.0, -800.0);
            look_at = point!(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            world = World::final_scene();

            aspect_ratio = 1.0;
            image_height = 800;
            // samples_per_pixel = 10_000;
            samples_per_pixel = 10;
            background = Color::black();
            look_from = point!(478.0, 278.0, -600.0);
            look_at = point!(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
    };

    let world = Arc::new(world);

    let image_width = (image_height as f32 * aspect_ratio) as u32;

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
                    let mut pixel_color = Color::black();
                    let mut rng = rand::thread_rng();

                    for _ in 0..samples_per_pixel {
                        let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                        let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;

                        let r = cam.get_ray(&mut rng, u, v);
                        pixel_color +=
                            ray_color(&mut rng, &r, background, world.clone(), MAX_DEPTH);
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
