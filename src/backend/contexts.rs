use smallvec::SmallVec;
use std::{fmt::Debug, sync::Arc};

use crate::core::{Amplitude, QuantumGate};

/// The CompiledCircuit gate that is returned after transpiling a Circuit IR
pub struct CompiledCircuit {
    num_qubits: usize,
    optimization_level: u8,
    ops: Vec<NativeOp>,
    measurements: Vec<MeasurementOp>,
    creg_size: usize,
    creg_mapping: Vec<(usize, usize)>,
    metadata: CompiledCircuitMetadata,
}

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
    ParamUnitary { fun: fn(&[f64]) -> Vec<Amplitude>, params: Vec<f64>},
    // other kernel stuff (KernelCX, KernelCZ)
}

impl NativeOp {
    pub fn new(gate: QuantumGate, controls: SmallVec<[usize; 2]>, targets: SmallVec<[usize; 2]>) -> Self {
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
                fun: |params| {
                    let half_theta = params[0] / 2.0;
                    let cos_val = half_theta.cos();
                    let sin_val = half_theta.sin();
                    vec![
                        Amplitude::new(cos_val, 0.0),
                        Amplitude::new(0.0, -sin_val),
                        Amplitude::new(0.0, -sin_val),
                        Amplitude::new(cos_val, 0.0)
                    ]
                },
                params: vec![*theta]
            },
            QuantumGate::RY { theta } => ExecutableGate::ParamUnitary {
                fun: |params| {
                    let half_theta = params[0] / 2.0;
                    let cos_val = half_theta.cos();
                    let sin_val = half_theta.sin();
                    vec![
                        Amplitude::new(cos_val, 0.0),
                        Amplitude::new(-sin_val, 0.0),
                        Amplitude::new(sin_val, 0.0),
                        Amplitude::new(cos_val, 0.0)
                    ]
                },
                params: vec![*theta]
            },
            QuantumGate::RZ { theta } => ExecutableGate::ParamUnitary {
                fun: |params| {
                    let half_theta = params[0] / 2.0;
                    vec![
                        Amplitude::new(half_theta.cos(), -half_theta.sin()),
                        Amplitude::new(0.0, 0.0),
                        Amplitude::new(0.0, 0.0),
                        Amplitude::new(half_theta.cos(), half_theta.sin())
                    ]
                },
                params: vec![*theta]
            },
            QuantumGate::U3 { theta, phi, lambda } => ExecutableGate::ParamUnitary {
                fun: |params| {
                    let theta = params[0];
                    let phi = params[1];
                    let lambda = params[2];
                    let half_theta = theta / 2.0;
                    let cos_val = half_theta.cos();
                    let sin_val = half_theta.sin();
                    vec![
                        Amplitude::new(cos_val, 0.0),
                        Amplitude::new(-lambda.cos() * sin_val, -lambda.sin() * sin_val),
                        Amplitude::new(phi.cos() * sin_val, phi.sin() * sin_val),
                        Amplitude::new((phi + lambda).cos() * cos_val, (phi + lambda).sin() * cos_val)
                    ]
                },
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

#[derive(Debug)]
pub struct MeasurementOp {
    qubit_index: usize,
    creg_index: usize
}

impl MeasurementOp {
    pub fn new(qubit_index: usize, creg_index: usize) -> Self {
        Self {
            qubit_index,
            creg_index
        }
    }
}

#[derive(Debug)]
pub struct CompiledCircuitMetadata {
    depth: Option<usize>
}

impl CompiledCircuitMetadata {
    pub fn new(depth: Option<usize>) -> Self {
        Self {
            depth
        }
    }
}

impl CompiledCircuit {
    pub fn new(
        num_qubits: usize,
        optimization_level: u8,
        ops: Vec<NativeOp>,
        measurements: Vec<MeasurementOp>,
        creg_size: usize,
        creg_mapping: Vec<(usize, usize)>,
        metadata: CompiledCircuitMetadata,
    ) -> Self {
        Self {
            num_qubits,
            optimization_level,
            ops,
            measurements,
            creg_size,
            creg_mapping,
            metadata,
        }
    }

    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    pub fn optimization_level(&self) -> u8 {
        self.optimization_level
    }

    pub fn ops(&self) -> &Vec<NativeOp> {
        &self.ops
    }

    pub fn measurements(&self) -> &Vec<MeasurementOp> {
        &self.measurements
    }

    pub fn creg_size(&self) -> usize {
        self.creg_size
    }

    pub fn creg_mapping(&self) -> &Vec<(usize, usize)> {
        &self.creg_mapping
    }

    pub fn metadata(&self) -> &CompiledCircuitMetadata {
        &self.metadata
    }
}

impl Debug for CompiledCircuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledCircuit").field("num_qubits", &self.num_qubits).field("optimization_level", &self.optimization_level).field("ops", &self.ops).field("measurements", &self.measurements).field("creg_size", &self.creg_size).field("creg_mapping", &self.creg_mapping).field("metadata", &self.metadata).finish()
    }
}

/// The ExecutionContext holds everything about the Quantum State
/// This is generated by preparing the CompiledCircuit for execution
pub struct ExecutionContext {
    compiled_ref: Arc<CompiledCircuit>,
    statevector: Vec<Amplitude>,
    fused_matrices: Vec<Vec<Amplitude>>,
    rng_seed: Option<usize>,
    // metrics: ExecutionMetrics
}

impl ExecutionContext {
    pub fn new(compiled_circuit: Arc<CompiledCircuit>, rng_seed: Option<usize>) -> Self {

        let dimension = 1 << compiled_circuit.num_qubits;

        // create new statevector and set to ket 00..0
        let mut statevector = vec![Amplitude::new(0.0, 0.0); dimension];
        statevector[0] = Amplitude::new(1.0, 0.0);


        ExecutionContext {
            compiled_ref: compiled_circuit,
            statevector,
            fused_matrices: vec![],
            rng_seed
        }
    }

    pub fn compiled_ref(&self) -> &Arc<CompiledCircuit> {
        &self.compiled_ref
    }

    pub fn statevector(&mut self) -> &mut [Amplitude] {
        self.statevector.as_mut_slice()
    }

    pub fn fused_matrices(&self) -> &Vec<Vec<Amplitude>> {
        &self.fused_matrices
    }

    pub fn rng_seed(&self) -> Option<usize> {
        self.rng_seed
    }
}