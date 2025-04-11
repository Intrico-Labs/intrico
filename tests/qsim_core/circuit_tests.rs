#[test]
fn test_cnot_gate_display() {
    let mut qc = QuantumCircuit::new(2);
    qc.cnot(0, 1);
    let expected = "q0: ───●───\nq1: ───⊕───";
    assert_eq!(qc.display_ascii(), expected);
}

#[test]
fn test_cnot_gate_reversed_display() {
    let mut qc = QuantumCircuit::new(2);
    qc.cnot(1, 0);
    let expected = "q0: ───⊕───\nq1: ───●───";
    assert_eq!(qc.display_ascii(), expected);
}

#[test]
fn test_cnot_with_other_gates_display() {
    let mut qc = QuantumCircuit::new(3);
    qc.h(0);
    qc.cnot(0, 1);
    qc.x(2);
    let expected = "q0: ───H───●───\nq1: ──────⊕───\nq2: ───X──────";
    assert_eq!(qc.display_ascii(), expected);
}

#[test]
#[should_panic(expected = "Qubit index out of bounds")]
fn test_cnot_invalid_qubit() {
    let mut qc = QuantumCircuit::new(2);
    qc.cnot(0, 2);  // Should panic as qubit 2 doesn't exist
} 