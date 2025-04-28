use std::{cmp, fmt};
use rusticle::complex::{Complex, ComplexVector};

use crate::{core::gate::{GateOp, QuantumGate}, utility::round_if_close};

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
    ///
    /// # Arguments
    /// * `target` - The index of the qubit to apply the gate to
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.x(0);  // Apply X gate to the first qubit
    /// ```
    pub fn x(&mut self, target: usize) {
        self.add_gate(QuantumGate::X, target);
    }

    /// Applies a Pauli-Y gate to the specified qubit
    /// 
    /// # Arguments
    /// * `target` - The index of the qubit to apply the gate to
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.y(0);  // Apply Y gate to the first qubit
    /// ```
    pub fn y(&mut self, target: usize) {
        self.add_gate(QuantumGate::Y, target);
    }

    /// Applies a Pauli-Z gate to the specified qubit
    /// 
    /// # Arguments
    /// * `target` - The index of the qubit to apply the gate to
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.z(0);  // Apply Z gate to the first qubit
    /// ```
    pub fn z(&mut self, target: usize) {
        self.add_gate(QuantumGate::Z, target);
    }

    /// Applies an S gate to the specified qubit
    /// 
    /// # Arguments
    /// * `target` - The index of the qubit to apply the gate to
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.s(0);  // Apply S gate to the first qubit
    /// ```
    pub fn s(&mut self, target: usize) {
        self.add_gate(QuantumGate::S, target);
    }

    /// Applies a T gate to the specified qubit
    /// 
    /// # Arguments
    /// * `target` - The index of the qubit to apply the gate to
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.t(0);  // Apply T gate to the first qubit
    /// ```
    pub fn t(&mut self, target: usize) {
        self.add_gate(QuantumGate::T, target);
    }

    /// Applies a CNOT gate with the specified control and target qubits
    /// 
    /// # Arguments
    /// * `control` - The index of the control qubit
    /// * `target` - The index of the target qubit
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(2);
    /// qc.cnot(0, 1);  // Apply CNOT gate with control qubit 0 and target qubit 1
    /// ```
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
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(2);
    /// qc.cx(0, 1);  // Apply CNOT gate with control qubit 0 and target qubit 1
    /// ```

    pub fn cx(&mut self, control: usize, target: usize) {
        self.cnot(control, target);
    }

    /// Applies a Rx gate to the specified qubit
    /// 
    /// # Arguments
    /// * `target` - The index of the qubit to apply the gate to
    /// * `angle` - The angle of the Rx gate
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.rx(0, std::f64::consts::PI / 2.0);  // Apply Rx gate to the first qubit with angle π/2
    /// ```
    pub fn rx(&mut self, target: usize, angle: f64) {
        self.add_gate(QuantumGate::Rx(angle), target);
    }

    /// Applies a Ry gate to the specified qubit
    /// 
    /// # Arguments
    /// * `target` - The index of the qubit to apply the gate to
    /// * `angle` - The angle of the Ry gate
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.ry(0, std::f64::consts::PI / 2.0);  // Apply Ry gate to the first qubit with angle π/2
    /// ```
    pub fn ry(&mut self, target: usize, angle: f64) {
        self.add_gate(QuantumGate::Ry(angle), target);
    }

    /// Applies a Rz gate to the specified qubit
    /// 
    /// # Arguments
    /// * `target` - The index of the qubit to apply the gate to
    /// * `angle` - The angle of the Rz gate
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.rz(0, std::f64::consts::PI / 2.0);  // Apply Rz gate to the first qubit with angle π/2
    /// ```
    pub fn rz(&mut self, target: usize, angle: f64) {
        self.add_gate(QuantumGate::Rz(angle), target);
    }

    /// Applies a Measurement
    /// 
    /// # Arguments
    /// * `qubit` - The index of the qubit to measure
    /// * `classical_bit` - The index of the classical bit to store the result
    /// 
    /// # Examples
    /// ``` 
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.measure(0, 0);  // Measure the first qubit and store the result in the first classical bit
    /// ``` 
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
    /// # Examples
    /// ```
    /// use intrico::{QuantumCircuit, QuantumGate};
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.add_gate(QuantumGate::H, 0);  // Add a Hadamard gate to the first qubit
    /// ```

    pub fn add_gate(&mut self, gate: QuantumGate, target: usize) {
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

    fn apply_two_qubit_gate(&self, state_vector: &mut Vec<Complex>, gate: QuantumGate, control: usize, target: usize) {
        let n = self.num_qubits;
        let dim = 1 << n;

        let (low, high) = if control < target { (control, target) } else { (target, control) };

        let mut visited = vec![false; dim];  

        for i in 0..dim {
            if visited[i] {
                continue;
            }

            // Compute 4 indices for this 2-qubit subspace
            let base = i & !(1 << low) & !(1 << high); 
            let mut indices = [0usize; 4];
            for k in 0..4 {
                let b0 = k & 1;
                let b1 = (k >> 1) & 1;
                indices[k] = base | (b0 << low) | (b1 << high);
            }

            if indices.iter().any(|&idx| visited[idx]) {
                continue;
            }

            // Extract amplitudes
            let original: [Complex; 4] = indices.map(|idx| state_vector[idx]);

            // Apply gate
            let mut new_values = [Complex::new(0.0, 0.0); 4];
            for r in 0..4 {
                for c in 0..4 {
                    new_values[r] += *gate.matrix().get(r, c) * original[c];
                }
            }

            for (k, &val) in indices.iter().zip(&new_values) {
                state_vector[*k] = val;
                visited[*k] = true;
            }
        }
    }

    fn apply_cnot(&self, state_vector: &mut Vec<Complex>, control: usize, target: usize) {
        let dim = state_vector.len();
        let mut new_state = state_vector.clone();
    
        for i in 0..dim {
            let control_bit = (i >> control) & 1;
            if control_bit == 1 {
                let flipped = i ^ (1 << target);  // Flip target bit
                new_state[flipped] = state_vector[i];
                new_state[i] = state_vector[flipped];
            }
        }
    
        *state_vector = new_state;
    }

    /// Executes the circuit on a set of qubits
    /// 
    /// # Arguments
    /// * `qubits` - A slice of qubits to apply the circuit to
    /// 
    /// # Examples
    /// ```
    /// use intrico::QuantumCircuit;
    /// 
    /// let mut qc = QuantumCircuit::new(1);
    /// qc.h(0);
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
                    self.apply_single_qubit_gate(&mut state_vector, op.gate.clone(), op.target());
                },
                2 => {
                    if op.gate == QuantumGate::CNOT {
                        self.apply_cnot(&mut state_vector, op.controls()[0], op.target());
                    } else {
                        self.apply_two_qubit_gate(&mut state_vector, op.gate.clone(), op.controls()[0], op.target());
                    }
                },
                _ => {}
            }
        }

        state_vector
            .into_iter()
            .map(|c| Complex {
                real: round_if_close(c.real, 1e-10),
                imag: round_if_close(c.imag, 1e-10),
            })
            .collect()
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
        let wire = "───".to_string();
        let vert_line = " │ ".to_string();
        let ctrl_dot = "─●─".to_string();
        
        let mut grid = vec![vec![wire; max_step + 1]; height];
        
        for op in &self.operations {
            let row = 2 * op.target();
            let col = op.step;
            
            // Skip if the operation is out of bounds (safety check)
            if row >= height || col > max_step {
                continue;
            }
            
            match op.gate.arity() {
                1 => {
                    grid[row][col] = op.gate.display_symbol();
                },
                2 => {
                    let control = op.controls()[0];
                    let ctrl_row = 2 * control;
                    
                    // Skip if control is out of bounds
                    if ctrl_row >= height {
                        continue;
                    }
                    
                    grid[ctrl_row][col] = ctrl_dot.clone();  
                    grid[row][col] = op.gate.display_symbol();
                    
                    let (start, end) = if ctrl_row < row {
                        (ctrl_row + 1, row)
                    } else {
                        (row + 1, ctrl_row)
                    };
                    
                    for r in start..end {
                        grid[r][col] = vert_line.clone(); 
                    }
                },
                _ => {}
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
                
                writeln!(f, "  {}. {} on qubit {} by {} (Step: {})", 
                        i + 1, op.gate, op.target(), op.controls()[0], op.step)?;
                
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