use ai::TetrisAi;
use game::{pieces::Piece, rng::ClassicRng, Game};

pub fn game_with_start_piece(piece: Piece, level: u8) -> Game<ClassicRng> {
    let mut game = Game::<ClassicRng>::new(level);

    game.current = piece;
    game.pos = game.current.start_pos();

    game
}

pub fn tetris_ai_with_start_piece(
    piece: Piece,
    level: u8,
) -> TetrisAi<ClassicRng> {
    let mut ai = TetrisAi::<ClassicRng>::new(level);

    ai.current = piece;
    ai.pos = ai.current.start_pos();

    ai
}
