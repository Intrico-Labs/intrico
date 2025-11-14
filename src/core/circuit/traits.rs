use crate::core::circuit::GateOp;

pub trait QuantumCircuit {
    fn n_qubits(&self) -> usize;
    fn add_op(&mut self, op: GateOp);
    fn depth(&self) -> usize;
    // fn iter_ops(&self);
    // fn to_ir(&self) -> IntricoIr;
}