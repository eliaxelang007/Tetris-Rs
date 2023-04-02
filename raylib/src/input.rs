use super::raylib::Raylib;
use super::raylib_c::IsKeyDown;
pub use super::raylib_c::KeyboardKey;

impl Raylib {
    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        unsafe { IsKeyDown(key as i32) }
    }
}
