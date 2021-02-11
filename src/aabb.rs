use crate::ray::Ray;
use crate::vec3::Point3;

#[derive(Debug, Clone)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, r: Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for i in 0..3 {
            let t0 = (self.min[i] - r.orig[i] / r.dir[i]).min(self.max[i] - r.orig[i] / r.dir[i]);
            let t1 = (self.min[i] - r.orig[i] / r.dir[i]).max(self.max[i] - r.orig[i] / r.dir[i]);

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        let small = Point3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );

        let big = Point3::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );

        Self::new(small, big)
    }
}
