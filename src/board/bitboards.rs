use std::ops::{BitOr, BitAnd, BitXor, Shl, Shr, BitOrAssign, BitAndAssign, BitXorAssign};
use anyhow::anyhow;

#[derive(Copy, Clone)]
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

    pub fn valid_square(sq: &u8) -> bool {
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
                return Err(anyhow!("invalid file in rank/file"));
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
                return Err(anyhow!("invalid rank in rank/file"));
            }
        }

        Ok(square)
    }
}

// Define operations for bitboards

impl BitOr for BitBoard {
    type Output = ();

    fn bitor(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl BitAnd for BitBoard {
    type Output = ();

    fn bitand(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl BitXor for BitBoard {
    type Output = ();

    fn bitxor(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl Shl for BitBoard {
    type Output = ();

    fn shl(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl Shr for BitBoard {
    type Output = ();

    fn shr(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}