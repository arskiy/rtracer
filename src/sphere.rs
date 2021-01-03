use crate::hittable::{Hittable, HitRecord};
use crate::vec3::*;
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere<M: Material> {
    pub center: Point3,
    pub radius: f32,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f32, material: M) -> Self {
        Self { center, radius, material }
    }
}

impl<M: Sync + Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return None; }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut hr = HitRecord {
            normal: Vec3::new_empty(),
            p: r.at(root),
            t: root,
            front_face: false,
            material: &self.material,
        };

        let outward_normal = (hr.p - self.center) / self.radius;
        hr.set_face_normal(r, outward_normal);

        Some(hr)
    }
}
