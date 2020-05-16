use crate::board_representation::bitboard::shift::Direction;
use crate::board_representation::bitboard::shift::Direction::{East, North, South, West};
use crate::board_representation::bitboard::BitBoard;
use crate::board_representation::square::Square;
use once_cell::sync::Lazy;
use std::convert::TryFrom;

const KNIGHT_ATTACKS: [[Direction; 3]; 8] = [
    [North, North, East],
    [North, East, East],
    [East, East, South],
    [East, South, South],
    [South, South, West],
    [South, West, West],
    [West, West, North],
    [West, North, North],
];

pub static ATTACK_TABLE_KNIGHT: Lazy<[BitBoard; 64]> = Lazy::new(|| {
    let mut bbs: [BitBoard; 64] = [0_u64.into(); 64];
    // For each square on the board
    for (square, bitboard) in bbs.iter_mut().enumerate() {
        // For each of the (up to) 8 attacked squares
        *bitboard = KNIGHT_ATTACKS.iter().fold(0_u64.into(), |attacks, steps| {
            attacks
                | steps.iter().fold(
                    // Apply the 3 shifts
                    BitBoard::from(Square::try_from(square as u64).unwrap()),
                    |square, step| BitBoard::shift(*step, square),
                )
        })
    }
    bbs
});
