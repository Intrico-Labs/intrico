use std::collections::HashMap;

use crate::core::Amplitude;

/// This file contains all the important result structs for systematic output

/// The backend result is returned on executing a backend
#[derive(Debug)]
pub struct BackendResult {
    final_state: Vec<Amplitude>,
    pub metrics: ExecutionMetrics,
    rng_seed: usize
}

impl BackendResult {
    pub fn new(
        final_state: Vec<Amplitude>,
        metrics: ExecutionMetrics,
        rng_seed: usize
    ) -> Self {
        Self {
            final_state,
            metrics,
            rng_seed
        }
    }

    pub fn final_state(&self) -> &[Amplitude] {
        &self.final_state
    }
}

#[derive(Debug)]
pub struct ExecutionMetrics {
    pub execution_time: u128,
    pub ops_applied: usize
}

impl ExecutionMetrics {
    pub fn new(execution_time: u128, ops_applied: usize) -> Self {
        Self {
            execution_time,
            ops_applied
        }
    }
}

/// The sample result is returned on sampling a circuit
#[derive(Debug)]
pub struct SampleResult {
    counts: HashMap<String, usize>,
    pub shots: usize,
    pub execution_time: u128
}

impl SampleResult {
    pub fn new(counts: HashMap<String, usize>, shots: usize, execution_time: u128) -> Self {
        Self {
            counts,
            shots,
            execution_time
        }
    }

    pub fn counts(&self) -> &HashMap<String, usize> {
        &self.counts
    }
}