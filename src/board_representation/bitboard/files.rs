use crate::board_representation::bitboard::BitBoard;

pub const FILE_A_BITBOARD: BitBoard = BitBoard::from_shifted(0x101010101010101);
pub const FILE_B_BITBOARD: BitBoard = BitBoard::from_shifted(0x202020202020202);
pub const FILE_C_BITBOARD: BitBoard = BitBoard::from_shifted(0x303030303030303);
pub const FILE_D_BITBOARD: BitBoard = BitBoard::from_shifted(0x404040404040404);
pub const FILE_E_BITBOARD: BitBoard = BitBoard::from_shifted(0x505050505050505);
pub const FILE_F_BITBOARD: BitBoard = BitBoard::from_shifted(0x606060606060606);
pub const FILE_G_BITBOARD: BitBoard = BitBoard::from_shifted(0x707070707070707);
pub const FILE_H_BITBOARD: BitBoard = BitBoard::from_shifted(0x808080808080808);