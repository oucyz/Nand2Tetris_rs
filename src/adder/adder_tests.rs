use crate::adder::adder;

#[test]
fn half_adder_test() {
    assert_eq!(adder::half_adder(0, 0), [0, 0]);
    assert_eq!(adder::half_adder(0, 1), [0, 1]);
    assert_eq!(adder::half_adder(1, 0), [0, 1]);
    assert_eq!(adder::half_adder(1, 1), [1, 0]);
}

#[test]
fn full_adder_test() {
    assert_eq!(adder::full_adder(0, 0, 0), [0, 0]);
    assert_eq!(adder::full_adder(1, 0, 0), [0, 1]);
    assert_eq!(adder::full_adder(0, 1, 0), [0, 1]);
    assert_eq!(adder::full_adder(0, 0, 1), [0, 1]);
    assert_eq!(adder::full_adder(1, 1, 0), [1, 0]);
    assert_eq!(adder::full_adder(0, 1, 1), [1, 0]);
    assert_eq!(adder::full_adder(1, 0, 1), [1, 0]);
    assert_eq!(adder::full_adder(1, 1, 1), [1, 1]);
}

