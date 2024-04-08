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
    input_speed: u8,
    level: u8,
) -> TetrisAi<ClassicRng> {
    let mut ai = TetrisAi::<ClassicRng>::new(input_speed, level);

    ai.game.current = piece;
    ai.game.pos = ai.game.current.start_pos();

    ai
}
