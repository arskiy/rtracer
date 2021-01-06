use crate::vec3::{Point3, Vec3};
use rand::prelude::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    pub ranfloat: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut ranfloat = vec!();

        for _ in 0..POINT_COUNT {
            ranfloat.push(Vec3::random_range(-1.0, 1.0));
        }

        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();

        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: Point3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as usize;
        let j = p.y.floor() as usize;
        let k = p.z.floor() as usize;

        let mut c: [[[Vec3; 3]; 3]; 3] = [[[Vec3::new_empty(); 3]; 3]; 3];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[
                        self.perm_x[(i + di) & 0xFF] ^
                        self.perm_y[(j + dj) & 0xFF] ^
                        self.perm_z[(k + dk) & 0xFF]
                    ];
                }
            }
        }

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * u + (1.0 - i as f32) * (1.0 - u)) *
                        (j as f32 * v + (1.0 - j as f32) * (1.0 - v)) *
                        (k as f32 * w + (1.0 - k as f32) * (1.0 - w)) * c[i][j][k].dot(weight_v);
                }
            }
        }

        accum
    }

    pub fn turb(&self, mut p: Point3, depth: usize) -> f32 {
        let mut accum = 0.0;
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            p *= 2.0;
        }

        accum.abs()
    }
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut p = vec!();

    for i in 0..POINT_COUNT {
        p.push(i);
    }

    let mut rng = rand::thread_rng();
    
    for i in (1..POINT_COUNT).rev() {
        let target = rng.gen_range(0..i);
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }

    p
}
