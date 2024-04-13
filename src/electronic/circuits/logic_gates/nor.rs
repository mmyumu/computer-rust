use crate::electronic::components::transistor::{Transistor, NMOSTransistor, PMOSTransistor};
use crate::electronic::components::voltage_levels::{GND, VDD};

pub struct Nor {
    _nmos_a: NMOSTransistor,
    _nmos_b: NMOSTransistor,
    _pmos_a: PMOSTransistor,
    _pmos_b: PMOSTransistor
}

impl Nor {
    pub fn new() -> Self {
        Nor {
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

        self._nmos_a.connect_source(GND);
        self._nmos_b.connect_source(GND);

        self._pmos_a.connect_source(VDD);
        self._pmos_b.connect_source(self._pmos_a.drain());

        self._pmos_b.drain() && !(self._nmos_a.drain() || self._nmos_b.drain())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nor_evaluate_with_signal_a_false_signal_b_false() {
        let mut nor = Nor::new();
        let result = nor.evaluate(false, false);
        assert!(result);
    }

    #[test]
    fn nor_evaluate_with_signal_a_false_signal_b_true() {
        let mut nor = Nor::new();
        let result = nor.evaluate(false, true);
        assert!(!result);
    }

    #[test]
    fn nor_evaluate_with_signal_a_true_signal_b_false() {
        let mut nor = Nor::new();
        let result = nor.evaluate(true, false);
        assert!(!result);
    }

    #[test]
    fn nor_evaluate_with_signal_a_true_signal_b_true() {
        let mut nor = Nor::new();
        let result = nor.evaluate(true, true);
        assert!(!result);
    }
}