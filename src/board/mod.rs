use std::sync::Arc;
use crate::common::colour::Colour;
use crate::board::bitboards::BitBoard;

pub mod piece;
pub mod bitboards;
pub mod fen;
pub mod castling;

pub const PLAYERS_COUNT: usize = 2; // Number of players
pub const PIECES_TYPE_COUNT: usize = 6; // Number of types of pieces there are for each side

pub const SQUARES: usize = 64;
pub const FILES: usize = 8;
pub const RANKS: usize = 8;

pub struct Board {
    player: Colour,

    bb_pieces: [BitBoard; PIECES_TYPE_COUNT],
    bb_player: [BitBoard; PLAYERS_COUNT],

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

            castling_rights: [[false; 2]; PLAYERS_COUNT],
            en_passant: BitBoard::zero(),

            half_moves: 0,
            full_moves: 0,
            previous: None
        }
    }
}