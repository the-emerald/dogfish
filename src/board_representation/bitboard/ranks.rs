use crate::board_representation::bitboard::BitBoard;

pub const RANK_1_BITBOARD: BitBoard = BitBoard::from_shifted(0xff);
pub const RANK_2_BITBOARD: BitBoard = BitBoard::from_shifted(0xff00);
pub const RANK_3_BITBOARD: BitBoard = BitBoard::from_shifted(0x00ff_0000);
pub const RANK_4_BITBOARD: BitBoard = BitBoard::from_shifted(0xff00_0000);
pub const RANK_5_BITBOARD: BitBoard = BitBoard::from_shifted(0x00ff_0000_0000);
pub const RANK_6_BITBOARD: BitBoard = BitBoard::from_shifted(0xff00_0000_0000);
pub const RANK_7_BITBOARD: BitBoard = BitBoard::from_shifted(0x00ff_0000_0000_0000);
pub const RANK_8_BITBOARD: BitBoard = BitBoard::from_shifted(0xff00_0000_0000_0000);