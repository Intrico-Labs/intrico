//! Pre-built quantum circuit algorithms.
//!
//! Each module exposes a builder struct with a `new(n)` method that returns a
//! fully assembled [`QuantumCircuit`] ready for execution or encoding.
//!
//! # Modules
//! - [`qft`] — [`QFT`]: n-qubit Quantum Fourier Transform.

pub mod qft;
