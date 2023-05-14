use std::process::exit;

use super::raylib::input::{Input, KeyboardKey};
use super::tetromino::{Rotation, Step};

#[derive(Clone)]
pub enum TetrisMove {
    Rotate(Rotation),
    HardDrop,
    SoftDrop,
    Shift(Step),
}

pub(super) struct Human<'a> {
    pub(super) input: &'a Input,
}

// impl<'a> Human<'a> {
//     pub() fn new(input: &'a Input) -> Self {
//         Human { input }
//     }
// }

pub(super) type Moves = Vec<TetrisMove>;

impl Iterator for Human<'_> {
    type Item = Moves;

    fn next(&mut self) -> Option<Self::Item> {
        use KeyboardKey::*;
        use Rotation::*;
        use TetrisMove::*;

        let key_down: Box<dyn Fn(KeyboardKey) -> bool> = Box::new(|key| self.input.key_down(key));
        let key_pressed: Box<dyn Fn(KeyboardKey) -> bool> = Box::new(|key| self.input.key_pressed(key));

        Some(
            [
                (&key_down, KEY_RIGHT, Shift(Step::Right)),
                (&key_down, KEY_LEFT, Shift(Step::Left)),
                (&key_pressed, KEY_UP, Rotate(Clockwise)),
                (&key_pressed, KEY_LEFT_CONTROL, Rotate(Counterclockwise)),
                (&key_down, KEY_DOWN, SoftDrop),
                (&key_pressed, KEY_SPACE, HardDrop),
            ]
            .iter()
            .filter_map(|(reader, key, action)| if reader(*key) { Some(action.clone()) } else { None })
            .collect(),
        )
    }
}

pub(super) trait Player: Iterator<Item = TetrisMove> {}
impl<T: Iterator<Item = TetrisMove>> Player for T {}
