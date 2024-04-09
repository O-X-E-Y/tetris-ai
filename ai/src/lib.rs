pub mod row_ai;

use arrayvec::ArrayVec;

use game::{
    board::*,
    pieces::{Piece, Rotation},
    rng::*,
    Frames, Game, Level,
};

#[derive(Debug, Clone)]
pub struct TetrisAi<R> {
    pub game: Game<R>,
    pub input_speed: Frames,
    pub highest_blocks: [u8; BOARD_WIDTH],
}

impl<R> TetrisAi<R> {
    pub fn from_game(game: Game<R>, input_speed: impl Into<Frames>) -> Self {
        let highest_blocks = game.board.find_highest_blocks();
        let input_speed = input_speed.into();

        Self {
            game,
            input_speed,
            highest_blocks,
        }
    }

    pub fn find_best_move(&mut self) -> Option<(PiecePositions, u32)> {
        let positions = self.search_recursive();
        let mut best_score = u32::MAX;
        let mut best_pos = None;

        for pos in positions {
            self.game.pos = pos;

            let highest_blocks_old = self.highest_blocks;

            for p in self.game.pos {
                self.game.board.0[p as usize] = Some(self.game.current);

                if self.highest_blocks[(p % 10) as usize] > p {
                    self.highest_blocks[(p % 10) as usize] = p;
                }
            }

            let score = self.eval();

            if score < best_score {
                best_score = score;
                best_pos = Some(pos);
            }

            for p in self.game.pos {
                self.game.board.0[p as usize] = None;
            }

            self.highest_blocks = highest_blocks_old;
        }

        best_pos.map(|p| (p, best_score))
    }

    pub fn eval(&self) -> u32 {
        let mut hole_score = 0u32;

        for mut h in self.highest_blocks {
            h += BW;
            while h < BOARD_SIZE_U8 {
                if self.game.board.0[h as usize].is_none() {
                    hole_score += 2;
                }

                h += BW;
            }
        }

        let mut flatness = 0u32;

        for h in self.highest_blocks.windows(2) {
            let diff = h[0].abs_diff(h[1]) as u32;
            // println!("diff: {diff}");
            flatness += diff;
        }

        // let mut left_weight = 0u32;

        // for (h, w) in self.highest_blocks.into_iter().zip((0..BOARD_WIDTH as u32).rev()) {
        //     left_weight += (BOARD_SIZE_U8 - h) as u32 * w;
        // }

        // println!("flatness: {}, hole score: {}", flatness, hole_score.saturating_pow(2));

        flatness + hole_score.saturating_pow(2) * 1000
    }

    pub fn search(&self) -> ArrayVec<PiecePositions, 100> {
        use Piece::*;
        
        let mut final_states = ArrayVec::new();
        let mut searched_states = [0u8; BOARD_SIZE];
        let mut stack = Vec::with_capacity(70);
        stack.push((self.game.pos, self.game.rot));

        while let Some((pos, rot)) = stack.pop() {

            if let Some(new_pos) = self.game.board.try_left(pos) {
                if searched_states[new_pos[0] as usize] == 0 {
                    searched_states[new_pos[0] as usize] |= rot as u8;
                    stack.push((new_pos, rot));
                }
            }
    
            if let Some(new_pos) = self.game.board.try_right(pos) {
                if searched_states[new_pos[0] as usize] & rot as u8 == 0 {
                    searched_states[new_pos[0] as usize] |= rot as u8;
                    stack.push((new_pos, rot));
                }
            }
    
            match self.game.current {
                I | S | Z => {
                    if let Some((new_pos, new_rot)) = self.game.board.try_rot_cw(pos, rot, self.game.current) {
                        if searched_states[new_pos[0] as usize] & new_rot as u8 == 0 {
                            searched_states[new_pos[0] as usize] |= new_rot as u8;
                            stack.push((new_pos, new_rot));
                        }
                    }
                },
                L | J | T => {
                    if let Some((new_pos, new_rot)) = self.game.board.try_rot_cw(pos, rot, self.game.current) {
                        if searched_states[new_pos[0] as usize] & new_rot as u8 == 0 {
                            searched_states[new_pos[0] as usize] |= new_rot as u8;
                            stack.push((new_pos, new_rot));
                        }
                    }
            
                    if let Some((new_pos, new_rot)) = self.game.board.try_rot_ccw(pos, rot, self.game.current) {
                        if searched_states[new_pos[0] as usize] & new_rot as u8 == 0 {
                            searched_states[new_pos[0] as usize] |= new_rot as u8;
                            stack.push((new_pos, new_rot));
                        }
                    }
                },
                O => {}
            }
    
            match self.game.board.try_down(pos) {
                Some(new_pos) if searched_states[new_pos[0] as usize] & rot as u8 == 0 => {
                    searched_states[new_pos[0] as usize] |= rot as u8;
                    stack.push((new_pos, rot));
                }
                None => final_states.push(pos),
                _ => {}
            }
        }

        final_states
    }

    pub fn search_recursive(&self) -> ArrayVec<PiecePositions, 100> {
        let mut final_states = ArrayVec::new();
        let mut searched_states = [0u8; BOARD_SIZE];

        self.search_helper(
            self.game.pos,
            self.game.rot,
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

        if let Some(new_pos) = self.game.board.try_left(pos) {
            if searched_states[new_pos[0] as usize] == 0 {
                searched_states[new_pos[0] as usize] |= rot as u8;
                self.search_helper(new_pos, rot, searched_states, final_states)
            }
        }

        if let Some(new_pos) = self.game.board.try_right(pos) {
            if searched_states[new_pos[0] as usize] & rot as u8 == 0 {
                searched_states[new_pos[0] as usize] |= rot as u8;
                self.search_helper(new_pos, rot, searched_states, final_states)
            }
        }

        match self.game.current {
            I | S | Z => self.search_only_cw(pos, rot, searched_states, final_states),
            L | J | T => self.search_cw_ccw(pos, rot, searched_states, final_states),
            O => {}
        }

        match self.game.board.try_down(pos) {
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
        if let Some((new_pos, new_rot)) = self.game.board.try_rot_cw(pos, rot, self.game.current) {
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
        if let Some((new_pos, new_rot)) = self.game.board.try_rot_cw(pos, rot, self.game.current) {
            if searched_states[new_pos[0] as usize] & new_rot as u8 == 0 {
                searched_states[new_pos[0] as usize] |= new_rot as u8;
                self.search_helper(new_pos, new_rot, searched_states, final_states)
            }
        }

        if let Some((new_pos, new_rot)) = self.game.board.try_rot_ccw(pos, rot, self.game.current) {
            if searched_states[new_pos[0] as usize] & new_rot as u8 == 0 {
                searched_states[new_pos[0] as usize] |= new_rot as u8;
                self.search_helper(new_pos, new_rot, searched_states, final_states)
            }
        }
    }
}

impl<R: Rng> TetrisAi<R> {
    pub fn new(input_speed: impl Into<Frames>, level: impl Into<Level>) -> Self {
        Self {
            game: Game::new(level),
            input_speed: input_speed.into(),
            highest_blocks: [BOARD_SIZE_U8; BOARD_WIDTH],
        }
    }

    pub fn from_board(
        board: Board,
        input_speed: impl Into<Frames>,
        level: impl Into<Level>,
    ) -> Self {
        let highest_pieces = board.find_highest_blocks();

        Self {
            game: Game::from_board(board, level),
            input_speed: input_speed.into(),
            highest_blocks: highest_pieces,
        }
    }
}

#[test]
fn search_l() {
    let ai = TetrisAi::<game::rng::ClassicRng>::new(1, 19);

    // ai.current.pos
    // ai.game.pos = game::pieces::Piece::L.start_pos();

    ai.search();
}
