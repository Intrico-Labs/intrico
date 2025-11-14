//! The crucial traits required for implementing different QuantumState representations.

use rusticle::Complex;

pub type Amplitude = Complex<f64>;

// The QuantumState footprint
pub trait QuantumState {
    fn n_qubits(&self) -> usize;
    fn len(&self) -> usize;
    
    fn as_slice(&self) -> &[Amplitude];
    fn as_mut_slice(&mut self) -> &mut [Amplitude];
}