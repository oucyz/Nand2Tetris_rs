pub type Word16 = [u8; 16];
pub type Word8 = [u8; 8];

pub fn n_and(a: u8, b: u8) -> u8 {
    if a == 1 && b == 1{
        0
    } else {
        1
    }
}

// reference for flipflop: https://caddi.tech/archives/2099
pub struct Flipflop{bit: u8}

impl Flipflop{
    pub fn new() -> Self{
        Flipflop{bit: 0}
    }

    pub fn out(&self) -> u8{
        self.bit
    }
    
    pub fn input(&mut self, a: u8){
        self.bit = a;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_and(){
        assert_eq!(n_and(0, 0), 1);
        assert_eq!(n_and(0, 1), 1);
        assert_eq!(n_and(1, 0), 1);
        assert_eq!(n_and(1, 1), 0);
    }

    #[test]
    fn test_flipflop(){
        let mut flipflop = Flipflop::new();
        assert_eq!(flipflop.out(), 0);
        flipflop.input(1);
        assert_eq!(flipflop.out(), 1);
        flipflop.input(0);
        assert_eq!(flipflop.out(), 0);
    }
}