pub type Word = [u8; 16];
pub type Word8 = [u8; 8];

pub fn n_and(a: u8, b: u8) -> u8 {
    if a == 1 && b == 1{
        0
    } else {
        1
    }
}

pub fn not(a: u8) -> u8 {
    n_and(a, a)
}

pub fn and(a: u8, b: u8) -> u8 {
    not(n_and(a, b))
}

pub fn or(a: u8, b: u8) -> u8 {
    not(and(not(a), not(b)))
}

pub fn xor(a: u8, b: u8) -> u8 {
    and(n_and(a, b), or(a, b))
    // another implementation
    //  not(or(and(a, b), not(or(a, b))))
}

pub fn mux(a: u8, b: u8, sel: u8) -> u8 {
    or(and(a, not(sel)), and(b, sel))
}

pub fn demux(in_a: u8, sel: u8) -> [u8; 2] {
    [and(in_a, not(sel)), and(in_a, sel)]
}


pub fn not16(a: Word) -> Word{
    [
        not(a[0]), not(a[1]), not(a[2]), not(a[3]),
        not(a[4]), not(a[5]), not(a[6]), not(a[7]),
        not(a[8]), not(a[9]), not(a[10]), not(a[11]),
        not(a[12]), not(a[13]), not(a[14]), not(a[15]),
    ]
}

pub fn and16(a: Word, b: Word) -> Word{
    [
        and(a[0], b[0]), and(a[1], b[1]), and(a[2], b[2]), and(a[3], b[3]),
        and(a[4], b[4]), and(a[5], b[5]), and(a[6], b[6]), and(a[7], b[7]),
        and(a[8], b[8]), and(a[9], b[9]), and(a[10], b[10]), and(a[11], b[11]),
        and(a[12], b[12]), and(a[13], b[13]), and(a[14], b[14]), and(a[15], b[15]),
    ]
}

pub fn or16(a: Word, b: Word) -> Word{
    [
        or(a[0], b[0]), or(a[1], b[1]), or(a[2], b[2]), or(a[3], b[3]),
        or(a[4], b[4]), or(a[5], b[5]), or(a[6], b[6]), or(a[7], b[7]),
        or(a[8], b[8]), or(a[9], b[9]), or(a[10], b[10]), or(a[11], b[11]),
        or(a[12], b[12]), or(a[13], b[13]), or(a[14], b[14]), or(a[15], b[15]),
    ]
}

pub fn mux16(a: Word, b: Word, sel: u8) -> Word{
    [
        mux(a[0], b[0], sel), mux(a[1], b[1], sel), mux(a[2], b[2], sel), mux(a[3], b[3], sel),
        mux(a[4], b[4], sel), mux(a[5], b[5], sel), mux(a[6], b[6], sel), mux(a[7], b[7], sel),
        mux(a[8], b[8], sel), mux(a[9], b[9], sel), mux(a[10], b[10], sel), mux(a[11], b[11], sel),
        mux(a[12], b[12], sel), mux(a[13], b[13], sel), mux(a[14], b[14], sel), mux(a[15], b[15], sel),
    ]
}

pub fn or8way(a: Word8) -> u8{
    or(or(or(or(or(or(or(a[0], a[1]), a[2]), a[3]), a[4]), a[5]), a[6]), a[7])
}

pub fn mux4way16(a:[Word; 4], sel: [u8; 2]) -> Word{
    mux16(mux16(a[0], a[1], sel[0]), mux16(a[2], a[3], sel[0]), sel[1])
}

pub fn mux8way16(a: [Word; 8], sel: [u8; 3]) -> Word{
    mux16(
        mux4way16([a[0], a[1], a[2], a[3]], [sel[0], sel[1]]),
        mux4way16([a[4], a[5], a[6], a[7]], [sel[0], sel[1]]),
        sel[2]
    )
}

pub fn demux4way(in_a: u8, sel: [u8; 2]) -> [u8; 4]{
    let [ab, cd] = demux(in_a, sel[1]);
    let [a, b] = demux(ab, sel[0]);
    let [c, d] = demux(cd, sel[0]);
    [a, b, c, d]
    // [
    //     demux(demux(in_a, sel[1])[0], sel[0])[0],
    //     demux(demux(in_a, sel[1])[0], sel[0])[1],
    //     demux(demux(in_a, sel[1])[1], sel[0])[0],
    //     demux(demux(in_a, sel[1])[1], sel[0])[1]
    // ]
}

pub fn demux8way(in_a: u8, sel: [u8; 3]) -> [u8; 8]{
    let [abcd, efgh] = demux(in_a, sel[2]);
    let [a, b, c, d] = demux4way(abcd, [sel[0], sel[1]]);
    let [e, f, g, h] = demux4way(efgh, [sel[0], sel[1]]);
    [a, b, c, d, e, f, g, h]
}