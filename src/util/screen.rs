use util::size2::Size2;
use util::point3::Point3;
use util::polygon3::Polygon3;

#[derive(Debug)]
pub struct Screen {
    pub size: Size2,
    pub scale: f64
}

impl Screen {
    pub fn new(size: &Size2, scale: f64) -> Screen {
        Screen { size: *size, scale }
    }
    // 投影面をディスプレイに合わせる
    fn _convert_to_screen(&self, point : &Point3) -> Point3 {
        Point3 {
            x: self.size.width / 2.0f64 + point.x * self.scale,
            y: self.size.height / 2.0f64 - point.y * self.scale,
            z: point.z,
            w: 1.0f64
        }
    }
    // ポリゴンをスクリーン(画面)の座標に合わせる
    pub fn convert_to_screen(&self, polygon : &Polygon3) -> Polygon3 {
        Polygon3 {
            p1: self._convert_to_screen(&polygon.p1),
            p2: self._convert_to_screen(&polygon.p2),
            p3: self._convert_to_screen(&polygon.p3)
        }
    }
}
