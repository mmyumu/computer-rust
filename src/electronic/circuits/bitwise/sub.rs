use std::iter::zip;

use crate::data::bits::Bits;
use crate::electronic::circuits::bitwise::Bitwise;
use crate::electronic::circuits::subtractor::FullSubtractor;

use super::BitwiseCheck;



pub struct BitwiseSub {
    bitwise: Bitwise,
    subtractors: Vec<FullSubtractor>
}

impl BitwiseCheck for BitwiseSub {
    fn size(&self) -> u8 {
        self.bitwise.size
    }
}

impl BitwiseSub {
    pub fn new(size: u8) -> Self {
        let subtractors = (0..size).map(|_| FullSubtractor::new()).collect::<Vec<FullSubtractor>>();
        BitwiseSub {
            bitwise: Bitwise::new(size),
            subtractors
        }
    }

    pub fn evaluate(&mut self, d1: &Bits, d2: &Bits, borrow: bool) -> (Bits, bool) {
        self.check_input(d1);
        self.check_input(d2);

        let mut borrow_in = borrow;
        let mut output = Vec::<bool>::new();
        let mut borrow_out= false;
        for (subtractor, (bit1, bit2)) in zip(self.subtractors.iter_mut(), zip(d1.iter().rev(), d2.iter().rev())) {
            let subtractor_result = subtractor.evaluate(*bit1, *bit2, borrow_in);
            borrow_in = subtractor_result.borrow_out;
            output.push(subtractor_result.difference);
            borrow_out = subtractor_result.borrow_out
        }
        output.reverse();

        (Bits::from_vector_b(output, None), borrow_out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitwise_sub_evaluate() {
        for d1 in 0..4 {
            for d2 in 0..4 {
                for borrow in [false, true] {
                    let mut bitwise_sub = BitwiseSub::new(4);
                    let data1 = Bits::from_int(d1, Some(4));
                    let data2 = Bits::from_int(d2, Some(4));
                    let (result, borrow_out) = bitwise_sub.evaluate(&data1, &data2, borrow);

                    let borrow_i32 = borrow as i32;
                    let d1_i32 = d1 as i32;
                    let d2_i32 = d2 as i32;
                    if d1_i32 < d2_i32 + borrow_i32 {
                        assert_eq!(result.to_int(), (16 + (d1_i32 - d2_i32 - borrow_i32)) as u32);
                        assert!(borrow_out)
                    } else {
                        assert_eq!(result.to_int(), (d1_i32 - d2_i32 - borrow_i32) as u32);
                        assert!(!borrow_out)
                    }
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn bitwise_sub_wrong_size_d1() {
        let mut bitwise_sub = BitwiseSub::new(4);
        let data1 = Bits::from_int(1, Some(3));
        let data2 = Bits::from_int(2, Some(4));

        bitwise_sub.evaluate(&data1, &data2, true);
    }

    #[test]
    #[should_panic]
    fn bitwise_sub_wrong_size_d2() {
        let mut bitwise_sub = BitwiseSub::new(4);
        let data1 = Bits::from_int(1, Some(4));
        let data2 = Bits::from_int(2, Some(3));

        bitwise_sub.evaluate(&data1, &data2, true);
    }
}