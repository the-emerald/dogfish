use once_cell::sync::Lazy;
use crate::board_representation::bitboard::BitBoard;
use crate::board_representation::square::Square;
use std::convert::TryFrom;
use crate::piece::piecetype::PieceType;
use itertools::iproduct;

pub mod moves;

pub static LINE_BETWEEN: Lazy<[[BitBoard; 64]; 64]> = Lazy::new(|| {
    let mut lb: [[BitBoard; 64]; 64] = [[BitBoard::new(0); 64]; 64];

    let all_sq_1 = (0_u64..64).map(|s| Square::try_from(s).unwrap());
    let all_sq_2 = (0_u64..64).map(|s| Square::try_from(s).unwrap());

    for (s1, s2) in iproduct!(all_sq_1, all_sq_2) {
        let start_finish: BitBoard = BitBoard::from(s1) | BitBoard::from(s2);

        if (PieceType::bishop_attack(s1, 0.into()) & s2.into()) != 0.into() {
            lb[u64::from(s1) as usize][u64::from(s2) as usize] =
                (
                    PieceType::bishop_attack(s1, s2.into()) &
                    PieceType::bishop_attack(s2, s1.into())
                ) | start_finish;
        }
        else if (PieceType::rook_attack(s1, 0.into()) & s2.into()) != 0.into() {
            lb[u64::from(s1) as usize][u64::from(s2) as usize] =
                (
                    PieceType::rook_attack(s1, s2.into()) &
                    PieceType::rook_attack(s2, s1.into())
                ) | start_finish;
        }
    }

    lb
});

#[cfg(test)]
mod tests {
    use crate::board_representation::square::Square;
    use std::convert::TryInto;
    use crate::common::LINE_BETWEEN;
    use crate::board_representation::bitboard::BitBoard;

    #[test]
    fn line_between_rook() {
        let point_1: Square = 9_u64.try_into().unwrap();
        let point_2: Square = 49_u64.try_into().unwrap();

        println!("{:?}", BitBoard::from(point_1));
        println!("{:?}", BitBoard::from(point_2));

        let line_between = LINE_BETWEEN[u64::from(point_1) as usize][u64::from(point_2) as usize];

        println!("{:?}", line_between);

        assert_eq!(line_between, BitBoard::from(0x2020202020200))

    }

    #[test]
    fn line_between_bishop() {
        let point_1: Square = 10_u64.try_into().unwrap();
        let point_2: Square = 46_u64.try_into().unwrap();

        println!("{:?}", BitBoard::from(point_1));
        println!("{:?}", BitBoard::from(point_2));

        let line_between = LINE_BETWEEN[u64::from(point_1) as usize][u64::from(point_2) as usize];

        println!("{:?}", line_between);

        assert_eq!(line_between, BitBoard::from(0x402010080400))

    }
}