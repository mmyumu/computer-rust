use std::iter::zip;

use crate::data::bits::Bits;
use crate::electronic::circuits::bitwise::add::BitwiseAdd;
use crate::electronic::circuits::bitwise::Bitwise;
use crate::electronic::circuits::logic_gates::and::And;

use super::BitwiseCheck;

pub struct BitwiseMult {
    bitwise: Bitwise,
    ands: Vec<Vec<And>>,
    bitwise_adds: Vec<BitwiseAdd>,
}

impl BitwiseCheck for BitwiseMult {
    fn size(&self) -> u8 {
        self.bitwise.size
    }
}

impl BitwiseMult {
    pub fn new(size: u8) -> Self {
        let mut ands = Vec::<Vec<And>>::new();
        for _ in 0..size {
            let mut ands_row = Vec::<And>::new();
            for _ in 0..size {
                ands_row.push(And::new());
            }
            ands.push(ands_row);
        }

        let bitwise_adds = (0..size - 1)
            .map(|_| BitwiseAdd::new(size * 2))
            .collect::<Vec<BitwiseAdd>>();
        BitwiseMult {
            bitwise: Bitwise::new(size),
            ands,
            bitwise_adds,
        }
    }

    pub fn evaluate(&mut self, d1: &Bits, d2: &Bits) -> Bits {
        self.check_input(d1);
        self.check_input(d2);

        // let mut output = Vec::<bool>::new();
        let mut output: Bits = Bits::from_vector_b(Vec::new(), None);
        for (i, (bit2, ands_row)) in zip(d2.iter().rev(), self.ands.iter_mut()).enumerate() {
            let mut and_result = (0..d2.len() - i).map(|_| false).collect::<Vec<bool>>();
            for (bit1, and) in zip(d1.iter(), ands_row) {
                let and_bit = and.evaluate(*bit1, *bit2);
                and_result.push(and_bit);
            }

            and_result.resize(d1.len() + d2.len(), false);

            if output.len() == 0 {
                output = Bits::from_vector_b(and_result, None);
            } else {
                let (aggregated_result, _) = self.bitwise_adds[i - 1].evaluate(
                    &Bits::from_vector_b(and_result, None),
                    &output,
                    false,
                );
                output = aggregated_result;
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitwise_mult_evaluate() {
        for d1 in 0..4 {
            for d2 in 0..4 {
                let mut bitwise_mult = BitwiseMult::new(4);
                let data1 = Bits::from_int(d1, Some(4));
                let data2 = Bits::from_int(d2, Some(4));
                let result = bitwise_mult.evaluate(&data1, &data2);
                assert_eq!(result.to_int(), d1 * d2);
            }
        }
    }
}
