use intrico::{QuantumCircuit, plot_histogram};

fn main() {
    // Bell pair: H(0) -> CX(0, 1) -> Measure all
    let mut qc = QuantumCircuit::new(2);
    qc.h(0).cx(0, 1).measure_all();

    // Single execution
    let result = qc.execute();
    println!("=== Single Execution ===");
    println!("Statevector: {:.4}", result.statevector);
    println!("Measurement: {}", result.classical_register.bitstring());
    println!("Time: {}", result.execution_time);

    // Sampling over 1000 shots
    let sample = qc.sample(1000);
    plot_histogram(&sample);
    println!("Time: {}", sample.execution_time);
}
