use std::sync::Arc;
use crate::common::colour::Colour;
use crate::board::bitboards::BitBoard;
use crate::board::piecetype::PieceType;
use crate::board::mailbox::Mailbox;

pub mod piecetype;
pub mod bitboards;
pub mod fen;
pub mod castling;
pub mod mailbox;
pub mod piece;

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

    castling_rights: [[bool; 2]; PLAYERS_COUNT],
    en_passant: BitBoard,

    half_moves: u8,
    full_moves: u8,

    previous: Option<Arc<Board>>
}

impl Board {
    pub fn new() -> Self {
        Self {
            player: Colour::White,
            bb_pieces: [BitBoard::zero(); PIECES_TYPE_COUNT],
            bb_player: [BitBoard::zero(); PLAYERS_COUNT],

            mailbox: Mailbox::new(),

            castling_rights: [[false; 2]; PLAYERS_COUNT],
            en_passant: BitBoard::zero(),

            half_moves: 0,
            full_moves: 0,
            previous: None
        }
    }

    pub fn set_piece(&mut self, square: u64, piece: PieceType, colour: Colour) {
        #[cfg(debug_assertions)]
            if !Board::valid_square(&square) {
            println!("Debug");
            panic!("invalid square being set on board")
        }

        // Set bitboards
        self.bb_pieces[piece as usize] |= BitBoard::from_unshifted(square);
        self.bb_player[colour as usize] |= BitBoard::from_unshifted(square);

        // Set piece
        self.mailbox.set_piece(square as usize, piece, colour);
    }

    pub fn remove_piece(&mut self, square: u64) {
        unimplemented!()
    }
}

// Helper functions
impl Board {
    pub fn valid_square(sq: &u64) -> bool {
        (0..64).contains(sq)
    }
}