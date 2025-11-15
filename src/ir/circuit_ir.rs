use crate::core::circuit::GateOp;

pub enum CircuitIR {
    Sequential(SequentialIR),
}

impl CircuitIR {
    pub fn n_qubits(&self) -> usize {
        match self {
            CircuitIR::Sequential(ir) => ir.n_qubits,
        }
    }

    pub fn save_json(&self) {
        todo!()
    }
}

pub struct SequentialIR {
    n_qubits: usize,
    ops: Vec<GateOp>
}

impl SequentialIR {
    pub fn new(n_qubits: usize, ops: Vec<GateOp>) -> Self {
        Self {
            n_qubits,
            ops
        }
    }
}