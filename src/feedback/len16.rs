// x^16 + x^12 + x^3 + x^1 + 1
// bits 0, 4, 13, 15 i.e. 0xA011
pub fn feedback_a011(state: u16) -> bool {
    const POLY: u16 = 0xA011;
    (state & POLY).count_ones() % 2 == 1
}
