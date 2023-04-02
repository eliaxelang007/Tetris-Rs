pub use super::raylib_c::Vector2;

impl Default for Vector2 {
    fn default() -> Self {
        Vector2 { x: 0.0, y: 0.0 }
    }
}

pub struct Circle {
    pub center: Vector2,
    pub radius: f32,
}
