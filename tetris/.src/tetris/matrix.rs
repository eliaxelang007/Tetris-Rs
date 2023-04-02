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

    pub(super) fn solidify(mut self, tetromino: Tetromino) -> Self {
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

use super::graphics::Drawable;
use raylib::prelude::{color::Color, RaylibDraw, RaylibDrawHandle};

impl Drawable for Matrix {
    fn draw(&self, canvas: &mut RaylibDrawHandle) {
        canvas.draw_rectangle(0, 0, 100, 100, Color::WHITE);
    }
}

// impl Display for Matrix {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(
//             f,
//             "{}",
//             self.cells
//                 .iter()
//                 .rev()
//                 .map(|row| {
//                     row.iter()
//                         .map(|cell| match cell {
//                             Cell::Empty => '░',
//                             Cell::Filled => '█',
//                         })
//                         .collect::<String>()
//                 })
//                 .collect::<Vec<String>>()
//                 .join("\n")
//         )
//     }
// }

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
