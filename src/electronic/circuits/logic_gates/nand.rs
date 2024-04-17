use crate::electronic::components::transistor::{NMOSTransistor, PMOSTransistor, Transistor};
use crate::electronic::components::voltage_levels::{GND, VDD};

pub struct Nand {
    nmos_a: NMOSTransistor,
    nmos_b: NMOSTransistor,
    pmos_a: PMOSTransistor,
    pmos_b: PMOSTransistor,
}

impl Nand {
    pub fn new() -> Self {
        Nand {
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

        self.pmos_a.connect_source(VDD);
        self.pmos_b.connect_source(VDD);

        self.nmos_b.connect_source(GND);
        self.nmos_a.connect_source(self.nmos_b.drain());

        (self.pmos_a.drain() || self.pmos_b.drain()) && !self.nmos_a.drain()
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
