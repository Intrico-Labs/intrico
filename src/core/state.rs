//! Quantum Statevector representation
//! 
//! This module provides a dense statevector representation where
//! amplitudes are stored in a contiguous memory buffer

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use rusticle::Complex;

/// Dense statevector representation
/// 
/// Stores all 2^n quantum state amplitudes in contiguous memory.
pub struct QuantumState {
    pub statevector: Vec<Complex>
}

impl QuantumState {
    /// Creates a new statevector and initialises to |0...0⟩
    pub fn new(num_qubits: usize) -> Self {
        let dim = 1 << num_qubits;

        // init to 00..00 state
        let mut sv = vec![Complex::new(0.0, 0.0); dim];
        sv[0] = Complex::new(1.0, 0.0);

        Self {
            statevector: sv
        }
    }

    /// Getter fn for statevector 
    /// (Use `statevector_mut()` for mutable instance)
    pub fn statevector(&self) -> &Vec<Complex> {
        &self.statevector
    }
    
    /// Getter fn for mutable statevector
    pub fn statevector_mut(&mut self) -> &mut Vec<Complex> {
        &mut self.statevector
    }
}

impl Display for QuantumState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[ ")?;

        for (i, complex) in self.statevector.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }

            write!(f, "{:.prec$}", complex, prec = f.precision().unwrap_or(4))?;
        }

        write!(f, " ]")
    }
}

impl Debug for QuantumState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("QuantumState").field("statevector", &self.statevector).finish()
    }
}