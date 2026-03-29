use std::f64::consts::PI;

use intrico_core::QuantumCircuit;


/// Quantum Fourier Transform circuit
pub struct QFT {}

impl QFT {
    pub fn new(num_qubits: usize) -> QuantumCircuit {
        let mut qc = QuantumCircuit::new(num_qubits);

        for i in (0..num_qubits).rev() {
            for k in ((i+1)..num_qubits).rev() {
                let exp = (2 as u32).pow((k-1) as u32);
                qc.cp(k, i, PI / exp as f64);
            }

            qc.h(i);
        }

        // swap gates for correct order
        for i in 0..num_qubits / 2 {
            qc.swap(i, num_qubits - 1 - i);
        }

        qc
    }
}
