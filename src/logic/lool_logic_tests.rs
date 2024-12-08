use crate::logic::bool_logic;
use crate::logic::bit::Bit;

#[test]
fn n_and_test() {
    assert_eq!(bool_logic::n_and(Bit::O, Bit::O), Bit::I);
    assert_eq!(bool_logic::n_and(Bit::O, Bit::I), Bit::I);
    assert_eq!(bool_logic::n_and(Bit::I, Bit::O), Bit::I);
    assert_eq!(bool_logic::n_and(Bit::I, Bit::I), Bit::O);
}

#[test]
fn not_test() {
    assert_eq!(bool_logic::not(Bit::O), Bit::I);
    assert_eq!(bool_logic::not(Bit::I), Bit::O);
}

#[test]
fn and_test() {
    assert_eq!(bool_logic::and(Bit::O, Bit::O), Bit::O);
    assert_eq!(bool_logic::and(Bit::O, Bit::I), Bit::O);
    assert_eq!(bool_logic::and(Bit::I, Bit::O), Bit::O);
    assert_eq!(bool_logic::and(Bit::I, Bit::I), Bit::I);
}

#[test]
fn or_test() {
    assert_eq!(bool_logic::or(Bit::O, Bit::O), Bit::O);
    assert_eq!(bool_logic::or(Bit::O, Bit::I), Bit::I);
    assert_eq!(bool_logic::or(Bit::I, Bit::O), Bit::I);
    assert_eq!(bool_logic::or(Bit::I, Bit::I), Bit::I);
}

#[test]
fn xor_test() {
    assert_eq!(bool_logic::xor(Bit::O, Bit::O), Bit::O);
    assert_eq!(bool_logic::xor(Bit::O, Bit::I), Bit::I);
    assert_eq!(bool_logic::xor(Bit::I, Bit::O), Bit::I);
    assert_eq!(bool_logic::xor(Bit::I, Bit::I), Bit::O);
}

#[test]
fn multi_plexor_test() {
    assert_eq!(bool_logic::multiplexor(Bit::O, Bit::O, Bit::O), Bit::O);
    assert_eq!(bool_logic::multiplexor(Bit::O, Bit::I, Bit::O), Bit::O);
    assert_eq!(bool_logic::multiplexor(Bit::I, Bit::O, Bit::O), Bit::I);
    assert_eq!(bool_logic::multiplexor(Bit::I, Bit::I, Bit::O), Bit::I);
    assert_eq!(bool_logic::multiplexor(Bit::O, Bit::O, Bit::I), Bit::O);
    assert_eq!(bool_logic::multiplexor(Bit::O, Bit::I, Bit::I), Bit::I);
    assert_eq!(bool_logic::multiplexor(Bit::I, Bit::O, Bit::I), Bit::O);
    assert_eq!(bool_logic::multiplexor(Bit::I, Bit::I, Bit::I), Bit::I);
}

#[test]
fn de_multi_plexor_test() {
    assert_eq!(bool_logic::de_multiplexor(Bit::O, Bit::O), (Bit::O, Bit::O));
    assert_eq!(bool_logic::de_multiplexor(Bit::I, Bit::O), (Bit::I, Bit::O));
    assert_eq!(bool_logic::de_multiplexor(Bit::O, Bit::I), (Bit::O, Bit::O));
    assert_eq!(bool_logic::de_multiplexor(Bit::I, Bit::I), (Bit::O, Bit::I));
}
