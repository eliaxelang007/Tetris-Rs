pub(in super::super) use super::raylib_c::Vector2;

impl Default for Vector2 {
    fn default() -> Self {
        Vector2 { x: 0.0, y: 0.0 }
    }
}

pub(in super::super) struct Circle {
    pub(in super::super) radius: f32,
}

pub(in super::super) struct Rectangle {
    pub(in super::super) size: Vector2,
}
