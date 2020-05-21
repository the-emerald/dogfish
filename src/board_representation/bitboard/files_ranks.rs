use crate::board_representation::bitboard::BitBoard;

pub const FILE_A_BITBOARD: BitBoard = BitBoard { board: 0x0101_0101_0101_0101};
pub const FILE_B_BITBOARD: BitBoard = BitBoard { board: 0x0202_0202_0202_0202};
pub const FILE_C_BITBOARD: BitBoard = BitBoard { board: 0x0404_0404_0404_0404};
pub const FILE_D_BITBOARD: BitBoard = BitBoard { board: 0x0808_0808_0808_0808};
pub const FILE_E_BITBOARD: BitBoard = BitBoard { board: 0x1010_1010_1010_1010};
pub const FILE_F_BITBOARD: BitBoard = BitBoard { board: 0x2020_2020_2020_2020};
pub const FILE_G_BITBOARD: BitBoard = BitBoard { board: 0x4040_4040_4040_4040};
pub const FILE_H_BITBOARD: BitBoard = BitBoard { board: 0x8080_8080_8080_8080};

pub const RANK_1_BITBOARD: BitBoard = BitBoard { board: 0xff};
pub const RANK_2_BITBOARD: BitBoard = BitBoard { board: 0xff00};
pub const RANK_3_BITBOARD: BitBoard = BitBoard { board: 0x00ff_0000};
pub const RANK_4_BITBOARD: BitBoard = BitBoard { board: 0xff00_0000};
pub const RANK_5_BITBOARD: BitBoard = BitBoard { board: 0x00ff_0000_0000};
pub const RANK_6_BITBOARD: BitBoard = BitBoard { board: 0xff00_0000_0000};
pub const RANK_7_BITBOARD: BitBoard = BitBoard { board: 0x00ff_0000_0000_0000};
pub const RANK_8_BITBOARD: BitBoard = BitBoard { board: 0xff00_0000_0000_0000};