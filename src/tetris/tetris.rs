use super::raylib::{
    drawing::{Background, Canvas, Color, Drawable, RectangleGraphic},
    shapes::{Rectangle, Vector2},
};

use super::matrix::{Matrix, TetrominoValidity, PLAYFIELD_COLUMNS, PLAYFIELD_ROWS};
use super::next_queue::NextQueue;
use super::player::{Moves, Player, TetrisMove};
use super::tetromino::{Tetromino, TetrominoKind};

use std::time::Duration;

pub(super) struct Tetris {
    matrix: Matrix,
    falling_tetromino: Tetromino,
    next_queue: NextQueue<5>,
}

impl Tetris {
    pub(super) fn new() -> Self {
        let mut next_queue = NextQueue::new();

        Tetris {
            matrix: Matrix::new(),
            falling_tetromino: next_queue
                .next()
                .expect("Should be safe because [NextQueue::next] will never return [None]")
                .new(),
            next_queue: next_queue,
        }
    }

    pub(super) fn update(mut self, delta_time: Duration, actions: Moves) -> Self {
        let previous_tetromino = self.falling_tetromino.clone();

        let mut fall_speed: f32 = 2.0;
        let shift_speed: f32 = 13.0;

        for tetris_move in actions {
            match tetris_move {
                TetrisMove::Rotate(rotation) => {
                    self.falling_tetromino = self.falling_tetromino.rotate(rotation);
                }

                TetrisMove::Shift(step) => {
                    self.falling_tetromino = self.falling_tetromino.shift(step, shift_speed, delta_time);
                }

                TetrisMove::SoftDrop => {
                    fall_speed *= 10.0;
                }

                _ => {}
            }
        }

        if self.matrix.validate(&self.falling_tetromino) == TetrominoValidity::Invalid {
            self.falling_tetromino = previous_tetromino;
        }

        let previous_tetromino = self.falling_tetromino.clone();

        self.falling_tetromino = self.falling_tetromino.fall(fall_speed, delta_time);

        if self.matrix.validate(&self.falling_tetromino) == TetrominoValidity::Invalid {
            self.matrix = self.matrix.solidify(&previous_tetromino).clear_lines();
            self.falling_tetromino = self.next_queue.next().unwrap().new();
        }

        self
    }
}

impl<'a> Drawable<'a> for Tetris {
    fn draw(&self, canvas: Canvas<'a>) -> Canvas<'a> {
        let matrix = self.matrix.clone().solidify(&self.falling_tetromino);
        canvas.draw(&matrix).draw(&self.next_queue)
    }
}
