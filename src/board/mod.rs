use std::sync::Arc;
use crate::board::piecetype::PieceType;
use anyhow::anyhow;
use crate::board_representation::bitboard::BitBoard;
use crate::board_representation::mailbox::Mailbox;
use crate::board::colour::Colour;
use crate::board_representation::square::Square;

pub mod piecetype;
pub mod fen;
pub mod castling;
pub mod piece;
pub mod colour;

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
            bb_pieces: [0.into(); PIECES_TYPE_COUNT],
            bb_player: [0.into(); PLAYERS_COUNT],
            mailbox: Mailbox::new(),
            castling_rights: [[false; 2]; PLAYERS_COUNT],
            en_passant: 0.into(),
            half_moves: 0,
            full_moves: 0,
            previous: None
        }
    }

    pub fn set_piece(&mut self, square: Square, piece: PieceType, colour: Colour) {
        // Set bitboards
        self.bb_pieces[piece as usize] |= square.into();
        self.bb_player[colour as usize] |= square.into();

        // Set mailbox
        self.mailbox.set_piece(square, piece, colour);
    }

    pub fn remove_piece(&mut self, square: u64) {
        unimplemented!() // TODO: Implement remove piece
    }

    pub fn move_piece(from: u64, to: u64) {
        unimplemented!() // TODO: Implement move piece
    }
}