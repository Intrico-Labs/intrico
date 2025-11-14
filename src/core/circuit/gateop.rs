use crate::core::{QuantumGate};

pub struct GateOp {
    gate: QuantumGate,
    targets: Vec<usize>,
}

impl GateOp {
    pub fn new(gate: QuantumGate, targets: Vec<usize>) -> Self {
        GateOp { gate, targets }
    }
}

