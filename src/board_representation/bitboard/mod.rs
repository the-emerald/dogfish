use anyhow::anyhow;
use std::str::FromStr;
use std::fmt;
use std::fmt::Formatter;
use itertools::Itertools;
use std::borrow::Borrow;

pub mod files;
pub mod ranks;
pub mod shift;
pub mod ops;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BitBoard {
    board: u64,
}

impl BitBoard {
    pub const fn zero() -> Self {
        Self {
            board: 0
        }
    }

    pub const fn from_square(val: u64) -> Self {
        Self {
            board: 1u64 << val
        }
    }

    pub const fn from_shifted(val: u64) -> Self {
        Self {
            board: val
        }
    }

    pub fn iter_bits(self) -> impl Iterator<Item = bool> {
        (0..64).rev().map(move |x| (self.board >> x) & 1 == 1)
    }
}

impl FromStr for BitBoard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:#018x}", self.board))
    }
}

impl fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let file = "\n  A B C D E F G H";
        let board = self.iter_bits()
            .map(
                |x| if x {"x "} else {". "}
            )
            .collect::<String>()
            .chars()
            .chunks(16)
            .into_iter()
            .enumerate()
            .map(|x| format!("{} {}", 8-x.0, x.1
                .collect::<String>()
                .chars()
                .rev()
                .collect::<String>()
            ))
            .join("\n");

        write!(f, "{}\n{}", file, board)
    }
}