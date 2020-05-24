use crate::piece::Piece;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Colour {
    White = 0,
    Black = 1,
}

impl From<Piece> for Colour {
    fn from(value: Piece) -> Self {
        value.colour()
    }
}

impl Colour {
    pub fn other(self) -> Self {
        match self {
            Colour::White => {Colour::Black},
            Colour::Black => {Colour::White},
        }
    }
}