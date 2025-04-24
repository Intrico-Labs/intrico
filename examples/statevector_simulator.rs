use intrico::{simulator::Simulator, QuantumCircuit};

fn main() {
    let mut qc = QuantumCircuit::new(3);
    qc.h(0);
    qc.h(1);
    qc.h(2);
    qc.cnot(0, 2);

    qc.display();

    let states = qc.execute();

    println!("{:?}", states);

    let sim = Simulator::new()
        .with_circuit(qc);

    let result = sim.run(1024);

    println!("Result: {:?}", result);

}