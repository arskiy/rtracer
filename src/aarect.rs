use crate::material::Material;
use crate::hittable::*;
use crate::aabb::AABB;
use crate::ray::Ray;
use crate::vec3::*;

pub struct XYRect<M: Material>{
    pub material: M,

    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32
}

impl<M: Material> XYRect<M> {
    pub fn new(material: M, x0: f32, x1: f32, y0: f32, y1: f32, k: f32) -> Self {
        Self { material, x0, x1, y0, y1, k }
    }
}

impl<M: Sync + Material> Hittable for XYRect<M> { 
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.orig.z) / r.dir.z;

        if t < t_min || t > t_max { return None; }

        let x = r.orig.x + t * r.dir.x;
        let y = r.orig.y + t * r.dir.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);

        let p = r.at(t);

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        
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

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(Point3::new(self.x0, self.y0, self.k - 0.0001), Point3::new(self.x1, self.y1, self.k + 0.0001)))
    }
}


pub struct XZRect<M: Material>{
    pub material: M,

    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32
}

impl<M: Material> XZRect<M> {
    pub fn new(material: M, x0: f32, x1: f32, z0: f32, z1: f32, k: f32) -> Self {
        Self { material, x0, x1, z0, z1, k }
    }
}

impl<M: Sync + Material> Hittable for XZRect<M> { 
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.orig.y) / r.dir.y;

        if t < t_min || t > t_max { return None; }

        let x = r.orig.x + t * r.dir.x;
        let z = r.orig.z + t * r.dir.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        let p = r.at(t);

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        
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

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(Point3::new(self.x0, self.k - 0.0001, self.z0), Point3::new(self.x1, self.k + 0.0001, self.z1)))
    }
}

pub struct YZRect<M: Material>{
    pub material: M,

    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32
}

impl<M: Material> YZRect<M> {
    pub fn new(material: M, y0: f32, y1: f32, z0: f32, z1: f32, k: f32) -> Self {
        Self { material, y0, y1, z0, z1, k }
    }
}

impl<M: Sync + Material> Hittable for YZRect<M> { 
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.orig.z) / r.dir.z;

        if t < t_min || t > t_max { return None; }

        let y = r.orig.y + t * r.dir.y;
        let z = r.orig.z + t * r.dir.z;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);

        let p = r.at(t);

        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        
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

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        Some(AABB::new(Point3::new(self.k - 0.0001, self.y0, self.z0), Point3::new(self.k + 0.0001, self.y1, self.z1)))
    }
}
