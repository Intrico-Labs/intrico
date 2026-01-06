use crate::Complex;

/// Quantum State representation
pub struct QuantumState {
    pub statevector: Vec<Complex>
}

impl QuantumState {
    pub fn new(num_qubits: usize) -> Self {
        let dim = 1 << num_qubits;

        // init to 00..00 state
        let mut sv = vec![Complex::new(0.0, 0.0); dim];
        sv[0] = Complex::new(1.0, 0.0);

        Self {
            statevector: sv
        }
    }

    pub fn statevector(&self) -> &Vec<Complex> {
        &self.statevector
    }
    
    pub fn statevector_mut(&mut self) -> &mut Vec<Complex> {
        &mut self.statevector
    }
}