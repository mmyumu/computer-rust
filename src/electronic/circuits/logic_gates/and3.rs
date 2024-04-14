use crate::electronic::circuits::logic_gates::and::And;

pub struct And3 {
    and0: And,
    and1: And
}

impl And3 {
    pub fn new() -> Self {
        And3 {
            and0: And::new(),
            and1: And::new()
        }
    }

    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool, signal_c: bool) -> bool {
        let _and0_result = self.and0.evaluate(signal_a, signal_b);
        self.and1.evaluate(_and0_result, signal_c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn and3_evaluate_with_signal_a_false_signal_b_false_signal_c_false() {
        let mut and3 = And3::new();
        let result = and3.evaluate(false, false, false);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_false_signal_b_false_signal_c_true() {
        let mut and3 = And3::new();
        let result = and3.evaluate(false, false, true);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_false_signal_b_true_signal_c_false() {
        let mut and3 = And3::new();
        let result = and3.evaluate(false, true, false);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_false_signal_b_true_signal_c_true() {
        let mut and3 = And3::new();
        let result = and3.evaluate(false, true, true);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_true_signal_b_false_signal_c_false() {
        let mut and3 = And3::new();
        let result = and3.evaluate(true, false, false);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_true_signal_b_false_signal_c_true() {
        let mut and3 = And3::new();
        let result = and3.evaluate(true, false, true);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_true_signal_b_true_signal_c_false() {
        let mut and3 = And3::new();
        let result = and3.evaluate(true, true, false);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_true_signal_b_true_signal_c_true() {
        let mut and3 = And3::new();
        let result = and3.evaluate(true, true, true);
        assert!(result);
    }
}