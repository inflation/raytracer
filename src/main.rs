mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod util;
mod vec3;

use camera::*;
use color::*;
use hittable_list::*;
use ray::*;
use sphere::*;
use vec3::*;

use std::io::Write;
use std::io::{stderr, stdout};

fn ray_color(r: &Ray, world: &impl Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let target = rec.p + rec.normal + random_unit_vecotr();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 20;
    const MAX_DEPTH: u32 = 50;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::default();

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush()?;

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(&mut stdout(), pixel_color, SAMPLES_PER_PIXEL)?;
        }
    }

    eprintln!("\nDone.");

    Ok(())
}
