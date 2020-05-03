use std::str::FromStr;
use crate::board::{Board, PIECES_TYPE_COUNT, PLAYERS_COUNT};
use crate::board::bitboards::BitBoard;
use crate::board::piece::PieceType;
use anyhow::anyhow;
use crate::common::colour::Colour;
use crate::board::castling::CastlingRights;

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut board = Board::new();

        // Check if there are enough parts for the FEN
        let v: Vec<&str> = s.trim().split(' ').collect();
        if v.len() != 6 {
            return Err(anyhow!("invalid FEN size"));
        }

        // Check if there are 8 ranks to use
        let pieces: Vec<&str> = v[0].trim().split('/').collect();
        if pieces.len() != 8 {
            return Err(anyhow!("invalid number of ranks in FEN"));
        }

        // Used to populate bitboard before the other information about the BitBoard can be filled

        for (idx, rank_str) in pieces.iter().enumerate() {
            let mut file: u64 = 0;
            let val: u64 = (7 - (idx as u64)) * 8 + file;

            for char in rank_str.chars() {
                match char {
                    'P' => {
                        board.bb_pieces[PieceType::P as usize] |= BitBoard::from_unshifted(val);
                        board.bb_player[Colour::White as usize] |= BitBoard::from_unshifted(val);
                        file += 1;
                    },
                    'p' => {
                        board.bb_pieces[PieceType::P as usize] |= BitBoard::from_unshifted(val);
                        board.bb_player[Colour::Black as usize] |= BitBoard::from_unshifted(val);
                        file += 1;
                    },

                    'N' => {
                        board.bb_pieces[PieceType::N as usize] |= BitBoard::from_unshifted(val);
                        board.bb_player[Colour::White as usize] |= BitBoard::from_unshifted(val);
                        file += 1;
                    },
                    'n' => {
                        board.bb_pieces[PieceType::N as usize] |= BitBoard::from_unshifted(val);
                        board.bb_player[Colour::Black as usize] |= BitBoard::from_unshifted(val);
                        file += 1;
                    },

                    'B' => {
                        board.bb_pieces[PieceType::B as usize] |= BitBoard::from_unshifted(val);
                        board.bb_player[Colour::White as usize] |= BitBoard::from_unshifted(val);
                        file += 1;
                    },
                    'b' => {
                        board.bb_pieces[PieceType::B as usize] |= BitBoard::from_unshifted(val);
                        board.bb_player[Colour::Black as usize] |= BitBoard::from_unshifted(val);
                        file += 1;
                    },

                    'R' => {
                        board.bb_pieces[PieceType::R as usize] |= BitBoard::from_unshifted(val);
                        board.bb_player[Colour::White as usize] |= BitBoard::from_unshifted(val);
                        file += 1;
                    },
                    'r' => {
                        board.bb_pieces[PieceType::R as usize] |= BitBoard::from_unshifted(val);
                        board.bb_player[Colour::Black as usize] |= BitBoard::from_unshifted(val);
                        file += 1;
                    },

                    'Q' => {
                        board.bb_pieces[PieceType::Q as usize] |= BitBoard::from_unshifted(val);
                        board.bb_player[Colour::White as usize] |= BitBoard::from_unshifted(val);
                        file += 1;
                    },
                    'q' => {
                        board.bb_pieces[PieceType::Q as usize] |= BitBoard::from_unshifted(val);
                        board.bb_player[Colour::Black as usize] |= BitBoard::from_unshifted(val);
                        file += 1;
                    },

                    'K' => {
                        board.bb_pieces[PieceType::K as usize] |= BitBoard::from_unshifted(val);
                        board.bb_player[Colour::White as usize] |= BitBoard::from_unshifted(val);
                        file += 1;
                    },
                    'k' => {
                        board.bb_pieces[PieceType::K as usize] |= BitBoard::from_unshifted(val);
                        board.bb_player[Colour::Black as usize] |= BitBoard::from_unshifted(val);
                        file += 1;
                    },

                    '1' => { file += 1 },
                    '2' => { file += 2 },
                    '3' => { file += 3 },
                    '4' => { file += 4 },
                    '5' => { file += 5 },
                    '6' => { file += 6 },
                    '7' => { file += 7 },
                    '8' => { file += 8 },

                    _ => {
                        return Err(anyhow!("invalid value in FEN"));
                    }
                }
            }
        }

        // Turn
        board.player = match v[1] {
            "w" => {
                Colour::White
            },
            "b" => {
                Colour::Black
            }
            _ => {
                return Err(anyhow!("invalid player to move in FEN"));
            }
        };

        // Castling rights
        for char in v[2].chars() {
            match char {
                'K' => {
                    board.castling_rights[Colour::White as usize][CastlingRights::KingSide as usize];
                },
                'Q' => {
                    board.castling_rights[Colour::White as usize][CastlingRights::QueenSide as usize];
                },
                'k' => {
                    board.castling_rights[Colour::Black as usize][CastlingRights::KingSide as usize];
                },
                'q' => {
                    board.castling_rights[Colour::Black as usize][CastlingRights::QueenSide as usize];
                }
                _ => {
                    return Err(anyhow!("invalid symbol for castling rights in FEN"))
                }
            }
        }

        // En passant square
        board.en_passant = {
            if v[3] != "-" {
                match (v[3].chars().nth(0), v[3].chars().nth(1)) {
                    (Some(r), Some(f)) => {
                        BitBoard::from_unshifted(BitBoard::rank_file_to_square(r, f)?)
                    },
                    _ => {
                        return Err(anyhow!("invalid en passant position in FEN"))
                    }
                }
            }
            else {
                BitBoard::zero()
            }
        };

        // Half moves and full moves
        board.half_moves = v[4].parse()?;
        board.full_moves = v[5].parse()?;

        Ok(board)
    }
}

impl ToString for Board {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}