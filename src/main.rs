use std::collections::HashSet;

use ai::TetrisAi;
use game::{board::*, pieces::Piece, rng::*, Game, row_board::{RowBoard, PiecePos as PP}};

fn main() {
    // piece_rot();

    // play();

    // let mut b = RowBoard::new();
    // let pos = Piece::T.row_start_pos();
    // let pos = b.try_left(pos).unwrap();
    // let pos = b.try_left(pos).unwrap();
    // let (pos, rot) = b.try_rot_ccw(pos, Piece::T, game::pieces::Rotation::Right).unwrap();
    // let pos = b.try_left(pos).unwrap();
    // // let pos = b.try_left(pos).unwrap();
    // // let pos = b.try_left(pos).unwrap();
    // // let pos = b.try_left(pos).unwrap();
    // // let pos = b.try_left(pos).unwrap();
    // println!("{pos:?}");
    // // println!("{b}");
    // // b.lock(pos);
    // println!("{b}");

    let mut g = game::RowGame::<ClassicRng>::new(1);
    println!("{g}");

    g.down().unwrap();
    g.left().unwrap();
    g.left().unwrap();
    g.right().unwrap();
    g.left().unwrap();
    
    println!("{g}");

    g.rot_ccw().unwrap();
    g.rot_cw();

    println!("{g}");

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
    let mut g = Game::<OrderedRng>::new(19);

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

    g.left();

    println!("{g}");

    g.rot_cw();

    println!("{g}");

    g.right();
    g.rot_cw();
    g.drop();
    g.down();
    g.down();

    println!("{g}");
    println!("{:?}: {:?}", g.rot, g.pos);
    g.rot_cw();
    println!("{:?}: {:?}", g.rot, g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}: {:?}", g.rot, g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}: {:?}", g.rot, g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}: {:?}", g.rot, g.pos);
    println!("{g}");

    g.right();
    g.drop();

    g.down();

    println!("{g}");

    println!("{:?}: {:?}", g.rot, g.pos);
    g.rot_cw();
    println!("{:?}: {:?}", g.rot, g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}: {:?}", g.rot, g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}: {:?}", g.rot, g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}: {:?}", g.rot, g.pos);
    println!("{g}");

    g.left();
    g.left();
    g.drop();

    g.down();
    g.right();
    g.right();
    g.right();

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
    g.drop();

    g.down();
    g.down();

    println!("{g}");

    println!("{:?}: {:?}", g.rot, g.pos);
    g.rot_cw();
    println!("{:?}: {:?}", g.rot, g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}: {:?}", g.rot, g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}: {:?}", g.rot, g.pos);
    println!("{g}");
    g.rot_cw();
    println!("{:?}: {:?}", g.rot, g.pos);
    println!("{g}");

    g.rot_ccw();
    g.rot_ccw();
    g.left();
    g.left();
    g.drop();

    println!("{g}");

    // g.rot_ccw();

    // g.right();
    // g.right();
    // g.right();
    // g.right();
    // for _ in 0..18 {
    //     g.down();
    // }

    // println!("{g}");

    // g.rot_ccw();

    // println!("{g}");

    // g.rot_ccw();

    // println!("{g}");

    // g.rot_cw();

    // println!("{g}");

    // g.drop();

    // println!("{g}");

    println!("{:?}", g.board.find_highest_blocks());

    // g.left();

    // println!("{g}");

    let mut ai = ai::TetrisAi::from_game(g, 3);

    let score = ai.eval();

    println!("eval score: {}", score);

    let pos = time_this::time!(ai.search());
    let unique = pos.iter().collect::<HashSet<_>>();

    for u in &pos {
        ai.game.pos = *u;

        println!("{}", ai.game);

        let highest_blocks_old = ai.highest_blocks;

        for p in ai.game.pos {
            ai.game.board.0[p as usize] = Some(ai.game.current);

            if ai.highest_blocks[(p % 10) as usize] > p {
                ai.highest_blocks[(p % 10) as usize] = p;
            }
        }

        let score = time_this::time!(ai.eval());

        println!("eval score: {}", score);

        for p in ai.game.pos {
            ai.game.board.0[p as usize] = None;
        }

        ai.highest_blocks = highest_blocks_old;
    }

    println!(
        "found {} final positions out of which {} are unique!",
        pos.len(),
        unique.len()
    );
}
