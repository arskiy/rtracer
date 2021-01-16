use crate::vec3::*;
use crate::onb::ONB;
use crate::hittable::*;

use std::f32::consts;

pub trait PDF {
    fn value(&self, dir: Vec3) -> f32;
    fn generate(&self) -> Vec3;
}

pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        Self { uvw: ONB::build_from_w(w) }
    }
}

impl PDF for CosinePDF {
    fn value(&self, dir: Vec3) -> f32 {
        let cosine = dir.unit_vector().dot(self.uvw.w);
        if cosine <= 0.0 { 0.0 } else { cosine / consts::PI }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_vec3(Vec3::random_cosine_dir())
    }
}

pub struct HittablePDF {
    orig: Point3,
    hit: Box<dyn Hittable>,
}

impl HittablePDF {
    pub fn new(orig: Point3, hit: Box<dyn Hittable>) -> Self {
        Self { orig, hit }
    }
}

impl PDF for HittablePDF {
    fn value(&self, dir: Vec3) -> f32 {
        self.hit.pdf_value(self.orig, dir)
    }

    fn generate(&self) -> Vec3 {
        self.hit.random(self.orig)
    }
}
