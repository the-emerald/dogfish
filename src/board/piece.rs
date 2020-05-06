use crate::board::piecetype::PieceType;
use crate::board::colour::Colour;

#[derive(Copy, Clone)]
pub struct Piece {
    colour: Colour,
    piece_type: PieceType
}

impl Piece {
    pub fn new(colour: Colour, piece_type: PieceType) -> Self {
        Self {
            colour,
            piece_type
        }
    }

    pub fn colour(self) -> Colour {
        self.colour
    }

    pub fn piece_type(self) -> PieceType {
        self.piece_type
    }
}

impl From<(Colour, PieceType)> for Piece {
    fn from(value: (Colour, PieceType)) -> Self {
        Piece::new(value.0, value.1)
    }
}
