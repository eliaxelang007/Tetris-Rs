use super::raylib_c::{CloseWindow, InitWindow, IsWindowReady, SetConfigFlags};

use std::ffi::CString;

use std::sync::atomic::{AtomicBool, Ordering};

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Raylib;

impl Raylib {
    pub fn new(title: impl Into<String>, width: u16, height: u16, flags: u32) -> Raylib {
        unsafe {
            SetConfigFlags(flags);
        }

        if INITIALIZED.load(Ordering::Relaxed) {
            panic!("You can't have more than one Raylib Instance at a time.");
        }

        let title = CString::new(title.into()).unwrap();

        unsafe {
            InitWindow(width.into(), height.into(), title.as_ptr());
        }

        if !unsafe { IsWindowReady() } {
            panic!("Couldn't initialize a window for you.");
        }

        INITIALIZED.store(true, Ordering::Relaxed);

        Raylib
    }
}

impl Drop for Raylib {
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
