use crate::hittable::*;
use crate::vec3::*;
use crate::ray::Ray;
use crate::aabb::AABB;
use crate::matrix4::Matrix4;

use std::f32;

pub enum Axis {
    X,
    Y,
    Z,
}

pub struct Transform {
  pub hit: Box<dyn Hittable>,
  pub transform_mat: Matrix4,
  inv_transform: Matrix4
}

impl Transform {
    pub fn new(hit: impl Hittable + 'static, transform_mat: Matrix4) -> Self {
        Self {
            hit: Box::new(hit),
            transform_mat,
            inv_transform: transform_mat.inverse().unwrap(),
        }
    }
}

impl Hittable for Transform {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let transformed_ray = Ray::new(self.transform_mat * r.orig, self.transform_mat.mul_as_33(r.dir), r.time);
        if let Some(mut hit) = self.hit.hit(&transformed_ray, t_min, t_max) {
            hit.p = self.inv_transform * r.orig;
            hit.normal = self.inv_transform.mul_as_33(hit.normal);
            return Some(hit);
        }
        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if let Some(bbox) = self.hit.bounding_box(time0, time1) {
            let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
            let mut max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let v = self.inv_transform * Vec3::new(
                            i as f32 * bbox.max.x + ((1 - i) as f32) * bbox.min.x,
                            j as f32 * bbox.max.y + ((1 - j) as f32) * bbox.min.y,
                            k as f32 * bbox.max.z + ((1 - k) as f32) * bbox.min.z);

                        for c in 0..3 {
                            if v[c] < min[c] {
                                min[c] = v[c];
                            }
                            if v[c] > max[c] {
                                max[c] = v[c];
                            }
                        }
                    }
                }
            }

            return Some(AABB::new(min, max));
        }

        None
    }
}

pub struct Translate {
    pub hit: Box<dyn Hittable>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(hit: impl Hittable + 'static, offset: Vec3) -> Self {
        Self { hit: Box::new(hit), offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray::new(r.orig - self.offset, r.dir, r.time);
        if let Some(mut rec) = self.hit.hit(&moved_r, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, rec.normal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if let Some(output_box) = self.hit.bounding_box(time0, time1) {
            Some(AABB::new(
                    output_box.min + self.offset,
                    output_box.max + self.offset))
        } else {
            None
        }
    }
}

pub struct Rotate {
    axis: Axis,
    hit: Box<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bbox: AABB,
}

impl Rotate {
    pub fn new(hit: impl Hittable + 'static, axis: Axis, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        let bbox = hit.bounding_box(0.0, 1.0).unwrap();

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.max.x + (1 - i) as f32 * bbox.min.x;
                    let y = j as f32 * bbox.max.y + (1 - j) as f32 * bbox.min.y;
                    let z = k as f32 * bbox.max.z + (1 - k) as f32 * bbox.min.z;

                    let tester = match axis {
                        Axis::X => {
                            let newy = cos_theta * y + sin_theta * z;
                            let newz = -sin_theta * y + cos_theta * z;

                            Vec3::new(x, newy, newz)
                        },
                        Axis::Y => {
                            let newx = cos_theta * x + sin_theta * z;
                            let newz = -sin_theta * x + cos_theta * z;

                            Vec3::new(newx, y, newz)
                        },
                        Axis::Z => {
                            let newx = cos_theta * x + sin_theta * y;
                            let newy = -sin_theta * x + cos_theta * y;

                            Vec3::new(newx, newy, z)
                        }
                    };

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = AABB::new(min, max);

        Self {
            hit: Box::new(hit),
            sin_theta,
            cos_theta,
            bbox,
            axis,
        }
    }
}

impl Hittable for Rotate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let (a, b) = match self.axis {
            Axis::X => (1, 2),
            Axis::Y => (0, 2),
            Axis::Z => (0, 1),
        };

        let mut origin = r.orig;
        let mut dir = r.dir;

        origin[a] = self.cos_theta * r.orig[a] - self.sin_theta * r.orig[b];
        origin[b] = self.sin_theta * r.orig[a] + self.cos_theta * r.orig[b];

        dir[a] = self.cos_theta * r.dir[a] - self.sin_theta * r.dir[b];
        dir[b] = self.sin_theta * r.dir[a] + self.cos_theta * r.dir[b];

        let rotated_r = Ray::new(origin, dir, r.time);

        if let Some(rec) = self.hit.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[a] = self.cos_theta * rec.p[a] + self.sin_theta * rec.p[b];
            p[b] = -self.sin_theta * rec.p[a] + self.cos_theta * rec.p[b];

            normal[a] = self.cos_theta * rec.normal[a] + self.sin_theta * rec.normal[b];
            normal[b] = -self.sin_theta * rec.normal[a] + self.cos_theta * rec.normal[b];

            let mut ret = HitRecord {
                p,
                normal,
                t: rec.t,
                u: rec.u,
                v: rec.v,
                front_face: true,
                material: rec.material,
            };

            ret.set_face_normal(&rotated_r, normal);

            Some(ret)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}

/*
pub struct Scale {
    pub hit: Box<dyn Hittable>,
    pub offset: Vec3,
}

impl Scale {
    pub fn new(hit: impl Hittable + 'static, offset: Vec3) -> Self {
        Self { hit: Box::new(hit), offset }
    }
}

impl Hittable for Scale { 
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut rec) = self.hit.hit(&r, t_min, t_max) {
            rec.set_face_normal(&r, rec.normal);
            Some(rec)
        } else {
            None
        }
    }
    
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if let Some(output_box) = self.hit.bounding_box(time0, time1) {
            Some(AABB::new(
                    output_box.min,
                    output_box.max * self.offset))
        } else {
            None
        }
    }
}
*/
