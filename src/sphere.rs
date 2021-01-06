use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::*;
use crate::aabb::AABB;

use std::f32::consts::{FRAC_PI_2, PI};

pub struct Sphere<M: Material> {
    pub center: Point3,
    pub radius: f32,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Point3, radius: f32, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: Sync + Material> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
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
            u: 0.0,
            v: 0.0,
            front_face: false,
            material: &self.material,
        };

        let outward_normal = (hr.p - self.center) / self.radius;
        let (u, v) = get_sphere_uv(outward_normal);
        hr.u = u;
        hr.v = v;
        hr.set_face_normal(r, outward_normal);

        Some(hr)
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

pub struct MovingSphere<M: Material> {
    pub center0: Point3,
    pub center1: Point3,
    pub radius: f32,
    pub material: M,

    pub time0: f32,
    pub time1: f32,
}

impl<M: Material> MovingSphere<M> {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: M,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    fn calc_time(&self, time: f32) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl<M: Sync + Material> Hittable for MovingSphere<M> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.orig - self.calc_time(r.time);
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
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
            u: 0.0,
            v: 0.0,
            front_face: false,
            material: &self.material,
        };

        let outward_normal = (hr.p - self.calc_time(r.time)) / self.radius;
        let (u, v) = get_sphere_uv(outward_normal);
        hr.u = u;
        hr.v = v;
        hr.set_face_normal(r, outward_normal);

        Some(hr)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        let box0 = AABB::new(
            self.calc_time(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.calc_time(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );

        let box1 = AABB::new(
            self.calc_time(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.calc_time(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );

        Some(AABB::surrounding_box(box0, box1))
    }
}


fn get_sphere_uv(p: Point3) -> (f32, f32) {
    let theta = p.y.asin();
    let phi = p.z.atan2(p.x);

    let u = 1.0 - (phi + PI) / (2.0 * PI);
    let v = (theta + FRAC_PI_2) / PI;

    (u, v)
}
