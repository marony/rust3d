#[derive(Debug, Copy, Clone)]
pub struct Size2 {
    pub width: f64,
    pub height: f64
}

impl Size2 {
    pub fn new(width: f64, height: f64) -> Size2 {
        Size2 { width: width, height: height }
    }
}