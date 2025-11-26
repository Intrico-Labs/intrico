use crate::backend::{MeasurementOp, NativeOp};
use std::fmt::Debug;

/// The CompiledCircuit gate that is returned after transpiling a Circuit IR
pub struct CompiledCircuit {
    num_qubits: usize,
    optimization_level: u8,
    ops: Vec<NativeOp>,
    measurements: Vec<MeasurementOp>,
    creg_size: usize,
    creg_mapping: Vec<(usize, usize)>,
    metadata: CompiledCircuitMetadata,
}


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

    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn optimization_level(&self) -> u8 {
        self.optimization_level
    }

    pub fn ops(&self) -> &Vec<NativeOp> {
        &self.ops
    }

    pub fn measurements(&self) -> &Vec<MeasurementOp> {
        &self.measurements
    }

    pub fn creg_size(&self) -> usize {
        self.creg_size
    }

    pub fn creg_mapping(&self) -> &Vec<(usize, usize)> {
        &self.creg_mapping
    }

    pub fn metadata(&self) -> &CompiledCircuitMetadata {
        &self.metadata
    }
}

impl Debug for CompiledCircuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledCircuit").field("num_qubits", &self.num_qubits).field("optimization_level", &self.optimization_level).field("ops", &self.ops).field("measurements", &self.measurements).field("creg_size", &self.creg_size).field("creg_mapping", &self.creg_mapping).field("metadata", &self.metadata).finish()
    }
}