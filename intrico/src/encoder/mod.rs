//! Encoders that compile a [`QuantumCircuit`] into a portable bytecode format.
//!
//! Currently the only supported encoding is **QISA** (Quantum Instruction Set Architecture),
//! a compact binary format used to transfer circuits to `intrico-node` executor nodes.
//!
//! # Modules
//! - [`qisa`] — [`QisaEncoder`]: encodes a circuit to a `Vec<u8>` QISA blob.

pub mod qisa;
