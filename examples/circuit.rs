use intrico::QuantumCircuit;

fn main() {
    let mut qc = QuantumCircuit::new(2);

    qc.h(0);
    qc.rx(0, 0.0);
    qc.cx(1, 0);
    qc.measure(0, 0);
    qc.measure(1, 1);

    println!("{}", qc);

    qc.display();
}

