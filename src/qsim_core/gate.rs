use rusticle::complex::Complex;
use rusticle::linalg::Matrix;

/// Represents a basic quantum gate that can be applied to a qubit.
/// 
/// Each variant represents a different quantum gate with its corresponding
/// unitary matrix representation.
#[derive(Clone, Copy, PartialEq)]
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
        }
    }

    /// Returns the name of the quantum gate.
    pub fn name(&self) -> &'static str {
        match self {
            QuantumGate::X => "Pauli-X",
            QuantumGate::Y => "Pauli-Y",
            QuantumGate::Z => "Pauli-Z",
            QuantumGate::H => "Hadamard",
            QuantumGate::S => "S",
            QuantumGate::T => "T",
        }
    }

    /// Returns the symbol used to represent the gate in circuit diagrams.
    pub fn symbol(&self) -> &'static str {
        match self {
            QuantumGate::X => "X",
            QuantumGate::Y => "Y",
            QuantumGate::Z => "Z",
            QuantumGate::H => "H",
            QuantumGate::S => "S",
            QuantumGate::T => "T",
        }
    }
}

impl std::fmt::Display for QuantumGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl std::fmt::Debug for QuantumGate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{:?}", self.name(), self.matrix())
    }
}
