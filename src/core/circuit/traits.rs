//! Quantum circuit trait definition.
//!
//! This module defines the core trait that all circuit types must implement.

use crate::{core::circuit::GateOp, ir::CircuitIR};

/// Core trait for quantum circuits.
///
/// Defines the interface that all circuit implementations must provide.
pub trait QuantumCircuit {
    /// Returns the number of qubits in the circuit.
    fn num_qubits(&self) -> usize;
    /// Adds a gate operation to the circuit.
    fn add_op(&mut self, op: GateOp);
    /// Returns the circuit depth.
    fn depth(&self) -> usize;
    // fn iter_ops(&self);
    /// Converts the circuit to intermediate representation.
    fn to_ir(&self) -> CircuitIR;
}