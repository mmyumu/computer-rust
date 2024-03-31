pub trait Transistor {
    fn apply_control_signal(&mut self, signal: bool);
    fn connect_source(&mut self, source: bool);
    fn is_conducting(&self) -> bool;

    fn drain(&self) -> bool {
        self.is_conducting()
    }
}

#[derive(Default)]
pub struct TransistorState {
    control_gate: bool,
    source: bool,
}

impl TransistorState {
    pub fn apply_control_signal(&mut self, signal: bool) {
        self.control_gate = signal;
    }

    pub fn connect_source(&mut self, source: bool) {
        self.source = source;
    }
}


pub struct NMOSTransistor {
    state: TransistorState,
}

impl NMOSTransistor {
    pub fn new() -> Self {
        NMOSTransistor {
            state: TransistorState::default(),
        }
    }
}

impl Transistor for NMOSTransistor {
    fn apply_control_signal(&mut self, signal: bool) {
        self.state.apply_control_signal(signal);
    }

    fn connect_source(&mut self, source: bool) {
        self.state.connect_source(source);
    }

    fn is_conducting(&self) -> bool {
        self.state.control_gate && self.state.source
    }
}

pub struct PMOSTransistor {
    state: TransistorState,
}

impl PMOSTransistor {
    pub fn new() -> Self {
        PMOSTransistor {
            state: TransistorState::default(),
        }
    }
}

impl Transistor for PMOSTransistor {
    fn apply_control_signal(&mut self, signal: bool) {
        self.state.apply_control_signal(signal);
    }

    fn connect_source(&mut self, source: bool) {
        self.state.connect_source(source);
    }

    fn is_conducting(&self) -> bool {
        !self.state.control_gate && self.state.source
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nmos_conducting_with_no_source() {
        let nmos = NMOSTransistor::new();
        assert!(!nmos.is_conducting());
        assert!(nmos.is_conducting() == nmos.drain());
    }
    
    #[test]
    fn pmos_conducting_with_no_source() {
        let pmos = PMOSTransistor::new();
        assert!(!pmos.is_conducting());
        assert!(pmos.is_conducting() == pmos.drain());
    }

    #[test]
    fn nmos_conducting_with_source_false() {
        let mut nmos = NMOSTransistor::new();
        nmos.connect_source(false);
        assert!(!nmos.is_conducting());
        assert!(nmos.is_conducting() == nmos.drain());
    }

    #[test]
    fn pmos_conducting_with_source_false() {
        let mut pmos = PMOSTransistor::new();
        pmos.connect_source(false);
        assert!(!pmos.is_conducting());
        assert!(pmos.is_conducting() == pmos.drain());
    }

    #[test]
    fn nmos_conducting_with_source_true() {
        let mut nmos = NMOSTransistor::new();
        nmos.connect_source(true);
        assert!(!nmos.is_conducting());
        assert!(nmos.is_conducting() == nmos.drain());
    }

    #[test]
    fn pmos_conducting_with_source_true() {
        let mut pmos = PMOSTransistor::new();
        pmos.connect_source(true);
        assert!(pmos.is_conducting());
        assert!(pmos.is_conducting() == pmos.drain());
    }

    #[test]
    fn nmos_conducting_with_signal_false() {
        let mut nmos = NMOSTransistor::new();
        nmos.apply_control_signal(false);
        assert!(!nmos.is_conducting());
        assert!(nmos.is_conducting() == nmos.drain());
    }

    #[test]
    fn pmos_conducting_with_signal_false() {
        let mut pmos = PMOSTransistor::new();
        pmos.apply_control_signal(false);
        assert!(!pmos.is_conducting());
        assert!(pmos.is_conducting() == pmos.drain());
    }

    #[test]
    fn nmos_conducting_with_source_false_signal_false() {
        let mut nmos = NMOSTransistor::new();
        nmos.connect_source(false);
        nmos.apply_control_signal(false);
        assert!(!nmos.is_conducting());
        assert!(nmos.is_conducting() == nmos.drain());
    }

    #[test]
    fn pmos_conducting_with_source_false_signal_false() {
        let mut pmos = PMOSTransistor::new();
        pmos.connect_source(false);
        pmos.apply_control_signal(false);
        assert!(!pmos.is_conducting());
        assert!(pmos.is_conducting() == pmos.drain());
    }

    #[test]
    fn nmos_conducting_with_source_false_signal_true() {
        let mut nmos = NMOSTransistor::new();
        nmos.connect_source(false);
        nmos.apply_control_signal(true);
        assert!(!nmos.is_conducting());
        assert!(nmos.is_conducting() == nmos.drain());
    }

    #[test]
    fn pmos_conducting_with_source_false_signal_true() {
        let mut pmos = PMOSTransistor::new();
        pmos.connect_source(false);
        pmos.apply_control_signal(true);
        assert!(!pmos.is_conducting());
    }

    #[test]
    fn nmos_conducting_with_source_true_signal_false() {
        let mut nmos = NMOSTransistor::new();
        nmos.connect_source(true);
        nmos.apply_control_signal(false);
        assert!(!nmos.is_conducting());
        assert!(nmos.is_conducting() == nmos.drain());
    }

    #[test]
    fn pmos_conducting_with_source_true_signal_false() {
        let mut pmos = PMOSTransistor::new();
        pmos.connect_source(true);
        pmos.apply_control_signal(false);
        assert!(pmos.is_conducting());
    }

    #[test]
    fn nmos_conducting_with_source_true_signal_true() {
        let mut nmos = NMOSTransistor::new();
        nmos.connect_source(true);
        nmos.apply_control_signal(true);
        assert!(nmos.is_conducting());
    }

    #[test]
    fn pmos_conducting_with_source_true_signal_true() {
        let mut pmos = PMOSTransistor::new();
        pmos.connect_source(true);
        pmos.apply_control_signal(true);
        assert!(!pmos.is_conducting());
    }
}
 