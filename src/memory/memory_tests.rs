use crate::given::{Flipflop, n_and, Word16, Word8};
use crate::memory::memory::{Bit, Register, RAM8, RAM64};

#[test]
fn bit_test(){
    assert_eq!(Bit::new().out(), 0);
    
    let mut bit = Bit::new();
    bit.input(1, 1);
    assert_eq!(bit.out(), 1);

    bit.input(0, 1);
    assert_eq!(bit.out(), 0);

    bit.input(1, 0);
    assert_eq!(bit.out(), 0);
}

#[test]
fn register_test(){
    let mut register = Register::new();
    assert_eq!(register.out(), [0; 16]);

    register.input([1; 16], 1);
    assert_eq!(register.out(), [1; 16]);

    register.input([0; 16], 1);
    assert_eq!(register.out(), [0; 16]);

    register.input([1; 16], 0);
    assert_eq!(register.out(), [0; 16]);
}

#[test]
fn ram8_test(){
    let mut ram8 = RAM8::new();
    assert_eq!(ram8.out([0; 3]), [0; 16]);

    ram8.input([1; 16], 1, [0, 0, 1]);
    assert_eq!(ram8.out([0, 0, 1]), [1; 16]);
    
    ram8.input([0; 16], 1, [0, 0, 1]);
    assert_eq!(ram8.out([0, 0, 1]), [0; 16]);
}

#[test]
fn ram64s_test(){
    let mut ram64 = RAM64::new();
    assert_eq!(ram64.out([0; 6]), [0; 16]);

    ram64.input([1; 16], 1, [0, 0, 0, 0, 0, 1]);
    assert_eq!(ram64.out([0, 0, 0, 0, 0, 1]), [1; 16]);
    
    ram64.input([0; 16], 1, [0, 0, 0, 0, 0, 1]);
    assert_eq!(ram64.out([0, 0, 0, 0, 0, 1]), [0; 16]);
}