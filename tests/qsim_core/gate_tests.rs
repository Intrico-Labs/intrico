use intrico::qsim_core::gate::QuantumGate;
use rusticle::complex::Complex;

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
    fn test_pauli_x_matrix() {
        let x_gate = QuantumGate::X;
        let matrix = x_gate.matrix();
        
        assert_eq!(*matrix.get(0, 0), Complex::new(0.0, 0.0));
        assert_eq!(*matrix.get(0, 1), Complex::new(1.0, 0.0));
        assert_eq!(*matrix.get(1, 0), Complex::new(1.0, 0.0));
        assert_eq!(*matrix.get(1, 1), Complex::new(0.0, 0.0));
    }

    /// Tests the matrix representation of the Pauli-Y gate.
    #[test]
    fn test_pauli_y_matrix() {
        let y_gate = QuantumGate::Y;
        let matrix = y_gate.matrix();
        
        assert_eq!(*matrix.get(0, 0), Complex::new(0.0, 0.0));
        assert_eq!(*matrix.get(0, 1), Complex::new(0.0, -1.0));
        assert_eq!(*matrix.get(1, 0), Complex::new(0.0, 1.0));
        assert_eq!(*matrix.get(1, 1), Complex::new(0.0, 0.0));
    }

    /// Tests the matrix representation of the Pauli-Z gate.
    #[test]
    fn test_pauli_z_matrix() {
        let z_gate = QuantumGate::Z;
        let matrix = z_gate.matrix();
        
        assert_eq!(*matrix.get(0, 0), Complex::new(1.0, 0.0));
        assert_eq!(*matrix.get(0, 1), Complex::new(0.0, 0.0));
        assert_eq!(*matrix.get(1, 0), Complex::new(0.0, 0.0));
        assert_eq!(*matrix.get(1, 1), Complex::new(-1.0, 0.0));
    }

    /// Tests the matrix representation of the Hadamard gate.
    #[test]
    fn test_hadamard_matrix() {
        let h_gate = QuantumGate::H;
        let matrix = h_gate.matrix();
        let factor = 1.0/2.0_f64.sqrt();
        
        assert!((matrix.get(0, 0).real - factor).abs() < 1e-10);
        assert!((matrix.get(0, 1).real - factor).abs() < 1e-10);
        assert!((matrix.get(1, 0).real - factor).abs() < 1e-10);
        assert!((matrix.get(1, 1).real + factor).abs() < 1e-10);
    }

    /// Tests the matrix representation of the S gate.
    #[test]
    fn test_s_gate_matrix() {
        let s_gate = QuantumGate::S;
        let matrix = s_gate.matrix();
        
        assert_eq!(*matrix.get(0, 0), Complex::new(1.0, 0.0));
        assert_eq!(*matrix.get(0, 1), Complex::new(0.0, 0.0));
        assert_eq!(*matrix.get(1, 0), Complex::new(0.0, 0.0));
        assert_eq!(*matrix.get(1, 1), Complex::new(0.0, 1.0));
    }

    /// Tests the matrix representation of the T gate.
    #[test]
    fn test_t_gate_matrix() {
        let t_gate = QuantumGate::T;
        let matrix = t_gate.matrix();
        let phase = Complex::new(0.0, std::f64::consts::PI/4.0).exp();
        
        assert_eq!(*matrix.get(0, 0), Complex::new(1.0, 0.0));
        assert_eq!(*matrix.get(0, 1), Complex::new(0.0, 0.0));
        assert_eq!(*matrix.get(1, 0), Complex::new(0.0, 0.0));
        assert!((*matrix.get(1, 1) - phase).norm_squared() < 1e-10);
    }

} 