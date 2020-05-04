use std::str::FromStr;
use crate::board::{Board, PIECES_TYPE_COUNT, PLAYERS_COUNT};
use crate::board::piecetype::PieceType;
use anyhow::anyhow;
use crate::board::castling::CastlingRights;
use crate::board_representation::bitboard::BitBoard;
use crate::board::colour::Colour;

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

        // Pieces
        for (idx, rank_str) in pieces.iter().enumerate() {
            let mut file: u64 = 0;

            for char in rank_str.chars() {
                if file >= 8 {
                    return Err(anyhow!("invalid piece positioning in FEN"))
                }
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
                    board.castling_rights[Colour::White as usize][CastlingRights::KingSide as usize] = true;
                },
                'Q' => {
                    board.castling_rights[Colour::White as usize][CastlingRights::QueenSide as usize] = true;
                },
                'k' => {
                    board.castling_rights[Colour::Black as usize][CastlingRights::KingSide as usize] = true;
                },
                'q' => {
                    board.castling_rights[Colour::Black as usize][CastlingRights::QueenSide as usize] = true;
                }
                _ => {
                    return Err(anyhow!("invalid symbol for castling rights in FEN"))
                }
            }
        }

        // Validate said castling rights
        let white_castling_squares = [0_usize, 7];
        for wc in board.castling_rights[Colour::White as usize].iter().zip(white_castling_squares.iter()) {
            if *wc.0 {
                match board.mailbox.get_piece(*wc.1) {
                    None => {
                        return Err(anyhow!("invalid castling rights detected in FEN"))
                    }
                    Some(p) if (p.piece_type().unwrap() != PieceType::R || p.colour().unwrap() != Colour::White)=> {
                        return Err(anyhow!("invalid castling rights detected in FEN"))
                    }
                    _ => {}
                }
            }
        }

        // Black
        let black_castling_squares = [56_usize, 63];
        for bc in board.castling_rights[Colour::Black as usize].iter().zip(black_castling_squares.iter()) {
            if *bc.0 {
                match board.mailbox.get_piece(*bc.1) {
                    None => {
                        return Err(anyhow!("invalid castling rights detected in FEN"))
                    }
                    Some(p)
                    if (p.piece_type().unwrap() != PieceType::R ||
                        p.colour().unwrap() != Colour::Black) => {

                        return Err(anyhow!("invalid castling rights detected in FEN"))
                    }
                    _ => {}
                }
            }
        }

        // En passant square
        board.en_passant = {
            if v[3] != "-" {
                match (v[3].chars().nth(1), v[3].chars().nth(0)) {
                    (Some(r), Some(f)) => {
                        BitBoard::from_square(Board::rank_file_to_square(r, f)?)
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
        unimplemented!() // TODO: Write a Board to FEN converter
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::colour::Colour;

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

        assert_eq!(board.bb_player[Colour::White as usize].to_string(), "0x000000000000ffff");
        assert_eq!(board.bb_player[Colour::Black as usize].to_string(), "0xffff000000000000");
    }

    #[test]
    fn fen_parse_valid() {
        let fen_str = "r3k1r1/pp3ppp/2pp1nb1/q2Pp3/P3P3/2N5/1PP2PPP/R3K2R w KQq e6 0 1";
        let board: Board = fen_str.parse().unwrap();

        assert_eq!(board.bb_player[Colour::White as usize].to_string(), "0x000000081104e691");
        assert_eq!(board.bb_player[Colour::Black as usize].to_string(), "0x51e36c1100000000");
    }

    #[test]
    #[should_panic(expected = "castling")]
    fn fen_parse_invalid_castling_black_kingside() {
        let fen_str = "r3k1r1/pp3ppp/2pp1nb1/q2Pp3/P3P3/2N5/1PP2PPP/R3K2R w KQkq e6 0 1";
        let board: Board = fen_str.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "piece")]
    fn fen_parse_invalid_spacing() {
        // Notice the 2pp2nb1
        let fen_str = "r3k1r1/pp3ppp/2pp2nb1/q2Pp3/P3P3/2N5/1PP2PPP/R3K2R w KQq e6 0 1";
        let board: Board = fen_str.parse().unwrap();
    }
}