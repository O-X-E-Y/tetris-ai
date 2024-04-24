use arrayvec::ArrayVec;
use game::{board::{PiecePositions, BOARD_SIZE}, pieces::{Piece, Rotation}};

use crate::TetrisAi;


impl<R> TetrisAi<R> {
    pub fn search_recursive(&self) -> ArrayVec<PiecePositions, 100> {
        let mut final_states = ArrayVec::new();
        let mut searched_states = [0u8; BOARD_SIZE];

        self.search_helper(
            self.pos,
            self.rot,
            &mut searched_states,
            &mut final_states,
        );

        final_states
    }

    fn search_helper(
        &self,
        pos: PiecePositions,
        rot: Rotation,
        searched_states: &mut [u8; BOARD_SIZE],
        final_states: &mut ArrayVec<PiecePositions, 100>,
    ) {
        use Piece::*;

        if let Some(new_pos) = self.board.try_left(pos) {
            if searched_states[new_pos[0] as usize] == 0 {
                searched_states[new_pos[0] as usize] |= rot as u8;
                self.search_helper(new_pos, rot, searched_states, final_states)
            }
        }

        if let Some(new_pos) = self.board.try_right(pos) {
            if searched_states[new_pos[0] as usize] & rot as u8 == 0 {
                searched_states[new_pos[0] as usize] |= rot as u8;
                self.search_helper(new_pos, rot, searched_states, final_states)
            }
        }

        match self.current {
            I | S | Z => self.search_only_cw(pos, rot, searched_states, final_states),
            L | J | T => self.search_cw_ccw(pos, rot, searched_states, final_states),
            O => {}
        }

        match self.board.try_down(pos) {
            Some(new_pos) if searched_states[new_pos[0] as usize] & rot as u8 == 0 => {
                searched_states[new_pos[0] as usize] |= rot as u8;
                self.search_helper(new_pos, rot, searched_states, final_states)
            }
            None => final_states.push(pos),
            _ => {}
        }
    }

    #[inline]
    fn search_only_cw(
        &self,
        pos: PiecePositions,
        rot: Rotation,
        searched_states: &mut [u8; BOARD_SIZE],
        final_states: &mut ArrayVec<PiecePositions, 100>,
    ) {
        if let Some((new_pos, new_rot)) = self.board.try_rot_cw(pos, rot, self.current) {
            if searched_states[new_pos[0] as usize] & new_rot as u8 == 0 {
                searched_states[new_pos[0] as usize] |= new_rot as u8;
                self.search_helper(new_pos, new_rot, searched_states, final_states)
            }
        }
    }

    #[inline]
    fn search_cw_ccw(
        &self,
        pos: PiecePositions,
        rot: Rotation,
        searched_states: &mut [u8; BOARD_SIZE],
        final_states: &mut ArrayVec<PiecePositions, 100>,
    ) {
        if let Some((new_pos, new_rot)) = self.board.try_rot_cw(pos, rot, self.current) {
            if searched_states[new_pos[0] as usize] & new_rot as u8 == 0 {
                searched_states[new_pos[0] as usize] |= new_rot as u8;
                self.search_helper(new_pos, new_rot, searched_states, final_states)
            }
        }

        if let Some((new_pos, new_rot)) = self.board.try_rot_ccw(pos, rot, self.current) {
            if searched_states[new_pos[0] as usize] & new_rot as u8 == 0 {
                searched_states[new_pos[0] as usize] |= new_rot as u8;
                self.search_helper(new_pos, new_rot, searched_states, final_states)
            }
        }
    }
}