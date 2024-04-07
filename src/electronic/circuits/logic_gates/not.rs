use crate::electronic::components::transistor::{Transistor, NMOSTransistor, PMOSTransistor};
use crate::electronic::components::voltage_levels::{GND, VDD};

pub struct Not {
    _nmos: NMOSTransistor,
    _pmos: PMOSTransistor
}

impl Not {
    pub fn new() -> Self {
        Not{
            _nmos: NMOSTransistor::new(),
            _pmos: PMOSTransistor::new()
        }
    }

    pub fn evaluate(&mut self, _signal: bool) -> bool {
        self._nmos.apply_control_signal(_signal);
        self._pmos.apply_control_signal(_signal);

        self._nmos.connect_source(GND);
        self._pmos.connect_source(VDD);

        self._pmos.drain() && !self._nmos.drain()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_evaluate_with_signal_true() {
        let mut not = Not::new();
        let result = not.evaluate(true);
        assert!(!result);
    }

    #[test]
    fn not_evaluate_with_signal_false() {
        let mut not = Not::new();
        let result = not.evaluate(false);
        assert!(result);
    }
}