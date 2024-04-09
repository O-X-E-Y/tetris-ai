use arrayvec::ArrayVec;

use game::{
    pieces::{Piece, Rotation},
    rng::*,
    row_board::*,
    Frames, Level, RowGame,
};

#[derive(Debug, Clone)]
pub struct RowTetrisAi<R> {
    pub game: RowGame<R>,
    pub input_speed: Frames,
    pub highest_blocks: [u8; 10],
}

impl<R> RowTetrisAi<R> {
    pub fn from_game(game: RowGame<R>, input_speed: impl Into<Frames>) -> Self {
        let highest_blocks = game.board.find_highest_blocks();
        let input_speed = input_speed.into();

        Self {
            game,
            input_speed,
            highest_blocks,
        }
    }

    pub fn find_best_move(&mut self) -> Option<(PiecePos, u32)> {
        let positions = self.search();
        let mut best_score = u32::MAX;
        let mut best_pos = None;

        for pos in positions {
            self.game.pos = pos;
            let masks = pos.get_masks();

            let highest_blocks_old = self.highest_blocks;

            for i in 0..4 {
                let y = self.game.pos.y as usize - i;
                self.game.board.0[y] |= masks[3 - i];

                // if self.highest_blocks[self.game.pos.x] > p {
                //     self.highest_blocks[self.game.pos.x] = p;
                // }
            }

            let score = self.eval();

            if score < best_score {
                best_score = score;
                best_pos = Some(pos);
            }

            for i in 0..4 {
                let y = self.game.pos.y as usize - i;
                self.game.board.0[y] ^= masks[3 - i];

                // if self.highest_blocks[(p % 10) as usize] > p {
                //     self.highest_blocks[(p % 10) as usize] = p;
                // }
            }

            self.highest_blocks = highest_blocks_old;
        }

        best_pos.map(|p| (p, best_score))
    }

    pub fn eval(&self) -> u32 {
        // let mut hole_score = 0u32;

        // for mut h in self.highest_blocks {
        //     h += BW;
        //     while h < 10_U8 {
        //         if self.game.board.0[h as usize].is_none() {
        //             hole_score += 2;
        //         }

        //         h += BW;
        //     }
        // }

        // let mut flatness = 0u32;

        // for h in self.highest_blocks.windows(2) {
        //     let diff = h[0].abs_diff(h[1]) as u32;
        //     // println!("diff: {diff}");
        //     flatness += diff;
        // }

        // // let mut left_weight = 0u32;

        // // for (h, w) in self.highest_blocks.into_iter().zip((0..BOARD_WIDTH as u32).rev()) {
        // //     left_weight += (10_U8 - h) as u32 * w;
        // // }

        // // println!("flatness: {}, hole score: {}", flatness, hole_score.saturating_pow(2));

        // flatness + hole_score.saturating_pow(2) * 1000

        0
    }

    pub fn search(&self) -> ArrayVec<PiecePos, 100> {
        let mut final_states = ArrayVec::new();
        let mut searched_states = [0u8; BOARD_HEIGHT * 10];

        self.search_helper(self.game.pos, &mut searched_states, &mut final_states);

        final_states
    }

    fn search_helper(
        &self,
        pos: PiecePos,
        searched_states: &mut [u8; BOARD_HEIGHT * 10],
        final_states: &mut ArrayVec<PiecePos, 100>,
    ) {
        use Piece::*;

        if let Some(new_pos) = self.game.board.try_left(pos) {
            let searched_i = new_pos.y as usize * 10 + new_pos.x as usize;
            if searched_i < searched_states.len()
                && searched_states[searched_i] & new_pos.rot as u8 == 0
            {
                searched_states[searched_i] |= new_pos.rot as u8;
                self.search_helper(new_pos, searched_states, final_states)
            }
        }

        if let Some(new_pos) = self.game.board.try_right(pos) {
            let searched_i = new_pos.y as usize * 10 + new_pos.x as usize;
            // if searched_states[searched_i] & new_pos.rot as u8 != 0 {
            //     dbg!(searched_i, pos.rot, new_pos);
            // }
            if searched_i < searched_states.len()
                && searched_states[searched_i] & new_pos.rot as u8 == 0
            {
                searched_states[searched_i] |= new_pos.rot as u8;
                self.search_helper(new_pos, searched_states, final_states)
            }
        }

        match self.game.pos.piece {
            I | S | Z => self.search_only_cw(pos, searched_states, final_states),
            L | J | T => self.search_cw_ccw(pos, searched_states, final_states),
            O => {}
        }

        match self.game.board.try_down(pos) {
            Some(new_pos) => {
                let searched_i = new_pos.y as usize * 10 + new_pos.x as usize;
                if searched_i < searched_states.len()
                    && searched_states[searched_i] & new_pos.rot as u8 == 0
                {
                    searched_states[searched_i] |= new_pos.rot as u8;
                    self.search_helper(new_pos, searched_states, final_states)
                }
            }
            None => final_states.push(pos),
        }
    }

    #[inline]
    fn search_only_cw(
        &self,
        pos: PiecePos,
        searched_states: &mut [u8; BOARD_HEIGHT * 10],
        final_states: &mut ArrayVec<PiecePos, 100>,
    ) {
        if let Some(new_pos) = self.game.board.try_rot_cw(pos) {
            let searched_i = new_pos.y as usize * 10 + new_pos.x as usize;
            if searched_i < searched_states.len()
                && searched_states[searched_i] & new_pos.rot as u8 == 0
            {
                searched_states[searched_i] |= new_pos.rot as u8;
                self.search_helper(new_pos, searched_states, final_states)
            }
        }
    }

    #[inline]
    fn search_cw_ccw(
        &self,
        pos: PiecePos,
        searched_states: &mut [u8; BOARD_HEIGHT * 10],
        final_states: &mut ArrayVec<PiecePos, 100>,
    ) {
        if let Some(new_pos) = self.game.board.try_rot_cw(pos) {
            let searched_i = new_pos.y as usize * 10 + new_pos.x as usize;
            if searched_i < searched_states.len()
                && searched_states[searched_i] & new_pos.rot as u8 == 0
            {
                searched_states[searched_i] |= new_pos.rot as u8;
                self.search_helper(new_pos, searched_states, final_states)
            }
        }

        if let Some(new_pos) = self.game.board.try_rot_ccw(pos) {
            let searched_i = new_pos.y as usize * 10 + new_pos.x as usize;
            if searched_i < searched_states.len()
                && searched_states[searched_i] & new_pos.rot as u8 == 0
            {
                searched_states[searched_i] |= new_pos.rot as u8;
                self.search_helper(new_pos, searched_states, final_states)
            }
        }
    }
}

impl<R: Rng> RowTetrisAi<R> {
    pub fn new(input_speed: impl Into<Frames>, level: impl Into<Level>) -> Self {
        Self {
            game: RowGame::new(level),
            input_speed: input_speed.into(),
            highest_blocks: [BOARD_HEIGHT as u8; 10],
        }
    }

    pub fn from_board(
        board: RowBoard,
        input_speed: impl Into<Frames>,
        level: impl Into<Level>,
    ) -> Self {
        let highest_pieces = board.find_highest_blocks();

        Self {
            game: RowGame::from_board(board, level),
            input_speed: input_speed.into(),
            highest_blocks: highest_pieces,
        }
    }
}

#[test]
fn search_l() {
    let mut ai = RowTetrisAi::<game::rng::ClassicRng>::new(1, 19);

    ai.game.pos = game::pieces::Piece::L.row_start_pos();

    ai.search();
}
