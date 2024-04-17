use crate::electronic::components::transistor::{NMOSTransistor, PMOSTransistor, Transistor};
use crate::electronic::components::voltage_levels::{GND, VDD};

pub struct Xor {
    pmos_a: PMOSTransistor,
    pmos_a_bar: PMOSTransistor,
    pmos_b: PMOSTransistor,
    pmos_b_bar: PMOSTransistor,
    nmos_a: NMOSTransistor,
    nmos_a_bar: NMOSTransistor,
    nmos_b: NMOSTransistor,
    nmos_b_bar: NMOSTransistor,
}

impl Xor {
    pub fn new() -> Self {
        Xor {
            pmos_a: PMOSTransistor::new(),
            pmos_a_bar: PMOSTransistor::new(),
            pmos_b: PMOSTransistor::new(),
            pmos_b_bar: PMOSTransistor::new(),
            nmos_a: NMOSTransistor::new(),
            nmos_a_bar: NMOSTransistor::new(),
            nmos_b: NMOSTransistor::new(),
            nmos_b_bar: NMOSTransistor::new(),
        }
    }

    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool) -> bool {
        self.pmos_a.apply_control_signal(signal_a);
        self.pmos_a_bar.apply_control_signal(!signal_a);
        self.pmos_b_bar.apply_control_signal(!signal_b);
        self.pmos_b.apply_control_signal(signal_b);

        self.nmos_a.apply_control_signal(signal_a);
        self.nmos_a_bar.apply_control_signal(!signal_a);
        self.nmos_b.apply_control_signal(signal_b);
        self.nmos_b_bar.apply_control_signal(!signal_b);

        self.pmos_a.connect_source(VDD);
        self.pmos_a_bar.connect_source(VDD);
        self.nmos_b.connect_source(GND);
        self.nmos_b_bar.connect_source(GND);

        self.pmos_b_bar.connect_source(self.pmos_a.drain());
        self.pmos_b.connect_source(self.pmos_a_bar.drain());
        self.nmos_a.connect_source(self.nmos_b.drain());
        self.nmos_a_bar.connect_source(self.nmos_b_bar.drain());

        (self.pmos_b.drain() || self.pmos_b_bar.drain())
            && (!self.nmos_a_bar.drain() || !self.nmos_a.drain())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_evaluate_with_signal_a_false_signal_b_false() {
        let mut xor = Xor::new();
        let result = xor.evaluate(false, false);
        assert!(!result);
    }

    #[test]
    fn xor_evaluate_with_signal_a_false_signal_b_true() {
        let mut xor = Xor::new();
        let result = xor.evaluate(false, true);
        assert!(result);
    }

    #[test]
    fn xor_evaluate_with_signal_a_true_signal_b_false() {
        let mut xor = Xor::new();
        let result = xor.evaluate(true, false);
        assert!(result);
    }

    #[test]
    fn xor_evaluate_with_signal_a_true_signal_b_true() {
        let mut xor = Xor::new();
        let result = xor.evaluate(true, true);
        assert!(!result);
    }
}
