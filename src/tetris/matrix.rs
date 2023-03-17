use super::tetromino::{Snapped, Tetromino};
use std::fmt::{Debug, Display, Formatter, Result};

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

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            self.cells
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|cell| match cell {
                            Cell::Empty => '░',
                            Cell::Filled => '█',
                        })
                        .collect::<String>()
                })
                .rev()
                .collect::<Vec<String>>()
                .join("\n")
        )
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
