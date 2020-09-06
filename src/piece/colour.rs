use crate::piece::Piece;
use crate::piece::colour::Colour::{Black, White};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Colour {
    White = 0,
    Black = 1,
}

impl Colour {
    pub fn other(self) -> Self {
        match self {
            White => { Black }
            Black => { White }
        }
    }
}

impl From<Piece> for Colour {
    fn from(value: Piece) -> Self {
        value.colour()
    }
}