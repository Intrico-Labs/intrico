//! ## The Backend Layer of Intrico
//! This layer contains all the backend related stuff for simulating Quantum Circuits
//!
//! ### Available Backends
//!
//! | Backend | Description | State Representation |
//! |---------|-------------|----------------------|
//! | **StatevectorBackend** | Dense statevector simulation with kernel-based parameterized gates | Dense vector (2^n amplitudes) |
//! 

pub mod traits;
pub mod results;
pub mod statevector;
pub mod contexts;
pub mod config;
pub mod kernels;
pub mod compiled_circuit;
pub mod nativeop;
pub mod meas_op;

pub use config::BackendConfig;
pub use results::{BackendResult, SampleResult};
pub use traits::QuantumBackend;
pub use statevector::StatevectorBackend;
pub use contexts::ExecutionContext;
pub use compiled_circuit::CompiledCircuit;
pub use nativeop::{NativeOp, ExecutableGate};
pub use meas_op::MeasurementOp;