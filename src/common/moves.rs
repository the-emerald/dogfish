use crate::board_representation::square::Square;
use crate::piece::piecetype::PieceType;

pub struct Move {
    source: Square,
    destination: Square,
    promotion: Option<PieceType>
}

impl Move {
    pub fn new(source: Square, destination: Square, promotion: Option<PieceType>) -> Self {
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

    pub fn promotion(&self) -> Option<PieceType> {
        self.promotion
    }
}