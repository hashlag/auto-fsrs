pub mod register_matrix_storage;
pub mod register_matrix_vector;

use register_matrix_storage::RegisterMatrixStorage;
use register_matrix_vector::RegisterMatrixVector;
use std::ops::{Index, IndexMut};

pub struct RegisterMatrix<BitVector: RegisterMatrixVector> {
    storage: BitVector::SquareMatrixStorage,
}

impl<BitVector> Default for RegisterMatrix<BitVector>
where
    BitVector: RegisterMatrixVector,
{
    fn default() -> Self {
        Self {
            storage: BitVector::SquareMatrixStorage::zero_matrix(),
        }
    }
}

impl<BitVector> RegisterMatrix<BitVector>
where
    BitVector: RegisterMatrixVector,
{
    pub fn from_storage(matrix_storage: BitVector::SquareMatrixStorage) -> Self {
        Self {
            storage: matrix_storage,
        }
    }

    pub fn size(&self) -> usize {
        self.storage.as_ref().len()
    }

    pub fn transform(&self, vector: BitVector) -> BitVector {
        const {
            assert!(BitVector::SquareMatrixStorage::SIZE > 0);

            let max_index = BitVector::SquareMatrixStorage::SIZE - 1;
            assert!((max_index as u64) <= u8::MAX as u64);

            assert!(
                (BitVector::SquareMatrixStorage::SIZE as u64)
                    == (BitVector::BITS as u64)
            );
        };

        let mut result: BitVector = BitVector::ZERO;

        for (idx, row) in self.storage.as_ref().iter().rev().enumerate() {
            result |= ((*row & vector).count_ones() & BitVector::from(1))
                << BitVector::from(idx as u8);
        }

        result
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
    fn default_test() {
        {
            let matrix: RegisterMatrix<u16> = RegisterMatrix::default();
            assert_eq!(16, matrix.size());

            for idx in 0..matrix.size() {
                assert_eq!(0, matrix[idx as usize]);
            }
        }

        {
            let matrix: RegisterMatrix<u32> = RegisterMatrix::default();
            assert_eq!(32, matrix.size());

            for idx in 0..matrix.size() {
                assert_eq!(0, matrix[idx as usize]);
            }
        }

        {
            let matrix: RegisterMatrix<u64> = RegisterMatrix::default();
            assert_eq!(64, matrix.size());

            for idx in 0..matrix.size() {
                assert_eq!(0, matrix[idx as usize]);
            }
        }
    }

    #[test]
    fn index_mut_test() {
        let mut matrix: RegisterMatrix<u32> = RegisterMatrix::default();
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

    #[test]
    fn transform_test() {
        {
            let matrix: RegisterMatrix<u16> = RegisterMatrix::from_storage([
                0b1000_0000_0000_0000u16,
                0b0100_0000_0000_0000u16,
                0b0010_0000_0000_0000u16,
                0b0001_0000_0000_0000u16,
                0b0000_1000_0000_0000u16,
                0b0000_0100_0000_0000u16,
                0b0000_0010_0000_0000u16,
                0b0000_0001_0000_0000u16,
                0b0000_0000_1000_0000u16,
                0b0000_0000_0100_0000u16,
                0b0000_0000_0010_0000u16,
                0b0000_0000_0001_0000u16,
                0b0000_0000_0000_1000u16,
                0b0000_0000_0000_0100u16,
                0b0000_0000_0000_0010u16,
                0b0000_0000_0000_0001u16,
            ]);

            assert_eq!(
                0b0000_1111_0000_1111u16,
                matrix.transform(0b0000_1111_0000_1111u16)
            );

            assert_eq!(
                0b1111_1100_1100_0000u16,
                matrix.transform(0b1111_1100_1100_0000u16)
            );
        }

        {
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

            assert_eq!(
                0b1111_0000_1111_0000u16,
                matrix.transform(0b0000_1111_0000_1111u16)
            );

            assert_eq!(
                0b0000_0000_0000_0000u16,
                matrix.transform(0b0000_0000_0000_0000u16)
            );

            assert_eq!(
                0b0001_0100_1111_0101u16,
                matrix.transform(0b1010_1111_0010_1000u16)
            );
        }

        {
            let matrix: RegisterMatrix<u16> = RegisterMatrix::default();

            assert_eq!(0, matrix.transform(u16::MAX));
        }
    }
}
