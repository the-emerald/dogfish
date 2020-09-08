use crate::board_representation::square::Square;
use crate::piece::Piece;

#[derive(Copy, Clone)]
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

impl Default for Mailbox {
    fn default() -> Self {
        Self::new()
    }
}
