use super::raylib::Raylib;
use super::raylib_c::ConfigFlags;

use ConfigFlags::*;

impl<'a> RaylibBuilder<'a> {
    pub(in super::super) fn new(title: impl Into<&'a str>, width: u16, height: u16) -> Self {
        RaylibBuilder {
            width,
            height,
            title: title.into(),
            ..Default::default()
        }
    }

    // pub(in super::super) fn fullscreen(mut self) -> Self {
    //     self.flags |= FLAG_FULLSCREEN_MODE as u32;
    //     self
    // }

    // pub(in super::super) fn resizable(mut self) -> Self {
    //     self.flags |= FLAG_WINDOW_RESIZABLE as u32;
    //     self
    // }

    // pub(in super::super) fn undecorated(mut self) -> Self {
    //     self.flags |= FLAG_WINDOW_UNDECORATED as u32;
    //     self
    // }

    // pub(in super::super) fn transparent(mut self) -> Self {
    //     self.flags |= FLAG_WINDOW_TRANSPARENT as u32;
    //     self
    // }

    // pub(in super::super) fn msaa_4x(mut self) -> Self {
    //     self.flags |= FLAG_MSAA_4X_HINT as u32;
    //     self
    // }

    pub(in super::super) fn vsync(mut self) -> Self {
        self.flags |= FLAG_VSYNC_HINT as u32;
        self
    }

    // pub(in super::super) fn width(mut self, width: u16) -> Self {
    //     self.width = width;
    //     self
    // }

    // pub(in super::super) fn height(mut self, height: u16) -> Self {
    //     self.height = height;
    //     self
    // }

    // pub(in super::super) fn title(mut self, text: impl Into<&'a str>) -> Self {
    //     self.title = text.into();
    //     self
    // }

    pub(in super::super) fn build(self) -> Raylib {
        Raylib::new(self.title, self.width, self.height, self.flags)
    }
}

#[derive(Clone, Default)]
pub struct RaylibBuilder<'a> {
    pub(super) flags: u32,
    pub(super) width: u16,
    pub(super) height: u16,
    pub(super) title: &'a str,
}
