use std::{cmp::{max, min}, sync::Arc};

use rusticle::{Complex};

use crate::{backend::{BackendConfig, BackendResult, CompiledCircuit, ExecutionContext, QuantumBackend, contexts::{CompiledCircuitMetadata, MeasurementOp, NativeOp}, results::ExecutionMetrics}, core::{Amplitude, gate::Gate}, ir::CircuitIR};

pub struct StatevectorBackend {
    config: BackendConfig,
    // capabilities (supports_simd, supports_gpu_accel, etc)
    // logging (enable_metrics, etc)
}

impl QuantumBackend for StatevectorBackend {

    fn transpile(&self, circuit_ir: &CircuitIR) -> CompiledCircuit {

        let mut ops: Vec<NativeOp> = vec![];

        // Measurements
        let mut measurements: Vec<MeasurementOp> = vec![];

        // Classical register mapping
        let creg_size = 0;
        let mut creg_mapping: Vec<(usize, usize)> = vec![];


        match self.config.optimization_level {
            0 => {
                // No optimizations
                let gate_ops = circuit_ir.ops();
                let creg_size = gate_ops.len();
                for (idx, op) in gate_ops.iter().enumerate() {
                    // Copy gateop to nativeop as it is
                    let n_op = NativeOp::new(op.gate().clone(), op.gate().params().copied(), op.controls().clone(), op.targets().clone());

                    ops.push(n_op);
                    creg_mapping.push((idx, idx));

                    let meas = MeasurementOp::new(idx, idx);

                    measurements.push(meas);
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
            vec![],
            measurements,
            creg_size,
            creg_mapping,
            CompiledCircuitMetadata::new(None),
        )
    }
    
    fn prepare(&self, compiled: Arc<CompiledCircuit>, rng_seed: Option<usize>) -> ExecutionContext {
        ExecutionContext::new(compiled, rng_seed)
    }
    
    fn execute(&self, execution_ctx: &mut ExecutionContext, shots: usize) -> BackendResult {
        
        let gateops = {
            execution_ctx.compiled_ref().ops().to_vec()
        };
        
        let statevec = execution_ctx.statevector();

        let n = statevec.len();

        for op in gateops {
            let gate = op.gate();
            
            match gate.gate() {
                Gate::OneQubit { matrix } => {
                    apply_single_qubit_gate(matrix, statevec, op.targets()[0], n);
                },
                Gate::TwoQubit { matrix } => {
                    apply_two_qubit_gate(matrix, statevec, op.controls()[0], op.targets()[0], n);
                },
                Gate::Param1Q { gate, params } => todo!(),
                Gate::Controlled { controls, gate } => todo!(),
                Gate::Custom { arity, matrix } => todo!(),
            }
            
        }

        let metrics = ExecutionMetrics::new(1.0, 10);

        BackendResult::new(vec![], statevec.to_vec(), metrics, 12345)

    }
    
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
    pub fn new(config: BackendConfig) -> Self {
        StatevectorBackend { config }
    }
}