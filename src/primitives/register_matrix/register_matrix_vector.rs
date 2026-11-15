use crate::primitives::register_matrix::RegisterMatrixStorage;
use crate::primitives::register_uint::RegisterUInt;

pub trait RegisterMatrixVector: RegisterUInt {
    type SquareMatrixStorage: RegisterMatrixStorage<Self>;
}

impl RegisterMatrixVector for u16 {
    type SquareMatrixStorage = [u16; u16::BITS as usize];
}

impl RegisterMatrixVector for u32 {
    type SquareMatrixStorage = [u32; u32::BITS as usize];
}

impl RegisterMatrixVector for u64 {
    type SquareMatrixStorage = [u64; u64::BITS as usize];
}
