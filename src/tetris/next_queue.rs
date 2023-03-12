use super::tetromino::TetrominoType;

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};
use strum::{EnumCount, IntoEnumIterator};

pub(super) struct NextQueue {
    bag: [TetrominoType; TetrominoType::COUNT],
    bag_item_index: u8,
    randomizer: ThreadRng,
}

impl NextQueue {
    pub(super) fn new() -> Self {
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

    pub(super) fn current(&self) -> TetrominoType {
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
