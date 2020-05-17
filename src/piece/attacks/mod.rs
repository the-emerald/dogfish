use crate::board_representation::bitboard::BitBoard;
use crate::piece::piecetype::PieceType;
use crate::piece::colour::Colour;
use crate::piece::piecetype::PieceType::{P, N, K, Q, R, B};

pub mod knight;
pub mod king;

#[cfg(debug_assertions)]
fn piecetype_ne_panic(lhs: PieceType, rhs: PieceType) {
    if lhs != rhs {
        panic!("attempted to use {:?} attacks with {:?}", lhs, rhs);
    }
}

// impl Piece {
//     pub fn attack(self, bitboard: BitBoard) -> BitBoard {
//         match self.piece_type {
//             PieceType::P => {}, // WTF
//             PieceType::N => {}, // Lookup
//             PieceType::B => {}, // Sliding
//             PieceType::R => {}, // Sliding
//             PieceType::Q => {}, // Sliding
//             PieceType::K => {}, // Lookup
//         }
//         unimplemented!()
//     }
// }

impl PieceType {
    pub fn pawn_attack(self, colour: Colour) -> BitBoard {
        #[cfg(debug_assertions)]
            piecetype_ne_panic(P, self);
        unimplemented!()
    }

    pub fn knight_attack(self) -> BitBoard {
        #[cfg(debug_assertions)]
            piecetype_ne_panic(N, self);
        unimplemented!()
    }

    pub fn bishop_attack(self, colour: Colour, occupancy: BitBoard) -> BitBoard {
        #[cfg(debug_assertions)]
            piecetype_ne_panic(B, self);
        unimplemented!()
    }

    pub fn rook_attack(self, colour: Colour, occupancy: BitBoard) -> BitBoard {
        #[cfg(debug_assertions)]
            piecetype_ne_panic(R, self);
        unimplemented!()
    }

    pub fn queen_attack(self, colour: Colour, occupancy: BitBoard) -> BitBoard {
        #[cfg(debug_assertions)]
            piecetype_ne_panic(Q, self);
        unimplemented!()
    }

    pub fn king_attack(self, colour: Colour) -> BitBoard {
        #[cfg(debug_assertions)]
            piecetype_ne_panic(K, self);
        unimplemented!()
    }
}