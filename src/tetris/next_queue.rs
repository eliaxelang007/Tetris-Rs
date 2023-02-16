use super::tetromino::TetrominoType;
use strum::EnumCount;
use strum::IntoEnumIterator;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

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
                .unwrap(), // Safe because TetrominoType::iter() will always have the same length as TetrominoType::COUNT
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

#[test]
fn test_next_queue() {
    let mut next_queue = NextQueue::new();

    for _ in 0..255u8 {
        use std::collections::HashSet;

        let mut tetrominoes = HashSet::<TetrominoType>::with_capacity(TetrominoType::COUNT);

        for _ in 0..TetrominoType::COUNT {
            let previous_length = tetrominoes.len();
            tetrominoes.insert(next_queue.next().unwrap()); // Safe because next_queue.next() will never be Option::None.
            assert_ne!(previous_length, tetrominoes.len());
        }

        assert_eq!(tetrominoes.len(), TetrominoType::COUNT);
    }
}
