//! Sequential circuit implementation.
//!
//! This module provides the sequential circuit type where gates are executed
//! in linear order with automatic layer assignment.

use std::fmt::{Display, Formatter, Result as FmtResult};

use smallvec::SmallVec;

use crate::{core::{QuantumGate, circuit::{GateOp, QuantumCircuit}}, ir::{CircuitIR, SequentialIR}};

/// Sequential quantum circuit.
///
/// Executes gates in linear order with automatic depth tracking.
pub struct SequentialCircuit {
    num_qubits: usize,
    operations: Vec<GateOp>,
    cached_depth: Option<usize>,
    qubit_layers: Vec<usize>
}

impl QuantumCircuit for SequentialCircuit {
    fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    fn add_op(&mut self, mut op: super::GateOp) {
        let qubits: SmallVec<[usize; 4]> = op.targets().iter()
            .chain(op.controls().iter())
            .copied()
            .collect();

        let max_layer = qubits.iter()
            .map(|&qubit| self.qubit_layers[qubit])
            .max()
            .unwrap_or(0);

        op.set_layer(max_layer);

        for &qubit in &qubits {
            self.qubit_layers[qubit] = max_layer+1;
        }

        self.operations.push(op);
    }

    fn depth(&self) -> usize {
        if self.cached_depth != None {
            self.cached_depth.unwrap()
        } else {
            // TODO: depth calculation logic here
            todo!()
        }
    }

    fn to_ir(&self) -> CircuitIR {
        let ir = SequentialIR::new(self.num_qubits, self.operations.clone());
        CircuitIR::Sequential(ir)
    }
}

impl SequentialCircuit {
    /// Creates a new sequential circuit.
    /// # Arguments:
    /// * `num_qubits` - Number of qubits in the circuit
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            operations: Vec::new(),
            cached_depth: None,
            qubit_layers: vec![0; num_qubits]
        }
    }

    /// Returns an iterator over gate operations.
    pub fn iter_ops(&self) -> impl Iterator<Item=&GateOp> {
        self.operations.iter()
    }

    /// Returns the number of gates in the circuit.
    pub fn num_gates(&self) -> usize {
        self.operations.len()
    }
}

// Builder APIs
impl SequentialCircuit {
    /// Applies Pauli-X gate to target qubit.
    ///
    /// # Arguments
    /// * `target` - Target qubit index
    pub fn x(&mut self, target: usize) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.num_qubits);
        }
        let op = GateOp::new(QuantumGate::X, &[target], &[]);
        self.add_op(op);
        self
    }

    /// Applies Pauli-Y gate to target qubit.
    ///
    /// # Arguments
    /// * `target` - Target qubit index
    pub fn y(&mut self, target: usize) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.num_qubits);
        }
        let op = GateOp::new(QuantumGate::Y, &[target], &[]);
        self.add_op(op);
        self
    }

    /// Applies Pauli-Z gate to target qubit.
    ///
    /// # Arguments
    /// * `target` - Target qubit index
    pub fn z(&mut self, target: usize) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.num_qubits);
        }
        let op = GateOp::new(QuantumGate::z(), &[target], &[]);
        self.add_op(op);
        self
    }

    /// Applies Hadamard gate to target qubit.
    ///
    /// # Arguments
    /// * `target` - Target qubit index
    pub fn h(&mut self, target: usize) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.num_qubits);
        }
        let op = GateOp::new(QuantumGate::h(), &[target], &[]);
        self.add_op(op);
        self
    }

    /// Applies S gate (phase) to target qubit.
    ///
    /// # Arguments
    /// * `target` - Target qubit index
    pub fn s(&mut self, target: usize) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.num_qubits);
        }
        let op = GateOp::new(QuantumGate::s(), &[target], &[]);
        self.add_op(op);
        self
    }

    /// Applies T gate (π/8) to target qubit.
    ///
    /// # Arguments
    /// * `target` - Target qubit index
    pub fn t(&mut self, target: usize) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.num_qubits);
        }
        let op = GateOp::new(QuantumGate::t(), &[target], &[]);
        self.add_op(op);
        self
    }

    /// Applies RX rotation gate to target qubit.
    ///
    /// # Arguments
    /// * `target` - Target qubit index
    /// * `theta` - Rotation angle in radians
    pub fn rx(&mut self, target: usize, theta: f64) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.num_qubits);
        }
        let op = GateOp::new(QuantumGate::rx(theta), &[target], &[]);
        self.add_op(op);
        self
    }

    /// Applies RY rotation gate to target qubit.
    ///
    /// # Arguments
    /// * `target` - Target qubit index
    /// * `theta` - Rotation angle in radians
    pub fn ry(&mut self, target: usize, theta: f64) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.num_qubits);
        }
        let op = GateOp::new(QuantumGate::ry(theta), &[target], &[]);
        self.add_op(op);
        self
    }

    /// Applies RZ rotation gate to target qubit.
    ///
    /// # Arguments
    /// * `target` - Target qubit index
    /// * `theta` - Rotation angle in radians
    pub fn rz(&mut self, target: usize, theta: f64) -> &mut Self {
        if target > self.num_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.num_qubits);
        }
        let op = GateOp::new(QuantumGate::rz(theta), &[target], &[]);
        self.add_op(op);
        self
    }

    /// Applies controlled-X (CNOT) gate.
    ///
    /// # Arguments
    /// * `control` - Control qubit index
    /// * `target` - Target qubit index
    pub fn cx(&mut self, control: usize, target: usize) -> &mut Self {
        if control > self.num_qubits-1 || target > self.num_qubits-1 {
            panic!("Qubit index out of bounds (The circuit has only {} qubits)", self.num_qubits);
        }
        if control == target {
            panic!("Control and target qubits must be different");
        }
        let op = GateOp::new(QuantumGate::cx(), &[target], &[control]);
        self.add_op(op);
        self
    }

    /// Applies controlled-Z gate.
    ///
    /// # Arguments
    /// * `control` - Control qubit index
    /// * `target` - Target qubit index
    pub fn cz(&mut self, control: usize, target: usize) -> &mut Self {
        if control > self.num_qubits-1 || target > self.num_qubits-1 {
            panic!("Qubit index out of bounds (The circuit has only {} qubits)", self.num_qubits);
        }
        if control == target {
            panic!("Control and target qubits must be different");
        }
        let op = GateOp::new(QuantumGate::cz(), &[target], &[control]);
        self.add_op(op);
        self
    }

    /// Applies controlled-phase gate
    /// 
    /// # Arguments
    /// * `theta` - Phase
    /// * `control` - Control qubit index
    /// * `target` - Target qubit index
    pub fn cp(&mut self, theta: f64, control: usize, target: usize) -> &mut Self {
        if control > self.num_qubits-1 || target > self.num_qubits-1 {
            panic!("Qubit index out of bounds (The circuit has only {} qubits)", self.num_qubits);
        }
        if control == target {
            panic!("Control and target qubits must be different");
        }
        let op = GateOp::new(QuantumGate::cp(theta), &[target], &[control]);
        self.add_op(op);
        self
    }
}


// Display and Debug trait implementations
impl Display for SequentialCircuit {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(
            f,
            "Sequential QuantumCircuit({} qubits, {} gates)",
            self.num_qubits,
            self.num_gates()
        )?;

        if self.num_gates() == 0 {
            writeln!(f, "   (empty circuit)")?;
            return Ok(())
        }

        for (idx, op) in self.operations.iter().enumerate() {
            let name = op.gate().clone();
            let qubits = if op.targets().len() == 1 {
                format!("q[{}]", op.targets()[0])
            } else {
                format!("q{:?}", op.targets())
            };

            writeln!(
                f, 
                "   Gate {}: {:?} -> {} (Layer {:?})",
                idx+1,
                name,
                qubits,
                op.get_layer()
            )?;
        }

        Ok(())
    }
}