use crate::electronic::circuits::logic_gates::{nand::Nand, not::Not};
use crate::electronic::circuits::logic_gates::gate::{Gate, GateInput};

pub struct And {
    _nand: Nand,
    _not: Not
}

impl And {
    pub fn new() -> Self {
        And  {
            _nand: Nand::new(),
            _not: Not::new()
        }
    }
}

impl Gate for And {
    fn evaluate(&mut self, input: GateInput) -> bool {
        match input {
            GateInput::Dual(_signal_a, _signal_b) => {
                let _nand_input = GateInput::Dual(_signal_a, _signal_b);
                let _nand_result = self._nand.evaluate(input);

                let _not_input = GateInput::Single(_nand_result);
                self._not.evaluate(_not_input)
            },
            _ => panic!("And gate expects exactly two input signal."),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn and_evaluate_wrong_inputs() {
        let mut and = And::new();

        let input = GateInput::Triple(false, false, false);
        and.evaluate(input);
    }

    #[test]
    fn and_evaluate_with_signal_a_false_signal_b_false() {
        let mut and = And::new();

        let input = GateInput::Dual(false, false);
        let result = and.evaluate(input);
        assert!(!result);
    }

    #[test]
    fn and_evaluate_with_signal_a_false_signal_b_true() {
        let mut and = And::new();

        let input = GateInput::Dual(false, true);
        let result = and.evaluate(input);
        assert!(!result);
    }

    #[test]
    fn and_evaluate_with_signal_a_true_signal_b_false() {
        let mut and = And::new();

        let input = GateInput::Dual(true, false);
        let result = and.evaluate(input);
        assert!(!result);
    }

    #[test]
    fn and_evaluate_with_signal_a_true_signal_b_true() {
        let mut and = And::new();

        let input = GateInput::Dual(true, true);
        let result = and.evaluate(input);
        assert!(result);
    }
}