use crate::board_representation::bitboard::BitBoard;

pub const FILE_A_BITBOARD: BitBoard = BitBoard::from_shifted(0x0101_0101_0101_0101);
pub const FILE_B_BITBOARD: BitBoard = BitBoard::from_shifted(0x0202_0202_0202_0202);
pub const FILE_C_BITBOARD: BitBoard = BitBoard::from_shifted(0x0404_0404_0404_0404);
pub const FILE_D_BITBOARD: BitBoard = BitBoard::from_shifted(0x0808_0808_0808_0808);
pub const FILE_E_BITBOARD: BitBoard = BitBoard::from_shifted(0x1010_1010_1010_1010);
pub const FILE_F_BITBOARD: BitBoard = BitBoard::from_shifted(0x2020_2020_2020_2020);
pub const FILE_G_BITBOARD: BitBoard = BitBoard::from_shifted(0x4040_4040_4040_4040);
pub const FILE_H_BITBOARD: BitBoard = BitBoard::from_shifted(0x8080_8080_8080_8080);