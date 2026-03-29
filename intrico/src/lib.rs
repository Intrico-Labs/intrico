//! # 🚀 Intrico
//!
//! ### High-performance quantum computing library for Rust
//! **Simulate quantum circuits with precision and speed**
//!

pub mod encoder;
pub mod library;
pub mod visualisations;

// Re-exports from intrico-core — users only need to depend on `intrico`
pub use intrico_core::{
    ClassicalRegister,
    Complex,
    ExecutionContext,
    ExecutionTime,
    GateKind,
    MeasurementResult,
    Operation,
    QuantumCircuit,
    QuantumGate,
    QuantumNode,
    QuantumState,
    SamplingResult,
};

pub use visualisations::plot_histogram;

/// Execute a circuit once, returning the statevector and classical register.
pub fn execute(circuit: &QuantumCircuit) -> MeasurementResult {
    use std::time::Instant;
    let start = Instant::now();
    let mut ctx = ExecutionContext::new(circuit.num_qubits(), circuit.classical_regs());
    ctx.run(circuit);
    let (statevector, classical_register) = ctx.into_inner();
    MeasurementResult {
        statevector,
        classical_register,
        shots: 1,
        execution_time: ExecutionTime::from_duration(start.elapsed()),
    }
}

/// Sample a circuit over `shots` executions, returning outcome counts.
pub fn sample(circuit: &QuantumCircuit, shots: usize) -> SamplingResult {
    use std::collections::HashMap;
    use std::time::Instant;
    use rand::RngExt;

    let start = Instant::now();
    let mut counts: HashMap<String, usize> = HashMap::new();
    let creg_size = circuit.classical_regs();

    if circuit.has_terminal_measurements_only() {
        // Optimised: run gates once, sample from the statevector probability distribution
        let mut ctx = ExecutionContext::new(circuit.num_qubits(), 0);
        ctx.run_gates_only(circuit);

        let probs: Vec<f64> = ctx
            .state()
            .statevector()
            .iter()
            .map(|a| a.norm_squared())
            .collect();

        let measurements: Vec<(usize, usize)> = circuit
            .nodes()
            .iter()
            .filter_map(|n| match &n.operation {
                Operation::Measure { qubit, classical_bit } => Some((*qubit, *classical_bit)),
                _ => None,
            })
            .collect();

        let dim = 1 << circuit.num_qubits();
        let mut rng = rand::rng();

        for _ in 0..shots {
            let r: f64 = rng.random();
            let mut cumulative = 0.0;
            let mut sampled_index = dim - 1;
            for (i, &p) in probs.iter().enumerate() {
                cumulative += p;
                if r < cumulative {
                    sampled_index = i;
                    break;
                }
            }

            let mut creg = ClassicalRegister::new(creg_size);
            for &(qubit, classical_bit) in &measurements {
                creg.set(classical_bit, ((sampled_index >> qubit) & 1) as u8);
            }
            *counts.entry(creg.bitstring()).or_insert(0) += 1;
        }
    } else {
        // Full re-execution each shot (required for mid-circuit measurement)
        for _ in 0..shots {
            let mut ctx = ExecutionContext::new(circuit.num_qubits(), creg_size);
            ctx.run(circuit);
            *counts
                .entry(ctx.classical_register().bitstring())
                .or_insert(0) += 1;
        }
    }

    SamplingResult {
        counts,
        shots,
        execution_time: ExecutionTime::from_duration(start.elapsed()),
    }
}
