#![allow(dead_code, unused)]

mod tetris;
use tetris::Game;

fn main() {
    let mut tetris = Game::new();
    tetris.start();
}
