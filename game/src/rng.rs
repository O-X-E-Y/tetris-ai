use nanorand::{Rng as _, WyRand};

use crate::pieces::Piece;

pub trait Rng {
    fn init() -> Self;

    fn next(&mut self) -> Piece;
}

/// Not actually classic... But close enough
#[derive(Debug)]
pub struct ClassicRng {
    rng: WyRand,
    current: Piece,
}

impl Rng for ClassicRng {
    fn init() -> Self {
        Self {
            rng: WyRand::new(),
            current: Piece::random(),
        }
    }

    fn next(&mut self) -> Piece {
        use Piece::*;

        let res = match self.rng.generate_range(0u8..8) {
            0 if self.current != I => I,
            1 if self.current != L => L,
            2 if self.current != J => J,
            3 if self.current != O => O,
            4 if self.current != T => T,
            5 if self.current != S => S,
            6 if self.current != Z => Z,
            _ => match self.rng.generate_range(0u8..7) {
                0 => I,
                1 => L,
                2 => J,
                3 => O,
                4 => T,
                5 => S,
                6 => Z,
                _ => unreachable!("Range is between 0 and 7, anything else is not possible"),
            },
        };

        self.current = res;

        res
    }
}

#[derive(Debug)]
pub struct SevenBag {
    rng: WyRand,
    current: usize,
    bag: [Piece; 7],
}

impl SevenBag {
    pub fn update_bag(&mut self) {
        self.rng.shuffle(&mut self.bag);
    }
}

impl Rng for SevenBag {
    fn init() -> Self {
        let current = 0;
        let mut rng = WyRand::new();
        let mut bag = Piece::PIECES;
        rng.shuffle(&mut bag);

        Self { current, rng, bag }
    }

    fn next(&mut self) -> Piece {
        let res = self.bag[self.current];
        self.current += 1;

        if self.current % 7 == 0 {
            self.current = 0;
            self.update_bag();
        }

        res
    }
}

#[derive(Debug)]
pub struct OrderedRng {
    index: usize,
    bag: [Piece; 7],
}

impl Rng for OrderedRng {
    fn init() -> Self {
        Self {
            index: 0,
            bag: Piece::PIECES,
        }
    }

    fn next(&mut self) -> Piece {
        let piece = self.bag[self.index];
        self.index = (self.index + 1) % 7;
        piece
    }
}
