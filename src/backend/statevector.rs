//! Statevector backend implementation.
//!
//! This module implements the statevector simulation backend, which represents
//! quantum states as dense complex vectors and applies gates via matrix operations.

use std::{cmp::{max, min}, collections::{HashMap, BTreeMap}, sync::Arc, time::Instant};

use rand::{Rng, SeedableRng, rngs::StdRng};
use rusticle::Complex;

use crate::{
    backend::{
        BackendConfig, BackendResult, CompiledCircuit, ExecutableGate, ExecutionContext, MeasurementOp, NativeOp, QuantumBackend, SampleResult, compiled_circuit::CompiledCircuitMetadata, contexts::PrecomputedGate, kernels::{KernelDef, kernel_map}, results::ExecutionMetrics
    },
    core::{Amplitude, QuantumGate},
    ir::CircuitIR
};

/// Statevector quantum simulation backend.
///
/// Simulates quantum circuits using dense statevector representation
pub struct StatevectorBackend {
    config: BackendConfig,
    kernels: HashMap<usize, KernelDef>,
    // capabilities (supports_simd, supports_gpu_accel, etc)
    // logging (enable_metrics, etc)
}

impl QuantumBackend for StatevectorBackend {

    fn transpile(&self, circuit_ir: &CircuitIR) -> CompiledCircuit {

        let mut ops: Vec<NativeOp> = vec![];

        // Measurements
        let mut measurements: Vec<MeasurementOp> = vec![];

        // Classical register mapping
        let mut creg_size = 0;
        let mut creg_mapping: Vec<(usize, usize)> = vec![];


        match self.config.optimization_level {
            0 => {
                // No optimizations
                let gate_ops = circuit_ir.ops();
                for op in gate_ops {
                    match op.gate() {
                        QuantumGate::Measurement => {
                            let qubit = op.targets()[0];
                            let creg = measurements.len();
                            let meas = MeasurementOp::new(creg, qubit);

                            measurements.push(meas);

                            creg_size += 1;
                        }
                        _ => {
                            // Copy gateop to nativeop as it is
                            let n_op = NativeOp::new(op.gate(), op.controls().clone(), op.targets().clone());

                            ops.push(n_op);
                        }
                    }
                }

                for meas in &measurements {
                    creg_mapping.push((meas.creg_index(), meas.qubit_index()));
                }

            }

            // Other optimization levels
            1 => {todo!()}
            2 => {todo!()}
            _ => {
                panic!("Invalid optimization level. Available optimization levels [0, 1, 2]")
            }
        }

        CompiledCircuit::new(
            circuit_ir.num_qubits(),
            self.config.optimization_level,
            ops,
            measurements,
            creg_size,
            creg_mapping,
            CompiledCircuitMetadata::new(None),
        )
    }
    
    fn prepare(&self, compiled: Arc<CompiledCircuit>, rng_seed: Option<usize>) -> ExecutionContext {
        // clone to avoid borrow checker errors
        let compiled_clone = Arc::clone(&compiled);

        let mut ctx = ExecutionContext::new(compiled_clone, rng_seed);

        let ops = compiled.ops();

        ctx.precomputed = vec![PrecomputedGate::None; ops.len()];

        for (i, op) in ops.iter().enumerate() {
            match op.gate() {
                ExecutableGate::Unitary { matrix, arity } => {
                    match arity {
                        1 => {
                            let arr: [Amplitude;4] = [
                                matrix[0], matrix[1],
                                matrix[2], matrix[3],
                            ];
                            ctx.precomputed[i] = PrecomputedGate::OneQ(arr);
                        }
                        2 => {
                            let mut a: [Amplitude;16] = [Amplitude::new(0.0,0.0); 16];
                            for j in 0..16 { a[j] = matrix[j]; }
                            ctx.precomputed[i] = PrecomputedGate::TwoQ(a);
                        }
                        _ => {}
                    }
                }

                ExecutableGate::ParamUnitary { kernel_id, params } => {
                    let kernel = self.kernels.get(kernel_id)
                        .expect("kernel id missing in registry");
                    let mat = (kernel.eval)(&params);
                    if kernel.arity == 1 {
                        let arr: [Amplitude;4] = [ mat[0], mat[1], mat[2], mat[3] ];
                        ctx.precomputed[i] = PrecomputedGate::OneQ(arr);
                    } else if kernel.arity == 2 {
                        let mut a: [Amplitude;16] = [Amplitude::new(0.0,0.0); 16];
                        for j in 0..16 { a[j] = mat[j]; }
                        ctx.precomputed[i] = PrecomputedGate::TwoQ(a);
                    } else {
                    }
                }

                ExecutableGate::Measurement => { }
            }
        }

        ctx

    }
    
    fn execute(&self, execution_ctx: &mut ExecutionContext) -> BackendResult {

        let gateops = execution_ctx.compiled_ops().to_vec();

        let pre = execution_ctx.precomputed().to_vec();

        let statevec = execution_ctx.statevector();

        let n = statevec.len();

        let mut applied_ops = 0;

        let start_time = Instant::now();

        for (idx, op) in gateops.iter().enumerate() {
            match pre[idx] {
                PrecomputedGate::OneQ(mat) => {
                    apply_single_qubit_gate(&mat, statevec, op.targets()[0], n);

                    applied_ops += 1;
                },
                PrecomputedGate::TwoQ(mat) => {
                    apply_two_qubit_gate(&mat, statevec, op.controls()[0], op.targets()[0], n);

                    applied_ops += 1;
                },
                PrecomputedGate::None => {},
            }

        }

        let execution_time = start_time.elapsed().as_nanos();

        let metrics = ExecutionMetrics::new(execution_time, applied_ops);

        BackendResult::new(statevec.to_vec(), metrics)

    }
    
}

impl StatevectorBackend {
    /// Performs measurement sampling on a quantum state.
    ///
    /// Samples from the probability distribution defined by the statevector,
    /// returning measurement counts for each observed bitstring.
    pub fn sample(&self, state: &[Complex<f64>], shots: usize, seed: Option<u64>) -> SampleResult {

        let num_qubits = state.len().ilog2();

        let start_time = Instant::now();

        let probs = compute_probabilities(state);

        let rng_seed = if seed.is_some() {
            seed.unwrap()
        } else {
            rand::rng().random::<u64>()
        };

        let raw_counts = multinomial_sample(&probs, shots, rng_seed);

        let mut counts = BTreeMap::new();
        for (basis, c) in raw_counts.into_iter().enumerate() {
            if c == 0 { continue; }
            let bits = extract_bitstring(basis, num_qubits as usize);
            counts.insert(bits, c);
        }

        let end_time = start_time.elapsed().as_micros() as f64 / 1000.0;



        SampleResult::new(counts, shots, end_time, rng_seed)
    }
}

fn cumulative_probs(probs: &[f64]) -> Vec<f64> {
    let mut cum = Vec::with_capacity(probs.len());
    let mut sum = 0.0;

    for &p in probs {
        sum += p;
        cum.push(sum);
    }

    cum
}

fn sample_single(cum: &[f64], x: f64) -> usize {
    match cum.binary_search_by(|v| v.partial_cmp(&x).unwrap()) {
        Ok(i) => i,
        Err(i) => i,
    }
}

fn multinomial_sample(probs: &[f64], shots: usize, rng_seed: u64) -> Vec<usize> {
    let cum = cumulative_probs(probs);

    let mut counts = vec![0usize; probs.len()];

    let mut rng = StdRng::seed_from_u64(rng_seed);

    for _ in 0..shots {
        let x: f64 = rng.random();
        let idx = sample_single(&cum, x);
        counts[idx] += 1;
    }

    counts
}


fn compute_probabilities(state: &[Amplitude]) -> Vec<f64> {
    let mut probs = Vec::with_capacity(state.len());
    for amp in state {
        probs.push(amp.real * amp.real + amp.imag * amp.imag);
    }
    probs
}

fn extract_bitstring(idx: usize, num_qubits: usize) -> String {
    let mut s = String::with_capacity(num_qubits);
    for q in 0..num_qubits {
        let bit = (idx >> q) & 1;
        s.push(if bit == 1 { '1' } else { '0' });
    }
    s
}



fn apply_single_qubit_gate(matrix: &[Complex<f64>; 4], state: &mut [Amplitude], target: usize, n: usize) {
    let stride = 1 << target;
    let period = stride << 1;
    
    let mut idx = 0;
    while idx < n {
        let limit = idx + stride;
        let mut i0 = idx;

        while i0 < limit { 
            let i1 = i0 + stride;
            let (a0, a1) = (state[i0], state[i1]);

            state[i0] = matrix[0]*a0 + matrix[1]*a1;
            state[i1] = matrix[2]*a0 + matrix[3]*a1;

            i0 += 1;
        }
        idx += period;
    }
    
}

fn apply_two_qubit_gate(matrix: &[Complex<f64>; 16], state: &mut [Amplitude], control: usize, target: usize, n: usize) {

    let a = min(control, target);
    let b = max(control, target);

    let bit_t = 1 << target;
    let bit_c = 1 << control;


    let specs = n/4;

    for k in 0..specs {
        let mask1 = (1 << a) - 1;
        let temp = ((k & !mask1) << 1) | (k & mask1);

        let mask2 = (1 << b) - 1;
        let base = ((temp & !mask2) << 1) | (temp & mask2);

        let i00 = base; // 00 state
        let i01 = base | bit_t; // 01 state
        let i10 = base | bit_c; // 10 state
        let i11 = base | bit_c | bit_t; // 11 state

        // caching current values
        let (a0, a1) = (state[i00], state[i01]);
        let (a2, a3) = (state[i10], state[i11]);
    
        // multiplying matrix
        state[i00] = matrix[0] * a0 +
            matrix[1] * a1 +
            matrix[2] * a2 +
            matrix[3] * a3;

        state[i01] = matrix[4] * a0 +
            matrix[5] * a1 +
            matrix[6] * a2 +
            matrix[7] * a3;

        state[i10] = matrix[8] * a0 +
            matrix[9] * a1 +
            matrix[10] * a2 +
            matrix[11] * a3;

        state[i11] = matrix[12] * a0 +
            matrix[13] * a1 +
            matrix[14] * a2 +
            matrix[15] * a3;
    }
}



impl StatevectorBackend {
    /// Creates a new statevector backend with kernel registry.
    ///
    /// # Arguments
    /// * `config` - The backend configuration to be used for execution
    pub fn new(config: BackendConfig) -> Self {

        let kernels = kernel_map();

        StatevectorBackend { config, kernels }
    }
}