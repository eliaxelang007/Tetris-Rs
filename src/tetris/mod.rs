mod matrix;
mod next_queue;
mod player;
mod raylib;
mod tetris;
mod tetromino;

use raylib::{
    drawing::{Background, Color},
    window::Window,
    Raylib, RaylibBuilder,
};

use player::{Human, Player};
use tetris::Tetris;

use self::player::TetrisMove;

pub struct Game {
    raylib: Raylib,
    tetris: Tetris,
}

impl Game {
    pub fn new() -> Self {
        const WINDOW_WIDTH: u16 = 1256;
        const WINDOW_HEIGHT: u16 = 942;

        Game {
            raylib: RaylibBuilder::new("Tetris in Rust", WINDOW_WIDTH, WINDOW_HEIGHT)
                .vsync()
                .build(),
            tetris: Tetris::new(),
        }
    }

    pub fn start(mut self) {
        let mut player = Human {
            input: &self.raylib.input,
        };

        while !self.raylib.window.should_close() {
            let action = player.next();
            self.tetris = self.tetris.update(self.raylib.window.frame_time(), action);

            self.raylib
                .window
                .canvas()
                .draw(&Background {
                    color: Color::RAY_WHITE,
                })
                .draw(&self.tetris);
        }
    }
}
