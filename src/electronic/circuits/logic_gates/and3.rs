use crate::electronic::circuits::logic_gates::and::And;
use crate::electronic::circuits::logic_gates::gate::{Gate, GateInput};

pub struct And3 {
    _and0: And,
    _and1: And
}

impl And3 {
    pub fn new() -> Self {
        And3 {
            _and0: And::new(),
            _and1: And::new()
        }
    }
}

impl Gate for And3 {
    fn evaluate(&mut self, input: GateInput) -> bool {
        match input {
            GateInput::Triple(_signal_a, _signal_b, _signal_c) => {
                let _and0_input = GateInput::Dual(_signal_a, _signal_b);
                let _and0_result = self._and0.evaluate(_and0_input);

                let _and1_input = GateInput::Dual(_and0_result, _signal_c);
                self._and1.evaluate(_and1_input)
            },
            _ => panic!("And3 gate expects exactly two input signal."),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn and3_evaluate_wrong_inputs() {
        let mut and3 = And3::new();

        let input = GateInput::Dual(false, false);
        and3.evaluate(input);
    }

    #[test]
    fn and3_evaluate_with_signal_a_false_signal_b_false_signal_c_false() {
        let mut and3 = And3::new();

        let input = GateInput::Triple(false, false, false);
        let result = and3.evaluate(input);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_false_signal_b_false_signal_c_true() {
        let mut and3 = And3::new();

        let input = GateInput::Triple(false, false, true);
        let result = and3.evaluate(input);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_false_signal_b_true_signal_c_false() {
        let mut and3 = And3::new();

        let input = GateInput::Triple(false, true, false);
        let result = and3.evaluate(input);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_false_signal_b_true_signal_c_true() {
        let mut and3 = And3::new();

        let input = GateInput::Triple(false, true, true);
        let result = and3.evaluate(input);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_true_signal_b_false_signal_c_false() {
        let mut and3 = And3::new();

        let input = GateInput::Triple(true, false, false);
        let result = and3.evaluate(input);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_true_signal_b_false_signal_c_true() {
        let mut and3 = And3::new();

        let input = GateInput::Triple(true, false, true);
        let result = and3.evaluate(input);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_true_signal_b_true_signal_c_false() {
        let mut and3 = And3::new();

        let input = GateInput::Triple(true, true, false);
        let result = and3.evaluate(input);
        assert!(!result);
    }

    #[test]
    fn and3_evaluate_with_signal_a_true_signal_b_true_signal_c_true() {
        let mut and3 = And3::new();

        let input = GateInput::Triple(true, true, true);
        let result = and3.evaluate(input);
        assert!(result);
    }
}