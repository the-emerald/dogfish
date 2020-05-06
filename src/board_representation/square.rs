use std::convert::TryFrom;
use anyhow::anyhow;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Square(u64);

impl TryFrom<u64> for Square {
    type Error = anyhow::Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if !Square::valid_square(value) {
            return Err(anyhow!("attempted to create invalid square from u64"));
        }
        Ok(Square(value))
    }
}

impl TryFrom<(char, char)> for Square {
    type Error = anyhow::Error;

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
                return Err(anyhow!("attempted to create invalid square from ([char], char)"));
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
                return Err(anyhow!("attempted to create invalid square from (char, [char])"));
            }
        }

        Ok(Square(square))
    }
}

impl Square {
    pub fn valid_square(sq: u64) -> bool {
        (0..64).contains(&sq)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}