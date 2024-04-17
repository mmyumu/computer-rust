use crate::electronic::components::transistor::{NMOSTransistor, PMOSTransistor, Transistor};
use crate::electronic::components::voltage_levels::{GND, VDD};

pub struct Nor {
    nmos_a: NMOSTransistor,
    nmos_b: NMOSTransistor,
    pmos_a: PMOSTransistor,
    pmos_b: PMOSTransistor,
}

impl Nor {
    pub fn new() -> Self {
        Nor {
            nmos_a: NMOSTransistor::new(),
            nmos_b: NMOSTransistor::new(),
            pmos_a: PMOSTransistor::new(),
            pmos_b: PMOSTransistor::new(),
        }
    }

    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool) -> bool {
        self.nmos_a.apply_control_signal(signal_a);
        self.pmos_a.apply_control_signal(signal_a);
        self.nmos_b.apply_control_signal(signal_b);
        self.pmos_b.apply_control_signal(signal_b);

        self.nmos_a.connect_source(GND);
        self.nmos_b.connect_source(GND);

        self.pmos_a.connect_source(VDD);
        self.pmos_b.connect_source(self.pmos_a.drain());

        self.pmos_b.drain() && !(self.nmos_a.drain() || self.nmos_b.drain())
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
