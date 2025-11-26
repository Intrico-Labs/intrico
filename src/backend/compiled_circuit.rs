//! Compiled circuit representation for backend execution.
//!
//! This module contains the compiled circuit structure that is produced after
//! transpiling a high-level circuit IR into a backend-specific executable format.

use crate::backend::{MeasurementOp, NativeOp};
use std::fmt::Debug;

/// Backend-specific compiled circuit representation.
///
/// A `CompiledCircuit` is created during the transpilation phase by a backend's
/// `transpile()` method. It contains all the information needed for execution:
/// - Native operations (gates) ready for execution
/// - Measurement operations mapped to classical registers
/// - Circuit metadata and optimization level
pub struct CompiledCircuit {
    num_qubits: usize,
    optimization_level: u8,
    ops: Vec<NativeOp>,
    measurements: Vec<MeasurementOp>,
    creg_size: usize,
    creg_mapping: Vec<(usize, usize)>,
    metadata: CompiledCircuitMetadata,
}


/// Metadata associated with a compiled circuit.
#[derive(Debug)]
pub struct CompiledCircuitMetadata {
    #[allow(dead_code)]
    depth: Option<usize>
}

impl CompiledCircuitMetadata {
    pub fn new(depth: Option<usize>) -> Self {
        Self {
            depth
        }
    }
}

impl CompiledCircuit {
    /// Creates a new compiled circuit.
    ///
    /// # Arguments
    /// * `num_qubits` - Number of qubits in the circuit
    /// * `optimization_level` - Optimization level applied (0-2)
    /// * `ops` - Vector of native operations to execute
    /// * `measurements` - Measurement operations
    /// * `creg_size` - Size of the classical register
    /// * `creg_mapping` - Mapping from classical register to qubit indices
    /// * `metadata` - Circuit compilation metadata
    pub fn new(
        num_qubits: usize,
        optimization_level: u8,
        ops: Vec<NativeOp>,
        measurements: Vec<MeasurementOp>,
        creg_size: usize,
        creg_mapping: Vec<(usize, usize)>,
        metadata: CompiledCircuitMetadata,
    ) -> Self {
        Self {
            num_qubits,
            optimization_level,
            ops,
            measurements,
            creg_size,
            creg_mapping,
            metadata,
        }
    }

    /// Returns the number of qubits in the circuit.
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the optimization level applied during compilation.
    pub fn optimization_level(&self) -> u8 {
        self.optimization_level
    }

    /// Returns a reference to the vector of native operations.
    pub fn ops(&self) -> &Vec<NativeOp> {
        &self.ops
    }

    /// Returns a reference to the measurement operations.
    pub fn measurements(&self) -> &Vec<MeasurementOp> {
        &self.measurements
    }

    /// Returns the size of the classical register.
    pub fn creg_size(&self) -> usize {
        self.creg_size
    }

    /// Returns the mapping from classical register indices to qubit indices.
    pub fn creg_mapping(&self) -> &Vec<(usize, usize)> {
        &self.creg_mapping
    }

    /// Returns the circuit metadata.
    pub fn metadata(&self) -> &CompiledCircuitMetadata {
        &self.metadata
    }
}

impl Debug for CompiledCircuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledCircuit").field("num_qubits", &self.num_qubits).field("optimization_level", &self.optimization_level).field("ops", &self.ops).field("measurements", &self.measurements).field("creg_size", &self.creg_size).field("creg_mapping", &self.creg_mapping).field("metadata", &self.metadata).finish()
    }
}