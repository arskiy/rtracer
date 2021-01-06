use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::*;
use crate::texture::*;

pub trait Material {
    fn scatter(&self, ray: Ray, hr: &HitRecord) -> Option<(Ray, Color)>;
    fn emitted(&self, u: f32, v: f32, p: Point3) -> Color { Color::new_empty() }
}

pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo: Box::new(SolidColorTexture::new(albedo)) }
    }

    pub fn new_texture(albedo: Box<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, hr: &HitRecord) -> Option<(Ray, Color)> {
        let scatter_dir = hr.p + hr.normal + Vec3::random_in_unit_sphere();

        let scattered = Ray::new(hr.p, scatter_dir - hr.p, ray.time);
        let attenuation = self.albedo.value(hr.u, hr.v, hr.p);
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
    fn scatter(&self, ray: Ray, hr: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(ray.dir.unit_vector(), hr.normal);
        let scattered = Ray::new(
            hr.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            ray.time,
        );
        let attenuation = self.albedo;

        if scattered.dir.dot(hr.normal) > 0.0 {
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
    fn scatter(&self, ray: Ray, hr: &HitRecord) -> Option<(Ray, Color)> {
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
                let scattered = Ray::new(hr.p, -refraction, ray.time);
                return Some((scattered, attenuation));
            }
        }

        let reflected = reflect(ray.dir, hr.normal);
        let scattered = Ray::new(hr.p, reflected, ray.time);
        Some((scattered, attenuation))
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

pub struct DiffuseLight {
    emit: Box<Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Box<Texture>) -> Self {
        Self { emit }
    }

    pub fn new_color(emit: Color) -> Self {
        Self { emit: Box::new(SolidColorTexture::new(emit)) }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, ray: Ray, hr: &HitRecord) -> Option<(Ray, Color)> { None }

    fn emitted(&self, u: f32, v: f32, p: Point3) -> Color { self.emit.value(u, v, p) }
}
