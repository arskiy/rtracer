pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod camera;
pub mod material;

use vec3::*;
use ray::Ray;
use sphere::Sphere;
use hittable::{Hittable};
use hittable_list::HittableList;
use camera::Camera;
use material::*;

use rand::prelude::*;

use rayon::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let nx: i32 = 600;
    let ny: i32 = (nx as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    //let mut world = random_scene_book();
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left = Dieletric::new(1.5);
    let material_right = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);

    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // Camera
    let lookfrom = Point3::new(0.0, 0.0, 0.5);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    //let lookfrom = Point3::new(-2.0, 2.0, -1.0);
    //let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 1.5;
    let aperture = 0.1;
    let cam = Camera::new(lookfrom, lookat, vup, 90.0, aspect_ratio, aperture, dist_to_focus);

    // Render

    println!("P3\n{} {}\n255", nx, ny);

    /*
    for y in (0..ny).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for x in 0..nx {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f32 + rng.gen::<f32>()) / (nx - 1) as f32;
                let v = (y as f32 + rng.gen::<f32>()) / (ny - 1) as f32;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            color::write_color(pixel_color, samples_per_pixel);
        }
    }
    */

    eprintln!("Rendering!");
    let mut image: Vec<Vec<Color>> = vec!(vec!());

    (0..ny).into_par_iter().rev().for_each(|y| {
        for x in 0..nx {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (x as f32 + rand::random::<f32>()) / (nx - 1) as f32;
                let v = (y as f32 + rand::random::<f32>()) / (ny - 1) as f32;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            image[y as usize][x as usize] = color::write_color(pixel_color, samples_per_pixel);
        }
    });

    for y in 0..image.len() {
        for x in 0..image[y].len() {
            println!("{} {} {}", image[y][x].x, image[y][x].y, image[y][x].z);
        }
    }
}

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new_empty();
    }

    match world.hit(&r, 0.001, std::f32::INFINITY) {
        Some(hit) => {
            if let Some((scattered, attenuation)) = hit.material.scatter(r, hit.normal, hit.p, hit.front_face) {
                let x = attenuation * ray_color(scattered, world, depth - 1);
                return x;
            }
            return Color::new_empty();
            /*
            let target = hit.p + hit.normal + Vec3::random_unit_vector();
            0.5 * ray_color(Ray::new(hit.p, target - hit.p), world, depth - 1)
            */ 
        },
        None => {
            let unit_dir = r.dir.unit_vector();
            let t = 0.5 * (unit_dir.y + 1.0);
            return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
        },
    }
}

fn random_scene_book() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut world = HittableList::new();

    let ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground)));

    for i in -11..11 {
        for j in -11..11 {
            let choose_material = rand::random::<f32>();
            let mut center = Point3::new(i as f32 + 0.9 * rng.gen::<f32>(), 0.2, j as f32 + 0.9 * rng.gen::<f32>());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let radius = rng.gen_range(0.1..0.3);
                center.y += radius - 0.2;

                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    world.push(Box::new(Sphere::new(center, radius, sphere_material)));
                } else {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.push(Box::new(Sphere::new(center, radius, sphere_material)));
                }
            }
        }
    }

    let material1 = Lambertian::new(Color::random());
    world.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Metal::new(Color::new(0.9, 0.4, 0.4), 0.2);
    world.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Metal::new(Color::new(0.95, 0.95, 0.95), 0.0);
    world.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    world
}
