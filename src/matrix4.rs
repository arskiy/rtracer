use std::f32;
use std::ops;
use crate::vec3::*;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4(pub [[f32; 4]; 4]);

impl Matrix4 {
    pub fn identity() -> Self {
        Self([
             [1.0, 0.0, 0.0, 0.0],
             [0.0, 1.0, 0.0, 0.0],
             [0.0, 0.0, 1.0, 0.0],
             [0.0, 0.0, 0.0, 1.0]])
    }

    pub fn scale(offset: Vec3) -> Self {
        Self([
             [offset.x, 0.0, 0.0, 0.0],
             [0.0, offset.y, 0.0, 0.0],
             [0.0, 0.0, offset.z, 0.0],
             [0.0, 0.0, 0.0, 1.0]])
    }

    pub fn translate(offset: Vec3) -> Self {
        Self([
             [1.0, 0.0, 0.0, -offset.x],
             [0.0, 1.0, 0.0, -offset.y],
             [0.0, 0.0, 1.0, -offset.z],
             [0.0, 0.0, 0.0, 1.0]])

    }

    pub fn rotate(angle: f32, axis: Vec3) -> Self {
        let norm = axis.unit_vector();
        let rad = angle.to_radians();
        let s = rad.sin();
        let c = rad.cos();
        let t = 1.0 - c;

        Self([
             [t * norm.x * norm.x + c, t * norm.x * norm.y + s * norm.z, t * norm.x * norm.z - s * norm.y, 0.0],
             [t * norm.y * norm.x - s * norm.z, t * norm.y * norm.y + c, t * norm.y * norm.z + s * norm.x, 0.0],
             [t * norm.z * norm.x + s * norm.y, t * norm.z * norm.y - s * norm.x, t * norm.z * norm.z + c, 0.0],
             [0.0, 0.0, 0.0, 1.0]])
    }

    pub fn inverse(&self) -> Option<Self> {
        let mut dst = Self::identity();
        let mut temp = &mut self.clone();

        for i in 0..=3 {
            let mut diag = temp.0[i][i];
            let mut ind = i;
            for j in (i + 1)..=3 {
                if temp.0[i][j].abs() > diag.abs() {
                    ind = j;
                    diag = temp.0[i][j];
                }
            }

            if ind != i {
                for j in 0..=3 {
                    dst.0[j].swap(i, ind);
                    temp.0[j].swap(i, ind);
                }
            }

            if diag.abs() < 1e-6 {
                return None;
            }

            let inv_diag = 1.0 / diag;
            for j in 0..=3 {
                dst.0[j][i] *= inv_diag;
                temp.0[j][i] *= inv_diag;
            }

            for j in 0..=3 {
                if j == i { continue; }

                diag = temp.0[i][j];
                for k in 0..=3 {
                    temp.0[k][j] -= temp.0[k][i] * diag;
                    dst.0[k][j] -= dst.0[k][i] * diag;
                }
            }
        }

        Some(dst)
    }

    pub fn mul_as_33(&self, other: Vec3) -> Vec3 {
        Vec3::new(self.0[0][0] * other.x + self.0[0][1] * other.y + self.0[0][2] * other.z,
            self.0[1][0] * other.x + self.0[1][1] * other.y + self.0[1][2] * other.z,
            self.0[2][0] * other.x + self.0[2][1] * other.y + self.0[2][2] * other.z)
    }
}

impl ops::Mul<Vec3> for Matrix4 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.0[0][0] * other.x + self.0[0][1] * other.y + self.0[0][2] * other.z + self.0[0][3],
            self.0[1][0] * other.x + self.0[1][1] * other.y + self.0[1][2] * other.z + self.0[1][3],
            self.0[2][0] * other.x + self.0[2][1] * other.y + self.0[2][2] * other.z + self.0[2][3]) 
    }
}

impl ops::Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut r = Matrix4([[0.0; 4]; 4]);
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    r.0[i][j] += self.0[i][k] * other.0[k][j];
                }
            }
        }
        r
    }
}
