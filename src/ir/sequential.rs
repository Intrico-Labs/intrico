use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{core::{circuit::GateOp}, ir::{IRMetadata}};

pub struct SequentialIR {
    n_qubits: usize,
    ops: Vec<GateOp>,
    pub metadata: IRMetadata
}

impl SequentialIR {
    pub fn new(n_qubits: usize, ops: Vec<GateOp>) -> Self {
        Self {
            n_qubits,
            ops,
            metadata: IRMetadata::default()
        }
    }

    pub fn n_qubits(&self) -> usize {
        self.n_qubits
    }
}

impl Display for SequentialIR {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {

        writeln!(f, "CircuitIR (Sequential)")?;
        writeln!(f, "Qubits: {}", self.n_qubits)?;
        writeln!(f, "Operations:")?;

        if self.ops.is_empty() {
            writeln!(f, "   (no operations)")?;
        } else {
            for (idx, op) in self.ops.iter().enumerate() {
                let gate_name = op.gate().name().as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or("Custom");

                let targets: Vec<String> = op.targets().iter()
                    .map(|t| t.to_string())
                    .collect();
                let controls: Vec<String> = op.controls().iter()
                    .map(|c| c.to_string())
                    .collect();

                let params = op.gate().params();
                let metadata = op.metadata();

                writeln!(f, "   {{ ({idx}) Gate: {gate_name} Params: {params:?} Controls: {controls:?} Targets: {targets:?} Metadata: {metadata:?} }}")?;
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