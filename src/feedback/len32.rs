// x^32 + x^22 + x^2 + x^1 + 1
// bits 0, 10, 30, 31 i.e. 0xC0000401
pub fn feedback_c0000401(state: u32) -> bool {
    const POLY: u32 = 0xC000_0401;
    (state & POLY).count_ones() % 2 == 1
}

// x^32 + x^28 + x^19 + x^18 + x^16 + x^14 + x^11 + x^10 + x^9 + x^6 + x^5 + x^1 + 1
// bits 0, 4, 13, 14, 16, 18, 21, 22, 23, 26, 27, 31 i.e. 0x8CE56011
pub fn feedback_8ce56011(state: u32) -> bool {
    const POLY: u32 = 0x8CE5_6011;
    (state & POLY).count_ones() % 2 == 1
}

// f = b31 + b11 + (b9*b6) + (b4*b0) + 1
pub fn feedback_nl_0(state: u32) -> bool {
    let b31: bool = ((state >> 31) & 1) == 1;
    let b11: bool = ((state >> 11) & 1) == 1;
    let b9: bool = ((state >> 9) & 1) == 1;
    let b6: bool = ((state >> 6) & 1) == 1;
    let b4: bool = ((state >> 4) & 1) == 1;
    let b0: bool = (state & 1) == 1;

    b31 ^ b11 ^ (b9 & b6) ^ (b4 & b0) ^ true
}
