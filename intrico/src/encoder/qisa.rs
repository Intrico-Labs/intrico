use qisa::core::{
    constant::{ConstEntry, ConstKind},
    footer::Footer,
    header::Header,
    instruction::Instruction,
    program::Program,
};

use intrico_core::{GateKind, Operation, QuantumCircuit};

pub struct QisaEncoder {}

impl QisaEncoder {
    pub fn encode(circuit: &QuantumCircuit) -> Result<Vec<u8>, &'static str> {
        let header = Header::new(circuit.num_qubits() as u32, circuit.classical_regs() as u32);

        let mut constants: Vec<ConstEntry> = Vec::new();
        let mut instructions: Vec<Instruction> = Vec::new();
        let footer = Footer::new();

        // ---------------------------
        // Instructions Parsing
        // ---------------------------

        // QInit instructions for every qubit initialisation
        for q in 0..circuit.num_qubits() {
            instructions.push(Instruction::QInit { qubit: q as u32 });
        }

        for i in circuit.nodes() {
            let instr = &i.operation;
            match instr {
                Operation::Gate { gate, targets } => match gate.kind() {
                    GateKind::H => {
                        instructions.push(Instruction::H {
                            qubit: targets[0] as u32,
                        });
                    }
                    GateKind::X => {
                        instructions.push(Instruction::X {
                            qubit: targets[0] as u32,
                        });
                    }
                    GateKind::Y => {
                        instructions.push(Instruction::Y {
                            qubit: targets[0] as u32,
                        });
                    }
                    GateKind::Z => {
                        instructions.push(Instruction::Z {
                            qubit: targets[0] as u32,
                        });
                    }
                    // TODO: add QISA support for these
                    // GateKind::S => {
                    //     instructions.push(Instruction::S {
                    //         qubit: targets[0] as u32,
                    //     });
                    // }
                    // GateKind::T => {
                    //     instructions.push(Instruction::T {
                    //         qubit: targets[0] as u32,
                    //     });
                    // }
                    GateKind::CX => {
                        instructions.push(Instruction::CNOT {
                            control: targets[0] as u32,
                            target: targets[1] as u32,
                        });
                    }
                    GateKind::CP => {
                        // CP matrix[15] = e^(i*theta) = cos(theta) + i*sin(theta)
                        let elem = &gate.matrix()[15];
                        let theta = elem.imag().atan2(elem.real());
                        let const_index = constants.len() as u64;
                        constants.push(ConstEntry { kind: ConstKind::F64(theta) });
                        instructions.push(Instruction::CPHASE {
                            q1: targets[0] as u32,
                            q2: targets[1] as u32,
                            const_index,
                        });
                    }
                    // GateKind::Swap => todo!(),
                    // GateKind::Custom => todo!(),
                    _ => return Err("Unsupported gate type"),
                },
                Operation::Measure {
                    qubit,
                    classical_bit,
                } => {
                    instructions.push(Instruction::Measure {
                        qubit: *qubit as u32,
                        classical: *classical_bit as u32,
                    });
                }
            }
        }

        // QEnd instruction marking end of circuit
        instructions.push(Instruction::QEnd);

        let program = Program {
            header,
            constants,
            instructions,
            footer,
        };

        let bytes = program.compile_to_bytes();

        Ok(bytes)
    }
}
