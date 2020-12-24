use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::*;

pub trait Material {
    fn scatter(&self, r_in: Ray, normal: Vec3, p: Vec3, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, normal: Vec3, p: Vec3, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_dir = normal + Vec3::random_unit_vector();

        if scatter_dir.near_zero() { scatter_dir = normal }

        *scattered = Ray::new(p, scatter_dir);
        *attenuation = self.albedo;
        true
    }
}


pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, normal: Vec3, p: Vec3, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = r_in.dir.unit_vector().reflect(normal);
        *scattered = Ray::new(p, reflected);
        *attenuation = self.albedo;
        
        scattered.dir.dot(normal) > 0.0
    }
}
