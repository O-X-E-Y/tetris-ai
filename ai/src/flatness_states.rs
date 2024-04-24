// use itertools::Itertools;

// use std::{fs::OpenOptions, io::Write};

// use game::{board::BOARD_SIZE, pieces::Piece, rng::Rng};

// use crate::TetrisAi;

// fn rate_state(a: u8, b: u8, c: u8) -> u32 {
//     assert_eq!(a % 10, 0);
//     assert_eq!(b % 10, 0);
//     assert_eq!(c % 10, 0);

//     let min = a.min(b).min(c);
//     let (ai, bi, ci) = (
//         ((a - min) / 10) as i16,
//         ((b - min) / 10) as i16,
//         ((c - min) / 10) as i16
//     );
//     match (bi - ci, ci - bi) {
        
//     }
// }

// fn rate_board_state<R: Rng>(ai: &mut TetrisAi<R>, depth: usize) {
//     for placement in ai.search() {
//         for seq in Piece::PIECES.into_iter().combinations_with_replacement(depth) {
//             let holes = 0u64;
//             for piece
//         }
//     }
// }

// #[test]
// fn gen() {
//     // let mut buf = String::new();
//     // buf.push_str("pub const STATES: [([u8; 3], u8); 1800] = [\n");
//     // for i in (0..BOARD_SIZE).step_by(10) {
//     //     for j in (0..BOARD_SIZE).step_by(10) {
//     //         for k in (0..BOARD_SIZE).step_by(10) {
//     //             buf.push_str(&format!("    ([{i}, {j}, {k}], 25),\n"))
//     //         }
//     //     }
//     // }
//     // buf.push_str("];\n");

//     let mut buf = String::new();
//     buf.push_str("pub const fn rate_state(state: &[u8]) -> u8 {\n");
//     buf.push_str("    match state {\n");
//     for i in (0..BOARD_SIZE).step_by(10) {
//         for j in (0..BOARD_SIZE).step_by(10) {
//             for k in (0..BOARD_SIZE).step_by(10) {
//                 buf.push_str(&format!("        [{i}, {j}, {k}] => 25,\n"))
//             }
//         }
//     }
//     buf.push_str("        _ => 0\n}    }\n");
    
//     let mut f = OpenOptions::new()
//         .write(true)
//         .truncate(true)
//         .open("./src/states.rs")
//         .unwrap();

//     f.write_all(buf.as_bytes()).unwrap();
// }