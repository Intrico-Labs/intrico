//! Quantum circuit representations.
//!
//! This module provides circuit types and gate operation structures
//! for building and manipulating quantum circuits.

pub mod sequential;
pub mod traits;
pub mod gateop;

pub use gateop::GateOp;
pub use traits::QuantumCircuit;
pub use sequential::SequentialCircuit;