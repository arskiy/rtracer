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

// ---------------------------------------------------------------

// Probability Density Function for Hittable objects, mostly used for light sampling.
pub struct HittablePDF<'a> {
    orig: Point3,
    hit: &'a dyn Hittable,
}

impl<'a> HittablePDF<'a> {
    pub fn new(orig: Point3, hit: &'a dyn Hittable) -> Self {
        Self { orig, hit }
    }
}

impl PDF for HittablePDF<'_> {
    fn value(&self, dir: Vec3) -> f32 {
        self.hit.pdf_value(self.orig, dir)
    }

    fn generate(&self) -> Vec3 {
        self.hit.random(self.orig)
    }
}

// ---------------------------------------------------------------

// Mixture density of cosine and light sampling. 50/50 chance to pdfLight or pdfReflection.
pub struct MixturePDF<'a> {
    p0: &'a dyn PDF,
    p1: &'a dyn PDF,
}

impl<'a> MixturePDF<'a> {
    pub fn new(p0: &'a dyn PDF, p1: &'a dyn PDF) -> Self {
        Self { p0, p1 }
    }
}

impl PDF for MixturePDF<'_> {
    fn value(&self, dir: Vec3) -> f32 {
        0.5 * self.p0.value(dir) + 0.5 * self.p1.value(dir)
    }

    fn generate(&self) -> Vec3 {
        if rand::random::<f32>() < 0.5 {
            self.p0.generate()
        } else {
            self.p1.generate()
        }
    }
}

pub fn random_to_sphere(radius: f32, distance_squared: f32) -> Vec3 {
    let r1 = rand::random::<f32>();
    let r2 = rand::random::<f32>();

    let z = 1.0 + r2 * ((1.0 - radius.powi(2) / distance_squared).sqrt() - 1.0);
    let phi = 2.0 * consts::PI * r1;

    let x = phi.cos() * (1.0 - z.powi(2)).sqrt();
    let y = phi.cos() * (1.0 - z.powi(2)).sqrt();

    Vec3::new(x, y, z)
}
