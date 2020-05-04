use crate::board_representation::bitboard::BitBoard;

pub const RANK_1_BITBOARD: BitBoard = BitBoard::from_shifted(0xff);
pub const RANK_2_BITBOARD: BitBoard = BitBoard::from_shifted(0xff00);
pub const RANK_3_BITBOARD: BitBoard = BitBoard::from_shifted(0xff0000);
pub const RANK_4_BITBOARD: BitBoard = BitBoard::from_shifted(0xff000000);
pub const RANK_5_BITBOARD: BitBoard = BitBoard::from_shifted(0xff00000000);
pub const RANK_6_BITBOARD: BitBoard = BitBoard::from_shifted(0xff0000000000);
pub const RANK_7_BITBOARD: BitBoard = BitBoard::from_shifted(0xff000000000000);
pub const RANK_8_BITBOARD: BitBoard = BitBoard::from_shifted(0xff00000000000000);