use std::ops;

use rand;
use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        self.x /= t;
        self.y /= t;
        self.z /= t;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, t: f32) -> Self::Output {
        Self {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, t: f32) -> Self::Output {
        self * (1.0 / t)
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        other / self
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("oob access in Index"),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("oob mutable access in IndexMut"),
        }
    }
}

impl Vec3 {
    pub fn new_empty() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn random() -> Self {
        Self {
            x: rand::random(),
            y: rand::random(),
            z: rand::random(),
        }
    }

    pub fn random_range(min: f32, max: f32) -> Self {
        let mut rng = thread_rng();
        Self {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn print(&self) -> String {
        format!("{} {} {}", self.x as i32, self.y as i32, self.z as i32)
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = thread_rng();
        let mut p;
        loop {
            p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() >= 1.0 {
                break;
            }
        }
        p
    }

    pub fn calc_color(pixel_color: Color, samples_per_pixel: usize) -> Vec3 {
        let mut ret = pixel_color;

        if ret.x.is_nan() { ret.x = 0.0; }
        if ret.y.is_nan() { ret.y = 0.0; }
        if ret.z.is_nan() { ret.z = 0.0; }
    
        let scale = 1.0 / samples_per_pixel as f32;
        ret.x = (scale * ret.x).sqrt();
        ret.y = (scale * ret.y).sqrt();
        ret.z = (scale * ret.z).sqrt();
    
        ret.x = 256.0 * Self::clamp(ret.x, 0.0, 0.99);
        ret.y = 256.0 * Self::clamp(ret.y, 0.0, 0.99);
        ret.z = 256.0 * Self::clamp(ret.z, 0.0, 0.99);
    
        ret
    }

    pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
        if x < min {
            min
        } else if x > max {
            max
        } else {
            x
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        let unit = Vec3::new(1.0, 1.0, 1.0);
        loop {
            let p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) - unit;
            if p.length_squared() < 1.0 {
                return p
            }
        }
    }

    pub fn random_in_hemisphere(normal: Self) -> Self {
        let in_unit = Self::random_in_unit_sphere();
        if in_unit.dot(normal) > 0.0 {
            in_unit
        } else {
            -in_unit
        }
    }

    pub fn random_cosine_dir() -> Self {
        let r1 = rand::random::<f32>();
        let r2 = rand::random::<f32>();

        let z = (1.0 - r2).sqrt();

        let phi = 2.0 * std::f32::consts::PI * r1;
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();

        Self { x, y, z }
    }

    pub fn is_nan(&self) -> bool {
        if self.x.is_nan() { return true; }
        if self.y.is_nan() { return true; }
        if self.z.is_nan() { return true; }

        false
    }
}


impl IntoIterator for Vec3 {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y, self.z].into_iter()
    }
}


pub type Color = Vec3;
pub type Point3 = Vec3;


