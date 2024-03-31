use crate::electronic::components::transistor::{Transistor, NMOSTransistor, PMOSTransistor};
use crate::electronic::circuits::logic_gates::gate::{GND, VDD};

use super::gate::{Gate, GateInput};

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
}

impl Gate for Not {
    fn evaluate(&mut self, input: super::gate::GateInput) -> bool {
        match input {
            GateInput::Single(_signal) => {
                self._nmos.apply_control_signal(_signal);
                self._pmos.apply_control_signal(_signal);

                self._nmos.connect_source(GND);
                self._pmos.connect_source(VDD);

                self._pmos.drain() && !self._nmos.drain()
            },
            _ => panic!("NotGate expects exactly one input signal."),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_evaluate_with_signal_true() {
        let mut not = Not::new();

        let input = GateInput::Single(true);

        let result = not.evaluate(input);
        assert!(!result);
    }

    #[test]
    fn not_evaluate_with_signal_false() {
        let mut not = Not::new();

        let input = GateInput::Single(false);

        let result = not.evaluate(input);
        assert!(result);
    }
}