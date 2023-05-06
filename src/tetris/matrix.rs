use super::tetromino::{Snapped, Tetromino};

pub(super) const PLAYFIELD_ROWS: usize = 20;
pub(super) const PLAYFIELD_COLUMNS: usize = 10;

#[derive(Clone)]
pub(super) struct Matrix {
    cells: [[Cell; PLAYFIELD_COLUMNS]; PLAYFIELD_ROWS],
}

impl Matrix {
    pub(super) fn new() -> Self {
        Matrix {
            cells: [[Cell::Empty; PLAYFIELD_COLUMNS]; PLAYFIELD_ROWS],
        }
    }

    pub(super) fn solidify(mut self, tetromino: &Tetromino) -> Self {
        for position in tetromino.snap_to_grid() {
            self.cells[position.row as usize][position.column as usize] = Cell::Filled;
        }

        self
    }

    pub(super) fn clear_lines(mut self) -> Self {
        fn clear_lines(matrix: &mut [[Cell; PLAYFIELD_COLUMNS]]) {
            let length = matrix.len();

            for (index, line) in matrix.iter().enumerate() {
                if line.filled() {
                    let shift_down_range = (index + 1)..length;

                    let cleared = clear_lines(&mut matrix[shift_down_range.clone()]);
                    matrix.copy_within(shift_down_range, index);

                    matrix[length - 1] = [Cell::Empty; PLAYFIELD_COLUMNS];

                    break;
                }
            }
        }

        clear_lines(&mut self.cells);

        self
    }

    pub(super) fn validate(&self, tetromino: &Tetromino) -> TetrominoValidity {
        match tetromino.snap_to_grid().iter().all(|Snapped { row, column }| {
            (0..(PLAYFIELD_ROWS as i8)).contains(&row)
                && (0..(PLAYFIELD_COLUMNS as i8)).contains(&column)
                && self.cells[*row as usize][*column as usize] != Cell::Filled
        }) {
            true => TetrominoValidity::Valid,
            false => TetrominoValidity::Invalid,
        }
    }
}

use super::raylib::{
    drawing::{Canvas, Color, Drawable, RectangleGraphic},
    shapes::{Rectangle, Vector2},
};

impl<'a> Drawable<'a> for Matrix {
    fn draw(&self, canvas: Canvas<'a>) -> Canvas<'a> {
        const CELL_SIZE: f32 = 45.0;

        const TOP_RIGHT_CELL_POSITION: Vector2 = Vector2 { x: 403.0, y: 21.0 };

        let mut canvas = canvas;

        for row in 0..PLAYFIELD_ROWS {
            for column in 0..PLAYFIELD_COLUMNS {
                canvas = canvas.draw(&RectangleGraphic {
                    rectangle: Rectangle {
                        size: Vector2 {
                            x: CELL_SIZE,
                            y: CELL_SIZE,
                        },
                    },
                    position: TOP_RIGHT_CELL_POSITION
                        + Vector2 {
                            x: CELL_SIZE * (column as f32),
                            y: CELL_SIZE * ((PLAYFIELD_ROWS - row - 1) as f32),
                        },
                    color: if self.cells[row][column] == Cell::Filled {
                        Color::MAROON
                    } else {
                        Color::GRAY
                    },
                });
            }
        }

        canvas

        // canvas.draw(&RectangleGraphic {
        //     rectangle: Rectangle {
        //         size: Vector2 { x: 450.0, y: 900.0 },
        //     },
        //     position: Vector2 { x: 403.0, y: 21.0 },
        //     color: Color::MAROON,
        // })
    }
}

trait RowExtension {
    fn filled(&self) -> bool;
}

impl RowExtension for [Cell; PLAYFIELD_COLUMNS] {
    fn filled(&self) -> bool {
        self.iter().all(|&cell| cell == Cell::Filled)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) enum Cell {
    Filled,
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum TetrominoValidity {
    Valid,
    Invalid,
}
