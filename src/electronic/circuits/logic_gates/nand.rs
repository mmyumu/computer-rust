use crate::electronic::components::transistor::{Transistor, NMOSTransistor, PMOSTransistor};
use crate::electronic::components::voltage_levels::{GND, VDD};

pub struct Nand {
    _nmos_a: NMOSTransistor,
    _nmos_b: NMOSTransistor,
    _pmos_a: PMOSTransistor,
    _pmos_b: PMOSTransistor
}

impl Nand {
    pub fn new() -> Self {
        Nand{
            _nmos_a: NMOSTransistor::new(),
            _nmos_b: NMOSTransistor::new(),
            _pmos_a: PMOSTransistor::new(),
            _pmos_b: PMOSTransistor::new()
        }
    }

    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool) -> bool {
        self._nmos_a.apply_control_signal(signal_a);
        self._pmos_a.apply_control_signal(signal_a);
        self._nmos_b.apply_control_signal(signal_b);
        self._pmos_b.apply_control_signal(signal_b);

        self._pmos_a.connect_source(VDD);
        self._pmos_b.connect_source(VDD);

        self._nmos_b.connect_source(GND);
        self._nmos_a.connect_source(self._nmos_b.drain());

        (self._pmos_a.drain() || self._pmos_b.drain()) && !self._nmos_a.drain()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nand_evaluate_with_signal_a_false_signal_b_false() {
        let mut nand = Nand::new();
        let result = nand.evaluate(false, false);
        assert!(result);
    }

    #[test]
    fn nand_evaluate_with_signal_a_false_signal_b_true() {
        let mut nand = Nand::new();
        let result = nand.evaluate(false, true);
        assert!(result);
    }

    #[test]
    fn nand_evaluate_with_signal_a_true_signal_b_false() {
        let mut nand = Nand::new();
        let result = nand.evaluate(true, false);
        assert!(result);
    }

    #[test]
    fn nand_evaluate_with_signal_a_true_signal_b_true() {
        let mut nand = Nand::new();
        let result = nand.evaluate(true, true);
        assert!(!result);
    }
}