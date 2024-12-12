use crate::logic::bool_logic;
// use crate::logic::bit::Bit;

#[test]
fn n_and_test() {
    assert_eq!(bool_logic::n_and(0, 0), 1);
    assert_eq!(bool_logic::n_and(0, 1), 1);
    assert_eq!(bool_logic::n_and(1, 0), 1);
    assert_eq!(bool_logic::n_and(1, 1), 0);
}

#[test]
fn not_test() {
    assert_eq!(bool_logic::not(0), 1);
    assert_eq!(bool_logic::not(1), 0);
}

#[test]
fn and_test() {
    assert_eq!(bool_logic::and(0, 0), 0);
    assert_eq!(bool_logic::and(0, 1), 0);
    assert_eq!(bool_logic::and(1, 0), 0);
    assert_eq!(bool_logic::and(1, 1), 1);
}

#[test]
fn or_test() {
    assert_eq!(bool_logic::or(0, 0), 0);
    assert_eq!(bool_logic::or(0, 1), 1);
    assert_eq!(bool_logic::or(1, 0), 1);
    assert_eq!(bool_logic::or(1, 1), 1);
}

#[test]
fn xor_test() {
    assert_eq!(bool_logic::xor(0, 0), 0);
    assert_eq!(bool_logic::xor(0, 1), 1);
    assert_eq!(bool_logic::xor(1, 0), 1);
    assert_eq!(bool_logic::xor(1, 1), 0);
}

#[test]
fn multi_plexor_test() {
    assert_eq!(bool_logic::multiplexor(0, 0, 0), 0);
    assert_eq!(bool_logic::multiplexor(0, 1, 0), 0);
    assert_eq!(bool_logic::multiplexor(1, 0, 0), 1);
    assert_eq!(bool_logic::multiplexor(1, 1, 0), 1);
    assert_eq!(bool_logic::multiplexor(0, 0, 1), 0);
    assert_eq!(bool_logic::multiplexor(0, 1, 1), 1);
    assert_eq!(bool_logic::multiplexor(1, 0, 1), 0);
    assert_eq!(bool_logic::multiplexor(1, 1, 1), 1);
}

#[test]
fn de_multi_plexor_test() {
    assert_eq!(bool_logic::de_multiplexor(0, 0), (0, 0));
    assert_eq!(bool_logic::de_multiplexor(1, 0), (1, 0));
    assert_eq!(bool_logic::de_multiplexor(0, 1), (0, 0));
    assert_eq!(bool_logic::de_multiplexor(1, 1), (0, 1));
}
