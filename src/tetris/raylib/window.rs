use super::raylib_c::{CloseWindow, GetScreenHeight, GetScreenWidth, InitWindow, IsWindowReady, SetConfigFlags};

use std::ffi::CString;

use std::sync::atomic::{AtomicBool, Ordering};

static INITIALIZED: AtomicBool = AtomicBool::new(false);

use super::raylib_c::WindowShouldClose;

pub(in super::super) struct Window {
    title: CString,
}

impl Window {
    pub(super) fn new(title: impl Into<String>, width: u16, height: u16, flags: u32) -> Self {
        unsafe {
            SetConfigFlags(flags);
        }

        if INITIALIZED.load(Ordering::Relaxed) {
            panic!("You can't have more than one Raylib Instance at a time.");
        }

        INITIALIZED.store(true, Ordering::Relaxed);

        let window = Window {
            title: CString::new(title.into()).unwrap(),
        };

        unsafe {
            InitWindow(width.into(), height.into(), window.title.as_ptr());
        }

        if !unsafe { IsWindowReady() } {
            panic!("Couldn't initialize a window for you.");
        }

        window
    }
}

impl Window {
    pub(in super::super) fn should_close(&self) -> bool {
        unsafe { WindowShouldClose() }
    }

    pub(in super::super) fn height(&self) -> u16 {
        unsafe { GetScreenHeight() as u16 }
    }

    pub(in super::super) fn width(&self) -> u16 {
        unsafe { GetScreenWidth() as u16 }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        if !INITIALIZED.load(Ordering::Relaxed) {
            return;
        }

        unsafe {
            CloseWindow();
        }

        INITIALIZED.store(false, Ordering::Relaxed);
    }
}
