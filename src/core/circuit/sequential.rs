use crate::core::{circuit::{GateOp, QuantumCircuit}};

pub struct SequentialCircuit {
    n_qubits: usize,
    operations: Vec<GateOp>,
    cached_depth: Option<usize>
}

impl QuantumCircuit for SequentialCircuit {
    fn n_qubits(&self) -> usize {
        self.n_qubits
    }

    fn add_op(&mut self, op: super::GateOp) {
        self.operations_mut().push(op);
    }

    fn depth(&self) -> usize {
        todo!()
    }
}

impl SequentialCircuit {
    // new circuit
    pub fn new(n_qubits: usize) -> Self {
        Self {
            n_qubits,
            operations: Vec::new(),
            cached_depth: None
        }
    }

    pub fn operations(&self) -> &Vec<GateOp> {
        &self.operations
    }

    pub fn operations_mut(&mut self) -> &mut Vec<GateOp> {
        &mut self.operations
    }

    pub fn depth(&self) -> usize {
        if self.cached_depth != None {
            self.cached_depth.unwrap()
        } else {
            // TODO: depth calculation logic here
            todo!()
        }
    }

    // Standard gates
    pub fn x(&mut self) -> Self {
        todo!()
    }


}