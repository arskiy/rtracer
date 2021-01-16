use std::ops;
use crate::vec3::Vec3;

pub struct ONB {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl ops::Index<usize> for ONB {
    type Output = Vec3;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.u,
            1 => &self.v,
            2 => &self.w,
            _ => panic!("oob in ONB index"),
        }
    }
}


impl ONB {
    pub fn local(&self, a: f32, b: f32, c: f32) -> Vec3 {
        a * self.u + b * self.v + c * self.w
    }

    pub fn local_vec3(&self, a: Vec3) -> Vec3 {
        a.x * self.u + a.y * self.v + a.z * self.w
    }

    pub fn build_from_w(n: Vec3) -> Self {
        let w = n.unit_vector();
        let a = if w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let v = w.cross(a).unit_vector();
        let u = w.cross(v);

        Self { u, v, w }
    }
}
