use crate::{backend::{BackendConfig, BackendResult, CompiledCircuit, ExecutionContext, QuantumBackend}, ir::CircuitIR};

pub struct StatevectorBackend {
    config: BackendConfig,
    // capabilities (supports_simd, supports_gpu_accel, etc)
    // logging (enable_metrics, etc)
}

impl QuantumBackend for StatevectorBackend {

    fn transpile(&self, circuit_ir: &CircuitIR) -> CompiledCircuit {
        todo!()
    }
    
    fn prepare(&self, compiled_ctx: &CompiledCircuit) -> ExecutionContext {
        todo!()
    }
    
    fn execute(&self, execution_ctx: &mut ExecutionContext) -> BackendResult {
        todo!()
    }
    
}