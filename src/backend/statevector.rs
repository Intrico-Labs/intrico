use crate::{backend::{BackendConfig, BackendResult, CompiledCircuit, ExecutionContext, QuantumBackend, contexts::{CompiledCircuitMetadata, MeasurementOp, NativeOp}}, ir::CircuitIR};

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
        let mut creg_size = 0;
        let mut creg_mapping: Vec<(usize, usize)> = vec![];


        match self.config.optimization_level {
            0 => {
                // No optimizations
                let gate_ops = circuit_ir.ops();
                creg_size = gate_ops.len();
                for (idx, op) in gate_ops.iter().enumerate() {
                    // Copy gateop to nativeop as it is
                    let n_op = NativeOp::new(op.gate().clone(), op.gate().params().copied(), op.controls().clone(), op.targets().clone());

                    ops.push(n_op);
                    creg_mapping.push((idx, idx));

                    let meas = MeasurementOp::new(idx, idx);

                    measurements.push(meas);
                }

            }
            1 => {}
            2 => {}
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
    
    fn prepare(&self, compiled_ctx: &CompiledCircuit) -> ExecutionContext {
        todo!()
    }
    
    fn execute(&self, execution_ctx: &mut ExecutionContext) -> BackendResult {
        todo!()
    }
    
}

impl StatevectorBackend {
    pub fn new(config: BackendConfig) -> Self {
        StatevectorBackend { config }
    }
}