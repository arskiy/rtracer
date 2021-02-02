use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::*;
use crate::texture::*;
use crate::pdf::*;

use std::sync::Arc;
use std::f32::consts::PI;

pub trait Material: Sync + Send {
    fn scatter(&self, _ray: &Ray, _hr: &HitRecord) -> Option<ReflectionRecord> { None }
    fn emitted(&self, _ray: &Ray, _hr: &HitRecord) -> Color { Color::new_empty() }
    fn scattering_pdf(&self, _ray: &Ray, _hr: &HitRecord, _scattered: &Ray) -> f32 { 0.0 }
}

#[derive(Clone)]
pub struct Lambertian<A: Texture> {
    pub albedo: A,
}

impl<A: Texture> Lambertian<A> {
    pub fn new(albedo: A) -> Self {
        Self { albedo }
    }
}

pub enum ReflectionRecord {
    Specular { specular_ray: Ray, attenuation: Vec3 },
    Scatter { pdf: Arc<dyn PDF>, attenuation: Vec3 }
}

impl<A: Texture> Material for Lambertian<A> {
    fn scatter(&self, _ray: &Ray, hr: &HitRecord) -> Option<ReflectionRecord> {
        Some(ReflectionRecord::Scatter {
            pdf: Arc::new(CosinePDF::new(hr.normal)),
            attenuation: self.albedo.value(hr.u, hr.v, hr.p),
        })
    }

    fn scattering_pdf(&self, _ray: &Ray, hr: &HitRecord, scattered: &Ray) -> f32 { 
        // if cosine < 0, return 0
        let cosine = hr.normal.dot(scattered.dir.unit_vector()).max(0.0);
        cosine / PI
    }
}

#[derive(Clone)]
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
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<ReflectionRecord> {
        let mut reflected = reflect(ray.dir.unit_vector(), hr.normal);
        if self.fuzz > 0.0 { reflected += self.fuzz * Vec3::random_in_unit_sphere() };

        let attenuation = self.albedo;

        if reflected.dot(hr.normal) > 0.0 {
            Some(ReflectionRecord::Specular {
                specular_ray: Ray::new(hr.p, reflected, ray.time),
                attenuation,
            })
        } else {
            None
        }
    }
}

#[derive(Clone)]
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
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<ReflectionRecord> {
        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let cosine: f32;
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        if ray.dir.dot(hr.normal) > 0.0 {
            outward_normal = -hr.normal;
            ni_over_nt = self.ir;
            cosine = self.ir * ray.dir.dot(hr.normal) / ray.dir.length();
        } else {
            outward_normal = hr.normal;
            ni_over_nt = 1.0 / self.ir;
            cosine = -ray.dir.dot(hr.normal) / ray.dir.length();
        }

        if let Some(refraction) = refract(ray.dir, outward_normal, ni_over_nt) {
            if rand::random::<f32>() > schlick(cosine, self.ir) {
                let refraction = Ray::new(hr.p, -refraction, ray.time);
                return Some(ReflectionRecord::Specular {
                    specular_ray: refraction,
                    attenuation,
                });
            }
        }

        let reflected = reflect(ray.dir, hr.normal);
        if reflected.is_nan() { panic!(format!("reflected: {:?}", reflected)); }

        let reflected = Ray::new(hr.p, reflected, ray.time);

        Some(ReflectionRecord::Specular {
            specular_ray: reflected,
            attenuation,
        })
    }
}

pub fn reflect(m: Vec3, n: Vec3) -> Vec3 {
    m - 2.0 * m.dot(n) * n
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt.powi(2) * (1.0 - dt.powi(2));
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

#[derive(Clone)]
pub struct DiffuseLight<A: Texture> {
    emit: A,
}

impl<A: Texture> DiffuseLight<A> {
    pub fn new(emit: A) -> Self {
        Self { emit }
    }
}

impl<A: Texture> Material for DiffuseLight<A> {
    fn emitted(&self, ray: &Ray, hr: &HitRecord) -> Color { 
        if hr.normal.dot(ray.dir) < 0.0 { self.emit.value(hr.u, hr.v, hr.p) } else { Color::new_empty() }
    }
}

/*
#[derive(Clone)]
pub struct Isotropic<A: Texture> {
    albedo: A,
}

impl<A: Texture> Isotropic<A> {
    pub fn new(albedo: A) -> Self {
        Self { albedo }
    }
}

impl<A: Texture> Material for Isotropic<A> {
    fn scatter(&self, ray: Ray, hr: &HitRecord) -> Option<(Ray, Color, f32)> { 
        let scattered = Ray::new(hr.p, Vec3::random_in_unit_sphere(), ray.time);
        let attenuation = self.albedo.value(hr.u, hr.v, hr.p);
        let pdf = 0.5 / PI;
        Some((scattered, attenuation, pdf))
    }
}
*/

pub struct Isotropic {
    albedo: Box<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<ReflectionRecord> { 
        let specular_ray = Ray::new(hr.p, Vec3::random_in_unit_sphere(), ray.time);
        let attenuation = self.albedo.value(hr.u, hr.v, hr.p);

        Some(ReflectionRecord::Specular {
            specular_ray, attenuation
        })
    }
}
