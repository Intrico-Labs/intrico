//! Circuit IR interface
//!
//! This module defines the top-level circuit IR enum that wraps different
//! IR variants (sequential, parallel, etc.) with a common interface.

use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{core::circuit::GateOp, ir::SequentialIR};

/// Unified circuit intermediate representation.
///
/// Wraps different IR variants and provides a common interface for
/// backend-agnostic circuit manipulation.
pub enum CircuitIR {
    Sequential(SequentialIR),
}

impl CircuitIR {
    /// Saves the IR to JSON format.
    pub fn save_json(&self) {
        todo!()
    }

    /// Returns the number of qubits in the circuit.
    pub fn num_qubits(&self) -> usize {
        match self {
            CircuitIR::Sequential(ir) => ir.num_qubits(),
        }
    }

    /// Returns the gate operations in the circuit.
    pub fn ops(&self) -> &Vec<GateOp> {
        match self {
            CircuitIR::Sequential(ir) => ir.ops(),
        }
    }
}

impl Display for CircuitIR {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CircuitIR::Sequential(ir) => {
                writeln!(f, "{}", ir)?;
                Ok(())
            }
        }
    }
}

