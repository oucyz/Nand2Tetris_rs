use crate::logic::bit::Bit;

pub fn n_and(a: Bit, b: Bit) -> Bit {
    if a == Bit::I && b == Bit::I {
        Bit::O
    } else {
        Bit::I
    }
}

pub fn not(a: Bit) -> Bit {
    n_and(a, a)
}

pub fn and(a: Bit, b: Bit) -> Bit {
    not(n_and(a, b))
}

pub fn or(a: Bit, b: Bit) -> Bit {
    not(and(not(a), not(b)))
}

pub fn xor(a: Bit, b: Bit) -> Bit {
    and(n_and(a, b), or(a, b))
    // another implementation
    //  not(or(and(a, b), not(or(a, b))))
}

pub fn multiplexor(a: Bit, b: Bit, sel: Bit) -> Bit {
    or(and(a, not(sel)), and(b, sel))
}

pub fn de_multiplexor(in_a: Bit, sel: Bit) -> (Bit, Bit) {
    (and(in_a, not(sel)), and(in_a, sel))
}
