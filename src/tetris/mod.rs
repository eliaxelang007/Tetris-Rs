mod matrix;
mod player;
mod tetromino;
mod util;

pub use player::Human;

use self::{player::Player, util::Constructable};

pub struct Tetris {}

impl Tetris {
    pub fn new() -> Self {
        Tetris {}
    }
    pub fn start<T: Player + Constructable>(&self) {
        let _player = T::new();
    }
}
