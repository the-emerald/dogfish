use crate::board_representation::bitboard::BitBoard;

pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    Northwest,
}
impl BitBoard {
    pub fn shift(direction: Direction, bitboard: Self) -> Self {
        // TODO: Implement shifting
        // match direction {
        //     Direction::North => {bitboard << BitBoard::from_square(8)},
        //     Direction::NorthEast => {},
        //     Direction::East => {},
        //     Direction::SouthEast => {},
        //     Direction::South => {bitboard >> BitBoard::from_square(8)},
        //     Direction::SouthWest => {},
        //     Direction::West => {},
        //     Direction::Northwest => {},
        // }
        unimplemented!()
    }
}