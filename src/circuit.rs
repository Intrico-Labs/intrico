use crate::gate::QuantumGate;

/// Quantum Circuit - A graph representation of a quantum circuit
pub struct QuantumCircuit {
    num_qubits: usize,
    gates: Vec<QuantumNode>,
    frontier: Vec<Option<usize>>,
}

/// Quantum Node - represents a node in the graph which holds the QuantumGate
pub struct QuantumNode {
    pub gate: QuantumGate,
    pub targets: Vec<usize>,
    pub parents: Vec<usize>
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            gates: Vec::new(),
            frontier: vec![None; num_qubits]
        }
    }

    pub fn add_gate(&mut self, targets: Vec<usize>, gate: QuantumGate) -> &mut Self {
        let mut parents: Vec<usize> = vec![];

        match gate.arity() {
            1 => {
                // Finding parents
                if let Some(i) = self.frontier[targets[0]] {
                    parents.push(i);
                }

                // Updating frontier
                self.frontier[targets[0]] = Some(self.gates.len());
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
                self.frontier[ctrl] = Some(self.gates.len());
                self.frontier[target] = Some(self.gates.len());
            }
            _ => {
                panic!("Invalid arity?")
            }
        }

        let node = QuantumNode {
            gate,
            targets,
            parents
        };

        self.gates.push(node);

        self
    }

    /// Getters
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn gates(&self) -> &Vec<QuantumNode> {
        &self.gates
    }

    pub fn frontier(&self) -> &Vec<Option<usize>> {
        &self.frontier
    }
}