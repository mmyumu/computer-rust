use crate::electronic::circuits::logic_gates::xor::Xor;
use crate::electronic::circuits::logic_gates::not::Not;
use crate::electronic::circuits::logic_gates::and::And;
use crate::electronic::circuits::logic_gates::or::Or;
use crate::electronic::circuits::mux::Mux2To1;


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

    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool) -> SubtractorResult {
        let _difference = self._xor.evaluate(signal_a, signal_b);
        let _not_signal_a = self._not.evaluate(signal_a);
        let _borrow_out = self._and.evaluate(_not_signal_a, signal_b);
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

    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool, borrow_in: bool) -> SubtractorResult {
        let _xor0_result = self._xor0.evaluate(signal_a, signal_b);
        let _difference = self._xor1.evaluate(_xor0_result, borrow_in);
        let _not_signal_a = self._not0.evaluate(signal_a);
        let _and0_result = self._and0.evaluate(_not_signal_a, signal_b);
        let _not1_result = self._not1.evaluate(_xor0_result);
        let _and1_result = self._and1.evaluate(_not1_result, borrow_in);
        let _borrow_out = self._or.evaluate(_and0_result, _and1_result);
        SubtractorResult{difference: _difference, borrow_out: _borrow_out}
    }
}

pub struct FullSubtractorRestore {
    _full_subtractor: FullSubtractor,
    _mux: Mux2To1
}

impl FullSubtractorRestore {
    pub fn new() -> Self {
        FullSubtractorRestore {
            _full_subtractor: FullSubtractor::new(),
            _mux: Mux2To1::new()
        }
    }

    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool, borrow_in: bool, carry: bool) -> (bool, bool) {
        let subtractor_result = self._full_subtractor.evaluate(signal_a, signal_b, borrow_in);
        let mux_result = self._mux.evaluate(signal_a, subtractor_result.difference, carry);

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