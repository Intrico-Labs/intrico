//! Result structures for backend execution and sampling.
//!
//! This module defines the result types returned by backend operations,
//! including execution results and measurement sampling results.

use std::collections::BTreeMap;

use crate::core::Amplitude;

/// Result of backend circuit execution.
///
/// Contains the final quantum statevector and execution metrics.
#[derive(Debug)]
pub struct BackendResult {
    final_state: Vec<Amplitude>,
    pub metrics: ExecutionMetrics,
}

impl BackendResult {
    /// Creates a new backend result.
    pub(crate) fn new(
        final_state: Vec<Amplitude>,
        metrics: ExecutionMetrics,
    ) -> Self {
        Self {
            final_state,
            metrics,
        }
    }

    /// Returns the final statevector after execution.
    pub fn final_state(&self) -> &[Amplitude] {
        &self.final_state
    }
}

/// Performance metrics from circuit execution.
#[derive(Debug)]
pub struct ExecutionMetrics {
    /// Execution time in nanoseconds.
    pub execution_time: u128,
    /// Number of operations applied.
    pub ops_applied: usize
}

impl ExecutionMetrics {
    /// Creates new execution metrics.
    pub fn new(execution_time: u128, ops_applied: usize) -> Self {
        Self {
            execution_time,
            ops_applied
        }
    }
}

/// Result of measurement sampling.
///
/// Contains measurement counts for each observed bitstring.
#[derive(Debug)]
pub struct SampleResult {
    counts: BTreeMap<String, usize>,
    pub shots: usize,
    /// Execution time (in milliseconds)
    pub execution_time: f64,
    #[allow(dead_code)]
    rng_seed: u64,
}

impl SampleResult {
    /// Creates a new sample result.
    pub fn new(counts: BTreeMap<String, usize>, shots: usize, execution_time: f64, rng_seed: u64) -> Self {
        Self {
            counts,
            shots,
            execution_time,
            rng_seed,
        }
    }

    /// Returns the measurement counts for each bitstring.
    pub fn counts(&self) -> &BTreeMap<String, usize> {
        &self.counts
    }
}