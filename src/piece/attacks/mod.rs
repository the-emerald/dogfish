use crate::board_representation::bitboard::BitBoard;
use crate::piece::piecetype::PieceType;
use crate::piece::colour::Colour;
use crate::piece::piecetype::PieceType::{P, N, K, Q, R, B};
use crate::board_representation::bitboard::shift::Direction::{North, NorthEast, NorthWest, SouthEast, SouthWest};
use crate::board_representation::bitboard::shift::Direction;
use crate::board_representation::square::Square;
use crate::piece::attacks::knight::ATTACK_TABLE_KNIGHT;
use crate::piece::attacks::king::ATTACK_TABLE_KING;
use crate::board_representation::square::ParseError::BitBoardNotUnit;
use crate::piece::attacks::magic::SLIDING_ROOK;

pub mod knight;
pub mod king;
pub mod magic;

impl PieceType {
    pub fn pawn_attack(pawns: BitBoard, colour: Colour) -> BitBoard {
        let attack_pattern: [Direction; 2] = match colour {
            Colour::White => {[NorthEast, NorthWest]},
            Colour::Black => {[SouthEast, SouthWest]},
        };

        pawns.shift(attack_pattern[0]) |
        pawns.shift(attack_pattern[1])
    }

    pub fn knight_attack(square: Square) -> BitBoard {
        ATTACK_TABLE_KNIGHT[u64::from(square) as usize]
    }

    pub fn bishop_attack(square: Square, occupancy: BitBoard) -> BitBoard {
        unimplemented!()
    }

    pub fn rook_attack(square: Square, occupancy: BitBoard) -> BitBoard {
        let magic = SLIDING_ROOK.magic[square.value() as usize];
        let idx: usize = u64::from(
            (occupancy & magic.mask()) * (magic.magic().into()) >> (magic.shift().into())
        ) as usize;
        return SLIDING_ROOK.table[magic.table() + idx];
    }

    pub fn queen_attack(square: Square, occupancy: BitBoard) -> BitBoard {
        unimplemented!()
    }

    pub fn king_attack(square: Square) -> BitBoard {
        ATTACK_TABLE_KING[u64::from(square) as usize]
    }

    pub fn sliding_attack(attack_directions: [Direction; 4], square: Square, occupancy: BitBoard) -> BitBoard {
        let mut attacks: BitBoard = 0.into();

        for direction in attack_directions.iter() {
            let mut next = BitBoard::from(square).shift(*direction);
            // First: Bitboard of square is not empty
            // Second: No piece occupies that square
            while BitBoard::from(next) != BitBoard::new(0) && (occupancy & next.into()) == 0.into() {
                // println!("{:?}", next);
                // println!("{:?}", occupancy);
                // println!("-----");
                attacks |= next;
                next = next.shift(*direction);
            }
        }
        attacks
    }
}

#[cfg(test)]
mod tests {
    use crate::board_representation::bitboard::BitBoard;
    use crate::board_representation::square::Square;
    use std::convert::TryFrom;
    use crate::piece::piecetype::PieceType;
    use crate::board_representation::bitboard::shift::Direction::{North, East, South, West};

    #[test]
    fn rook() {
        let occupancy = BitBoard::new(0);
        let a1 = Square::try_from(0).unwrap();

        let attacks = PieceType::rook_attack(a1, occupancy);
        println!("{:?}", attacks);
    }

    #[test]
    fn sliding_attacks() {
        let occupancy = BitBoard::new(0x100000000);
        let a1 = Square::try_from(0).unwrap();
        let directions = [North, East, South, West];

        let sliding = PieceType::sliding_attack(directions, a1, occupancy);
        println!("{:?}", sliding);
    }
}