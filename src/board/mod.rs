use std::sync::Arc;
use crate::board::piecetype::PieceType;
use anyhow::anyhow;
use crate::board_representation::bitboard::BitBoard;
use crate::board_representation::mailbox::Mailbox;
use crate::board::colour::Colour;

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
        {
            if !Board::valid_square(&square) {
                panic!("invalid square being set on board")
            }
        }

        // Set bitboards
        self.bb_pieces[piece as usize] |= BitBoard::from_square(square);
        self.bb_player[colour as usize] |= BitBoard::from_square(square);

        // Set mailbox
        self.mailbox.set_piece(square as usize, piece, colour);
    }

    pub fn remove_piece(&mut self, square: u64) {
        unimplemented!() // TODO: Implement remove piece
    }

    pub fn move_piece(from: u64, to: u64) {
        unimplemented!() // TODO: Implement move piece
    }
}

// Helper functions
impl Board {
    pub fn valid_square(sq: &u64) -> bool {
        (0..64).contains(sq)
    }

    pub fn rank_file_to_square(rank: char, file: char) -> anyhow::Result<u64> {
        let mut square: u64 = 0;
        match file {
            'A' | 'a' => {}
            'B' | 'b' => {
                square += 1;
            }
            'C' | 'c' => {
                square += 2;
            }
            'D' | 'd' => {
                square += 3;
            }
            'E' | 'e' => {
                square += 4;
            }
            'F' | 'f' => {
                square += 5;
            }
            'G' | 'g' => {
                square += 6;
            }
            'H' | 'h' => {
                square += 7;
            }
            _ => {
                return Err(anyhow!("invalid file in rank/file: {}", file));
            }
        }
        match rank {
            '1' => {},
            '2' => {
                square += 1 * 8;
            },
            '3' => {
                square += 2 * 8;
            },
            '4' => {
                square += 3 * 8;
            },
            '5' => {
                square += 4 * 8;
            },
            '6' => {
                square += 5 * 8;
            },
            '7' => {
                square += 6 * 8;
            },
            '8' => {
                square += 7 * 8;
            },
            _ => {
                return Err(anyhow!("invalid rank in rank/file: {}", rank));
            }
        }
        Ok(square)
    }
}