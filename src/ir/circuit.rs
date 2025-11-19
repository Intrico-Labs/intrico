use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{core::circuit::GateOp, ir::SequentialIR};

pub enum CircuitIR {
    Sequential(SequentialIR),
}

impl CircuitIR {
    pub fn save_json(&self) {
        todo!()
    }

    pub fn num_qubits(&self) -> usize {
        match self {
            CircuitIR::Sequential(ir) => ir.num_qubits(),
        }
    }

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

