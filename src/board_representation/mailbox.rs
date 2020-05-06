use crate::board::piecetype::PieceType;
use crate::board::piece::Piece;
use crate::board::colour::Colour;
use crate::board_representation::square::Square;

pub struct Mailbox {
    pieces: [Option<Piece>; 64],
}

impl Mailbox {
    pub fn new() -> Self {
        Self {
            pieces: [None; 64],
        }
    }

    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        self.pieces[square.value() as usize] = Some(piece);
    }

    pub fn remove_piece(&mut self, square: Square) {
        self.pieces[square.value() as usize] = None;
    }

    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        self.pieces[square.value() as usize]
    }
}