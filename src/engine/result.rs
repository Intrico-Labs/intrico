//! Execution result types
//!
//! Contains MeasurementResult for single-shot execution and
//! SamplingResult for multi-shot sampling.

use std::collections::HashMap;
use std::time::Duration;

use crate::core::{ClassicalRegister, QuantumState};

/// Execution time in milliseconds with 4-decimal precision
#[derive(Debug, Clone, Copy)]
pub struct ExecutionTime(f64);

impl ExecutionTime {
    pub fn from_duration(duration: Duration) -> Self {
        let ms = duration.as_secs_f64() * 1000.0;
        Self((ms * 10000.0).round() / 10000.0)
    }

    pub fn as_ms(&self) -> f64 {
        self.0
    }
}

impl std::fmt::Display for ExecutionTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.4}ms", self.0)
    }
}

/// Result of a single circuit execution
pub struct MeasurementResult {
    pub statevector: QuantumState,
    pub classical_register: ClassicalRegister,
    pub shots: usize,
    pub execution_time: ExecutionTime,
}

/// Result of sampling a circuit over multiple shots
pub struct SamplingResult {
    pub counts: HashMap<String, usize>,
    pub shots: usize,
    pub execution_time: ExecutionTime,
}
