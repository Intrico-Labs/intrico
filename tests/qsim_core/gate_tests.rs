use intrico::QuantumGate;
use rusticle::complex::Complex;
use rusticle::linalg::Matrix;

/// Test suite for the QuantumGate type.
/// 
/// These tests verify the core functionality of quantum gates, including:
/// - Matrix representations
/// - Gate properties
/// - Display formatting
mod gate_tests {
    use super::*;

    /// Tests the matrix representation of the Pauli-X gate.
    #[test]
    fn test_pauli_x_gate() {
        let x = QuantumGate::X;
        let expected = Matrix::new(2, 2, vec![
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ]);
        assert_eq!(x.matrix(), expected);
    }

    /// Tests the matrix representation of the Pauli-Y gate.
    #[test]
    fn test_pauli_y_gate() {
        let y = QuantumGate::Y;
        let expected = Matrix::new(2, 2, vec![
            Complex::new(0.0, 0.0), Complex::new(0.0, -1.0),
            Complex::new(0.0, 1.0), Complex::new(0.0, 0.0),
        ]);
        assert_eq!(y.matrix(), expected);
    }

    /// Tests the matrix representation of the Pauli-Z gate.
    #[test]
    fn test_pauli_z_gate() {
        let z = QuantumGate::Z;
        let expected = Matrix::new(2, 2, vec![
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0),
        ]);
        assert_eq!(z.matrix(), expected);
    }

    /// Tests the matrix representation of the Hadamard gate.
    #[test]
    fn test_hadamard_gate() {
        let h = QuantumGate::H;
        let factor = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
        let expected = Matrix::new(2, 2, vec![
            factor, factor,
            factor, -factor,
        ]);
        assert_eq!(h.matrix(), expected);
    }

    /// Tests the matrix representation of the S gate.
    #[test]
    fn test_s_gate() {
        let s = QuantumGate::S;
        let expected = Matrix::new(2, 2, vec![
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 1.0),
        ]);
        assert_eq!(s.matrix(), expected);
    }

    /// Tests the matrix representation of the T gate.
    #[test]
    fn test_t_gate() {
        let t = QuantumGate::T;
        let phase = Complex::new(0.0, std::f64::consts::PI/4.0).exp();
        let expected = Matrix::new(2, 2, vec![
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), phase,
        ]);
        assert_eq!(t.matrix(), expected);
    }
} 
