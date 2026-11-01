use crate::register::FsRegister;

pub struct Fsr<Register: FsRegister> {
    state: Register,
    feedback_fn: Box<dyn Fn(Register) -> bool>,
}

impl<Register> Fsr<Register>
where
    Register: FsRegister,
{
    pub fn new(initial_state: Register, feedback_fn: impl Fn(Register) -> bool + 'static) -> Self {
        Self {
            state: initial_state,
            feedback_fn: Box::new(feedback_fn),
        }
    }

    pub fn skip(&mut self, n: u32) {
        for _ in 0..n {
            let feedback: bool = (self.feedback_fn)(self.state);
            self.shift_impl(feedback);
        }
    }

    #[must_use]
    pub fn step(&mut self) -> bool {
        let feedback: bool = (self.feedback_fn)(self.state);
        let discarded_bit: bool = (self.state & 1u8.into()) == 1u8.into();

        self.shift_impl(feedback);

        discarded_bit
    }

    fn shift_impl(&mut self, feedback: bool) {
        self.state >>= 1u8.into();
        self.state |= Register::from(feedback as u8) << (Register::BITS - 1u8.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feedback;

    use std::fmt::Write;

    #[test]
    fn basic_test() {
        let mut lfsr: Fsr<u32> = Fsr::new(0b1, feedback::len32::feedback_c0000401);

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
    fn basic2_test() {
        let mut lfsr: Fsr<u32> = Fsr::new(
            0b00010000110000100000000100001111,
            feedback::len32::feedback_8ce56011,
        );

        let expected: String = String::from(
            "11110000100000000100001100001000101101011110011100101110011101000\
             11100010000011000111001110111110110001011100011101010100001110011\
             11010101100110010001101111010101111111111101001001101101110001000\
             0011001011101001010010000001010101010110111111000010011",
        );

        let mut fact: String = String::new();

        for _ in 0..expected.chars().count() {
            write!(fact, "{}", lfsr.step() as u8).unwrap();
        }

        assert_eq!(expected, fact);
    }

    #[test]
    fn basic16bit_test() {
        let mut lfsr: Fsr<u16> = Fsr::new(2, feedback::len16::feedback_a011);

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
    fn zero_init_test() {
        let mut lfsr: Fsr<u32> = Fsr::new(0, feedback::len32::feedback_8ce56011);

        for _ in 0..250 {
            assert!(!lfsr.step());
        }
    }

    #[test]
    fn skip_test() {
        let mut lfsr: Fsr<u32> = Fsr::new(
            0b00010000110000100000000100001111,
            feedback::len32::feedback_8ce56011,
        );

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
