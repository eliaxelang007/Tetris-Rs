use std::{
    ops::{Add, Sub},
    time::Duration,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Tetromino {
    center: Center,
    minoes: [Mino; 4],
}

impl Tetromino {
    pub(super) fn rotate(&self, rotation: Rotation) -> Self {
        Tetromino {
            minoes: self.minoes.map(|mino| mino.rotate(rotation)),
            ..(*self)
        }
    }

    pub(super) fn fall(&self, speed: f32, delta_time: Duration) -> Self {
        Tetromino {
            center: Center {
                row: self.center.row - (speed * delta_time.as_secs_f32()),
                ..self.center
            },
            ..*self
        }
    }

    pub(super) fn shift(&self, shifter: Shifter) -> Self {
        Tetromino {
            center: Center {
                column: self.center.column + shifter.x_axis_shift().into(),
                ..self.center
            },
            ..*self
        }
    }

    pub(super) fn snap_to_grid(&self) -> [Snapped; 4] {
        self.minoes.map(|mino| Snapped {
            row: (self.center.row + f32::from(mino.y_to_center)).floor() as i8,
            column: (f32::from(self.center.column) + f32::from(mino.x_to_center)).floor() as i8,
        })
    }
}

#[test]
fn test_tetromino_rotation() {
    let l_block = TetrominoType::L.new();

    let clockwise_rotated_minoes = [
        Mino::new(0.0, 0.0),
        Mino::new(0.0, 1.0),
        Mino::new(0.0, -1.0),
        Mino::new(1.0, -1.0),
    ];

    let clockwise = l_block.rotate(Rotation::Clockwise);

    assert_eq!(clockwise.minoes, clockwise_rotated_minoes);

    let counterclockwise = clockwise
        .rotate(Rotation::Counterclockwise)
        .rotate(Rotation::Counterclockwise);

    let counterclockwise_rotated_minoes = [
        Mino::new(0.0, 0.0),
        Mino::new(0.0, -1.0),
        Mino::new(0.0, 1.0),
        Mino::new(-1.0, 1.0),
    ];

    assert_eq!(counterclockwise.minoes, counterclockwise_rotated_minoes);
}

#[test]
fn test_tetromino_snapping() {
    pub(super) struct Vector2 {
        x: f32,
        y: f32,
    }

    impl Vector2 {
        pub(super) fn distance(&self, other: &Vector2) -> f32 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }

    impl From<Mino> for Vector2 {
        fn from(mino: Mino) -> Self {
            Vector2 {
                x: mino.x_to_center.into(),
                y: mino.y_to_center.into(),
            }
        }
    }

    impl From<Snapped> for Vector2 {
        fn from(mino: Snapped) -> Self {
            Vector2 {
                x: mino.column.into(),
                y: mino.row.into(),
            }
        }
    }

    use itertools::Itertools;

    fn get_distances<T>(coordinates: [T; 4]) -> Vec<f32>
    where
        Vector2: From<T>,
        T: Clone,
    {
        coordinates
            .iter()
            .combinations(2)
            .map(|pair| Vector2::from(pair[0].clone()).distance(&pair[1].clone().into()))
            .collect()
    }

    let t_block = TetrominoType::Z.new();

    for fractional_part in (0..5000).map(|i| (i as f32) / 1000.0) {
        let old_distances = get_distances(t_block.minoes);

        let snapped = Tetromino {
            center: Center {
                row: t_block.center.row - fractional_part,
                ..t_block.center
            },
            ..t_block
        }
        .snap_to_grid();

        let new_distances = get_distances(snapped);

        assert_eq!(old_distances, new_distances);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

    fn rotate(&self, rotation: Rotation) -> Self {
        const NINETY_DEG_COS: f32 = -0.00000004371138828673792886547744274139404296875;
        const NINETY_DEG_SIN: f32 = 1.0;

        let rotation_multiplier = rotation.rotation_multiplier() as f32;

        let ninety_deg_cos = NINETY_DEG_COS * rotation_multiplier;
        let ninety_deg_sin = NINETY_DEG_SIN * rotation_multiplier;

        let x_to_center: f32 = self.x_to_center.into();
        let y_to_center: f32 = self.y_to_center.into();

        Mino::new(
            x_to_center * ninety_deg_cos - y_to_center * ninety_deg_sin,
            x_to_center * ninety_deg_sin + y_to_center * ninety_deg_cos,
        )
    }
}

#[test]
fn test_mino_rotation() {
    let mut minoes = [
        Mino::new(0.5, 0.5),
        Mino::new(0.5, -0.5),
        Mino::new(-0.5, -0.5),
        Mino::new(-0.5, 0.5),
    ];

    let rotated = minoes.map(|mino| mino.rotate(Rotation::Clockwise));

    use std::iter::zip;

    minoes.rotate_left(1);

    assert_eq!(minoes, rotated);

    let rotated = minoes.map(|mino| mino.rotate(Rotation::Counterclockwise));

    minoes.rotate_right(1);

    assert_eq!(minoes, rotated);
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
        match self {
            TetrominoType::O => Tetromino {
                center: Center::new(19.5 - 1.0, 5.5 - 1.0),
                minoes: [
                    Mino::new(-0.5, 0.5),
                    Mino::new(0.5, 0.5),
                    Mino::new(0.5, -0.5),
                    Mino::new(-0.5, -0.5),
                ],
            },
            TetrominoType::I => Tetromino {
                center: Center::new(18.5 - 1.0, 5.5 - 1.0),
                minoes: [
                    Mino::new(-1.5, 0.5),
                    Mino::new(-0.5, 0.5),
                    Mino::new(0.5, 0.5),
                    Mino::new(1.5, 0.5),
                ],
            },
            TetrominoType::T => Tetromino {
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(0.0, 1.0),
                    Mino::new(1.0, 0.0),
                ],
            },
            TetrominoType::L => Tetromino {
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(1.0, 0.0),
                    Mino::new(1.0, 1.0),
                ],
            },
            TetrominoType::J => Tetromino {
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(1.0, 0.0),
                    Mino::new(-1.0, 1.0),
                ],
            },
            TetrominoType::S => Tetromino {
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(0.0, 1.0),
                    Mino::new(1.0, 1.0),
                ],
            },
            TetrominoType::Z => Tetromino {
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

#[derive(Debug, PartialEq, Clone, Copy)]
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
pub enum Shifter {
    Left,
    Right,
}

impl Shifter {
    fn x_axis_shift(&self) -> i8 {
        match self {
            Shifter::Left => -1,
            Shifter::Right => 1,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Add for HalfStep {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        HalfStep(self.0 + other.0)
    }
}

#[test]
fn test_conversions() {
    #[derive(Clone)]
    struct Conversion {
        original: f32,
        doubled: i8,
        stepped: f32,
    }

    impl Conversion {
        fn new(original: f32, doubled: i8, stepped: f32) -> Self {
            Conversion {
                original,
                doubled,
                stepped,
            }
        }
    }

    let mut tests: Vec<Conversion> = vec![
        Conversion::new(0.24, 0, 0.0),
        Conversion::new(0.25, 1, 0.5),
        Conversion::new(0.26, 1, 0.5),
        Conversion::new(0.5, 1, 0.5),
        Conversion::new(0.6, 1, 0.5),
    ];

    let negative_tests = tests.iter().map(
        |Conversion {
             original,
             doubled,
             stepped,
         }| Conversion::new(original * -1.0, doubled * -1, stepped * -1.0),
    );

    for Conversion {
        original,
        doubled,
        stepped,
    } in tests.clone().into_iter().chain(negative_tests)
    {
        let half_step: HalfStep = original.into();

        assert_eq!(half_step.0, doubled);
        assert_eq!(f32::from(half_step), stepped);
    }
}
