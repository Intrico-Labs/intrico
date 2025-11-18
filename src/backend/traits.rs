use crate::{backend::{BackendResult, SampleResult}, ir::CircuitIR};

/// The intrico backend trait that all backends must implement
pub trait IntricoBackend {
    /// Fit the circuit ir in the backend
    fn prepare(&mut self, circuit_ir: &CircuitIR);

    /// Execute the circuit once (destroys its state)
    fn run(&mut self, rng_seed: Option<u64>) -> BackendResult;

    /// Indestructible sampling
    fn sample(&mut self, shots: usize, rng_seed: Option<u64>) -> SampleResult;
}