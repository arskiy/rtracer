pub mod vec3;
pub mod color;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod camera;

use vec3::*;
use ray::Ray;
use sphere::Sphere;
use hittable::{Hittable, HitRecord};
use hittable_list::HittableList;
use camera::Camera;

use rand::prelude::*;

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
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

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
            for s in 0..samples_per_pixel {
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
            let target = hit.p + hit.normal + Vec3::random_in_unit_sphere();
            let x = 0.5 * ray_color(Ray::new(hit.p, target - hit.p), world, depth - 1);
            //eprintln!("x: {:?}", x);
            x
            
            //return 0.5 * (hit.normal + Color::new(1.0,1.0,1.0));
        },
        None => {
            let unit_dir = r.dir.unit_vector();
            let t = 0.5 * (unit_dir.y + 1.0);
            return Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;
        },
    }
}
