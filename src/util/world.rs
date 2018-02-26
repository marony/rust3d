extern crate gio;
extern crate gtk;
extern crate cairo;

use std::cmp::Ordering;
use std::f64::consts::PI;
use gio::prelude::*;
use gtk::prelude::*;

use util::screen::Screen;
use util::color::Color;
use util::size2::Size2;
use util::point3::Point3;
use util::vector3::Vector3;
use util::polygon3::Polygon3;
use util::camera::Camera;
use util::light::Light;

#[derive(Debug)]
pub struct World {
    pub polygons: Vec<(Polygon3, Color)>,
    pub screen: Screen,
    pub camera: Camera,
    pub light: Light
}

impl World {
    pub fn new(polygons: &Vec<(Polygon3, Color)>, width: f64, height: f64, scale: f64) -> World {
        let rotate = 0i32;

        // スクリーン(画面)
        let screen = Screen::new(&Size2::new(width, height), scale);
        // カメラ
        let position = Point3::new(0.0f64, -1.0f64, -100.0f64, 1.0f64);
        let look_at = Point3::new(0.0f64, -1.0f64, 0.0f64, 1.0f64);
        let up = Vector3::new(0.0f64, 1.0f64, 0.0f64, 1.0f64);
        let near = 10.0f64;
        let far = 300.0f64;
        let camera = Camera::new(&position, &look_at, &up, near, far);
        // 光源
        let light = Light::new(&Point3::new(-500.0f64, 500.0f64, -500.0f64, 1.0f64));

        World { polygons: polygons.clone(), screen, camera, light }
    }

    pub fn draw(&self, rotate: i32, cr: &cairo::Context) -> () {
        let r = 0.0f64;
        // ？？？
        cr.scale(1.0f64, 1.0f64);
        cr.translate(self.screen.size.width / 2.0f64, self.screen.size.height);
        // 背景を塗る
        cr.set_source_rgba(0.0f64, 0.0f64, 0.0f64, 1.0f64);
        cr.paint();
        // ポリゴン群を投影面の座標に変換
        let mut v1: Vec<(Polygon3, Color)> = self.polygons.iter()
            .map(|&(p, c)|
                // 回転
                (p.rotate_x(r / 360.0f64 * 2.0f64 * PI)
                  .rotate_y(f64::from(rotate) / 360.0f64 * 2.0f64 * PI), c))
            // 拡散光の計算
            .map(|(p, c)| (p, self.light.get_diffuse_color(&c, &p)))
            // カリング(カメラから見て裏面のポリゴンは省略)
            .filter(|&(p, c)| !self.camera.is_cull(p))
            // ビューポート変換
            .map(|(p, c)| (self.camera.convert_to_view(p), c))
            .collect();
        // 奥からソート
        v1.sort_by(|&(l, _), &(r, _)| {
            let _l = l.p1.z + l.p2.z + l.p3.z;
            let _r = r.p1.z + r.p2.z + r.p3.z;
            if _l - _r < 1e-10f64 { Ordering::Equal } else if _l < _r { Ordering::Greater } else { Ordering::Less }
        });
        v1.iter()
            // 射影変換
            .map(|&(p, c)| (self.camera.projection(&p, &self.screen), c))
            // 遠近感
            .filter(|&(p, c)|
                p.p1.z >= self.camera.near && p.p1.z <= self.camera.far &&
                p.p2.z >= self.camera.near && p.p2.z <= self.camera.far &&
                p.p3.z >= self.camera.near && p.p3.z <= self.camera.far)
            .map(|(p, c)| (self.camera.perspective(&p), c))
            // スクリーン変換
            .map(|(p, c)| (self.screen.convert_to_screen(&p), c))
            // 描画
            .for_each(|(p, c)| p.draw(cr, &c));
    }
}
