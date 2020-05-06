use crate::piece::Piece;
use crate::board_representation::bitboard::BitBoard;
use crate::piece::piecetype::PieceType;

impl Piece {
    pub fn attack(self, bitboard: BitBoard) -> BitBoard {
        match self.piece_type {
            PieceType::P => {},
            PieceType::N => {},
            PieceType::B => {},
            PieceType::R => {},
            PieceType::Q => {},
            PieceType::K => {},
        }
        unimplemented!()
    }
}