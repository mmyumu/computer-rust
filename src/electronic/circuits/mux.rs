use crate::electronic::circuits::logic_gates::not::Not;
use crate::electronic::circuits::logic_gates::and::And;
use crate::electronic::circuits::logic_gates::or::Or;

pub struct Mux2To1 {
    _not: Not,
    _and0: And,
    _and1: And,
    _or: Or
}

impl Mux2To1 {
    pub fn new() -> Self {
        Mux2To1 {
            _not: Not::new(),
            _and0: And::new(),
            _and1: And::new(),
            _or: Or::new()
        }
    }

    pub fn evaluate(&mut self, _signal_a: bool, _signal_b: bool, _s: bool) -> bool {
        let _not_result = self._not.evaluate(_s);
        let _and0_result = self._and0.evaluate(_signal_a, _not_result);
        let _and1_result = self._and1.evaluate(_signal_b, _s);
        self._or.evaluate(_and0_result, _and1_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mux2to1_evaluate() {
        let mut mux2to1 = Mux2To1::new();

        for a in [false, true] {
            for b in [false, true] {
                let result = mux2to1.evaluate(a, b, false);
                assert!(result == a);

                let result = mux2to1.evaluate(a, b, true);
                assert!(result == b);
            }
        }
    }
}