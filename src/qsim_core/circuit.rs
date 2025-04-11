use std::fmt;
use crate::qsim_core::gate::{QuantumGate, GateOp};
use crate::qsim_core::qubit::Qubit;

/// Represents a quantum circuit that can be built and executed
/// 
/// A quantum circuit is a sequence of quantum gates applied to one or more qubits.
/// This implementation allows for building circuits incrementally and executing them
/// on a set of qubits.
pub struct QuantumCircuit {
    /// The number of qubits in the circuit
    num_qubits: usize,
    /// The sequence of gate operations to apply
    operations: Vec<GateOp>,
}

impl QuantumCircuit {
    /// Creates a new quantum circuit with the specified number of qubits
    /// 
    /// # Arguments
    /// * `num_qubits` - The number of qubits in the circuit
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);  // Create a 1-qubit circuit
    /// ```
    pub fn new(num_qubits: usize) -> Self {
        QuantumCircuit {
            num_qubits,
            operations: Vec::new(),
        }
    }

    /// Applies a Hadamard gate to the specified qubit
    /// 
    /// # Arguments
    /// * `target` - The index of the qubit to apply the gate to
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.h(0);  // Apply Hadamard gate to the first qubit
    /// ```
    pub fn h(&mut self, target: usize) {
        self.add_gate(QuantumGate::H, target);
    }

    /// Applies a Pauli-X gate to the specified qubit
    pub fn x(&mut self, target: usize) {
        self.add_gate(QuantumGate::X, target);
    }

    /// Applies a Pauli-Y gate to the specified qubit
    pub fn y(&mut self, target: usize) {
        self.add_gate(QuantumGate::Y, target);
    }

    /// Applies a Pauli-Z gate to the specified qubit
    pub fn z(&mut self, target: usize) {
        self.add_gate(QuantumGate::Z, target);
    }

    /// Applies an S gate to the specified qubit
    pub fn s(&mut self, target: usize) {
        self.add_gate(QuantumGate::S, target);
    }

    /// Applies a T gate to the specified qubit
    pub fn t(&mut self, target: usize) {
        self.add_gate(QuantumGate::T, target);
    }

    /// Adds a gate operation to the circuit
    /// 
    /// # Arguments
    /// * `gate` - The quantum gate to apply
    /// * `target` - The index of the qubit to apply the gate to
    /// 
    /// # Panics
    /// Panics if the target qubit index is out of bounds
    fn add_gate(&mut self, gate: QuantumGate, target: usize) {
        if target >= self.num_qubits {
            panic!("Qubit index {} is out of bounds for circuit with {} qubits", 
                   target, self.num_qubits);
        }
        self.operations.push(GateOp { gate, target });
    }

    /// Executes the circuit on a set of qubits
    /// 
    /// # Arguments
    /// * `qubits` - A slice of qubits to apply the circuit to
    /// 
    /// # Panics
    /// Panics if the number of qubits doesn't match the circuit size
    /// 
    /// # Examples
    /// ```
    /// use intrico::{QuantumCircuit, Qubit};
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.h(0);
    /// 
    /// let mut qubits = vec![Qubit::zero()];
    /// qc.execute(&mut qubits);
    /// ```
    pub fn execute(&self, qubits: &mut [Qubit]) {
        if qubits.len() != self.num_qubits {
            panic!("Number of qubits ({}) doesn't match circuit size ({})", 
                   qubits.len(), self.num_qubits);
        }

        for op in &self.operations {
            qubits[op.target].apply(op.gate);
        }
    }

    /// Returns the number of qubits in the circuit
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the number of operations in the circuit
    pub fn num_operations(&self) -> usize {
        self.operations.len()
    }

    /// Displays the quantum circuit in ASCII format
    /// 
    /// This method generates a text-based representation of the quantum circuit,
    /// showing the qubit lines and gates in a circuit diagram format.
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(2);
    /// qc.h(0);
    /// qc.x(1);
    /// println!("{}", qc.display_ascii());
    /// ```
    /// 
    /// Output:
    /// ```text
    /// q0: ───H───
    /// q1: ───X───
    /// ```
    pub fn display_ascii(&self) -> String {
        if self.operations.is_empty() {
            return (0..self.num_qubits)
                .map(|i| format!("q{}: ───", i))
                .collect::<Vec<_>>()
                .join("\n");
        }

        // Find the maximum number of gates on any qubit
        let max_gates = self.operations
            .iter()
            .fold(vec![0; self.num_qubits], |mut counts, op| {
                counts[op.target] += 1;
                counts
            })
            .into_iter()
            .max()
            .unwrap_or(0);

        // Create a grid to represent the circuit
        let mut grid = vec![vec![' '; max_gates * 4 + 4]; self.num_qubits];
        
        // Draw the qubit lines
        for i in 0..self.num_qubits {
            for j in 0..max_gates * 4 + 4 {
                grid[i][j] = '─';
            }
        }

        // Place the gates
        for op in &self.operations {
            let qubit = op.target;
            let gate_pos = self.operations
                .iter()
                .filter(|o| o.target == qubit)
                .position(|o| o.gate == op.gate && o.target == op.target)
                .unwrap() * 4 + 2;
            
            // Place the gate symbol
            grid[qubit][gate_pos] = op.gate.symbol().chars().next().unwrap();
        }

        // Convert the grid to a string
        let mut result = String::new();
        for (i, row) in grid.iter().enumerate() {
            result.push_str(&format!("q{}: ", i));
            result.extend(row.iter());
            result.push('\n');
        }

        result
    }
}

impl fmt::Display for QuantumCircuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Quantum Circuit ({} qubits, {} operations):", 
                 self.num_qubits, self.num_operations())?;
        for (i, op) in self.operations.iter().enumerate() {
            writeln!(f, "  {}. {} on qubit {}", i + 1, op.gate, op.target)?;
        }
        Ok(())
    }
}

impl fmt::Debug for QuantumCircuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QuantumCircuit {{ num_qubits: {}, operations: {:?} }}", 
               self.num_qubits, self.operations)
    }
} 