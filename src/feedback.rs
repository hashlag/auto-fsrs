// x^32 + x^22 + x^2 + x^1 + 1
// bits 0, 10, 30, 31 i.e. 0xC0000401
pub fn feedback32_22_2_1(state: u32) -> bool {
    const POLY: u32 = 0xC000_0401;
    (state & POLY).count_ones() % 2 == 1
}

// x^32 + x^28 + x^19 + x^18 + x^16 + x^14 + x^11 + x^10 + x^9 + x^6 + x^5 + x^1 + 1
// bits 0, 4, 13, 14, 16, 18, 21, 22, 23, 26, 27, 31 i.e. 0x8CE56011
pub fn feedback_32_28_19_18_16_14_11_10_9_6_5_1(state: u32) -> bool {
    const POLY: u32 = 0x8CE5_6011;
    (state & POLY).count_ones() % 2 == 1
}
