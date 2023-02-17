use super::tetromino::{Position, Tetromino};

pub(super) const PLAYFIELD_ROWS: usize = 20;
pub(super) const PLAYFIELD_COLUMNS: usize = 10;

pub(super) struct Matrix {
    pub(super) cells: [[Cell; PLAYFIELD_COLUMNS]; PLAYFIELD_ROWS],
}

impl Matrix {
    pub(super) fn new() -> Self {
        Matrix {
            cells: [[Cell::Empty; PLAYFIELD_COLUMNS]; PLAYFIELD_ROWS],
        }
    }

    pub(super) fn solidify(&self, tetromino: Tetromino) -> Self {
        let mut copy = self.cells.clone();

        for position in tetromino.to_position() {
            copy[position.0 as usize][position.1 as usize] = Cell::Filled;
        }

        Matrix { cells: copy }
    }

    pub(super) fn validate(&self, tetromino: &Tetromino) -> TetrominoState {
        match tetromino.to_position().iter().all(|(row, column)| {
            (0..(PLAYFIELD_ROWS as i32)).contains(&row)
                && (0..(PLAYFIELD_COLUMNS as i32)).contains(&column)
                && self.cells[*row as usize][*column as usize] != Cell::Filled
        }) {
            true => TetrominoState::Valid,
            false => TetrominoState::Invalid,
        }
    }
}

#[derive(Eq, PartialEq)]
pub(super) enum TetrominoState {
    Valid,
    Invalid,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(super) enum Cell {
    Filled,
    Empty,
}
