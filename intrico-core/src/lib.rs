//! # intrico-core
//!
//! Core execution engine and circuit primitives for Intrico quantum computing.
//! Contains the statevector simulation engine, gate definitions, circuit DAG,
//! and execution context. Use the `intrico` crate for the full user-facing SDK.

pub mod circuit;
pub mod core;
pub mod engine;

pub use circuit::{Operation, QuantumCircuit, QuantumNode};
pub use core::{ClassicalRegister, QuantumGate, QuantumState};
pub use core::gate::GateKind;
pub use engine::{ExecutionContext, ExecutionTime, MeasurementResult, SamplingResult};
pub use rusticle::Complex;
