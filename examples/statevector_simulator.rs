use intrico::{simulator::Simulator, QuantumCircuit};

fn main() {
    let mut qc = QuantumCircuit::new(2);
    qc.h(0);

    qc.display();

    let states = qc.execute();

    println!("{:?}", states);

    let sim = Simulator::new()
        .with_circuit(qc);

    let result = sim.run(1024);

    println!("Result: {:?}", result);

}