use crate::electronic::circuits::logic_gates::xor::Xor;
use crate::electronic::circuits::logic_gates::and::And;
use crate::electronic::circuits::logic_gates::or::Or;

pub struct AdderResult {
    sum: bool,
    carry_out: bool
}

pub struct HalfAdder {
    xor: Xor,
    and: And
}

impl HalfAdder {
    pub fn new() -> Self {
        HalfAdder {
            xor: Xor::new(),
            and: And::new()
        }
    }
    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool) -> AdderResult {
        let _sum_result = self.xor.evaluate(signal_a, signal_b);
        let _carry = self.and.evaluate(signal_a, signal_b);
        AdderResult{sum: _sum_result, carry_out: _carry}
    }
}

pub struct FullAdder {
    xor0: Xor,
    xor1: Xor,
    and0: And,
    and1: And,
    or: Or
}

impl FullAdder {
    pub fn new() -> Self {
        FullAdder {
            xor0: Xor::new(),
            xor1: Xor::new(),
            and0: And::new(),
            and1: And::new(),
            or: Or::new()
        }
    }

    pub fn evaluate(&mut self, _signal_a: bool, _signal_b: bool, _carry_in: bool) -> AdderResult {
        let _xor0_result = self.xor0.evaluate(_signal_a, _signal_b);
        let _sum_result = self.xor1.evaluate(_xor0_result, _carry_in);
        let _and0_result = self.and0.evaluate(_carry_in, _xor0_result);
        let _and1_result = self.and1.evaluate(_signal_a, _signal_b);
        let _carry = self.or.evaluate(_and0_result, _and1_result);
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