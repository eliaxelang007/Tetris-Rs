#![allow(dead_code, unused_variables, unused_assignments)]
mod tetris;
use tetris::Tetris;

fn main() {
    let mut game = Tetris::new();
    game.start();
}
