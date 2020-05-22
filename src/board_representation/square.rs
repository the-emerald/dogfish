use std::convert::{TryFrom, TryInto};
use crate::board_representation::square::ParseError::{InvalidSquare, InvalidRankFile, BitBoardNotUnit};
use crate::board_representation::bitboard::BitBoard;
use crate::board_representation::bitboard::shift::Direction;

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("square was {0} (must be 0..64)")]
    InvalidSquare(u64),
    #[error("rank/file was {0:?} (must be a..h 1..=8)")]
    InvalidRankFile((char, char)),
    #[error("bitboard contained >1 pieces")]
    BitBoardNotUnit
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Square(u64);

impl TryFrom<u64> for Square {
    type Error = ParseError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if !Square::valid_square(value) {
            return Err(InvalidSquare(value));
        }
        Ok(Square(value))
    }
}

impl TryFrom<(char, char)> for Square {
    type Error = ParseError;

    fn try_from(value: (char, char)) -> Result<Self, Self::Error> {
        let mut square: u64 = 0;
        match value.0 {
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
                return Err(InvalidRankFile(value));
            }
        }
        match value.1 {
            '1' => {},
            '2' => {
                square += 8;
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
                return Err(InvalidRankFile(value));
            }
        }

        Ok(Square(square))
    }
}

impl TryFrom<BitBoard> for Square {
    type Error = ParseError;

    fn try_from(value: BitBoard) -> Result<Self, Self::Error> {
        // TODO: Do something for when the bitboard is empty
        if (value & (value - 1_u64.into())) != 0_u64.into() {
            return Err(BitBoardNotUnit);
        }
        Ok((u64::from(value).trailing_zeros() as u64).try_into().unwrap())
    }
}

impl From<Square> for u64 {
    fn from(value: Square) -> Self {
        value.0
    }
}

impl Square {
    pub fn valid_square(sq: u64) -> bool {
        (0..64).contains(&sq)
    }

    pub const fn value(self) -> u64 {
        self.0
    }

    pub fn shift(self, direction: Direction) -> Self {
        BitBoard::from(self).shift(direction).try_into().unwrap()
    }
}