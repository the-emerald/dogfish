use std::sync::Arc;
use crate::board_representation::bitboard::BitBoard;
use crate::board_representation::mailbox::Mailbox;
use crate::board_representation::square::Square;
use crate::piece::colour::Colour;
use crate::piece::Piece;
use crate::board::state::State;

pub mod fen;
pub mod castling;
pub mod state;

pub const PLAYERS_COUNT: usize = 2; // Number of players
pub const PIECES_TYPE_COUNT: usize = 6; // Number of types of pieces there are for each side

pub const SQUARES: usize = 64;
pub const FILES: usize = 8;
pub const RANKS: usize = 8;

pub struct Board {
    player: Colour,

    bb_pieces: [BitBoard; PIECES_TYPE_COUNT],
    bb_player: [BitBoard; PLAYERS_COUNT],

    mailbox: Mailbox,

    state: Arc<State>
}

impl Board {
    pub fn new() -> Self {
        Self {
            player: Colour::White,
            bb_pieces: [0.into(); PIECES_TYPE_COUNT],
            bb_player: [0.into(); PLAYERS_COUNT],
            mailbox: Mailbox::new(),
            state: Arc::new(State::default())
        }
    }

    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        self.bb_player[piece.colour() as usize] |= square.into();
        self.bb_pieces[piece.piece_type() as usize] |= square.into();

        self.mailbox.set_piece(square, piece);
    }

    pub fn remove_square(&mut self, square: Square) {
        let piece = self.mailbox.get_piece(square);

        if let Some(p) = piece {
            let s: BitBoard = square.into();
            self.bb_player[p.colour() as usize] &= !s;
            self.bb_pieces[p.piece_type() as usize] &= !s;
            self.mailbox.remove_piece(square);
        }
    }

    pub fn move_square(&mut self, from: Square, to: Square) {
        self.remove_square(from);

        if let Some(fp) = self.mailbox.get_piece(from) {
            self.set_piece(to, fp);
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}