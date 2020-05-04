use crate::board_representation::bitboard::BitBoard;

pub const FILE_A_BITBOARD: BitBoard = BitBoard::from_shifted(0x0101010101010101);
pub const FILE_B_BITBOARD: BitBoard = BitBoard::from_shifted(0x0202020202020202);
pub const FILE_C_BITBOARD: BitBoard = BitBoard::from_shifted(0x0404040404040404);
pub const FILE_D_BITBOARD: BitBoard = BitBoard::from_shifted(0x0808080808080808);
pub const FILE_E_BITBOARD: BitBoard = BitBoard::from_shifted(0x1010101010101010);
pub const FILE_F_BITBOARD: BitBoard = BitBoard::from_shifted(0x2020202020202020);
pub const FILE_G_BITBOARD: BitBoard = BitBoard::from_shifted(0x4040404040404040);
pub const FILE_H_BITBOARD: BitBoard = BitBoard::from_shifted(0x8080808080808080);