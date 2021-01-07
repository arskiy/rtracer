use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::aabb::AABB;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
    pub material: &'a Box<dyn Material>,
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

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AABB>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn push(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object)
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
                    AABB::surrounding_box(output_box, temp_box)
                };

                first_box = false;
            } else {
                return None;
            }
        }

        Some(output_box)
    }
}
