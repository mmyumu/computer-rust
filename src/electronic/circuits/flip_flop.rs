use rand::random;
use crate::electronic::circuits::logic_gates::nand::Nand;
use crate::electronic::circuits::logic_gates::not::Not;

pub struct DFlipFlop {
    _nand0: Nand,
    _nand1: Nand,
    _nand2: Nand,
    _nand3: Nand,
    _not: Not,

    _set: bool,
    _reset: bool,

    q: bool,
    q_bar: bool
}

impl DFlipFlop {
    pub fn new() -> Self {
        DFlipFlop {
            _nand0: Nand::new(),
            _nand1: Nand::new(),
            _nand2: Nand::new(),
            _nand3: Nand::new(),
            _not: Not::new(),
            _set: random(),
            _reset: random(),
            q: random(),
            q_bar: random(),
        }
    }

    pub fn set_sr(&mut self, _set: bool, _reset: bool) {
        self._set = _set;
        self._reset = _reset;
    }

    pub fn set_d(&mut self, _d: bool) {
        let _not_d = self._not.evaluate(_d);
        self.set_sr(_d, _not_d);
    }

    pub fn output(&mut self) -> (bool, bool) {
        (self.q, self.q_bar)
    }

    pub fn reset_states(&mut self) {
        self._set = false;
        self._set = true;
        self.q = false;
        self.q_bar = true;
    }

    pub fn clock_tick(&mut self, _enable: bool) -> (bool, bool) {
        if self._set && self._reset {
            panic!("Invalid state: set and reset are both high")
        }

        let _nand0_result = self._nand0.evaluate(self._set, _enable);
        let _nand1_result = self._nand1.evaluate(_enable, self._reset);

        let mut _next_q = self._nand2.evaluate(_nand0_result, self.q_bar);
        let mut _next_q_bar = self._nand3.evaluate(self.q, _nand1_result);

        // 2nd signal propagation
        _next_q = self._nand2.evaluate(_nand0_result, _next_q_bar);
        _next_q_bar = self._nand3.evaluate(_next_q, _nand1_result);

        self.q = _next_q;
        self.q_bar = _next_q_bar;

        self.output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d_flip_flop_evaluate_set_false_clock_false() {
        let mut dflipflop = DFlipFlop::new();
        dflipflop.reset_states();
        dflipflop.set_d(false);
        assert!(dflipflop.output() == (false, true));
        assert!(dflipflop.clock_tick(false) == (false, true));
    }

    #[test]
    fn d_flip_flop_evaluate_set_false_clock_true() {
        let mut dflipflop = DFlipFlop::new();
        dflipflop.reset_states();
        dflipflop.set_d(false);
        assert!(dflipflop.output() == (false, true));
        assert!(dflipflop.clock_tick(true) == (false, true));
    }

    #[test]
    fn d_flip_flop_evaluate_set_true_clock_false() {
        let mut dflipflop = DFlipFlop::new();
        dflipflop.reset_states();
        dflipflop.set_d(true);
        assert!(dflipflop.output() == (false, true));
        assert!(dflipflop.clock_tick(false) == (false, true));
    }

    #[test]
    fn d_flip_flop_evaluate_set_true_clock_true() {
        let mut dflipflop = DFlipFlop::new();
        dflipflop.reset_states();
        dflipflop.set_d(true);
        assert!(dflipflop.output() == (false, true));
        assert!(dflipflop.clock_tick(true) == (true, false));
    }

    #[test]
    fn d_flip_flop_evaluate_set_false_clock_false_initial_q_true_qbar_false() {
        let mut dflipflop = DFlipFlop::new();

        dflipflop.q = true;
        dflipflop.q_bar = false;

        dflipflop.set_d(false);
        assert!(dflipflop.output() == (true, false));
        assert!(dflipflop.clock_tick(false) == (true, false));
    }

    #[test]
    fn d_flip_flop_evaluate_set_false_clock_true_initial_q_true_qbar_false() {
        let mut dflipflop = DFlipFlop::new();

        dflipflop.q = true;
        dflipflop.q_bar = false;

        dflipflop.set_d(false);
        assert!(dflipflop.output() == (true, false));
        assert!(dflipflop.clock_tick(true) == (false, true));
    }

    #[test]
    fn d_flip_flop_evaluate_set_true_clock_false_initial_q_true_qbar_false() {
        let mut dflipflop = DFlipFlop::new();

        dflipflop.q = true;
        dflipflop.q_bar = false;

        dflipflop.set_d(true);
        assert!(dflipflop.output() == (true, false));
        assert!(dflipflop.clock_tick(false) == (true, false));
    }

    #[test]
    fn d_flip_flop_evaluate_set_true_clock_true_initial_q_true_qbar_false() {
        let mut dflipflop = DFlipFlop::new();

        dflipflop.q = true;
        dflipflop.q_bar = false;

        dflipflop.set_d(true);
        assert!(dflipflop.output() == (true, false));
        assert!(dflipflop.clock_tick(true) == (true, false));
    }

    #[test]
    fn d_flip_flop_memory_sequence() {
        let mut dflipflop = DFlipFlop::new();

        dflipflop.reset_states();

        // Initial state
        dflipflop.set_d(false);
        assert!(dflipflop.clock_tick(false) == (false, true));

        // Set new state: 1
        dflipflop.set_d(true);
        assert!(dflipflop.clock_tick(true) == (true, false));

        // Memorized previous state: 1
        assert!(dflipflop.clock_tick(false) == (true, false));

        // Set new state: 0
        dflipflop.set_d(false);
        assert!(dflipflop.clock_tick(true) == (false, true));
    }

    #[test]
    fn d_flip_flop_unstable_initial_state_true() {
        let mut dflipflop = DFlipFlop::new();
    
        // This state should be avoided, we need to reset state before using dflipflop
        dflipflop.q = true;
        dflipflop.q_bar = true;

        dflipflop.set_d(false);
        assert!(dflipflop.clock_tick(false) == (true, false));
        assert!(dflipflop.clock_tick(false) == (true, false));
    }

    #[test]
    fn d_flip_flop_unstable_initial_state_false() {
        let mut dflipflop = DFlipFlop::new();
    
        // This state should be avoided, we need to reset state before using dflipflop
        dflipflop.q = false;
        dflipflop.q_bar = false;

        dflipflop.set_d(false);
        assert!(dflipflop.clock_tick(false) == (false, true));
        assert!(dflipflop.clock_tick(false) == (false, true));
    }
}
