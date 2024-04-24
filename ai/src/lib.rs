pub mod flatness_states;
pub mod row_ai;
mod recursive_search;
pub mod states;

use arrayvec::ArrayVec;

use game::{
    board::*,
    pieces::{Piece, Rotation},
    rng::*,
    Level,
};

#[derive(Debug, Clone)]
pub struct TetrisAi<R> {
    pub board: Board,
    pub rng: R,
    pub current: Piece,
    pub next: Piece,
    pub pos: PiecePositions,
    pub rot: Rotation,
    pub level: Level,
    pub score: usize,
    pub highest_blocks: [u8; BOARD_WIDTH],
}

impl<R> TetrisAi<R> {
    pub fn find_best_move(&mut self) -> Option<(PiecePositions, u32)> {
        let positions = self.search();
        let mut best_score = u32::MAX;
        let mut best_pos = None;

        for pos in positions {
            self.pos = pos;

            let highest_blocks_old = self.highest_blocks;

            for p in self.pos {
                self.board.0[p as usize] = Some(self.current);

                let rem = p % BW;
                let new = p - rem;
                if self.highest_blocks[rem as usize] > new {
                    self.highest_blocks[rem as usize] = new;
                }
            }

            let score = self.eval();

            if score < best_score {
                best_score = score;
                best_pos = Some(pos);
            }

            for p in self.pos {
                self.board.0[p as usize] = None;
            }

            self.highest_blocks = highest_blocks_old;
        }

        best_pos.map(|p| (p, best_score))
    }

    pub fn holes(&self) -> u64 {
        let mut holes = 0;

        for (i, h) in self.highest_blocks.into_iter().enumerate() {
            let mut offset = BW;
            while h + offset < BOARD_SIZE_U8 {
                if self.board.0[(h + offset) as usize + i].is_none() {
                    holes += 1;
                }

                offset += BW;
            }
        }

        holes
    }

    pub fn eval(&self) -> u32 {
        let mut hole_score = 0u32;

        for (i, h) in self.highest_blocks.into_iter().enumerate() {
            let mut weight = BW;
            while h + weight < BOARD_SIZE_U8 {
                if self.board.0[(h + weight) as usize + i].is_none() {
                    hole_score += weight as u32;
                }

                weight += BW;
            }
        }

        let mut flatness = 0u32;

        for h in self.highest_blocks.windows(3) {
            let diff = h[0].abs_diff(h[1]) as u32;
            // println!("diff: {diff}");
            flatness += diff;
        }

        // let mut left_weight = 0u32;

        // for (h, w) in self.highest_blocks.into_iter().zip((0..BOARD_WIDTH as u32).rev()) {
        //     left_weight += (BOARD_SIZE_U8 - h) as u32 * w;
        // }

        // println!("flatness: {}, hole score: {}", flatness, hole_score.saturating_pow(2));

        flatness + hole_score * 50
    }

    pub fn search(&self) -> ArrayVec<PiecePositions, 100> {
        use Piece::*;
        
        let mut final_states = ArrayVec::new();
        let mut searched_states = [0u8; BOARD_SIZE];
        let mut stack = Vec::with_capacity(70);
        stack.push((self.pos, self.rot));

        while let Some((pos, rot)) = stack.pop() {

            if let Some(new_pos) = self.board.try_left(pos) {
                if searched_states[new_pos[0] as usize] == 0 {
                    searched_states[new_pos[0] as usize] |= rot as u8;
                    stack.push((new_pos, rot));
                }
            }
    
            if let Some(new_pos) = self.board.try_right(pos) {
                if searched_states[new_pos[0] as usize] & rot as u8 == 0 {
                    searched_states[new_pos[0] as usize] |= rot as u8;
                    stack.push((new_pos, rot));
                }
            }
    
            match self.current {
                I | S | Z => {
                    if let Some((new_pos, new_rot)) = self.board.try_rot_cw(pos, rot, self.current) {
                        if searched_states[new_pos[0] as usize] & new_rot as u8 == 0 {
                            searched_states[new_pos[0] as usize] |= new_rot as u8;
                            stack.push((new_pos, new_rot));
                        }
                    }
                },
                L | J | T => {
                    if let Some((new_pos, new_rot)) = self.board.try_rot_cw(pos, rot, self.current) {
                        if searched_states[new_pos[0] as usize] & new_rot as u8 == 0 {
                            searched_states[new_pos[0] as usize] |= new_rot as u8;
                            stack.push((new_pos, new_rot));
                        }
                    }
            
                    if let Some((new_pos, new_rot)) = self.board.try_rot_ccw(pos, rot, self.current) {
                        if searched_states[new_pos[0] as usize] & new_rot as u8 == 0 {
                            searched_states[new_pos[0] as usize] |= new_rot as u8;
                            stack.push((new_pos, new_rot));
                        }
                    }
                },
                O => {}
            }
    
            match self.board.try_down(pos) {
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
}

impl<R: Rng> TetrisAi<R> {
    pub fn new(level: impl Into<Level>) -> Self {
        let mut rng = R::init();
        let current = rng.next();

        let level = level.into();

        Self {
            board: Board::new(),
            highest_blocks: [BOARD_SIZE_U8; BOARD_WIDTH],
            current,
            next: rng.next(),
            pos: current.start_pos(),
            rot: Rotation::Right,
            level,
            score: 0,
            rng,
        }
    }

    pub fn from_board(
        board: Board,
        level: impl Into<Level>,
    ) -> Self {
        let mut res = Self::new(level);
        res.highest_blocks = board.find_highest_blocks();
        res.board = board;
        res
    }

    pub fn lock(&mut self) {
        for p in self.pos {
            self.board.0[p as usize] = Some(self.current);

            let rem = p % BW;
            let new = p - rem;
            if self.highest_blocks[rem as usize] > new {
                self.highest_blocks[rem as usize] = new;
            }
        }

        let mut r = 20;

        while r < BOARD_SIZE {
            if self.board.0[r..(r + BOARD_WIDTH)]
                .iter()
                .all(|p| p.is_some())
            {
                for p in (20..r).rev() {
                    self.board.0[p + BOARD_WIDTH] = self.board.0[p];
                }
            }
            r += 10;
        }

        self.pos = self.next.start_pos();
        self.rot = Rotation::Right;

        self.current = self.next;
        self.next = self.rng.next();
    }
}

#[test]
fn search_l() {
    let ai = TetrisAi::<game::rng::ClassicRng>::new(19);

    // ai.current.pos
    // ai.pos = game::pieces::Piece::L.start_pos();

    ai.search();
}
