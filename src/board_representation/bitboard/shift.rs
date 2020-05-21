use crate::board_representation::bitboard::BitBoard;
use crate::board_representation::bitboard::files_ranks::{FILE_H_BITBOARD, FILE_A_BITBOARD};

#[derive(Copy, Clone)]
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
    pub fn shift(self, direction: Direction) -> Self {
        match direction {
            Direction::North => { self << 8.into() },
            Direction::NorthEast => { (self & !FILE_H_BITBOARD) << 9.into() },
            Direction::East => { (self & !FILE_H_BITBOARD) << 1.into() },
            Direction::SouthEast => { (self & !FILE_H_BITBOARD) >> 7.into() },
            Direction::South => {self >> 8.into()},
            Direction::SouthWest => { (self & !FILE_A_BITBOARD) >> 9.into() },
            Direction::West => { (self & !FILE_A_BITBOARD) >> 1.into() },
            Direction::NorthWest => { (self & !FILE_A_BITBOARD) << 7.into() },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board_representation::bitboard::BitBoard;
    use crate::board_representation::bitboard::shift::Direction;
    use crate::board_representation::bitboard::files_ranks::{FILE_A_BITBOARD, FILE_B_BITBOARD, FILE_H_BITBOARD, FILE_G_BITBOARD, RANK_2_BITBOARD, RANK_1_BITBOARD, RANK_8_BITBOARD, RANK_7_BITBOARD};

    #[test]
    fn shift_north() {
        assert_eq!(
            RANK_1_BITBOARD.shift(Direction::North),
            RANK_2_BITBOARD
        );
    }

    #[test]
    fn shift_northeast() {
        let a1 = FILE_A_BITBOARD | RANK_1_BITBOARD;
        assert_eq!(
            a1.shift(Direction::NorthEast),
            0x20202020202fe00.into()
        )
    }

    #[test]
    fn shift_east() {
        assert_eq!(
            FILE_A_BITBOARD.shift(Direction::East),
            FILE_B_BITBOARD
        )
    }

    #[test]
    fn shift_southeast() {
        let a8 = FILE_A_BITBOARD | RANK_8_BITBOARD;
        assert_eq!(
            a8.shift(Direction::SouthEast),
            0xfe020202020202.into()
        )
    }

    #[test]
    fn shift_south() {
        assert_eq!(
            RANK_8_BITBOARD.shift(Direction::South),
            RANK_7_BITBOARD
        )
    }

    #[test]
    fn shift_southwest() {
        let h8 = FILE_H_BITBOARD | RANK_8_BITBOARD;
        assert_eq!(
            h8.shift(Direction::SouthWest),
            0x7f404040404040.into()
        )
    }

    #[test]
    fn shift_west() {
        assert_eq!(
            FILE_H_BITBOARD.shift(Direction::West),
            FILE_G_BITBOARD
        )
    }

    #[test]
    fn shift_northwest() {
        let h1 = FILE_H_BITBOARD | RANK_1_BITBOARD;
        assert_eq!(h1, 0x80808080808080ff.into());
        assert_eq!(
            h1.shift(Direction::NorthWest),
            0x4040404040407f00.into()
        )
    }

    #[test]
    fn shift_h8() {
        let h8: BitBoard = (1 << 63).into();
        assert_eq!(
            h8.shift(Direction::North),
            0.into()
        );

        assert_eq!(
            h8.shift(Direction::NorthEast),
            0.into()
        );

        assert_eq!(
            h8.shift(Direction::East),
            0.into()
        );

    }

    #[test]
    fn shift_full_board() {
        let all: BitBoard = u64::MAX.into();
        assert_eq!(
            all.shift(Direction::North),
            all ^ RANK_1_BITBOARD
        );

        assert_eq!(
            all.shift(Direction::NorthEast),
            all ^ (FILE_A_BITBOARD | RANK_1_BITBOARD)
        );

        assert_eq!(
            all.shift(Direction::East),
            all ^ FILE_A_BITBOARD
        );

        assert_eq!(
            all.shift(Direction::SouthEast),
            all ^ (FILE_A_BITBOARD | RANK_8_BITBOARD)
        );

        assert_eq!(
            all.shift(Direction::South),
            all ^ RANK_8_BITBOARD
        );

        assert_eq!(
            all.shift(Direction::SouthWest),
            all ^ (FILE_H_BITBOARD | RANK_8_BITBOARD)
        );

        assert_eq!(
            all.shift(Direction::West),
            all ^ FILE_H_BITBOARD
        );

        assert_eq!(
            all.shift(Direction::NorthWest),
            all ^ (FILE_H_BITBOARD | RANK_1_BITBOARD)
        );

    }
}