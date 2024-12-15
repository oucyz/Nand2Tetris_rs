pub type Word16 = [u8; 16];
pub type Word8 = [u8; 8];

pub fn n_and(a: u8, b: u8) -> u8 {
    if a == 1 && b == 1{
        0
    } else {
        1
    }
}
