#[allow(dead_code)]
#[allow(unused)]
mod tetris;
use tetris::Game;

fn main() {
    let tetris = Game::new();
    tetris.start();
}
