use std::process::exit;

use super::tetromino::{Rotation, Shifter};
use raylib::prelude::{KeyboardKey, RaylibHandle};

#[derive(Clone)]
pub enum TetrisMove {
    Rotate(Rotation),
    HardDrop,
    SoftDrop,
    Shift(Shifter),
}

pub struct Human<'a> {
    engine: &'a RaylibHandle,
}

impl Iterator for Human<'_> {
    type Item = TetrisMove;

    fn next(&mut self) -> Option<Self::Item> {
        use KeyboardKey::*;
        use Rotation::*;
        use TetrisMove::*;

        [
            (KEY_RIGHT, Shift(Shifter::Right)),
            (KEY_LEFT, Shift(Shifter::Left)),
            (KEY_UP, Rotate(Clockwise)),
            (KEY_Z, Rotate(Counterclockwise)),
            (KEY_DOWN, SoftDrop),
            (KEY_SPACE, HardDrop),
        ]
        .iter()
        .find(|(key, _)| self.engine.is_key_down(*key))
        .map(|(_, action)| action.clone())
    }
}

pub trait Player: Iterator<Item = TetrisMove> {}
impl<T: Iterator<Item = TetrisMove>> Player for T {}
