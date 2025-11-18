//! The StateVectorDense implementation
//! The amplitudes of the quantum state are stored in a single contiguous memory buffer.

use crate::core::{Amplitude, state::QuantumState};

// The dense statevector struct
pub struct StateVectorDense {
    amplitudes: Vec<Amplitude>,
    num_qubits: usize,
    len: usize,
}

// QuantumState trait implementation
impl QuantumState for StateVectorDense {
    fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    fn len(&self) -> usize {
        self.len
    }

    fn as_slice(&self) -> &[Amplitude] {
        &self.amplitudes
    }

    fn as_mut_slice(&mut self) -> &mut [Amplitude] {
        &mut self.amplitudes
    }
}

// StateVector implementations
impl StateVectorDense {
    // Create a new statevector
    pub fn new(num_qubits: usize) -> Self {
        let len = 1<<num_qubits;
        let mut amplitudes = vec![Amplitude::new(0.0, 0.0); len];

        // Defaulting to |0..0⟩ state
        amplitudes[0] = Amplitude::new(1.0, 0.0);

        Self {
            amplitudes,
            num_qubits,
            len
        }
    }
}