use std::ops::{BitOr, BitAnd, BitXor, Shl, Shr, BitOrAssign, BitAndAssign, BitXorAssign};
use anyhow::anyhow;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct BitBoard {
    board: u64,
}

impl BitBoard {
    pub fn zero() -> Self {
        Self {
            board: 0
        }
    }

    pub fn from_unshifted(val: u64) -> Self {
        Self {
            board: 1u64 << val
        }
    }

    pub fn from_shifted(val: u64) -> Self {
        Self {
            board: val
        }
    }

    pub fn valid_square(sq: &u64) -> bool {
        (0..64).contains(sq)
    }

    pub fn iter_bits(self) -> impl Iterator<Item = bool> {
        (0..64).rev().map(move |x| (self.board >> x) & 1 == 1)
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

// Define operations for bitboards

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard::from_shifted(self.board | rhs.board)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.board |= rhs.board
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard::from_shifted(self.board & rhs.board)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.board &= rhs.board
    }
}

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard::from_shifted(self.board ^ rhs.board)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.board ^= rhs.board
    }
}

impl Shl for BitBoard {
    type Output = BitBoard;

    fn shl(self, rhs: Self) -> Self::Output {
        BitBoard::from_shifted(self.board << rhs.board)
    }
}

impl Shr for BitBoard {
    type Output = BitBoard;

    fn shr(self, rhs: Self) -> Self::Output {
        BitBoard::from_shifted(self.board >> rhs.board)
    }
}

impl FromStr for BitBoard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl ToString for BitBoard {
    fn to_string(&self) -> String {
        self.iter_bits().map(|x| (x as usize).to_string()).collect()
    }
}