use crate::board_representation::bitboard::BitBoard;
use crate::piece::piecetype::PieceType;
use crate::piece::colour::Colour;
use crate::piece::piecetype::PieceType::{P, N, K, Q, R, B};
use crate::board_representation::bitboard::shift::Direction::{North, NorthEast, NorthWest, SouthEast, SouthWest};
use crate::board_representation::bitboard::shift::Direction;
use crate::board_representation::square::Square;
use crate::piece::attacks::knight::ATTACK_TABLE_KNIGHT;
use crate::piece::attacks::king::ATTACK_TABLE_KING;

pub mod knight;
pub mod king;

impl PieceType {
    pub fn pawn_attack(pawns: BitBoard, colour: Colour) -> BitBoard {
        let attack_pattern: [Direction; 2] = match colour {
            Colour::White => {[NorthEast, NorthWest]},
            Colour::Black => {[SouthEast, SouthWest]},
        };

        BitBoard::shift(attack_pattern[0], pawns) |
        BitBoard::shift(attack_pattern[1], pawns)
    }

    pub fn knight_attack(square: Square) -> BitBoard {
        ATTACK_TABLE_KNIGHT[u64::from(square) as usize]
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
        ATTACK_TABLE_KING[u64::from(square) as usize]
    }
}