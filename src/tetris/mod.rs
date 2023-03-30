mod matrix;
mod next_queue;
mod player;
mod tetromino;

use std::{
    fmt::{Debug, Display, Formatter, Result},
    process::exit,
};

use matrix::{Matrix, TetrominoValidity, PLAYFIELD_COLUMNS, PLAYFIELD_ROWS};
use next_queue::NextQueue;
use player::{Constructable, GameAction, Player, TetrisMove};
use tetromino::Tetromino;

pub use player::Human;

pub struct Tetris {
    matrix: Matrix,
    falling_tetromino: Tetromino,
    next_queue: NextQueue,
}

use crossterm_terminal::{ClearType, Terminal};
use std::time::{Duration, Instant};

impl Tetris {
    pub fn new() -> Self {
        let mut next_queue = NextQueue::new();

        Tetris {
            matrix: Matrix::new(),
            falling_tetromino: next_queue.next().unwrap().new(),
            next_queue: next_queue,
        }
    }

    pub fn start<T: Player + Constructable>(mut self) {
        let mut previous_time = Instant::now();

        let mut terminal = Terminal::new();

        let mut player = T::new();

        let (columns, rows) = terminal.size().unwrap(); // Safe because this shouldn't fail

        terminal
            .set_size((PLAYFIELD_COLUMNS as u16), (PLAYFIELD_ROWS as u16))
            .unwrap(); // Safe because this shouldn't fail

        loop {
            let current_time = Instant::now();
            let delta_time = current_time.duration_since(previous_time);

            previous_time = current_time;

            self = self.update(
                delta_time,
                match player.next() {
                    Some(action) => match action {
                        GameAction::Gameplay(tetris_move) => Some(tetris_move),
                        GameAction::Exit => {
                            break;
                        }
                    },
                    None => None,
                },
            );

            self.render(&mut terminal);
        }

        terminal.set_size(columns, rows).unwrap(); // Safe because this shouldn't fail
    }

    fn update(mut self, delta_time: Duration, tetris_move: Option<TetrisMove>) -> Self {
        let previous_tetromino = self.falling_tetromino.clone();

        let mut cell_fall_per_frame: f32 = 3.0;

        if let Some(tetris_move) = tetris_move {
            match tetris_move {
                TetrisMove::Rotate(rotation) => {
                    self.falling_tetromino = self.falling_tetromino.rotate(rotation);
                }

                TetrisMove::Shift(shifter) => {
                    self.falling_tetromino = self.falling_tetromino.shift(shifter);
                }

                TetrisMove::SoftDrop => {
                    cell_fall_per_frame *= 20.0;
                }

                _ => {}
            }
        }

        if self.matrix.validate(&self.falling_tetromino) == TetrominoValidity::Invalid {
            self.falling_tetromino = previous_tetromino;
        }

        let previous_tetromino = self.falling_tetromino.clone();

        self.falling_tetromino = self.falling_tetromino.fall(cell_fall_per_frame, delta_time);

        if self.matrix.validate(&self.falling_tetromino) == TetrominoValidity::Invalid {
            self.matrix = self.matrix.solidify(previous_tetromino).clear_lines();

            self.falling_tetromino = self.next_queue.next().unwrap().new();
        }

        self
    }

    fn render(&self, terminal: &mut Terminal) {
        // There's no reason why these two functions should fail, so I think it's safe to unwrap them.
        terminal.write(self).unwrap();
        terminal.clear(ClearType::All).unwrap();
    }
}

impl Display for Tetris {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.matrix.clone().solidify(self.falling_tetromino.clone()))
    }
}
