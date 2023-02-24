const PLAYFIELD_ROWS: usize = 20;
const PLAYFIELD_COLUMNS: usize = 10;

pub(super) struct Matrix {
    cells: [[Cell; PLAYFIELD_COLUMNS]; PLAYFIELD_ROWS],
}

impl Matrix {
    pub(super) fn new() -> Self {
        Matrix {
            cells: [[Cell::Empty; PLAYFIELD_COLUMNS]; PLAYFIELD_ROWS],
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(super) enum Cell {
    Filled,
    Empty,
}
