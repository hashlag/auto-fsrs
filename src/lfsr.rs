use crate::{fsr::Fsr, register::FsRegister};

pub struct Lfsr<Register: FsRegister> {
    fsr: Fsr<Register>,
}

impl<Register> Lfsr<Register>
where
    Register: FsRegister,
{
    pub fn new(initial_state: Register, feedback_poly: Register) -> Self {
        let feedback_fn = move |state: Register| -> bool {
            (state & feedback_poly).count_ones() & Register::from(1u8) == Register::from(1u8)
        };

        Self {
            fsr: Fsr::new(initial_state, feedback_fn),
        }
    }

    pub fn skip(&mut self, n: u32) {
        self.fsr.skip(n);
    }

    #[must_use]
    pub fn step(&mut self) -> bool {
        self.fsr.step()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fmt::Write;

    #[test]
    fn basic_test() {
        let mut lfsr: Lfsr<u32> = Lfsr::new(1, 0xC000_0401);

        let expected: String = String::from(
            "10000000000000000000000000000000110110110110110110110100010100011\
             11001111001000010011001000010010101110010011101010010110010001011\
             10000101000001000000001110100100110001110110001110101101010011000\
             1100001000111100001000010111011011110101010011001001010",
        );

        let mut fact: String = String::new();

        for _ in 0..expected.chars().count() {
            write!(fact, "{}", lfsr.step() as u8).unwrap();
        }

        assert_eq!(expected, fact);
    }

    #[test]
    fn basic16bit_test() {
        let mut lfsr: Lfsr<u16> = Lfsr::new(2, 0xA011);

        let expected: String = String::from(
            "01000000000000000111010011101100110000111001110110001101010010000\
             11001100000101001110000110110111111110010100000001000110111011101\
             01010011101010011001001001010011011011001010000100000100001001101\
             1101101010000000010100001110111100101101001011111111111",
        );

        let mut fact: String = String::new();

        for _ in 0..expected.chars().count() {
            write!(fact, "{}", lfsr.step() as u8).unwrap();
        }

        assert_eq!(expected, fact);
    }

    #[test]
    fn basic64bit_test() {
        let mut lfsr: Lfsr<u64> = Lfsr::new(10, 0xB000000000000001);

        let expected: String = String::from(
            "01010000000000000000000000000000000000000000000000000000000000000\
             11011011011011011011011011011011011011011011011011011011011011010\
             01101001101001101001101001101001101001101001101001101001101001110\
             1011010011101011010011101011010011101011010011101011010",
        );

        let mut fact: String = String::new();

        for _ in 0..expected.chars().count() {
            write!(fact, "{}", lfsr.step() as u8).unwrap();
        }

        assert_eq!(expected, fact);
    }

    #[test]
    fn skip_test() {
        let mut lfsr: Lfsr<u32> = Lfsr::new(0b00010000110000100000000100001111, 0x8CE5_6011);

        let expected: String = String::from(
            "01011111111110111001110000111111100010010010000100011110010010111\
             11001001000101100001101000100010001",
        );

        let mut fact: String = String::new();

        lfsr.skip(400);

        for _ in 0..expected.chars().count() {
            write!(fact, "{}", lfsr.step() as u8).unwrap();
        }

        assert_eq!(expected, fact);
    }
}
