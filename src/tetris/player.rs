use super::util::Constructable;
use crossterm_input::{input, AsyncReader, InputEvent, KeyEvent, RawScreen};

pub enum TetrisMove {
    RotateClockwise,
    RotateCounterclockwise,
    HardDrop,
    SoftDrop,
    MoveLeft,
    MoveRight,
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
    fn new() -> Self {
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
        match self.reader.next() {
            Some(input_details) => match input_details {
                InputEvent::Keyboard(key_event) => match key_event {
                    KeyEvent::Up => Some(GameAction::Gameplay(TetrisMove::RotateClockwise)),
                    KeyEvent::Ctrl(_) => Some(GameAction::Gameplay(TetrisMove::RotateCounterclockwise)),
                    KeyEvent::Esc => Some(GameAction::Exit),
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
