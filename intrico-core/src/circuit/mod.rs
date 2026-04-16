//! Quantum Circuit representation
//!
//! This module contains the quantum circuit definitions and the relevant
//! implementations for executing a quantum circuit on a specific quantum state
//!
//! The architecture of the quantum circuit that is built is purely graphical -
//! specifically a DAG to keep it memory efficient and support parallelism

use crate::core::QuantumGate;

/// Quantum Circuit - A graph representation of a quantum circuit
pub struct QuantumCircuit {
    num_qubits: usize,
    classical_regs: usize,
    nodes: Vec<QuantumNode>,
    frontier: Vec<Option<usize>>,
}

/// Quantum Node - represents a node in the graph which holds a quantum operation
pub struct QuantumNode {
    pub node_id: usize,
    pub operation: Operation,
    pub parents: Vec<usize>,
    pub children: Vec<usize>,
}

/// Operation - Holds a quantum operation (a gate operation or a measurement operation)
#[derive(Clone)]
pub enum Operation {
    Gate {
        gate: QuantumGate,
        targets: Vec<usize>,
    },
    Measure {
        qubit: usize,
        classical_bit: usize,
    },
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            classical_regs: 0,
            nodes: Vec::new(),
            frontier: vec![None; num_qubits],
        }
    }

    fn add_node(&mut self, operation: Operation) -> &mut Self {
        let node_id = self.nodes.len();
        let mut parents: Vec<usize> = vec![];

        // Auto-update classical register count
        if let Operation::Measure { classical_bit, .. } = &operation {
            if *classical_bit >= self.classical_regs {
                self.classical_regs = classical_bit + 1;
            }
        }

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
            children: Vec::new(),
        };

        self.nodes.push(node);

        self
    }

    pub fn add_gate(&mut self, targets: Vec<usize>, gate: QuantumGate) -> &mut Self {
        for &t in &targets {
            if t >= self.num_qubits {
                panic!(
                    "Target qubit index {} out of bounds for {}-qubit circuit.",
                    t, self.num_qubits
                );
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

    /// Returns true if all measurements come after all gates in the DAG.
    /// When true, sample() can execute gates once and sample from probabilities.
    pub fn has_terminal_measurements_only(&self) -> bool {
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

    /// Getters
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn classical_regs(&self) -> usize {
        self.classical_regs
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

    pub fn rx(&mut self, target: usize, theta: f64) -> &mut Self {
        if target >= self.num_qubits {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::rx(theta));
        self
    }

    pub fn ry(&mut self, target: usize, theta: f64) -> &mut Self {
        if target >= self.num_qubits {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::ry(theta));
        self
    }

    pub fn rz(&mut self, target: usize, theta: f64) -> &mut Self {
        if target >= self.num_qubits {
            panic!("Target qubit index out of bounds.")
        }
        self.add_gate(vec![target], QuantumGate::rz(theta));
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
        self.add_node(Operation::Measure {
            qubit,
            classical_bit,
        })
    }

    pub fn measure_all(&mut self) -> &mut Self {
        for i in 0..self.num_qubits {
            self.add_node(Operation::Measure {
                qubit: i,
                classical_bit: i,
            });
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
