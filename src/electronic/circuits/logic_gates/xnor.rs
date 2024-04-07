use crate::electronic::components::transistor::{Transistor, NMOSTransistor, PMOSTransistor};
use crate::electronic::components::voltage_levels::{GND, VDD};

pub struct Xnor {
    _pmos_a: PMOSTransistor,
    _pmos_a_bar: PMOSTransistor,
    _pmos_b: PMOSTransistor,
    _pmos_b_bar: PMOSTransistor,
    _nmos_a: NMOSTransistor,
    _nmos_a_bar: NMOSTransistor,
    _nmos_b: NMOSTransistor,
    _nmos_b_bar: NMOSTransistor
}

impl Xnor {
    pub fn new() -> Self {
        Xnor {
            _pmos_a: PMOSTransistor::new(),
            _pmos_a_bar: PMOSTransistor::new(),
            _pmos_b: PMOSTransistor::new(),
            _pmos_b_bar: PMOSTransistor::new(),
            _nmos_a: NMOSTransistor::new(),
            _nmos_a_bar: NMOSTransistor::new(),
            _nmos_b: NMOSTransistor::new(),
            _nmos_b_bar: NMOSTransistor::new()
        }
    }

    pub fn evaluate(&mut self, _signal_a: bool, _signal_b: bool) -> bool {
        self._pmos_a.apply_control_signal(_signal_a);
        self._pmos_a_bar.apply_control_signal(!_signal_a);
        self._pmos_b.apply_control_signal(_signal_b);
        self._pmos_b_bar.apply_control_signal(!_signal_b);

        self._nmos_a.apply_control_signal(_signal_a);
        self._nmos_a_bar.apply_control_signal(!_signal_a);
        self._nmos_b_bar.apply_control_signal(!_signal_b);
        self._nmos_b.apply_control_signal(_signal_b);

        self._pmos_a.connect_source(VDD);
        self._pmos_a_bar.connect_source(VDD);
        self._nmos_b_bar.connect_source(GND);
        self._nmos_b.connect_source(GND);

        self._pmos_b.connect_source(self._pmos_a.drain());
        self._pmos_b_bar.connect_source(self._pmos_a_bar.drain());
        self._nmos_a.connect_source(self._nmos_b_bar.drain());
        self._nmos_a_bar.connect_source(self._nmos_b.drain());

        (self._pmos_b.drain() || self._pmos_b_bar.drain()) && (!self._nmos_a_bar.drain() || !self._nmos_a.drain())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xnor_evaluate_with_signal_a_false_signal_b_false() {
        let mut xnor = Xnor::new();
        let result = xnor.evaluate(false, false);
        assert!(result);
    }

    #[test]
    fn xnor_evaluate_with_signal_a_false_signal_b_true() {
        let mut xnor = Xnor::new();
        let result = xnor.evaluate(false, true);
        assert!(!result);
    }

    #[test]
    fn xnor_evaluate_with_signal_a_true_signal_b_false() {
        let mut xnor = Xnor::new();
        let result = xnor.evaluate(true, false);
        assert!(!result);
    }

    #[test]
    fn xnor_evaluate_with_signal_a_true_signal_b_true() {
        let mut xnor = Xnor::new();
        let result = xnor.evaluate(true, true);
        assert!(result);
    }
}