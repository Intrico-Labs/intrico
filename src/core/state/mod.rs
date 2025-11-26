//! Quantum state representations.
//!
//! This module provides quantum state abstractions and implementations,
//! including dense statevector representation.

pub mod traits;
pub mod dense;

pub use traits::{Amplitude, QuantumState};
pub use dense::StatevectorDense;