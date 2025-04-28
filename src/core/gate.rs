use rusticle::complex::Complex;
use rusticle::linalg::Matrix;

/// Represents a basic quantum gate that can be applied to a qubit.
/// 
/// Each variant represents a different quantum gate with its corresponding
/// unitary matrix representation.
#[derive(Clone, PartialEq)]
pub enum QuantumGate {
    /// The Pauli-X gate (quantum NOT gate)
    /// 
    /// Matrix representation:
    /// ```text
    /// [0 1]
    /// [1 0]
    /// ```
    X,
    
    /// The Pauli-Y gate
    /// 
    /// Matrix representation:
    /// ```text
    /// [0 -i]
    /// [i  0]
    /// ```
    Y,
    
    /// The Pauli-Z gate
    /// 
    /// Matrix representation:
    /// ```text
    /// [1  0]
    /// [0 -1]
    /// ```
    Z,
    
    /// The Hadamard gate
    /// 
    /// Matrix representation:
    /// ```text
    /// 1/√2 [1  1]
    ///      [1 -1]
    /// ```
    H,
    
    /// The S gate (√Z gate)
    /// 
    /// Matrix representation:
    /// ```text
    /// [1 0]
    /// [0 i]
    /// ```
    S,
    
    /// The T gate (π/4 phase gate)
    /// 
    /// Matrix representation:
    /// ```text 
    /// [1 0]
    /// [0 e^(iπ/4)]
    /// ```
    T,

    /// The Rx gate (rotation around X axis)
    /// 
    /// Matrix representation:
    /// ```text
    /// [cos(theta/2) -isin(theta/2)]
    /// [-isin(theta/2) cos(theta/2)]
    /// ```
    Rx(f64),

    /// The Ry gate (rotation around Y axis)
    /// 
    /// Matrix representation:
    /// ```text
    /// [cos(theta/2) -sin(theta/2)]
    /// [sin(theta/2) cos(theta/2)]
    /// ```
    Ry(f64),

    /// The Rz gate (rotation around Z axis)
    /// 
    /// Matrix representation:
    /// ```text
    /// [e^(-itheta/2) 0]
    /// [0 e^(itheta/2)]
    /// ```
    Rz(f64),
    
    /// The Controlled-NOT gate
    /// 
    /// Matrix representation:
    /// ```text
    /// [1 0 0 0]
    /// [0 1 0 0]
    /// [0 0 0 1]
    /// [0 0 1 0]
    /// ```
    CNOT,

    /// Measurement
    Measure,

    /// Custom Gate (Matrix, Name, Symbol)
    Custom(Matrix<Complex>, String, String),
}

/// Represents a quantum gate operation in a circuit
/// 
/// This struct efficiently stores gate operations by using a compact representation
/// that includes the gate type and the target qubit index.
#[derive(Debug, Clone, PartialEq)]
pub struct GateOp {
    /// The quantum gate to apply
    pub gate: QuantumGate,
    /// The indices of qubits the gate applies on
    pub qubit: Vec<usize>,
    /// The step in the circuit where this operation occurs
    pub step: usize,
    /// The classical bit index (for storing measurement results)
    pub classical_bit: Option<usize>,
}

impl GateOp {
    /// Creates a new single-qubit gate operation
    pub fn new(gate: QuantumGate, target: usize, step: usize) -> Self {
        GateOp {
            gate,
            qubit: vec![target],
            step,
            classical_bit: None,
        }
    }

    /// Creates a new controlled gate operation
    pub fn controlled(gate: QuantumGate, control: usize, target: usize, step: usize) -> Self {
        GateOp {
            gate,
            qubit: vec![control, target],
            step,
            classical_bit: None,
        }
    }
    
    /// Get the target qubit (last qubit in the list)
    pub fn target(&self) -> usize {
        *self.qubit.last().unwrap_or(&0)
    }
    
    /// Get the control qubits for controlled gates (all qubits except the last)
    pub fn controls(&self) -> Vec<usize> {
        if self.gate.arity() < 2 {
            panic!("Cannot get control qubits for single-qubit gate");
        }
        self.qubit[..self.qubit.len()-1].to_vec()
    }
}

impl QuantumGate {
    /// Returns the matrix representation of the quantum gate.
    pub fn matrix(&self) -> Matrix<Complex> {
        match self {
            QuantumGate::X => Matrix::new(2, 2, vec![
                        Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
                        Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                    ]),
            QuantumGate::Y => Matrix::new(2, 2, vec![
                        Complex::new(0.0, 0.0), Complex::new(0.0, -1.0),
                        Complex::new(0.0, 1.0), Complex::new(0.0, 0.0),
                    ]),
            QuantumGate::Z => Matrix::new(2, 2, vec![
                        Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                        Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0),
                    ]),
            QuantumGate::H => {
                        let factor = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
                        Matrix::new(2, 2, vec![
                            factor, factor,
                            factor, -factor,
                        ])
                    },
            QuantumGate::S => Matrix::new(2, 2, vec![
                        Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                        Complex::new(0.0, 0.0), Complex::new(0.0, 1.0),
                    ]),
            QuantumGate::T => {
                        let phase = Complex::new(0.0, std::f64::consts::PI/4.0).exp();
                        Matrix::new(2, 2, vec![
                            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                            Complex::new(0.0, 0.0), phase,
                        ])
                    },
            QuantumGate::CNOT => Matrix::new(4, 4, vec![
                        Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
                        Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
                        Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
                        Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                    ]),
            QuantumGate::Measure => {
                        // Return zero for measurement matrix
                        Matrix::zeros(1, 1)
                    }
            QuantumGate::Rx(angle) => {
                let cos = Complex::new((angle / 2.0).cos(), 0.0);
                let neg_isin = Complex::new(0.0, -(angle / 2.0).sin());

                Matrix::new(2, 2, vec![
                    cos, neg_isin, 
                    neg_isin, cos
                ])
            },
            QuantumGate::Ry(angle) => {
                let cos = Complex::new((angle / 2.0).cos(), 0.0);
                let sin = Complex::new((angle / 2.0).sin(), 0.0);

                Matrix::new(2, 2, vec![
                    cos, -sin, 
                    sin, cos
                ])
            },
            QuantumGate::Rz(angle) => {
                let minus_i = Complex::new(0.0, -angle / 2.0).exp();
                let plus_i = Complex::new(0.0, angle / 2.0).exp();
                Matrix::new(2, 2, vec![
                    minus_i, Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0), plus_i,
                ])
            },
            QuantumGate::Custom(matrix, _, _) => matrix.clone(),
        }
    }

    /// Returns the name of the quantum gate.
    pub fn name(&self) -> String {
        match self {
            QuantumGate::X => "Pauli-X".to_string(),
            QuantumGate::Y => "Pauli-Y".to_string(),
            QuantumGate::Z => "Pauli-Z".to_string(),
            QuantumGate::H => "Hadamard".to_string(),
            QuantumGate::S => "S".to_string(),
            QuantumGate::T => "T".to_string(),
            QuantumGate::CNOT => "CNOT".to_string(),
            QuantumGate::Measure => "Measurement".to_string(),
            QuantumGate::Rx(angle) => format!("Rx({})", angle),
            QuantumGate::Ry(angle) => format!("Ry({})", angle),
            QuantumGate::Rz(angle) => format!("Rz({})", angle),
            QuantumGate::Custom(_, name, _) => format!("{}", name),
        }
    }

    /// Returns the symbol used to represent the gate in circuit diagrams.
    pub fn symbol(&self) -> String {
        match self {
            QuantumGate::X => "X".to_string(),
            QuantumGate::Y => "Y".to_string(),
            QuantumGate::Z => "Z".to_string(),
            QuantumGate::H => "H".to_string(),
            QuantumGate::S => "S".to_string(),
            QuantumGate::T => "T".to_string(),
            QuantumGate::CNOT => "CX".to_string(),
            QuantumGate::Measure => "M".to_string(),
            QuantumGate::Rx(angle) => format!("Rx({})", angle),
            QuantumGate::Ry(angle) => format!("Ry({})", angle),
            QuantumGate::Rz(angle) => format!("Rz({})", angle),
            QuantumGate::Custom(_, _, symbol) => format!("{}", symbol),
        }
    }
    
    /// Returns the display symbol with connecting wires for ASCII circuit diagrams.
    pub fn display_symbol(&self) -> String {
        match self {
            QuantumGate::X => "─X─".to_string(),
            QuantumGate::Y => "─Y─".to_string(),
            QuantumGate::Z => "─Z─".to_string(),
            QuantumGate::H => "─H─".to_string(),
            QuantumGate::S => "─S─".to_string(),
            QuantumGate::T => "─T─".to_string(),
            QuantumGate::CNOT => "─x─".to_string(),
            QuantumGate::Measure => "─[M]─".to_string(),
            QuantumGate::Rx(angle) => format!("─Rx({:.2})─", angle),
            QuantumGate::Ry(angle) => format!("─Ry({:.2})─", angle),
            QuantumGate::Rz(angle) => format!("─Rz({:.2})─", angle),
            QuantumGate::Custom(_, _, symbol) => format!("─{}─", symbol),
        }
    }

    /// Returns the number of qubits that the gate operates on.
    pub fn arity(&self) -> usize {
        match self {
            QuantumGate::CNOT => 2,
            _ => 1,
        }
    }
}

impl std::fmt::Display for QuantumGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuantumGate::Rx(angle) => write!(f, "Rx({:.2})", angle),
            QuantumGate::Ry(angle) => write!(f, "Ry({:.2})", angle),
            QuantumGate::Rz(angle) => write!(f, "Rz({:.2})", angle),
            _ => write!(f, "{}", self.symbol()),
        }
    }
}

impl std::fmt::Debug for QuantumGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuantumGate::Rx(angle) => write!(f, "Rx({:.2})", angle),
            QuantumGate::Ry(angle) => write!(f, "Ry({:.2})", angle),
            QuantumGate::Rz(angle) => write!(f, "Rz({:.2})", angle),
            _ => write!(f, "{}", self.symbol()),
        }
    }
}
