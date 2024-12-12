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

pub fn multiplexor(a: u8, b: u8, sel: u8) -> u8 {
    or(and(a, not(sel)), and(b, sel))
}

pub fn de_multiplexor(in_a: u8, sel: u8) -> (u8, u8) {
    (and(in_a, not(sel)), and(in_a, sel))
}

pub type Word = [u8; 8];

