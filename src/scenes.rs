use crate::aarect::*;
use crate::bvh::*;
use crate::camera::*;
use crate::gltf::GLTF;
use crate::hittable::*;
use crate::material::*;
use crate::matrix4::Matrix4;
use crate::sphere::*;
use crate::texture::*;
use crate::transforms::*;
use crate::triangle::Triangle;
use crate::vec3::*;

use std::sync::Arc;
use rand::Rng;

pub fn cornell_box(aspect_ratio: f32) -> (Vec<HittableList>, Camera, Color, Vec<HittableList>) {
    let background = Color::new(0.0, 0.0, 0.0);
    let mut world_vec = vec![];
    let mut lights_vec = vec![];

    let gltf = GLTF::new("../models/matilda/scene.gltf".to_string()).unwrap();

    let mut world = HittableList::new();
    let mut lights = HittableList::new();

    let red = Lambertian::new(SolidColorTexture::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColorTexture::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColorTexture::new(Color::new(0.12, 0.45, 0.15)));
    let aluminum = Metal::new(Color::new(0.8, 0.85, 0.88), 0.0);

    let mut gltf_import: Vec<Arc<dyn Hittable>> = Vec::new();

    eprintln!("materials: {:?}", gltf.materials);
    for mesh in gltf.meshes {
        for indices in mesh.indices.chunks(3) {
            let gltf_mat = &gltf.materials[mesh.mat_index];
            let albedo = gltf_mat.albedo;
            eprintln!("{:?}", mesh.transform);

            // let (albedo, roughness) = &gltf_mat.metallic_roughness();

            /*
            let mat: Material = if gltf_mat.metallic == 0.0 {
                Lambertian::new(SolidColorTexture::new(albedo))
            } else {
                Metal::new(albedo, roughness)
            };*/

            gltf_import.push(Arc::new(Translate::new(
                Rotate::new(
                Rotate::new(
                Rotate::new(
                    Triangle::new(
                        Lambertian::new(SolidColorTexture::new(albedo)),
                        Matrix4::scale(Vec3::new(100.0, 100.0, 100.0))
                            * mesh.transform
                            * mesh.positions[indices[0] as usize],
                        Matrix4::scale(Vec3::new(100.0, 100.0, 100.0))
                            * mesh.transform
                            * mesh.positions[indices[1] as usize],
                        Matrix4::scale(Vec3::new(100.0, 100.0, 100.0))
                            * mesh.transform
                            * mesh.positions[indices[2] as usize],
                    ),
                    Axis::X,
                    90.0,
                ),
                Axis::Z,
                180.0,
                ),
                Axis::Y,
                -90.0,
                ),
                // Vec3::new(235.0, 200.0, 100.0),
                Vec3::new(270.0, 60.0, 110.0),
            )));
        }
    }

    world.push(BVH::new(gltf_import, 0.0, 1.0));

    let light = DiffuseLight::new(SolidColorTexture::new(Color::new(12.0, 6.807, 2.086)));
    let light_ceiling = AARect::new(Plane::XZ, light.clone(), 177.0, 392.0, 163.0, 393.0, 554.0);
    world.push(light_ceiling);
    let light_ceiling = AARect::new(Plane::XZ, light.clone(), 177.0, 392.0, 163.0, 393.0, 554.0);
    lights.push(light_ceiling);

    world.push(AARect::new(
        Plane::YZ,
        green.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    ));
    world.push(AARect::new(
        Plane::YZ,
        red.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    ));
    world.push(AARect::new(
        Plane::XZ,
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
    ));
    world.push(AARect::new(
        Plane::XZ,
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    ));
    world.push(AARect::new(
        Plane::XY,
        white.clone(),
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
    ));

    let box1 = RectBox::new(
        Point3::new(130.0, 0.0, 65.0),
        Point3::new(295.0, 165.0, 230.0),
        white.clone(),
    );

    let box1 = Rotate::new(box1, Axis::Y, -18.0);
    let box1 = Translate::new(box1, Vec3::new(0.0, 0.0, -30.0));
    // world.push(box1);

    /*
    let glass_sphere = Sphere::new(Vec3::new(190.0, 90.0, 190.0), 90.0, Dieletric::new(1.5));
    world.push(glass_sphere.clone());
    lights.push(glass_sphere);
    */

    let box2 = RectBox::new(
        Point3::new(265.0, 0.0, 295.0),
        Point3::new(430.0, 330.0, 460.0),
        aluminum,
    );
    let box2 = Rotate::new(box2, Axis::Y, 12.0);
    let box2 = Translate::new(box2, Vec3::new(-35.0, 0.0, 40.0));
    world.push(box2);

    world_vec.push(world);
    lights_vec.push(lights);

    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let fov = 40.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (world_vec, cam, background, lights_vec)
}

pub fn book2_scene(
    aspect_ratio: f32
) -> (Vec<HittableList>, Camera, Color, Vec<HittableList>) {
    let mut lights = HittableList::new();
    let mut objects = HittableList::new();

    let ground = Lambertian::new(SolidColorTexture::new(Vec3::new(0.48, 0.83, 0.53)));

    let mut boxes1: Vec<Arc<dyn Hittable>> = Vec::new();
    let boxes_per_side = 20;

    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand::random::<f32>() * 100.0;
            let z1 = z0 + w;
            boxes1.push(Arc::new(RectBox::new(Point3::new(x0, y0, z0), Point3::new(x1, y1, z1), ground.clone())));
        }
    }

    objects.push(BVH::new(boxes1, 0.0, 1.0));


    let light = DiffuseLight::new(SolidColorTexture::new(Color::new(7.0, 7.0, 7.0)));
    objects.push(AARect::new(Plane::XZ, light.clone(), 123.0, 423.0, 147.0, 412.0, 554.0));
    lights.push(AARect::new(Plane::XZ, light.clone(), 123.0, 423.0, 147.0, 412.0, 554.0));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);

    let moving_sphere_mat = Lambertian::new(SolidColorTexture::new(Color::new(0.7, 0.3, 0.1)));
    objects.push(MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, moving_sphere_mat));
    objects.push(Sphere::new(Point3::new(0.0, 150.0, 145.0), 50.0, Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)));

    let mut boxes2: Vec<Arc<dyn Hittable>> = Vec::new();
    let white = Lambertian::new(SolidColorTexture::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.push(Arc::new(Translate::new(Sphere::new(Point3::random_range_i32(0, 165), 10.0, white.clone()), Vec3::new(-100.0, 270.0, 395.0))));
    }

    objects.push(BVH::new(boxes2, 0.0, 1.0));

    let background = Color::new(0.0, 0.0, 0.0);

    let lookfrom = Point3::new(478.0, 278.0, -600.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let fov = 40.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    (vec!(objects), cam, background, vec!(lights))
}

pub fn cornell_box_animated(
    aspect_ratio: f32,
) -> (Vec<HittableList>, Camera, Color, Vec<HittableList>) {
    let background = Color::new(0.0, 0.0, 0.0);
    let mut world_vec = vec![];
    let mut lights_vec = vec![];

    for i in 0..15 {
        let mut world = HittableList::new();
        let mut lights = HittableList::new();

        let red: Lambertian<SolidColorTexture> =
            Lambertian::new(SolidColorTexture::new(Color::new(0.65, 0.05, 0.05)));
        let white = Lambertian::new(SolidColorTexture::new(Color::new(0.73, 0.73, 0.73)));
        let green = Lambertian::new(SolidColorTexture::new(Color::new(0.12, 0.45, 0.15)));
        let aluminum = Metal::new(Color::new(0.8, 0.85, 0.88), 0.0);

        let light = DiffuseLight::new(SolidColorTexture::new(Color::new(12.0, 6.807, 2.086)));
        let light_ceiling =
            AARect::new(Plane::XZ, light.clone(), 177.0, 392.0, 163.0, 393.0, 554.0);
        world.push(light_ceiling);
        let light_ceiling =
            AARect::new(Plane::XZ, light.clone(), 177.0, 392.0, 163.0, 393.0, 554.0);
        lights.push(light_ceiling);

        world.push(AARect::new(Plane::YZ, green, 0.0, 555.0, 0.0, 555.0, 555.0));
        world.push(AARect::new(Plane::YZ, red, 0.0, 555.0, 0.0, 555.0, 0.0));
        world.push(AARect::new(
            Plane::XZ,
            white.clone(),
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
        ));
        world.push(AARect::new(
            Plane::XZ,
            white.clone(),
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
        ));
        world.push(AARect::new(
            Plane::XY,
            white.clone(),
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
        ));

        let box1 = RectBox::new(
            Point3::new(130.0, 0.0, 65.0),
            Point3::new(295.0, 165.0, 230.0),
            white.clone(),
        );
        let box1 = Rotate::new(box1, Axis::X, i as f32 * -18.0);
        let box1 = Translate::new(box1, Vec3::new(0.0, 0.0, -30.0));
        world.push(box1);
        /*
        let glass_sphere = Sphere::new(Vec3::new(190.0, 90.0, 190.0), 90.0, Dieletric::new(1.5));
        world.push(glass_sphere.clone());
        lights.push(glass_sphere);
        */

        let box2 = RectBox::new(
            Point3::new(265.0, 0.0, 295.0),
            Point3::new(430.0, 330.0, 460.0),
            aluminum,
        );
        let box2 = Rotate::new(box2, Axis::Y, 18.0);
        let box2 = Translate::new(box2, Vec3::new(-35.0, 0.0, 40.0));
        world.push(box2);

        world_vec.push(world);
        lights_vec.push(lights);
    }

    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let fov = 40.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (world_vec, cam, background, lights_vec)
}

pub fn simple_light(aspect_ratio: f32) -> (HittableList, Camera, Color, HittableList) {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();
    let background = Color::new_empty();

    let pertext = NoiseTexture::new(4.0);
    world.push(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(pertext),
    ));

    let pertext = NoiseTexture::new(4.0);
    world.push(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(pertext),
    ));

    let difflight = DiffuseLight::new(SolidColorTexture::new(Color::new(4.0, 7.0, 9.0)));
    world.push(AARect::new(
        Plane::XY,
        difflight.clone(),
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
    ));
    lights.push(AARect::new(Plane::XY, difflight, 3.0, 5.0, 1.0, 3.0, -2.0));

    let difflight = DiffuseLight::new(SolidColorTexture::new(Color::new(9.0, 3.0, 2.0)));
    world.push(Sphere::new(
        Point3::new(1.0, 5.0, 4.0),
        1.0,
        difflight.clone(),
    ));
    lights.push(Sphere::new(Point3::new(1.0, 5.0, 4.0), 1.0, difflight));

    let lookfrom = Point3::new(26.0, 3.0, 6.0);
    let lookat = Point3::new(0.0, 2.0, 0.0);
    let fov = 20.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (world, cam, background, lights)
}

pub fn first_scene(aspect_ratio: f32) -> (Vec<HittableList>, Camera, Color, Vec<HittableList>) {
    let mut world = HittableList::new();
    let background = Color::new(0.7, 0.8, 1.0);

    let material_ground = Lambertian::new(SolidColorTexture::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Lambertian::new(SolidColorTexture::new(Color::new(0.7, 0.3, 0.3)));
    let material_bh = Lambertian::new(SolidColorTexture::new(Color::new(0.2, 0.3, 0.8)));
    let material_left = Dieletric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.2, 0.8), 0.2);

    world.push(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.push(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    ));
    world.push(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.push(Sphere::new(Point3::new(-1.0, 0.0, -2.0), 0.5, material_bh));
    world.push(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let lookfrom = Point3::new(0.0, 0.0, 1.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        90.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (vec![world], cam, background, vec![])
}

/*
fn random_scene_book() -> (HittableList, Camera, Color) {
    let mut rng = rand::thread_rng();
    let mut world = HittableList::new();
    let background = Color::new(0.7, 0.8, 1.0);

    let ground = Lambertian::new_texture(Box::new(CheckerTexture::new_color(Color::new(0.1, 0.1, 0.1), Color::new(0.9, 0.9, 0.9))));
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground,
    )));

    for i in -11..11 {
        for j in -11..11 {
            let choose_material = rand::random::<f32>();
            let mut center = Point3::new(
                i as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                j as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let radius = rng.gen_range(0.1..0.3);
                center.y += radius - 0.2;

                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.7), 0.0);
                    world.push(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        radius,
                        sphere_material,
                    )));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.push(Box::new(Sphere::new(center, radius, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Dieletric::new(1.5);
                    world.push(Box::new(Sphere::new(center, radius, sphere_material)));
                }
            }
        }
    }

    let material1 = Lambertian::new(Color::random());
    world.push(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Metal::new(Color::new(0.9, 0.4, 0.4), 0.2);
    world.push(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Metal::new(Color::new(0.95, 0.95, 0.95), 0.0);
    world.push(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));


    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let fov = 60.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (world, cam, background)
}

fn two_checkered_spheres() -> (HittableList, Camera, Color) {
    let mut world = HittableList::new();
    let background = Color::new(0.7, 0.8, 1.0);

    let checker = Lambertian::new_texture(Box::new(CheckerTexture::new_color(Color::new(0.1, 0.1, 0.1), Color::new(0.9, 0.9, 0.9))));

    world.push(Box::new(Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, checker)));

    let checker = Lambertian::new_texture(Box::new(CheckerTexture::new_color(Color::new(0.1, 0.1, 0.1), Color::new(0.9, 0.9, 0.9))));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, checker)));

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let fov = 20.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (world, cam, background)
}

fn polka_sphere() -> (HittableList, Camera, Color) {
    let mut world = HittableList::new();
    let background = Color::new(0.7, 0.8, 1.0);

    let polka = Lambertian::new_texture(Box::new(PolkaDotTexture::new_color(Color::new(0.1, 0.1, 0.1), Color::new(0.9, 0.9, 0.9), 0.2, 1.0)));


    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, polka)));

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let fov = 20.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (world, cam, background)
}

fn two_perlin_spheres() -> (HittableList, Camera, Color) {
    let mut world = HittableList::new();
    let background = Color::new(0.7, 0.8, 1.0);

    let pertext = NoiseTexture::new(4.0);
    world.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Lambertian::new_texture(Box::new(pertext)))));

    let pertext = NoiseTexture::new(4.0);
    world.push(Box::new(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Lambertian::new_texture(Box::new(pertext)))));

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let fov = 20.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (world, cam, background)
}

fn image() -> (HittableList, Camera, Color) {
    let mut world = HittableList::new();
    let background = Color::new(0.7, 0.8, 1.0);

    let texture = ImageTexture::new("../alteredstate-realbig.jpg");
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, Lambertian::new_texture(Box::new(texture)))));

    let lookfrom = Point3::new(13.0, -2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let fov = 20.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (world, cam, background)
}

fn cornell_smoke() -> (HittableList, Camera, Color) {
    let mut world = HittableList::new();
    let background = Color::new(0.0, 0.0, 0.0);

    let red = Lambertian::new(SolidColorTexture::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(SolidColorTexture::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(SolidColorTexture::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(SolidColorTexture::new(Color::new(12.0, 6.807, 2.086)));

    world.push(AARect::new(Plane::YZ, green, 0.0, 555.0, 0.0, 555.0, 555.0));
    world.push(AARect::new(Plane::YZ, red, 0.0, 555.0, 0.0, 555.0, 0.0));
    world.push(AARect::new(Plane::XZ, light, 177.0, 392.0, 163.0, 393.0, 554.0));
    world.push(AARect::new(Plane::XZ, white.clone(), 0.0, 555.0, 0.0, 555.0, 0.0));
    world.push(AARect::new(Plane::XZ, white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0));
    world.push(AARect::new(Plane::XY, white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0));

    let box1 = RectBox::new(Point3::new(130.0, 0.0, 65.0), Point3::new(295.0, 165.0, 230.0), white.clone());
    let box1 = RotateY::new(box1, -18.0);
    let box1 = Translate::new(box1, Vec3::new(0.0, 0.0, -30.0));
    world.push(ConstantMedium::new(box1, 0.01, SolidColorTexture::new(Color::new(0.0, 0.0, 0.0))));

    let box2 = RectBox::new(Point3::new(265.0, 0.0, 295.0), Point3::new(430.0, 330.0, 460.0), white);
    let box2 = RotateY::new(box2, 15.0);
    let box2 = Translate::new(box2, Vec3::new(-35.0, 0.0, 40.0));
    world.push(ConstantMedium::new(box2, 0.01, SolidColorTexture::new(Color::new(1.0, 1.0, 1.0))));

    let sphere_mat = NoiseTexture::new(5.0);
    // let sphere = Sphere::new(Point3::new(170.5, 240.0, 117.5), 75.0, Lambertian::new(sphere_mat));
    let sph = Sphere::new(Point3::new(277.0, 180.0, 350.0), 75.0, Lambertian::new(sphere_mat));
    world.push(sph);

    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let fov = 40.0;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        fov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (world, cam, background)
}
*/
