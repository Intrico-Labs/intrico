//! Quantum Gate representation
//!
//! This module contains the quantum gate definition that stores the gate matrices
//! and the metadata required for operating on circuits.
//!

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_4};

use rusticle::Complex;

/// Quantum Gate representation
#[derive(Debug, Clone)]
pub struct QuantumGate {
    /// Gate kind
    kind: GateKind,
    /// Gate matrix
    matrix: Vec<Complex>,
    /// Number of qubits the gate operates on
    arity: usize,
    /// Name of the quantum gate
    name: String,
}

#[derive(Debug, Clone)]
pub enum GateKind {
    H,
    X,
    Y,
    Z,
    S,
    T,
    CX,
    CP,
    Swap,
    Custom,
}

/// Initializers
impl QuantumGate {
    /// Initialises a new quantum gate
    ///
    /// # Panics
    /// Panics if the matrix length does not match the expected dimension for the given arity.
    /// A gate with arity `n` must have a `2^n × 2^n` matrix, i.e. `4^n` elements.
    pub fn new(matrix: Vec<Complex>, arity: usize, name: String) -> Self {
        let expected_len = 1 << (2 * arity); // (2^n)^2 = 4^n
        if matrix.len() != expected_len {
            panic!(
                "Invalid matrix size for gate '{}': expected {} elements for arity {}, got {}.",
                name,
                expected_len,
                arity,
                matrix.len()
            );
        }
        Self {
            kind: GateKind::Custom,
            matrix,
            arity,
            name,
        }
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

    pub fn kind(&self) -> &GateKind {
        &self.kind
    }
}

/// Standard Gates
/// This is a set of commonly used gates
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
            kind: GateKind::X,
            matrix,
            arity: 1,
            name: "X".to_string(),
        }
    }

    /// Pauli Y Gate
    pub fn y() -> Self {
        let matrix = vec![
            Complex::new(0.0, 0.0),
            Complex::new(0.0, -1.0),
            Complex::new(0.0, 1.0),
            Complex::new(0.0, 0.0),
        ];
        Self {
            kind: GateKind::Y,
            matrix,
            arity: 1,
            name: "Y".to_string(),
        }
    }

    /// Pauli Z Gate
    pub fn z() -> Self {
        let matrix = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(-1.0, 0.0),
        ];
        Self {
            kind: GateKind::Z,
            matrix,
            arity: 1,
            name: "Z".to_string(),
        }
    }

    /// S Gate (Phase Gate)
    pub fn s() -> Self {
        let matrix = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 1.0),
        ];
        Self {
            kind: GateKind::S,
            matrix,
            arity: 1,
            name: "S".to_string(),
        }
    }

    /// T Gate (pi/8 Gate)
    pub fn t() -> Self {
        let angle = FRAC_PI_4;
        let matrix = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(angle.cos(), angle.sin()),
        ];
        Self {
            kind: GateKind::T,
            matrix,
            arity: 1,
            name: "T".to_string(),
        }
    }

    /// Hadamard Gate
    pub fn h() -> Self {
        let amp = FRAC_1_SQRT_2;
        let matrix = vec![
            Complex::new(amp, 0.0),
            Complex::new(amp, 0.0),
            Complex::new(amp, 0.0),
            Complex::new(-amp, 0.0),
        ];
        Self {
            kind: GateKind::H,
            matrix,
            arity: 1,
            name: "H".to_string(),
        }
    }

    /// Controlled-X Gate (CNOT)
    pub fn cx() -> Self {
        let matrix = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
        ];
        Self {
            kind: GateKind::CX,
            matrix,
            arity: 2,
            name: "CX".to_string(),
        }
    }

    /// Controlled Phase Gate
    pub fn cp(theta: f64) -> Self {
        let matrix = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(theta.cos(), theta.sin()),
        ];
        Self {
            kind: GateKind::CP,
            matrix,
            arity: 2,
            name: "CP".to_string(),
        }
    }

    /// SWAP Gate
    pub fn swap() -> Self {
        let matrix = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
        ];
        Self {
            kind: GateKind::Swap,
            matrix,
            arity: 2,
            name: "SWAP".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_gate_creation() {
        let matrix = vec![
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
        ];
        let gate = QuantumGate::new(matrix, 1, "X".to_string());
        assert_eq!(gate.arity(), 1);
        assert_eq!(gate.matrix().len(), 4);
    }

    #[test]
    #[should_panic(expected = "Invalid matrix size")]
    fn test_gate_creation_wrong_matrix_size() {
        let matrix = vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)];
        QuantumGate::new(matrix, 1, "Bad".to_string());
    }

    #[test]
    #[should_panic(expected = "Invalid matrix size")]
    fn test_gate_creation_empty_matrix() {
        QuantumGate::new(vec![], 1, "Empty".to_string());
    }
}
