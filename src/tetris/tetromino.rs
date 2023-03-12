struct Tetromino {
    center: Center,
    minoes: [Mino; 4],
}

impl Tetromino {
    fn rotate(&self, rotation: Rotation) -> Self {
        Tetromino {
            center: self.center,
            minoes: self.minoes.map(|mino| mino.rotate(rotation)),
        }
    }

    fn snap_to_grid(&self) -> [Snapped; 4] {
        self.minoes.map(|mino| Snapped {
            row: (self.center.row + f32::from(mino.y_to_center)) as i8,
            column: (f32::from(self.center.column) + f32::from(mino.x_to_center)) as i8,
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
    struct Coordinate {
        x: f32,
        y: f32,
    }

    impl Coordinate {
        fn distance(&self, other: &Coordinate) -> f32 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }

    impl From<Mino> for Coordinate {
        fn from(mino: Mino) -> Self {
            Coordinate {
                x: mino.x_to_center.into(),
                y: mino.y_to_center.into(),
            }
        }
    }

    impl From<Snapped> for Coordinate {
        fn from(mino: Snapped) -> Self {
            Coordinate {
                x: mino.column.into(),
                y: mino.row.into(),
            }
        }
    }

    use itertools::Itertools;

    let t_block = TetrominoType::T.new();

    for fractional_part in (0..5000).map(|i| (i as f32) / 1000.0) {
        let old_distances = t_block
            .minoes
            .iter()
            .combinations(2)
            .map(|minoes| Coordinate::from(minoes[0].clone()).distance(&minoes[1].clone().into()))
            .collect::<Vec<_>>();

        let snapped = Tetromino {
            center: Center {
                row: t_block.center.row + fractional_part,
                column: t_block.center.column,
            },
            minoes: t_block.minoes,
        }
        .snap_to_grid();

        let new_distances = snapped
            .iter()
            .combinations(2)
            .map(|items| Coordinate::from(items[0].clone()).distance(&items[1].clone().into()))
            .collect::<Vec<_>>();

        assert_eq!(old_distances, new_distances);

        let unsnapped = snapped.map(|Snapped { row, column }| {
            Mino::new(
                (column as f32) - f32::from(t_block.center.column),
                (row as f32) - t_block.center.row,
            )
        });

        assert_eq!(unsnapped, t_block.minoes);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Mino {
    x_to_center: HalfStep,
    y_to_center: HalfStep,
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
enum TetrominoType {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

impl TetrominoType {
    fn new(&self) -> Tetromino {
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
struct Snapped {
    row: i8,
    column: i8,
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
enum Rotation {
    Clockwise,
    Counterclockwise,
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
struct HalfStep(i8);

impl From<f32> for HalfStep {
    fn from(half_step: f32) -> Self {
        HalfStep(((half_step.abs() * 2.0).round() * half_step.signum()) as i8)
    }
}

impl From<HalfStep> for f32 {
    fn from(half_step: HalfStep) -> Self {
        (half_step.0 as f32) / 2.0
    }
}

#[test]
fn test_conversions() {
    let mut tests: Vec<(f32, i8, f32)> = vec![
        (0.24, 0, 0.0),
        (0.25, 1, 0.5),
        (0.26, 1, 0.5),
        (0.5, 1, 0.5),
        (0.6, 1, 0.5),
    ];

    let negative_tests = tests
        .iter()
        .map(|(start, doubled, stepped)| (start * -1.0, doubled * -1, stepped * -1.0))
        .collect::<Vec<(f32, i8, f32)>>();

    tests.extend(negative_tests);

    for (start, doubled, stepped) in tests {
        let half_step: HalfStep = start.into();

        assert_eq!(half_step.0, doubled);
        assert_eq!(f32::from(half_step), stepped);
    }
}
