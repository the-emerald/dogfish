use std::str::FromStr;
use crate::board::{Board};
use crate::board::castling::CastlingRights;
use crate::board_representation::square::Square;
use std::convert::TryInto;
use crate::piece::colour::Colour;
use crate::piece::piecetype::PieceType;
use crate::board_representation;
use crate::board::fen::ParseError::{Size, Rank, PiecePosition, Unrecognised, Castling, EnPassant};
use std::num::ParseIntError;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("invalid square: {0}")]
    SquareParse(#[from] board_representation::square::ParseError),
    #[error("invalid number of fields: {0}, expected 6")]
    Size(usize),
    #[error("invalid number of ranks")]
    Rank,
    #[error("invalid number of files")]
    File,
    #[error("invalid piece position")]
    PiecePosition,
    #[error("invalid castling state")]
    Castling,
    #[error("invalid en passant")]
    EnPassant,
    #[error("unrecognised token found: {0}")]
    Unrecognised(String),
    #[error("expected number: {0}")]
    NotNumber(#[from] ParseIntError),

}

impl FromStr for Board {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, ParseError> {
        let mut board = Board::new();

        // Check if there are enough parts for the FEN
        let v: Vec<&str> = s.trim().split(' ').collect();
        if v.len() != 6 {
            return Err(Size(v.len()));
        }

        // Check if there are 8 ranks to use
        let pieces: Vec<&str> = v[0].trim().split('/').collect();
        if pieces.len() != 8 {
            return Err(Rank);
        }

        // Pieces
        for (idx, rank_str) in pieces.iter().enumerate() {
            let mut file: u64 = 0;

            for char in rank_str.chars() {
                if file >= 8 {
                    return Err(PiecePosition)
                }
                let val: Square = ((7 - (idx as u64)) * 8 + file).try_into()?;
                match char {
                    'P' => {
                        board.set_piece(val, (Colour::White, PieceType::P).into());
                        file += 1;
                    },
                    'p' => {
                        board.set_piece(val, (Colour::Black, PieceType::P).into());
                        file += 1;
                    },

                    'N' => {
                        board.set_piece(val, (Colour::White, PieceType::N).into());
                        file += 1;
                    },
                    'n' => {
                        board.set_piece(val, (Colour::Black, PieceType::N).into());
                        file += 1;
                    },

                    'B' => {
                        board.set_piece(val, (Colour::White, PieceType::B).into());
                        file += 1;
                    },
                    'b' => {
                        board.set_piece(val, (Colour::Black, PieceType::B).into());
                        file += 1;
                    },

                    'R' => {
                        board.set_piece(val, (Colour::White, PieceType::R).into());
                        file += 1;
                    },
                    'r' => {
                        board.set_piece(val, (Colour::Black, PieceType::R).into());
                        file += 1;
                    },

                    'Q' => {
                        board.set_piece(val, (Colour::White, PieceType::Q).into());
                        file += 1;
                    },
                    'q' => {
                        board.set_piece(val, (Colour::Black, PieceType::Q).into());
                        file += 1;
                    },

                    'K' => {
                        board.set_piece(val, (Colour::White, PieceType::K).into());
                        file += 1;
                    },
                    'k' => {
                        board.set_piece(val, (Colour::Black, PieceType::K).into());
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
                        return Err(Unrecognised(char.to_string()));
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
                return Err(Unrecognised(v[1].into()));
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
                '-' => {
                    break
                }
                _ => {
                    return Err(Unrecognised(char.to_string()))
                }
            }
        }

        // Validate said castling rights
        let white_castling_squares = [0_u64, 7];
        for wc in board.castling_rights[Colour::White as usize].iter().zip(white_castling_squares.iter()) {
            if *wc.0 {
                match board.mailbox.get_piece((*wc.1).try_into()?) {
                    None => {
                        return Err(Castling)
                    }
                    Some(p) if (p.piece_type() != PieceType::R || p.colour() != Colour::White)=> {
                        return Err(Castling)
                    }
                    _ => {}
                }
            }
        }

        // Black
        let black_castling_squares = [56_u64, 63];
        for bc in board.castling_rights[Colour::Black as usize].iter().zip(black_castling_squares.iter()) {
            if *bc.0 {
                match board.mailbox.get_piece((*bc.1).try_into()?) {
                    None => {
                        return Err(Castling)
                    }
                    Some(p)
                    if (p.piece_type() != PieceType::R ||
                        p.colour() != Colour::Black) => {

                        return Err(Castling)
                    }
                    _ => {}
                }
            }
        }

        // En passant square
        board.en_passant = {
            if v[3] != "-" {
                match (v[3].chars().nth(1), v[3].chars().next()) {
                    (Some(r), Some(f)) => {
                        let ep_square: Square = (f, r).try_into()?;
                        ep_square.into()
                    },
                    _ => {
                        return Err(EnPassant)
                    }
                }
            }
            else {
                0.into()
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
    use crate::piece::colour::Colour;

    #[test]
    #[should_panic()]
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
    #[should_panic(expected = "Castling")]
    fn fen_parse_invalid_castling_black_kingside() {
        let fen_str = "r3k1r1/pp3ppp/2pp1nb1/q2Pp3/P3P3/2N5/1PP2PPP/R3K2R w KQkq e6 0 1";
        let board: Board = fen_str.parse().unwrap();
    }

    #[test]
    #[should_panic(expected = "PiecePosition")]
    fn fen_parse_invalid_spacing() {
        // Notice the 2pp2nb1
        let fen_str = "r3k1r1/pp3ppp/2pp2nb1/q2Pp3/P3P3/2N5/1PP2PPP/R3K2R w KQq e6 0 1";
        let board: Board = fen_str.parse().unwrap();
    }
}