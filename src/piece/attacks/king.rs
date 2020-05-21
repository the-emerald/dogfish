use crate::board_representation::bitboard::shift::Direction;
use crate::board_representation::bitboard::shift::Direction::{
    East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West,
};
use crate::board_representation::bitboard::BitBoard;
use crate::board_representation::square::Square;
use once_cell::sync::Lazy;
use std::convert::TryFrom;

const KING_ATTACKS: [Direction; 8] = [
    North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
];

pub static ATTACK_TABLE_KING: Lazy<[BitBoard; 64]> = Lazy::new(|| {
    let mut bbs: [BitBoard; 64] = [0_u64.into(); 64];
    // For each square on the board
    for (square, bitboard) in bbs.iter_mut().enumerate() {
        // Apply the eight directions once
        *bitboard = KING_ATTACKS.iter().fold(0_u64.into(), |acc, step| {
            acc | {
                let b: BitBoard = Square::try_from(square as u64).unwrap().into();
                b.shift(*step)
            }
        });
    }
    bbs
});
