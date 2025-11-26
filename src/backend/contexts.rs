//! Execution context and precomputed gate structures.
//!
//! This module provides the runtime execution environment for quantum circuits,
//! including the statevector state and precomputed gate matrices for efficient execution.

use std::{sync::Arc};

use crate::{backend::{CompiledCircuit, NativeOp}, core::{Amplitude}};

/// Precomputed gate matrix for fast execution.
///
/// Gates are precomputed during the preparation phase to avoid redundant
/// matrix calculations during execution. This enum stores gate matrices
/// as fixed-size arrays for optimal performance.
#[derive(Clone)]
pub enum PrecomputedGate {
    /// Single-qubit gate 
    OneQ([Amplitude; 4]),
    /// Two-qubit gate
    TwoQ([Amplitude; 16]),
    /// No precomputation needed (e.g., measurement operations)
    None,
}

/// Runtime execution context for quantum circuit simulation.
///
/// The `ExecutionContext` encapsulates all state required for executing a compiled
/// quantum circuit. It is created during the preparation phase and maintains:
/// - The quantum statevector
/// - Precomputed gate matrices
/// - Reference to the compiled circuit
/// - Optional RNG seed for reproducibility
pub struct ExecutionContext {
    compiled_ref: Arc<CompiledCircuit>,
    statevector: Vec<Amplitude>,
    pub(crate) precomputed: Vec<PrecomputedGate>,
    rng_seed: Option<usize>,
}

impl ExecutionContext {
    /// Creates a new execution context with initialized statevector.
    ///
    /// Initializes the statevector to |00...0⟩ state.
    pub fn new(compiled_circuit: Arc<CompiledCircuit>, rng_seed: Option<usize>) -> Self {

        let dimension = 1 << compiled_circuit.num_qubits();

        // create new statevector and set to |00...0⟩
        let mut statevector = vec![Amplitude::new(0.0, 0.0); dimension];
        statevector[0] = Amplitude::new(1.0, 0.0);


        ExecutionContext {
            compiled_ref: compiled_circuit,
            statevector,
            precomputed: vec![],
            rng_seed
        }
    }

    /// Returns the compiled circuit reference.
    pub fn compiled_ref(&self) -> &Arc<CompiledCircuit> {
        &self.compiled_ref
    }

    /// Returns the native operations from the compiled circuit.
    pub fn compiled_ops(&self) -> &[NativeOp] {
        self.compiled_ref.ops()
    }

    /// Returns a mutable reference to the statevector.
    pub fn statevector(&mut self) -> &mut [Amplitude] {
        self.statevector.as_mut_slice()
    }

    /// Returns the precomputed gate matrices.
    pub fn precomputed(&self) -> &[PrecomputedGate] {
        &self.precomputed
    }

    /// Returns the RNG seed if set.
    pub fn rng_seed(&self) -> Option<usize> {
        self.rng_seed
    }
}