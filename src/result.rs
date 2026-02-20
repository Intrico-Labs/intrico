//! Execution result types
//!
//! Contains MeasurementResult for single-shot execution and
//! SamplingResult for multi-shot sampling.

use std::collections::HashMap;
use std::time::Duration;

use crate::creg::ClassicalRegister;
use crate::state::QuantumState;

/// Result of a single circuit execution
pub struct MeasurementResult {
    pub statevector: QuantumState,
    pub classical_register: ClassicalRegister,
    pub shots: usize,
    pub execution_time: Duration,
}

/// Result of sampling a circuit over multiple shots
pub struct SamplingResult {
    pub counts: HashMap<String, usize>,
    pub shots: usize,
    pub execution_time: Duration,
}
