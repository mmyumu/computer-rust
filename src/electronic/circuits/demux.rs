use crate::electronic::circuits::logic_gates::and::And;
use crate::electronic::circuits::logic_gates::not::Not;

pub struct Demux1To2 {
    not: Not,
    and0: And,
    and1: And,
}

impl Demux1To2 {
    pub fn new() -> Self {
        Demux1To2 {
            not: Not::new(),
            and0: And::new(),
            and1: And::new(),
        }
    }

    pub fn evaluate(&mut self, signal: bool, s: bool) -> (bool, bool) {
        let _not_result = self.not.evaluate(s);

        let _and0_result = self.and0.evaluate(_not_result, signal);
        let _and1_result = self.and1.evaluate(signal, s);

        (_and1_result, _and0_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demux2to1_evaluate() {
        let mut demux1to2 = Demux1To2::new();

        for a in [false, true] {
            let result = demux1to2.evaluate(a, false);
            assert!(result == (false, a));

            let result = demux1to2.evaluate(a, true);
            assert!(result == (a, false));
        }
    }
}
