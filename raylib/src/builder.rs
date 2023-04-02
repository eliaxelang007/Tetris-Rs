use super::raylib::Raylib;
use super::raylib_c::ConfigFlags;

use ConfigFlags::*;

impl RaylibBuilder {
    pub fn new(title: impl Into<String>, width: u16, height: u16) -> Self {
        RaylibBuilder {
            width,
            height,
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn fullscreen(mut self) -> Self {
        self.flags |= FLAG_FULLSCREEN_MODE as u32;
        self
    }

    pub fn resizable(mut self) -> Self {
        self.flags |= FLAG_WINDOW_RESIZABLE as u32;
        self
    }

    pub fn undecorated(mut self) -> Self {
        self.flags |= FLAG_WINDOW_UNDECORATED as u32;
        self
    }

    pub fn transparent(mut self) -> Self {
        self.flags |= FLAG_WINDOW_TRANSPARENT as u32;
        self
    }

    pub fn msaa_4x(mut self) -> Self {
        self.flags |= FLAG_MSAA_4X_HINT as u32;
        self
    }

    pub fn vsync(mut self) -> Self {
        self.flags |= FLAG_VSYNC_HINT as u32;
        self
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    pub fn title(mut self, text: impl Into<String>) -> Self {
        self.title = text.into();
        self
    }

    pub fn build(self) -> Raylib {
        Raylib::new(self.title, self.width, self.height, self.flags)
    }
}

#[derive(Clone, Default)]
pub struct RaylibBuilder {
    pub(super) flags: u32,
    pub(super) width: u16,
    pub(super) height: u16,
    pub(super) title: String,
}
