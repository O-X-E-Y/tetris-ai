pub mod board;
pub mod consts;
pub mod consts_row;
pub mod pieces;
pub mod rng;
pub mod row_board;

use crate::board::*;
use crate::pieces::*;
use crate::rng::*;
use crate::row_board::*;

#[derive(Debug, Clone, Copy)]
pub enum Input {
    Left,
    Right,
    RotateCW,
    RotateCCW,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Level(pub u8);

impl<T: Into<u8>> From<T> for Level {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl Level {
    pub fn next(&mut self) {
        self.0 += 1
    }

    pub fn drop_speed(&self) -> Frames {
        let speed = match self.0 {
            0 => 48,
            1 => 43,
            2 => 38,
            3 => 33,
            4 => 28,
            5 => 23,
            6 => 18,
            7 => 13,
            8 => 8,
            9 => 6,
            10..=12 => 5,
            13..=15 => 4,
            16..=18 => 3,
            19..=28 => 2,
            _ => 1,
        };

        Frames(speed)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Frames(pub u8);

impl<T: Into<u8>> From<T> for Frames {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, Clone)]
pub struct Game<R> {
    pub board: Board,
    pub current: Piece,
    pub next: Piece,
    pub pos: PiecePositions,
    pub rot: Rotation,
    pub level: Level,
    pub drop_speed: Frames,
    pub score: usize,
    pub rng: R,
    // pub frames_since_last_move: u32,
    // pub frames_since_drop: u8,
    // pub finished: bool,
}

impl<R: Rng> Game<R> {
    pub fn new(level: impl Into<Level>) -> Self {
        let mut rng = R::init();
        let current = rng.next();
        let next = rng.next();

        let rot = Rotation::Right;
        let pos = current.start_pos();

        let board = Board::new();

        let level = level.into();
        let drop_speed = level.drop_speed();
        let score = 0;

        Self {
            board,
            current,
            next,
            pos,
            rot,
            level,
            drop_speed,
            score,
            rng,
        }
    }

    pub fn from_board(board: Board, level: impl Into<Level>) -> Self {
        let mut res = Self::new(level);
        res.board = board;
        res
    }

    pub fn up(&mut self) -> Option<PiecePositions> {
        match self.board.try_up(self.pos) {
            Some(pos) => {
                self.pos = pos;

                Some(pos)
            }
            None => None,
        }
    }

    pub fn down(&mut self) -> Option<PiecePositions> {
        match self.board.try_down(self.pos) {
            Some(pos) => {
                self.pos = pos;

                Some(pos)
            }
            None => None,
        }
    }

    pub fn left(&mut self) -> Option<PiecePositions> {
        match self.board.try_left(self.pos) {
            Some(pos) => {
                self.pos = pos;

                Some(pos)
            }
            None => None,
        }
    }

    pub fn right(&mut self) -> Option<PiecePositions> {
        match self.board.try_right(self.pos) {
            Some(pos) => {
                self.pos = pos;

                Some(pos)
            }
            None => None,
        }
    }

    pub fn rot_cw(&mut self) -> Option<(PiecePositions, Rotation)> {
        match self.board.try_rot_cw(self.pos, self.rot, self.current) {
            Some((pos, rot)) => {
                self.pos = pos;
                self.rot = rot;

                Some((pos, rot))
            }
            None => None,
        }
    }

    pub fn rot_ccw(&mut self) -> Option<(PiecePositions, Rotation)> {
        match self.board.try_rot_ccw(self.pos, self.rot, self.current) {
            Some((pos, rot)) => {
                self.pos = pos;
                self.rot = rot;

                Some((pos, rot))
            }
            None => None,
        }
    }

    pub fn drop(&mut self) -> u8 {
        while self.down().is_some() {}

        self.lock();

        let mut lines_cleared = 0;
        let mut r = 20;

        while r < BOARD_SIZE {
            if self.board.0[r..(r + BOARD_WIDTH)]
                .iter()
                .all(|p| p.is_some())
            {
                lines_cleared += 1;
                for p in (20..r).rev() {
                    self.board.0[p + BOARD_WIDTH] = self.board.0[p];
                }
            }
            r += 10;
        }

        lines_cleared
    }

    pub fn lock(&mut self) {
        for p in self.pos {
            self.board.0[p as usize] = Some(self.current);
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

impl<R> std::fmt::Display for Game<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 20..BOARD_SIZE {
            if i % BOARD_WIDTH == 0 {
                writeln!(f)?
            }

            if self.pos.contains(&(i as u8)) {
                write!(f, "{} ", self.current)?;
                continue;
            }

            match self.board.0.get(i) {
                Some(Some(p)) => write!(f, "{} ", p)?,
                Some(None) => write!(f, ". ")?,
                _ => panic!("invalid index"),
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RowGame<R> {
    pub board: RowBoard,
    pub current: Piece,
    pub next: Piece,
    pub pos: PiecePos,
    pub rot: Rotation,
    pub level: Level,
    pub drop_speed: Frames,
    pub score: usize,
    pub rng: R,
    // pub frames_since_last_move: u32,
    // pub frames_since_drop: u8,
    // pub finished: bool,
}

impl<R: Rng> RowGame<R> {
    pub fn new(level: impl Into<Level>) -> Self {
        let mut rng = R::init();
        let current = rng.next();
        let next = rng.next();

        let rot = Rotation::Right;
        let pos = current.row_start_pos();

        let board = RowBoard::new();

        let level = level.into();
        let drop_speed = level.drop_speed();
        let score = 0;

        Self {
            board,
            current,
            next,
            pos,
            rot,
            level,
            drop_speed,
            score,
            rng,
        }
    }

    pub fn from_board(board: RowBoard, level: impl Into<Level>) -> Self {
        let mut res = Self::new(level);
        res.board = board;
        res
    }

    pub fn up(&mut self) -> Option<PiecePos> {
        match self.board.try_up(self.pos) {
            Some(pos) => {
                self.pos = pos;

                Some(pos)
            }
            None => None,
        }
    }

    pub fn down(&mut self) -> Option<PiecePos> {
        match self.board.try_down(self.pos) {
            Some(pos) => {
                self.pos = pos;

                Some(pos)
            }
            None => None,
        }
    }

    pub fn left(&mut self) -> Option<PiecePos> {
        match self.board.try_left(self.pos) {
            Some(pos) => {
                self.pos = pos;

                Some(pos)
            }
            None => None,
        }
    }

    pub fn right(&mut self) -> Option<PiecePos> {
        match self.board.try_right(self.pos) {
            Some(pos) => {
                self.pos = pos;

                Some(pos)
            }
            None => None,
        }
    }

    pub fn rot_cw(&mut self) -> Option<(PiecePos, Rotation)> {
        match self.board.try_rot_cw(self.pos, self.current, self.rot) {
            Some((pos, rot)) => {
                self.pos = pos;
                self.rot = rot;

                Some((pos, rot))
            }
            None => None,
        }
    }

    pub fn rot_ccw(&mut self) -> Option<(PiecePos, Rotation)> {
        match self.board.try_rot_ccw(self.pos, self.current, self.rot) {
            Some((pos, rot)) => {
                self.pos = pos;
                self.rot = rot;

                Some((pos, rot))
            }
            None => None,
        }
    }

    pub fn lock(&mut self) {
        for i in 0..4 {
            let y = self.pos.y - i;
            self.board.0[y] |= self.pos.masks[3 - i];
        }
        
        for r in 2..BOARD_HEIGHT {
            if self.board.0[r] ^ FULL_LINE == 0 {
                for i in 0..r {
                    self.board.0[i] = self.board.0[i + 1]
                }
            }
        }

        self.pos = self.next.row_start_pos();
        self.rot = Rotation::Right;

        self.current = self.next;
        self.next = self.rng.next();
    }

    pub fn drop_piece(&mut self) {
        while self.down().is_some() {}

        self.lock();
    }
}

impl<R> std::fmt::Display for RowGame<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cmp = 0b1000000000000000;
        for (i, &r) in self.board.0.iter().enumerate().skip(2) {
            let mut r = r;
            if self.pos.y.wrapping_sub(i) < 4 {
                r |= self.pos.masks[self.pos.y - i];
            }
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
