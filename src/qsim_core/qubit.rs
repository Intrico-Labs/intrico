use std::fmt;
use rusticle::complex::Complex;

/// Represents a quantum bit (qubit) with its state vector
#[derive(Clone)]
pub struct Qubit {
    /// The state vector of the qubit represented as a complex vector
    /// [alpha, beta] where |ψ⟩ = α|0⟩ + β|1⟩
    state: [Complex; 2],
}

impl Qubit {
    /// Creates a new qubit with a custom state vector
    /// # Arguments
    /// * `alpha` - The complex amplitude for |0⟩ state
    /// * `beta` - The complex amplitude for |1⟩ state
    /// 
    /// # Panics
    /// Panics if the state vector is not normalized (|α|² + |β|² ≠ 1)
    pub fn new(alpha: Complex, beta: Complex) -> Self {
        let norm = alpha.norm_squared() + beta.norm_squared();
        if (norm - 1.0).abs() > 1e-10 {
            panic!("State vector must be normalized");
        }
        Qubit {
            state: [alpha, beta],
        }
    }

    /// Creates a qubit in the |0⟩ state
    pub fn zero() -> Self {
        Qubit {
            state: [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
        }
    }

    /// Creates a qubit in the |1⟩ state
    pub fn one() -> Self {
        Qubit {
            state: [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
        }
    }

    /// Returns the probability of measuring the qubit in the |0⟩ state
    pub fn probability_zero(&self) -> f64 {
        self.state[0].norm_squared()
    }

    /// Returns the probability of measuring the qubit in the |1⟩ state
    pub fn probability_one(&self) -> f64 {
        self.state[1].norm_squared()
    }

    /// Returns the state vector of the qubit
    pub fn state_vector(&self) -> [Complex; 2] {
        self.state
    }

    /// Returns true if the qubit is in a basis state (|0⟩ or |1⟩)
    pub fn is_basis_state(&self) -> bool {
        (self.state[0] == Complex::new(1.0, 0.0) && self.state[1] == Complex::new(0.0, 0.0)) || 
        (self.state[0] == Complex::new(0.0, 0.0) && self.state[1] == Complex::new(1.0, 0.0))
    }
}

impl Default for Qubit {
    fn default() -> Self {
        Qubit::zero()
    }
}

impl fmt::Display for Qubit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (alpha, beta) = (self.state[0], self.state[1]);
        if alpha.norm_squared() == 1.0 {
            write!(f, "|ψ⟩ = |0⟩")
        } else if beta.norm_squared() == 1.0 {
            write!(f, "|ψ⟩ = |1⟩")
        } else {
            write!(f, "|ψ⟩ = {:?}|0⟩ + {:?}|1⟩", alpha, beta)
        }
    }
}

impl fmt::Debug for Qubit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (alpha, beta) = (self.state[0], self.state[1]);
        if alpha.norm_squared() == 1.0 {
            write!(f, "|ψ⟩ = |0⟩")
        } else if beta.norm_squared() == 1.0 {
            write!(f, "|ψ⟩ = |1⟩")
        } else {
            write!(f, "|ψ⟩ = {:?}|0⟩ + {:?}|1⟩", alpha, beta)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusticle::complex::Complex;

    #[test]
    fn test_zero_state() {
        let qubit = Qubit::zero();
        assert_eq!(qubit.state_vector()[0], Complex::new(1.0, 0.0));
        assert_eq!(qubit.state_vector()[1], Complex::new(0.0, 0.0));
        assert_eq!(qubit.probability_zero(), 1.0);
        assert_eq!(qubit.probability_one(), 0.0);
    }

    #[test]
    fn test_one_state() {
        let qubit = Qubit::one();
        assert_eq!(qubit.state_vector()[0], Complex::new(0.0, 0.0));
        assert_eq!(qubit.state_vector()[1], Complex::new(1.0, 0.0));
        assert_eq!(qubit.probability_zero(), 0.0);
        assert_eq!(qubit.probability_one(), 1.0);
    }

    #[test]
    fn test_custom_state() {
        let alpha = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
        let beta = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
        let qubit = Qubit::new(alpha, beta);
        assert!((qubit.probability_zero() - 0.5).abs() < 1e-10);
        assert!((qubit.probability_one() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_complex_state() {
        let alpha = Complex::new(0.5, 0.5);
        let beta = Complex::new(0.5, -0.5);
        let qubit = Qubit::new(alpha, beta);
        assert!((qubit.probability_zero() - 0.5).abs() < 1e-10);
        assert!((qubit.probability_one() - 0.5).abs() < 1e-10);
    }

    #[test]
    #[should_panic(expected = "State vector must be normalized")]
    fn test_invalid_state() {
        Qubit::new(
            Complex::new(0.5, 0.0),
            Complex::new(0.5, 0.0)
        );
    }
}
