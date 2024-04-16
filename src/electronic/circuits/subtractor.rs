use crate::electronic::circuits::logic_gates::xor::Xor;
use crate::electronic::circuits::logic_gates::not::Not;
use crate::electronic::circuits::logic_gates::and::And;
use crate::electronic::circuits::logic_gates::or::Or;
use crate::electronic::circuits::mux::Mux2To1;


pub struct SubtractorResult {
    pub difference: bool,
    pub borrow_out: bool
}

pub struct HalfSubtractor {
    xor: Xor,
    not: Not,
    and: And
}

impl HalfSubtractor {
    pub fn new() -> Self {
        HalfSubtractor {
            xor: Xor::new(),
            not: Not::new(),
            and: And::new()
        }
    }

    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool) -> SubtractorResult {
        let _difference = self.xor.evaluate(signal_a, signal_b);
        let _not_signal_a = self.not.evaluate(signal_a);
        let _borrow_out = self.and.evaluate(_not_signal_a, signal_b);
        SubtractorResult{difference: _difference, borrow_out: _borrow_out}
    }
}

pub struct FullSubtractor {
    xor0: Xor,
    xor1: Xor,
    not0: Not,
    not1: Not,
    and0: And,
    and1: And,
    or: Or
}

impl FullSubtractor {
    pub fn new() -> Self {
        FullSubtractor {
            xor0: Xor::new(),
            xor1: Xor::new(),
            not0: Not::new(),
            not1: Not::new(),
            and0: And::new(),
            and1: And::new(),
            or: Or::new()
        }
    }

    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool, borrow_in: bool) -> SubtractorResult {
        let _xor0_result = self.xor0.evaluate(signal_a, signal_b);
        let _difference = self.xor1.evaluate(_xor0_result, borrow_in);
        let _not_signal_a = self.not0.evaluate(signal_a);
        let _and0_result = self.and0.evaluate(_not_signal_a, signal_b);
        let _not1_result = self.not1.evaluate(_xor0_result);
        let _and1_result = self.and1.evaluate(_not1_result, borrow_in);
        let _borrow_out = self.or.evaluate(_and0_result, _and1_result);
        SubtractorResult{difference: _difference, borrow_out: _borrow_out}
    }
}

pub struct FullSubtractorRestore {
    full_subtractor: FullSubtractor,
    mux: Mux2To1
}

impl FullSubtractorRestore {
    pub fn new() -> Self {
        FullSubtractorRestore {
            full_subtractor: FullSubtractor::new(),
            mux: Mux2To1::new()
        }
    }

    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool, borrow_in: bool, carry: bool) -> (bool, bool) {
        let subtractor_result = self.full_subtractor.evaluate(signal_a, signal_b, borrow_in);
        let mux_result = self.mux.evaluate(signal_a, subtractor_result.difference, carry);

        (mux_result, subtractor_result.borrow_out)
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

    #[test]
    fn full_subtractor_restore() {
        for a in [false, true] {
            for b in [false, true] {
                for borrow_in in [false, true] {
                    for carry in [false, true] {
                        let mut full_subtractor_restore = FullSubtractorRestore::new();
                        let (result, borrow_out) = full_subtractor_restore.evaluate(a, b, borrow_in, carry);
                        if carry {
                            if (a as u8) < (b as u8 + borrow_in as u8) {
                                assert!(borrow_out);
                                // assert borrow_out is True, f"Inputs: a={a}, b={b}, borrow_in={borrow_in}, carry={carry}"
                            }
                            let expected_result = (a as u8 + b as u8 + borrow_in as u8) % 2 != 0;
                            assert!(result == expected_result);
                        } else {
                            assert!(result == a);
                        }
                    }
                }
            }
        }
    }
}