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
pub mod aarect;
pub mod onb;
pub mod pdf;
pub mod scenes;

use hittable::*;
use material::*;
use ray::Ray;
use vec3::*;
use pdf::*;

use std::mem;
use std::ptr;

use std::sync::{Arc, Mutex};
use rayon::prelude::*;

use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

const ASPECT_RATIO: f32 = 1.0;
const NX: usize = 500;
const NY: usize = (NX as f32 / ASPECT_RATIO) as usize;
const SAMPLES_PER_PIXEL: usize = 10;
const MAX_DEPTH: i32 = 50;

// assumes constructor will never panic. we're safe using just Box::new()
macro_rules! make_array {
    ($constructor:expr; $n:expr) => {{
        let mut items: [_; $n] = mem::MaybeUninit::uninit().assume_init();
        for place in items.iter_mut() {
            ptr::write(place, $constructor);
        }
        items
    }}
}

fn main() -> std::io::Result<()> {
    let (world, cam, background, lights) = scenes::cornell_box(ASPECT_RATIO);

    eprintln!("Rendering!");

    // deterministic and low-discrepancy sequence for MC sims
    let hx = halton::Sequence::new(2).map(|x| x as f32).take(SAMPLES_PER_PIXEL).collect::<Vec<f32>>();
    let hy = halton::Sequence::new(3).map(|x| x as f32).take(SAMPLES_PER_PIXEL).collect::<Vec<f32>>();

    /*
    if world.len() == 1 {
        println!("P3\n{} {}\n255", NX, NY);
        (0..NY).into_par_iter().rev().for_each(|y| {
            eprintln!("Scanlines remaining: {}", y);
            for x in 0..NX {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for i in 0..SAMPLES_PER_PIXEL {
                    let u = (x as f32 + hx[i]) / (NX - 1) as f32;
                    let v = (y as f32 + hy[i]) / (NY - 1) as f32;

                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(r, background, &world[0], &lights[0], MAX_DEPTH);
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
    } else {
    */
        for frame in 0..world.len() {
            let image = unsafe { Arc::new(Mutex::new(
                Box::new(make_array!( Box::new([Vec3::new_empty(); NX]); NY ),
            ))) };

            (0..NY).into_par_iter().rev().for_each(|y| {
                // eprintln!("Scanlines remaining: {} / i: {}", y, i);
                for x in 0..NX {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                    for i in 0..SAMPLES_PER_PIXEL {
                        let u = (x as f32 + hx[i]) / (NX - 1) as f32;
                        let v = (y as f32 + hy[i]) / (NY - 1) as f32;

                        let r = cam.get_ray(u, v);
                        pixel_color += ray_color(r, background, &world[frame], &lights[frame], MAX_DEPTH);
                    }

                    image.lock().unwrap()[y as usize][x as usize] =
                        Vec3::calc_color(pixel_color, SAMPLES_PER_PIXEL);
                }
            });

            eprintln!("Outputting image {}!", frame);
            let f = File::create(format!("image{:03}.ppm", frame))?;
            let mut f = LineWriter::new(f);
            f.write_all(format!("P3\n{} {}\n255\n", NX, NY).as_bytes())?;

            let img = image.lock().unwrap();
            for y in (0..img.len()).rev() {
                for x in 0..img[y].len() {
                    f.write_all(format!(
                        "{} {} {}\n",
                        img[y][x].x as u8, img[y][x].y as u8, img[y][x].z as u8
                    ).as_bytes())?;
                }
            }
        }
    // }
    Ok(())
}

fn ray_color(ray: Ray, background: Color, world: &HittableList, lights: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new_empty();
    }

    match world.hit(&ray, 0.001, std::f32::INFINITY) {
        Some(hit) => {
            let emitted = hit.material.emitted(&ray, &hit);

            if let Some(reflection) = hit.material.scatter(&ray, &hit) {
                match reflection {
                    ReflectionRecord::Specular { specular_ray, attenuation } => {
                        return attenuation *
                            ray_color(specular_ray, background, world, &lights, depth - 1);
                    }

                    ReflectionRecord::Scatter { pdf: reflection_cosine_pdf, attenuation } => {
                        let light_pdf = HittablePDF::new(hit.p, lights);
                        let mixture_pdf = MixturePDF::new(&light_pdf, &*reflection_cosine_pdf);

                        let scattered = Ray::new(hit.p, mixture_pdf.generate(), ray.time);
                        let pdf_val = mixture_pdf.value(scattered.dir);

                        return emitted + attenuation
                            * hit.material.scattering_pdf(&ray, &hit, &scattered)
                            * ray_color(scattered, background, world, &lights, depth - 1) / pdf_val
                    }
                }
            }

            return emitted;
        }
        None => {
            return background;
        }
    }
}
