use rand::random;
use crate::data::bits::Bits;
use crate::electronic::circuits::flip_flop::DFlipFlop;

pub struct PIPORegister {
    _size: u8,
    _d_flip_flops: Vec<DFlipFlop>,
    _ds: Vec<bool>
}

impl PIPORegister {
    pub fn new(size: u8) -> Self {
        let _d_flip_flops = (0..size).map(|_| DFlipFlop::new()).collect::<Vec<DFlipFlop>>();
        let _ds = (0..size).map(|_| random()).collect::<Vec<bool>>();
        PIPORegister {
            _size: size,
            _d_flip_flops,
            _ds
        }
    }

    pub fn output(&mut self) -> Bits {
        Bits::from_vector_b(self._d_flip_flops.iter().rev().map(|d_flip_flop| d_flip_flop.q).collect::<Vec<bool>>(), None)
    }

    pub fn reset_states(&mut self) {
        for d_flip_flop in self._d_flip_flops.iter_mut() {
            d_flip_flop.reset_states();
        }
        self._ds = (0..self._size).map(|_| false).collect::<Vec<bool>>();
    }

    pub fn set_d(&mut self, ds: &[bool]) {
        if ds.len() != self._size as usize {
            panic!("Input length should be {} but is {}", self._size, ds.len());
        }
        self._ds = ds.to_vec();
        self._ds.reverse();
    }

    pub fn clock_tick(&mut self, enable: bool) -> Bits {
        for (d_flip_flop, ds) in self._d_flip_flops.iter_mut().zip(&self._ds) {
            d_flip_flop.set_d(*ds);
            d_flip_flop.clock_tick(enable);
        }
        self.output()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipo_set1011() {
        let mut pipo_register = PIPORegister::new(4);
        pipo_register.reset_states();
        pipo_register.set_d(&[true, false, true, true]);
        assert!(pipo_register.clock_tick(true) == [true, false, true, true]);
    }

    #[test]
    fn pipo_set1111() {
        let mut pipo_register = PIPORegister::new(4);
        pipo_register.reset_states();
        pipo_register.set_d(&[true, true, true, true]);
        assert!(pipo_register.clock_tick(true) == &[true, true, true, true]);
    }

    #[test]
    fn tpipo_set1110() {
        let mut pipo_register = PIPORegister::new(4);
        pipo_register.reset_states();
        pipo_register.set_d(&[true, true, true, false]);
        assert!(pipo_register.clock_tick(true) == &[true, true, true, false]);
    }

    #[test]
    fn pipo_set1110_memorize() {
        let mut pipo_register = PIPORegister::new(4);
        pipo_register.reset_states();
        pipo_register.set_d(&[true, true, true, false]);
        assert!(pipo_register.clock_tick(true) == &[true, true, true, false]);

        for _ in 0..10 {
            assert!(Bits::from_slice_b(&[true, true, true, false], None) == &[true, true, true, false]);
        }
    }

    #[test]
    #[should_panic]
    fn pipo_wrong_input_size() {
        let mut pipo_register = PIPORegister::new(4);
        pipo_register.set_d(&[true, true, true, false, true]);
    }
}