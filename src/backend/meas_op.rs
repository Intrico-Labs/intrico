//! Measurement operation representation.
//!
//! This module defines the structure for measurement operations that map
//! qubit measurements to classical register indices.

/// Measurement operation for mapping qubit to classical register.
///
/// Represents a single measurement that collapses a qubit's state and
/// stores the result in a classical register.
#[derive(Debug)]
pub struct MeasurementOp {
    creg_index: usize,
    qubit_index: usize,
}

impl MeasurementOp {
    /// Creates a new measurement operation.
    pub fn new(creg_index: usize, qubit_index: usize) -> Self {
        Self {
            creg_index,
            qubit_index
        }
    }

    /// Returns the qubit index being measured.
    pub fn qubit_index(&self) -> usize {
        self.qubit_index
    }

    /// Returns the classical register index for storing the result.
    pub fn creg_index(&self) -> usize {
        self.creg_index
    }
}