//! Quantum gate definitions.
//!
//! This module defines all quantum gate types supported by Intrico,
//! including standard gates, parameterized rotations, and custom gates.

/// Quantum gate enumeration.
///
/// Represents all supported quantum gates including single-qubit, two-qubit,
/// parameterized, and custom gates.
#[derive(Clone, Debug)]
pub enum QuantumGate {
    // single qubit gates
    X,
    Y,
    Z,
    H,
    S,
    T,

    // two qubit gates
    CX,
    CZ,
    CP {theta: f64},
    Swap,

    // parameterized gates
    RX {theta: f64},
    RY {theta: f64},
    RZ {theta: f64},
    U3 {theta: f64, phi: f64, lambda: f64},

    Measurement,

    // custom gates
    Custom { name: String, arity: usize }
}

impl QuantumGate {
    /// Returns the number of qubits this gate operates on.
    pub fn arity(&self) -> usize {
        match self {
            QuantumGate::CX | QuantumGate::CZ => 2,
            QuantumGate::Custom { arity, .. } => *arity,
            _ => 1
        }
    }

    // single qubit gate builders
    pub fn x() -> Self {
        QuantumGate::X
    }

    pub fn y() -> Self {
        QuantumGate::Y
    }

    pub fn z() -> Self {
        QuantumGate::Z
    }

    pub fn h() -> Self {
        QuantumGate::H
    }

    pub fn s() -> Self {
        QuantumGate::S
    }

    pub fn t() -> Self {
        QuantumGate::T
    }

    // two qubit gate builders
    pub fn cx() -> Self {
        QuantumGate::CX
    }

    pub fn cz() -> Self {
        QuantumGate::CZ
    }

    pub fn cp(theta: f64) -> Self {
        QuantumGate::CP { theta }
    }

    pub fn swap() -> Self {
        QuantumGate::Swap
    }

    // parameterized gate builders
    pub fn rx(theta: f64) -> Self {
        QuantumGate::RX { theta }
    }

    pub fn ry(theta: f64) -> Self {
        QuantumGate::RY { theta }
    }

    pub fn rz(theta: f64) -> Self {
        QuantumGate::RZ { theta }
    }

    pub fn u3(theta: f64, phi: f64, lambda: f64) -> Self {
        QuantumGate::U3 { theta, phi, lambda }
    }

    // custom gate builder
    pub fn custom(name: String, arity: usize) -> Self {
        QuantumGate::Custom { name, arity }
    }
}

