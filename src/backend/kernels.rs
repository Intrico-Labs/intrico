//! Kernel definitions for parameterized quantum gates.
//!
//! This module defines kernel IDs and the kernel evaluation structure for
//! parameterized gates like rotation gates (RX, RY, RZ) and the universal U3 gate.

use crate::core::Amplitude;

/// Kernel ID for RX (X-axis rotation) gate.
pub const KERNEL_RX: usize = 1;

/// Kernel ID for RY (Y-axis rotation) gate.
pub const KERNEL_RY: usize = 2;

/// Kernel ID for RZ (Z-axis rotation) gate.
pub const KERNEL_RZ: usize = 3;

/// Kernel ID for U3 (universal single-qubit) gate.
pub const KERNEL_U3: usize = 4;

/// Kernel definition for parameterized gates.
///
/// Contains the gate arity and an evaluation function that takes parameters
/// and returns the gate matrix as a vector of amplitudes.
pub struct KernelDef {
    /// Number of qubits the gate operates on.
    pub arity: usize,
    /// Function that evaluates the gate matrix given parameters.
    pub eval: fn(&[f64]) -> Vec<Amplitude>,
}