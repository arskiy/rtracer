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


pub struct PolkaDotTexture {
    bg: Box<dyn Texture>,
    fg: Box<dyn Texture>,
    dot_radius: f32,
    dots_distance: f32
}

impl PolkaDotTexture {
    pub fn new(bg: Box<dyn Texture>, fg: Box<dyn Texture>, dot_radius: f32, dots_distance: f32) -> Self {
        Self { bg, fg, dot_radius, dots_distance }
    }
    
    pub fn new_color(color_bg: Color, color_fg: Color, dot_radius: f32, dots_distance: f32) -> Self {
        Self { bg: Box::new(SolidColorTexture::new(color_bg)), fg: Box::new(SolidColorTexture::new(color_fg)), dot_radius, dots_distance }
    }
}

impl Texture for PolkaDotTexture {
    fn value(&self, mut u: f32, mut v: f32, p: Point3) -> Color {
        let grid = 600;

        u = Vec3::clamp(u, 0.0, 1.0);
        v = Vec3::clamp(v, 0.0, 1.0);

        let mut x = (u * grid as f32) as u32;
        let mut y = ((1.0 - v) * grid as f32) as u32;

        if x > grid - 1 { x = grid - 1 }
        if y > grid - 1 { y = grid - 1 }

        if x as f32 / (self.dots_distance * 5.0) + y as f32 / (self.dots_distance * 5.0) < self.dot_radius {
            self.fg.value(u, v, p)
        } else {
            self.bg.value(u, v, p)
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
        let image = image::open(path).expect("image not found").to_rgb();
        let (nx, ny) = image.dimensions();
        let data = image.into_raw();

        /*
        for y in 0..ny {
            for x in 0..nx {
                let idx = (3 * x + 3 * nx * y) as usize;
                println!("{} {} {}", data[idx], data[idx + 1], data[idx + 2]);
            }
        }
        */
            
        ImageTexture { data, nx, ny } 
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f32, mut v: f32, _p: Point3) -> Color {
        if self.data.is_empty() {
            // debugging aid
            return Color::new(0.2, 0.5, 1.0);
        }
        
        u = Vec3::clamp(u, 0.0, 1.0);
        v = Vec3::clamp(v, 0.0, 1.0);

        let mut x = (u * self.nx as f32) as u32;
        let mut y = ((1.0 - v) * self.ny as f32) as u32;

        if x > self.nx - 1 { x = self.nx - 1 }
        if y > self.ny - 1 { y = self.ny - 1 }

        let idx = (3 * x + 3 * self.nx * y) as usize;

        let r = self.data[idx] as f32 / 255.0;
        let g = self.data[idx + 1] as f32 / 255.0;
        let b = self.data[idx + 2] as f32 / 255.0;

        Color::new(r, g, b)
    }
}
