pub mod register_matrix_storage;
pub mod register_matrix_vector;

use register_matrix_storage::RegisterMatrixStorage;
use register_matrix_vector::RegisterMatrixVector;
use std::ops::{Index, IndexMut};

pub struct RegisterMatrix<BitVector: RegisterMatrixVector> {
    storage: BitVector::SquareMatrixStorage,
}

impl<BitVector> RegisterMatrix<BitVector>
where
    BitVector: RegisterMatrixVector,
{
    pub fn new() -> Self {
        Self {
            storage: BitVector::SquareMatrixStorage::zero_matrix(),
        }
    }

    pub fn from_storage(matrix_storage: BitVector::SquareMatrixStorage) -> Self {
        Self {
            storage: matrix_storage,
        }
    }

    pub fn size(&self) -> usize {
        self.storage.as_ref().len()
    }
}

impl<BitVector> Index<usize> for RegisterMatrix<BitVector>
where
    BitVector: RegisterMatrixVector,
{
    type Output = BitVector;

    fn index(&self, index: usize) -> &Self::Output {
        &self.storage[index]
    }
}

impl<BitVector> IndexMut<usize> for RegisterMatrix<BitVector>
where
    BitVector: RegisterMatrixVector,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.storage[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        {
            let matrix: RegisterMatrix<u16> = RegisterMatrix::new();
            assert_eq!(16, matrix.size());

            for idx in 0..matrix.size() {
                assert_eq!(0, matrix[idx as usize]);
            }
        }

        {
            let matrix: RegisterMatrix<u32> = RegisterMatrix::new();
            assert_eq!(32, matrix.size());

            for idx in 0..matrix.size() {
                assert_eq!(0, matrix[idx as usize]);
            }
        }

        {
            let matrix: RegisterMatrix<u64> = RegisterMatrix::new();
            assert_eq!(64, matrix.size());

            for idx in 0..matrix.size() {
                assert_eq!(0, matrix[idx as usize]);
            }
        }
    }

    #[test]
    fn index_mut_test() {
        let mut matrix: RegisterMatrix<u32> = RegisterMatrix::new();
        assert_eq!(0, matrix[0]);

        matrix[0] = 123;
        assert_eq!(123, matrix[0]);

        matrix[0] = 0;
        assert_eq!(0, matrix[0]);

        for idx in 0..matrix.size() {
            matrix[idx as usize] = idx as u32;
        }

        for idx in 0..matrix.size() {
            assert_eq!(idx as u32, matrix[idx as usize]);
        }

        matrix[1] = u32::MAX;
        assert_eq!(u32::MAX, matrix[1]);
    }

    #[test]
    fn from_storage_test() {
        let matrix: RegisterMatrix<u16> = RegisterMatrix::from_storage([
            0b0000_0000_0000_0001u16,
            0b0000_0000_0000_0010u16,
            0b0000_0000_0000_0100u16,
            0b0000_0000_0000_1000u16,
            0b0000_0000_0001_0000u16,
            0b0000_0000_0010_0000u16,
            0b0000_0000_0100_0000u16,
            0b0000_0000_1000_0000u16,
            0b0000_0001_0000_0000u16,
            0b0000_0010_0000_0000u16,
            0b0000_0100_0000_0000u16,
            0b0000_1000_0000_0000u16,
            0b0001_0000_0000_0000u16,
            0b0010_0000_0000_0000u16,
            0b0100_0000_0000_0000u16,
            0b1000_0000_0000_0000u16,
        ]);

        for idx in 0..matrix.size() {
            assert_eq!(1u16 << idx, matrix[idx]);
        }
    }
}
