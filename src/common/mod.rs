use once_cell::sync::Lazy;
use crate::board_representation::bitboard::BitBoard;
use crate::board_representation::square::Square;
use std::convert::TryFrom;
use crate::piece::piecetype::PieceType;
use itertools::iproduct;
use crate::board_representation::bitboard::files_ranks::FULL_BITBOARD;

pub mod moves;

pub static LINE_INTERSECTING: Lazy<[[BitBoard; 64]; 64]> = Lazy::new(|| {
    let mut lb: [[BitBoard; 64]; 64] = [[BitBoard::new(0); 64]; 64];

    let all_sq_1 = (0_u64..64).map(|s| Square::try_from(s).unwrap());
    let all_sq_2 = (0_u64..64).map(|s| Square::try_from(s).unwrap());

    for (s1, s2) in iproduct!(all_sq_1, all_sq_2) {
        let start_finish: BitBoard = BitBoard::from(s1) | BitBoard::from(s2);

        if (PieceType::bishop_attack(s1, 0.into()) & s2.into()) != 0.into() {
            lb[u64::from(s1) as usize][u64::from(s2) as usize] =
                (
                    PieceType::bishop_attack(s1, 0.into()) &
                    PieceType::bishop_attack(s2, 0.into())
                ) | start_finish;
        }
        else if (PieceType::rook_attack(s1, 0.into()) & s2.into()) != 0.into() {
            lb[u64::from(s1) as usize][u64::from(s2) as usize] =
                (
                    PieceType::rook_attack(s1, 0.into()) &
                    PieceType::rook_attack(s2, 0.into())
                ) | start_finish;
        }
    }
    lb
});

pub fn line_between(a: Square, b: Square) -> BitBoard {
    let b = LINE_INTERSECTING[u64::from(a) as usize][u64::from(b) as usize] &
        (
            (FULL_BITBOARD << a.into()) ^
            (FULL_BITBOARD << b.into())
        );
    b & (b - 1.into())
}

#[cfg(test)]
mod tests {
    use crate::board_representation::square::Square;
    use std::convert::TryInto;
    use crate::common::{LINE_INTERSECTING, line_between};
    use crate::board_representation::bitboard::BitBoard;

    #[test]
    fn line_intersecting_rook() {
        let point_1: Square = 9_u64.try_into().unwrap();
        let point_2: Square = 49_u64.try_into().unwrap();

        println!("Point 1:{:?}", BitBoard::from(point_1));
        println!("Point 2:{:?}", BitBoard::from(point_2));

        let line_between = LINE_INTERSECTING[u64::from(point_1) as usize][u64::from(point_2) as usize];

        println!("Connecting:{:?}", line_between);

        assert_eq!(line_between, BitBoard::from(0x202020202020202));
    }

    #[test]
    fn line_intersecting_bishop() {
        let point_1: Square = 10_u64.try_into().unwrap();
        let point_2: Square = 46_u64.try_into().unwrap();

        println!("Point 1:{:?}", BitBoard::from(point_1));
        println!("Point 2:{:?}", BitBoard::from(point_2));

        let line_between = LINE_INTERSECTING[u64::from(point_1) as usize][u64::from(point_2) as usize];

        println!("Connecting:{:?}", line_between);

        assert_eq!(line_between, BitBoard::from(0x80402010080402));
    }

    #[test]
    fn test_line_between() {
        let point_1: Square = 5_u64.try_into().unwrap();
        let point_2: Square = 61_u64.try_into().unwrap();

        println!("Point 1:{:?}", BitBoard::from(point_1));
        println!("Point 2:{:?}", BitBoard::from(point_2));

        let line_between = line_between(point_1, point_2);

        println!("Between:{:?}", line_between);

        assert_eq!(line_between, BitBoard::from(0x20202020202000));
    }

}