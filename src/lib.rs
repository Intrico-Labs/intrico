//! # 🚀 Intrico
//!
//! ### High-performance quantum computing library for Rust
//! **Simulate quantum circuits with precision and speed**
//! 
//! ## This SDK has a three-layered architecture
//! ### Layer 1: Core API Layer
//! - This layer contains all the builder APIs essential for building a Quantum Circuit
//! 
//! ### Layer 2: Intermediate Representation (IR) Layer
//! - This layer holds the IR description of the high-level circuit.
//! - The QuantumCircuit is lowered into CircuitIR in this layer.
//! 
//! ### Layer 3: Backend Execution Layer
//! - This layer builds the QuantumState information and evolves it
//! according to the CircuitIR passed into it.
//! - This layer is also responsible for fusion and optimization of matrices.
//! 

pub mod backend;
pub mod core;
pub mod ir;
pub mod library;

pub use core::{SequentialCircuit, QuantumCircuit};