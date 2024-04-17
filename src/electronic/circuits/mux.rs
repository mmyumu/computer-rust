use crate::electronic::circuits::logic_gates::and::And;
use crate::electronic::circuits::logic_gates::not::Not;
use crate::electronic::circuits::logic_gates::or::Or;

pub struct Mux2To1 {
    not: Not,
    and0: And,
    and1: And,
    or: Or,
}

impl Mux2To1 {
    pub fn new() -> Self {
        Mux2To1 {
            not: Not::new(),
            and0: And::new(),
            and1: And::new(),
            or: Or::new(),
        }
    }

    pub fn evaluate(&mut self, signal_a: bool, signal_b: bool, _s: bool) -> bool {
        let _not_result = self.not.evaluate(_s);
        let _and0_result = self.and0.evaluate(signal_a, _not_result);
        let _and1_result = self.and1.evaluate(signal_b, _s);
        self.or.evaluate(_and0_result, _and1_result)
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
