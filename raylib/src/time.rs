use super::raylib::Raylib;
use super::raylib_c::{GetFPS, GetFrameTime, GetTime, SetTargetFPS};

use std::time::Duration;

impl Raylib {
    pub fn set_target_fps(&self, fps: u16) {
        unsafe { SetTargetFPS(fps.into()) }
    }

    pub fn get_fps(&self) -> u16 {
        unsafe { GetFPS() as u16 }
    }

    pub fn get_frame_time(&self) -> Duration {
        let frame_time = unsafe { GetFrameTime() };
        Duration::from_secs_f32(frame_time)
    }

    pub fn get_elapsed_time(&self) -> Duration {
        let elapsed_time = unsafe { GetTime() };
        Duration::from_secs_f64(elapsed_time)
    }
}
