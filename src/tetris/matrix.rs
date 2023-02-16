use super::tetromino::Tetromino;

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
            copy[position.row as usize][position.column as usize] = Cell::Filled;
        }

        Matrix { cells: copy }
    }
}

#[derive(Copy, Clone)]
pub(super) enum Cell {
    Filled,
    Empty,
}
