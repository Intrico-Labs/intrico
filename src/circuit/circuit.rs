use std::{cmp, fmt};
use rusticle::complex::{Complex, ComplexVector};

use crate::core::gate::{QuantumGate, GateOp};

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
    /// Classical bits to store results
    classical_bits: Vec<u8>,
    /// Last step of the qubit (for step calculation)
    last_step: Vec<usize>,
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
            classical_bits: Vec::with_capacity(num_qubits),
            last_step: vec![0; num_qubits],
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

    /// Applies a CNOT gate with the specified control and target qubits
    /// 
    /// # Arguments
    /// * `control` - The index of the control qubit
    /// * `target` - The index of the target qubit
    /// 
    /// # Panics
    /// Panics if either qubit index is out of bounds
    pub fn cnot(&mut self, control: usize, target: usize) {
        if control >= self.num_qubits || target >= self.num_qubits {
            panic!("Qubit index out of bounds for circuit with {} qubits", self.num_qubits);
        }
        let max_step = cmp::max(self.last_step[control], self.last_step[target]) + 1;
        self.last_step[control] = max_step;
        self.last_step[target] = max_step;

        let step = self.last_step[target];
        self.operations.push(GateOp::controlled(QuantumGate::CNOT, control, target, step));
    }

    /// Applies a CNOT gate with the specified control and target qubits
    /// 
    /// # Arguments
    /// * `control` - The index of the control qubit
    /// * `target` - The index of the target qubit
    /// 
    /// # Panics
    /// Panics if either qubit index is out of bounds
    pub fn cx(&mut self, control: usize, target: usize) {
        self.cnot(control, target);
    }

    /// Applies a Measurement
    /// TODO
    pub fn measure(&mut self, qubit: usize, classical_bit: usize) {
        if qubit >= self.num_qubits {
            panic!("Qubit index {} is out of bounds for circuit with {} qubits", 
                   qubit, self.num_qubits);
        }

        // Ensure classical bits vector has enough space
        while classical_bit >= self.classical_bits.len() {
            self.classical_bits.push(0);
        }

        self.last_step[qubit]+=1;
        let step = self.last_step[qubit];


        let mut op = GateOp::new(QuantumGate::Measure, qubit, step);
        op.classical_bit = Some(classical_bit);
        self.operations.push(op);
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
        self.last_step[target] += 1;
        let step = self.last_step[target];
        self.operations.push(GateOp::new(gate, target, step));
    }

    fn apply_single_qubit_gate(&self, state_vector: &mut Vec<Complex>, gate: QuantumGate, target: usize) {
        let n = state_vector.len();
        let mask = 1 << target;

        for i in 0..n {
            if i & mask == 0 {
                let j = i | mask;  // Flip the target qubit
                let a = state_vector[i];      // Amplitude of the state |i⟩
                let b = state_vector[j];      // Amplitude of the state |j⟩

                let ampl_vec = ComplexVector::new(vec![a, b]);
                let ampl_vec = ampl_vec.mul_matrix(&gate.matrix());

                state_vector[i] = ampl_vec.components[0];
                state_vector[j] = ampl_vec.components[1];
            }
        }
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
    /// 
    /// qc.execute();
    /// ```
    pub fn execute(&self) -> Vec<Complex> {
        let dim = 1 << self.num_qubits;
        let mut state_vector = vec![Complex::new(0.0, 0.0); dim];

        // Selecting first state as active state
        state_vector[0] = Complex::new(1.0, 0.0);

        for op in &self.operations {
            match op.gate.arity() {
                // single qubit gates
                1 => {
                    self.apply_single_qubit_gate(&mut state_vector, op.gate, op.target());
                },
                2 => {unimplemented!()},
                _ => {}
            }
        }

        state_vector
    }

    /// Returns the number of qubits in the circuit
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Returns the number of operations in the circuit
    pub fn num_operations(&self) -> usize {
        self.operations.len()
    }

    /// Displays the quantum circuit in ASCII format to stdout
    pub fn display(&self) {
        // Handle empty circuit case
        if self.operations.is_empty() {
            for i in 0..self.num_qubits {
                println!("q{}: ───", i);
            }
            return;
        }

        let max_step = *self.last_step.iter()
            .max()
            .unwrap_or(&0);
        
        let height = 2 * self.num_qubits - 1;
        
        // Misc symbols
        let wire = "───";
        let vert_line = " │ ";
        let ctrl_dot = "─●─";
        
        let mut grid = vec![vec![wire; max_step + 1]; height];
        
        for op in &self.operations {
            let row = 2 * op.target();
            let col = op.step;
            
            // Skip if the operation is out of bounds (safety check)
            if row >= height || col > max_step {
                continue;
            }
            
            match op.gate {
                QuantumGate::X | QuantumGate::Y | QuantumGate::Z | 
                QuantumGate::H | QuantumGate::S | QuantumGate::T | QuantumGate::Measure => {
                    grid[row][col] = op.gate.display_symbol();
                },
                QuantumGate::CNOT => {
                    if let Some(control) = op.control() {
                        let ctrl_row = 2 * control;
                        
                        // Skip if control is out of bounds
                        if ctrl_row >= height {
                            continue;
                        }
                        
                        grid[ctrl_row][col] = ctrl_dot;  
                        grid[row][col] = op.gate.display_symbol();
                        
                        let (start, end) = if ctrl_row < row {
                            (ctrl_row + 1, row)
                        } else {
                            (row + 1, ctrl_row)
                        };
                        
                        for r in start..end {
                            grid[r][col] = vert_line; 
                        }
                    }
                },
            }
        }
        
        for i in 0..height {
            if i % 2 == 0 {
                print!("q{}: ", i/2);
            } else {
                print!("    ");  
            }
            
            // Print the row contents
            for j in 1..=max_step {
                if i % 2 == 1 && grid[i][j] != vert_line {
                    print!("   ");  
                } else {
                    print!("{}", grid[i][j]);
                }
            }
            println!();
        }
    }
}

impl fmt::Display for QuantumCircuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Quantum Circuit ({} qubits, {} operations):", 
                 self.num_qubits, self.num_operations())?;
        for (i, op) in self.operations.iter().enumerate() {
            if op.gate == QuantumGate::CNOT {
                if let Some(control) = op.control() {
                    writeln!(f, "  {}. {} on qubit {} by {} (Step: {})", 
                            i + 1, op.gate, op.target(), control, op.step)?;
                } else {
                    writeln!(f, "  {}. {} on qubit {} (Step: {})", 
                            i + 1, op.gate, op.target(), op.step)?;
                }
            } else {
                writeln!(f, "  {}. {} on qubit {} (Step: {})", 
                         i + 1, op.gate, op.target(), op.step)?;
            }
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