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
    for (square, bitboard) in bbs.iter_mut().enumerate() {
        *bitboard = KING_ATTACKS.iter().fold(0_u64.into(), |acc, step| {
            acc | BitBoard::shift(*step, Square::try_from(square as u64).unwrap().into())
        });
    }
    bbs
});

#[cfg(test)]
mod tests {
    use crate::board_representation::bitboard::BitBoard;
    use crate::board_representation::square::Square;
    use std::convert::TryFrom;
    use crate::piece::attacks::king::ATTACK_TABLE_KING;

    #[test]
    fn king_table() {
        let square = Square::try_from(34_u64).unwrap();
        let square_bb: BitBoard = square.into();

        println!("{:?}", square_bb);
        println!("{:?}", ATTACK_TABLE_KING[34]);

    }
}
