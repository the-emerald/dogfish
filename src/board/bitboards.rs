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

    pub fn iter_bits(self) -> impl Iterator<Item = bool> {
        (0..64).rev().map(move |x| (self.board >> x) & 1 == 1)
    }
}
