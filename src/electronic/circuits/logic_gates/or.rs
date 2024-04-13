use crate::electronic::circuits::logic_gates::nor::Nor;
use crate::electronic::circuits::logic_gates::not::Not;

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

    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool) -> bool {
        let _nor_result = self._nor.evaluate(signal_a, signal_b);
        self._not.evaluate(_nor_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn or_evaluate_with_signal_a_false_signal_b_false() {
        let mut or = Or::new();
        let result = or.evaluate(false, false);
        assert!(!result);
    }

    #[test]
    fn or_evaluate_with_signal_a_false_signal_b_true() {
        let mut or = Or::new();
        let result = or.evaluate(false, true);
        assert!(result);
    }

    #[test]
    fn or_evaluate_with_signal_a_true_signal_b_false() {
        let mut or = Or::new();
        let result = or.evaluate(true, false);
        assert!(result);
    }

    #[test]
    fn or_evaluate_with_signal_a_true_signal_b_true() {
        let mut or = Or::new();
        let result = or.evaluate(true, true);
        assert!(result);
    }
}