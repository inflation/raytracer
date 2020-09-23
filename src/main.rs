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
mod prelude;
mod ray;
mod sphere;
mod texture;
mod util;
mod vec3;

use aarect::*;
use bvh::*;
use camera::*;
use color::*;
use constant_medium::*;
use cuboid::*;
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
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    // Boxes
    let mut box1: Arc<dyn Hittable> = Arc::new(Cuboid::new(
        Point3::default(),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);
    let mut box2: Arc<dyn Hittable> = Arc::new(Cuboid::new(
        Point3::default(),
        Point3 { e: [165.0; 3] },
        white,
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2);

    objects
}

fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new(0.65, 0.05, 0.05));
    let white = Arc::new(Lambertian::new(0.73, 0.73, 0.73));
    let green = Arc::new(Lambertian::new(0.12, 0.45, 0.15));
    let light = Arc::new(DiffuseLight::new(7.0, 7.0, 7.0));

    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
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
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    // Boxes
    let mut box1: Arc<dyn Hittable> = Arc::new(Cuboid::new(
        Point3::default(),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    let mut box2: Arc<dyn Hittable> = Arc::new(Cuboid::new(
        Point3::default(),
        Point3 { e: [165.0; 3] },
        white,
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    objects.add(Arc::new(ConstantMedium::new(box1, 0.01, Color::default())));
    objects.add(Arc::new(ConstantMedium::new(
        box2,
        0.01,
        Color { e: [1.0; 3] },
    )));

    objects
}

fn final_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut objects = HittableList::new();

    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(0.48, 0.83, 0.53));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Cuboid::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    objects.add(Arc::new(BVHNode::new_with_list(&mut boxes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new(7.0, 7.0, 7.0));
    objects.add(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::new(0.7, 0.3, 0.1));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(0.8, 0.8, 0.9, 10.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new(
        Point3::default(),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::new(ConstantMedium::new(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian {
        albedo: Arc::new(ImageTexture::new("assets/earthmap.jpg")),
    });
    objects.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere::new(
        Point3::new(200.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian { albedo: pertext }),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(0.73, 0.73, 0.73));
    for _ in 0..1000 {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random_with_bound(&mut rng, 0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }
    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BVHNode::new_with_list(&mut boxes2, 0.0, 1.0)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

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
    let scene = 8;
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
        6 => {
            world = cornell_box();
            aspect_ratio = 1.0;
            image_height = 600;
            samples_per_pixel = 200;
            background = Color::default();
            look_from = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            image_height = 600;
            samples_per_pixel = 200;
            background = Color::default();
            look_from = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {
            world = final_scene();
            aspect_ratio = 1.0;
            image_height = 800;
            samples_per_pixel = 10_000;
            background = Color::default();
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
