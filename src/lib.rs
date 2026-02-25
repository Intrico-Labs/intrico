//! # 🚀 Intrico
//!
//! ### High-performance quantum computing library for Rust
//! **Simulate quantum circuits with precision and speed**
//!

pub mod circuit;
pub mod core;
pub mod encoder;
pub mod engine;
pub mod library;
pub mod visualisations;

// Re-exports
pub use circuit::{Operation, QuantumCircuit, QuantumNode};
pub use core::{ClassicalRegister, QuantumGate, QuantumState};
pub use engine::{ExecutionContext, ExecutionTime, MeasurementResult, SamplingResult};
pub use visualisations::plot_histogram;
