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
