#![allow(dead_code, unused)]

use std::fmt::{Debug, Display, Formatter, Result};

const PLAYFIELD_COLUMNS: usize = 10;
const PLAYFIELD_ROWS: usize = 20;

struct Matrix {
    cells: [[Cell; PLAYFIELD_COLUMNS]; PLAYFIELD_ROWS],
}

#[derive(Debug, Clone, Copy)]
struct Tetromino {
    center: Center,
    minoes: [Mino; 4],
}

#[derive(Debug, Clone, Copy)]
struct Mino {
    x_to_center: HalfStep,
    y_to_center: HalfStep,
}

#[derive(Debug, Clone, Copy)]
struct Center {
    row: f32,
    column: HalfStep,
}

struct Coordinate {
    row: i8,
    column: i8,
}

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Clockwise,
    Counterclockwise,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Filled,
    Empty,
}

use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(EnumCount, EnumIter, Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum TetrominoType {
    O,
    I,
    T,
    L,
    J,
    S,
    Z,
}

#[derive(Debug, PartialEq, Eq)]
enum TetrominoValidity {
    Valid,
    Invalid,
}

#[derive(Clone, Copy)]
struct HalfStep(i8);

impl Matrix {
    fn new() -> Self {
        Matrix {
            cells: [[Cell::Empty; PLAYFIELD_COLUMNS]; PLAYFIELD_ROWS],
        }
    }

    fn validate(&self, tetromino: &Tetromino) -> TetrominoValidity {
        use Cell::Filled;
        use TetrominoValidity::*;

        match tetromino.put_on_matrix().iter().all(|Coordinate { row, column }| {
            (0..(PLAYFIELD_ROWS as i8)).contains(&row)
                && (0..(PLAYFIELD_COLUMNS as i8)).contains(&column)
                && self.cells[*row as usize][*column as usize] != Filled
        }) {
            true => Valid,
            false => Invalid,
        }
    }

    fn solidify(&self, tetromino: Tetromino) -> Self {
        let mut copy = self.cells.clone();

        for position in tetromino.put_on_matrix() {
            copy[position.row as usize][position.column as usize] = Cell::Filled;
        }

        Matrix { cells: copy }
    }
}

impl Tetromino {
    fn rotate(&self, rotation: Rotation) -> Self {
        Tetromino {
            center: self.center,
            minoes: self.minoes.map(|mino| mino.rotate(rotation)),
        }
    }

    fn put_on_matrix(&self) -> [Coordinate; 4] {
        self.minoes.map(|mino| Coordinate {
            row: (self.center.row + f32::from(mino.y_to_center)) as i8,
            column: (f32::from(self.center.column) + f32::from(mino.x_to_center)) as i8,
        })
    }
}

impl Mino {
    fn rotate(&self, rotation: Rotation) -> Self {
        const NINETY_DEG_COS: f32 = -0.00000004371138828673792886547744274139404296875;
        const NINETY_DEG_SIN: f32 = 1.0;

        let rotation_multiplier = rotation.rotation_multiplier();

        let ninety_deg_cos = NINETY_DEG_COS * rotation_multiplier as f32;
        let ninety_deg_sin = NINETY_DEG_SIN * rotation_multiplier as f32;

        let x_to_center: f32 = self.x_to_center.into();
        let y_to_center: f32 = self.y_to_center.into();

        Mino {
            x_to_center: (x_to_center * ninety_deg_cos - y_to_center * ninety_deg_sin)
                .round()
                .into(),
            y_to_center: (x_to_center * ninety_deg_sin + y_to_center * ninety_deg_cos)
                .round()
                .into(),
        }
    }
}

impl TetrominoType {
    fn new(&self) -> Tetromino {
        use TetrominoType::*;

        match self {
            O => Tetromino {
                center: Center::new(19.5 - 1.0, 5.5 - 1.0),
                minoes: [
                    Mino::new(-0.5, 0.5),
                    Mino::new(0.5, 0.5),
                    Mino::new(0.5, -0.5),
                    Mino::new(-0.5, -0.5),
                ],
            },
            I => Tetromino {
                center: Center::new(18.5 - 1.0, 5.5 - 1.0),
                minoes: [
                    Mino::new(-1.5, 0.5),
                    Mino::new(-0.5, 0.5),
                    Mino::new(0.5, 0.5),
                    Mino::new(1.5, 0.5),
                ],
            },
            T => Tetromino {
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(0.0, 1.0),
                    Mino::new(1.0, 0.0),
                ],
            },
            L => Tetromino {
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(1.0, 0.0),
                    Mino::new(1.0, 1.0),
                ],
            },
            J => Tetromino {
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(1.0, 0.0),
                    Mino::new(-1.0, 1.0),
                ],
            },
            S => Tetromino {
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(-1.0, 0.0),
                    Mino::new(0.0, 1.0),
                    Mino::new(1.0, 1.0),
                ],
            },
            Z => Tetromino {
                center: Center::new(19.0 - 1.0, 5.0 - 1.0),
                minoes: [
                    Mino::new(0.0, 0.0),
                    Mino::new(0.0, 1.0),
                    Mino::new(-1.0, 1.0),
                    Mino::new(1.0, 0.0),
                ],
            },
        }
    }
}

impl Mino {
    fn new(x_to_center: f32, y_to_center: f32) -> Self {
        Mino {
            x_to_center: x_to_center.into(),
            y_to_center: y_to_center.into(),
        }
    }
}

impl Center {
    fn new(row: f32, column: f32) -> Self {
        Center {
            row,
            column: column.into(),
        }
    }
}

impl Rotation {
    const fn rotation_multiplier(&self) -> i8 {
        match self {
            Rotation::Clockwise => -1,
            Rotation::Counterclockwise => 1,
        }
    }
}

impl Debug for HalfStep {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "HalfStep({})", f32::from(self.clone()))
    }
}

impl From<f32> for HalfStep {
    fn from(half_stepped: f32) -> Self {
        HalfStep((half_stepped * 2.0) as i8)
    }
}

impl From<HalfStep> for f32 {
    fn from(half_step: HalfStep) -> Self {
        (half_step.0 as f32) / 2.0
    }
}

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

struct NextQueue {
    bag: [TetrominoType; TetrominoType::COUNT],
    bag_item_index: u8,
    randomizer: ThreadRng,
}

impl NextQueue {
    fn new() -> Self {
        let mut this = NextQueue {
            bag: TetrominoType::iter()
                .collect::<Vec<TetrominoType>>()
                .try_into()
                .unwrap(), // Safe because TetrominoType::iter() will always have the length of TetrominoType::COUNT
            bag_item_index: 0,
            randomizer: thread_rng(),
        };

        this.bag.shuffle(&mut this.randomizer);

        this
    }

    fn current(&self) -> TetrominoType {
        self.bag[self.bag_item_index as usize]
    }
}

impl Iterator for NextQueue {
    type Item = TetrominoType;

    fn next(&mut self) -> Option<Self::Item> {
        let current_piece = self.current();

        if self.bag_item_index == (TetrominoType::COUNT - 1) as u8 {
            self.bag.shuffle(&mut self.randomizer);
        }

        self.bag_item_index = (self.bag_item_index + 1) % (TetrominoType::COUNT as u8);

        Some(current_piece)
    }
}

pub trait Constructable {
    fn new() -> Self;
}

use crossterm_input::{input, AsyncReader, InputEvent, KeyEvent, RawScreen};

enum TetrisMove {
    Rotate(Rotation),
    HardDrop,
    SoftDrop,
    MoveLeft,
    MoveRight,
}

enum GameAction {
    Gameplay(TetrisMove),
    Exit,
}

struct Human {
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
        use GameAction::*;
        use InputEvent::*;
        use KeyEvent::*;
        use Rotation::*;
        use TetrisMove::*;

        match self.reader.next() {
            Some(input_details) => match input_details {
                Keyboard(key_event) => match key_event {
                    Up => Some(Gameplay(Rotate(Clockwise))),
                    Ctrl(_) => Some(Gameplay(Rotate(Counterclockwise))),
                    Esc => Some(Exit),
                    _ => None,
                },
                _ => None,
            },
            None => None,
        }
    }
}

trait Player: Iterator<Item = GameAction> {}
impl<T: Iterator<Item = GameAction>> Player for T {}

struct Tetris {
    matrix: Matrix,
    falling_tetromino: Tetromino,
    next_queue: NextQueue,
}

use crossterm_terminal::{ClearType, Terminal};
use std::time::{Duration, Instant};

impl Tetris {
    fn new() -> Self {
        let mut next_queue = NextQueue::new();

        Tetris {
            matrix: Matrix::new(),
            falling_tetromino: next_queue.next().unwrap().new(),
            next_queue: next_queue,
        }
    }

    fn start<T: Player + Constructable>(&mut self) {
        let mut previous_time = Instant::now();

        let mut terminal = Terminal::new();

        let mut player = T::new();

        // let (columns, rows) = terminal.size().unwrap(); // Safe because this shouldn't fail

        // terminal
        //     .set_size(PLAYFIELD_COLUMNS as u16, PLAYFIELD_ROWS as u16)
        //     .unwrap(); // Safe because this shouldn't fail.

        loop {
            let current_time = Instant::now();
            let delta_time = current_time.duration_since(previous_time);

            previous_time = current_time;

            use GameAction::*;

            self.update(
                delta_time,
                match player.next() {
                    Some(action) => match action {
                        Gameplay(tetris_move) => Some(tetris_move),
                        Exit => {
                            break;
                        }
                    },
                    None => None,
                },
            );

            self.render(&mut terminal);
        }

        //terminal.set_size(columns, rows).unwrap(); // Safe because this shouldn't fail.
    }

    fn update(&mut self, delta_time: Duration, tetris_move: Option<TetrisMove>) {
        let mut updated_tetromino = self.falling_tetromino;

        let mut cell_fall_per_frame: f32 = 3.0;

        if let Some(tetris_move) = tetris_move {
            use TetrisMove::*;

            match tetris_move {
                Rotate(rotation) => {
                    updated_tetromino = updated_tetromino.rotate(rotation);
                }

                SoftDrop => {
                    cell_fall_per_frame *= 5.0;
                }

                _ => {}
            }
        }

        if self.matrix.validate(&updated_tetromino) == TetrominoValidity::Invalid {
            updated_tetromino = self.falling_tetromino;
        }

        updated_tetromino.center.row -= cell_fall_per_frame * delta_time.as_secs_f32();

        if self.matrix.validate(&updated_tetromino) == TetrominoValidity::Invalid {
            self.matrix = self.matrix.solidify(self.falling_tetromino);
            self.falling_tetromino = self.next_queue.next().unwrap().new();
        } else {
            self.falling_tetromino = updated_tetromino;
        }
    }

    fn render(&self, terminal: &mut Terminal) {
        // There's no reason why these two functions should fail, so I think it's safe to unwrap them.
        terminal.clear(ClearType::All).unwrap();
        terminal.write(self).unwrap();
    }
}

impl Display for Tetris {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut characterized = self.matrix.cells.map(|row| {
            row.map(|cell| match cell {
                Cell::Empty => '░',
                Cell::Filled => '█',
            })
        });

        for snapped in self.falling_tetromino.put_on_matrix() {
            characterized[snapped.row as usize][snapped.column as usize] = '▓';
        }

        let stringified = characterized
            .iter()
            .rev()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", stringified)
    }
}

fn main() {
    let mut tetris = Tetris::new();
    tetris.start::<Human>();
}
