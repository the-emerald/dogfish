use once_cell::sync::Lazy;
use itertools::iproduct;
use crate::board_representation::square::Square;
use std::convert::TryFrom;
use std::cmp::max;

pub static SQUARE_DISTANCE: Lazy<[[u8; 64]; 64]> = Lazy::new(|| {
    let mut sd = [[0_u8; 64]; 64];
    let all_sq_1 = (0_u64..64).map(|s| Square::try_from(s).unwrap());
    let all_sq_2 = (0_u64..64).map(|s| Square::try_from(s).unwrap());

    for (s1, s2) in iproduct!(all_sq_1, all_sq_2) {
        sd[s1.value() as usize][s2.value() as usize] = distance(s1, s2)
    }

    sd
});

fn distance(a: Square, b: Square) -> u8 {
    let f1 = a.value() as i64 & 7;
    let f2 = b.value() as i64 & 7;

    let r1 = a.value() as i64 >> 3;
    let r2 = b.value() as i64 >> 3;

    let rd = (r2 - r1).abs();
    let fd = (f2 - f1).abs();

    max(rd as u8, fd as u8)
}

#[cfg(test)]
mod tests {
    use crate::common::distance::SQUARE_DISTANCE;

    #[test]
    fn file_distance() {
        assert_eq!(SQUARE_DISTANCE[0_usize][56_usize], 7);
        assert_eq!(SQUARE_DISTANCE[0_usize][24_usize], 3);
    }

    #[test]
    fn rank_distance() {
        assert_eq!(SQUARE_DISTANCE[0_usize][7_usize], 7);
        assert_eq!(SQUARE_DISTANCE[0_usize][4_usize], 4);
    }

    #[test]
    fn diag_distance() {
        assert_eq!(SQUARE_DISTANCE[0_usize][63_usize], 7);
        assert_eq!(SQUARE_DISTANCE[0_usize][45_usize], 5);

    }

    #[test]
    fn non_ray_distance() {
        assert_eq!(SQUARE_DISTANCE[0_usize][57_usize], 7);
        assert_eq!(SQUARE_DISTANCE[0_usize][10_usize], 2);

    }
}