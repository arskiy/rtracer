use crate::ray::Ray;
use crate::vec3::*;
use rand::prelude::*;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub w: Vec3,
    pub v: Vec3,
    pub u: Vec3,
    pub lens_radius: f32,
    pub time0: f32,
    pub time1: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        time0: f32,
        time1: f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = (vup.cross(w)).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            w,
            v,
            u,
            lens_radius,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let origin = if self.lens_radius == 0.0 {
            self.origin
        } else {
            let rd = self.lens_radius * Vec3::random_in_unit_disk();
            let offset = self.u * rd.x + self.v * rd.y;
            self.origin + offset
        };

        let time = self.time0 + rand::thread_rng().gen::<f32>() * (self.time1 - self.time0);
        let dir = self.lower_left_corner + s * self.horizontal + t * self.vertical - origin;

        Ray::new(origin, dir, time)
    }
}
