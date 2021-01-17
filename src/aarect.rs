use crate::material::Material;
use crate::hittable::*;
use crate::aabb::AABB;
use crate::ray::Ray;
use crate::vec3::*;

use rand::prelude::*;

#[derive(Clone, Debug)]
pub enum Plane {
    XY,
    XZ,
    YZ,
}

pub struct AARect<M: Material>{
    pub plane: Plane,
    pub material: M,

    pub a0: f32,
    pub a1: f32,
    pub b0: f32,
    pub b1: f32,
    pub k: f32
}

impl<M: Material> AARect<M> {
    pub fn new(plane: Plane, material: M, a0: f32, a1: f32, b0: f32, b1: f32, k: f32) -> Self {
        Self { plane, material, a0, a1, b0, b1, k }
    }
}

impl<M: Sync + Material + 'static> Hittable for AARect<M> { 
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (k_axis, a_axis, b_axis, outward_normal) = match &self.plane {
            Plane::XY => (2, 0, 1, Vec3::new(0.0, 0.0, 1.0)),
            Plane::XZ => (1, 0, 2, Vec3::new(0.0, 1.0, 0.0)),
            Plane::YZ => (0, 1, 2, Vec3::new(1.0, 0.0, 0.0)),
        };

        let t = (self.k - r.orig[k_axis]) / r.dir[k_axis];

        if t < t_min || t > t_max { return None; }

        let a = r.orig[a_axis] + t * r.dir[a_axis];
        let b = r.orig[b_axis] + t * r.dir[b_axis];

        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            return None;
        }

        let u = (a - self.a0) / (self.a1 - self.a0);
        let v = (b - self.b0) / (self.b1 - self.b0);

        let p = r.at(t);

        let mut hr = HitRecord {
            normal: Vec3::new_empty(),
            p,
            t,
            u,
            v,
            front_face: false,
            material: &self.material,
        };

        hr.set_face_normal(r, outward_normal);

        Some(hr)
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(AABB::new(Point3::new(self.a0, self.b0, self.k - 0.0001), Point3::new(self.a1, self.b1, self.k + 0.0001)))
    }

    fn pdf_value(&self, orig: Point3, v: Vec3) -> f32 { 
        if let Some(hit) = self.hit(&Ray::new(orig, v, 0.0), 0.001, std::f32::INFINITY) {
            let area = (self.a1 - self.a0) * (self.b1 - self.b0);

            let distance_squared = hit.t.powi(2) * v.length_squared();

            let cosine = v.dot(hit.normal).abs() / v.length();

            return distance_squared / (cosine * area);
        }

        0.0
    }

    fn random(&self, orig: Vec3) -> Vec3 { 
        let mut rng = rand::thread_rng();
        let random_point = match &self.plane {
            Plane::XY => Point3::new(rng.gen_range(self.a0..self.a1), rng.gen_range(self.b0..self.b1), self.k),
            Plane::XZ => Point3::new(rng.gen_range(self.a0..self.a1), self.k, rng.gen_range(self.b0..self.b1)),
            Plane::YZ => Point3::new(self.k, rng.gen_range(self.a0..self.a1), rng.gen_range(self.b0..self.b1)),
        };

        random_point - orig
    }
}

pub struct RectBox {
    box_min: Point3,
    box_max: Point3,
    sides: HittableList,
}

impl RectBox {
    pub fn new<M: Material + Clone + 'static>(p0: Point3, p1: Point3, material: M) -> Self {
        let box_min = p0;
        let box_max = p1;
        let mut sides = HittableList::new();
        sides.push(AARect::new(Plane::XY, material.clone(), p0.x, p1.x, p0.y, p1.y, p1.z));
        sides.push(AARect::new(Plane::XY, material.clone(), p0.x, p1.x, p0.y, p1.y, p0.z));
        sides.push(AARect::new(Plane::XZ, material.clone(), p0.x, p1.x, p0.z, p1.z, p1.y));
        sides.push(AARect::new(Plane::XZ, material.clone(), p0.x, p1.x, p0.z, p1.z, p0.y));
        sides.push(AARect::new(Plane::YZ, material.clone(), p0.y, p1.y, p0.z, p1.z, p1.x));
        sides.push(AARect::new(Plane::YZ, material, p0.y, p1.y, p0.z, p1.z, p0.x));
        Self {
            box_min,
            box_max,
            sides,
        }
    }
}

impl Hittable for RectBox {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
}
