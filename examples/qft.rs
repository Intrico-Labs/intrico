use intrico::{circuit::QuantumCircuit, library::qft::QFT, state::QuantumState};

fn main() {
    // Initializing the circuit to |110⟩ state
    let mut qc = QuantumCircuit::new(3);
    qc.x(2).x(1);

    // Appending the QFT circuit
    let qft = QFT::new(3);
    qc.append(&qft);

    let mut statevec = QuantumState::new(3);

    println!("Statevec before execution: {}", statevec);

    qc.execute_on_state(&mut statevec);

    println!("Statevec after execution: {:.4}", statevec);
}