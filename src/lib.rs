//! # Intrico - A high-performance Quantum Computing library
//! 
//! Welcome to Intrico, a powerful and intuitive quantum computing library for Rust.
//! 
//! This library provides a comprehensive set of tools for quantum computing simulation,
//! including support for qubits, quantum gates, and quantum state manipulation.
//! 
//! ## Usage
//! 
//! Add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! intrico = "1.0.0"
//! ```
//! 
//! or use `cargo add intrico` to add it to your project.
//! 
//! ## Features
//! 
//! | Feature | Description |
//! |---------|-------------|
//! | `core` | Core Quantum definitions like qubits, quantum gates, gate operations, etc |
//! | `circuit` | Quantum Circuit functionality including visualisations |
//! | `simulator` | Quantum Simulation functionality |
//! 
//! ## Quick Start
//! 
//! ```rust
//! use intrico::{Qubit, QuantumGate};
//! 
//! fn main() {
//!     // Create a qubit in the |0⟩ state
//!     let mut ket_0 = Qubit::zero();
//!     println!("ket_0 = {:?}\t{:?}", ket_0, ket_0.state_vector());
//! 
//!     // Apply a Hadamard gate to create a superposition
//!     ket_0.apply(QuantumGate::H);
//!     println!("psi = {:?}", ket_0);
//! }
//! ```

//! ## Examples
//! Checkout the [examples](./examples/) directory for all the examples. For convenience here's a list of some significant ones:

//! | Example | Description |
//! |---------|-------------|
//! | [`quantum-circuit`](./examples/quantum_circuit.rs) | A bell state quantum circuit visualisation |
//! | [`statevector-simulator`](./examples/statevector_simulator.rs) | A bell state simulation using statevector simulator |
//! | [`grovers-algorithm`](./examples/grovers_algorithm.rs) | A two-qubit grover's algorithm simulation |


pub mod core;
pub mod circuit;
pub mod simulator;
pub mod utility;

// Expose types from modules
pub use core::{Qubit, QuantumGate};
pub use circuit::QuantumCircuit;
