use std::{f64::consts::PI};

use crate::SequentialCircuit;

pub struct QFT {}

impl QFT {
    /// Builds a Quantum Fourier Transform circuit
    /// *Note:* This circuit is built backwards for correct order of output qubits
    /// 
    /// # Arguments
    /// * `num_qubits`: Number of qubits the QFT applies on
    pub fn new(num_qubits: usize) -> SequentialCircuit {
        let mut qc = SequentialCircuit::new(num_qubits);

        for i in (0..num_qubits).rev() {
            for k in ((i+1)..num_qubits).rev() {
                let exp = (2 as u32).pow((k-i) as u32); // 2^j where j is positional distance between qubits
                qc.cp(PI / exp as f64, k, i);
            }

            qc.h(i);
        }

        // Swap gates for correct order
        for i in 0..num_qubits / 2 {
            qc.swap(i, num_qubits - 1 - i);
        }
        
        qc
    }
}