use std::f64::consts::PI;

use crate::circuit::QuantumCircuit;


/// Quantum Fourier Transform circuit
pub struct QFT {}

impl QFT {
    pub fn new(num_qubits: usize) -> QuantumCircuit {
        let mut qc = QuantumCircuit::new(num_qubits);

        for i in (0..num_qubits).rev() {
            for k in ((i+1)..num_qubits).rev() {
                let angle = PI / 2.0_f64.powi((k - i) as i32);
                qc.cp(k, i, angle);
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