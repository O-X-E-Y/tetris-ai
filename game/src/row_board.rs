use crate::{consts_row::*, pieces::*};

pub const BOARD_HEIGHT: usize = 22;
pub const MIN_Y: usize = 3;
pub const MAX_Y: usize = BOARD_HEIGHT - 1;
pub const BOUNDS: u16 = 0b1110000000000111;
pub const FULL_LINE: u16 = 0b0001111111111000;
// const B16_64: u64 = BOUNDS as u64;
// pub const BOUNDS64: u64 = (B16_64 << 48) + (B16_64 << 32) + (B16_64 << 16) + (B16_64);

pub type BlockMasks = [u16; 4];

#[derive(Debug, Clone, Copy)]
pub struct PiecePos {
    pub y: usize,
    pub masks: BlockMasks,
    pub x: usize,
}

impl PiecePos {
    #[inline]
    pub const fn new(x: usize, y: usize, masks: BlockMasks) -> Self {
        Self { x, y, masks }
    }

    #[inline]
    pub const fn moved_left(self) -> Self {
        unsafe {
            let mut res = std::mem::transmute::<Self, (u64, u64, u64)>(self);
            res.1 <<= 1;
            res.2 = res.2.wrapping_sub(1);
            std::mem::transmute::<(u64, u64, u64), Self>(res)
        }
    }

    #[inline]
    pub const fn moved_right(self) -> Self {
        let mut res = unsafe { std::mem::transmute::<Self, (u64, u64, u64)>(self) };
        res.1 >>= 1;
        res.2 += 1;
        unsafe { std::mem::transmute::<(u64, u64, u64), Self>(res) }
    }

    #[inline]
    pub const fn with_u64_mask(self, mask: u64) -> Self {
        let mut res = unsafe { std::mem::transmute::<Self, (u64, u64, u64)>(self) };
        res.1 = mask;
        unsafe { std::mem::transmute::<(u64, u64, u64), Self>(res) }
    }

    // #[inline]
    // pub const fn with_u64_mask_and(self, mask: u64) -> Self {
    //     let mut res = unsafe { std::mem::transmute::<Self, (u64, u64, u64)>(self) };
    //     res.1 &= mask;
    //     unsafe { std::mem::transmute::<(u64, u64, u64), Self>(res) }
    // }

    // #[inline]
    // pub const fn with_u64_mask_or(self, mask: u64) -> Self {
    //     let mut res = unsafe { std::mem::transmute::<Self, (u64, u64, u64)>(self) };
    //     res.1 |= mask;
    //     unsafe { std::mem::transmute::<(u64, u64, u64), Self>(res) }
    // }

    // #[inline]
    // pub const fn with_u64_mask_xor(self, mask: u64) -> Self {
    //     let mut res = unsafe { std::mem::transmute::<Self, (u64, u64, u64)>(self) };
    //     res.1 ^= mask;
    //     unsafe { std::mem::transmute::<(u64, u64, u64), Self>(res) }
    // }
}

#[derive(Clone, Debug, Default)]
pub struct RowBoard(pub [u16; BOARD_HEIGHT]);

impl RowBoard {
    #[inline]
    pub const fn new() -> Self {
        Self([0; BOARD_HEIGHT])
    }

    #[inline]
    pub const fn has_collision(&self, pos: PiecePos) -> bool {
        // let [mask1, mask2, mask3, mask4] = pos.masks;

        // println!("(self.0[pos.y - 3]: {} & mask1: {mask1} & BOUNDS: {BOUNDS}) total: {}", (self.0[pos.y - 3]), (self.0[pos.y - 3] & mask1 & BOUNDS) == 0);
        // println!("(self.0[pos.y - 2]: {} & mask2: {mask2} & BOUNDS: {BOUNDS}) total: {}", (self.0[pos.y - 2]), (self.0[pos.y - 2] & mask2 & BOUNDS) == 0);
        // println!("(self.0[pos.y - 1]: {} & mask3: {mask3} & BOUNDS: {BOUNDS}) total: {}", (self.0[pos.y - 1]), (self.0[pos.y - 1] & mask3 & BOUNDS) == 0);
        // println!("(self.0[pos.y]: {} & mask4: {mask4} & BOUNDS: {BOUNDS}) total: {}", (self.0[pos.y]), (self.0[pos.y] & mask4 & BOUNDS) == 0);

        self.has_wall_collision(pos) || self.has_block_collision(pos)

        // pos.y >= MIN_Y &&
        //     pos.y < BOARD_HEIGHT &&
        //     (self.0[pos.y - 3] & mask1 & BOUNDS) != 0 &&
        //     (self.0[pos.y - 2] & mask2 & BOUNDS) != 0 &&
        //     (self.0[pos.y - 1] & mask3 & BOUNDS) != 0 &&
        //     (self.0[pos.y] & mask4 & BOUNDS) != 0
    }

    #[inline]
    pub const fn has_block_collision(&self, pos: PiecePos) -> bool {
        let [mask1, mask2, mask3, mask4] = pos.masks;

        pos.y < MIN_Y ||
            pos.y >= BOARD_HEIGHT ||
            (self.0[pos.y] & mask1) != 0 ||
            (self.0[pos.y - 1] & mask2) != 0 ||
            (self.0[pos.y - 2] & mask3) != 0 ||
            (self.0[pos.y - 3] & mask4) != 0
    }

    #[inline]
    pub const fn has_wall_collision(&self, pos: PiecePos) -> bool {
        let [mask1, mask2, mask3, mask4] = pos.masks;

        pos.y < MIN_Y ||    
            pos.y >= BOARD_HEIGHT ||
            mask1 & BOUNDS != 0 ||
            mask2 & BOUNDS != 0 ||
            mask3 & BOUNDS != 0 ||
            mask4 & BOUNDS != 0
    }

    #[inline]
    pub const fn try_up(&self, pos: PiecePos) -> Option<PiecePos> {
        let new_pos = PiecePos {
            y: pos.y.wrapping_sub(1),
            x: pos.x,
            masks: pos.masks,
        };

        match new_pos.y {
            MIN_Y.. if !self.has_block_collision(new_pos) => Some(new_pos),
            _ => None,
        }
    }

    #[inline]
    pub fn try_down(&self, pos: PiecePos) -> Option<PiecePos> {
        let new_pos = PiecePos {
            y: pos.y + 1,
            x: pos.x,
            masks: pos.masks,
        };

        match new_pos.y {
            MIN_Y..=MAX_Y if !self.has_block_collision(new_pos) => Some(new_pos),
            _ => None,
        }
    }

    #[inline]
    pub const fn try_left(&self, pos: PiecePos) -> Option<PiecePos> {
        let new_pos = pos.moved_left();

        if !self.has_collision(new_pos) {
            Some(new_pos)
        } else {
            None
        }
    }

    #[inline]
    pub const fn try_right(&self, pos: PiecePos) -> Option<PiecePos> {
        let new_pos = pos.moved_right();

        if !self.has_collision(new_pos) {
            Some(new_pos)
        } else {
            None
        }
    }

    #[inline]
    pub fn try_rot_cw(&self, pos: PiecePos, piece: Piece, rot: Rotation) -> Option<(PiecePos, Rotation)> {
        use Piece::*;
        use Rotation::*;

        let mask = match (piece, rot) {
            (I, Right | Left) => I_DOWN_UP.get(pos.x)?,
            (I, Down | Up) => I_RIGHT_LEFT.get(pos.x)?,
            (L, Right) => L_DOWN.get(pos.x)?,
            (L, Down) => L_LEFT.get(pos.x)?,
            (L, Left) => L_UP.get(pos.x)?,
            (L, Up) => L_RIGHT.get(pos.x)?,
            (J, Right) => J_DOWN.get(pos.x)?,
            (J, Down) => J_LEFT.get(pos.x)?,
            (J, Left) => J_UP.get(pos.x)?,
            (J, Up) => J_RIGHT.get(pos.x)?,
            (O, _) => return None,
            (T, Right) => T_DOWN.get(pos.x)?,
            (T, Down) => T_LEFT.get(pos.x)?,
            (T, Left) => T_UP.get(pos.x)?,
            (T, Up) => T_RIGHT.get(pos.x)?,
            (S, Right | Left) => S_DOWN_UP.get(pos.x)?,
            (S, Down | Up) => S_RIGHT_LEFT.get(pos.x)?,
            (Z, Right | Left) => Z_DOWN_UP.get(pos.x)?,
            (Z, Down | Up) => Z_RIGHT_LEFT.get(pos.x)?,
        };

        let new_pos = pos.with_u64_mask(*mask);

        if !self.has_collision(new_pos) {
            let rot = match (piece, rot) {
                (I | S | Z, Right | Left) => Down,
                (I | S | Z, Down | Up) => Right,
                (O, Right) => return None,
                _ => rot.as_cw()
            };

            Some((new_pos, rot))
        } else {
            None
        }
    }

    #[inline]
    pub fn try_rot_ccw(&self, pos: PiecePos, piece: Piece, rot: Rotation) -> Option<(PiecePos, Rotation)> {
        use Piece::*;
        use Rotation::*;

        let mask = match (piece, rot) {
            (I, Right | Left) => I_DOWN_UP.get(pos.x)?,
            (I, Down | Up) => I_RIGHT_LEFT.get(pos.x)?,
            (L, Right) => L_UP.get(pos.x)?,
            (L, Down) => L_RIGHT.get(pos.x)?,
            (L, Left) => L_DOWN.get(pos.x)?,
            (L, Up) => L_LEFT.get(pos.x)?,
            (J, Right) => J_UP.get(pos.x)?,
            (J, Down) => J_RIGHT.get(pos.x)?,
            (J, Left) => J_DOWN.get(pos.x)?,
            (J, Up) => J_LEFT.get(pos.x)?,
            (T, Right) => T_UP.get(pos.x)?,
            (T, Down) => T_RIGHT.get(pos.x)?,
            (T, Left) => T_DOWN.get(pos.x)?,
            (T, Up) => T_LEFT.get(pos.x)?,
            (S, Right | Left) => S_DOWN_UP.get(pos.x)?,
            (S, Down | Up) => S_RIGHT_LEFT.get(pos.x)?,
            (Z, Right | Left) => Z_DOWN_UP.get(pos.x)?,
            (Z, Down | Up) => Z_RIGHT_LEFT.get(pos.x)?,
            (O, _) => return None,
        };

        let new_pos = pos.with_u64_mask(*mask);

        if !self.has_collision(new_pos) {
            let rot = match (piece, rot) {
                (I | S | Z, Right | Left) => Down,
                (I | S | Z, Down | Up) => Right,
                (O, Right) => return None,
                _ => rot.as_cw()
            };

            Some((new_pos, rot))
        } else {
            None
        }
    }

    #[inline]
    pub fn find_highest_blocks(&self) -> [u8; 10] {
        let mut res = [22; 10];

        let cmp = 0b1000000000000000;
        for (row_i, &r) in self.0.iter().enumerate().rev().take(20) {
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