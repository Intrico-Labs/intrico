use crate::core::{QuantumGate};

pub struct GateOp {
    gate: QuantumGate,
    targets: Vec<usize>,
}

impl GateOp {
    pub fn new(gate: QuantumGate, targets: Vec<usize>) -> Self {
        GateOp { gate, targets }
    }

    pub fn gate(&self) -> &QuantumGate {
        &self.gate
    }

    pub fn targets(&self) -> &[usize] {
        &self.targets
    }
}

