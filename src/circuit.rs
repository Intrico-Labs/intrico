//! Quantum Circuit representation
//! 
//! This module contains the quantum circuit definitions and the relevant
//! implementations for executing a quantum circuit on a specific quantum state
//! 
//! The architecture of the quantum circuit that is built is purely graphical -
//! specifically a DAG to keep it memory efficient and support parallelism

use std::cmp::{max, min};
use std::collections::HashMap;
use std::time::Instant;

use rand::{Rng, RngExt};
use rusticle::Complex;

use crate::{creg::ClassicalRegister, gate::QuantumGate, result::{MeasurementResult, SamplingResult}, state::QuantumState};

/// Quantum Circuit - A graph representation of a quantum circuit
pub struct QuantumCircuit {
    num_qubits: usize,
    nodes: Vec<QuantumNode>,
    frontier: Vec<Option<usize>>,
}

/// Quantum Node - represents a node in the graph which holds a quantum operation
pub struct QuantumNode {
    pub node_id: usize,
    pub operation: Operation,
    pub parents: Vec<usize>,
    pub children: Vec<usize>
}

/// Operation - Holds a quantum operation (a gate operation or a measurement operation)
#[derive(Clone)]
pub enum Operation {
    Gate { gate: QuantumGate, targets: Vec<usize> },
    Measure { qubit: usize, classical_bit: usize },
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            nodes: Vec::new(),
            frontier: vec![None; num_qubits]
        }
    }

    fn add_node(&mut self, operation: Operation) -> &mut Self {
        let node_id = self.nodes.len();
        let mut parents: Vec<usize> = vec![];

        // Determine which qubits this operation touches
        let qubits: Vec<usize> = match &operation {
            Operation::Gate { targets, .. } => targets.clone(),
            Operation::Measure { qubit, .. } => vec![*qubit],
        };

        // Finding parents and updating frontier for each qubit
        for &q in &qubits {
            if let Some(i) = self.frontier[q] {
                if !parents.contains(&i) {
                    parents.push(i);
                }
            }
            self.frontier[q] = Some(node_id);
        }

        // Register this node as a child of each parent
        for &parent_id in &parents {
            self.nodes[parent_id].children.push(node_id);
        }

        let node = QuantumNode {
            node_id,
            operation,
            parents,
            children: Vec::new()
        };

        self.nodes.push(node);

        self
    }

    pub fn add_gate(&mut self, targets: Vec<usize>, gate: QuantumGate) -> &mut Self {
        for &t in &targets {
            if t >= self.num_qubits {
                panic!("Target qubit index {} out of bounds for {}-qubit circuit.", t, self.num_qubits);
            }
        }
        self.add_node(Operation::Gate { gate, targets })
    }

    pub fn append(&mut self, circuit: &QuantumCircuit) -> &mut Self {
        if circuit.num_qubits != self.num_qubits {
            panic!("Cannot append circuits with different qubit counts.")
        }

        for node in circuit.nodes() {
            self.add_node(node.operation.clone());
        }

        self
    }

    /// Execution Engine
    /// This basically holds all the logic for executing a quantum circuit on a given quantum state

    pub fn execute_on_state(&self, state: &mut QuantumState) -> ClassicalRegister {
        let operations = self.nodes();
        let dim = 1 << self.num_qubits;
        let creg_size = self.classical_register_size();
        let mut creg = ClassicalRegister::new(creg_size);
        let mut rng = rand::rng();

        for node in operations {
            match &node.operation {
                Operation::Gate { gate, targets } => {
                    match gate.arity() {
                        1 => {
                            apply_single_qubit_gate(gate.matrix(), state.statevector_mut(), targets[0], dim);
                        }
                        2 => {
                            apply_two_qubit_gate(gate.matrix(), state.statevector_mut(), targets[0], targets[1], dim);
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
                    let outcome = measure_qubit(state.statevector_mut(), *qubit, dim, &mut rng);
                    creg.set(*classical_bit, Some(outcome));
                }
            }
        }

        creg
    }

    pub fn execute(&self) -> MeasurementResult {
        let start = Instant::now();
        let mut state = QuantumState::new(self.num_qubits);
        let classical_register = self.execute_on_state(&mut state);
        let execution_time = start.elapsed();

        MeasurementResult {
            statevector: state,
            classical_register,
            shots: 1,
            execution_time,
        }
    }

    pub fn sample(&self, shots: usize) -> SamplingResult {
        let start = Instant::now();
        let mut counts: HashMap<String, usize> = HashMap::new();

        for _ in 0..shots {
            let mut state = QuantumState::new(self.num_qubits);
            let creg = self.execute_on_state(&mut state);
            *counts.entry(creg.bitstring()).or_insert(0) += 1;
        }

        let execution_time = start.elapsed();

        SamplingResult {
            counts,
            shots,
            execution_time,
        }
    }

    fn classical_register_size(&self) -> usize {
        self.nodes.iter()
            .filter_map(|n| match &n.operation {
                Operation::Measure { classical_bit, .. } => Some(classical_bit + 1),
                _ => None,
            })
            .max()
            .unwrap_or(0)
    }

    /// Getters
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn nodes(&self) -> &Vec<QuantumNode> {
        &self.nodes
    }

    pub fn frontier(&self) -> &Vec<Option<usize>> {
        &self.frontier
    }

    /// Builder functions
    /// These are helper functions that help build gates into circuits easily
    pub fn x(&mut self, target: usize) -> &mut Self {
        if target >= self.num_qubits {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::x());
        self
    }

    pub fn y(&mut self, target: usize) -> &mut Self {
        if target >= self.num_qubits {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::y());
        self
    }

    pub fn z(&mut self, target: usize) -> &mut Self {
        if target >= self.num_qubits {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::z());
        self
    }

    pub fn s(&mut self, target: usize) -> &mut Self {
        if target >= self.num_qubits {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::s());
        self
    }

    pub fn t(&mut self, target: usize) -> &mut Self {
        if target >= self.num_qubits {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::t());
        self
    }

    pub fn h(&mut self, target: usize) -> &mut Self {
        if target >= self.num_qubits {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::h());
        self
    }

    pub fn cx(&mut self, control: usize, target: usize) -> &mut Self {
        if control >= self.num_qubits || target >= self.num_qubits {
            panic!("Control or target qubit index out of bounds.")
        }
        if control == target {
            panic!("Control and target qubits must be different.")
        }
        self.add_gate(vec![control, target], QuantumGate::cx());
        self
    }

    pub fn cp(&mut self, control: usize, target: usize, theta: f64) -> &mut Self {
        if control >= self.num_qubits || target >= self.num_qubits {
            panic!("Control or target qubit index out of bounds.")
        }
        if control == target {
            panic!("Control and target qubits must be different.")
        }
        self.add_gate(vec![control, target], QuantumGate::cp(theta));
        self
    }

    pub fn swap(&mut self, qubit1: usize, qubit2: usize) -> &mut Self {
        if qubit1 >= self.num_qubits || qubit2 >= self.num_qubits {
            panic!("Qubit index out of bounds.")
        }
        if qubit1 == qubit2 {
            panic!("Qubits must be different.")
        }
        self.add_gate(vec![qubit1, qubit2], QuantumGate::swap());
        self
    }

    /// Measurement builder functions

    pub fn measure(&mut self, qubit: usize, classical_bit: usize) -> &mut Self {
        if qubit >= self.num_qubits {
            panic!("Qubit index out of bounds.")
        }
        self.add_node(Operation::Measure { qubit, classical_bit })
    }

    pub fn measure_all(&mut self) -> &mut Self {
        for i in 0..self.num_qubits {
            self.add_node(Operation::Measure { qubit: i, classical_bit: i });
        }
        self
    }
}

/// Helper functions
/// These are essential helper functions used in above implementations

fn measure_qubit(state: &mut [Complex], qubit: usize, dim: usize, rng: &mut impl Rng) -> usize {
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

fn apply_single_qubit_gate(matrix: &[Complex], state: &mut [Complex], target: usize, dim: usize) {
    let stride = 1 << target;
    let period = stride << 1;
    
    let mut idx = 0;
    while idx < dim {
        let limit = idx + stride;
        let mut i0 = idx;

        while i0 < limit {
            let i1 = i0 + stride;
            let a0 = state[i0].clone();
            let a1 = state[i1].clone();

            state[i0] = matrix[0] * a0 + matrix[1] * a1;
            state[i1] = matrix[2] * a0 + matrix[3] * a1;

            i0 += 1;
        }
        idx += period;
    }
}

fn apply_two_qubit_gate(matrix: &[Complex], state: &mut [Complex], control: usize, target: usize, dim: usize) {

    let a = min(control, target);
    let b = max(control, target);

    let bit_t = 1 << target;
    let bit_c = 1 << control;


    let specs = dim/4;

    for k in 0..specs {
        let mask1 = (1 << a) - 1;
        let temp = ((k & !mask1) << 1) | (k & mask1);

        let mask2 = (1 << b) - 1;
        let base = ((temp & !mask2) << 1) | (temp & mask2);

        let i00 = base; // 00 state
        let i01 = base | bit_t; // 01 state
        let i10 = base | bit_c; // 10 state
        let i11 = base | bit_c | bit_t; // 11 state

        // caching current values
        let (a0, a1) = (state[i00], state[i01]);
        let (a2, a3) = (state[i10], state[i11]);

        // multiplying matrix
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_creation() {
        let qc = QuantumCircuit::new(3);
        assert_eq!(qc.num_qubits(), 3);
        assert_eq!(qc.nodes().len(), 0);
    }

    #[test]
    fn test_valid_gate_operations() {
        let mut qc = QuantumCircuit::new(2);
        qc.h(0);
        qc.cx(0, 1);
        assert_eq!(qc.nodes().len(), 2);
    }

    #[test]
    #[should_panic(expected = "Target qubit index out of bounds")]
    fn test_x_gate_out_of_bounds() {
        let mut qc = QuantumCircuit::new(2);
        qc.x(2);
    }

    #[test]
    #[should_panic(expected = "Target qubit index out of bounds")]
    fn test_single_gate_on_zero_qubit_circuit() {
        let mut qc = QuantumCircuit::new(0);
        qc.x(0);
    }

    #[test]
    #[should_panic(expected = "Target qubit index")]
    fn test_add_gate_out_of_bounds() {
        let mut qc = QuantumCircuit::new(2);
        qc.add_gate(vec![5], QuantumGate::x());
    }

    #[test]
    #[should_panic(expected = "Unsupported gate arity")]
    fn test_unsupported_gate_arity() {
        let matrix = vec![Complex::new(0.0, 0.0); 64]; // 3-qubit gate (8x8)
        let gate = QuantumGate::new(matrix, 3, "ThreeQubit".to_string());
        let mut qc = QuantumCircuit::new(3);
        qc.add_gate(vec![0, 1, 2], gate);
        let mut state = QuantumState::new(3);
        qc.execute_on_state(&mut state);
    }

    #[test]
    fn test_execute_bell_state() {
        let mut qc = QuantumCircuit::new(2);
        qc.h(0);
        qc.cx(0, 1);
        let mut state = QuantumState::new(2);
        qc.execute_on_state(&mut state);
        let sv = state.statevector();
        // Bell state: (|00⟩ + |11⟩) / sqrt(2)
        let expected_amp = 1.0 / 2.0_f64.sqrt();
        assert!((sv[0].real() - expected_amp).abs() < 1e-10);
        assert!((sv[3].real() - expected_amp).abs() < 1e-10);
        assert!(sv[1].real().abs() < 1e-10);
        assert!(sv[2].real().abs() < 1e-10);
    }
}
