use crate::board::piecetype::PieceType;
use crate::common::colour::Colour;

pub struct Piece(u8); // TODO: Define this

pub struct Mailbox {
    pieces: [Option<PieceType>; 64],
}

impl Mailbox {
    pub fn new() -> Self {
        Self {
            pieces: [None; 64],
        }
    }

    pub fn set_piece(&mut self, square: usize, piece: PieceType, colour: Colour) {
        unimplemented!()
    }

    pub fn remove_piece(&mut self, square: usize) {
        unimplemented!()
    }
}

