pub const GND: bool = false;
pub const VDD: bool = true;

pub trait Gate {
    fn evaluate(&mut self, input: GateInput) -> bool;
}

pub enum GateInput {
    Single(bool),
    Dual(bool, bool),
    Triple(bool, bool, bool),
    // Multiple(Vec<bool>),
}