use crate::piece::Piece;
use crate::board_representation::square::Square;

pub struct Move {
    from: Square,
    to: Square,
    promotion: Option<Piece>
}

impl Move {
    pub fn new(from: Square, to: Square, promotion: Option<Piece>) -> Self {
        Self {
            from,
            to,
            promotion
        }
    }
}