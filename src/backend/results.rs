use crate::core::Amplitude;

/// This file contains all the important result structs for systematic output

/// The backend result is returned on executing a backend
pub struct BackendResult {
    counts: Vec<String>,
    final_state: Vec<Amplitude>,
    metrics: ExecutionMetrics,
    rng_seed: usize
}

impl BackendResult {
    pub fn new(
        counts: Vec<String>,
        final_state: Vec<Amplitude>,
        metrics: ExecutionMetrics,
        rng_seed: usize
    ) -> Self {
        Self {
            counts,
            final_state,
            metrics,
            rng_seed
        }
    }

    pub fn final_state(&self) -> &[Amplitude] {
        &self.final_state
    }
}

pub struct ExecutionMetrics {
    execution_time: f64,
    ops_applied: usize
}

impl ExecutionMetrics {
    pub fn new(execution_time: f64, ops_applied: usize) -> Self {
        Self {
            execution_time,
            ops_applied
        }
    }
}

/// The sample result is returned on sampling a circuit
pub struct SampleResult {

}