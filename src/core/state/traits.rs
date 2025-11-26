//! Quantum state trait and type definitions.
//!
//! This module defines the core trait for quantum state implementations
//! and the Amplitude type alias.

use rusticle::Complex;

/// Complex amplitude type for quantum states.
pub type Amplitude = Complex<f64>;

/// Core trait for quantum state representations.
///
/// Defines the interface for accessing and manipulating quantum states.
pub trait QuantumState {
    /// Returns the number of qubits.
    fn num_qubits(&self) -> usize;
    /// Returns the state dimension (2^n).
    fn len(&self) -> usize;

    /// Returns the state as an immutable slice.
    fn as_slice(&self) -> &[Amplitude];
    /// Returns the state as a mutable slice.
    fn as_mut_slice(&mut self) -> &mut [Amplitude];
}