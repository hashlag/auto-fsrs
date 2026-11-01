// x^64 + x^4 + x^3 + x^1 + 1
// bits 0, 60, 61, 63 i.e. 0xB000000000000001
pub fn feedback_b000000000000001(state: u64) -> bool {
    const POLY: u64 = 0xB000_0000_0000_0001;
    (state & POLY).count_ones() % 2 == 1
}
