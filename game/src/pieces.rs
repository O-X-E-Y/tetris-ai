use nanorand::{Rng, WyRand};

use crate::{board::Pos, PiecePos};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    Right = 0b1,
    Down = 0b10,
    Left = 0b100,
    Up = 0b1000,
}

use Rotation as R;

impl Rotation {
    pub fn cw(&mut self) {
        match self {
            R::Right => *self = R::Down,
            R::Down => *self = R::Left,
            R::Left => *self = R::Up,
            R::Up => *self = R::Right,
        }
    }

    pub fn ccw(&mut self) {
        match self {
            R::Right => *self = R::Up,
            R::Down => *self = R::Right,
            R::Left => *self = R::Down,
            R::Up => *self = R::Left,
        }
    }

    pub fn flip(&mut self) {
        match self {
            R::Right => *self = R::Left,
            R::Down => *self = R::Up,
            R::Left => *self = R::Right,
            R::Up => *self = R::Down,
        }
    }

    // pub fn rotated(self, input: RotationInput) -> Self {
    //     use Rotation::*;
    //     use RotationInput as RI;

    //     match (self, input) {
    //         (Right, RI::CW) => Up,
    //         (Right, RI::CCW) => Down,
    //         (Down, RI::CW) => Right,
    //         (Down, RI::CCW) => Left,
    //         (Left, RI::CW) => Down,
    //         (Left, RI::CCW) => Up,
    //         (Up, RI::CW) => Left,
    //         (Up, RI::CCW) => Right,
    //     }
    // }

    // pub fn rotate(&mut self, input: RotationInput) {
    //     use Rotation::*;
    //     use RotationInput as RI;

    //     match (&self, input) {
    //         (Right, RI::CCW) => *self = Up,
    //         (Right, RI::CW) => *self = Down,
    //         (Down, RI::CCW) => *self = Right,
    //         (Down, RI::CW) => *self = Left,
    //         (Left, RI::CCW) => *self = Down,
    //         (Left, RI::CW) => *self = Up,
    //         (Up, RI::CCW) => *self = Left,
    //         (Up, RI::CW) => *self = Right,
    //     }
    // }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    I,
    L,
    J,
    O,
    T,
    S,
    Z,
}

impl Piece {
    pub const PIECES: [Piece; 7] = [
        Self::I,
        Self::L,
        Self::J,
        Self::O,
        Self::T,
        Self::S,
        Self::Z,
    ];

    pub const START_POS_I: PiecePos = [23, 24, 25, 26];
    pub const START_POS_L: PiecePos = [24, 25, 26, 34];
    pub const START_POS_J: PiecePos = [24, 25, 26, 36];
    pub const START_POS_O: PiecePos = [24, 25, 34, 35];
    pub const START_POS_T: PiecePos = [24, 25, 26, 35];
    pub const START_POS_S: PiecePos = [25, 26, 34, 35];
    pub const START_POS_Z: PiecePos = [24, 25, 35, 36];

    pub fn random() -> Self {
        let mut rng = WyRand::new();

        Self::PIECES[rng.generate_range(0..7)]
    }

    pub const fn start_pos(&self) -> PiecePos {
        match self {
            Piece::I => Self::START_POS_I,
            Piece::L => Self::START_POS_L,
            Piece::J => Self::START_POS_J,
            Piece::O => Self::START_POS_O,
            Piece::T => Self::START_POS_T,
            Piece::S => Self::START_POS_S,
            Piece::Z => Self::START_POS_Z,
        }
    }

    pub fn starting_pos(&self) -> Pos {
        use Piece::*;

        match self {
            I => 3,
            L => 14,
            J => 14,
            O => 24,
            T => 14,
            S => 14,
            Z => 14,
        }
    }

    // pub fn repr(&self) -> char {
    //     use Piece::*;

    //     match self {
    //         I => 'I',
    //         L => 'L',
    //         J => 'J',
    //         O => 'O',
    //         T => 'T',
    //         S => 'S',
    //         Z => 'Z',
    //     }
    // }

    // pub fn unique_orientations(&self) -> &[(PiecePos, Rotation)] {
    //     use Piece::*;
    //     use Rotation::*;

    //     let thing: &[([(i8, i8); 4], Rotation)] = match self {
    //         I => &[
    //             ([(0, 2), (1, 2), (2, 2), (3, 2)], Right),
    //             ([(2, 0), (2, 1), (2, 2), (2, 3)], Down),
    //         ],
    //         L => &[
    //             ([(0, 1), (1, 1), (2, 1), (0, 2)], Right),
    //             ([(0, 0), (1, 0), (1, 1), (1, 2)], Down),
    //             ([(2, 0), (0, 1), (1, 1), (2, 1)], Left),
    //             ([(1, 0), (1, 1), (1, 2), (2, 2)], Up),
    //         ],
    //         J => &[
    //             ([(0, 1), (1, 1), (2, 1), (2, 2)], Right),
    //             ([(1, 0), (1, 1), (2, 0), (1, 2)], Down),
    //             ([(0, 0), (0, 1), (1, 1), (2, 1)], Left),
    //             ([(1, 0), (1, 1), (1, 2), (0, 2)], Up),
    //         ],
    //         O => &[([(0, 0), (1, 0), (0, 1), (1, 1)], Right)],
    //         T => &[
    //             ([(0, 1), (1, 1), (2, 1), (1, 2)], Right),
    //             ([(1, 0), (0, 1), (1, 1), (1, 2)], Down),
    //             ([(1, 0), (0, 1), (1, 1), (2, 1)], Left),
    //             ([(1, 0), (1, 1), (2, 1), (1, 2)], Up),
    //         ],
    //         S => &[
    //             ([(1, 1), (2, 1), (0, 2), (1, 2)], Right),
    //             ([(1, 0), (1, 1), (2, 1), (2, 2)], Down),
    //         ],
    //         Z => &[
    //             ([(0, 1), (1, 1), (1, 2), (2, 2)], Right),
    //             ([(2, 0), (1, 1), (2, 1), (1, 2)], Down),
    //         ],
    //     };

    //     unsafe { std::mem::transmute::<&[([(i8, i8); 4], Rotation)], &[(PiecePos, Rotation)]>(thing) }
    // }

    pub const fn positions(&self, rotation: Rotation) -> PiecePos {
        use Piece::*;
        use Rotation::*;

        match (self, rotation) {
            (I, r) => match r {
                Right => [20, 21, 22, 23],
                Down => [2, 12, 22, 32],
                Left => [20, 21, 22, 23],
                Up => [2, 12, 22, 32],
            },
            (L, r) => match r {
                Right => [10, 11, 12, 20],
                Down => [0, 1, 11, 21],
                Left => [2, 10, 11, 12],
                Up => [1, 11, 21, 22],
            },
            (J, r) => match r {
                Right => [10, 11, 12, 22],
                Down => [1, 11, 2, 21],
                Left => [0, 10, 11, 12],
                Up => [1, 11, 21, 20],
            },
            (O, r) => match r {
                Right => [0, 1, 10, 11],
                Down => [0, 1, 10, 11],
                Left => [0, 1, 10, 11],
                Up => [0, 1, 10, 11],
            },
            (T, r) => match r {
                Right => [10, 11, 12, 21],
                Down => [1, 10, 11, 21],
                Left => [1, 10, 11, 12],
                Up => [1, 11, 12, 21],
            },
            (S, r) => match r {
                Right => [11, 12, 20, 21],
                Down => [1, 11, 12, 22],
                Left => [11, 12, 20, 21],
                Up => [1, 11, 12, 22],
            },
            (Z, r) => match r {
                Right => [10, 11, 21, 22],
                Down => [2, 11, 12, 21],
                Left => [10, 11, 21, 22],
                Up => [2, 11, 12, 21],
            },
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[test]
fn print() {
    let p = Piece::I;

    println!("{}", p);
}
