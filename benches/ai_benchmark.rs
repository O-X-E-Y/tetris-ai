mod util;

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use util::tetris_ai_with_start_piece;

pub fn bench_search_l(c: &mut Criterion) {
    let ai = tetris_ai_with_start_piece(game::pieces::Piece::L, 1, 19);

    c.bench_function("search with L", |b| {
        b.iter(|| {
            ai.search();
        });
    });
}

pub fn bench_search_row_l(c: &mut Criterion) {
    let mut ai = ai::row_ai::RowTetrisAi::<game::rng::ClassicRng>::new(1, 19);

    ai.game.pos = game::pieces::Piece::L.row_start_pos();

    c.bench_function("search rows with L", |b| {
        b.iter(|| {
            ai.search();
        });
    });
}

pub fn bench_search_o(c: &mut Criterion) {
    let ai = tetris_ai_with_start_piece(game::pieces::Piece::O, 1, 19);

    c.bench_function("search with O", |b| {
        b.iter(|| {
            ai.search();
        });
    });
}

pub fn bench_try_left(c: &mut Criterion) {
    let mut board = util::game_with_start_piece(game::pieces::Piece::L, 19);

    board.down();
    board.down();

    c.bench_function("try left L", |b| {
        b.iter(|| board.board.try_left(black_box(board.pos)));
    });
}

pub fn bench_try_right(c: &mut Criterion) {
    let mut board = util::game_with_start_piece(game::pieces::Piece::L, 19);

    board.down();
    board.down();

    c.bench_function("try right L", |b| {
        b.iter(|| board.board.try_right(black_box(board.pos)));
    });
}

pub fn bench_try_rot_cw(c: &mut Criterion) {
    let piece = game::pieces::Piece::L;
    let mut board = util::game_with_start_piece(piece, 19);

    board.down();
    board.down();

    c.bench_function("try rotating L clockwise", |b| {
        b.iter(|| board.board.try_rot_cw(black_box(board.pos), black_box(board.rot), black_box(piece)));
    });
}

pub fn bench_try_rot_ccw(c: &mut Criterion) {
    let piece = game::pieces::Piece::L;
    let mut board = util::game_with_start_piece(piece, 19);

    board.down();
    board.down();

    c.bench_function("try rotating L counter clockwise", |b| {
        b.iter(|| board.board.try_rot_ccw(black_box(board.pos), black_box(board.rot), black_box(piece)));
    });
}

pub fn bench_find_best_move(c: &mut Criterion) {
    let mut ai = ai::TetrisAi::<game::rng::ClassicRng>::new(10, 19);

    c.bench_function("find best move in empty board", |b| {
        b.iter(|| ai.find_best_move());
    });
}

pub fn bench_try_up(c: &mut Criterion) {
    let board = game::board::Board::new();
    let pos = [63, 64, 65, 75];

    c.bench_function("try up on naive board", |b| {
        b.iter(|| board.try_up(black_box(pos)));
    });
}

pub fn bench_try_up_rows(c: &mut Criterion) {
    let board = game::row_board::RowBoard::new();

    let pos = game::row_board::PiecePos {
        x: 6,
        y: 6,
        piece: game::pieces::Piece::L,
        rot: game::pieces::Rotation::Right,
    };

    c.bench_function("try up on row board", |b| {
        b.iter(|| board.try_up(black_box(pos)));
    });
}

pub fn bench_try_down_rows(c: &mut Criterion) {
    let board = game::row_board::RowBoard::new();

    let pos = game::row_board::PiecePos {
        x: 6,
        y: 6,
        piece: game::pieces::Piece::L,
        rot: game::pieces::Rotation::Right,
    };

    c.bench_function("try down on row board", |b| {
        b.iter(|| board.try_down(black_box(pos)));
    });
}

pub fn bench_try_left_rows(c: &mut Criterion) {
    let board = game::row_board::RowBoard::new();

    let pos = game::row_board::PiecePos {
        x: 6,
        y: 6,
        piece: game::pieces::Piece::L,
        rot: game::pieces::Rotation::Right,
    };

    c.bench_function("try left on row board", |b| {
        b.iter(|| board.try_left(black_box(pos)));
    });
}

pub fn bench_try_right_rows(c: &mut Criterion) {
    let board = game::row_board::RowBoard::new();

    let pos = game::row_board::PiecePos {
        x: 6,
        y: 6,
        piece: game::pieces::Piece::L,
        rot: game::pieces::Rotation::Right,
    };

    c.bench_function("try right on row board", |b| {
        b.iter(|| board.try_right(black_box(pos)));
    });
}

pub fn bench_try_rot_cw_rows(c: &mut Criterion) {
    let board = game::row_board::RowBoard::new();

    let pos = game::pieces::Piece::L.row_start_pos();

    c.bench_function("try rotating L clockwise on row board", |b| {
        b.iter(|| board.try_rot_cw(black_box(pos)));
    });
}

criterion_group!(benches, bench_search_row_l);
criterion_main!(benches);
