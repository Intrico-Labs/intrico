use std::sync::Arc;

use rusticle::{Complex};

use crate::{backend::{BackendConfig, BackendResult, CompiledCircuit, ExecutionContext, QuantumBackend, contexts::{CompiledCircuitMetadata, MeasurementOp, NativeOp}}, core::{Amplitude, gate::Gate}, ir::CircuitIR};

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

        BackendResult {  }

    }
    
}

fn apply_single_qubit_gate(matrix: &[Complex<f64>; 4], state: &mut [Amplitude], target: usize, n: usize) {
    let stride = 1 << target;
    // let period = stride << 1;

    let (u00, u01) = (matrix[0], matrix[1]);
    let (u10, u11) = (matrix[2], matrix[3]);
    
    let mut idx = 0;
    while idx + stride < n { 
        println!("{}, {}", idx, idx+stride);
        let (a0, a1) = (state[idx], state[idx+stride]);
        state[idx] = u00*a0 + u01*a1;
        state[idx+stride] = u10*a0 + u11*a1;
        idx += 1;
    }
}

fn apply_two_qubit_gate(matrix: &[Complex<f64>; 16], state: &mut [Amplitude], control: usize, target: usize, n: usize) {

    let stride_c = 1 << control;
    let stride_t = 1 << target;

    for i in 0..n {
        let i00 = i & !(stride_c | stride_t); // both 0
        let i01 = i00 | stride_c; // ctrl 1
        let i10 = i00 | stride_t; // target 1
        let i11 = i01 | stride_t; // both 1

        if i != i00 {continue};

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