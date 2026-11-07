use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Shl, ShlAssign, Shr, ShrAssign, Sub};

pub trait RegisterUInt:
    Sized
    + Copy
    + Shl<Output = Self>
    + Shr<Output = Self>
    + ShlAssign
    + ShrAssign
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitAndAssign
    + BitOrAssign
    + Sub<Output = Self>
    + PartialEq
    + From<u8>
    + 'static
{
    const BITS: Self;
    fn count_ones(self) -> Self;
}

impl RegisterUInt for u16 {
    const BITS: u16 = u16::BITS as u16;

    fn count_ones(self) -> u16 {
        u16::count_ones(self) as u16
    }
}

impl RegisterUInt for u32 {
    const BITS: u32 = u32::BITS;

    fn count_ones(self) -> Self {
        u32::count_ones(self)
    }
}

impl RegisterUInt for u64 {
    const BITS: u64 = u64::BITS as u64;

    fn count_ones(self) -> Self {
        u64::count_ones(self) as u64
    }
}
