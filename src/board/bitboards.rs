use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct BitBoard {
    board: u64,
}

impl BitBoard {
    pub fn new() -> Self {
        Self {
            board: 0
        }
    }

    pub fn valid_square(sq: &u8) -> bool {
        (0..64).contains(sq)
    }
}

impl IntoIterator for BitBoard {
    type Item = u8;
    type IntoIter = BitBoardIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIntoIterator {
            bitboard: self,
            index: 1
        }
    }
}

impl<'a> IntoIterator for &'a BitBoard {
    type Item = u8;
    type IntoIter = BitBoardIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitBoardIterator {
            bitboard: self,
            index: 1
        }
    }
}

pub struct BitBoardIntoIterator {
    bitboard: BitBoard,
    index: u32
}

impl Iterator for BitBoardIntoIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let nxt = match self.index.cmp(&64) {
            Ordering::Less => {
                Some((self.bitboard.board.rotate_left(self.index) & 0b1) as u8)
            }
            _ => {
                None
            }
        };
        self.index += 1;
        nxt
    }
}


pub struct BitBoardIterator<'a> {
    bitboard: &'a BitBoard,
    index: u32
}

impl<'a> Iterator for BitBoardIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let nxt = match self.index.cmp(&64) {
            Ordering::Less => {
                Some((self.bitboard.board.rotate_left(self.index) & 0b1) as u8)
            }
            _ => {
                None
            }
        };
        self.index += 1;
        nxt
    }
}