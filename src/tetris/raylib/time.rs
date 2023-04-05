use super::raylib_c::{GetFPS, GetFrameTime, GetTime, SetTargetFPS};
use super::window::Window;

use std::time::Duration;

impl Window {
    pub(in super::super) fn set_target_fps(&self, fps: u16) {
        unsafe { SetTargetFPS(fps.into()) }
    }

    pub(in super::super) fn fps(&self) -> u16 {
        unsafe { GetFPS() as u16 }
    }

    pub(in super::super) fn frame_time(&self) -> Duration {
        let frame_time = unsafe { GetFrameTime() };
        Duration::from_secs_f32(frame_time)
    }

    pub(in super::super) fn elapsed_time(&self) -> Duration {
        let elapsed_time = unsafe { GetTime() };
        Duration::from_secs_f64(elapsed_time)
    }
}
