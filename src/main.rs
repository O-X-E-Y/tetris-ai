use std::collections::HashSet;

use ai::TetrisAi;
use game::{
    board::*,
    pieces::Piece,
    rng::*,
    row_board::{PiecePos as PP, RowBoard},
    Game,
};

fn main() {
    piece_rot();

    // play();
}

pub fn play() {
    let mut ai = TetrisAi::<SevenBag>::new(10, 19);

    let stdin = std::io::stdin();

    loop {
        match time_this::time!(ai.find_best_move()) {
            Some((pos, score)) => {
                ai.game.pos = pos;

                for p in ai.game.pos {
                    ai.game.board.0[p as usize] = Some(ai.game.current);

                    if ai.highest_blocks[(p % 10) as usize] > p {
                        ai.highest_blocks[(p % 10) as usize] = p;
                    }
                }

                println!("chosen board:\n{}\nscore: {}", ai.game.board, score);

                ai.game.lock();

                let _ = stdin.read_line(&mut String::new());
            }
            None => panic!("no possible moves found! board:\n{}", ai.game.board),
        }
    }
}

pub fn piece_rot() {
    let mut g = game::RowGame::<OrderedRng>::new(19);

    println!("{:?}", g.pos);
    for m in g.pos.get_masks().unwrap() {
        println!("{m:b}");
    }
    println!("{g}");

    g.left();
    g.left();
    g.left();
    g.left();
    g.left();
    g.down();
    g.down();
    g.down();

    println!("{g}");

    g.rot_cw();

    println!("{g}");
    // println!("{:?}", g.pos);

    g.left();
    g.left();
    g.left();
    println!("{g}");

    g.right();
    g.right();
    g.rot_cw();
    g.rot_ccw();
    g.right();
    g.right();
    g.right();
    g.right();
    g.right();
    g.right();
    g.right();
    g.right();
    g.right();

    println!("{g}");

    g.left();
    g.rot_ccw();
    g.left();
    g.left();
    g.left();
    g.left();
    g.left();
    g.left();
    g.left();
    g.left();
    g.drop_piece();
    g.down();
    g.down();

    println!("{g}");

    println!("{g}");
    println!("{:?}", g.pos);
    g.rot_cw();
    println!("{:?}", g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}", g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}", g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}", g.pos);
    println!("{g}");

    g.left();
    g.rot_ccw();
    g.drop_piece();

    g.down();

    println!("{g}");

    println!("{:?}", g.pos);
    g.rot_cw();
    println!("{:?}", g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}", g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}", g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}", g.pos);
    println!("{g}");

    g.left();
    g.left();
    g.rot_cw();

    println!("{g}");
    g.drop_piece();

    g.down();

    println!("{g}");

    g.left();
    g.left();
    g.left();
    g.left();
    g.left();
    g.left();
    g.left();
    g.left();
    g.rot_cw();
    g.rot_cw();

    g.drop_piece();
    g.down();
    g.down();

    println!("{g}");

    println!("{:?}", g.pos);
    g.rot_cw();
    println!("{:?}", g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}", g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}", g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}", g.pos);
    println!("{g}");

    g.right();
    g.drop_piece();
    // g.down();
    // g.right();
    // g.right();
    // g.right();
    // // g.down();
    // g.rot_ccw();
    
    // println!("{g}");

    // for _ in 0..18 {
    //     g.down();
    // }

    // println!("{g}");

    // g.rot_cw();

    // println!("{g}");

    // g.rot_ccw();

    // println!("{g}");

    // g.rot_ccw();

    // println!("{g}");

    // g.drop_piece();

    // println!("{g}");

    // println!("{:?}", g.board.find_highest_blocks());

    // g.left();

    println!("{g}");

    let mut ai = ai::row_ai::RowTetrisAi::from_game(g, 3);

    let score = ai.eval();

    println!("eval score: {}", score);

    let pos = time_this::time!(ai.search());
    let unique = pos.iter().collect::<HashSet<_>>();

    for u in &pos {
        ai.game.pos = *u;

        println!("{}", ai.game);

        let highest_blocks_old = ai.highest_blocks;

        let masks = ai.game.pos.get_masks().unwrap();
        masks.into_iter().enumerate().for_each(|(i, m)|
            ai.game.board.0[ai.game.pos.y as usize + i] |= m
        );

        let score = time_this::time!(ai.eval());

        println!("eval score: {}", score);

        masks.into_iter().enumerate().for_each(|(i, m)|
            ai.game.board.0[ai.game.pos.y as usize + i] ^= m
        );

        ai.highest_blocks = highest_blocks_old;
    }

    println!(
        "found {} final positions out of which {} are unique!",
        pos.len(),
        unique.len()
    );
}
