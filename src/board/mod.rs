use crate::board_representation::bitboard::BitBoard;
use crate::board_representation::mailbox::Mailbox;
use crate::board_representation::square::Square;
use crate::piece::colour::Colour;
use crate::piece::Piece;
use crate::piece::colour::Colour::{Black, White};
use crate::piece::piecetype::PieceType::{P, N, R, Q, B, K};
use crate::piece::piecetype::PieceType;
use std::convert::{TryFrom, TryInto};
use crate::common::line::line_between;
use crate::common::moves::Move;
use crate::board_representation::bitboard::files_ranks::{RANK_1_BITBOARD, RANK_8_BITBOARD};
use crate::common::distance::SQUARE_DISTANCE;
use crate::board_representation::bitboard::shift::Direction::{North, South};
use std::hint::unreachable_unchecked;

pub mod fen;
pub mod castling;
pub mod search;

pub const PLAYERS_COUNT: usize = 2; // Number of players
pub const PIECES_TYPE_COUNT: usize = 6; // Number of types of pieces there are for each side

pub const SQUARES: usize = 64;
pub const FILES: usize = 8;
pub const RANKS: usize = 8;

#[derive(Copy, Clone)]
pub struct Board {
    player: Colour,

    bb_pieces: [BitBoard; PIECES_TYPE_COUNT],
    bb_player: [BitBoard; PLAYERS_COUNT],

    mailbox: Mailbox,

    castling_rights: [[bool; 2]; PLAYERS_COUNT],
    en_passant: BitBoard,

    half_moves: u8,     // 50-move rule counter
    full_moves: u8,     // Number of plies from root

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
        }
    }

    pub fn set_piece(&mut self, square: Square, piece: Piece) {
        // First check if there is already a piece on this square
        if let Some(p) = self.mailbox.get_piece(square) {
            let s: BitBoard = square.into();
            self.bb_player[p.colour() as usize] &= !s;
            self.bb_pieces[p.piece_type() as usize] &= !s;
            self.mailbox.remove_piece(square);
        }

        // Then set the piece
        self.bb_player[piece.colour() as usize] |= square.into();
        self.bb_pieces[piece.piece_type() as usize] |= square.into();
        self.mailbox.set_piece(square, piece);
    }

    pub fn remove_square(&mut self, square: Square) {
        // Only toggle bits if there is something on that square
        if let Some(p) = self.mailbox.get_piece(square) {
            let s: BitBoard = square.into();
            self.bb_player[p.colour() as usize] &= !s;
            self.bb_pieces[p.piece_type() as usize] &= !s;
            self.mailbox.remove_piece(square);
        }
    }

    pub fn move_square(&mut self, from: Square, to: Square) {
        self.set_piece(to, self.mailbox.get_piece(from).expect("no piece in from square in move"));
        self.remove_square(from);
    }

    pub fn attacks_to_king(&self, square: Square, king_colour: Colour) -> BitBoard {
        let sq: BitBoard = square.into();
        let occupancy = self.bb_player[Black as usize] | self.bb_player[White as usize];

        let opponent_pawns = self.bb_pieces[P as usize] & self.bb_player[king_colour.other() as usize];
        let opponent_knights = self.bb_pieces[N as usize] & self.bb_player[king_colour.other() as usize];
        let opponent_rooks = {
            self.bb_pieces[R as usize] & self.bb_player[king_colour.other() as usize] |
                self.bb_pieces[Q as usize] & self.bb_player[king_colour.other() as usize]
        };
        let opponent_bishops = {
            self.bb_pieces[B as usize] & self.bb_player[king_colour.other() as usize] |
                self.bb_pieces[Q as usize] & self.bb_player[king_colour.other() as usize]
        };

        (PieceType::pawn_attack(sq, king_colour) & opponent_pawns) |
            (PieceType::knight_attack(square) & opponent_knights) |
            (PieceType::rook_attack(square, occupancy) & opponent_rooks) |
            (PieceType::bishop_attack(square, occupancy) & opponent_bishops)
    }

    pub fn attacks_to(&self, square: Square) -> BitBoard {
        (PieceType::pawn_attack(square.into(), White) & self.bb_player[Black as usize]) |
            (PieceType::pawn_attack(square.into(), Black) & self.bb_player[White as usize]) |
            (PieceType::knight_attack(square) & self.bb_pieces[N as usize]) |
            (PieceType::king_attack(square) & self.bb_pieces[K as usize]) |
            (PieceType::bishop_attack(square, self.occupancy()) & self.bishop_queens()) |
            (PieceType::rook_attack(square, self.occupancy()) & self.rook_queens())
    }

    fn occupancy(&self) -> BitBoard {
        self.bb_player[Black as usize] | self.bb_player[White as usize]
    }

    fn bishop_queens(&self) -> BitBoard {
        self.bb_pieces[B as usize] | self.bb_pieces[Q as usize]
    }

    fn rook_queens(&self) -> BitBoard {
        self.bb_pieces[R as usize] | self.bb_pieces[Q as usize]
    }

    pub fn pinned_to(&self, square: Square) -> BitBoard {
        let mut pin = BitBoard::default();

        let opponent_rays = self.bb_player[self.player.other() as usize] &
            (self.rook_queens() | self.bishop_queens());

        for piece in opponent_rays
            .iter_bits()
            .enumerate()
            .filter(|p| p.1)
            .map(|q| Square::try_from(q.0 as u64).unwrap())
        {
            let line = line_between(piece, square) & self.occupancy();

            // If line not occupancy one or pieces not same colour
            if u64::from(line).count_ones() != 1 || (line & self.bb_player[self.player.other() as usize]) == line {
                continue
            }
            else {
                pin |= line;
            }
        }
        pin
    }

    pub fn do_move(&mut self, mov: Move) {
        #[cfg(debug_assertions)]
        {
            // Capturing the king is impossible
            if self.mailbox.get_piece(mov.destination())
                .map_or(false, |p| p.piece_type() == K) {
                panic!("do move captured king")
            }
            if self.mailbox.get_piece(mov.destination())
                .map_or(false, |p| p.colour() == self.player) {
                panic!("do move captured friendly piece")
            }
        }

        self.full_moves += 1;
        self.player = self.player.other();
        self.en_passant = BitBoard::default();

        match self.mailbox.get_piece(mov.source()).unwrap().piece_type() {
            P => {
                // Promote if rank is 1/8
                if BitBoard::from(mov.destination()) & (RANK_1_BITBOARD | RANK_8_BITBOARD) != 0.into() {
                    let promote_to = mov.promotion().expect("promotion without piece defined");
                    self.set_piece(mov.destination(), promote_to);
                }
                // Set EP if move distance is 2
                if SQUARE_DISTANCE[mov.source().value() as usize][mov.destination().value() as usize] == 2 {
                    // Use other player because players were already flipped
                    self.en_passant = match self.player.other() {
                        White => {
                            BitBoard::from(mov.source()).shift(North)
                        },
                        Black => {
                            BitBoard::from(mov.source()).shift(South)
                        },
                    }
                }
                self.move_square(mov.source(), mov.destination());
            },
            K => {
                // Check if castling
                if SQUARE_DISTANCE[mov.source().value() as usize][mov.destination().value() as usize] == 2 {
                    let (rook_from, rook_to) = match mov.destination().value() {
                        6 => { (7, 5) },    // white O-O
                        2 => { (0, 3) }     // white O-O-O
                        62 => { (63, 61) }  // black O-O
                        58 => { (56, 59) }  // black O-O-O
                        _ => unsafe { unreachable_unchecked() }
                    };
                    self.move_square(rook_from.try_into().unwrap(), rook_to.try_into().unwrap());
                }
                // Move the king itself
                self.move_square(mov.source(), mov.destination());
                // Update castling rights
                self.castling_rights[self.player.other() as usize] = [false; 2];
            },
            R => {
                // If the rook is the piece being moved then castling rights are removed
                self.castling_rights[self.player.other() as usize] = [false; 2];
                self.move_square(mov.source(), mov.destination());
            }
            _ => {
                self.move_square(mov.source(), mov.destination());
            },
        }

    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board_representation::bitboard::BitBoard;
    use crate::board_representation::square::Square;
    use std::convert::TryFrom;

    #[test]
    fn pinned_fen_basic() {
        let board: Board = "3k4/3b4/8/8/8/8/3Q4/3K4 b - - 0 1".parse().unwrap();
        let king = Square::try_from(59).unwrap();
        assert_eq!(board.pinned_to(king), BitBoard::from(0x8000000000000))
    }

    #[test]
    fn pinned_fen_2colour() {
        let board: Board = "3k4/3n4/3N4/8/8/8/3Q4/3K4 b - - 0 1".parse().unwrap();
        let king = Square::try_from(59).unwrap();
        assert_eq!(board.pinned_to(king), BitBoard::default())
    }

    #[test]
    fn pinned_fen_same_colour() {
        let board: Board = "3k4/8/3N4/8/8/8/3Q4/3K4 b - - 0 1".parse().unwrap();
        let king = Square::try_from(59).unwrap();
        assert_eq!(board.pinned_to(king), BitBoard::default())
    }

    #[test]
    fn pinned_fen_complex() {
        let board: Board = "3k2R1/2n1p3/1n1n4/B5B1/8/8/3Q4/3K4 b - - 0 1".parse().unwrap();
        let king = Square::try_from(59).unwrap();
        assert_eq!(board.pinned_to(king), BitBoard::from(0x10080000000000))
    }

    #[test]
    fn pinned_fen_complex_inverted() {
        let board: Board = "3K2r1/2N1P3/1N1N4/b5b1/8/8/3q4/3k4 w - - 0 1".parse().unwrap();
        let king = Square::try_from(59).unwrap();
        assert_eq!(board.pinned_to(king), BitBoard::from(0x10080000000000))
    }
}