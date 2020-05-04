use crate::board_representation::bitboard::BitBoard;
use crate::board_representation::bitboard::files::{FILE_H_BITBOARD, FILE_A_BITBOARD};

pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
impl BitBoard {
    pub fn shift(direction: Direction, bitboard: Self) -> Self {
        match direction {
            Direction::North => { bitboard << BitBoard::from_shifted(8) },
            Direction::NorthEast => { (bitboard & !FILE_H_BITBOARD) << BitBoard::from_shifted(9) },
            Direction::East => { (bitboard & !FILE_H_BITBOARD) << BitBoard::from_shifted(1) },
            Direction::SouthEast => { (bitboard & !FILE_H_BITBOARD) >> BitBoard::from_shifted(7) },
            Direction::South => {bitboard >> BitBoard::from_shifted(8)},
            Direction::SouthWest => { (bitboard & !FILE_A_BITBOARD) >> BitBoard::from_shifted(9)},
            Direction::West => { (bitboard & !FILE_A_BITBOARD) >> BitBoard::from_shifted(1)},
            Direction::NorthWest => { (bitboard & !FILE_A_BITBOARD) << BitBoard::from_shifted(7) },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board_representation::bitboard::BitBoard;
    use crate::board_representation::bitboard::shift::Direction;
    use crate::board_representation::bitboard::ranks::{RANK_1_BITBOARD, RANK_2_BITBOARD, RANK_3_BITBOARD, RANK_8_BITBOARD, RANK_7_BITBOARD};
    use crate::board_representation::bitboard::files::{FILE_A_BITBOARD, FILE_B_BITBOARD, FILE_H_BITBOARD, FILE_G_BITBOARD};

    #[test]
    fn shift_north() {
        assert_eq!(
            BitBoard::shift(Direction::North, RANK_1_BITBOARD),
            RANK_2_BITBOARD
        )
    }

    #[test]
    fn shift_northeast() {
        let a1 = FILE_A_BITBOARD | RANK_1_BITBOARD;
        assert_eq!(
            BitBoard::shift(Direction::NorthEast, a1),
            BitBoard::from_shifted(0x20202020202fe00)
        )
    }

    #[test]
    fn shift_east() {
        assert_eq!(
            BitBoard::shift(Direction::East, FILE_A_BITBOARD),
            FILE_B_BITBOARD
        )
    }

    #[test]
    fn shift_southeast() {
        let a8 = FILE_A_BITBOARD | RANK_8_BITBOARD;
        assert_eq!(
            BitBoard::shift(Direction::SouthEast, a8),
            BitBoard::from_shifted(0xfe020202020202)
        )
    }

    #[test]
    fn shift_south() {
        assert_eq!(
            BitBoard::shift(Direction::South, RANK_8_BITBOARD),
            RANK_7_BITBOARD
        )
    }

    #[test]
    fn shift_southwest() {
        let h8 = FILE_H_BITBOARD | RANK_8_BITBOARD;
        assert_eq!(
            BitBoard::shift(Direction::SouthWest, h8),
            BitBoard::from_shifted(0x7f404040404040)
        )
    }

    #[test]
    fn shift_west() {
        assert_eq!(
            BitBoard::shift(Direction::West, FILE_H_BITBOARD),
            FILE_G_BITBOARD
        )
    }

    #[test]
    fn shift_northwest() {
        let h1 = FILE_H_BITBOARD | RANK_1_BITBOARD;
        assert_eq!(h1, BitBoard::from_shifted(0x80808080808080ff));
        assert_eq!(
            BitBoard::shift(Direction::NorthWest, h1),
            BitBoard::from_shifted(0x4040404040407f00)
        )
    }

    #[test]
    fn shift_h8_north() {
        let h8 = BitBoard::from_shifted(1 << 63);
        assert_eq!(
            BitBoard::shift(Direction::North, h8),
            BitBoard::zero()
        );

        assert_eq!(
            BitBoard::shift(Direction::NorthEast, h8),
            BitBoard::zero()
        );

        assert_eq!(
            BitBoard::shift(Direction::East, h8),
            BitBoard::zero()
        );

    }

    #[test]
    fn shift_full_board() {
        let all = BitBoard::from_shifted(u64::MAX);
        assert_eq!(
            BitBoard::shift(Direction::North, all),
            all ^ RANK_1_BITBOARD
        );

        assert_eq!(
            BitBoard::shift(Direction::NorthEast, all),
            all ^ (FILE_A_BITBOARD | RANK_1_BITBOARD)
        );

        assert_eq!(
            BitBoard::shift(Direction::East, all),
            all ^ FILE_A_BITBOARD
        );

        assert_eq!(
            BitBoard::shift(Direction::SouthEast, all),
            all ^ (FILE_A_BITBOARD | RANK_8_BITBOARD)
        );

        assert_eq!(
            BitBoard::shift(Direction::South, all),
            all ^ RANK_8_BITBOARD
        );

        assert_eq!(
            BitBoard::shift(Direction::SouthWest, all),
            all ^ (FILE_H_BITBOARD | RANK_8_BITBOARD)
        );

        assert_eq!(
            BitBoard::shift(Direction::West, all),
            all ^ FILE_H_BITBOARD
        );

        assert_eq!(
            BitBoard::shift(Direction::NorthWest, all),
            all ^ (FILE_H_BITBOARD | RANK_1_BITBOARD)
        );

    }
}