//! # 🚀 Intrico
//!
//! ### High-performance quantum computing library for Rust
//! **Simulate quantum circuits with precision and speed**
//!

pub mod core;
pub mod circuit;
pub mod engine;
pub mod library;
pub mod visualisations;

// Re-exports
pub use core::{QuantumState, QuantumGate, ClassicalRegister};
pub use circuit::{QuantumCircuit, Operation, QuantumNode};
pub use engine::{ExecutionContext, ExecutionTime, MeasurementResult, SamplingResult};
pub use visualisations::plot_histogram;
