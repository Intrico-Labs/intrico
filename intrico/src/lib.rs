//! # Intrico
//!
//! High-performance quantum computing simulation library for Rust.
//!
//! Intrico provides a statevector-based quantum circuit simulator with a DAG-backed
//! circuit representation. It is the user-facing layer over [`intrico_core`], adding
//! higher-level helpers, built-in algorithms, visualisation, and QISA bytecode encoding.
//!
//! # Quick start
//!
//! ```rust
//! use intrico::{QuantumCircuit, execute, sample, plot_histogram};
//!
//! // Build a Bell-pair circuit
//! let mut qc = QuantumCircuit::new(2);
//! qc.h(0).cx(0, 1).measure_all();
//!
//! // Single shot
//! let result = execute(&qc);
//!
//! // Multi-shot sampling
//! let samples = sample(&qc, 1000);
//! plot_histogram(&samples);
//! ```
//!
//! # Modules
//!
//! - [`encoder`] — compile circuits to QISA bytecode for `intrico-node` dispatch.
//! - [`library`] — pre-built algorithms: [`library::qft::QFT`].
//! - [`visualisations`] — terminal output: [`plot_histogram`].
//!
//! # Re-exports
//!
//! All core types from `intrico-core` are re-exported here so users only need a
//! single `intrico` dependency:
//!
//! [`QuantumCircuit`], [`QuantumGate`], [`GateKind`], [`QuantumState`],
//! [`ClassicalRegister`], [`ExecutionContext`], [`MeasurementResult`],
//! [`SamplingResult`], [`ExecutionTime`], [`Operation`], [`QuantumNode`], [`Complex`].

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

/// Execute a circuit once and return the full simulation result.
///
/// Creates a fresh `|0...0⟩` state, applies all gates, collapses measurements
/// probabilistically, and returns the final [`MeasurementResult`].
///
/// For repeated execution use [`sample`], which has an optimised path for
/// circuits where all measurements are at the end.
///
/// # Example
///
/// ```rust
/// use intrico::{QuantumCircuit, execute};
///
/// let mut qc = QuantumCircuit::new(1);
/// qc.h(0).measure(0, 0);
///
/// let result = execute(&qc);
/// println!("{}", result.classical_register.bitstring()); // "0" or "1"
/// ```
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

/// Sample a circuit over `shots` executions and return aggregated outcome counts.
///
/// Returns a [`SamplingResult`] whose `counts` map each measurement bitstring to
/// how many times that outcome was observed.
///
/// # Optimisation
///
/// When all measurements are at the end of the circuit (detected via
/// [`QuantumCircuit::has_terminal_measurements_only`]), gates are applied only
/// **once** and outcomes are drawn directly from the resulting probability
/// distribution — O(2^n + shots) instead of O(shots × 2^n). For circuits with
/// mid-circuit measurements the full circuit is re-executed every shot.
///
/// # Example
///
/// ```rust
/// use intrico::{QuantumCircuit, sample, plot_histogram};
///
/// let mut qc = QuantumCircuit::new(2);
/// qc.h(0).cx(0, 1).measure_all();
///
/// let result = sample(&qc, 1024);
/// plot_histogram(&result);
/// ```
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
