use crate::prelude::*;

use crate::{
    aarect::*, bvh::*, constant_medium::*, cuboid::*, hittable_list::*, moving_sphere::*, sphere::*,
};

use rand::Rng;
use std::sync::Arc;

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = CheckerTexture::new(rgb!(0.2, 0.3, 0.1), rgb!(0.9, 0.9, 0.9));
    let ground_material = Lambertian::new(checker);
    world.add(Sphere::new(
        point!(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

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

            if (center - point!(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(&mut rng) * Color::random(&mut rng);
                    let sphere_material = Lambertian::from_color(albedo);
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0, 0.5), 0.0);
                    balls.add(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_with_bound(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    balls.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    balls.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }
    world.add(BVHNode::new_with_list(balls, 0.0, 1.0));

    let glass = Dielectric::new(1.5);
    world.add(Sphere::new(point!(0.0, 1.0, 0.0), 1.0, glass));

    let mat = Lambertian::new_rgb(0.4, 0.2, 0.1);
    world.add(Sphere::new(point!(-4.0, 1.0, 0.0), 1.0, mat));

    let metal = Metal::new_rgbf(0.7, 0.6, 0.5, 0.0);
    world.add(Sphere::new(point!(4.0, 1.0, 0.0), 1.0, metal));

    world
}

pub fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let checker = CheckerTexture::new(rgb!(0.2, 0.3, 0.1), rgb!(0.9, 0.9, 0.9));

    objects.add(Sphere::new(
        point!(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(checker.clone()),
    ));
    objects.add(Sphere::new(
        point!(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new(checker),
    ));

    objects
}

pub fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = NoiseTexture::new(4.0);

    objects.add(Sphere::new(
        point!(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(pertext.clone()),
    ));
    objects.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(pertext),
    ));

    objects
}

pub fn earth() -> HittableList {
    let mut earth = HittableList::new();

    let earth_texture = ImageTexture::new("assets/earthmap.jpg");
    let earth_surface = Lambertian::new(earth_texture);
    let globe = Sphere::new(Point3::ORIGIN, 2.0, earth_surface);

    earth.add(globe);
    earth
}

pub fn simple_light() -> (HittableList, Arc<dyn Hittable>) {
    let mut objects = HittableList::new();

    let pertext = NoiseTexture::new(4.0);
    objects.add(Sphere::new(
        point!(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(pertext.clone()),
    ));
    objects.add(Sphere::new(
        point!(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(pertext),
    ));

    let difflight = DiffuseLight::white(4.0);
    let lights = XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight);
    objects.add(lights.clone());

    (objects, lights)
}

pub fn cornell_box() -> (HittableList, Arc<dyn Hittable>) {
    let mut objects = HittableList::new();
    let mut lights = HittableList::new();

    let red = Lambertian::new_rgb(0.65, 0.05, 0.05);
    let white = Lambertian::new_rgb(0.73, 0.73, 0.73);
    let green = Lambertian::new_rgb(0.12, 0.45, 0.15);
    let light = DiffuseLight::white(15.0);

    let light = FlipFace::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light));

    lights.add(light.clone());
    objects.add(light);

    objects.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green));
    objects.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));
    objects.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
    objects.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));
    objects.add(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));

    // Boxes
    let glass = Dielectric::new(1.5);
    let mut box1: Arc<dyn Hittable> =
        Cuboid::new(Point3::ORIGIN, point!(165.0, 330.0, 165.0), white);
    // let aluminum = Metal::new_rgbf(0.8, 0.85, 0.88, 0.0);
    // let mut box1: Arc<dyn Hittable> = Cuboid::new(
    //     Point3::ORIGION,
    //     point!(165.0, 330.0, 165.0),
    //     aluminuium,
    // )
    box1 = RotateY::new(box1, 15.0);
    box1 = Translate::new(box1, vec3!(265.0, 0.0, 295.0));
    objects.add(box1);

    // let mut box2: Arc<dyn Hittable> =
    //     Cuboid::new(Point3::ORIGION, point!(165.0, 165.0, 165.0), white);
    // box2 = RotateY::new(box2, -18.0);
    // box2 = Translate::new(box2, vec3!(130.0, 0.0, 65.0));
    // objects.add(box2);

    let sphere: Arc<dyn Hittable> = Sphere::new(point!(190.0, 90.0, 190.0), 90.0, glass);
    lights.add(sphere.clone());
    objects.add(sphere);

    // (objects, upper_light)

    (objects, Arc::new(lights))
}

pub fn cornell_smoke() -> (HittableList, Arc<dyn Hittable>) {
    let mut objects = HittableList::new();

    let red = Lambertian::new_rgb(0.65, 0.05, 0.05);
    let white = Lambertian::new_rgb(0.73, 0.73, 0.73);
    let green = Lambertian::new_rgb(0.12, 0.45, 0.15);
    let light = DiffuseLight::white(7.0);

    let lights = XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light);
    objects.add(FlipFace::new(lights.clone()));

    objects.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green));
    objects.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));
    objects.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
    objects.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));
    objects.add(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));

    // Boxes
    let mut box1: Arc<dyn Hittable> =
        Cuboid::new(Point3::ORIGIN, point!(165.0, 330.0, 165.0), white.clone());
    box1 = RotateY::new(box1, 15.0);
    box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));
    let mut box2: Arc<dyn Hittable> =
        Cuboid::new(Point3::ORIGIN, point!(165.0, 165.0, 165.0), white);
    box2 = RotateY::new(box2, -18.0);
    box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));

    objects.add(ConstantMedium::new(box1, 0.01, Color::BLACK));
    objects.add(ConstantMedium::new(box2, 0.01, rgb!(1.0, 1.0, 1.0)));

    (objects, lights)
}

pub fn final_scene() -> (HittableList, Arc<dyn Hittable>) {
    let mut rng = rand::thread_rng();
    let mut objects = HittableList::new();
    let mut lights = HittableList::new();

    let mut boxes1 = HittableList::new();
    let ground = Lambertian::new_rgb(0.48, 0.83, 0.53);

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

            boxes1.add(Cuboid::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }
    objects.add(BVHNode::new_with_list(boxes1, 0.0, 1.0));

    let light = DiffuseLight::white(7.0);
    let upper_light = XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light);
    objects.add(FlipFace::new(upper_light.clone()));
    lights.add(upper_light);

    let center1 = point!(400.0, 400.0, 200.0);
    let center2 = center1 + vec3!(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::new_rgb(0.7, 0.3, 0.1);
    objects.add(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    ));

    objects.add(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    ));
    objects.add(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new_rgbf(0.8, 0.8, 0.9, 10.0),
    ));

    let boundary = Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    objects.add(boundary.clone());
    lights.add(boundary.clone());
    objects.add(ConstantMedium::new(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ));
    let boundary = Sphere::new(Point3::ORIGIN, 5000.0, Dielectric::new(1.5));
    objects.add(ConstantMedium::new(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    ));

    let emat = Lambertian::new(ImageTexture::new("assets/earthmap.jpg"));
    objects.add(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat));
    let pertext = NoiseTexture::new(0.1);
    objects.add(Sphere::new(
        Point3::new(200.0, 280.0, 300.0),
        80.0,
        Lambertian::new(pertext),
    ));

    let mut boxes2 = HittableList::new();
    let white = Lambertian::new_rgb(0.73, 0.73, 0.73);
    for _ in 0..1000 {
        boxes2.add(Sphere::new(
            Point3::random_with_bound(&mut rng, 0.0, 165.0),
            10.0,
            white.clone(),
        ));
    }
    objects.add(Translate::new(
        RotateY::new(BVHNode::new_with_list(boxes2, 0.0, 1.0), 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    ));

    (objects, Arc::new(lights))
}
