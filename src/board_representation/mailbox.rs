use crate::board::piecetype::PieceType;
use crate::board::piece::Piece;
use crate::board::colour::Colour;

pub struct Mailbox {
    pieces: [Option<Piece>; 64],
}

impl Mailbox {
    pub fn new() -> Self {
        Self {
            pieces: [None; 64],
        }
    }

    pub fn set_piece(&mut self, square: usize, piece: PieceType, colour: Colour) {
        self.pieces[square] = Some(Piece::from(piece, colour));
    }

    pub fn remove_piece(&mut self, square: usize) {
        self.pieces[square] = None;
    }

    pub fn get_piece(&self, square: usize) -> Option<Piece> {
        self.pieces[square]
    }
}