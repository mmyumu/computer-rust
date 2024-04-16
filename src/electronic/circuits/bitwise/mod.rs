pub mod add;
pub mod sub;

use crate::data::bits::Bits;

pub struct Bitwise {
    size: u8
}

impl Bitwise {
    pub fn new(size: u8) -> Self {
        Bitwise {
            size
        }
    }
    // pub fn check_input(&mut self, i: &Bits) {
    //     if i.len() != self.size as usize {
    //         panic!("Length of {} should be {} but is {}", stringify!(i), self.size, i.len());
    //     }
    // }
}

pub trait BitwiseCheck {
    fn size(&self) -> u8;

    fn check_input(&mut self, i: &Bits) {
        if i.len() != self.size() as usize {
            panic!("Length of {} should be {} but is {}", stringify!(i), self.size(), i.len());
        }
    }
}