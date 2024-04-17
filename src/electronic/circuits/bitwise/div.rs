use std::iter::zip;

use crate::data::bits::Bits;
use crate::electronic::circuits::bitwise::Bitwise;
use crate::electronic::circuits::logic_gates::or::Or;
use crate::electronic::circuits::logic_gates::not::Not;
use crate::electronic::circuits::subtractor::FullSubtractorRestore;

use super::BitwiseCheck;

pub struct BitwiseDiv {
    bitwise: Bitwise,
    ors: Vec<Or>,
    nots: Vec<Not>,
    subrestores: Vec<Vec<FullSubtractorRestore>>
}

impl BitwiseCheck for BitwiseDiv {
    fn size(&self) -> u8 {
        self.bitwise.size
    }
}

impl BitwiseDiv {
    pub fn new(size: u8) -> Self {
        let mut ors = Vec::new();
        let mut nots = Vec::new();
        let mut subrestores = Vec::new();
        for _ in 0..size {
            ors.push(Or::new());
            nots.push(Not::new());
            let mut subrestores_row = Vec::<FullSubtractorRestore>::new();
            for _ in 0..size {
                subrestores_row.push(FullSubtractorRestore::new())                
            }
            subrestores.push(subrestores_row);
        }

        BitwiseDiv {
            bitwise: Bitwise::new(size),
            ors,
            nots,
            subrestores
        }
    }

    pub fn evaluate(&mut self, a: &Bits, d: &Bits) -> (Bits, Bits)  {
        self.check_input(a);
        self.check_input(d);

        if d.iter().all(|&b| !b) {
            panic!("Divider cannot be 0")
        }

        let mut filled_a = (0..a.len()).map(|_| false).collect::<Vec<bool>>();
        filled_a.extend(a.data());

        let mut quotient = Vec::<bool>::new();
        let mut last_remainder_row = Vec::<bool>::new();
        for i in 0..self.size() as usize {
            let (bit_or, row_a) = if i == 0 {
                (filled_a[0], filled_a[i + 1..i+ 1 + self.size() as usize].to_vec())
            } else {
                let r = &last_remainder_row[1..last_remainder_row.len()];
                let row_a: Vec<bool> = [r, &[a[i]]].concat();
                (last_remainder_row[0], row_a)
            };

            let (quotient_row, remainder_row) = self._row(i, row_a, d, bit_or);
            last_remainder_row = remainder_row;
            quotient.push(quotient_row);
        }
        (Bits::from_vector_b(quotient, None), Bits::from_vector_b(last_remainder_row, None))
    }

    fn _row(&mut self, row: usize, a: Vec<bool>, d: &Bits, bit_or: bool) -> (bool, Vec<bool>) {
        if a.len() != self.size() as usize {
            panic!("Length of input a should be {} but is {}", self.size(), a.len())
        }

        let mut borrow_in = false;
        let mut carry_in = true;
        let mut last_borrow_out = false;

        for (subrestore, (bit_a, bit_d)) in zip(self.subrestores[row].iter_mut(), zip(a.iter().rev(), d.iter().rev())) {
            let (_, borrow_out) = subrestore.evaluate(*bit_a, *bit_d, borrow_in, carry_in);
            borrow_in = borrow_out;
            last_borrow_out = borrow_out;
        }

        let not_borrow_out = self.nots[row].evaluate(last_borrow_out);
        let quotient = self.ors[row].evaluate(bit_or, not_borrow_out);

        let mut remainder = Vec::<bool>::new();
        carry_in = quotient;

        for (subrestore, (bit_a, bit_d)) in zip(self.subrestores[row].iter_mut(), zip(a.iter().rev(), d.iter().rev())) {
            let (subrestore_result, borrow_out) = subrestore.evaluate(*bit_a, *bit_d, borrow_in, carry_in);
            remainder.push(subrestore_result);
            borrow_in = borrow_out;
        }

        remainder.reverse();
        (quotient, remainder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitwise_div_evaluate() {
        for d1 in 0..4 {
            for d2 in 0..4 {
                let data = Bits::from_int(d1, Some(4));
                let divider = Bits::from_int(d2, Some(4));

                if d2 != 0 {
                    let mut bitwise_div = BitwiseDiv::new(4);
                    let (quotient, remainder) = bitwise_div.evaluate(&data,& divider);

                    assert_eq!(quotient.to_int(), d1 / d2);
                    assert_eq!(remainder.to_int(), d1 % d2);
                }
            }
        }
    }
}