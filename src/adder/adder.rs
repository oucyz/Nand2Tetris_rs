use crate::logic::bool_logic::{and, or, xor};
use crate::given::{Word16, Word8};

pub fn half_adder(a: u8, b: u8) -> [u8; 2]{
    [and(a, b), xor(a, b)]
}

pub fn full_adder(a: u8, b: u8, c: u8) -> [u8; 2]{
    let [carry_ab, sum_ab] = half_adder(a, b);
    let [carry, sum_abc] = half_adder(sum_ab, c);
    [or(carry_ab, carry), sum_abc]
}

pub fn add16(a: Word16, b: Word16) -> Word16{
    let [carry1, sum0] = half_adder(a[0], b[0]);
    let [carry2, sum1] = full_adder(a[1], b[1], carry1);
    let [carry3, sum2] = full_adder(a[2], b[2], carry2);
    let [carry4, sum3] = full_adder(a[3], b[3], carry3);
    let [carry5, sum4] = full_adder(a[4], b[4], carry4);
    let [carry6, sum5] = full_adder(a[5], b[5], carry5);
    let [carry7, sum6] = full_adder(a[6], b[6], carry6);
    let [carry8, sum7] = full_adder(a[7], b[7], carry7);
    let [carry9, sum8] = full_adder(a[8], b[8], carry8);
    let [carry10, sum9] = full_adder(a[9], b[9], carry9);
    let [carry11, sum10] = full_adder(a[10], b[10], carry10);
    let [carry12, sum11] = full_adder(a[11], b[11], carry11);
    let [carry13, sum12] = full_adder(a[12], b[12], carry12);
    let [carry14, sum13] = full_adder(a[13], b[13], carry13);
    let [carry15, sum14] = full_adder(a[14], b[14], carry14);
    let [carry16, sum15] = full_adder(a[15], b[15], carry15);
    
    [
        sum0, sum1, sum2, sum3,
        sum4, sum5, sum6, sum7,
        sum8, sum9, sum10, sum11,
        sum12, sum13, sum14, sum15
    ]
}

