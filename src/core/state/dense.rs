//! Dense statevector implementation.
//!
//! This module provides a dense statevector representation where amplitudes
//! are stored in a contiguous memory buffer.

use crate::core::{Amplitude, state::QuantumState};

/// Dense statevector representation.
///
/// Stores all 2^n quantum state amplitudes in contiguous memory.
pub struct StatevectorDense {
    amplitudes: Vec<Amplitude>,
    num_qubits: usize,
    len: usize,
}

impl QuantumState for StatevectorDense {
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

impl StatevectorDense {
    /// Creates a new dense statevector initialized to |0...0⟩.
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