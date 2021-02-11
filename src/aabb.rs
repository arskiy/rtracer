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

    pub fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.dir[a];
            let t0 = (self.min[a] - ray.orig[a]) * inv_d;
            let t1 = (self.max[a] - ray.orig[a]) * inv_d;

            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return false
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
