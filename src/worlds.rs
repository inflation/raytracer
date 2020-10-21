use crate::prelude::*;

use crate::{
    aarect::*, bvh::*, constant_medium::*, cuboid::*, hittable_list::*, moving_sphere::*, sphere::*,
};

use rand::Rng;
use std::sync::Arc;

fn add_lights(world: &mut HittableList, lights: &mut HittableList, object: Arc<dyn Hittable>) {
    lights.add(object.clone());
    world.add(object);
}

pub struct World {
    world: HittableList,
    lights: HittableList,
}

impl World {
    pub fn world(&self) -> &HittableList {
        &self.world
    }
    pub fn lights(&self) -> &HittableList {
        &self.lights
    }
}

impl World {
    pub fn random_scene() -> Self {
        let mut world = HittableList::new();
        let lights = HittableList::new();

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
                let choose_mat: f32 = rng.gen();
                let center = Point3::new(
                    a as f32 + 0.9 * rng.gen::<f32>(),
                    0.2,
                    b as f32 + 0.9 * rng.gen::<f32>(),
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
                        let sphere = Sphere::new(center, 0.2, sphere_material);

                        balls.add(sphere);
                    } else {
                        // glass
                        let sphere_material = Dielectric::new(1.5);
                        let sphere = Sphere::new(center, 0.2, sphere_material);

                        balls.add(sphere);
                    }
                }
            }
        }
        world.add(BVHNode::new_with_list(balls, 0.0, 1.0));

        let glass = Dielectric::new(1.5);
        let sphere = Sphere::new(point!(0.0, 1.0, 0.0), 1.0, glass);
        world.add(sphere);

        let mat = Lambertian::new_rgb(0.4, 0.2, 0.1);
        world.add(Sphere::new(point!(-4.0, 1.0, 0.0), 1.0, mat));

        let metal = Metal::new_rgbf(0.7, 0.6, 0.5, 0.0);
        let sphere = Sphere::new(point!(4.0, 1.0, 0.0), 1.0, metal);
        world.add(sphere);

        Self { world, lights }
    }

    pub fn two_spheres() -> Self {
        let mut world = HittableList::new();
        let lights = HittableList::new();

        let checker = CheckerTexture::new(rgb!(0.2, 0.3, 0.1), rgb!(0.9, 0.9, 0.9));

        world.add(Sphere::new(
            point!(0.0, -10.0, 0.0),
            10.0,
            Lambertian::new(checker.clone()),
        ));
        world.add(Sphere::new(
            point!(0.0, 10.0, 0.0),
            10.0,
            Lambertian::new(checker),
        ));
        let bvh = BVHNode::new_with_list(world, 0.0, 1.0);
        world = HittableList::new();
        world.add(bvh);

        Self { world, lights }
    }

    pub fn two_perlin_spheres() -> Self {
        let mut world = HittableList::new();
        let lights = HittableList::new();

        let pertext = NoiseTexture::new(4.0);

        world.add(Sphere::new(
            point!(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(pertext.clone()),
        ));
        world.add(Sphere::new(
            point!(0.0, 2.0, 0.0),
            2.0,
            Lambertian::new(pertext),
        ));

        Self { world, lights }
    }

    pub fn earth() -> Self {
        let mut world = HittableList::new();
        let lights = HittableList::new();

        let earth_texture = ImageTexture::new("assets/earthmap.jpg");
        let earth_surface = Lambertian::new(earth_texture);
        let globe = Sphere::new(Point3::origin(), 2.0, earth_surface);
        world.add(globe);

        Self { world, lights }
    }

    pub fn simple_light() -> Self {
        let mut world = HittableList::new();
        let lights = HittableList::new();

        let pertext = NoiseTexture::new(4.0);
        world.add(Sphere::new(
            point!(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(pertext.clone()),
        ));
        world.add(Sphere::new(
            point!(0.0, 2.0, 0.0),
            2.0,
            Lambertian::new(pertext),
        ));

        let difflight = DiffuseLight::white(4.0);
        let light = XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight);
        world.add(light);

        Self { world, lights }
    }

    pub fn cornell_box() -> Self {
        let mut world = HittableList::new();
        let mut lights = HittableList::new();

        let red = Lambertian::new_rgb(0.65, 0.05, 0.05);
        let white = Lambertian::new_rgb(0.73, 0.73, 0.73);
        let green = Lambertian::new_rgb(0.12, 0.45, 0.15);
        let light = DiffuseLight::white(15.0);

        let light = FlipFace::new(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light));

        add_lights(&mut world, &mut lights, light);

        world.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green));
        world.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));
        world.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
        world.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));
        world.add(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));

        // Boxes
        let glass = Dielectric::new(1.5);
        let mut box1: Arc<dyn Hittable> =
            Cuboid::new(Point3::origin(), point!(165.0, 330.0, 165.0), white);
        // let aluminum = Metal::new_rgbf(0.8, 0.85, 0.88, 0.0);
        // let mut box1: Arc<dyn Hittable> = Cuboid::new(
        //     Point3::origin(),
        //     point!(165.0, 330.0, 165.0),
        //     aluminuium,
        // )
        box1 = RotateY::new(box1, 15.0);
        box1 = Translate::new(box1, vec3!(265.0, 0.0, 295.0));
        world.add(box1);

        // let mut box2: Arc<dyn Hittable> =
        //     Cuboid::new(Point3::origin(), point!(165.0, 165.0, 165.0), white);
        // box2 = RotateY::new(box2, -18.0);
        // box2 = Translate::new(box2, vec3!(130.0, 0.0, 65.0));
        // world.add(box2);

        let sphere: Arc<dyn Hittable> = Sphere::new(point!(190.0, 90.0, 190.0), 90.0, glass);
        add_lights(&mut world, &mut lights, sphere);

        // (world, upper_light)

        Self { world, lights }
    }

    pub fn cornell_smoke() -> Self {
        let mut world = HittableList::new();
        let mut lights = HittableList::new();

        let red = Lambertian::new_rgb(0.65, 0.05, 0.05);
        let white = Lambertian::new_rgb(0.73, 0.73, 0.73);
        let green = Lambertian::new_rgb(0.12, 0.45, 0.15);
        let light = DiffuseLight::white(7.0);

        let light = FlipFace::new(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, light));
        add_lights(&mut world, &mut lights, light);

        world.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green));
        world.add(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red));
        world.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
        world.add(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));
        world.add(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));

        // Boxes
        let mut box1: Arc<dyn Hittable> =
            Cuboid::new(Point3::origin(), point!(165.0, 330.0, 165.0), white.clone());
        box1 = RotateY::new(box1, 15.0);
        box1 = Translate::new(box1, Vec3::new(265.0, 0.0, 295.0));
        let mut box2: Arc<dyn Hittable> =
            Cuboid::new(Point3::origin(), point!(165.0, 165.0, 165.0), white);
        box2 = RotateY::new(box2, -18.0);
        box2 = Translate::new(box2, Vec3::new(130.0, 0.0, 65.0));

        world.add(ConstantMedium::new(box1, 0.01, Color::black()));
        world.add(ConstantMedium::new(box2, 0.01, rgb!(1.0, 1.0, 1.0)));

        Self { world, lights }
    }

    pub fn final_scene() -> Self {
        let mut rng = rand::thread_rng();
        let mut world = HittableList::new();
        let mut lights = HittableList::new();

        let mut boxes1 = HittableList::new();
        let ground = Lambertian::new_rgb(0.48, 0.83, 0.53);

        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let w = 100.0;
                let x0 = -1000.0 + i as f32 * w;
                let z0 = -1000.0 + j as f32 * w;
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = rng.gen_range(1.0, 101.0);
                let z1 = z0 + w;

                boxes1.add(Cuboid::new(
                    point!(x0, y0, z0),
                    point!(x1, y1, z1),
                    ground.clone(),
                ));
            }
        }
        world.add(BVHNode::new_with_list(boxes1, 0.0, 1.0));

        let light = DiffuseLight::white(7.0);
        let light = FlipFace::new(XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light));
        add_lights(&mut world, &mut lights, light);

        let center1 = point!(400.0, 400.0, 200.0);
        let center2 = center1 + vec3!(30.0, 0.0, 0.0);
        let moving_sphere_material = Lambertian::new_rgb(0.7, 0.3, 0.1);
        world.add(MovingSphere::new(
            center1,
            center2,
            0.0,
            1.0,
            50.0,
            moving_sphere_material,
        ));

        let sphere = Sphere::new(point!(260.0, 150.0, 45.0), 50.0, Dielectric::new(1.5));
        add_lights(&mut world, &mut lights, sphere);
        let sphere = Sphere::new(
            point!(0.0, 150.0, 145.0),
            50.0,
            Metal::new_rgbf(0.8, 0.8, 0.9, 10.0),
        );
        add_lights(&mut world, &mut lights, sphere);

        let boundary = Sphere::new(point!(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
        world.add(ConstantMedium::new(
            boundary.clone(),
            0.2,
            rgb!(0.2, 0.4, 0.9),
        ));
        add_lights(&mut world, &mut lights, boundary);
        let boundary = Sphere::new(Point3::origin(), 5000.0, Dielectric::new(1.5));
        world.add(ConstantMedium::new(boundary, 0.0001, rgb!(1.0, 1.0, 1.0)));

        let emat = Lambertian::new(ImageTexture::new("assets/earthmap.jpg"));
        world.add(Sphere::new(point!(400.0, 200.0, 400.0), 100.0, emat));
        let pertext = NoiseTexture::new(0.1);
        world.add(Sphere::new(
            point!(200.0, 280.0, 300.0),
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
        world.add(Translate::new(
            RotateY::new(BVHNode::new_with_list(boxes2, 0.0, 1.0), 15.0),
            vec3!(-100.0, 270.0, 395.0),
        ));

        Self { world, lights }
    }
}
