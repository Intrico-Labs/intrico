//! Quantum Circuit representation
//! 
//! This module contains the quantum circuit definitions and the relevant
//! implementations for executing a quantum circuit on a specific quantum state
//! 
//! The architecture of the quantum circuit that is built is purely graphical -
//! specifically a DAG to keep it memory efficient and support parallelism

use std::cmp::{max, min};

use rusticle::Complex;

use crate::{gate::QuantumGate, state::QuantumState};

/// Quantum Circuit - A graph representation of a quantum circuit
pub struct QuantumCircuit {
    num_qubits: usize,
    nodes: Vec<QuantumNode>,
    frontier: Vec<Option<usize>>,
}

/// Quantum Node - represents a node in the graph which holds the QuantumGate
pub struct QuantumNode {
    pub node_id: usize,
    pub gate: QuantumGate,
    pub targets: Vec<usize>,
    pub parents: Vec<usize>,
    pub children: Vec<usize>
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            nodes: Vec::new(),
            frontier: vec![None; num_qubits]
        }
    }

    pub fn add_gate(&mut self, targets: Vec<usize>, gate: QuantumGate) -> &mut Self {
        let mut parents: Vec<usize> = vec![];

        // Calculate node id (incremental)
        let node_id = self.nodes.len();

        match gate.arity() {
            1 => {
                // Finding parents
                if let Some(i) = self.frontier[targets[0]] {
                    parents.push(i);
                }

                // Updating frontier
                self.frontier[targets[0]] = Some(node_id);
            }
            2 => {
                // Finding parents
                let (ctrl, target) = (targets[0], targets[1]);
                if let Some(i) = self.frontier[ctrl] {
                    parents.push(i);
                }
                if let Some(i) = self.frontier[target] {
                    parents.push(i);
                }

                // Updating frontier
                self.frontier[ctrl] = Some(node_id);
                self.frontier[target] = Some(node_id);
            }
            _ => {
                panic!("Invalid arity?")
            }
        }

        // Register this node as a child of each parent
        for &parent_id in &parents {
            self.nodes[parent_id].children.push(node_id);
        }

        let node = QuantumNode {
            node_id,
            gate,
            targets,
            parents,
            children: Vec::new()
        };

        self.nodes.push(node);

        self
    }

    pub fn append(&mut self, circuit: &QuantumCircuit) -> &mut Self {
        if circuit.num_qubits != self.num_qubits {
            panic!("Cannot append circuits with different qubit counts.")
        }

        for node in circuit.nodes() {
            self.add_gate(node.targets.clone(), node.gate.clone());
        }

        self
    }

    /// Execution Engine
    /// This basically holds all the logic for executing a quantum circuit on a given quantum state
    
    pub fn execute_on_state(&self, state: &mut QuantumState) {
        let operations = self.nodes();
        let dim = 1 << self.num_qubits; // State vector dimension = 2^num_qubits

        for op in operations {
            let gate = &op.gate;

            match gate.arity() {
                1 => {
                    apply_single_qubit_gate(gate.matrix(), state.statevector_mut(), op.targets[0], dim);
                }
                2 => {
                    apply_two_qubit_gate(gate.matrix(), state.statevector_mut(), op.targets[0], op.targets[1], dim);
                }
                _ => {}
            }
        }
    }

    pub fn execute(&self) {
        let mut state = QuantumState::new(self.num_qubits);
        self.execute_on_state(&mut state);
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
        if target > self.num_qubits-1 {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::x());
        self
    }

    pub fn y(&mut self, target: usize) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::y());
        self
    }

    pub fn z(&mut self, target: usize) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::z());
        self
    }

    pub fn s(&mut self, target: usize) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::s());
        self
    }

    pub fn t(&mut self, target: usize) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::t());
        self
    }

    pub fn h(&mut self, target: usize) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::h());
        self
    }

    pub fn cx(&mut self, control: usize, target: usize) -> &mut Self {
        if control > self.num_qubits-1 || target > self.num_qubits-1 {
            panic!("Control or target qubit index out of bounds.")
        }
        if control == target {
            panic!("Control and target qubits must be different.")
        }
        self.add_gate(vec![control, target], QuantumGate::cx());
        self
    }

    pub fn cp(&mut self, control: usize, target: usize, theta: f64) -> &mut Self {
        if control > self.num_qubits-1 || target > self.num_qubits-1 {
            panic!("Control or target qubit index out of bounds.")
        }
        if control == target {
            panic!("Control and target qubits must be different.")
        }
        self.add_gate(vec![control, target], QuantumGate::cp(theta));
        self
    }

    pub fn swap(&mut self, qubit1: usize, qubit2: usize) -> &mut Self {
        if qubit1 > self.num_qubits-1 || qubit2 > self.num_qubits-1 {
            panic!("Qubit index out of bounds.")
        }
        if qubit1 == qubit2 {
            panic!("Qubits must be different.")
        }
        self.add_gate(vec![qubit1, qubit2], QuantumGate::swap());
        self
    }
}

/// Helper functions
/// These are essential helper functions used in above implementations

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
