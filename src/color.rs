use crate::vec3::*;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) -> Vec3 {
    let mut ret = pixel_color;

    let scale = 1.0 / samples_per_pixel as f32;
    ret.x = (scale * ret.x).sqrt();
    ret.y = (scale * ret.y).sqrt();
    ret.z = (scale * ret.z).sqrt();

    ret.x = 256.0 * clamp(ret.x, 0.0, 0.99);
    ret.y = 256.0 * clamp(ret.y, 0.0, 0.99);
    ret.z = 256.0 * clamp(ret.z, 0.0, 0.99);

    ret
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
