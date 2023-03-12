use std::process::exit;

use super::tetromino::{Rotation, Shifter};

pub trait Constructable {
    fn new() -> Self;
}

use crossterm_input::{input, AsyncReader, InputEvent, KeyEvent, RawScreen};

pub enum TetrisMove {
    Rotate(Rotation),
    HardDrop,
    SoftDrop,
    Shift(Shifter),
}

pub enum GameAction {
    Gameplay(TetrisMove),
    Exit,
}

pub struct Human {
    _raw: RawScreen,
    reader: AsyncReader,
}

impl Human {
    pub(super) fn new() -> Self {
        Human {
            _raw: RawScreen::into_raw_mode().unwrap(), // This function failing is catastrophic, so we can unwrap.
            reader: input().read_async(),
        }
    }
}

impl Constructable for Human {
    fn new() -> Self {
        Self::new()
    }
}

impl Iterator for Human {
    type Item = GameAction;

    fn next(&mut self) -> Option<Self::Item> {
        use GameAction::*;
        use InputEvent::*;
        use KeyEvent::*;
        use Rotation::*;
        use TetrisMove::*;

        match self.reader.next() {
            Some(input_details) => match input_details {
                Keyboard(key_event) => match key_event {
                    Right => Some(Gameplay(Shift(Shifter::Right))),
                    Left => Some(Gameplay(Shift(Shifter::Left))),
                    Up => Some(Gameplay(Rotate(Clockwise))),
                    Char('z') => Some(Gameplay(Rotate(Counterclockwise))),
                    Down => Some(Gameplay(SoftDrop)),
                    Char(' ') => Some(Gameplay(HardDrop)),
                    Esc => Some(Exit),
                    _ => None,
                },
                _ => None,
            },
            None => None,
        }
    }
}

pub trait Player: Iterator<Item = GameAction> {}
impl<T: Iterator<Item = GameAction>> Player for T {}
