use super::raylib_c::IsKeyDown;
pub(in super::super) use super::raylib_c::KeyboardKey;

pub(in super::super) struct Input {
    pub(super) _use_raylib_builder_instead: (),
}

impl Input {
    pub(in super::super) fn key_down(&self, key: KeyboardKey) -> bool {
        unsafe { IsKeyDown(key as i32) }
    }
}
