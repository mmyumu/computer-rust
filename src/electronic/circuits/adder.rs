use crate::electronic::circuits::logic_gates::xor::Xor;
use crate::electronic::circuits::logic_gates::and::And;
use crate::electronic::circuits::logic_gates::or::Or;
use crate::electronic::circuits::logic_gates::gate::{Gate, GateInput};

pub struct AdderResult {
    sum: bool,
    carry_out: bool
}

pub struct HalfAdder {
    _xor: Xor,
    _and: And
}

impl HalfAdder {
    pub fn new() -> Self {
        HalfAdder {
            _xor: Xor::new(),
            _and: And::new()
        }
    }
    pub fn evaluate(&mut self, _signal_a: bool, _signal_b: bool) -> AdderResult {
        let _xor_input = GateInput::Dual(_signal_a, _signal_b);
        let _sum_result = self._xor.evaluate(_xor_input);

        let _and_input = GateInput::Dual(_signal_a, _signal_b);
        let _carry = self._and.evaluate(_and_input);

        AdderResult{sum: _sum_result, carry_out: _carry}
    }
}

pub struct FullAdder {
    _xor0: Xor,
    _xor1: Xor,
    _and0: And,
    _and1: And,
    _or: Or
}

impl FullAdder {
    pub fn new() -> Self {
        FullAdder {
            _xor0: Xor::new(),
            _xor1: Xor::new(),
            _and0: And::new(),
            _and1: And::new(),
            _or: Or::new()
        }
    }

    pub fn evaluate(&mut self, _signal_a: bool, _signal_b: bool, _carry_in: bool) -> AdderResult {
        let _xor0_input = GateInput::Dual(_signal_a, _signal_b);
        let _xor0_result = self._xor0.evaluate(_xor0_input);

        let _xor1_input = GateInput::Dual(_xor0_result, _carry_in);
        let _sum_result = self._xor1.evaluate(_xor1_input);

        let _and0_input = GateInput::Dual(_carry_in, _xor0_result);
        let _and0_result = self._and0.evaluate(_and0_input);

        let _and1_input = GateInput::Dual(_signal_a, _signal_b);
        let _and1_result = self._and1.evaluate(_and1_input);

        let _or_input = GateInput::Dual(_and0_result, _and1_result);
        let _carry = self._or.evaluate(_or_input);

        AdderResult{sum: _sum_result, carry_out: _carry}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn half_adder_evaluate_with_signal_a_false_signal_b_false() {
        let mut half_adder = HalfAdder::new();
        let result = half_adder.evaluate(false, false);
        assert!(!result.sum);
        assert!(!result.carry_out);
    }

    #[test]
    fn half_adder_evaluate_with_signal_a_false_signal_b_true() {
        let mut half_adder = HalfAdder::new();
        let result = half_adder.evaluate(false, true);
        assert!(result.sum);
        assert!(!result.carry_out);
    }

    #[test]
    fn half_adder_evaluate_with_signal_a_true_signal_b_false() {
        let mut half_adder = HalfAdder::new();
        let result = half_adder.evaluate(true, false);
        assert!(result.sum);
        assert!(!result.carry_out);
    }

    #[test]
    fn half_adder_evaluate_with_signal_a_true_signal_b_true() {
        let mut half_adder = HalfAdder::new();
        let result = half_adder.evaluate(true, true);
        assert!(!result.sum);
        assert!(result.carry_out);
    }
    
    #[test]
    fn full_adder_evaluate_with_signal_a_false_signal_b_false_carry_in_false() {
        let mut full_adder = FullAdder::new();
        let result = full_adder.evaluate(false, false, false);
        assert!(!result.sum);
        assert!(!result.carry_out);
    }

    #[test]
    fn full_adder_evaluate_with_signal_a_false_signal_b_false_carry_in_true() {
        let mut full_adder = FullAdder::new();
        let result = full_adder.evaluate(false, false, true);
        assert!(result.sum);
        assert!(!result.carry_out);
    }

    #[test]
    fn full_adder_evaluate_with_signal_a_false_signal_b_true_carry_in_false() {
        let mut full_adder = FullAdder::new();
        let result = full_adder.evaluate(false, true, false);
        assert!(result.sum);
        assert!(!result.carry_out);
    }

    #[test]
    fn full_adder_evaluate_with_signal_a_false_signal_b_true_carry_in_true() {
        let mut full_adder = FullAdder::new();
        let result = full_adder.evaluate(false, true, true);
        assert!(!result.sum);
        assert!(result.carry_out);
    }

    #[test]
    fn full_adder_evaluate_with_signal_a_true_signal_b_false_carry_in_false() {
        let mut full_adder = FullAdder::new();
        let result = full_adder.evaluate(true, false, false);
        assert!(result.sum);
        assert!(!result.carry_out);
    }

    #[test]
    fn full_adder_evaluate_with_signal_a_true_signal_b_false_carry_in_true() {
        let mut full_adder = FullAdder::new();
        let result = full_adder.evaluate(true, false, true);
        assert!(!result.sum);
        assert!(result.carry_out);
    }

    #[test]
    fn full_adder_evaluate_with_signal_a_true_signal_b_true_carry_in_false() {
        let mut full_adder = FullAdder::new();
        let result = full_adder.evaluate(true, true, false);
        assert!(!result.sum);
        assert!(result.carry_out);
    }

    #[test]
    fn full_adder_evaluate_with_signal_a_true_signal_b_true_carry_in_true() {
        let mut full_adder = FullAdder::new();
        let result = full_adder.evaluate(true, true, true);
        assert!(result.sum);
        assert!(result.carry_out);
    }
}