use std::{ops::AddAssign, time::Duration};

#[derive(Debug, Clone)]
pub(super) struct Tetromino {
    center: Center,
    minoes: [Mino; 4],
    kind: TetrominoType,
}

impl Tetromino {
    pub(super) fn rotate(mut self, rotation: Rotation) -> Self {
        self.minoes = self.minoes.map(|mino| mino.rotate(rotation));
        self
    }

    pub(super) fn fall(mut self, speed: f32, delta_time: Duration) -> Self {
        self.center.row -= speed * delta_time.as_secs_f32();
        self
    }

    pub(super) fn shift(mut self, step: Step) -> Self {
        self.center.column += step.x_axis_step().into();
        self
    }

    pub(super) fn snap_to_grid(&self) -> [Snapped; 4] {
        self.minoes.clone().map(|mino| Snapped {
            row: (self.center.row + f32::from(mino.y_to_center)).floor() as i8,
            column: (f32::from(self.center.column.clone()) + f32::from(mino.x_to_center)).floor() as i8,
        })
    }
}

use super::raylib::{
    drawing::{Canvas, Color, Drawable, RectangleGraphic},
    shapes::{Rectangle, Vector2},
};

struct TetrominoGraphic<'a> {
    tetromino: &'a Tetromino,
}

impl<'a, 'b> Drawable<'a> for TetrominoGraphic<'b> {
    fn draw(&self, canvas: Canvas<'a>) -> Canvas<'a> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Mino {
    pub(super) x_to_center: HalfStep,
    pub(super) y_to_center: HalfStep,
}

impl Mino {
    fn new(x_to_center: f32, y_to_center: f32) -> Self {
        Mino {
            x_to_center: x_to_center.into(),
            y_to_center: y_to_center.into(),
        }
    }

    fn rotate(mut self, rotation: Rotation) -> Self {
        const NINETY_DEG_COS: f32 = -0.00000004371138828673792886547744274139404296875;
        const NINETY_DEG_SIN: f32 = 1.0;

        let rotation_multiplier = rotation.rotation_multiplier() as f32;

        let ninety_deg_cos = NINETY_DEG_COS * rotation_multiplier;
        let ninety_deg_sin = NINETY_DEG_SIN * rotation_multiplier;

        let x_to_center: f32 = self.x_to_center.into();
        let y_to_center: f32 = self.y_to_center.into();

        self.x_to_center = (x_to_center * ninety_deg_cos - y_to_center * ninety_deg_sin).into();
        self.y_to_center = (x_to_center * ninety_deg_sin + y_to_center * ninety_deg_cos).into();

        self
    }
}

use strum::{EnumCount, EnumIter};

#[derive(EnumCount, EnumIter, Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum TetrominoType {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

impl TetrominoType {
    pub(super) fn new(&self) -> Tetromino {
        use TetrominoType::*;

        match self {
            O => Tetromino {
                kind: O,
                center: Center::new(19.5 - 1.0, 5.5 - 1.0),
                minoes: [
                    Mino::new(-0.5, 0.5),
                    Mino::new(0.5, 0.5),
                    Mino::new(0.5, -0.5),
                    Mino::new(-0.5, -0.5),
                ],
            },
            I => Tetromino {
                kind: I,
                center: Center::new(18.5 - 1.0, 5.5 - 1.0),
                minoes: [
                    Mino::new(-1.5, 0.5),
                    Mino::new(-0.5, 0.5),
                    Mino::new(0.5, 0.5),
                    Mino::new(1.5, 0.5),
                ],
            },
            T => Tetromino {
                kind: T,
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(0.0, 1.0),
                    Mino::new(1.0, 0.0),
                ],
            },
            L => Tetromino {
                kind: L,
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(1.0, 0.0),
                    Mino::new(1.0, 1.0),
                ],
            },
            J => Tetromino {
                kind: J,
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(1.0, 0.0),
                    Mino::new(-1.0, 1.0),
                ],
            },
            S => Tetromino {
                kind: S,
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(0.0, 1.0),
                    Mino::new(1.0, 1.0),
                ],
            },
            Z => Tetromino {
                kind: Z,
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(0.0, 1.0),
                    Mino::new(-1.0, 1.0),
                    Mino::new(1.0, 0.0),
                ],
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) struct Snapped {
    pub(super) row: i8,
    pub(super) column: i8,
}

#[derive(Debug, Clone)]
struct Center {
    row: f32,
    column: HalfStep,
}

impl Center {
    fn new(row: f32, column: f32) -> Self {
        Center {
            row,
            column: column.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Rotation {
    Clockwise,
    Counterclockwise,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Step {
    Left,
    Right,
}

impl Step {
    fn x_axis_step(&self) -> i8 {
        match self {
            Step::Left => -1,
            Step::Right => 1,
        }
    }
}

impl Rotation {
    fn rotation_multiplier(&self) -> i8 {
        match self {
            Rotation::Clockwise => -1,
            Rotation::Counterclockwise => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct HalfStep(i8);

impl From<f32> for HalfStep {
    fn from(half_step: f32) -> Self {
        HalfStep(((half_step.abs() * 2.0).round() * half_step.signum()) as i8)
    }
}

impl From<i8> for HalfStep {
    fn from(half_step: i8) -> Self {
        (half_step as f32).into()
    }
}

impl From<HalfStep> for f32 {
    fn from(half_step: HalfStep) -> Self {
        (half_step.0 as f32) / 2.0
    }
}

impl AddAssign for HalfStep {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}
