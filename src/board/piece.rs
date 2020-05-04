use crate::board::piecetype::PieceType;
use crate::common::colour::Colour;
use num::FromPrimitive;

#[derive(Copy, Clone)]
pub struct Piece(u8);

impl Piece {
    pub fn from(piece_type: PieceType, colour: Colour) -> Self {
        Self(colour as u8 | ((piece_type as u8) << 1))
    }

    pub fn colour(&self) -> Option<Colour> {
        num::FromPrimitive::from_u8(self.0 & 1)
    }

    pub fn piece_type(&self) -> Option<PieceType> {
        num::FromPrimitive::from_u8(self.0 >> 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::piece::Piece;
    use crate::common::colour::Colour;
    use crate::board::piecetype;
    use crate::board::piecetype::PieceType;

    #[test]
    fn piece_white_pawn() {
        let piece = Piece::from(PieceType::P, Colour::White);
        assert_eq!(piece.colour().unwrap(), Colour::White);
        assert_eq!(piece.piece_type().unwrap(), PieceType::P);

    }

    #[test]
    fn piece_white_knight() {
        let piece = Piece::from(PieceType::N, Colour::White);
        assert_eq!(piece.colour().unwrap(), Colour::White);
        assert_eq!(piece.piece_type().unwrap(), PieceType::N);
    }

    #[test]
    fn piece_white_bishop() {
        let piece = Piece::from(PieceType::B, Colour::White);
        assert_eq!(piece.colour().unwrap(), Colour::White);
        assert_eq!(piece.piece_type().unwrap(), PieceType::B);
    }

    #[test]
    fn piece_white_rook() {
        let piece = Piece::from(PieceType::R, Colour::White);
        assert_eq!(piece.colour().unwrap(), Colour::White);
        assert_eq!(piece.piece_type().unwrap(), PieceType::R);
    }

    #[test]
    fn piece_white_queen() {
        let piece = Piece::from(PieceType::Q, Colour::White);
        assert_eq!(piece.colour().unwrap(), Colour::White);
        assert_eq!(piece.piece_type().unwrap(), PieceType::Q);
    }

    #[test]
    fn piece_white_king() {
        let piece = Piece::from(PieceType::K, Colour::White);
        assert_eq!(piece.colour().unwrap(), Colour::White);
        assert_eq!(piece.piece_type().unwrap(), PieceType::K);
    }

    #[test]
    fn piece_black_pawn() {
        let piece = Piece::from(PieceType::P, Colour::Black);
        assert_eq!(piece.colour().unwrap(), Colour::Black);
        assert_eq!(piece.piece_type().unwrap(), PieceType::P);
    }

    #[test]
    fn piece_black_knight() {
        let piece = Piece::from(PieceType::N, Colour::Black);
        assert_eq!(piece.colour().unwrap(), Colour::Black);
        assert_eq!(piece.piece_type().unwrap(), PieceType::N);
    }

    #[test]
    fn piece_black_bishop() {
        let piece = Piece::from(PieceType::B, Colour::Black);
        assert_eq!(piece.colour().unwrap(), Colour::Black);
        assert_eq!(piece.piece_type().unwrap(), PieceType::B);
    }

    #[test]
    fn piece_black_rook() {
        let piece = Piece::from(PieceType::R, Colour::Black);
        assert_eq!(piece.colour().unwrap(), Colour::Black);
        assert_eq!(piece.piece_type().unwrap(), PieceType::R);
    }

    #[test]
    fn piece_black_queen() {
        let piece = Piece::from(PieceType::Q, Colour::Black);
        assert_eq!(piece.colour().unwrap(), Colour::Black);
        assert_eq!(piece.piece_type().unwrap(), PieceType::Q);
    }

    #[test]
    fn piece_black_king() {
        let piece = Piece::from(PieceType::K, Colour::Black);
        assert_eq!(piece.colour().unwrap(), Colour::Black);
        assert_eq!(piece.piece_type().unwrap(), PieceType::K);
    }
}
