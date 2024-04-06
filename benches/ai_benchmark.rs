mod util;

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
        b.iter(|| {
            board.board.try_left(board.pos)
        });
    });
}

pub fn bench_try_right(c: &mut Criterion) {
    let mut board = util::game_with_start_piece(game::pieces::Piece::L, 19);

    board.down();
    board.down();

    c.bench_function("try right L", |b| {
        b.iter(|| {
            board.board.try_right(board.pos)
        });
    });
}

pub fn bench_try_rot_cw(c: &mut Criterion) {
    let piece = game::pieces::Piece::L;
    let mut board = util::game_with_start_piece(piece, 19);

    board.down();
    board.down();

    c.bench_function("try rotating L clockwise", |b| {
        b.iter(|| {
            board.board.try_rot_cw(board.pos, board.rot, piece)
        });
    });
}

pub fn bench_try_rot_ccw(c: &mut Criterion) {
    let piece = game::pieces::Piece::L;
    let mut board = util::game_with_start_piece(piece, 19);

    board.down();
    board.down();

    c.bench_function("try rotating L counter clockwise", |b| {
        b.iter(|| {
            board.board.try_rot_ccw(board.pos, board.rot, piece)
        });
    });
}

pub fn bench_find_best_move(c: &mut Criterion) {
    let mut ai = ai::TetrisAi::<game::rng::ClassicRng>::new(10, 19);

    c.bench_function("find best move in empty board", |b| {
        b.iter(|| {
            ai.find_best_move()
        });
    });
}

criterion_group!(benches, bench_find_best_move);
criterion_main!(benches);