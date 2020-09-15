use crate::board::Board;
use crate::board_representation::bitboard::BitBoard;
use crate::piece::piecetype::PieceType;
use crate::piece::piecetype::PieceType::K;
use std::convert::{TryInto, TryFrom};
use crate::common::moves::Move;
use crate::piece::colour::Colour;
use crate::board_representation::bitboard::shift::Direction::{North, South};
use crate::board_representation::bitboard::files_ranks::{RANK_1_BITBOARD, RANK_8_BITBOARD};
use crate::board_representation::square::Square;
use crate::common::line::LINE_INTERSECTING;

impl Board {
    pub fn search(&self) {
        unimplemented!();
    }

    pub fn generate_moves(&self) -> impl Iterator<Item = Move> {
        let own_pieces = self.bb_pieces[self.player as usize];
        let our_king = self.bb_pieces[K as usize] & self.bb_player[self.player as usize];
        let pinned = self.pinned_to(our_king.try_into().expect("more than one king of the same colour"));

        let mut moves = Vec::new();
        for piece in self.bb_player[self.player as usize].iter_squares() {
            match self.mailbox.get_piece(piece).expect("empty square in move generation").piece_type() {
                PieceType::P => {
                    // TODO: Finish pawn move generation
                    let attacks = PieceType::pawn_attack(piece.into(), self.player) & self.bb_player[self.player.other() as usize];
                    let quiet = BitBoard::from(
                        match self.player {
                            Colour::White => { piece.shift(North) },
                            Colour::Black => { piece.shift(South) },
                        }
                    ) & !self.occupancy(); // Quiet moves cannot go into occupied space

                    let mut mvs = (attacks | quiet) & !own_pieces;

                    // If pawn is pinned then do checks
                    let mut pawn_really_pinned = piece_is_pinned(piece, pinned);
                    if pawn_really_pinned {
                        for potential_unpin in mvs.iter_squares() {
                            // If the line that intersects the king and the pawn-from also intersects the potentially unpinning piece:
                            if LINE_INTERSECTING[Square::try_from(our_king).unwrap().value() as usize][piece.value() as usize] & potential_unpin.into() != 0.into() {
                                mvs = potential_unpin.into();
                                pawn_really_pinned = false;
                            }
                        }
                        if pawn_really_pinned {
                            continue; // Do not generate any moves for this piece
                        }
                    }

                    // TODO: What about en passant captures?

                    // Promotion is possible if on the final rank
                    let promotions = if mvs & (RANK_1_BITBOARD | RANK_8_BITBOARD) == mvs {
                        Some([PieceType::B, PieceType::N, PieceType::Q, PieceType::R])
                    }
                    else {
                        None
                    };

                    moves.append({
                        &mut promotions.map_or_else(
                            // If no promotion possible
                            || mvs.iter_squares().map(|s| Move::new(piece, s, None)).collect::<Vec<Move>>(),
                            // Promotion possible, enumerate all possibilities
                            |y| y.iter().map(|z| mvs.iter_squares().map(|s| Move::new(piece, s, Some(*z))).collect::<Vec<Move>>()).flatten().collect()
                        )
                    });
                },
                PieceType::N => {
                    // TODO: Finish knight move generation
                    PieceType::knight_attack(piece);
                },
                PieceType::B => {
                    // TODO: Finish bishop move generation
                    PieceType::bishop_attack(piece, self.occupancy());
                },
                PieceType::R => {
                    // TODO: Finish rook move generation
                    PieceType::rook_attack(piece, self.occupancy());
                },
                PieceType::Q => {
                    // TODO: Finish queen move generation
                    PieceType::queen_attack(piece, self.occupancy());
                },
                PieceType::K => {
                    // TODO: Finish king move generation
                    PieceType::king_attack(piece);
                },
            };
        }
        moves.into_iter()
    }

}

fn piece_is_pinned(piece: Square, pin: BitBoard) -> bool {
    pin & piece.into() != 0.into()  // TODO: Refactor x and y.into != 0.into as x.contains(y)
}