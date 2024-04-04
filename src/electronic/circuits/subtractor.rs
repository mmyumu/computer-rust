use crate::electronic::circuits::logic_gates::xor::Xor;
use crate::electronic::circuits::logic_gates::not::Not;
use crate::electronic::circuits::logic_gates::and::And;
use crate::electronic::circuits::logic_gates::or::Or;
use crate::electronic::circuits::logic_gates::gate::{Gate, GateInput};

pub struct SubtractorResult {
    difference: bool,
    borrow_out: bool
}

pub struct HalfSubtractor {
    _xor: Xor,
    _not: Not,
    _and: And
}

impl HalfSubtractor {
    pub fn new() -> Self {
        HalfSubtractor {
            _xor: Xor::new(),
            _not: Not::new(),
            _and: And::new()
        }
    }

    pub fn evaluate(&mut self, _signal_a: bool, _signal_b: bool) -> SubtractorResult {
        let _xor_input = GateInput::Dual(_signal_a, _signal_b);
        let _difference = self._xor.evaluate(_xor_input);

        let _not_input = GateInput::Single(_signal_a);
        let _not_signal_a = self._not.evaluate(_not_input);

        let _and_input = GateInput::Dual(_not_signal_a, _signal_b);
        let _borrow_out = self._and.evaluate(_and_input);

        SubtractorResult{difference: _difference, borrow_out: _borrow_out}
    }
}

pub struct FullSubtractor {
    _xor0: Xor,
    _xor1: Xor,
    _not0: Not,
    _not1: Not,
    _and0: And,
    _and1: And,
    _or: Or
}

impl FullSubtractor {
    pub fn new() -> Self {
        FullSubtractor {
            _xor0: Xor::new(),
            _xor1: Xor::new(),
            _not0: Not::new(),
            _not1: Not::new(),
            _and0: And::new(),
            _and1: And::new(),
            _or: Or::new()
        }
    }

    pub fn evaluate(&mut self, _signal_a: bool, _signal_b: bool, _borrow_in: bool) -> SubtractorResult {
        let _xor0_input = GateInput::Dual(_signal_a, _signal_b);
        let _xor0_result = self._xor0.evaluate(_xor0_input);

        let _xor1_input = GateInput::Dual(_xor0_result, _borrow_in);
        let _difference = self._xor1.evaluate(_xor1_input);

        let _not0_input = GateInput::Single(_signal_a);
        let _not_signal_a = self._not0.evaluate(_not0_input);

        let _and0_input = GateInput::Dual(_not_signal_a, _signal_b);
        let _and0_result = self._and0.evaluate(_and0_input);

        let _not1_input = GateInput::Single(_xor0_result);
        let _not1_result = self._not1.evaluate(_not1_input);

        let _and1_input = GateInput::Dual(_not1_result, _borrow_in);
        let _and1_result = self._and1.evaluate(_and1_input);

        let _or_input = GateInput::Dual(_and0_result, _and1_result);
        let _borrow_out = self._or.evaluate(_or_input);

        SubtractorResult{difference: _difference, borrow_out: _borrow_out}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_subtractor_evaluate_with_signal_a_false_signal_b_false() {
        let mut half_subtractor = HalfSubtractor::new();
        let result = half_subtractor.evaluate(false, false);
        assert!(!result.difference);
        assert!(!result.borrow_out);
    }

    #[test]
    fn half_subtractor_evaluate_with_signal_a_false_signal_b_true() {
        let mut half_subtractor = HalfSubtractor::new();
        let result = half_subtractor.evaluate(false, true);
        assert!(result.difference);
        assert!(result.borrow_out);
    }

    #[test]
    fn half_subtractor_evaluate_with_signal_a_true_signal_b_false() {
        let mut half_subtractor = HalfSubtractor::new();
        let result = half_subtractor.evaluate(true, false);
        assert!(result.difference);
        assert!(!result.borrow_out);
    }

    #[test]
    fn half_subtractor_evaluate_with_signal_a_true_signal_b_true() {
        let mut half_subtractor = HalfSubtractor::new();
        let result = half_subtractor.evaluate(true, true);
        assert!(!result.difference);
        assert!(!result.borrow_out);
    }
    
    #[test]
    fn full_subtractor_evaluate_with_signal_a_false_signal_b_false_carry_in_false() {
        let mut full_subtractor = FullSubtractor::new();
        let result = full_subtractor.evaluate(false, false, false);
        assert!(!result.difference);
        assert!(!result.borrow_out);
    }

    #[test]
    fn full_subtractor_evaluate_with_signal_a_false_signal_b_false_carry_in_true() {
        let mut full_subtractor = FullSubtractor::new();
        let result = full_subtractor.evaluate(false, false, true);
        assert!(result.difference);
        assert!(result.borrow_out);
    }

    #[test]
    fn full_subtractor_evaluate_with_signal_a_false_signal_b_true_carry_in_false() {
        let mut full_subtractor = FullSubtractor::new();
        let result = full_subtractor.evaluate(false, true, false);
        assert!(result.difference);
        assert!(result.borrow_out);
    }

    #[test]
    fn full_subtractor_evaluate_with_signal_a_false_signal_b_true_carry_in_true() {
        let mut full_subtractor = FullSubtractor::new();
        let result = full_subtractor.evaluate(false, true, true);
        assert!(!result.difference);
        assert!(result.borrow_out);
    }

    #[test]
    fn full_subtractor_evaluate_with_signal_a_true_signal_b_false_carry_in_false() {
        let mut full_subtractor = FullSubtractor::new();
        let result = full_subtractor.evaluate(true, false, false);
        assert!(result.difference);
        assert!(!result.borrow_out);
    }

    #[test]
    fn full_subtractor_evaluate_with_signal_a_true_signal_b_false_carry_in_true() {
        let mut full_subtractor = FullSubtractor::new();
        let result = full_subtractor.evaluate(true, false, true);
        assert!(!result.difference);
        assert!(!result.borrow_out);
    }

    #[test]
    fn full_subtractor_evaluate_with_signal_a_true_signal_b_true_carry_in_false() {
        let mut full_subtractor = FullSubtractor::new();
        let result = full_subtractor.evaluate(true, true, false);
        assert!(!result.difference);
        assert!(!result.borrow_out);
    }

    #[test]
    fn full_subtractor_evaluate_with_signal_a_true_signal_b_true_carry_in_true() {
        let mut full_subtractor = FullSubtractor::new();
        let result = full_subtractor.evaluate(true, true, true);
        assert!(result.difference);
        assert!(result.borrow_out);
    }
}