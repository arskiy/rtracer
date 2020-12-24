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
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;
use camera::Camera;
use material::*;

use rand::prelude::*;

use std::rc::Rc;

fn main() {
    let mut rng = rand::thread_rng();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Dieletric::new(1.5);

    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Box::new(material_ground)))));
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Box::new(material_center)))));
    world.push(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Box::new(material_left)))));
    world.push(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Box::new(material_right)))));

    // Camera
    let cam = Camera::new();

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            /*
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;

            let ray = ray::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);

            let pixel_color = ray_color(ray, &mut world);
            */
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                let v = (j as f32 + rng.gen::<f32>()) / (image_height - 1) as f32;

                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            color::write_color(pixel_color, samples_per_pixel);
        }
    }
}

fn ray_color(r: Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new_empty();
    }

    match world.hit(&r, 0.001, std::f32::INFINITY) {
        Some(hit) => {

            let mut scattered = Ray::new(Vec3::new_empty(), Vec3::new_empty());
            let mut attenuation = Color::new_empty();
            if hit.material.scatter(r, hit.normal, hit.p, hit.front_face, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(scattered, world, depth - 1);
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
