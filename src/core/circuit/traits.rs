use crate::{core::circuit::GateOp, ir::CircuitIR};

pub trait QuantumCircuit {
    fn num_qubits(&self) -> usize;
    fn add_op(&mut self, op: GateOp);
    fn depth(&self) -> usize;
    // fn iter_ops(&self);
    fn to_ir(&self) -> CircuitIR;
}