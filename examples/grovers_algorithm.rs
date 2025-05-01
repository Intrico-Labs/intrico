use intrico::{simulator::Simulator, QuantumCircuit};

fn main() {
    let mut circuit = QuantumCircuit::new(2);

    // Step 1: Superposition
    circuit.h(0);
    circuit.h(1);

    // Step 2: Oracle for |11⟩
    circuit.cz(0, 1);

    // Step 3: Diffusion operator
    circuit.h(0);
    circuit.h(1);
    circuit.x(0);
    circuit.x(1);
    circuit.cz(0, 1);
    circuit.x(0);
    circuit.x(1);
    circuit.h(0);
    circuit.h(1);

    circuit.display();

    let states = circuit.execute();

    println!("Final states: {:?}", states);

    let sim = Simulator::new()
        .with_circuit(circuit);

    let result = sim.run(1024);

    println!("Result: {:?}", result);

}