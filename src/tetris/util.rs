pub(super) trait HandyF32 {
    fn round_to_nearest_half(self) -> f32;
}

impl HandyF32 for f32 {
    fn round_to_nearest_half(self) -> f32 {
        (self * 2.0).round() / 2.0
    }
}
