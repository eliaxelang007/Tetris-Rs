use super::raylib::{
    drawing::{Canvas, Color, Drawable, RectangleGraphic},
    shapes::{Rectangle, Vector2},
};
use super::tetromino::TetrominoKind;

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use strum::{EnumCount, IntoEnumIterator};

#[derive(Clone)]
struct Bag {
    tetrominoes: [TetrominoKind; TetrominoKind::COUNT],
    bag_item_index: u8,
    randomizer: ThreadRng,
}

impl Bag {
    fn new() -> Self {
        let mut randomizer = thread_rng();

        Bag {
            tetrominoes: {
                let mut bag: [TetrominoKind; TetrominoKind::COUNT] = TetrominoKind::iter()
                    .collect::<Vec<TetrominoKind>>()
                    .try_into()
                    .expect("Should be safe because [TetrominoKind::iter] will always have the length of [TetrominoKind::COUNT]");

                bag.shuffle(&mut randomizer);

                bag
            },
            bag_item_index: 0,
            randomizer: randomizer,
        }
    }

    fn current(&self) -> TetrominoKind {
        self.tetrominoes[self.bag_item_index as usize]
    }
}

impl Iterator for Bag {
    type Item = TetrominoKind;

    fn next(&mut self) -> Option<Self::Item> {
        let current_piece = self.current();

        let tetromino_kind_count = TetrominoKind::COUNT as u8;

        if self.bag_item_index == tetromino_kind_count - 1 {
            self.tetrominoes.shuffle(&mut self.randomizer);
        }

        self.bag_item_index = (self.bag_item_index + 1) % tetromino_kind_count;

        Some(current_piece)
    }
}

pub(super) struct NextQueue<const SIZE: usize> {
    bag: Bag,
    upcoming: [TetrominoKind; SIZE],
    queue_item_index: u8,
}

impl<const SIZE: usize> NextQueue<SIZE> {
    pub(super) fn new() -> Self {
        let mut bag = Bag::new();

        NextQueue {
            upcoming: bag
                .clone()
                .take(SIZE)
                .collect::<Vec<TetrominoKind>>()
                .try_into()
                .expect("Should be safe because [Bag::next] will never return [None]"),
            bag: bag,
            queue_item_index: 0,
        }
    }

    pub(super) fn upcoming(&self) -> impl Iterator<Item = &TetrominoKind> {
        self.upcoming
            .iter()
            .cycle()
            .skip(self.queue_item_index.into())
            .take(SIZE)
    }
}

impl<const SIZE: usize> Iterator for NextQueue<SIZE> {
    type Item = TetrominoKind;

    fn next(&mut self) -> Option<Self::Item> {
        let queue_item_index = self.queue_item_index as usize;

        let next_piece = self.upcoming[queue_item_index];

        self.upcoming[queue_item_index] = self
            .bag
            .next()
            .expect("Should be safe because [self.bag.next] will never return [None]");
        self.queue_item_index = (self.queue_item_index + 1) % (SIZE as u8);

        Some(next_piece)
    }
}

impl<'a, const SIZE: usize> Drawable<'a> for NextQueue<SIZE> {
    fn draw(&self, canvas: Canvas<'a>) -> Canvas<'a> {
        canvas.draw(&RectangleGraphic {
            rectangle: Rectangle {
                size: Vector2 { x: 232.0, y: 712.0 },
            },
            position: Vector2 { x: 883.0, y: 21.0 },
            color: Color::MAROON,
        })
    }
}

// impl<const SIZE: usize> Display for NextQueue<SIZE> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "{:?}", self.upcoming().collect::<Vec<&TetrominoKind>>())
//     }
// }
