use crate::electronic::components::transistor::{NMOSTransistor, PMOSTransistor, Transistor};
use crate::electronic::components::voltage_levels::{GND, VDD};

pub struct Not {
    nmos: NMOSTransistor,
    pmos: PMOSTransistor,
}

impl Not {
    pub fn new() -> Self {
        Not {
            nmos: NMOSTransistor::new(),
            pmos: PMOSTransistor::new(),
        }
    }

    pub fn evaluate(&mut self, signal: bool) -> bool {
        self.nmos.apply_control_signal(signal);
        self.pmos.apply_control_signal(signal);

        self.nmos.connect_source(GND);
        self.pmos.connect_source(VDD);

        self.pmos.drain() && !self.nmos.drain()
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
