//! Quantum Fourier Transform (QFT) circuit builder.

use std::f64::consts::PI;

use intrico_core::QuantumCircuit;


/// Builder for the n-qubit Quantum Fourier Transform circuit.
///
/// The QFT is the quantum analogue of the discrete Fourier transform. It maps
/// the computational basis state |x⟩ to a superposition where each amplitude
/// encodes a Fourier coefficient.
///
/// # Circuit structure
///
/// For each qubit `i` from high to low:
/// 1. Controlled-phase rotations `CP(π / 2^(k-1))` from qubit `k` to qubit `i`
///    for all `k > i`.
/// 2. Hadamard on qubit `i`.
///
/// After all rotations, bit-reversal SWAPs restore the standard output ordering.
pub struct QFT {}

impl QFT {
    /// Build an `n`-qubit QFT circuit.
    ///
    /// Returns a [`QuantumCircuit`] that can be executed directly or encoded to QISA.
    ///
    /// # Panics
    ///
    /// Panics if `num_qubits == 0`.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(num_qubits: usize) -> QuantumCircuit {
        let mut qc = QuantumCircuit::new(num_qubits);

        for i in (0..num_qubits).rev() {
            for k in ((i+1)..num_qubits).rev() {
                let exp = 2_u32.pow((k-1) as u32);
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
