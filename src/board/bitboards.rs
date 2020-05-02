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
}