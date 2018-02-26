use util::point3::Point3;
use util::vector3::Vector3;
use util::matrix4::Matrix4;
use util::polygon3::Polygon3;
use util::screen::Screen;

#[derive(Debug)]
pub struct Camera {
    pub position: Point3,
    pub look_at: Point3,
    pub up: Vector3,
    pub near: f64,
    pub far: f64
}

impl Camera {
    pub fn new(position: &Point3, look_at: &Point3, up: &Vector3, near: f64, far: f64) -> Camera {
        Camera { position: *position, look_at: *look_at, up: *up, near, far }
    }

    // 裏を向いているか？
    pub fn is_cull(&self, polygon : Polygon3) -> bool {
        polygon.normal().dot(&self.direction()) >= 0.0f64
    }

    // 見ている方向
    pub fn direction(&self) -> Vector3 {
        &Vector3::from(self.look_at) - &self.position
    }

    // ビューポート変換
    pub fn convert_to_view(&self, polygon: Polygon3) -> Polygon3 {
        let z = self.direction().normalize();
        let x = (&self.up * &z).normalize();
        let y = (&z * &x).normalize();
        let p = Vector3::from(self.position);
        let tx = -1.0f64 * p.dot(&x);
        let ty = -1.0f64 * p.dot(&y);
        let tz = -1.0f64 * p.dot(&z);
        let mat = Matrix4::new(
          [x.x, x.y, x.z, tx,
          y.x, y.y, y.z, ty,
          z.x, z.y, z.z, tz,
          0.0f64, 0.0f64, 0.0f64, 1.0f64]
        );
        polygon.affin(&mat)
    }

    // 投影変換
    fn _projection(&self, point : &Point3, screen : &Screen) -> Point3 {
      Point3::new(
        self.near * 2.0f64 * point.x / screen.size.width,
        self.near * 2.0f64 * point.y / screen.size.height,
        (self.far + self.near) * point.z / (self.far - self.near) + (2.0f64 * self.near * self.far) / (self.far - self.near),
        (2.0f64 * self.near * self.far) * point.w / (self.far - self.near)
      )
    }

    // 投影変換
    pub fn projection(&self, polygon : &Polygon3, screen : &Screen) -> Polygon3 {
      Polygon3::new(
        &self._projection(&(polygon.p1), screen),
        &self._projection(&(polygon.p2), screen),
        &self._projection(&(polygon.p3), screen)
      )
    }

    pub fn perspective(&self, polygon : &Polygon3) -> Polygon3 {
      Polygon3::new(
          &(&(polygon.p1) / polygon.p1.w),
          &(&(polygon.p2) / polygon.p2.w),
          &(&(polygon.p3) / polygon.p3.w)
      )
    }
}
