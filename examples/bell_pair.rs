use intrico::QuantumCircuit;

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
    println!("\n=== Sampling (1000 shots) ===");
    for (bitstring, count) in &sample.counts {
        println!("|{}⟩: {} ({:.1}%)", bitstring, count, *count as f64 / sample.shots as f64 * 100.0);
    }
    println!("Time: {}", sample.execution_time);
}
