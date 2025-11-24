use std::collections::{HashMap, BTreeMap};

use crate::core::Amplitude;

/// This file contains all the important result structs for systematic output

/// The backend result is returned on executing a backend
#[derive(Debug)]
pub struct BackendResult {
    final_state: Vec<Amplitude>,
    pub metrics: ExecutionMetrics,
}

impl BackendResult {
    pub fn new(
        final_state: Vec<Amplitude>,
        metrics: ExecutionMetrics,
    ) -> Self {
        Self {
            final_state,
            metrics,
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
    counts: BTreeMap<String, usize>,
    pub shots: usize,
    pub execution_time: u128,
    rng_seed: u64,
}

impl SampleResult {
    pub fn new(counts: BTreeMap<String, usize>, shots: usize, execution_time: u128, rng_seed: u64) -> Self {
        Self {
            counts,
            shots,
            execution_time,
            rng_seed,
        }
    }

    pub fn counts(&self) -> &BTreeMap<String, usize> {
        &self.counts
    }
}