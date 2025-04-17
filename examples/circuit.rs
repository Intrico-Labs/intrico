use intrico::QuantumCircuit;

fn main() {
    let mut qc = QuantumCircuit::new(3);

    qc.h(0);
    qc.x(0);
    qc.x(1);
    qc.y(1);
    qc.z(1);
    qc.z(2);

    println!("{}", qc);
}