use crate::given::{Flipflop, n_and, Word16, Word8};
use crate::logic::bool_logic::{and, and16, demux4way, demux8way, mux, mux16, mux4way16, mux8way16, not, not16, or, or16, xor};
use crate::adder::adder::{add16, increment16};

pub struct Bit{
    flipflop: Flipflop,
}
impl Bit{
    pub fn new() -> Self{
        Bit{flipflop: Flipflop::new()}
    }

    pub fn out(&self) -> u8{
        self.flipflop.out()
    }

    pub fn input(&mut self, input: u8, load: u8) -> (){
        let dff_in = mux(self.flipflop.out(), input, load);
        self.flipflop.input(dff_in);
    }
}

// Resister
pub struct Register{
    bits: [Bit; 16]
}
impl Register {
    pub fn new() -> Self {
        Register{bits: [
            Bit::new(), Bit::new(), Bit::new(), Bit::new(), 
            Bit::new(), Bit::new(), Bit::new(), Bit::new(), 
            Bit::new(), Bit::new(), Bit::new(), Bit::new(), 
            Bit::new(), Bit::new(), Bit::new(), Bit::new(), 
        ]}
    }

    pub fn out(&self) -> Word16 {
        [
            self.bits[0].out(), self.bits[1].out(), self.bits[2].out(), self.bits[3].out(), 
            self.bits[4].out(), self.bits[5].out(), self.bits[6].out(), self.bits[7].out(), 
            self.bits[8].out(), self.bits[9].out(), self.bits[10].out(), self.bits[11].out(), 
            self.bits[12].out(), self.bits[13].out(), self.bits[14].out(), self.bits[15].out()
        ]
    }
    
    pub fn input(&mut self, input: Word16, load: u8) -> (){
        let bit_in = mux16(self.out(), input, load);
        self.bits[0].input(bit_in[0], load);
        self.bits[1].input(bit_in[1], load);
        self.bits[2].input(bit_in[2], load);
        self.bits[3].input(bit_in[3], load);
        self.bits[4].input(bit_in[4], load);
        self.bits[5].input(bit_in[5], load);
        self.bits[6].input(bit_in[6], load);
        self.bits[7].input(bit_in[7], load);
        self.bits[8].input(bit_in[8], load);
        self.bits[9].input(bit_in[9], load);
        self.bits[10].input(bit_in[10], load);
        self.bits[11].input(bit_in[11], load);
        self.bits[12].input(bit_in[12], load);
        self.bits[13].input(bit_in[13], load);
        self.bits[14].input(bit_in[14], load);
        self.bits[15].input(bit_in[15], load);
    }
}


pub struct RAM8{
    registers: [Register; 8]
}

impl RAM8 {
    pub fn new() -> Self {
        RAM8{
            registers: [
                Register::new(), Register::new(), Register::new(), Register::new(), 
                Register::new(), Register::new(), Register::new(), Register::new(), 
            ]
        }
    }

    pub fn out(&self, address: [u8; 3]) -> Word16{
        mux8way16(
            [
                self.registers[0].out(), self.registers[1].out(), self.registers[2].out(), self.registers[3].out(),
                self.registers[4].out(), self.registers[5].out(), self.registers[6].out(), self.registers[7].out()
            ],
            address)
    }

    pub fn input(&mut self, input: Word16, load: u8, address: [u8; 3]) -> () {
        let loads = demux8way(load, address);
        self.registers[0].input(input, loads[0]);
        self.registers[1].input(input, loads[1]);
        self.registers[2].input(input, loads[2]);
        self.registers[3].input(input, loads[3]);
        self.registers[4].input(input, loads[4]);
        self.registers[5].input(input, loads[5]);
        self.registers[6].input(input, loads[6]);
        self.registers[7].input(input, loads[7]);
    }
}

pub struct RAM64{
    ram8s: [RAM8; 8]
}
impl RAM64 {
    pub fn new() -> Self{
        RAM64 { ram8s: [
            RAM8::new(), RAM8::new(), RAM8::new(), RAM8::new(), 
            RAM8::new(), RAM8::new(), RAM8::new(), RAM8::new()
        ]}
    }

    pub fn out(&self, address: [u8; 6]) -> Word16 {
        mux8way16([
            self.ram8s[0].out([address[0], address[1], address[2]]),
            self.ram8s[1].out([address[0], address[1], address[2]]),
            self.ram8s[2].out([address[0], address[1], address[2]]),
            self.ram8s[3].out([address[0], address[1], address[2]]),
            self.ram8s[4].out([address[0], address[1], address[2]]),
            self.ram8s[5].out([address[0], address[1], address[2]]),
            self.ram8s[6].out([address[0], address[1], address[2]]),
            self.ram8s[7].out([address[0], address[1], address[2]]),
        ],
        [address[3], address[4], address[5]])
    }

    pub fn input(&mut self, input: Word16, load: u8, address: [u8; 6]) -> () {
        let loads = demux8way(load, [address[3], address[4], address[5]]);
        self.ram8s[0].input(input, loads[0], [address[0], address[1], address[2]]);
        self.ram8s[1].input(input, loads[1], [address[0], address[1], address[2]]);
        self.ram8s[2].input(input, loads[2], [address[0], address[1], address[2]]);
        self.ram8s[3].input(input, loads[3], [address[0], address[1], address[2]]);
        self.ram8s[4].input(input, loads[4], [address[0], address[1], address[2]]);
        self.ram8s[5].input(input, loads[5], [address[0], address[1], address[2]]);
        self.ram8s[6].input(input, loads[6], [address[0], address[1], address[2]]);
        self.ram8s[7].input(input, loads[7], [address[0], address[1], address[2]]);
    }
}

pub struct RAM512{
    ram64s: [RAM64; 8]
}
impl RAM512{
    pub fn new() -> Self{
        RAM512 { ram64s: [
            RAM64::new(), RAM64::new(), RAM64::new(), RAM64::new(), 
            RAM64::new(), RAM64::new(), RAM64::new(), RAM64::new()
        ]}
    }

    pub fn out(&self, address: [u8; 9]) -> Word16 {
        let _address = [address[0], address[1], address[2], address[3], address[4], address[5]];
        mux8way16([
            self.ram64s[0].out(_address),
            self.ram64s[1].out(_address),
            self.ram64s[2].out(_address),
            self.ram64s[3].out(_address),
            self.ram64s[4].out(_address),
            self.ram64s[5].out(_address),
            self.ram64s[6].out(_address),
            self.ram64s[7].out(_address),
        ],
        [address[6], address[7], address[8]])
    }

    pub fn input(&mut self, input: Word16, load: u8, address: [u8; 9]) -> () {
        let _address = [address[0], address[1], address[2], address[3], address[4], address[5]];
        let loads = demux8way(load, [address[6], address[7], address[8]]);
        self.ram64s[0].input(input, loads[0], _address);
        self.ram64s[1].input(input, loads[1], _address);
        self.ram64s[2].input(input, loads[2], _address);
        self.ram64s[3].input(input, loads[3], _address);
        self.ram64s[4].input(input, loads[4], _address);
        self.ram64s[5].input(input, loads[5], _address);
        self.ram64s[6].input(input, loads[6], _address);
        self.ram64s[7].input(input, loads[7], _address);
    }
}

pub struct RAM4K{
    ram512s: [RAM512; 8]
}
impl RAM4K {
    pub fn new() -> Self{
        RAM4K{
            ram512s: [
                RAM512::new(), RAM512::new(), RAM512::new(), RAM512::new(), 
                RAM512::new(), RAM512::new(), RAM512::new(), RAM512::new()
            ]
        }
    }

    pub fn out(&self, address: [u8; 12]) -> Word16 {
        let _address = [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]];
        mux8way16([
            self.ram512s[0].out(_address),
            self.ram512s[1].out(_address),
            self.ram512s[2].out(_address),
            self.ram512s[3].out(_address),
            self.ram512s[4].out(_address),
            self.ram512s[5].out(_address),
            self.ram512s[6].out(_address),
            self.ram512s[7].out(_address),
        ],
        [address[9], address[10], address[11]])
    }

    pub fn input(&mut self, input: Word16, load: u8, address: [u8; 12]) -> () {
        let _address = [address[0], address[1], address[2], address[3], address[4], address[5], address[6], address[7], address[8]];
        let loads = demux8way(load, [address[9], address[10], address[11]]);
        self.ram512s[0].input(input, loads[0], _address);
        self.ram512s[1].input(input, loads[1], _address);
        self.ram512s[2].input(input, loads[2], _address);
        self.ram512s[3].input(input, loads[3], _address);
        self.ram512s[4].input(input, loads[4], _address);
        self.ram512s[5].input(input, loads[5], _address);
        self.ram512s[6].input(input, loads[6], _address);
        self.ram512s[7].input(input, loads[7], _address);
    }
}

pub struct RAM16K{
    ram4ks: [RAM4K; 4]
}
impl RAM16K {
    pub fn new() -> Self{
        RAM16K{
            ram4ks: [
                RAM4K::new(), RAM4K::new(), RAM4K::new(), RAM4K::new()
            ]
        }
    }
    pub fn out(&self, address: [u8; 14]) -> Word16 {
        let _address = [
            address[0], address[1], address[2], address[3],
            address[4], address[5], address[6], address[7],
            address[8], address[9], address[10], address[11]
        ];
        mux4way16([
            self.ram4ks[0].out(_address),
            self.ram4ks[1].out(_address),
            self.ram4ks[2].out(_address),
            self.ram4ks[3].out(_address),
        ],
        [address[12], address[13]])
    }
    pub fn input(&mut self, input: Word16, load: u8, address: [u8; 14]) -> () {
        let _address = [
            address[0], address[1], address[2], address[3],
            address[4], address[5], address[6], address[7],
            address[8], address[9], address[10], address[11]
        ];
        let loads = demux4way(load, [address[12], address[13]]);
        self.ram4ks[0].input(input, loads[0], _address);
        self.ram4ks[1].input(input, loads[1], _address);
        self.ram4ks[2].input(input, loads[2], _address);
        self.ram4ks[3].input(input, loads[3], _address);
    }
}


// PC, Program Counter
pub struct PC{
    register: Register
}
impl PC{
    pub fn new() -> Self{
        PC{register: Register::new()}
    }
    pub fn input(&mut self, input: Word16, reset: u8, load: u8, inc: u8) -> (){
        let _input = mux16(
            mux16(
                mux16(
                    self.register.out(),
                    increment16(self.register.out()),
                    inc
                ),
                input,
                load
            ), 
            [0; 16],
            reset
        );
        self.register.input(_input, 1);
    }
    pub fn out(&self) -> Word16{
        self.register.out()
    }
}