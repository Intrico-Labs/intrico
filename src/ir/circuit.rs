use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::ir::SequentialIR;

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

