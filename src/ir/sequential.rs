//! Sequential circuit IR representation.
//!
//! This module defines the sequential IR variant where operations are
//! executed in a strict linear order without parallelism.

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{core::{circuit::GateOp}, ir::IRMetadata};

/// Sequential circuit intermediate representation.
///
/// Represents circuits as a linear sequence of gate operations with metadata.
pub struct SequentialIR {
    num_qubits: usize,
    ops: Vec<GateOp>,
    pub metadata: IRMetadata
}

impl SequentialIR {
    /// Creates a new sequential IR.
    pub fn new(num_qubits: usize, ops: Vec<GateOp>) -> Self {
        Self {
            num_qubits,
            ops,
            metadata: IRMetadata::default()
        }
    }

    /// Returns the number of qubits.
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the gate operations.
    pub fn ops(&self) -> &Vec<GateOp> {
        &self.ops
    }
}

impl Display for SequentialIR {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {

        writeln!(f, "CircuitIR (Sequential)")?;
        writeln!(f, "Qubits: {}", self.num_qubits)?;
        writeln!(f, "Operations:")?;

        if self.ops.is_empty() {
            writeln!(f, "   (no operations)")?;
        } else {
            for (idx, op) in self.ops.iter().enumerate() {
                let gate_name = op.gate();

                let targets: Vec<String> = op.targets().iter()
                    .map(|t| t.to_string())
                    .collect();
                let controls: Vec<String> = op.controls().iter()
                    .map(|c| c.to_string())
                    .collect();

                // let params = op.gate().params();
                let metadata = op.metadata();

                writeln!(f, "   {{ ({idx}) Gate: {:?} Controls: {controls:?} Targets: {targets:?} Metadata: {metadata:?} }}", gate_name)?;
            }
        }

        writeln!(f, "Metadata:")?;
        writeln!(f, "   Version: {}", self.metadata.version)?;
        writeln!(f, "   Name: {}", self.metadata.name)?;
        writeln!(f, "   Depth: {:?}", self.metadata.depth)?;
        writeln!(f, "   Tags: {:?}", self.metadata.tags)?;
        writeln!(f, "   Created at: {}", self.metadata.created_at)?;

        Ok(())
    }
}