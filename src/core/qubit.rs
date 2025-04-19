use std::fmt;
use rusticle::complex::{Complex, ComplexVector};
use crate::core::gate::QuantumGate;

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
    /// 
    /// # Examples
    /// ```
    /// use rusticle::complex::Complex;
    /// use intrico::Qubit;
    /// 
    /// // Create a qubit in the |+⟩ state (equal superposition)
    /// let alpha = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
    /// let beta = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
    /// let qubit = Qubit::new(alpha, beta);
    /// ```
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
    /// 
    /// # Examples
    /// ```
    /// use rusticle::complex::Complex;
    /// use intrico::Qubit;
    /// 
    /// let qubit = Qubit::zero();
    /// assert_eq!(qubit.state_vector().components[0], Complex::new(1.0, 0.0));
    /// assert_eq!(qubit.state_vector().components[1], Complex::new(0.0, 0.0));
    /// ```
    pub fn zero() -> Self {
        Qubit {
            state: ComplexVector::new(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]),
        }
    }

    /// Creates a qubit in the |1⟩ state
    /// 
    /// # Examples
    /// ```
    /// use rusticle::complex::Complex;
    /// use intrico::Qubit;
    /// 
    /// let qubit = Qubit::one();
    /// assert_eq!(qubit.state_vector().components[0], Complex::new(0.0, 0.0));
    /// assert_eq!(qubit.state_vector().components[1], Complex::new(1.0, 0.0));
    /// ```
    pub fn one() -> Self {
        Qubit {
            state: ComplexVector::new(vec![Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]),
        }
    }

    /// Returns the probability of measuring the qubit in the |0⟩ state
    /// 
    /// # Examples
    /// ```
    /// use rusticle::complex::Complex;
    /// use intrico::Qubit;
    /// 
    /// let qubit = Qubit::zero();
    /// assert_eq!(qubit.probability_zero(), 1.0);
    /// 
    /// let qubit = Qubit::one();
    /// assert_eq!(qubit.probability_zero(), 0.0);
    /// ```
    pub fn probability_zero(&self) -> f64 {
        self.state.components[0].norm_squared()
    }

    /// Returns the probability of measuring the qubit in the |1⟩ state
    /// 
    /// # Examples
    /// ```
    /// use rusticle::complex::Complex;
    /// use intrico::Qubit;
    /// 
    /// let qubit = Qubit::zero();
    /// assert_eq!(qubit.probability_one(), 0.0);
    /// 
    /// let qubit = Qubit::one();
    /// assert_eq!(qubit.probability_one(), 1.0);
    /// ```
    pub fn probability_one(&self) -> f64 {
        self.state.components[1].norm_squared()
    }

    /// Returns the state vector of the qubit
    /// 
    /// # Examples
    /// ```
    /// use rusticle::complex::Complex;
    /// use intrico::Qubit;
    /// 
    /// let qubit = Qubit::zero();
    /// let state = qubit.state_vector();
    /// assert_eq!(state.components[0], Complex::new(1.0, 0.0));
    /// assert_eq!(state.components[1], Complex::new(0.0, 0.0));
    /// ```
    pub fn state_vector(&self) -> ComplexVector {
        self.state.clone()
    }

    /// Applies a quantum gate to the qubit, modifying its state.
    /// 
    /// # Arguments
    /// * `gate` - The quantum gate to apply
    /// 
    /// # Examples
    /// ```
    /// use rusticle::complex::Complex;
    /// use intrico::{Qubit, QuantumGate};
    /// 
    /// let mut qubit = Qubit::zero();
    /// qubit.apply(QuantumGate::X);
    /// assert_eq!(qubit.state_vector().components[0], Complex::new(0.0, 0.0));
    /// assert_eq!(qubit.state_vector().components[1], Complex::new(1.0, 0.0));
    /// ```
    pub fn apply(&mut self, gate: QuantumGate) {
        let gate_matrix = gate.matrix();
        self.state = gate_matrix.mul_vector(&self.state);
    }

    /// Returns true if the qubit is in a basis state (|0⟩ or |1⟩)
    /// 
    /// # Examples
    /// ```
    /// use rusticle::complex::Complex;
    /// use intrico::Qubit;
    /// 
    /// let qubit = Qubit::zero();
    /// assert!(qubit.is_basis_state());
    /// ```
    pub fn is_basis_state(&self) -> bool {
        (self.state.components[0] == Complex::new(1.0, 0.0) && self.state.components[1] == Complex::new(0.0, 0.0)) || 
        (self.state.components[0] == Complex::new(0.0, 0.0) && self.state.components[1] == Complex::new(1.0, 0.0))
    }
}

impl Default for Qubit {
    /// Creates a qubit in the |0⟩ state by default
    /// 
    /// # Examples
    /// ```
    /// use rusticle::complex::Complex;
    /// use intrico::Qubit;
    /// 
    /// let qubit = Qubit::default();
    /// assert_eq!(qubit.state_vector().components[0], Complex::new(1.0, 0.0));
    /// assert_eq!(qubit.state_vector().components[1], Complex::new(0.0, 0.0));
    /// ```
    fn default() -> Self {
        Qubit::zero()
    }
}

impl fmt::Display for Qubit {
    /// Formats the qubit state in Dirac notation
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
    /// Formats the qubit state in Dirac notation for debugging
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
