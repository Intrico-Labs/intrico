use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::core::circuit::GateOp;

pub enum CircuitIR {
    Sequential(SequentialIR),
}

impl CircuitIR {
    pub fn n_qubits(&self) -> usize {
        match self {
            CircuitIR::Sequential(ir) => ir.n_qubits,
        }
    }

    pub fn save_json(&self) {
        todo!()
    }
}

impl Display for CircuitIR {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CircuitIR::Sequential(ir) => {
                writeln!(f, "CircuitIR (Sequential)")?;
                writeln!(f, "n_qubits: {}", ir.n_qubits)?;
                writeln!(f, "operations:")?;

                if ir.ops.is_empty() {
                    writeln!(f, "   (no operations)")?;
                } else {
                    for (idx, op) in ir.ops.iter().enumerate() {
                        let gate_name = op.gate().name().as_ref()
                            .map(|s| s.as_str())
                            .unwrap_or("Custom");

                        let targets: Vec<String> = op.targets().iter()
                            .map(|t| t.to_string())
                            .collect();
                        let controls: Vec<String> = op.controls().iter()
                            .map(|c| c.to_string())
                            .collect();

                        write!(f, "   {}: {}", idx + 1, gate_name)?;

                        if !controls.is_empty() {
                            write!(f, " (ctrl: {})", controls.join(", "))?;
                        }

                        writeln!(f, " -> target: {}", targets.join(", "))?;
                    }
                }
                Ok(())
            }
        }
    }
}

pub struct SequentialIR {
    n_qubits: usize,
    ops: Vec<GateOp>
}

impl SequentialIR {
    pub fn new(n_qubits: usize, ops: Vec<GateOp>) -> Self {
        Self {
            n_qubits,
            ops
        }
    }
}
