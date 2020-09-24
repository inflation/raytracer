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
mod util;
mod vec3;

use prelude::*;

use aarect::*;
use bvh::*;
use camera::*;
use color::*;
use constant_medium::*;
use cuboid::*;
use hittable_list::*;
use moving_sphere::*;
use pdf::*;
use sphere::*;

use std::sync::Arc;

use indicatif::ParallelProgressIterator;
use rand::Rng;
use rayon::prelude::*;

#[global_allocator]
static GLOBAL: mimallocator::Mimalloc = mimallocator::Mimalloc;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker =
        CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)).into_arc();
    let ground_material = Lambertian { albedo: checker }.into_arc();
    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material).into_arc());

    let mut rng = rand::thread_rng();
    let mut balls = HittableList::new();

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
                    sphere_material = Lambertian::from_color(albedo).into_arc();
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0);
                    balls.add(
                        MovingSphere::new(center, center2, 0.0, 1.0, 0.2, sphere_material)
                            .into_arc(),
                    );
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_with_bound(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    sphere_material = Metal { albedo, fuzz }.into_arc();
                    balls.add(Sphere::new(center, 0.2, sphere_material).into_arc());
                } else {
                    // glass
                    sphere_material = Dielectric::new(1.5).into_arc();
                    balls.add(Sphere::new(center, 0.2, sphere_material).into_arc());
                }
            }
        }
    }
    world.add(BVHNode::new_with_list(balls, 0.0, 1.0).into_arc());

    let glass = Dielectric::new(1.5).into_arc();
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, glass).into_arc());

    let mat = Lambertian::new(0.4, 0.2, 0.1).into_arc();
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat).into_arc());

    let metal = Metal::new(0.7, 0.6, 0.5, 0.0).into_arc();
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        metal,
    )));

    world
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let checker =
        CheckerTexture::new(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)).into_arc();

    objects.add(
        Sphere::new(
            Point3::new(0.0, -10.0, 0.0),
            10.0,
            Lambertian {
                albedo: checker.clone(),
            }
            .into_arc(),
        )
        .into_arc(),
    );
    objects.add(
        Sphere::new(
            Point3::new(0.0, 10.0, 0.0),
            10.0,
            Lambertian { albedo: checker }.into_arc(),
        )
        .into_arc(),
    );

    objects
}

fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = NoiseTexture::new(4.0).into_arc();

    objects.add(
        Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian {
                albedo: pertext.clone(),
            }
            .into_arc(),
        )
        .into_arc(),
    );
    objects.add(
        Sphere::new(
            Point3::new(0.0, 2.0, 0.0),
            2.0,
            Lambertian { albedo: pertext }.into_arc(),
        )
        .into_arc(),
    );

    objects
}

fn earth() -> HittableList {
    let mut earth = HittableList::new();

    let earth_texture = ImageTexture::new("assets/earthmap.jpg").into_arc();
    let earth_surface = Lambertian {
        albedo: earth_texture,
    }
    .into_arc();
    let globe = Sphere::new(Point3::default(), 2.0, earth_surface).into_arc();

    earth.add(globe);
    earth
}

fn simple_light() -> (HittableList, Arc<dyn Hittable>) {
    let mut objects = HittableList::new();

    let pertext = NoiseTexture::new(4.0).into_arc();
    objects.add(
        Sphere::new(
            Point3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian {
                albedo: pertext.clone(),
            }
            .into_arc(),
        )
        .into_arc(),
    );
    objects.add(
        Sphere::new(
            Point3::new(0.0, 2.0, 0.0),
            2.0,
            Lambertian { albedo: pertext }.into_arc(),
        )
        .into_arc(),
    );

    let difflight = DiffuseLight::white(4.0).into_arc();
    let lights = XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight).into_arc();
    objects.add(lights.clone());

    (objects, lights)
}

fn cornell_box() -> (HittableList, Arc<dyn Hittable>) {
    let mut objects = HittableList::new();

    let red = Lambertian::new(0.65, 0.05, 0.05).into_arc();
    let white = Lambertian::new(0.73, 0.73, 0.73).into_arc();
    let green = Lambertian::new(0.12, 0.45, 0.15).into_arc();
    let light = DiffuseLight::white(15.0).into_arc();

    objects.add(
        FlipFace::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light.clone()).into_arc())
            .into_arc(),
    );

    objects.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green).into_arc());
    objects.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red).into_arc());
    objects.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()).into_arc());
    objects.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()).into_arc());
    objects.add(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()).into_arc());

    // Boxes
    let glass = Dielectric::new(1.5).into_arc();
    let mut box1: Arc<dyn Hittable> = Cuboid::new(
        Point3::default(),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    )
    .into_arc();
    // let aluminum = Metal::new(0.8, 0.85, 0.88, 0.0).into_arc();
    // let mut box1: Arc<dyn Hittable> = Cuboid::new(
    //     Point3::default(),
    //     Point3::new(165.0, 330.0, 165.0),
    //     aluminuium,
    // )
    // .into_arc();
    box1 = RotateY::new(box1, 15.0).into_arc();
    box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)).into_arc();
    objects.add(box1);

    // let mut box2: Arc<dyn Hittable> =
    //     Cuboid::new(Point3::default(), Point3 { e: [165.0; 3] }, white).into_arc();
    // box2 = RotateY::new(box2, -18.0).into_arc();
    // box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)).into_arc();
    // objects.add(box2);

    let sphere: Arc<dyn Hittable> =
        Sphere::new(Point3::new(190.0, 90.0, 190.0), 90.0, glass).into_arc();
    objects.add(sphere.clone());

    // (objects, upper_light)

    let mut lights = HittableList::new();
    let light_shape =
        XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, NoMaterial {}.into_arc()).into_arc();
    let sphere_shape: Arc<dyn Hittable> = Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        NoMaterial {}.into_arc(),
    )
    .into_arc();

    lights.add(light_shape);
    lights.add(sphere_shape);

    (objects, lights.into_arc())
}

fn cornell_smoke() -> (HittableList, Arc<dyn Hittable>) {
    let mut objects = HittableList::new();

    let red = Lambertian::new(0.65, 0.05, 0.05).into_arc();
    let white = Lambertian::new(0.73, 0.73, 0.73).into_arc();
    let green = Lambertian::new(0.12, 0.45, 0.15).into_arc();
    let light = DiffuseLight::white(7.0).into_arc();

    let lights = XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light.clone()).into_arc();
    objects.add(FlipFace::new(lights.clone()).into_arc());

    objects.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green).into_arc());
    objects.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red).into_arc());
    objects.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()).into_arc());
    objects.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()).into_arc());
    objects.add(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()).into_arc());

    // Boxes
    let mut box1: Arc<dyn Hittable> = Cuboid::new(
        Point3::default(),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    )
    .into_arc();
    box1 = RotateY::new(box1, 15.0).into_arc();
    box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)).into_arc();
    let mut box2: Arc<dyn Hittable> =
        Cuboid::new(Point3::default(), Point3 { e: [165.0; 3] }, white).into_arc();
    box2 = RotateY::new(box2, -18.0).into_arc();
    box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)).into_arc();

    objects.add(ConstantMedium::new(box1, 0.01, Color::default()).into_arc());
    objects.add(ConstantMedium::new(box2, 0.01, Color { e: [1.0; 3] }).into_arc());

    (objects, lights)
}

fn final_scene() -> (HittableList, Arc<dyn Hittable>) {
    let mut rng = rand::thread_rng();
    let mut objects = HittableList::new();
    let mut lights = HittableList::new();

    let mut boxes1 = HittableList::new();
    let ground = Lambertian::new(0.48, 0.83, 0.53).into_arc();

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

            boxes1.add(
                Cuboid::new(
                    Point3::new(x0, y0, z0),
                    Point3::new(x1, y1, z1),
                    ground.clone(),
                )
                .into_arc(),
            );
        }
    }
    objects.add(BVHNode::new_with_list(boxes1, 0.0, 1.0).into_arc());

    let light = DiffuseLight::new(7.0, 7.0, 7.0).into_arc();
    let upper_light = XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light.clone()).into_arc();
    objects.add(FlipFace::new(upper_light.clone()).into_arc());
    lights.add(upper_light);

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::new(0.7, 0.3, 0.1).into_arc();
    objects.add(
        MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, moving_sphere_material).into_arc(),
    );

    objects.add(
        Sphere::new(
            Point3::new(260.0, 150.0, 45.0),
            50.0,
            Dielectric::new(1.5).into_arc(),
        )
        .into_arc(),
    );
    objects.add(
        Sphere::new(
            Point3::new(0.0, 150.0, 145.0),
            50.0,
            Metal::new(0.8, 0.8, 0.9, 10.0).into_arc(),
        )
        .into_arc(),
    );

    let boundary = Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Dielectric::new(1.5).into_arc(),
    )
    .into_arc();
    objects.add(boundary.clone());
    lights.add(boundary.clone());
    objects.add(ConstantMedium::new(boundary, 0.2, Color::new(0.2, 0.4, 0.9)).into_arc());
    let boundary =
        Sphere::new(Point3::default(), 5000.0, Dielectric::new(1.5).into_arc()).into_arc();
    objects.add(ConstantMedium::new(boundary, 0.0001, Color::new(1.0, 1.0, 1.0)).into_arc());

    let emat = Lambertian {
        albedo: ImageTexture::new("assets/earthmap.jpg").into_arc(),
    }
    .into_arc();
    objects.add(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat).into_arc());
    let pertext = NoiseTexture::new(0.1).into_arc();
    objects.add(
        Sphere::new(
            Point3::new(200.0, 280.0, 300.0),
            80.0,
            Lambertian { albedo: pertext }.into_arc(),
        )
        .into_arc(),
    );

    let mut boxes2 = HittableList::new();
    let white = Lambertian::new(0.73, 0.73, 0.73).into_arc();
    for _ in 0..1000 {
        boxes2.add(
            Sphere::new(
                Point3::random_with_bound(&mut rng, 0.0, 165.0),
                10.0,
                white.clone(),
            )
            .into_arc(),
        );
    }
    objects.add(
        Translate::new(
            RotateY::new(BVHNode::new_with_list(boxes2, 0.0, 1.0).into_arc(), 15.0).into_arc(),
            Vec3::new(-100.0, 270.0, 395.0),
        )
        .into_arc(),
    );

    (objects, lights.into_arc())
}

fn ray_color(
    r: &Ray,
    background: Color,
    world: &HittableList,
    lights: Arc<dyn Hittable>,
    depth: u32,
) -> Color {
    if depth <= 0 {
        return Color::default();
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
            let p = MixturePDF::new(light_pdf.into_arc(), pdf_ptr.unwrap());

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
    let mut lights: Arc<dyn Hittable> = NoObject {}.into_arc();

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
            let (w, l) = simple_light();
            world = w;
            lights = l;

            background = Color::default();
            look_from = Point3::new(26.0, 3.0, 6.0);
            look_at = Point3::new(0.0, 2.0, 0.0);
            samples_per_pixel = 400;
        }
        6 => {
            let (w, l) = cornell_box();
            world = w;
            lights = l;

            aspect_ratio = 1.0;
            image_height = 600;
            samples_per_pixel = 1000;
            background = Color::default();
            look_from = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        7 => {
            let (w, l) = cornell_smoke();
            world = w;
            lights = l;

            aspect_ratio = 1.0;
            image_height = 600;
            samples_per_pixel = 1000;
            background = Color::default();
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
