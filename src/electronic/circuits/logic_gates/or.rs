use crate::electronic::circuits::logic_gates::nor::Nor;
use crate::electronic::circuits::logic_gates::not::Not;
use crate::electronic::circuits::logic_gates::gate::{Gate, GateInput};

pub struct Or {
    _nor: Nor,
    _not: Not
}

impl Or {
    pub fn new() -> Self {
        Or {
            _nor: Nor::new(),
            _not: Not::new()
        }
    }
}

impl Gate for Or {
    fn evaluate(&mut self, input: GateInput) -> bool {
        match input {
            GateInput::Dual(_signal_a, _signal_b) => {
                let _nor_input = GateInput::Dual(_signal_a, _signal_b);
                let _nor_result = self._nor.evaluate(_nor_input);

                let _not_input = GateInput::Single(_nor_result);
                self._not.evaluate(_not_input)
            },
            _ => panic!("Or gate expects exactly two input signal."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn or_evaluate_wrong_inputs() {
        let mut or = Or::new();

        let input = GateInput::Triple(false, false, false);
        or.evaluate(input);
    }

    #[test]
    fn or_evaluate_with_signal_a_false_signal_b_false() {
        let mut or = Or::new();

        let input = GateInput::Dual(false, false);
        let result = or.evaluate(input);
        assert!(!result);
    }

    #[test]
    fn or_evaluate_with_signal_a_false_signal_b_true() {
        let mut or = Or::new();

        let input = GateInput::Dual(false, true);
        let result = or.evaluate(input);
        assert!(result);
    }

    #[test]
    fn or_evaluate_with_signal_a_true_signal_b_false() {
        let mut or = Or::new();

        let input = GateInput::Dual(true, false);
        let result = or.evaluate(input);
        assert!(result);
    }

    #[test]
    fn or_evaluate_with_signal_a_true_signal_b_true() {
        let mut or = Or::new();

        let input = GateInput::Dual(true, true);
        let result = or.evaluate(input);
        assert!(result);
    }
}