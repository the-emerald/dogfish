use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Clone, Copy, FromPrimitive, Debug, PartialEq)]
#[repr(u8)]
pub enum Colour {
    White = 0,
    Black = 1,
}