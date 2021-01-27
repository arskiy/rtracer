use crate::material::Material;
use crate::hittable::*;
use crate::aabb::AABB;
use crate::ray::Ray;
use crate::vec3::*;

// https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/ray-triangle-intersection-geometric-solution

pub struct Triangle<M: Material> {
    pub material: M,

    pub v0: Point3,
    pub v1: Point3,
    pub v2: Point3,

    // precompute normal to make hit() faster
    normal: Point3,
}

impl<M: Material> Triangle<M> {
    pub fn new(material: M, v0: Point3, v1: Point3, v2: Point3) -> Self {
        // compute plane's normal
        let v0v1 = v1 - v0;
        let v0v2 = v2 - v0;
        let normal = v0v1.cross(v0v2).unit_vector();

        Self {
            material,
            v0,
            v1,
            v2,
            normal,
        }
    }
}

impl<M: Sync + Send + Material + 'static> Hittable for Triangle<M> { 
    fn hit(&self, r: &Ray, _t_min: f32, _t_max: f32) -> Option<HitRecord> {
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;

        let p = r.dir.cross(v0v2);
        let det = v0v1.dot(p);

        // back-face culling
        if det < 0.0001 { return None; }

        let inv_det = 1.0 / det;

        let tvec = r.orig - self.v0;
        let u = tvec.dot(p) * inv_det;
        if u < 0.0 || u > 1.0 { return None; }

        let q = tvec.cross(v0v1);
        let v = r.dir.dot(q) * inv_det;
        if v < 0.0 || u + v > 1.0 { return None; }

        let t = v0v2.dot(q) * inv_det;

        Some(HitRecord {
            normal: self.normal,
            p,
            t,
            u: 0.0,
            v: 0.0,
            front_face: true,
            material: &self.material,
        })
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(AABB::new(self.v0, self.v1))
    }
}
