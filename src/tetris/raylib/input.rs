pub(in super::super) use super::raylib_c::KeyboardKey;
use super::raylib_c::{IsKeyDown, IsKeyPressed};

pub(in super::super) struct Input(pub(super) ());

impl Input {
    pub(in super::super) fn key_down(&self, key: KeyboardKey) -> bool {
        unsafe { IsKeyDown(key as i32) }
    }

    pub(in super::super) fn key_pressed(&self, key: KeyboardKey) -> bool {
        unsafe { IsKeyPressed(key as i32) }
    }
}
