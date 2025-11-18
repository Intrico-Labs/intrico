use crate::{backend::{BackendResult, CompiledCircuit, ExecutionContext}, ir::CircuitIR};

/// The intrico backend trait that all backends must implement
pub trait QuantumBackend {
    /// Transpile the circuit
    fn transpile(&self, circuit_ir: &CircuitIR) -> CompiledCircuit;

    /// Fit the circuit ir in the backend
    fn prepare(&self, compiled: &CompiledCircuit) -> ExecutionContext;

    /// Execute the job
    fn execute(&self, execution_ctx: &mut ExecutionContext) -> BackendResult;

    /// Run the circuit
    fn run(&self, ir: CircuitIR) -> BackendResult {
        let compiled = self.transpile(&ir);
        let mut ctx = self.prepare(&compiled);
        self.execute(&mut ctx)
    }
}