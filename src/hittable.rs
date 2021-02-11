use crate::aabb::AABB;
use crate::material::{Isotropic, Material};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3::{Point3, Vec3};

use std::f32;
use std::sync::Arc;

use rand::prelude::*;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl HitRecord<'_> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB>;
    fn pdf_value(&self, _orig: Point3, _v: Vec3) -> f32 {
        0.0
    }
    fn random(&self, _orig: Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn first(&self) -> Option<&Arc<dyn Hittable>> {
        self.objects.first()
    }

    pub fn push(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Arc::new(object))
    }

    pub fn push_arc(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(candidate_hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = candidate_hit.t;
                hit = Some(candidate_hit);
            }
        }

        hit
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box: AABB = AABB::new(Vec3::new_empty(), Vec3::new_empty());
        let mut first_box = true;

        for obj in &self.objects {
            if let Some(temp_box) = obj.bounding_box(time0, time1) {
                output_box = if first_box {
                    temp_box
                } else {
                    AABB::surrounding_box(&output_box, &temp_box)
                };

                first_box = false;
            } else {
                return None;
            }
        }

        Some(output_box)
    }

    fn pdf_value(&self, orig: Point3, v: Vec3) -> f32 {
        self.objects
            .iter()
            .map(|h| h.pdf_value(orig, v))
            .sum::<f32>()
            / self.objects.len() as f32
    }

    fn random(&self, orig: Vec3) -> Vec3 {
        self.objects
            .choose(&mut rand::thread_rng())
            .unwrap()
            .random(orig)
    }
}

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn new(b: impl Hittable + 'static, d: f32, t: impl Texture + 'static) -> Self {
        Self {
            boundary: Arc::new(b),
            phase_function: Arc::new(Isotropic::new(Box::new(t))),
            neg_inv_density: (-1.0 / d),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(r, f32::NEG_INFINITY, f32::INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, f32::INFINITY) {
                if rec1.t < t_min {
                    rec1.t = t_min
                }
                if rec2.t > t_max {
                    rec2.t = t_max
                }

                if rec1.t >= rec2.t {
                    return None;
                }

                if rec1.t < 0.0 {
                    rec1.t = 0.0;
                }

                let ray_length = r.dir.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rand::random::<f32>().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;
                let p = r.at(t);

                let normal = Vec3::new(1.0, 0.0, 0.0);
                let front_face = true;

                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    front_face,
                    u: 0.0,
                    v: 0.0,
                    material: &*self.phase_function,
                });
            }
        }
        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}

pub struct FlipFace {
    hit: Box<dyn Hittable>,
}

impl FlipFace {
    pub fn new(hit: impl Hittable + 'static) -> Self {
        Self { hit: Box::new(hit) }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut rec) = self.hit.hit(r, t_min, t_max) {
            rec.front_face = !rec.front_face;
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB> {
        self.hit.bounding_box(time0, time1)
    }
}
