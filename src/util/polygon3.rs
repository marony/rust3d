extern crate gio;
extern crate gtk;
extern crate cairo;

use gio::prelude::*;
use gtk::prelude::*;

use util::point3::Point3;
use util::vector3::Vector3;
use util::matrix4::Matrix4;
use util::color::Color;

#[derive(Debug, Copy, Clone)]
pub struct Polygon3 {
    pub p1: Point3,
    pub p2: Point3,
    pub p3: Point3,
}

impl Polygon3 {
    pub fn draw(&self, cr: &cairo::Context, color: &Color) -> () {
        // 描画座標
//        println!("{:?}", self);
        // 面
        cr.set_source_rgb(
            (color.r as f64) / 256.0f64,
            (color.g as f64) / 256.0f64,
            (color.b as f64) / 256.0f64);
        cr.move_to(self.p1.x, self.p1.y);
        cr.line_to(self.p2.x, self.p2.y);
        cr.line_to(self.p3.x, self.p3.y);
        cr.line_to(self.p1.x, self.p1.y);
        cr.stroke_preserve();
        cr.fill();
    }

    pub fn new(p1: &Point3, p2: &Point3, p3: &Point3) -> Polygon3 {
        Polygon3 { p1: *p1, p2: *p2, p3: *p3 }
    }

    // 法線ベクトル
    pub fn normal(&self) -> Vector3 {
        let v1 = Vector3::from(self.p1);
        let a1 = Vector3::from(&(self.p2) - &v1);
        let a2 = Vector3::from(&(self.p3) - &v1);
        &a1 * &a2
    }

    // アフィン変換
    pub fn affin(&self, mat: &Matrix4) -> Polygon3 {
        let p1 = mat * &Vector3::from(self.p1);
        let p11 = Into::<Point3>::into(p1);
        let p2 = mat * &Vector3::from(self.p2);
        let p22 = Into::<Point3>::into(p2);
        let p3 = mat * &Vector3::from(self.p3);
        let p33 = Into::<Point3>::into(p3);
        Polygon3::new(&p11, &p22, &p33)
    }

    // 移動
    // | 1 | 0 | 0 | tx |
    // | 0 | 1 | 0 | ty |
    // | 0 | 0 | 1 | tz |
    // | 0 | 0 | 0 | 1  |
    pub fn move_(&self, v: &Vector3) -> Polygon3 {
        let mat = Matrix4::new(
            [1.0f64, 0.0f64, 0.0f64, v.x,
                0.0f64, 1.0f64, 0.0f64, v.y,
                0.0f64, 0.0f64, 1.0f64, v.z,
                0.0f64, 0.0f64, 0.0f64, 1.0f64]
        );
        self.affin(&mat)
    }
    // 拡大・縮小、反転
    // | sx | 0  | 0  | 0 |
    // | 0  | sy | 0  | 0 |
    // | 0  | 0  | sz | 0 |
    // | 0  | 0  | 0  | 1 |
    pub fn scale(&self, v: &Vector3) -> Polygon3 {
        let mat = Matrix4::new(
            [v.x, 0.0f64, 0.0f64, 0.0f64,
                0.0f64, v.y, 0.0f64, 0.0f64,
                0.0f64, 0.0f64, v.z, 0.0f64,
                0.0f64, 0.0f64, 0.0f64, 1.0f64]
        );
        self.affin(&mat)
    }
    // X軸周りに回転
    // | 1 | 0     | 0      | 0 |
    // | 0 | cos r | -sin r | 0 |
    // | 0 | sin r | cos r  | 0 |
    // | 0 | 0     | 0      | 1 |
    pub fn rotate_x(&self, r: f64) -> Polygon3 {
        //let a: f64 = cos::(r);
        let mat = Matrix4::new(
            [1.0f64, 0.0f64, 0.0f64, 0.0f64,
                0.0f64, r.cos(), -1.0f64 * r.sin(), 0.0f64,
                0.0f64, r.sin(), r.cos(), 0.0f64,
                0.0f64, 0.0f64, 0.0f64, 1.0f64]
        );
        self.affin(&mat)
    }
    // Y軸周りに回転
    // | cos r  | 0 | sin r | 0 |
    // | 0      | 1 | 0     | 0 |
    // | -sin r | 0 | cos r | 0 |
    // | 0      | 0 | 0     | 1 |
    pub fn rotate_y(&self, r: f64) -> Polygon3 {
        let mat = Matrix4::new(
            [r.cos(), 0.0f64, r.sin(), 0.0f64,
                0.0f64, 1.0f64, 0.0f64, 0.0f64,
                -1.0f64 * r.sin(), 0.0f64, r.cos(), 0.0f64,
                0.0f64, 0.0f64, 0.0f64, 1.0f64]
        );
        self.affin(&mat)
    }
    // Z軸周りに回転
    // | cos r | -sin r | 0 | 0 |
    // | sin r | cos r  | 0 | 0 |
    // | 0     | 0      | 1 | 0 |
    // | 0     | 0      | 0 | 1 |
    pub fn rotate_z(&self, r: f64) -> Polygon3 {
        let mat = Matrix4::new(
            [r.cos(), -1.0f64 * r.sin(), 0.0f64, 0.0f64,
                r.sin(), r.cos(), 0.0f64, 0.0f64,
                0.0f64, 0.0f64, 1.0f64, 0.0f64,
                0.0f64, 0.0f64, 0.0f64, 1.0f64]
        );
        self.affin(&mat)
    }
}
