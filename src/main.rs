#![allow(dead_code, unused)]

mod tetris;
use tetris::{Human, Tetris};

fn main() {
    let mut tetris = Tetris::new();
    tetris.start::<Human>();
}
