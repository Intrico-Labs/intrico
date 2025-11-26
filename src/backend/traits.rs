//! Backend trait definition.
//!
//! This module defines the core trait that all quantum backends must implement,
//! providing a standard interface for circuit transpilation, preparation, and execution.

use std::sync::Arc;

use crate::{QuantumCircuit, SequentialCircuit, backend::{BackendResult, CompiledCircuit, ExecutionContext}, ir::CircuitIR};

/// Core trait for quantum circuit execution backends.
///
/// All backends must implement this trait to provide circuit transpilation,
/// preparation, and execution capabilities.
pub trait QuantumBackend {
    /// Transpiles a circuit IR into a backend-specific compiled circuit.
    fn transpile(&self, circuit_ir: &CircuitIR) -> CompiledCircuit;

    /// Prepares an execution context with initialized state and precomputed gates.
    fn prepare(&self, compiled: Arc<CompiledCircuit>, rnd_seed: Option<usize>) -> ExecutionContext;

    /// Executes the circuit and returns the result.
    fn execute(&self, execution_ctx: &mut ExecutionContext) -> BackendResult;

    /// High-level method that runs a circuit through the full pipeline.
    ///
    /// Converts the circuit to IR, transpiles, prepares, and executes in sequence.
    /// 
    /// # Arguments
    /// * `circuit` - Quantum circuit you want to run.
    fn run(&self, circuit: SequentialCircuit) -> BackendResult {
        let ir = circuit.to_ir();
        let compiled = self.transpile(&ir);
        let compiled = Arc::new(compiled);
        let mut ctx = self.prepare(compiled, None);
        self.execute(&mut ctx)
    }
}