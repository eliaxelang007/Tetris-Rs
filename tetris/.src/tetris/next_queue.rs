use super::tetromino::TetrominoType;

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use strum::{EnumCount, IntoEnumIterator};

#[derive(Clone)]
struct Bag {
    tetrominoes: [TetrominoType; TetrominoType::COUNT],
    bag_item_index: u8,
    randomizer: ThreadRng,
}

impl Bag {
    fn new() -> Self {
        let mut randomizer = thread_rng();

        Bag {
            tetrominoes: {
                let mut bag: [TetrominoType; TetrominoType::COUNT] = TetrominoType::iter()
                    .collect::<Vec<TetrominoType>>()
                    .try_into()
                    .unwrap(); // Safe because [TetrominoType::iter] will always have the length of [TetrominoType::COUNT]

                bag.shuffle(&mut randomizer);

                bag
            },
            bag_item_index: 0,
            randomizer: randomizer,
        }
    }

    fn current(&self) -> TetrominoType {
        self.tetrominoes[self.bag_item_index as usize]
    }
}

impl Iterator for Bag {
    type Item = TetrominoType;

    fn next(&mut self) -> Option<Self::Item> {
        let current_piece = self.current();

        let tetromino_type_count = TetrominoType::COUNT as u8;

        if self.bag_item_index == tetromino_type_count - 1 {
            self.tetrominoes.shuffle(&mut self.randomizer);
        }

        self.bag_item_index = (self.bag_item_index + 1) % tetromino_type_count;

        Some(current_piece)
    }
}

pub(super) struct NextQueue<const SIZE: usize> {
    bag: Bag,
    upcoming: [TetrominoType; SIZE],
    queue_item_index: u8,
}

impl<const SIZE: usize> NextQueue<SIZE> {
    pub(super) fn new() -> Self {
        let mut bag = Bag::new();

        NextQueue {
            upcoming: bag
                .clone()
                .take(SIZE)
                .collect::<Vec<TetrominoType>>()
                .try_into()
                .unwrap(), // Safe because [Bag::next] will never return [None]
            bag: bag,
            queue_item_index: 0,
        }
    }

    pub(super) fn upcoming(&self) -> impl Iterator<Item = &TetrominoType> {
        self.upcoming
            .iter()
            .cycle()
            .skip(self.queue_item_index.into())
            .take(SIZE)
    }
}

impl<const SIZE: usize> Iterator for NextQueue<SIZE> {
    type Item = TetrominoType;

    fn next(&mut self) -> Option<Self::Item> {
        let queue_item_index = self.queue_item_index as usize;

        let next_piece = self.upcoming[queue_item_index];

        self.upcoming[queue_item_index] = self.bag.next().unwrap(); // Safe because [self.bag.next] will never return None
        self.queue_item_index = (self.queue_item_index + 1) % (SIZE as u8);

        Some(next_piece)
    }
}

// impl<const SIZE: usize> Display for NextQueue<SIZE> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "{:?}", self.upcoming().collect::<Vec<&TetrominoType>>())
//     }
// }
