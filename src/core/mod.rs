//! Core quantum computing primitives.
//!
//! This module provides the user-facing API for quantum circuit construction,
//! gate definitions, and quantum state representations.

pub mod state;
pub mod gate;
pub mod circuit;

pub use state::Amplitude;
pub use gate::QuantumGate;
pub use circuit::{SequentialCircuit, QuantumCircuit};