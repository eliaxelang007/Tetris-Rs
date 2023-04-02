use super::raylib::Raylib;
use super::raylib_c::WindowShouldClose;

impl Raylib {
    pub fn window_should_close(&self) -> bool {
        unsafe { WindowShouldClose() }
    }
}
