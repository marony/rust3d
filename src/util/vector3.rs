use std::ops::{Add, Sub, Mul, Div};
use util::point3::Point3;

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Vector3 {
    // TODO: wがないバージョンを定義したい
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector3 {
        Vector3 { x, y, z, w }
    }
    // 内積
    pub fn dot(&self, rhs: &Vector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    // ベクトルの大きさ
    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    // 単位ベクトル
    pub fn normalize(&self) -> Vector3 {
        Vector3::new(self.x / self.norm(), self.y / self.norm(), self.z / self.norm(), 1.0f64)
    }
}

// Point3<T>からVector3<T>への変換
impl From<Point3> for Vector3 {
    fn from(p: Point3) -> Vector3 {
        Vector3::new(p.x, p.y, p.z, p.w)
    }
}

/*
// FIXME: FromからIntoは自動で作られる？
impl Into<Point3> for Vector3 {
    fn into(self) -> Point3 {
        Point3::new(self.x, self.y, self.z, 1.0f64)
    }
}
*/

impl<'a> Add<&'a Vector3> for &'a Vector3 {
    type Output = Vector3;

    // ベクトル同士の加算
    fn add<'b>(self, rhs: &'b Vector3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, 1.0f64)
    }
}

impl<'a> Sub<&'a Vector3> for &'a Vector3 {
    type Output = Vector3;

    // ベクトル同士の減算
    fn sub<'b>(self, rhs: &'b Vector3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, 1.0f64)
    }
}

impl<'a> Add<&'a Point3> for &'a Vector3 {
    type Output = Vector3;

    // ベクトルと座標の加算
    fn add<'b>(self, rhs: &'b Point3) -> Vector3 {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, 1.0f64)
    }
}

impl<'a> Sub<&'a Point3> for &'a Vector3 {
    type Output = Vector3;

    // ベクトルと座標の減算
    fn sub<'b>(self, rhs: &'b Point3) -> Vector3 {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, 1.0f64)
    }
}

impl<'a> Mul<f64> for &'a Vector3 {
    type Output = Vector3;

    // スカラ値の乗算
    fn mul(self, rhs: f64) -> Vector3 {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs, 1.0f64)
    }
}

impl<'a> Div<f64> for &'a Vector3 {
    type Output = Vector3;

    // スカラ値の除算
    fn div(self, rhs: f64) -> Vector3 {
        Vector3::new(self.x / rhs, self.y / rhs, self.z / rhs, 1.0f64)
    }
}

impl<'a> Mul<&'a Vector3> for &'a Vector3 {
    type Output = Vector3;

    // 外積
    fn mul<'b>(self, rhs: &'b Vector3) -> Vector3 {
        Vector3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
            1.0f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm() {
        {
/*
import numpy as np
>>> v1 = np.array([2, 3, 4])
>>> np.linalg.norm(v1)
5.3851648071345037
*/
            // 長さ
            let a = Vector3::new(2f32, 3f32, 4f32, 1f32);
            let norm = a.norm();
            assert!(norm - 5.3851648071345037f32 < 1e-10f32);
        }
    }
    #[test]
    fn normalize() {
        {
/*
import numpy as np
>>> v1 = np.array([2, 3, 4])
>>> v1 / np.linalg.norm(v1)
array([ 0.37139068,  0.55708601,  0.74278135])
*/
            // 単位ベクトル
            let a = Vector3::new(2f32, 3f32, 4f32, 1f32);
            let b = a.normalize();
            println!("{:?}", b);
            assert!(b.x - 0.37139068f32 < 1e-5f32);
            assert!(b.y - 0.55708601f32 < 1e-5f32);
            assert!(b.z - 0.74278135f32 < 1e-5f32);
            //assert!(b.w - 0.18257419f32 < 1e-5f32);
        }
    }

    #[test]
    fn dot() {
        {
/*
import numpy as np
>>> v1 = np.array([2, 3, 4])
>>> v2 = np.array([5, 6, 7])
>>> v1.dot(v2) // np.inner(v1, v2)と同じ？？
56
*/
            // 内積
            let a = Vector3::new(2f32, 3f32, 4f32, 1f32);
            let b = Vector3::new(5f32, 6f32, 7f32, 1f32);
            let dot = a.dot(b);
            assert_eq!(56f32, dot);
        }
    }

    #[test]
    fn cross_product() {
        {
/*
import numpy as np
>>> v2 = np.array([6, 7, 8])
>>> v1 = np.array([3, 4, 5])
>>> np.cross(v1, v2) // np.outer(v1, v2)は直積
array([-3,  6, -3])
>>> np.outer(v1, v2)
array([[18, 21, 24],
       [24, 28, 32],
       [30, 35, 40]])
*/
            // 外積
            let a = Vector3::new(3f32, 4f32, 5f32, 1f32);
            let b = Vector3::new(6f32, 7f32, 8f32, 1f32);
            let cp = a * b;
            assert_eq!(Vector3::new(-3f32, 6f32, -3f32, 1f32), cp);
        }
    }
}
