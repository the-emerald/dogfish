use crate::piece::Piece;
use crate::board_representation::square::Square;

pub struct Move {
    source: Square,
    destination: Square,
    promotion: Option<Piece>
}

impl Move {
    pub fn new(source: Square, destination: Square, promotion: Option<Piece>) -> Self {
        Self {
            source,
            destination,
            promotion
        }
    }

    pub fn source(&self) -> Square {
        self.source
    }

    pub fn destination(&self) -> Square {
        self.destination
    }

    pub fn promotion(&self) -> Option<Piece> {
        self.promotion
    }
}