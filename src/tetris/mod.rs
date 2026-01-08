mod engine;
mod matrix;
mod next_queue;
mod player;
mod tetris;
mod tetromino;

use engine::{
    vectors::{Background, Color, FPSGraphic},
    window::Window,
    Raylib, RaylibBuilder,
};

use player::{Human, Player};
use tetris::Tetris;

use self::{engine::shapes::Vector2, player::TetrisMove};

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
            let action = player
                .next()
                .expect("Should be safe because [player.next] will never return [None]");
            self.tetris = self.tetris.update(self.raylib.window.frame_time(), action);

            self.raylib
                .window
                .canvas()
                .draw(&Background {
                    color: Color::RAY_WHITE,
                })
                .draw(&FPSGraphic {
                    position: Vector2 { x: 10.0, y: 10.0 },
                })
                .draw(&self.tetris);
        }
    }
}
