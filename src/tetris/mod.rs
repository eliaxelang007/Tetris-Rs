mod matrix;
mod next_queue;
mod tetromino;
mod util;

use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use matrix::{Cell, Matrix};
use next_queue::NextQueue;

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

        self.falling_tetromino = self.falling_tetromino.rotate(Rotation::Clockwise);

        for _ in 0..5 {
            let current_time = Instant::now();
            let delta_time = current_time.duration_since(previous_time);

            previous_time = current_time;

            self.update(total_time, delta_time);

            total_time += delta_time;

            self.render();
        }
    }

    fn update(&mut self, total_time: Duration, delta_time: Duration) {
        self.falling_tetromino = self.falling_tetromino.rotate(Rotation::Counterclockwise);
    }
    fn render(&self) {
        println!("{}\n", self);
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
            characterized[mino_position.row as usize][mino_position.column as usize] = '▓';
        }

        let stringified = characterized
            .iter()
            .rev()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}: {:?}", stringified, self.next_queue.current())
    }
}
