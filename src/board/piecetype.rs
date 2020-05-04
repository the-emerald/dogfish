use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Copy, Clone, FromPrimitive, Debug, PartialEq)]
#[repr(u8)]
pub enum PieceType {
    P = 0,
    N = 1,
    B = 2,
    R = 3,
    Q = 4,
    K = 5
}