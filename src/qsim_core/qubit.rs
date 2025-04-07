use std::fmt;
use rusticle::complex::{Complex, ComplexVector};

/// Represents a quantum bit (qubit) with its state vector
#[derive(Clone)]
pub struct Qubit {
    /// The state vector of the qubit represented as a complex vector
    /// [alpha, beta] where |ψ⟩ = α|0⟩ + β|1⟩
    state: ComplexVector,
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
            state: ComplexVector::new(vec![alpha, beta]),
        }
    }

    /// Creates a qubit in the |0⟩ state
    pub fn zero() -> Self {
        Qubit {
            state: ComplexVector::new(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]),
        }
    }

    /// Creates a qubit in the |1⟩ state
    pub fn one() -> Self {
        Qubit {
            state: ComplexVector::new(vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]),
        }
    }

    /// Returns the probability of measuring the qubit in the |0⟩ state
    pub fn probability_zero(&self) -> f64 {
        self.state.components[0].norm_squared()
    }

    /// Returns the probability of measuring the qubit in the |1⟩ state
    pub fn probability_one(&self) -> f64 {
        self.state.components[1].norm_squared()
    }

    /// Returns the state vector of the qubit
    pub fn state_vector(&self) -> ComplexVector {
        self.state.clone()
    }

    /// Returns true if the qubit is in a basis state (|0⟩ or |1⟩)
    pub fn is_basis_state(&self) -> bool {
        (self.state.components[0] == Complex::new(1.0, 0.0) && self.state.components[1] == Complex::new(0.0, 0.0)) || 
        (self.state.components[0] == Complex::new(0.0, 0.0) && self.state.components[1] == Complex::new(1.0, 0.0))
    }
}

impl Default for Qubit {
    fn default() -> Self {
        Qubit::zero()
    }
}

impl fmt::Display for Qubit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let alpha = self.state.components[0];
        let beta = self.state.components[1];
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
        let alpha = self.state.components[0];
        let beta = self.state.components[1];
        if alpha.norm_squared() == 1.0 {
            write!(f, "|ψ⟩ = |0⟩")
        } else if beta.norm_squared() == 1.0 {
            write!(f, "|ψ⟩ = |1⟩")
        } else {
            write!(f, "|ψ⟩ = {:?}|0⟩ + {:?}|1⟩", alpha, beta)
        }
    }
}
