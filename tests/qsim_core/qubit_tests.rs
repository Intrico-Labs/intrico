use intrico::core::qubit::Qubit;
use rusticle::complex::Complex;

/// Test suite for the Qubit type.
/// 
/// These tests verify the core functionality of the Qubit type, including:
/// - Basis state initialization (|0⟩ and |1⟩)
/// - Custom state creation and normalization
/// - Probability calculations
/// - State vector operations
mod qubit_tests {
    use super::*;

    /// Tests the initialization of basis states |0⟩ and |1⟩.
    #[test]
    fn test_basis_states() {
        // Test |0⟩ state
        let zero_state = Qubit::zero();
        assert_eq!(zero_state.state_vector().components[0], Complex::new(1.0, 0.0));
        assert_eq!(zero_state.state_vector().components[1], Complex::new(0.0, 0.0));
        assert_eq!(zero_state.probability_zero(), 1.0);
        assert_eq!(zero_state.probability_one(), 0.0);
        assert!(zero_state.is_basis_state());

        // Test |1⟩ state
        let one_state = Qubit::one();
        assert_eq!(one_state.state_vector().components[0], Complex::new(0.0, 0.0));
        assert_eq!(one_state.state_vector().components[1], Complex::new(1.0, 0.0));
        assert_eq!(one_state.probability_zero(), 0.0);
        assert_eq!(one_state.probability_one(), 1.0);
        assert!(one_state.is_basis_state());
    }

    /// Tests the creation of custom quantum states with real amplitudes.
    #[test]
    fn test_custom_real_states() {
        // Test equal superposition state (|+⟩)
        let alpha = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
        let beta = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
        let superposition = Qubit::new(alpha, beta);
        
        assert!((superposition.probability_zero() - 0.5).abs() < 1e-10);
        assert!((superposition.probability_one() - 0.5).abs() < 1e-10);
        assert!(!superposition.is_basis_state());
    }

    /// Tests the creation of custom quantum states with complex amplitudes.
    #[test]
    fn test_custom_complex_states() {
        // Test state with complex amplitudes
        let alpha = Complex::new(0.5, 0.5);
        let beta = Complex::new(0.5, -0.5);
        let complex_state = Qubit::new(alpha, beta);
        
        assert!((complex_state.probability_zero() - 0.5).abs() < 1e-10);
        assert!((complex_state.probability_one() - 0.5).abs() < 1e-10);
        assert!(!complex_state.is_basis_state());
    }

    /// Tests the normalization requirement for quantum states.
    #[test]
    #[should_panic(expected = "State vector must be normalized")]
    fn test_normalization_requirement() {
        // Attempt to create a non-normalized state
        Qubit::new(
            Complex::new(0.5, 0.0),
            Complex::new(0.5, 0.0)
        );
    }

    /// Tests the Display and Debug trait implementations for Qubit.
    #[test]
    fn test_qubit_display() {
        // Test |0⟩ state display
        let zero_state = Qubit::zero();
        assert_eq!(format!("{}", zero_state), "|ψ⟩ = |0⟩");
        assert_eq!(format!("{:?}", zero_state), "|ψ⟩ = |0⟩");

        // Test |1⟩ state display
        let one_state = Qubit::one();
        assert_eq!(format!("{}", one_state), "|ψ⟩ = |1⟩");
        assert_eq!(format!("{:?}", one_state), "|ψ⟩ = |1⟩");

        // Test superposition state display
        let alpha = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
        let beta = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
        let superposition = Qubit::new(alpha, beta);
        assert!(format!("{}", superposition).contains("|ψ⟩ = "));
        assert!(format!("{:?}", superposition).contains("|ψ⟩ = "));
    }
}
