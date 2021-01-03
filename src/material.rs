use crate::ray::Ray;
use crate::vec3::*;

pub trait Material {
    fn scatter(&self, ray: Ray, normal: Vec3, p: Vec3, front_face: bool) -> Option<(Ray, Color)>;
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
    fn scatter(&self, _ray: Ray, normal: Vec3, p: Vec3, _front_face: bool) -> Option<(Ray, Color)> {
        let mut scatter_dir = normal + Vec3::random_unit_vector();

        if scatter_dir.near_zero() { scatter_dir = normal }

        let scattered = Ray::new(p, scatter_dir);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
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
    fn scatter(&self, ray: Ray, normal: Vec3, p: Vec3, _front_face: bool) -> Option<(Ray, Color)> {
        let reflected = ray.dir.unit_vector().reflect(normal);
        let scattered = Ray::new(p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;
        
        if scattered.dir.dot(normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
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
    fn scatter(&self, ray: Ray, normal: Vec3, p: Vec3, front_face: bool) -> Option<(Ray, Color)> {
        let attenuation = Color::new(0.5, 1.0, 1.0);
        let refraction_ratio = if front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = ray.dir.unit_vector();
        let cos_theta = min(-unit_dir.dot(normal), 1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || schlick(cos_theta, refraction_ratio) > rand::random::<f32>() {
            unit_dir.reflect(normal)
        } else {
            refract(unit_dir, normal, refraction_ratio)
        };

        let scattered = Ray::new(p, direction);
        Some((scattered, attenuation))
    }
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = min(-uv.dot(n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;

    r_out_perp + r_out_parallel
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 -r0) * (1.0 - cosine).powi(5)
}

fn min(n: f32, m: f32) -> f32 {
    if n > m {
        return m;
    }
    n
}
