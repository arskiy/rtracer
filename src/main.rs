#[allow(dead_code)]

pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;
pub mod aabb;
pub mod bvh;
pub mod texture;
pub mod perlin;

use camera::Camera;
use hittable::{Hittable, HittableList};
use material::*;
use ray::Ray;
use sphere::*;
use vec3::*;
use texture::*;

use rand::prelude::*;
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

// Image
const ASPECT_RATIO: f32 = 3.0 / 2.0;
const NX: i32 = 500;
const NY: i32 = (NX as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

fn main() {
    println!("P3\n{} {}\n255", NX, NY);
    // println!("P3\n1200 900\n255");

    let (world, cam) = image();

    eprintln!("Rendering!");
    let image: Arc<Mutex<Box<[[Color; NX as usize]; NY as usize]>>> = Arc::new(Mutex::new(
        Box::new([[Vec3::new_empty(); NX as usize]; NY as usize]),
    ));

    (0..NY).into_par_iter().rev().for_each(|y| {
        eprintln!("Scanlines remaining: {}", y);
        for x in 0..NX {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f32 + rand::random::<f32>()) / (NX - 1) as f32;
                let v = (y as f32 + rand::random::<f32>()) / (NY - 1) as f32;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }
            image.lock().unwrap()[y as usize][x as usize] =
                Vec3::calc_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    });

    eprintln!("Outputting image!");
    let img = image.lock().unwrap();
    for y in (0..img.len()).rev() {
        for x in 0..img[y].len() {
            println!(
                "{} {} {}",
                img[y][x].x as u8, img[y][x].y as u8, img[y][x].z as u8
            );
        }
    }
}

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new_empty();
    }

    match world.hit(&r, 0.001, std::f32::INFINITY) {
        Some(hit) => {
            if let Some((scattered, attenuation)) = hit.material.scatter(r, &hit) {
                let x = attenuation * ray_color(scattered, world, depth - 1);
                return x;
            }
            return Color::new_empty();
        }
        None => {
            let unit_dir = r.dir.unit_vector();
            let t = 0.5 * (unit_dir.y + 1.0);
            return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
        }
    }
}

fn first_scene() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_bh = Lambertian::new(Color::new(0.2, 0.3, 0.8));
    let material_left = Dieletric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.2, 0.8), 0.2);

    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -2.0), 0.5, material_bh)));
    world.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));


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
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    (world, cam)
}

fn random_scene_book() -> (HittableList, Camera) {
    let mut rng = rand::thread_rng();
    let mut world = HittableList::new();

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
    (world, cam)
}

fn two_checkered_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();
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
    (world, cam)
}

fn two_perlin_spheres() -> (HittableList, Camera) {
    let mut world = HittableList::new();

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
    (world, cam)
}

fn image() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let texture = ImageTexture::new("../shapiro.jpg");
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, Lambertian::new_texture(Box::new(texture)))));

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
    (world, cam)
}
