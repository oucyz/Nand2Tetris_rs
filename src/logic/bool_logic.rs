pub fn n_and(a: i32, b: i32) -> i32 {
    if a == 1 && b == 1 {
        0
    } else {
        1
    }
}

pub fn not(a: i32) -> i32 {
    n_and(a, a)
}

pub fn and(a: i32, b: i32) -> i32 {
    not(n_and(a, b))
}

pub fn or(a: i32, b: i32) -> i32 {
    not(and(not(a), not(b)))
}

pub fn xor(a: i32, b: i32) -> i32 {
    println!("n_or");
}
