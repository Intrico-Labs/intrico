//! Native operations for backend execution.
//!
//! This module defines the low-level gate representations used by backends,
//! including executable gate variants and native operation structures.

use smallvec::SmallVec;

use crate::{backend::kernels::{KERNEL_CP, KERNEL_RX, KERNEL_RY, KERNEL_RZ, KERNEL_U3}, core::{Amplitude, QuantumGate}};

/// Low-level gate operation ready for backend execution.
///
/// A `NativeOp` is created during transpilation and contains the executable
/// gate representation along with control and target qubit indices.
#[derive(Debug, Clone)]
pub struct NativeOp {
    gate: ExecutableGate,
    controls: SmallVec<[usize; 2]>,
    targets: SmallVec<[usize; 2]>
}

/// Executable gate representation for backend processing.
///
/// Represents gates in their executable form, either as precomputed matrices
/// or as parameterized gates with kernel IDs.
#[derive(Debug, Clone)]
pub enum ExecutableGate {
    /// Static unitary gate with precomputed matrix.
    Unitary { matrix: Vec<Amplitude>, arity: usize },
    /// Parameterized gate referencing a kernel function.
    ParamUnitary { kernel_id: usize, params: Vec<f64>},
    /// Measurement operation.
    Measurement,
}

impl NativeOp {
    /// Converts a high-level quantum gate into a native operation.
    ///
    /// Maps gate types to their executable representations with matrices or kernel IDs.
    pub(crate) fn new(gate: &QuantumGate, controls: SmallVec<[usize; 2]>, targets: SmallVec<[usize; 2]>) -> Self {
        use std::f64::consts::FRAC_1_SQRT_2;

        let exec_gate = match &gate {
            QuantumGate::X => ExecutableGate::Unitary {
                matrix: vec![
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0)
                ],
                arity: 1
            },
            QuantumGate::Y => ExecutableGate::Unitary {
                matrix: vec![
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, -1.0),
                    Amplitude::new(0.0, 1.0),
                    Amplitude::new(0.0, 0.0)
                ],
                arity: 1
            },
            QuantumGate::Z => ExecutableGate::Unitary {
                matrix: vec![
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(-1.0, 0.0)
                ],
                arity: 1
            },
            QuantumGate::H => ExecutableGate::Unitary {
                matrix: vec![
                    Amplitude::new(FRAC_1_SQRT_2, 0.0),
                    Amplitude::new(FRAC_1_SQRT_2, 0.0),
                    Amplitude::new(FRAC_1_SQRT_2, 0.0),
                    Amplitude::new(-FRAC_1_SQRT_2, 0.0)
                ],
                arity: 1
            },
            QuantumGate::S => ExecutableGate::Unitary {
                matrix: vec![
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 1.0)
                ],
                arity: 1
            },
            QuantumGate::T => ExecutableGate::Unitary {
                matrix: vec![
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2)
                ],
                arity: 1
            },
            QuantumGate::CX => ExecutableGate::Unitary {
                matrix: vec![
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0)
                ],
                arity: 2
            },
            QuantumGate::CZ => ExecutableGate::Unitary {
                matrix: vec![
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(-1.0, 0.0)
                ],
                arity: 2
            },
            QuantumGate::CP { theta } => ExecutableGate::ParamUnitary { 
                kernel_id: KERNEL_CP, 
                params: vec![*theta]
            },
            QuantumGate::RX { theta } => ExecutableGate::ParamUnitary {
                kernel_id: KERNEL_RX,
                params: vec![*theta]
            },
            QuantumGate::RY { theta } => ExecutableGate::ParamUnitary {
                kernel_id: KERNEL_RY,
                params: vec![*theta]
            },
            QuantumGate::RZ { theta } => ExecutableGate::ParamUnitary {
                kernel_id: KERNEL_RZ,
                params: vec![*theta]
            },
            QuantumGate::U3 { theta, phi, lambda } => ExecutableGate::ParamUnitary {
                kernel_id: KERNEL_U3,
                params: vec![*theta, *phi, *lambda]
            },
            QuantumGate::Custom { name: _, arity } => {
                // For custom gates, create an identity matrix as placeholder
                let dim = 1 << arity;
                let size = dim * dim;
                let mut elements = vec![Amplitude::new(0.0, 0.0); size];
                for i in 0..dim {
                    elements[i * dim + i] = Amplitude::new(1.0, 0.0);
                }
                ExecutableGate::Unitary {
                    matrix: elements,
                    arity: *arity
                }
            },
            QuantumGate::Measurement => { 
                ExecutableGate::Measurement
            },
        };

        Self {
            gate: exec_gate,
            controls,
            targets
        }
    }

    /// Returns the executable gate.
    pub fn gate(&self) -> &ExecutableGate {
        &self.gate
    }

    /// Returns the control qubit indices.
    pub fn controls(&self) -> &SmallVec<[usize; 2]> {
        &self.controls
    }

    /// Returns the target qubit indices.
    pub fn targets(&self) -> &SmallVec<[usize; 2]> {
        &self.targets
    }
}
