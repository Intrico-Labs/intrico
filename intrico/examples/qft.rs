use intrico::{ExecutionContext, QuantumCircuit};
use intrico::library::qft::QFT;

fn main() {
    // Initializing the circuit to |110⟩ state
    let mut qc = QuantumCircuit::new(3);
    qc.x(2).x(1);

    // Appending the QFT circuit
    let qft = QFT::new(3);
    qc.append(&qft);

    let mut ctx = ExecutionContext::new(3, 0);

    println!("Statevec before execution: {}", ctx.state());

    ctx.run(&qc);

    println!("Statevec after execution: {:.4}", ctx.state());
}
