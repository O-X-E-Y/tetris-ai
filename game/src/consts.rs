use crate::PiecePos;

#[inline(always)]
pub const fn apply_mask(pos: PiecePos, mask: i64) -> PiecePos {
    let u = i32::from_le_bytes(pos) as i64;
    let new = u + mask;
    u32::to_le_bytes(new as u32)
}

pub const I_RIGHT_LEFT_CW: i64 = 150992622;
pub const I_DOWN_UP_CW: i64 = -150992622;
pub const L_RIGHT_CW: i64 = 16709110;
pub const L_DOWN_CW: i64 = -150992638;
pub const L_LEFT_CW: i64 = 168427775;
pub const L_UP_CW: i64 = -34144247;
pub const J_RIGHT_CW: i64 = -32964617;
pub const J_DOWN_CW: i64 = -151585025;
pub const J_LEFT_CW: i64 = 167704577;
pub const J_UP_CW: i64 = 16845065;
pub const T_RIGHT_CW: i64 = -65801;
pub const T_DOWN_CW: i64 = -150994944;
pub const T_LEFT_CW: i64 = 151060736;
pub const T_UP_CW: i64 = 9;
pub const S_RIGHT_LEFT_CW: i64 = 16252662;
pub const S_DOWN_UP_CW: i64 = -16252662;
pub const Z_RIGHT_LEFT_CW: i64 = -17367048;
pub const Z_DOWN_UP_CW: i64 = 17367048;

pub const I_RIGHT_LEFT_CCW: i64 = 150992622;
pub const I_DOWN_UP_CCW: i64 = -150992622;
pub const L_RIGHT_CCW: i64 = 34144247;
pub const L_DOWN_CCW: i64 = -16709110;
pub const L_LEFT_CCW: i64 = 150992638;
pub const L_UP_CCW: i64 = -168427775;
pub const J_RIGHT_CCW: i64 = -16845065;
pub const J_DOWN_CCW: i64 = 32964617;
pub const J_LEFT_CCW: i64 = 151585025;
pub const J_UP_CCW: i64 = -167704577;
pub const T_RIGHT_CCW: i64 = -9;
pub const T_DOWN_CCW: i64 = 65801;
pub const T_LEFT_CCW: i64 = 150994944;
pub const T_UP_CCW: i64 = -151060736;
pub const S_RIGHT_LEFT_CCW: i64 = 16252662;
pub const S_DOWN_UP_CCW: i64 = -16252662;
pub const Z_RIGHT_LEFT_CCW: i64 = -17367048;
pub const Z_DOWN_UP_CCW: i64 = 17367048;

pub const MOVE_UP: i64 = -168430090;
pub const MOVE_LEFT: i64 = -16843009;
pub const MOVE_DOWN: i64 = 168430090;
pub const MOVE_RIGHT: i64 = 16843009;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cw_circular() {
        assert_eq!(I_RIGHT_LEFT_CW + I_DOWN_UP_CW, 0);
        assert_eq!(S_RIGHT_LEFT_CW + S_DOWN_UP_CW, 0);
        assert_eq!(Z_RIGHT_LEFT_CW + Z_DOWN_UP_CW, 0);

        assert_eq!(L_RIGHT_CW + L_DOWN_CW + L_LEFT_CW + L_UP_CW, 0);
        assert_eq!(J_RIGHT_CW + J_DOWN_CW + J_LEFT_CW + J_UP_CW, 0);
        assert_eq!(T_RIGHT_CW + T_DOWN_CW + T_LEFT_CW + T_UP_CW, 0);
    }

    #[test]
    fn test_ccw_circular() {
        assert_eq!(I_RIGHT_LEFT_CCW + I_DOWN_UP_CCW, 0);
        assert_eq!(S_RIGHT_LEFT_CCW + S_DOWN_UP_CCW, 0);
        assert_eq!(Z_RIGHT_LEFT_CCW + Z_DOWN_UP_CCW, 0);

        assert_eq!(L_RIGHT_CCW + L_DOWN_CCW + L_LEFT_CCW + L_UP_CCW, 0);
        assert_eq!(J_RIGHT_CCW + J_DOWN_CCW + J_LEFT_CCW + J_UP_CCW, 0);
        assert_eq!(T_RIGHT_CCW + T_DOWN_CCW + T_LEFT_CCW + T_UP_CCW, 0);
    }

    #[test]
    fn generate_cw() {
        let cw = [
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 18, p2 - 9, p3, p4 + 9], "I_Right_Left"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 18, p2 + 9, p3, p4 - 9], "I_Down_Up"),

            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 10, p2 - 10, p3 - 1, p4 + 1], "L_Right"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 2, p2 + 9, p3, p4 - 9], "L_Down"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 1, p2 + 1, p3 + 10, p4 + 10], "L_Left"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 9, p2, p3 - 9, p4 - 2], "L_Up"),

            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 9, p2, p3 + 9, p4 - 2], "J_Right"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 1, p2 - 1, p3 - 9, p4 - 9], "J_Down"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 1, p2 - 8, p3 - 1, p4 + 10], "J_Left"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 9, p2 + 9, p3 + 1, p4 + 1], "J_Up"),

            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 9, p2 - 1, p3 - 1, p4], "T_Right"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1, p2, p3, p4 - 9], "T_Down"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1, p2 + 1, p3 + 1, p4 + 9], "T_Left"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 9, p2, p3, p4], "T_Up"),

            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 10, p2 - 1, p3 - 8, p4 + 1], "S_Right_Left"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 10, p2 + 1, p3 + 8, p4 - 1], "S_Down_Up"),
            
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 8, p2, p3 - 9, p4 - 1], "Z_Right_Left"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 8, p2, p3 + 9, p4 + 1], "Z_Down_Up"),
        ];

        for rot_fn in cw {
            let base = [20; 4];
            let base_u32 = u32::from_le_bytes(base);

            let (new, apply_to) = rot_fn(base);
            let new_u32 = u32::from_le_bytes(new);

            let diff = base_u32.abs_diff(new_u32);
            let op = if new_u32 < base_u32 { "-" } else { "" };

            println!("pub const {}_CW: i64 = {op}{diff};", apply_to.to_uppercase())
        }
    }

    #[test]
    fn generate_ccw() {
        let ccw = [
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 18, p2 - 9, p3, p4 + 9], "I_Right_Left"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 18, p2 + 9, p3, p4 - 9], "I_Down_Up"),

            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 9, p2, p3 + 9, p4 + 2], "L_Right"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 10, p2 + 10, p3 + 1, p4 - 1], "L_Down"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 2, p2 - 9, p3, p4 + 9], "L_Left"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 1, p2 - 1, p3 - 10, p4 - 10], "L_Up"),

            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 9, p2 - 9, p3 - 1, p4 - 1], "J_Right"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 9, p2, p3 - 9, p4 + 2], "J_Down"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 1, p2 + 1, p3 + 9, p4 + 9], "J_Left"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 1, p2 + 8, p3 + 1, p4 - 10], "J_Up"),

            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 9, p2, p3, p4], "T_Right"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 9, p2 + 1, p3 + 1, p4], "T_Down"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1, p2, p3, p4 + 9], "T_Left"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1, p2 - 1, p3 - 1, p4 - 9], "T_Up"),

            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 10, p2 - 1, p3 - 8, p4 + 1], "S_Right_Left"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 10, p2 + 1, p3 + 8, p4 - 1], "S_Down_Up"),

            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 8, p2, p3 - 9, p4 - 1], "Z_Right_Left"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 8, p2, p3 + 9, p4 + 1], "Z_Down_Up"),
        ];

        for rot_fn in ccw {
            let base = [20; 4];
            let base_u32 = u32::from_le_bytes(base);

            let (new, apply_to) = rot_fn(base);
            let new_u32 = u32::from_le_bytes(new);

            let diff = base_u32.abs_diff(new_u32);
            let op = if new_u32 < base_u32 { "-" } else { "" };

            println!("pub const {}_CCW: i64 = {op}{diff};", apply_to.to_uppercase())

            // println!("new bytes: {new:?}, old u32: {base_u32}, new_u32: {new_u32}, abs difference: {diff}");
        }
    }

    #[test]
    fn up_down_left_right() {
        let masks = [
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 10, p2 - 10, p3 - 10, p4 - 10], "UP"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 - 1, p2 - 1, p3 - 1, p4 - 1], "LEFT"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 10, p2 + 10, p3 + 10, p4 + 10], "DOWN"),
            |[p1, p2, p3, p4]: [u8; 4]| ([p1 + 1, p2 + 1, p3 + 1, p4 + 1], "RIGHT"),
        ];

        for move_fn in masks {
            let base = [20; 4];
            let base_u32 = u32::from_le_bytes(base);

            let (new, apply_to) = move_fn(base);
            let new_u32 = u32::from_le_bytes(new);

            let diff = base_u32.abs_diff(new_u32);
            let op = if new_u32 < base_u32 { "-" } else { "" };

            println!("pub const MOVE_{apply_to}: i64 = {op}{diff};")
        }
    }

    #[test]
    fn start_pos() {
        for piece in crate::Piece::PIECES {
            let mut pos = piece.positions(crate::Rotation::Right);
            pos
                .iter_mut()
                .for_each(|v| *v += piece.starting_pos());

            println!("pub const START_POS_{piece:?}: PiecePos = {pos:?};")
        }
    }
}