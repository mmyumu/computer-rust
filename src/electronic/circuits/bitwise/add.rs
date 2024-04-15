use std::iter::zip;

use crate::data::bits::Bits;
use crate::electronic::circuits::bitwise::Bitwise;
use crate::electronic::circuits::adder::FullAdder;

pub struct BitwiseAdd {
    bitwise: Bitwise,
    adders: Vec<FullAdder>
}

impl BitwiseAdd {
    pub fn new(size: u8) -> Self {
        let adders = (0..size).map(|_| FullAdder::new()).collect::<Vec<FullAdder>>();
        BitwiseAdd {
            bitwise: Bitwise::new(size),
            adders
        }
    }

    pub fn evaluate(&mut self, d1: &Bits, d2: &Bits, carry: bool) -> (Bits, bool) {
        self.bitwise.check_input(d1);
        self.bitwise.check_input(d2);

        let mut carry_in = carry;
        let mut output = Vec::<bool>::new();
        let mut carry_out= false;
        for (adder, (bit1, bit2)) in zip(self.adders.iter_mut(), zip(d1.iter().rev(), d2.iter().rev())) {
            let adder_result = adder.evaluate(*bit1, *bit2, carry_in);
            carry_in = adder_result.carry_out;
            output.push(adder_result.sum);
            carry_out = adder_result.carry_out
        }
        output.reverse();

        (Bits::from_vector_b(output, None), carry_out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitwise_add_evaluate() {
        for d1 in 0..4 {
            for d2 in 0..4 {
                for carry in [false, true] {
                    let mut bitwise_add = BitwiseAdd::new(4);
                    let data1 = Bits::from_int(d1, Some(4));
                    let data2 = Bits::from_int(d2, Some(4));
    
                    let (result, carry_out) = bitwise_add.evaluate(&data1, &data2, carry);

                    assert!(result.to_int() == d1 + d2 + (carry as u32));
                    assert!(!carry_out)
                }
            }
        }
    }
}