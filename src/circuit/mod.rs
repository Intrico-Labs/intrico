//! Quantum Circuit representation
//!
//! This module contains the quantum circuit definitions and the relevant
//! implementations for executing a quantum circuit on a specific quantum state
//!
//! The architecture of the quantum circuit that is built is purely graphical -
//! specifically a DAG to keep it memory efficient and support parallelism

use std::collections::HashMap;
use std::time::Instant;

use rand::RngExt;

use crate::core::{ClassicalRegister, QuantumGate};
use crate::engine::{ExecutionContext, ExecutionTime, MeasurementResult, SamplingResult};

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

    /// Execution

    pub fn execute(&self) -> MeasurementResult {
        let start = Instant::now();
        let creg_size = self.classical_register_size();
        let mut ctx = ExecutionContext::new(self.num_qubits, creg_size);
        ctx.run(self);
        let (statevector, classical_register) = ctx.into_inner();

        MeasurementResult {
            statevector,
            classical_register,
            shots: 1,
            execution_time: ExecutionTime::from_duration(start.elapsed()),
        }
    }

    pub fn sample(&self, shots: usize) -> SamplingResult {
        let start = Instant::now();
        let mut counts: HashMap<String, usize> = HashMap::new();
        let creg_size = self.classical_register_size();

        if self.has_terminal_measurements_only() {
            // Optimized: execute gates once, sample from the probability distribution
            let mut ctx = ExecutionContext::new(self.num_qubits, 0);
            ctx.run_gates_only(self);

            let probs: Vec<f64> = ctx.state().statevector()
                .iter()
                .map(|a| a.norm_squared())
                .collect();

            let measurements: Vec<(usize, usize)> = self.nodes.iter()
                .filter_map(|n| match &n.operation {
                    Operation::Measure { qubit, classical_bit } => Some((*qubit, *classical_bit)),
                    _ => None,
                })
                .collect();

            let dim = 1 << self.num_qubits;
            let mut rng = rand::rng();

            for _ in 0..shots {
                let r: f64 = rng.random();
                let mut cumulative = 0.0;
                let mut sampled_index = dim - 1;
                for (i, &p) in probs.iter().enumerate() {
                    cumulative += p;
                    if r < cumulative {
                        sampled_index = i;
                        break;
                    }
                }

                let mut creg = ClassicalRegister::new(creg_size);
                for &(qubit, classical_bit) in &measurements {
                    creg.set(classical_bit, ((sampled_index >> qubit) & 1) as u8);
                }
                *counts.entry(creg.bitstring()).or_insert(0) += 1;
            }
        } else {
            // Fallback: full re-execution each shot (required for mid-circuit measurement)
            for _ in 0..shots {
                let mut ctx = ExecutionContext::new(self.num_qubits, creg_size);
                ctx.run(self);
                *counts.entry(ctx.classical_register().bitstring()).or_insert(0) += 1;
            }
        }

        SamplingResult {
            counts,
            shots,
            execution_time: ExecutionTime::from_duration(start.elapsed()),
        }
    }

    /// Returns true if all measurements come after all gates in the DAG.
    /// When true, sample() can execute gates once and sample from probabilities.
    fn has_terminal_measurements_only(&self) -> bool {
        let mut seen_measure = false;
        for node in &self.nodes {
            match &node.operation {
                Operation::Measure { .. } => seen_measure = true,
                Operation::Gate { .. } if seen_measure => return false,
                _ => {}
            }
        }
        true
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::ExecutionContext;
    use rusticle::Complex;

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
        let mut ctx = ExecutionContext::new(3, 0);
        ctx.run(&qc);
    }

    #[test]
    fn test_execute_bell_state() {
        let mut qc = QuantumCircuit::new(2);
        qc.h(0);
        qc.cx(0, 1);
        let mut ctx = ExecutionContext::new(2, 0);
        ctx.run(&qc);
        let sv = ctx.state().statevector();
        // Bell state: (|00⟩ + |11⟩) / sqrt(2)
        let expected_amp = 1.0 / 2.0_f64.sqrt();
        assert!((sv[0].real() - expected_amp).abs() < 1e-10);
        assert!((sv[3].real() - expected_amp).abs() < 1e-10);
        assert!(sv[1].real().abs() < 1e-10);
        assert!(sv[2].real().abs() < 1e-10);
    }
}
