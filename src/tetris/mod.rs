mod matrix;
mod next_queue;
mod tetromino;
mod util;

use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use crossterm_input::{input, AsyncReader, InputEvent, KeyEvent, RawScreen};
use crossterm_terminal::{ClearType, Terminal};

use matrix::{Cell, Matrix};
use next_queue::NextQueue;

use crate::tetris::matrix::TetrominoState;

use self::tetromino::{Rotation, Tetromino};

pub(super) struct Tetris {
    matrix: Matrix,
    next_queue: NextQueue,
    falling_tetromino: Tetromino,
}

impl Tetris {
    pub fn new() -> Self {
        let next_queue = NextQueue::new();

        Tetris {
            matrix: Matrix::new(),
            falling_tetromino: next_queue.current().new(),
            next_queue: next_queue,
        }
    }

    pub fn start(&mut self) {
        let mut total_time: Duration = Duration::ZERO;
        let mut previous_time = Instant::now();

        let mut terminal = Terminal::new();

        let _raw = RawScreen::into_raw_mode();
        let input = input();

        let mut reader = input.read_async();

        let (columns, rows) = terminal.size().unwrap(); // Safe because this shouldn't fail

        // terminal
        //     .set_size(PLAYFIELD_COLUMNS as u16, PLAYFIELD_ROWS as u16)
        //     .unwrap(); // Safe because this shouldn't fail.

        loop {
            let current_time = Instant::now();
            let delta_time = current_time.duration_since(previous_time);

            previous_time = current_time;

            let GameMessage::Continue = self.update(total_time, delta_time, &mut reader) else {
                break;
            };

            total_time += delta_time;

            self.render(&mut terminal);
        }

        terminal.set_size(columns, rows).unwrap(); // Safe because this shouldn't fail.
    }

    fn update(&mut self, total_time: Duration, delta_time: Duration, reader: &mut AsyncReader) -> GameMessage {
        let mut next_tetromino = self.falling_tetromino;

        if let Some(event) = reader.next() {
            match event {
                InputEvent::Keyboard(event) => match event {
                    KeyEvent::Up => {
                        next_tetromino = next_tetromino.rotate(Rotation::Clockwise);
                    }
                    KeyEvent::Ctrl(_) => {
                        next_tetromino = next_tetromino.rotate(Rotation::Counterclockwise);
                    }
                    KeyEvent::Esc => return GameMessage::Quit,
                    _ => {}
                },
                _ => {}
            }
        }

        if self.matrix.validate(&next_tetromino) == TetrominoState::Invalid {
            next_tetromino = self.falling_tetromino;
        }

        const CELL_FALL_PER_FRAME: f32 = 3.0;

        next_tetromino.center.row -= CELL_FALL_PER_FRAME * delta_time.as_secs_f32();

        if self.matrix.validate(&next_tetromino) == TetrominoState::Invalid {
            self.matrix = self.matrix.solidify(self.falling_tetromino);
            self.falling_tetromino = self.next_queue.next().unwrap().new();
        } else {
            self.falling_tetromino = next_tetromino;
        }

        GameMessage::Continue
    }

    fn render(&self, terminal: &mut Terminal) {
        // There's no reason why these two functions should fail, so I think it's safe to unwrap them.
        terminal.clear(ClearType::All).unwrap();
        terminal.write(self).unwrap();
    }
}

impl Display for Tetris {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut characterized = self.matrix.cells.map(|row| {
            row.map(|cell| match cell {
                Cell::Empty => '░',
                Cell::Filled => '█',
            })
        });

        for mino_position in self.falling_tetromino.to_position() {
            characterized[mino_position.0 as usize][mino_position.1 as usize] = '▓';
        }

        let stringified = characterized
            .iter()
            .rev()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", stringified)
    }
}

#[derive(PartialEq, Eq)]
enum GameMessage {
    Continue,
    Quit,
}
