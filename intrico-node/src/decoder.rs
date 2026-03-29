use std::f64::consts::FRAC_PI_2;

use intrico_core::QuantumCircuit;
use qisa::core::{instruction::Instruction, program::Program};

pub fn decode_program(program: &Program) -> Result<QuantumCircuit, String> {
    let num_qubits = program.header.logical_qubit_count as usize;
    let mut circuit = QuantumCircuit::new(num_qubits);

    for instr in &program.instructions {
        match instr {
            Instruction::QInit { .. } => {}
            Instruction::QEnd => break,
            Instruction::H { qubit } => {
                circuit.h(*qubit as usize);
            }
            Instruction::X { qubit } => {
                circuit.x(*qubit as usize);
            }
            Instruction::Y { qubit } => {
                circuit.y(*qubit as usize);
            }
            Instruction::Z { qubit } => {
                circuit.z(*qubit as usize);
            }
            Instruction::CNOT { control, target } => {
                circuit.cx(*control as usize, *target as usize);
            }
            Instruction::SWAP { q1, q2 } => {
                circuit.swap(*q1 as usize, *q2 as usize);
            }
            Instruction::CPHASE { q1, q2 } => {
                // QISA CPHASE carries no theta — defaulting to π/2
                // TODO: extend QISA CPHASE to carry a const_index for theta
                eprintln!(
                    "Warning: CPHASE decoded with default theta=π/2 \
                     (theta not stored in QISA format)"
                );
                circuit.cp(*q1 as usize, *q2 as usize, FRAC_PI_2);
            }
            Instruction::RX { qubit, .. }
            | Instruction::RY { qubit, .. }
            | Instruction::RZ { qubit, .. } => {
                return Err(format!(
                    "Parameterized rotation gate at qubit {} not yet supported",
                    qubit
                ));
            }
            Instruction::Measure { qubit, classical } => {
                circuit.measure(*qubit as usize, *classical as usize);
            }
            Instruction::MeasureAll => {
                circuit.measure_all();
            }
            Instruction::Barrier | Instruction::Wait { .. } => {}
        }
    }

    Ok(circuit)
}
