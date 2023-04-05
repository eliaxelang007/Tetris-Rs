use super::input::Input;
use super::window::Window;

pub struct Raylib {
    _use_raylib_builder_instead: (),
    pub(in super::super) window: Window,
    pub(in super::super) input: Input,
}

impl Raylib {
    pub(super) fn new(title: impl Into<String>, width: u16, height: u16, flags: u32) -> Self {
        Raylib {
            _use_raylib_builder_instead: (),
            window: Window::new(title, width, height, flags),
            input: Input {
                _use_raylib_builder_instead: (),
            },
        }
    }
}
