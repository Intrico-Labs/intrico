use smallvec::SmallVec;

use crate::{backend::kernels::{KERNEL_RX, KERNEL_RY, KERNEL_RZ, KERNEL_U3}, core::{Amplitude, QuantumGate}};


/// A NativeOp is a low-level interpretation of the GateOp
#[derive(Debug, Clone)]
pub struct NativeOp {
    gate: ExecutableGate,
    controls: SmallVec<[usize; 2]>,
    targets: SmallVec<[usize; 2]>
}

#[derive(Debug, Clone)]
pub enum ExecutableGate {
    Unitary { matrix: Vec<Amplitude>, arity: usize },
    ParamUnitary { kernel_id: usize, params: Vec<f64>},
    Measurement,
    // other kernel stuff (KernelCX, KernelCZ)
}

impl NativeOp {
    pub fn new(gate: &QuantumGate, controls: SmallVec<[usize; 2]>, targets: SmallVec<[usize; 2]>) -> Self {
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

    pub fn gate(&self) -> &ExecutableGate {
        &self.gate
    }

    pub fn controls(&self) -> &SmallVec<[usize; 2]> {
        &self.controls
    }

    pub fn targets(&self) -> &SmallVec<[usize; 2]> {
        &self.targets
    }
}
