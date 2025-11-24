use std::sync::Arc;

use crate::{QuantumCircuit, SequentialCircuit, backend::{BackendResult, CompiledCircuit, ExecutionContext}, ir::CircuitIR};

/// The intrico backend trait that all backends must implement
pub trait QuantumBackend {
    /// Transpile the circuit
    fn transpile(&self, circuit_ir: &CircuitIR) -> CompiledCircuit;

    /// Fit the circuit ir in the backend
    fn prepare(&self, compiled: Arc<CompiledCircuit>, rnd_seed: Option<usize>) -> ExecutionContext;

    /// Execute the job
    fn execute(&self, execution_ctx: &mut ExecutionContext) -> BackendResult;

    /// Run the circuit
    fn run(&self, circuit: SequentialCircuit) -> BackendResult {
        let ir = circuit.to_ir();
        let compiled = self.transpile(&ir);
        let compiled = Arc::new(compiled);
        let mut ctx = self.prepare(compiled, None);
        self.execute(&mut ctx)
    }
}