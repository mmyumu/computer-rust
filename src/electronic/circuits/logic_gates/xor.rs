use crate::electronic::components::transistor::{Transistor, NMOSTransistor, PMOSTransistor};
use crate::electronic::circuits::logic_gates::gate::{Gate, GateInput};

use super::gate::{GND, VDD};

pub struct Xor {
    _pmos_a: PMOSTransistor,
    _pmos_a_bar: PMOSTransistor,
    _pmos_b: PMOSTransistor,
    _pmos_b_bar: PMOSTransistor,
    _nmos_a: NMOSTransistor,
    _nmos_a_bar: NMOSTransistor,
    _nmos_b: NMOSTransistor,
    _nmos_b_bar: NMOSTransistor
}

impl Xor {
    pub fn new() -> Self {
        Xor {
            _pmos_a: PMOSTransistor::new(),
            _pmos_a_bar: PMOSTransistor::new(),
            _pmos_b: PMOSTransistor::new(),
            _pmos_b_bar: PMOSTransistor::new(),
            _nmos_a: NMOSTransistor::new(),
            _nmos_a_bar: NMOSTransistor::new(),
            _nmos_b: NMOSTransistor::new(),
            _nmos_b_bar: NMOSTransistor::new(),
        }
    }
}

impl Gate for Xor {
    fn evaluate(&mut self, input: GateInput) -> bool {
        match input {
            GateInput::Dual(_signal_a, _signal_b) => {
                self._pmos_a.apply_control_signal(_signal_a);
                self._pmos_a_bar.apply_control_signal(!_signal_a);
                self._pmos_b_bar.apply_control_signal(!_signal_b);
                self._pmos_b.apply_control_signal(_signal_b);
        
                self._nmos_a.apply_control_signal(_signal_a);
                self._nmos_a_bar.apply_control_signal(!_signal_a);
                self._nmos_b.apply_control_signal(_signal_b);
                self._nmos_b_bar.apply_control_signal(!_signal_b);
        
                self._pmos_a.connect_source(VDD);
                self._pmos_a_bar.connect_source(VDD);
                self._nmos_b.connect_source(GND);
                self._nmos_b_bar.connect_source(GND);
        
                self._pmos_b_bar.connect_source(self._pmos_a.drain());
                self._pmos_b.connect_source(self._pmos_a_bar.drain());
                self._nmos_a.connect_source(self._nmos_b.drain());
                self._nmos_a_bar.connect_source(self._nmos_b_bar.drain());
        
                (self._pmos_b.drain() || self._pmos_b_bar.drain()) && (!self._nmos_a_bar.drain() || !self._nmos_a.drain())
            },
            _ => panic!("Xor gate expects exactly two input signal."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_evaluate_with_signal_a_false_signal_b_false() {
        let mut xor = Xor::new();

        let input = GateInput::Dual(false, false);
        let result = xor.evaluate(input);
        assert!(!result);
    }

    #[test]
    fn xor_evaluate_with_signal_a_false_signal_b_true() {
        let mut xor = Xor::new();

        let input = GateInput::Dual(false, true);
        let result = xor.evaluate(input);
        assert!(result);
    }

    #[test]
    fn xor_evaluate_with_signal_a_true_signal_b_false() {
        let mut xor = Xor::new();

        let input = GateInput::Dual(true, false);
        let result = xor.evaluate(input);
        assert!(result);
    }

    #[test]
    fn xor_evaluate_with_signal_a_true_signal_b_true() {
        let mut xor = Xor::new();

        let input = GateInput::Dual(true, true);
        let result = xor.evaluate(input);
        assert!(!result);
    }
}