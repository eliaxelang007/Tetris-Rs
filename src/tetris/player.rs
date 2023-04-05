use std::process::exit;

use super::raylib::input::{Input, KeyboardKey};
use super::tetromino::{Rotation, Shifter};

#[derive(Clone)]
pub enum TetrisMove {
    Rotate(Rotation),
    HardDrop,
    SoftDrop,
    Shift(Shifter),
}

pub(super) struct Human<'a> {
    pub(super) input: &'a Input,
}

// impl<'a> Human<'a> {
//     pub() fn new(input: &'a Input) -> Self {
//         Human { input }
//     }
// }

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
        .find(|(key, _)| self.input.key_down(*key))
        .map(|(_, action)| action.clone())
    }
}

pub(super) trait Player: Iterator<Item = TetrisMove> {}
impl<T: Iterator<Item = TetrisMove>> Player for T {}
