use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Shl, ShlAssign, Shr, ShrAssign, Sub};

pub trait FsRegister:
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
{
    const BITS: Self;
}

impl FsRegister for u16 {
    const BITS: u16 = u16::BITS as u16;
}

impl FsRegister for u32 {
    const BITS: u32 = u32::BITS;
}
