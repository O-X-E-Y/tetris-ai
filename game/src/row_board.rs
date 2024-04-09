use crate::{consts_row::*, pieces::*};

pub const BOARD_HEIGHT: usize = 26;
pub const BOARD_HEIGHT_U8: u8 = BOARD_HEIGHT as u8;
pub const MAX_Y: u8 = (BOARD_HEIGHT - 4) as u8;
pub const BOUNDS: u16 = 0b1110000000000111;
pub const FULL_LINE: u16 = u16::MAX;
// const B16_64: u64 = BOUNDS as u64;
// pub const BOUNDS64: u64 = (B16_64 << 48) + (B16_64 << 32) + (B16_64 << 16) + (B16_64);

pub type BlockMasks = [u16; 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PiecePos {
    pub piece: Piece,
    pub rot: Rotation,
    pub x: u8,
    pub y: u8,
}

impl PiecePos {
    #[inline]
    pub const fn new(x: u8, y: u8, piece: Piece, rot: Rotation) -> Self {
        Self { x, y, piece, rot }
    }

    #[inline]
    pub const fn moved_left(mut self) -> Self {
        self.x = self.x.wrapping_sub(1);
        self
    }

    #[inline]
    pub const fn moved_right(mut self) -> Self {
        self.x += 1;
        self
    }

    #[inline]
    pub const fn get_masks(&self) -> BlockMasks {
        use Piece::*;
        use Rotation::*;

        let mask = match (self.piece, self.rot) {
            (I, Right | Left) => I_RIGHT_LEFT >> self.x,
            (I, Down | Up) => I_DOWN_UP >> self.x,
            (L, Right) => L_RIGHT >> self.x,
            (L, Down) => L_DOWN >> self.x,
            (L, Left) => L_LEFT >> self.x,
            (L, Up) => L_UP >> self.x,
            (J, Right) => J_RIGHT >> self.x,
            (J, Down) => J_DOWN >> self.x,
            (J, Left) => J_LEFT >> self.x,
            (J, Up) => J_UP >> self.x,
            (O, _) => O_ALL >> self.x,
            (T, Right) => T_RIGHT >> self.x,
            (T, Down) => T_DOWN >> self.x,
            (T, Left) => T_LEFT >> self.x,
            (T, Up) => T_UP >> self.x,
            (S, Right | Left) => S_RIGHT_LEFT >> self.x,
            (S, Down | Up) => S_DOWN_UP >> self.x,
            (Z, Right | Left) => Z_RIGHT_LEFT >> self.x,
            (Z, Down | Up) => Z_DOWN_UP >> self.x,
        };

        unsafe { std::mem::transmute::<u64, [u16; 4]>(mask) }
    }
}

#[derive(Clone, Debug)]
pub struct RowBoard(pub [u16; BOARD_HEIGHT]);

impl Default for RowBoard {
    fn default() -> Self {
        let board = [
            0, 0, 0, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS,
            BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS,
            FULL_LINE, FULL_LINE, FULL_LINE,
        ];
        Self(board)
    }
}

impl RowBoard {
    #[inline]
    pub const fn new() -> Self {
        let board = [
            0, 0, 0, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS,
            BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS, BOUNDS,
            FULL_LINE, FULL_LINE, FULL_LINE,
        ];
        Self(board)
    }

    #[inline]
    pub const fn no_collision(&self, pos: PiecePos) -> bool {
        let [mask1, mask2, mask3, mask4] = pos.get_masks();

        // dbg!(pos.y);
        // println!("{:b}, {:b}", self.0[pos.y as usize], mask1);
        // println!("{:b}, {:b}", self.0[pos.y as usize + 1], mask2);
        // println!("{:b}, {:b}", self.0[pos.y as usize + 2], mask3);
        // println!("{:b}, {:b}\n-----------", self.0[pos.y as usize + 3], mask4);

        !(pos.y > MAX_Y ||
            (self.0[pos.y as usize] & mask1) != 0 ||
            (self.0[pos.y as usize + 1] & mask2) != 0 ||
            (self.0[pos.y as usize + 2] & mask3) != 0 ||
            (self.0[pos.y as usize + 3] & mask4) != 0)
    }

    #[inline]
    pub const fn try_up(&self, pos: PiecePos) -> Option<PiecePos> {
        let new_pos = PiecePos {
            y: pos.y.wrapping_sub(1),
            x: pos.x,
            piece: pos.piece,
            rot: pos.rot,
        };

        match self.no_collision(new_pos) {
            true => Some(new_pos),
            _ => None
        }
    }

    #[inline]
    pub const fn try_down(&self, pos: PiecePos) -> Option<PiecePos> {
        let new_pos = PiecePos {
            y: pos.y + 1,
            x: pos.x,
            piece: pos.piece,
            rot: pos.rot,
        };

        match self.no_collision(new_pos) {
            true => Some(new_pos),
            _ => None
        }
    }

    #[inline]
    pub const fn try_left(&self, pos: PiecePos) -> Option<PiecePos> {
        let new_pos = pos.moved_left();

        match self.no_collision(new_pos) {
            true => Some(new_pos),
            _ => None
        }
    }

    #[inline]
    pub const fn try_right(&self, pos: PiecePos) -> Option<PiecePos> {
        let new_pos = pos.moved_right();

        match self.no_collision(new_pos) {
            true => Some(new_pos),
            _ => None
        }
    }

    #[inline]
    pub const fn try_rot_cw(&self, mut pos: PiecePos) -> Option<PiecePos> {
        use Piece::*;
        use Rotation::*;

        pos.rot = match (pos.piece, pos.rot) {
            (I | S | Z, Right | Left) => Down,
            (I | S | Z, Down | Up) => Right,
            (O, Right) => return None,
            (_, rot) => rot.as_cw(),
        };

        match self.no_collision(pos) {
            true => Some(pos),
            _ => None
        }
    }

    #[inline]
    pub const fn try_rot_ccw(&self, mut pos: PiecePos) -> Option<PiecePos> {
        use Piece::*;
        use Rotation::*;

        pos.rot = match (pos.piece, pos.rot) {
            (I | S | Z, Right | Left) => Down,
            (I | S | Z, Down | Up) => Right,
            (O, Right) => return None,
            (_, rot) => rot.as_ccw(),
        };

        match self.no_collision(pos) {
            true => Some(pos),
            _ => None
        }
    }

    #[inline]
    pub fn find_highest_blocks(&self) -> [u8; 10] {
        let mut res = [BOARD_HEIGHT_U8; 10];

        let cmp = 0b1000000000000000;
        for (row_i, &r) in self.0.iter().skip(3).enumerate().rev().skip(3) {
            let mut row = r;
            row <<= 3;
            for r in res.iter_mut() {
                if row & cmp == cmp {
                    *r = row_i as u8;
                }
                row <<= 1;
            }
        }

        res
    }
}

impl std::fmt::Display for RowBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cmp = 0b1000000000000000;
        for &r in &self.0[2..] {
            let mut r = r;
            r <<= 3;
            for _ in 0..10 {
                if r & cmp == cmp {
                    write!(f, "O ")?
                } else {
                    write!(f, ". ")?
                }
                r <<= 1;
            }
            writeln!(f)?
        }

        Ok(())
    }
}
