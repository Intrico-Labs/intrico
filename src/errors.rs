#[derive(Debug)]
pub enum CircuitError {
    /// Target or control qubit index is out of bounds
    QubitOutOfRange {
        index: usize,
        n_qubits: usize
    }
}
