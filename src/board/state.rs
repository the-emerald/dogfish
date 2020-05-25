use crate::board::{Board, PLAYERS_COUNT};
use std::sync::Arc;
use crate::board_representation::bitboard::BitBoard;
use crate::moves::Move;
use crate::board_representation::square::Square;
use crate::piece::piecetype::PieceType;
use std::convert::TryInto;

pub struct State {
    // Copy-and-update on move
    // These must be set up during FEN constructions or otherwise.
    pub rule_50: u8,
    pub ply: u8,
    pub castling: [[bool; 2]; PLAYERS_COUNT],
    pub en_passant: Option<BitBoard>,

    // Recalculate on move
    // These must be set up bu applying a board.
    pub bb_checkers: BitBoard, // Pieces giving check
    pub bb_pinned: BitBoard, // Pieces that are pinned
    pub bb_pinners: BitBoard, // Pieces that pin other pieces


    pub previous_move: Option<Move>,
    pub previous_state: Option<Arc<State>>
}

impl State {
    // Populate fields using a board.
    pub fn apply_board(&mut self, board: &Board) {
        let king = board.bb_pieces[PieceType::K as usize] & board.bb_player[board.player as usize];

        self.bb_checkers = board.attacks_to_king(king.try_into().unwrap(), board.player);
        unimplemented!()
    }

    // Create a copy with copied fields for copy-and-update fields
    // and empty for recalculate fields
    pub fn next(&self) -> Self {
        unimplemented!()
    }
}

// Used to create a placeholder struct
impl Default for State {
    fn default() -> Self {
        Self {
            rule_50: 0,
            ply: 0,
            castling: [[false; 2]; PLAYERS_COUNT],
            en_passant: None,
            bb_checkers: 0.into(),
            bb_pinned: 0.into(),
            bb_pinners: 0.into(),
            previous_move: None,
            previous_state: None
        }
    }
}