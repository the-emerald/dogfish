use crate::board_representation::bitboard::BitBoard;
use crate::piece::piecetype::PieceType;
use crate::piece::colour::Colour;
use crate::piece::piecetype::PieceType::{P, N, K, Q, R, B};
use crate::board_representation::bitboard::shift::Direction::North;
use crate::board_representation::bitboard::shift::Direction;
use crate::board_representation::square::Square;

pub mod knight;
pub mod king;

impl PieceType {
    pub fn pawn_attack(pawns: BitBoard, colour: Colour) -> BitBoard {
        unimplemented!()
    }

    pub fn knight_attack(square: Square) -> BitBoard {
        unimplemented!()
    }

    pub fn bishop_attack(square: Square, occupancy: BitBoard) -> BitBoard {
        unimplemented!()
    }

    pub fn rook_attack(square: Square, occupancy: BitBoard) -> BitBoard {
        unimplemented!()
    }

    pub fn queen_attack(square: Square, occupancy: BitBoard) -> BitBoard {
        unimplemented!()
    }

    pub fn king_attack(square: Square) -> BitBoard {
        unimplemented!()
    }
}