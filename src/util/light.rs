use util::point3::Point3;
use util::polygon3::Polygon3;
use util::vector3::Vector3;
use util::color::Color;

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub position: Point3
}

impl Light {
    pub fn new(position: &Point3) -> Light {
        Light { position: *position }
    }
    // 拡散光の計算(ランバードの余弦則)
    pub fn get_diffuse_color(&self, color : &Color, polygon3: &Polygon3) -> Color {
        let l = Vector3::from(&self.position - &polygon3.p1).normalize();
        let cosa = l.dot(&polygon3.normal().normalize());
        let level = if cosa >= 0.0f64 { cosa * 0.9f64 } else { 0.0f64 };
        let mut r = color.r as f64 * level + 0.1f64;
        let mut g = color.g as f64 * level + 0.1f64;
        let mut b = color.b as f64 * level + 0.1f64;
        r = 255.0f64.min((255.0f64 * 0.1f64).max(r));
        g = 255.0f64.min((255.0f64 * 0.1f64).max(g));
        b = 255.0f64.min((255.0f64 * 0.1f64).max(b));
        Color::new(r as u8, g as u8, b as u8)
    }
}
