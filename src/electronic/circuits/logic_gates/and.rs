use crate::electronic::circuits::logic_gates::{nand::Nand, not::Not};

pub struct And {
    nand: Nand,
    not: Not
}

impl And {
    pub fn new() -> Self {
        And  {
            nand: Nand::new(),
            not: Not::new()
        }
    }

    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool) -> bool {
        let _nand_result = self.nand.evaluate(signal_a, signal_b);
        self.not.evaluate(_nand_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn and_evaluate_with_signal_a_false_signal_b_false() {
        let mut and = And::new();
        let result = and.evaluate(false, false);
        assert!(!result);
    }

    #[test]
    fn and_evaluate_with_signal_a_false_signal_b_true() {
        let mut and = And::new();
        let result = and.evaluate(false, true);
        assert!(!result);
    }

    #[test]
    fn and_evaluate_with_signal_a_true_signal_b_false() {
        let mut and = And::new();
        let result = and.evaluate(true, false);
        assert!(!result);
    }

    #[test]
    fn and_evaluate_with_signal_a_true_signal_b_true() {
        let mut and = And::new();
        let result = and.evaluate(true, true);
        assert!(result);
    }
}