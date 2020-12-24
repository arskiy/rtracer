use crate::vec3::Color;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    println!("{} {} {}", (256.0 * clamp(r, 0.0, 0.99)) as i32, (256.0 * clamp(g, 0.0, 0.99)) as i32, (256.0 * clamp(b, 0.0, 0.99)) as i32);
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
