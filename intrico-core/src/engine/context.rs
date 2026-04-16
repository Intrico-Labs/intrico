//! Execution Context
//!
//! Handles quantum circuit execution on a quantum state,
//! including gate application and measurement collapse.

use std::cmp::{max, min};

use rand::{Rng, RngExt};
use rusticle::Complex;

use crate::circuit::{Operation, QuantumCircuit};
use crate::core::{ClassicalRegister, QuantumState};

/// Execution context that holds the mutable state for circuit execution
pub struct ExecutionContext {
    state: QuantumState,
    classical_register: ClassicalRegister,
}

impl ExecutionContext {
    /// Creates a new context with a fresh |0...0⟩ state
    pub fn new(num_qubits: usize, num_clbits: usize) -> Self {
        Self {
            state: QuantumState::new(num_qubits),
            classical_register: ClassicalRegister::new(num_clbits),
        }
    }

    /// Creates a context with a custom initial state
    pub fn with_state(state: QuantumState, num_clbits: usize) -> Self {
        Self {
            state,
            classical_register: ClassicalRegister::new(num_clbits),
        }
    }

    /// Executes only gate operations (skips measurements)
    pub fn run_gates_only(&mut self, circuit: &QuantumCircuit) -> &mut Self {
        let dim = 1 << circuit.num_qubits();

        for node in circuit.nodes() {
            if let Operation::Gate { gate, targets } = &node.operation {
                match gate.arity() {
                    1 => {
                        apply_single_qubit_gate(gate.matrix(), self.state.statevector_mut(), targets[0], dim);
                    }
                    2 => {
                        apply_two_qubit_gate(gate.matrix(), self.state.statevector_mut(), targets[0], targets[1], dim);
                    }
                    _ => {
                        panic!(
                            "Unsupported gate arity {}: only 1-qubit and 2-qubit gates are supported.",
                            gate.arity()
                        );
                    }
                }
            }
        }

        self
    }

    /// Executes the full circuit (gates + measurements) on this context's state
    pub fn run(&mut self, circuit: &QuantumCircuit) -> &mut Self {
        let dim = 1 << circuit.num_qubits();
        let mut rng = rand::rng();

        for node in circuit.nodes() {
            match &node.operation {
                Operation::Gate { gate, targets } => {
                    match gate.arity() {
                        1 => {
                            apply_single_qubit_gate(gate.matrix(), self.state.statevector_mut(), targets[0], dim);
                        }
                        2 => {
                            apply_two_qubit_gate(gate.matrix(), self.state.statevector_mut(), targets[0], targets[1], dim);
                        }
                        _ => {
                            panic!(
                                "Unsupported gate arity {}: only 1-qubit and 2-qubit gates are supported.",
                                gate.arity()
                            );
                        }
                    }
                }
                Operation::Measure { qubit, classical_bit } => {
                    let outcome = measure_qubit(self.state.statevector_mut(), *qubit, dim, &mut rng);
                    self.classical_register.set(*classical_bit, outcome as u8);
                }
            }
        }

        self
    }

    pub fn state(&self) -> &QuantumState {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut QuantumState {
        &mut self.state
    }

    pub fn classical_register(&self) -> &ClassicalRegister {
        &self.classical_register
    }

    /// Consumes the context and returns the state and classical register
    pub fn into_inner(self) -> (QuantumState, ClassicalRegister) {
        (self.state, self.classical_register)
    }
}

// Helper functions for statevector manipulation

pub(crate) fn measure_qubit(state: &mut [Complex], qubit: usize, dim: usize, rng: &mut impl Rng) -> usize {
    let stride = 1 << qubit;
    let period = stride << 1;

    // Calculate probability of measuring |0⟩
    let mut prob_zero = 0.0;
    let mut idx = 0;
    while idx < dim {
        let limit = idx + stride;
        let mut i0 = idx;
        while i0 < limit {
            prob_zero += state[i0].norm_squared();
            i0 += 1;
        }
        idx += period;
    }

    // Sample outcome
    let r: f64 = rng.random();
    let outcome = if r < prob_zero { 0 } else { 1 };

    // Collapse and renormalize
    let prob = if outcome == 0 { prob_zero } else { 1.0 - prob_zero };
    let inv_norm = Complex::new(1.0 / prob.sqrt(), 0.0);
    let zero = Complex::new(0.0, 0.0);

    idx = 0;
    while idx < dim {
        let limit = idx + stride;
        let mut i0 = idx;
        while i0 < limit {
            let i1 = i0 + stride;
            if outcome == 0 {
                state[i0] = state[i0] * inv_norm;
                state[i1] = zero;
            } else {
                state[i0] = zero;
                state[i1] = state[i1] * inv_norm;
            }
            i0 += 1;
        }
        idx += period;
    }

    outcome
}

pub(crate) fn apply_single_qubit_gate(matrix: &[Complex], state: &mut [Complex], target: usize, dim: usize) {
    let stride = 1 << target;
    let period = stride << 1;

    let mut idx = 0;
    while idx < dim {
        let limit = idx + stride;
        let mut i0 = idx;

        while i0 < limit {
            let i1 = i0 + stride;
            let a0 = state[i0];
            let a1 = state[i1];

            state[i0] = matrix[0] * a0 + matrix[1] * a1;
            state[i1] = matrix[2] * a0 + matrix[3] * a1;

            i0 += 1;
        }
        idx += period;
    }
}

pub(crate) fn apply_two_qubit_gate(matrix: &[Complex], state: &mut [Complex], control: usize, target: usize, dim: usize) {
    let a = min(control, target);
    let b = max(control, target);

    let bit_t = 1 << target;
    let bit_c = 1 << control;

    let specs = dim / 4;

    for k in 0..specs {
        let mask1 = (1 << a) - 1;
        let temp = ((k & !mask1) << 1) | (k & mask1);

        let mask2 = (1 << b) - 1;
        let base = ((temp & !mask2) << 1) | (temp & mask2);

        let i00 = base;
        let i01 = base | bit_t;
        let i10 = base | bit_c;
        let i11 = base | bit_c | bit_t;

        let (a0, a1) = (state[i00], state[i01]);
        let (a2, a3) = (state[i10], state[i11]);

        state[i00] = matrix[0] * a0 +
            matrix[1] * a1 +
            matrix[2] * a2 +
            matrix[3] * a3;

        state[i01] = matrix[4] * a0 +
            matrix[5] * a1 +
            matrix[6] * a2 +
            matrix[7] * a3;

        state[i10] = matrix[8] * a0 +
            matrix[9] * a1 +
            matrix[10] * a2 +
            matrix[11] * a3;

        state[i11] = matrix[12] * a0 +
            matrix[13] * a1 +
            matrix[14] * a2 +
            matrix[15] * a3;
    }
}
