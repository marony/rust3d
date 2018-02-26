use std::ops::{Add, Sub, Mul, Div};
use util::vector3::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Point3 {
        Point3 { x, y, z, w }
    }
}

// Vector3<T>からPoint3<T>への変換
impl From<Vector3> for Point3 {
    fn from(v: Vector3) -> Point3 {
        Point3::new(v.x, v.y, v.z, v.w)
    }
}

/*
// FIXME: FromからIntoは自動で作られる？
impl Into<Vector3> for Point3 {
    fn into(self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z, 1.0f64)
    }
}
*/

impl<'a> Add<&'a Point3> for &'a Point3 {
    type Output = Point3;

    // 座標同士の加算
    fn add<'b>(self, rhs: &'b Point3) -> Point3 {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, 1.0f64)
    }
}

impl<'a> Sub<&'a Point3> for &'a Point3 {
    type Output = Point3;

    // 座標同士の減算
    fn sub<'b>(self, rhs: &'b Point3) ->Point3 {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, 1.0f64)
    }
}

impl<'a> Add<&'a Vector3> for &'a Point3 {
    type Output = Point3;

    // ベクトルとの加算
    fn add<'b>(self, rhs: &'b Vector3) -> Point3 {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, 1.0f64)
    }
}

impl<'a> Sub<&'a Vector3> for &'a Point3 {
    type Output = Point3;

    // ベクトルとの減算
    fn sub<'b>(self, rhs: &'b Vector3) -> Point3 {
        Point3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, 1.0f64)
    }
}

impl<'a> Mul<f64> for &'a Point3 {
    type Output = Point3;

    // スカラ値の乗算
    fn mul(self, rhs: f64) -> Point3 {
        Point3::new(self.x * rhs, self.y * rhs, self.z * rhs, 1.0f64)
    }
}

impl<'a> Div<f64> for &'a Point3 {
    type Output = Point3;

    // スカラ値の除算
    fn div(self, rhs: f64) -> Point3 {
        Point3::new(self.x / rhs, self.y / rhs, self.z / rhs, 1.0f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add1() {
        {
            let a = Point3::new(1, 2, 3, 1);
            let b = Vector3::new(2, 3, 4, 1);
            let c = a + b;
            assert_eq!(Point3::new(3, 5, 7, 1), c);
        }
    }

    #[test]
    fn sub1() {
        {
            let a = Point3::new(1, 2, 3, 1);
            let b = Vector3::new(2, 3, 4, 1);
            let c = a - b;
            assert_eq!(Point3::new(-1, -1, -1, 1), c);
        }
    }

    #[test]
    fn sub2() {
        {
            let a = Point3::new(1, 2, 3, 1);
            let b = Point3::new(2, 3, 4, 1);
            let c = a - b;
            assert_eq!(Point3::new(-1, -1, -1, 1), c);
        }
    }

}