use crate::vec3::{Vec3, Point3, Color};
use crate::perlin::Perlin;

pub trait Texture: Sync {
    fn value(&self, u: f32, v: f32, p: Point3) -> Color;
}

pub struct SolidColorTexture {
    color_value: Color,
}

impl SolidColorTexture {
    pub fn new(color_value: Color) -> Self {
        Self { color_value }
    }
    
    pub fn new_from_color(r: f32, g: f32, b: f32) -> Self {
        Self { color_value: Color::new(r, g, b) }
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _u: f32, _v: f32, _p: Point3) -> Color {
        return self.color_value
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>) -> Self {
        Self { odd, even }
    }

    pub fn new_color(c1: Color, c2: Color) -> Self {
        Self { odd: Box::new(SolidColorTexture::new(c2)), even: Box::new(SolidColorTexture::new(c1)) }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self { noise: Perlin::new(), scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + f32::sin(self.scale * (p.z + p.x) + 10.0 * self.noise.turb(p, 7)))
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    nx: u32,
    ny: u32
}

impl ImageTexture {
    pub fn new(path: &str) -> Self { 
        let image = image::open(path).expect("image not found").to_rgb8();
        let (nx, ny) = image.dimensions();
        let data = image.into_raw();
            
        ImageTexture { data, nx, ny } 
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f32, mut v: f32, _p: Point3) -> Color {
        /*
        let nx = self.nx as usize;
        let ny = self.ny as usize;

        let mut i = (u * nx as f32) as usize;
        let mut j = ((1.0 - v) * ny as f32) as usize;

        if i > nx - 1 { i = nx - 1 }
        if j > ny - 1 { j = ny - 1 }

        let idx = 3 * i + 3 * nx * j;

        let r = self.data[idx] as f32 / 255.0;
        let g = self.data[idx + 1] as f32 / 255.0;
        let b = self.data[idx + 2] as f32 / 255.0;
        */

        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }
        
        u = Vec3::clamp(u, 0.0, 1.0);
        v = 1.0 - Vec3::clamp(v, 0.0, 1.0);

        let mut i = (u * self.nx as f32) as u32;
        let mut j = (v * self.ny as f32) as u32;

        if i >= self.nx { i = self.nx - 1 }
        if j >= self.ny { j = self.ny - 1 }

        let color_scale = 1.0 / 255.0;

        let idx = (3 * i + 3 * self.nx * j) as usize;

        let r = self.data[idx] as f32 * color_scale;
        let g = self.data[idx + 1] as f32 * color_scale;
        let b = self.data[idx + 2] as f32 * color_scale;

        Color::new(r, g, b)
    }
}
