use smallvec::SmallVec;

use crate::core::{QuantumGate};

#[derive(Clone)]
pub struct GateOp {
    gate: QuantumGate,
    targets: SmallVec<[usize; 2]>,
    controls: SmallVec<[usize; 2]>,
    metadata: GateMetadata,
}

#[derive(Debug, Default, Clone)]
pub struct GateMetadata {
    layer: Option<usize>,
}

impl GateOp {
    pub fn new(gate: QuantumGate, targets: &[usize], controls: &[usize]) -> Self {
        GateOp {
            gate,
            targets: SmallVec::from_slice(targets),
            controls: SmallVec::from_slice(controls),
            metadata: GateMetadata::default()
        }
    }

    // Getters
    pub fn gate(&self) -> &QuantumGate {
        &self.gate
    }

    pub fn targets(&self) -> &SmallVec<[usize; 2]> {
        &self.targets
    }

    pub fn controls(&self) -> &SmallVec<[usize; 2]> {
        &self.controls
    }

    pub fn metadata(&self) -> &GateMetadata {
        &self.metadata
    }
}

