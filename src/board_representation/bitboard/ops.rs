// Define operations for bitboards

use std::ops::{BitOr, BitOrAssign, BitAnd, BitAndAssign, BitXor, BitXorAssign, Shl, Shr, Not};
use crate::board_representation::bitboard::BitBoard;

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard::from_shifted(self.board | rhs.board)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.board |= rhs.board
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard::from_shifted(self.board & rhs.board)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.board &= rhs.board
    }
}

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard::from_shifted(self.board ^ rhs.board)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.board ^= rhs.board
    }
}

impl Shl for BitBoard {
    type Output = BitBoard;

    fn shl(self, rhs: Self) -> Self::Output {
        BitBoard::from_shifted(self.board << rhs.board)
    }
}

impl Shr for BitBoard {
    type Output = BitBoard;

    fn shr(self, rhs: Self) -> Self::Output {
        BitBoard::from_shifted(self.board >> rhs.board)
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard::from_shifted(!self.board)
    }
}