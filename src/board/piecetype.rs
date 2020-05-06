use crate::board::piece::Piece;

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum PieceType {
    P = 0,
    N = 1,
    B = 2,
    R = 3,
    Q = 4,
    K = 5
}

impl From<Piece> for PieceType {
    fn from(value: Piece) -> Self {
        value.piece_type()
    }
}