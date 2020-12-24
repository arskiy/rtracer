use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::*;

pub trait Material {
    fn scatter(&self, r_in: Ray, normal: Vec3, p: Vec3, front_face: bool, attenuation: &mut Color, scattered: &mut Ray) -> bool;
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
    fn scatter(&self, r_in: Ray, normal: Vec3, p: Vec3, front_face: bool, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_dir = normal + Vec3::random_unit_vector();

        if scatter_dir.near_zero() { scatter_dir = normal }

        *scattered = Ray::new(p, scatter_dir);
        *attenuation = self.albedo;
        true
    }
}


pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, f: f32) -> Self {
        Self { 
            albedo,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, normal: Vec3, p: Vec3, front_face: bool, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = r_in.dir.unit_vector().reflect(normal);
        *scattered = Ray::new(p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;
        
        scattered.dir.dot(normal) > 0.0
    }
}

pub struct Dieletric {
    ir: f32,
}

impl Dieletric {
    pub fn new(index_of_refraction: f32) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
}

impl Material for Dieletric {
    fn scatter(&self, r_in: Ray, normal: Vec3, p: Vec3, front_face: bool, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if front_face { 1.0 / self.ir } else { self.ir };

        let unit_dir = r_in.dir.unit_vector();
        let refracted = unit_dir.refract(normal, refraction_ratio);

        *scattered = Ray::new(p, refracted);
        true
    }
}
