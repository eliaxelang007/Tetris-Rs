#[derive(Debug, Copy, Clone)]
struct HalfStep(i8);

impl HalfStep {
    fn from<const POSITION: f32>() {
        HalfStep(((POSITION.abs() * 2.0).round() * POSITION.signum()) as i8)
    }
}

impl From<HalfStep> for f32 {
    fn from(half_step: HalfStep) -> Self {
        (half_step.0 as f32) / 2.0
    }
}
