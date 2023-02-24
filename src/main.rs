#![allow(dead_code)]

mod tetris;

use tetris::{Human, Tetris};

fn main() {
    let tetris = Tetris::new();
    tetris.start::<Human>();
}
