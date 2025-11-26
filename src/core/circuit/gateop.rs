//! Gate operation representation.
//!
//! This module defines the gate operation structure that holds a gate
//! with target and control qubits, along with metadata.

use smallvec::SmallVec;

use crate::core::{QuantumGate};

/// Gate operation with targets, controls, and metadata.
///
/// Represents a single gate application in a quantum circuit.
#[derive(Clone)]
pub struct GateOp {
    gate: QuantumGate,
    targets: SmallVec<[usize; 2]>,
    controls: SmallVec<[usize; 2]>,
    metadata: GateMetadata,
}

/// Metadata associated with a gate operation.
#[derive(Debug, Default, Clone)]
pub struct GateMetadata {
    layer: Option<usize>,
}

impl GateOp {
    /// Creates a new gate operation.
    pub(crate) fn new(gate: QuantumGate, targets: &[usize], controls: &[usize]) -> Self {
        GateOp {
            gate,
            targets: SmallVec::from_slice(targets),
            controls: SmallVec::from_slice(controls),
            metadata: GateMetadata::default()
        }
    }

    /// Returns the quantum gate.
    pub fn gate(&self) -> &QuantumGate {
        &self.gate
    }

    /// Returns the target qubit indices.
    pub fn targets(&self) -> &SmallVec<[usize; 2]> {
        &self.targets
    }

    /// Returns the control qubit indices.
    pub fn controls(&self) -> &SmallVec<[usize; 2]> {
        &self.controls
    }

    /// Returns the gate metadata.
    pub fn metadata(&self) -> &GateMetadata {
        &self.metadata
    }

    /// Sets the circuit layer for this operation.
    pub fn set_layer(&mut self, layer: usize) {
        self.metadata.layer = Some(layer);
    }

    /// Returns the circuit layer if set.
    pub fn get_layer(&self) -> Option<usize> {
        self.metadata.layer
    }
}

