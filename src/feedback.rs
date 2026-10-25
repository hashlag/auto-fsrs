// x^32 + x^22 + x^2 + x^1 + 1
// bits 0, 10, 30, 31 i.e. 0xC0000401
pub fn feedback32_22_2_1(state: u32) -> bool {
    const POLY: u32 = 0xC000_0401;
    (state & POLY).count_ones() % 2 == 1
}
