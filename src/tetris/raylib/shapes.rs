pub(in super::super) use super::raylib_c::Vector2;
use std::ops::Add;

impl Default for Vector2 {
    fn default() -> Self {
        Vector2 { x: 0.0, y: 0.0 }
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub(in super::super) struct Circle {
    pub(in super::super) radius: f32,
}

pub(in super::super) struct Rectangle {
    pub(in super::super) size: Vector2,
}
