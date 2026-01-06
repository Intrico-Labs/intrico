use crate::gate::QuantumGate;

/// Quantum Circuit representation
pub struct QuantumCircuit {
    num_qubits: usize,
    gates: Vec<QuantumNode>,
    frontier: Vec<Option<usize>>,
}

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
        match gate.arity() {
            1 => {
                // Finding parents
                let mut parents: Vec<usize> = vec![];
                if let Some(i) = self.frontier[targets[0]] {
                    parents.push(i);
                }

                // Updating frontier
                self.frontier[targets[0]] = Some(self.gates.len());

                let node = QuantumNode {
                    gate,
                    targets,
                    parents
                };

                self.gates.push(node);
            }
            _ => {}
        }
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