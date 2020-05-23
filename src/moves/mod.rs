use crate::board_representation::square::Square;
use crate::moves::types::MoveType;
use crate::piece::piecetype::PieceType;

pub mod types;

pub struct Move {
    source: Square,
    destination: Square,
    move_type: MoveType,
    promo_type: Option<PieceType>
}

impl Move {
    pub fn new(source: Square, destination: Square, move_type: MoveType, promo_type: Option<PieceType>) -> Self {
        Self {
            source,
            destination,
            move_type,
            promo_type
        }
    }
}