//! QISA (Quantum Instruction Set Architecture) encoder.
//!
//! Converts a [`QuantumCircuit`] into a compact binary blob (`.qisa` file) that can
//! be transmitted to an [`intrico-node`] executor or stored on disk.
//!
//! # Binary layout
//!
//! ```text
//! [ Header | Constants pool | Instructions | Footer ]
//! ```
//!
//! - **Header** — qubit count, classical bit count, magic bytes.
//! - **Constants pool** — ordered `ConstEntry` values referenced by `const_index`.
//!   Used by parameterised gates (CP, Rx, Ry, Rz) to store `f64` rotation angles.
//! - **Instructions** — one entry per node in the circuit DAG.
//!   Always starts with `QInit` for each qubit, always ends with `QEnd`.
//! - **Footer** — checksum / end-of-file marker.
//!
//! # Supported gates
//!
//! | [`GateKind`] | QISA instruction | Theta recovery |
//! |---|---|---|
//! | H / X / Y / Z | `H` / `X` / `Y` / `Z` | — |
//! | CX | `CNOT` | — |
//! | Swap | `SWAP` | — |
//! | CP | `CPHASE { const_index }` | `atan2(imag, real)` of `matrix[15]` |
//! | Rx | `RX { const_index }` | `-imag(matrix[1]) × 2` |
//! | Ry | `RY { const_index }` | `asin(real(matrix[2])) × 2` |
//! | Rz | `RZ { const_index }` | `atan2(imag, real)` of `matrix[3]` × 2 |
//!
//! S, T, and Custom gates are not supported by QISA and return `Err`.

use qisa::core::{
    constant::{ConstEntry, ConstKind},
    footer::Footer,
    header::Header,
    instruction::Instruction,
    program::Program,
};

use intrico_core::{GateKind, Operation, QuantumCircuit};

/// Encodes a [`QuantumCircuit`] to QISA bytecode.
pub struct QisaEncoder {}

impl QisaEncoder {
    /// Compile `circuit` to a QISA binary blob.
    ///
    /// Returns the raw bytes of the `.qisa` program, which can be written to a file
    /// and executed by `intrico-node`.
    ///
    /// # Errors
    ///
    /// Returns `Err("Unsupported gate type")` if the circuit contains a gate that
    /// has no QISA opcode (currently: S, T, Custom).
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
                    GateKind::Rx => {
                        // Rx matrix[1] = -i·sin(θ/2), recover theta from imag of element [1]
                        let elem = &gate.matrix()[1];
                        let theta = -elem.imag() * 2.0;
                        let const_index = constants.len() as u64;
                        constants.push(ConstEntry { kind: ConstKind::F64(theta) });
                        instructions.push(Instruction::RX {
                            qubit: targets[0] as u32,
                            const_index,
                        });
                    }
                    GateKind::Ry => {
                        // Ry matrix[2] = sin(θ/2), recover theta from element [2]
                        let elem = &gate.matrix()[2];
                        let theta = elem.real().asin() * 2.0;
                        let const_index = constants.len() as u64;
                        constants.push(ConstEntry { kind: ConstKind::F64(theta) });
                        instructions.push(Instruction::RY {
                            qubit: targets[0] as u32,
                            const_index,
                        });
                    }
                    GateKind::Rz => {
                        // Rz matrix[3] = e^(iθ/2), recover theta from imag/real of element [3]
                        let elem = &gate.matrix()[3];
                        let theta = elem.imag().atan2(elem.real()) * 2.0;
                        let const_index = constants.len() as u64;
                        constants.push(ConstEntry { kind: ConstKind::F64(theta) });
                        instructions.push(Instruction::RZ {
                            qubit: targets[0] as u32,
                            const_index,
                        });
                    }
                    GateKind::Swap => {
                        instructions.push(Instruction::SWAP {
                            q1: targets[0] as u32,
                            q2: targets[1] as u32,
                        });
                    }
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
