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
//! intrico = "0.3.2"
//! ```
//! 
//! or use `cargo add intrico` to add it to your project.
//! 
//! ## Features
//! 
//! | Feature | Description |
//! |---------|-------------|
//! | `qsim_core` | Core quantum simulation functionality including qubits and quantum gates |
//! 
//! ## Example
//! 
//! ```rust
//! use intrico::qsim_core::{Qubit, QuantumGate};
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

pub mod qsim_core;

pub use qsim_core::{Qubit, QuantumGate};
