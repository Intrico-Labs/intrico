use intrico::{QuantumCircuit, QuantumGate, Qubit};
use rusticle::{complex::Complex, linalg::Matrix};

fn main() {
    let mut qc = QuantumCircuit::new(1);

    let x_mat = Matrix::new(2, 2, vec![
        Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)
    ]);

    let custom = QuantumGate::Custom(x_mat, "X-Gate".to_string(), "G".to_string());

    qc.add_gate(custom.clone(), 0);

    qc.display();

    let mut q = Qubit::zero();

    q.apply(custom.clone());

    println!("{:?}", q);
}