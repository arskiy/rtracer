use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec!(),
        }
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
}
