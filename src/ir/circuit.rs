use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::ir::SequentialIR;

pub enum CircuitIR {
    Sequential(SequentialIR),
}

impl CircuitIR {
    pub fn save_json(&self) {
        todo!()
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

