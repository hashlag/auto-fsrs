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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u16_test() {
        assert_eq!(16, <u16 as RegisterUInt>::BITS);

        assert_eq!(0, <u16 as RegisterUInt>::count_ones(0u16));
        assert_eq!(1, <u16 as RegisterUInt>::count_ones(1u16));
        assert_eq!(16, <u16 as RegisterUInt>::count_ones(u16::MAX));
    }

    #[test]
    fn u32_test() {
        assert_eq!(32, <u32 as RegisterUInt>::BITS);

        assert_eq!(0, <u32 as RegisterUInt>::count_ones(0u32));
        assert_eq!(1, <u32 as RegisterUInt>::count_ones(1u32));
        assert_eq!(32, <u32 as RegisterUInt>::count_ones(u32::MAX));
    }

    #[test]
    fn u64_test() {
        assert_eq!(64, <u64 as RegisterUInt>::BITS);

        assert_eq!(0, <u64 as RegisterUInt>::count_ones(0u64));
        assert_eq!(1, <u64 as RegisterUInt>::count_ones(1u64));
        assert_eq!(64, <u64 as RegisterUInt>::count_ones(u64::MAX));
    }
}
