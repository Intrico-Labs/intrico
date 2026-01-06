use std::f32::consts::FRAC_1_SQRT_2;
use crate::Complex;

/// Quantum Gate implementation
#[derive(Debug)]
pub struct QuantumGate {
    matrix: Vec<Complex>,
    arity: usize,
    name: String
}

/// Initializers
impl QuantumGate {
    pub fn new(matrix: Vec<Complex>, arity: usize, name: String) -> Self {
        Self { matrix, arity, name }
    }
}

/// Getters
impl QuantumGate {
    pub fn matrix(&self) -> &Vec<Complex> {
        &self.matrix
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

/// Standard Gates
/// This is a set of commonly used gates like the Clifford Gates
impl QuantumGate {
    /// Pauli X Gate
    pub fn x() -> Self {
        let matrix = vec![
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
        ];
        Self {
            matrix,
            arity: 1,
            name: "X".to_string(),
        }
    }

    /// Hadamard Gate
    pub fn h() -> Self {
        let amp = FRAC_1_SQRT_2 as f64;
        let matrix = vec![
            Complex::new(amp, 0.0),
            Complex::new(amp, 0.0),
            Complex::new(amp, 0.0),
            Complex::new(-amp, 0.0),
        ];
        Self {
            matrix,
            arity: 1,
            name: "H".to_string(),
        }
    }

    /// Controlled-X Gate (CNOT)
    pub fn cx() -> Self {
        let matrix = vec![
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ];
        Self {
            matrix,
            arity: 2,
            name: "CX".to_string(),
        }
    }
}