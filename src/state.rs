use smallvec::{SmallVec, smallvec};

type Complex = SmallVec<[f64; 2]>;

/// Quantum State representation
pub struct QuantumState {
    pub statevector: Vec<Complex>
}

impl QuantumState {
    pub fn new(num_qubits: usize) -> Self {
        let dim = 1 << num_qubits;

        // init to 00..00 state
        let mut sv = vec![smallvec![0.0, 0.0]; dim];
        sv[0] = smallvec![1.0, 0.0];
        
        Self {
            statevector: sv
        }
    }
}