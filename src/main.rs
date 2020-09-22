mod aabb;
// mod bvh;
mod aarect;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod ray;
mod sphere;
mod texture;
mod util;
mod vec3;

use aarect::*;
use camera::*;
use color::*;
use hittable::*;
use hittable_list::*;
use material::*;
use moving_sphere::*;
use ray::*;
use sphere::*;
use texture::*;
use vec3::*;

use std::sync::Arc;

use indicatif::ParallelProgressIterator;
use rand::Rng;
use rayon::prelude::*;

#[global_allocator]
static GLOBAL: mimallocator::Mimalloc = mimallocator::Mimalloc;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::from_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian { albedo: checker });
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(&mut rng) * Color::random(&mut rng);
                    sphere_material = Arc::new(Lambertian::from_color(albedo));
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_with_bound(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal { albedo, fuzz });
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)))
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        glass,
    )));

    let mat = Arc::new(Lambertian::new(0.4, 0.2, 0.1));
    world.add(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat)));

    let metal = Arc::new(Metal::new(0.7, 0.6, 0.5, 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        metal,
    )));

    world
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let checker = Arc::new(CheckerTexture::from_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian {
            albedo: checker.clone(),
        }),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian { albedo: checker }),
    )));

    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian {
            albedo: pertext.clone(),
        }),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian { albedo: pertext }),
    )));

    objects
}

fn earth() -> HittableList {
    let mut earth = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new("assets/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian {
        albedo: earth_texture,
    });
    let globe = Arc::new(Sphere::new(Point3::default(), 2.0, earth_surface));

    earth.add(globe);
    earth
}

fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian {
            albedo: pertext.clone(),
        }),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian { albedo: pertext }),
    )));

    let difflight = Arc::new(DiffuseLight::new(4.0, 4.0, 4.0));
    objects.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new(0.65, 0.05, 0.05));
    let white = Arc::new(Lambertian::new(0.73, 0.73, 0.73));
    let green = Arc::new(Lambertian::new(0.12, 0.45, 0.15));
    let light = Arc::new(DiffuseLight::new(15.0, 15.0, 15.0));

    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

    objects
}

fn ray_color(r: &Ray, background: &Color, world: &impl Hittable, depth: u32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        let emitted = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);
        if let Some((scattered, attenuation)) = rec.mat_ptr.scatter(r, &rec) {
            emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
        } else {
            emitted
        }
    } else {
        *background
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

    // Camera
    let mut look_from = Point3::new(13.0, 2.0, 3.0);
    let mut look_at = Point3::default();
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
            world = simple_light();
            background = Color::default();
            look_from = Point3::new(26.0, 3.0, 6.0);
            look_at = Point3::new(0.0, 2.0, 0.0);
            samples_per_pixel = 400;
        }
        _ => {
            world = cornell_box();
            aspect_ratio = 1.0;
            image_height = 600;
            samples_per_pixel = 200;
            background = Color::default();
            look_from = Point3::new(278.0, 278.0, -800.0);
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
                    let mut pixel_color = Color::default();
                    let mut rng = rand::thread_rng();

                    for _ in 0..samples_per_pixel {
                        let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                        let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;

                        let r = cam.get_ray(u, v);
                        pixel_color += ray_color(&r, &background, &world, MAX_DEPTH);
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
