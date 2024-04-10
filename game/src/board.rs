use crate::{consts::*, pieces::*};

pub const BOARD_SIZE: usize = 220;
pub const ACTUAL_BOARD_SIZE: usize = u8::MAX as usize + 11;
pub const BOARD_SIZE_U8: u8 = BOARD_SIZE as u8;
pub const BOARD_WIDTH: usize = 10;
pub const BW: u8 = BOARD_WIDTH as u8;

pub type Pos = u8;
pub type PiecePositions = [Pos; 4];

#[derive(Clone, Debug)]
pub struct Board(pub [Option<Piece>; ACTUAL_BOARD_SIZE]);

impl Default for Board {
    fn default() -> Self {
        Self([None; ACTUAL_BOARD_SIZE])
    }
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn try_up(&self, pos: PiecePositions) -> Option<PiecePositions> {
        for p in pos {
            if p < BW || self.0[p as usize - 10].is_some() {
                return None;
            }
        }
        let next_pos = apply_mask(pos, MOVE_UP);
        Some(next_pos)

        // for p in pos {
        //     if p < BW {
        //         return None;
        //     }
        // }

        // let next_pos = apply_mask(pos, MOVE_UP);

        // for p in next_pos {
        //     if self.0[p as usize].is_some() {
        //         return None;
        //     }
        // }

        // Some(next_pos)
    }

    #[inline]
    pub fn try_down(&self, pos: PiecePositions) -> Option<PiecePositions> {
        for p in pos {
            if p >= BOARD_SIZE_U8 - BW || self.0[p as usize + 10].is_some() {
                return None;
            }
        }
        let next_pos = apply_mask(pos, MOVE_DOWN);
        Some(next_pos)
        // for p in pos {
        //     if p >= BOARD_SIZE_U8 - BW {
        //         return None;
        //     }
        // }

        // let next_pos = apply_mask(pos, MOVE_DOWN);

        // for p in next_pos {
        //     if self.0[p as usize].is_some() {
        //         return None;
        //     }
        // }

        // Some(next_pos)
    }

    #[inline]
    pub fn try_left(&self, pos: PiecePositions) -> Option<PiecePositions> {
        for p in pos {
            if p % BW == 0 || self.0[p as usize - 1].is_some() {
                return None;
            }
        }
        let next_pos = apply_mask(pos, MOVE_LEFT);
        Some(next_pos)

        // unsafe {
        //     for p in pos {
        //         // the get_unchecked call adds about 2% of performance. Can't UB because the board size is greater
        //         // than the width of `u8`, still feels kind of nasty to use
        //         if p % BW == 0 || self.0.get_unchecked(p as usize - 1).is_some() {
        //             return None;
        //         }
        //     }
        //     let next_pos = apply_mask(pos, MOVE_LEFT);
        //     Some(next_pos)
        // }

        /*
            THE PERFORMANCE OF THESE IS BASICALLY IDENTICAL
            */

        // for p in pos.into_iter().rev() {
        //     if p % BW == 0 {
        //         return None;
        //     }
        // }

        // let next_pos = apply_mask(pos, MOVE_LEFT);

        // for p in next_pos {
        //     if self.0[p as usize].is_some() {
        //         return None;
        //     }
        // }

        // Some(next_pos)
    }

    #[inline]
    pub fn try_right(&self, pos: PiecePositions) -> Option<PiecePositions> {
        for p in pos {
            if p % BW == BW - 1 || self.0[p as usize + 1].is_some() {
                return None;
            }
        }
        let next_pos = apply_mask(pos, MOVE_RIGHT);
        Some(next_pos)

        // for p in pos.into_iter().rev() {
        //     if p % BW == BW - 1 {
        //         return None;
        //     }
        // }

        // let next_pos = apply_mask(pos, MOVE_RIGHT);

        // for p in next_pos {
        //     if self.0[p as usize].is_some() {
        //         return None;
        //     }
        // }

        // Some(next_pos)
    }

    #[inline]
    pub fn try_rot_cw(
        &self,
        pos: PiecePositions,
        rot: Rotation,
        piece: Piece,
    ) -> Option<(PiecePositions, Rotation)> {
        use Piece::*;
        use Rotation::*;

        let [p1, _, _, _] = pos;

        let mask = match (piece, rot) {
            (I, Right | Left) if p1 + BW + 2 < BOARD_SIZE_U8 => I_RIGHT_LEFT_CW,
            (I, Down | Up) if ![0, 1, BW - 1].contains(&(p1 % BW)) => I_DOWN_UP_CW,
            (L, Right) => L_RIGHT_CW,
            (L, Down) if p1 % BW != BW - 2 => L_DOWN_CW,
            (L, Left) if p1 + 2 * BW < BOARD_SIZE_U8 => L_LEFT_CW,
            (L, Up) if p1 % BW != 0 => L_UP_CW,
            (J, Right) => J_RIGHT_CW,
            (J, Down) if p1 % BW != BW - 1 => J_DOWN_CW,
            (J, Left) if p1 + 2 * BW + 1 < BOARD_SIZE_U8 => J_LEFT_CW,
            (J, Up) if p1 % BW != 0 => J_UP_CW,
            (O, _) => return None,
            (T, Right) => T_RIGHT_CW,
            (T, Down) if p1 % BW != BW - 1 => T_DOWN_CW,
            (T, Left) if p1 + 2 * BW < BOARD_SIZE_U8 => T_LEFT_CW,
            (T, Up) if p1 % BW != 0 => T_UP_CW,
            (S, Right | Left) => S_RIGHT_LEFT_CW,
            (S, Down | Up) if p1 % BW != 0 => S_DOWN_UP_CW,
            (Z, Right | Left) => Z_RIGHT_LEFT_CW,
            (Z, Down | Up) if p1 % BW != 1 => Z_DOWN_UP_CW,
            _ => return None,
        };

        let next_pos = apply_mask(pos, mask);

        for p in next_pos {
            if self.0[p as usize].is_some() {
                return None;
            }
        }

        let rot = match (piece, rot) {
            (O, Right) => return None,
            (I | S | Z, Right | Left) => Down,
            (I | S | Z, Down | Up) => Right,
            _ => rot.as_cw()
        };

        Some((next_pos, rot))
    }

    #[inline]
    pub fn try_rot_ccw(
        &self,
        pos: PiecePositions,
        rot: Rotation,
        piece: Piece,
    ) -> Option<(PiecePositions, Rotation)> {
        use Piece::*;
        use Rotation::*;

        let [p1, _, _, _] = pos;

        let mask = match (piece, rot) {
            (O, _) => return None,
            (I | S | Z, rot) => return self.try_rot_cw(pos, rot, piece),
            // (I, Right | Left) if p1 + BW + 2 < BOARD_SIZE_U8 => I_RIGHT_LEFT_CCW,
            // (I, Down | Up) if ![0, 1, BW - 1].contains(&(p1 % BW)) => I_DOWN_UP_CCW,
            (L, Right) => L_RIGHT_CCW,
            (L, Down) if p1 % BW != BW - 2 => L_DOWN_CCW,
            (L, Left) if p1 + 2 * BW - 1 < BOARD_SIZE_U8 => L_LEFT_CCW,
            (L, Up) if p1 % BW != 0 => L_UP_CCW,
            (J, Right) => J_RIGHT_CCW,
            (J, Down) if p1 % BW != BW - 1 => J_DOWN_CCW,
            (J, Left) if p1 + 2 * BW + 1 < BOARD_SIZE_U8 => J_LEFT_CCW,
            (J, Up) if p1 % BW != 0 => J_UP_CCW,
            (T, Right) => T_RIGHT_CCW,
            (T, Down) if p1 % BW != BW - 1 => T_DOWN_CCW,
            (T, Left) if p1 + 2 * BW < BOARD_SIZE_U8 => T_LEFT_CCW,
            (T, Up) if p1 % BW != 0 => T_UP_CCW,
            // (S, Right | Left) => S_RIGHT_LEFT_CCW,
            // (S, Down | Up) if p1 % BW != 0 => S_DOWN_UP_CCW,
            // (Z, Right | Left) => Z_RIGHT_LEFT_CCW,
            // (Z, Down | Up) if p1 % BW != 1 => Z_DOWN_UP_CCW,
            _ => return None,
        };

        let next_pos = apply_mask(pos, mask);

        for p in next_pos {
            if self.0[p as usize].is_some() {
                return None;
            }
        }

        let rot = match (piece, rot) {
            (O, _) => return None,
            (I | S | Z, Right | Left) => Down,
            (I | S | Z, Down | Up) => Right,
            _ => rot.as_ccw()
        };

        Some((next_pos, rot))
    }

    pub fn find_highest_blocks(&self) -> [u8; BOARD_WIDTH] {
        let mut res = [BOARD_SIZE_U8; BOARD_WIDTH];

        for (i, chunk) in self.0.chunks_exact(BOARD_WIDTH).enumerate() {
            for (j, p) in chunk.iter().enumerate() {
                if let (Some(_), BOARD_SIZE_U8) = (p, res[j]) {
                    res[j] = i as u8 * BW + j as u8
                }
            }
        }

        res
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 2 * BOARD_WIDTH..BOARD_SIZE {
            if i % BOARD_WIDTH == 0 {
                writeln!(f)?
            }

            match self.0.get(i) {
                Some(Some(p)) => write!(f, "{} ", p)?,
                Some(None) => write!(f, ". ")?,
                _ => unreachable!("Index greater than board size"),
            }
        }

        Ok(())
    }
}
