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

            for char in rank_str.chars() {
                let val: u64 = (7 - (idx as u64)) * 8 + file;
                match char {
                    'P' => {
                        board.set_piece(val, PieceType::P, Colour::White);
                        file += 1;
                    },
                    'p' => {
                        board.set_piece(val, PieceType::P, Colour::Black);
                        file += 1;
                    },

                    'N' => {
                        board.set_piece(val, PieceType::N, Colour::White);
                        file += 1;
                    },
                    'n' => {
                        board.set_piece(val, PieceType::N, Colour::Black);
                        file += 1;
                    },

                    'B' => {
                        board.set_piece(val, PieceType::B, Colour::White);
                        file += 1;
                    },
                    'b' => {
                        board.set_piece(val, PieceType::B, Colour::Black);
                        file += 1;
                    },

                    'R' => {
                        board.set_piece(val, PieceType::R, Colour::White);
                        file += 1;
                    },
                    'r' => {
                        board.set_piece(val, PieceType::R, Colour::Black);
                        file += 1;
                    },

                    'Q' => {
                        board.set_piece(val, PieceType::Q, Colour::White);
                        file += 1;
                    },
                    'q' => {
                        board.set_piece(val, PieceType::Q, Colour::Black);
                        file += 1;
                    },

                    'K' => {
                        board.set_piece(val, PieceType::K, Colour::White);
                        file += 1;
                    },
                    'k' => {
                        board.set_piece(val, PieceType::K, Colour::Black);
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
                match (v[3].chars().nth(1), v[3].chars().nth(0)) {
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

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::common::colour::Colour;

    #[test]
    #[should_panic(expected = "FEN")]
    fn fen_parse_non_fen() {
        let fen_str = "cat dog meow woof";
        let board: Board = fen_str.parse().unwrap();
    }

    #[test]
    fn fen_parse_starting() {
        let fen_str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let board: Board = fen_str.parse().unwrap();

        assert_eq!(board.bb_player[Colour::White as usize].to_string(), "0000000000000000000000000000000000000000000000001111111111111111");
        assert_eq!(board.bb_player[Colour::Black as usize].to_string(), "1111111111111111000000000000000000000000000000000000000000000000");
    }

    #[test]
    fn fen_parse_valid() {
        let fen_str = "r3k1r1/pp3ppp/2pp1nb1/q2Pp3/P3P3/2N5/1PP2PPP/R3K2R w KQq e6 0 1";
        let board: Board = fen_str.parse().unwrap();

        assert_eq!(board.bb_player[Colour::White as usize].to_string(), "0000000000000000000000000000100000010001000001001110011010010001");
        assert_eq!(board.bb_player[Colour::Black as usize].to_string(), "0101000111100011011011000001000100000000000000000000000000000000");
    }

    #[test]
    #[should_panic(expected = "FEN")]
    fn fen_parse_invalid_castling_black_kingside() {
        let fen_str = "r3k1r1/pp3ppp/2pp1nb1/q2Pp3/P3P3/2N5/1PP2PPP/R3K2R w KQkq e6 0 1";
        let board: Board = fen_str.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "FEN")]
    fn fen_parse_invalid_spacing() {
        // Notice the 2pp2nb1
        let fen_str = "r3k1r1/pp3ppp/2pp2nb1/q2Pp3/P3P3/2N5/1PP2PPP/R3K2R w KQq e6 0 1";
        let board: Board = fen_str.parse().unwrap();
    }
}