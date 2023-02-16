use super::util::HandyF32;
use strum::{EnumCount, EnumIter};

pub(super) struct Tetromino {
    pub center: Position,
    pub minos: [Mino; 4],
}

impl Tetromino {
    pub(super) fn rotate(&self, rotation: Rotation) -> Self {
        Tetromino {
            center: self.center,
            minos: self.minos.map(|mino| mino.rotate(rotation)),
        }
    }

    pub(super) fn to_position(&self) -> [Position; 4] {
        self.minos.map(|mino| Position {
            row: self.center.row + mino.y_to_center,
            column: self.center.column + mino.x_to_center,
        })
    }
}

#[derive(Copy, Clone)]
pub(super) struct Mino {
    pub(super) x_to_center: f32,
    pub(super) y_to_center: f32,
}

impl Mino {
    fn new(x_to_center: f32, y_to_center: f32) -> Self {
        Mino {
            x_to_center: x_to_center,
            y_to_center: y_to_center,
        }
    }

    fn rotate(&self, rotation: Rotation) -> Self {
        const NINETY_DEG_COS: f32 = -0.00000004371138828673792886547744274139404296875;
        const NINETY_DEG_SIN: f32 = 1.0;

        let rotation_multiplier = match rotation {
            Rotation::Clockwise => -1.0,
            Rotation::Counterclockwise => 1.0,
        };

        let ninety_deg_cos = NINETY_DEG_COS * rotation_multiplier;
        let ninety_deg_sin = NINETY_DEG_SIN * rotation_multiplier;

        Mino {
            x_to_center: (self.x_to_center * ninety_deg_cos - self.y_to_center * ninety_deg_sin)
                .round_to_nearest_half(),
            y_to_center: (self.x_to_center * ninety_deg_sin + self.y_to_center * ninety_deg_cos)
                .round_to_nearest_half(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(super) struct Position {
    pub(super) row: f32,
    pub(super) column: f32,
}

#[derive(Clone, Copy)]
pub(super) enum Rotation {
    Clockwise,
    Counterclockwise,
}

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
        match self {
            TetrominoType::O => Tetromino {
                center: Position {
                    row: 19.5 - 1.0,
                    column: 5.5 - 1.0,
                },
                minos: [
                    Mino::new(-0.5, 0.5),
                    Mino::new(0.5, 0.5),
                    Mino::new(0.5, -0.5),
                    Mino::new(-0.5, -0.5),
                ],
            },
            TetrominoType::I => Tetromino {
                center: Position {
                    row: 18.5 - 1.0,
                    column: 5.5 - 1.0,
                },
                minos: [
                    Mino::new(-1.5, 0.5),
                    Mino::new(-0.5, 0.5),
                    Mino::new(0.5, 0.5),
                    Mino::new(1.5, 0.5),
                ],
            },
            TetrominoType::T => Tetromino {
                center: Position {
                    row: 19.0 - 1.0,
                    column: 5.0 - 1.0,
                },
                minos: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(0.0, 1.0),
                    Mino::new(1.0, 0.0),
                ],
            },
            TetrominoType::L => Tetromino {
                center: Position {
                    column: 5.0 - 1.0,
                    row: 19.0 - 1.0,
                },
                minos: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(1.0, 0.0),
                    Mino::new(1.0, 1.0),
                ],
            },
            TetrominoType::J => Tetromino {
                center: Position {
                    row: 19.0 - 1.0,
                    column: 5.0 - 1.0,
                },
                minos: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(1.0, 0.0),
                    Mino::new(-1.0, 1.0),
                ],
            },
            TetrominoType::S => Tetromino {
                center: Position {
                    row: 19.0 - 1.0,
                    column: 5.0 - 1.0,
                },
                minos: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(0.0, 1.0),
                    Mino::new(1.0, 1.0),
                ],
            },
            TetrominoType::Z => Tetromino {
                center: Position {
                    row: 19.0 - 1.0,
                    column: 5.0 - 1.0,
                },
                minos: [
                    Mino::new(0.0, 0.0),
                    Mino::new(0.0, 1.0),
                    Mino::new(-1.0, 1.0),
                    Mino::new(1.0, 0.0),
                ],
            },
        }
    }
}
