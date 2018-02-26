use std::ops::{Add, Sub, Mul};
use util::vector3::Vector3;

// 以下の順序で要素を渡す
// |  0 |  1 |  2 |  3 |
// |  4 |  5 |  6 |  7 |
// |  8 |  9 | 10 | 11 |
// | 12 | 13 | 14 | 15 |
#[derive(Debug, Copy, Clone)]
pub struct Matrix4 {
    pub xs: [f64; 16]
}

impl Matrix4 {
    // 配列から作成
    pub fn new(xs: [f64; 16]) -> Matrix4 {
        Matrix4 { xs }
    }
    // Vecから作成
    pub fn new_from_vec(xs: Vec<f64>) -> Matrix4 {
        let mut ys = [0.0f64; 16];
        for i in 0..xs.len() {
            ys[i] = xs[i];
        }
        Matrix4::new(ys)
    }
}

impl<'a> Add<&'a Matrix4> for &'a Matrix4 {
    type Output = Matrix4;

    // 行列同士の加算
    fn add<'b>(self, rhs: &'b Matrix4) -> Matrix4 {
        let v = self.xs
            .iter()
            .zip(rhs.xs.iter())
            .map(|(&x, &y)| x + y)
            .collect::<Vec<_>>();
        Matrix4::new_from_vec(v)
    }
}

impl<'a> Sub<&'a Matrix4> for &'a Matrix4 {
    type Output = Matrix4;

    // 行列同士の減算
    fn sub<'b>(self, rhs: &'b Matrix4) -> Matrix4 {
        let v = self.xs
            .iter()
            .zip(rhs.xs.iter())
            .map(|(&x, &y)| x - y)
            .collect::<Vec<_>>();
        Matrix4::new_from_vec(v)
    }
}

impl<'a> Mul<f64> for &'a Matrix4 {
    type Output = Matrix4;

    // スカラ値との乗算
    fn mul(self, rhs: f64) -> Matrix4 {
        let v = self.xs
            .iter()
            .map(|&x| x * rhs)
            .collect::<Vec<_>>();
        Matrix4::new_from_vec(v)
    }
}

impl<'a> Mul<&'a Vector3> for &'a Matrix4 {
    type Output = Vector3;

    //ベクトルとの積(ドット積)
    fn mul<'b>(self, v: &'b Vector3) -> Vector3 {
        Vector3::new(
            self.xs[0] * v.x + self.xs[1] * v.y + self.xs[2] * v.z + self.xs[3] * v.w,
            self.xs[4] * v.x + self.xs[5] * v.y + self.xs[6] * v.z + self.xs[7] * v.w,
            self.xs[8] * v.x + self.xs[9] * v.y + self.xs[10] * v.z + self.xs[11] * v.w,
            self.xs[12] * v.x + self.xs[13] * v.y + self.xs[14] * v.z + self.xs[15] * v.w
        )
    }
}

impl<'a> Mul<&'a Matrix4> for &'a Matrix4 {
    type Output = Matrix4;

    // 行列との積(ドット積)
    fn mul<'b>(self, mat: &'b Matrix4) -> Matrix4 {
        let mut xs: [f64; 16] = [0.0f64; 16];
        for y in 0..4 {
            for x in 0..4 {
                xs[y * 4 + x] = self.xs[y * 4 + 0] * mat.xs[x + 0] +
                    self.xs[y * 4 + 1] * mat.xs[x + 4] +
                    self.xs[y * 4 + 2] * mat.xs[x + 8] +
                    self.xs[y * 4 + 3] * mat.xs[x + 12];
            }
        }
        Matrix4::new(xs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        {
            // 行列の加算
            let a = Matrix4::new(
                [1f64, 2f64, 3f64, 4f64,
                 5f64, 6f64, 7f64, 8f64,
                 9f64, 10f64, 11f64, 12f64,
                 13f64, 14f64, 15f64, 16f64]);
            let b = Matrix4::new(
                [15f64, 14f64, 13f64, 12f64,
                    11f64, 10f64, 9f64, 8f64,
                    7f64, 6f64, 5f64, 4f64,
                    3f64, 2f64, 1f64, 0f64]);
            let c = a + b;
            let d = Matrix4::new(
                [16f64, 16f64, 16f64, 16f64,
                    16f64, 16f64, 16f64, 16f64,
                    16f64, 16f64, 16f64, 16f64,
                    16f64, 16f64, 16f64, 16f64]);
            assert_eq!(c, d);
        }
    }

    #[test]
    fn sub() {
        {
            // 行列の減算
            let a = Matrix4::new(
                [1f64, 2f64, 3f64, 4f64,
                    5f64, 6f64, 7f64, 8f64,
                    9f64, 10f64, 11f64, 12f64,
                    13f64, 14f64, 15f64, 16f64]);
            let b = Matrix4::new(
                [15f64, 14f64, 13f64, 12f64,
                    11f64, 10f64, 9f64, 8f64,
                    7f64, 6f64, 5f64, 4f64,
                    3f64, 2f64, 1f64, 0f64]);
            let c = a - b;
            let d = Matrix4::new(
                [-14f64, -12f64, -10f64, -8f64,
                    -6f64, -4f64, -2f64, 0f64,
                    2f64, 4f64, 6f64, 8f64,
                    10f64, 12f64, 14f64, 16f64]);
            assert_eq!(c, d);
        }
    }

    #[test]
    fn mul() {
        {
            // スカラ値との乗算
            let a = Matrix4::new(
                [1f64, 2f64, 3f64, 4f64,
                    5f64, 6f64, 7f64, 8f64,
                    9f64, 10f64, 11f64, 12f64,
                    13f64, 14f64, 15f64, 16f64]);
            let b = a * 3f64;
            let c = Matrix4::new(
                [3f64, 6f64, 9f64, 12f64,
                    15f64, 18f64, 21f64, 24f64,
                    27f64, 30f64, 33f64, 36f64,
                    39f64, 42f64, 45f64, 48f64]);
            assert_eq!(b, c);
        }
    }
    #[test]
    fn vecmul() {
        {
/*
import numpy as np
>>> m1 = np.matrix([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]])
>>> v1 = np.array([1, 2, 3, 4])
>>> m1.dot(v1)
matrix([[ 30,  70, 110, 150]])
*/
            // ベクトルとの積(ドット積)
            let a = Matrix4::new(
                [1f64, 2f64, 3f64, 4f64,
                    5f64, 6f64, 7f64, 8f64,
                    9f64, 10f64, 11f64, 12f64,
                    13f64, 14f64, 15f64, 16f64]);
            let b = Vector3::new(1f64, 2f64, 3f64, 4f64);
            let c = a * b;
            let d = Vector3::new(30f64, 70f64, 110f64, 150f64);
            assert_eq!(c, d);
        }
    }
    #[test]
    fn matmul() {
        {
/*
import numpy as np
>>> m1 = np.matrix([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]])
>>> m2 = np.matrix([[2, 3, 4, 5], [6, 7, 8, 9], [10, 11, 12, 13], [14, 15, 16, 17]])
>>> m1 * m2 or m1.dot(m2)
matrix([[100, 110, 120, 130],
        [228, 254, 280, 306],
        [356, 398, 440, 482],
        [484, 542, 600, 658]])
*/
            // 行列との積(ドット積)
            let a = Matrix4::new(
                [1f64, 2f64, 3f64, 4f64,
                    5f64, 6f64, 7f64, 8f64,
                    9f64, 10f64, 11f64, 12f64,
                    13f64, 14f64, 15f64, 16f64]);
            let b = Matrix4::new(
                [2f64, 3f64, 4f64, 5f64,
                    6f64, 7f64, 8f64, 9f64,
                    10f64, 11f64, 12f64, 13f64,
                    14f64, 15f64, 16f64, 17f64]);
            let c = a * b;
            let d = Matrix4::new(
                [100f64, 110f64, 120f64, 130f64,
                228f64, 254f64, 280f64, 306f64,
                356f64, 398f64, 440f64, 482f64,
                484f64, 542f64, 600f64, 658f64]);
            assert_eq!(c, d);
        }
    }
}