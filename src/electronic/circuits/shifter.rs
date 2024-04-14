use crate::data::bits::Bits;
use crate::electronic::circuits::mux::Mux2To1;

pub struct BarrelShifter {
    size: u8,
    right: bool,
    muxes: Vec<Vec<Mux2To1>>
}

impl BarrelShifter {
    pub fn new(size: u8, right: bool) -> Self {
        let mut muxes = Vec::<Vec<Mux2To1>>::new();
        for _ in 0..size {
            let mut row_muxes = Vec::<Mux2To1>::new();
            for _ in 0..2_u8.pow(size as u32) {
                row_muxes.push(Mux2To1::new());
                
            }
            muxes.push(row_muxes);
        }

        BarrelShifter {
            size,
            right,
            muxes
        }
    }

    pub fn evaluate(&mut self, i: &Bits, s: &Bits) -> (Bits, bool) {
        let i_length = 2_u8.pow(self.size as u32);
        if i.len() as u8 != i_length {
            panic!("Length of i should be {} but is {}", i_length, i.len());
        }

        if s.len() as u8 != self.size {
            panic!("Length of s should be {} but is {}", self.size, s.len());
        }

        let mut current_i = i.data();
        if !self.right {
            current_i.reverse();
        }

        for (index_s, bit_s) in s.iter().rev().enumerate() {
            let mut output = Vec::<bool>::new();
            let number_of_zeros = 2_u8.pow(index_s as u32);
            for (index_i, bit_i) in current_i.iter().enumerate() {
                let a1 = if (index_i as u8) < number_of_zeros {
                    current_i[(i_length - number_of_zeros) as usize + index_i]
                } else {
                    current_i[index_i - (number_of_zeros as usize)]
                };

                let a0 = bit_i;
                let mux_result = self.muxes[index_s][index_i].evaluate(*a0, a1, *bit_s);

                output.push(mux_result);
            }
            current_i = output.clone();
        }

        if self.right {
            let result = Bits::from_slice_b(&current_i, None);
            let first_bit = result[0];
            (result, first_bit)
        } else {
            current_i.reverse();
            let result = Bits::from_slice_b(&current_i, None);
            let first_bit = result[result.len() - 1];
            (result, first_bit)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn barrel_shifter_evaluate() {
        let mut barrel_shiter_right = BarrelShifter::new(3, true);
        let mut barrel_shiter_left = BarrelShifter::new(3, false);

        for index_i in 0..256 {
            let i = Bits::from_int(index_i, Some(8));

            for index_s in 0..8 {
                let s = Bits::from_int(index_s, Some(3));

                let (result_right, _) = barrel_shiter_right.evaluate(&i, &s);
                assert!(((i.to_int() >> s.to_int()) | (i.to_int() << (8 - s.to_int())) & 0xFF) % (2u32.pow(8)) == result_right.to_int());

                let (result_left, _) = barrel_shiter_left.evaluate(&i, &s);
                assert!(((i.to_int() << s.to_int()) | (i.to_int() >> (8 - s.to_int())) & 0xFF) % (2u32.pow(8)) == result_left.to_int());
            }

        }
    }

    #[test]
    #[should_panic]
    fn barrel_shifter_wrong_s_size() {
        let mut barrel_shiter = BarrelShifter::new(3, true);
        let i = Bits::from_int(10, Some(8));
        let s = Bits::from_int(2, Some(4));

        barrel_shiter.evaluate(&i, &s);
    }

    #[test]
    #[should_panic]
    fn barrel_shifter_wrong_i_size() {
        let mut barrel_shiter = BarrelShifter::new(3, true);
        let i = Bits::from_int(10, Some(9));
        let s = Bits::from_int(2, Some(3));

        barrel_shiter.evaluate(&i, &s);
    }
}