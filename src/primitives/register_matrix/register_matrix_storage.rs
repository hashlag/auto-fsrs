use crate::primitives::register_uint::RegisterUInt;
use std::ops::{Index, IndexMut};

pub trait RegisterMatrixStorage<Register: RegisterUInt>:
    AsRef<[Register]>
    + AsMut<[Register]>
    + Index<usize, Output = Register>
    + IndexMut<usize, Output = Register>
    + Copy
{
    const SIZE: usize;
    fn zero_matrix() -> Self;
}

impl<Register, const N: usize> RegisterMatrixStorage<Register> for [Register; N]
where
    Register: RegisterUInt,
{
    const SIZE: usize = N;

    fn zero_matrix() -> Self {
        [Register::ZERO; N]
    }
}
