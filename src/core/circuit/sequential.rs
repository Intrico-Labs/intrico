use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{core::{QuantumGate, circuit::{GateOp, QuantumCircuit}}};

pub struct SequentialCircuit {
    n_qubits: usize,
    operations: Vec<GateOp>,
    cached_depth: Option<usize>
}

impl QuantumCircuit for SequentialCircuit {
    fn n_qubits(&self) -> usize {
        self.n_qubits
    }

    fn add_op(&mut self, op: super::GateOp) {
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
}

impl SequentialCircuit {
    // new circuit
    pub fn new(n_qubits: usize) -> Self {
        Self {
            n_qubits,
            operations: Vec::new(),
            cached_depth: None
        }
    }

    pub fn iter_ops(&self) -> impl Iterator<Item=&GateOp> {
        self.operations.iter()
    }

    pub fn num_gates(&self) -> usize {
        self.operations.len()
    }

    // Standard single-qubit gates
    pub fn x(&mut self, target: usize) -> &mut Self {
        if target > self.n_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.n_qubits);
        }
        let op = GateOp::new(QuantumGate::x(), &[target], &[]);
        self.add_op(op);
        self
    }

    pub fn y(&mut self, target: usize) -> &mut Self {
        if target > self.n_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.n_qubits);
        }
        let op = GateOp::new(QuantumGate::y(), &[target], &[]);
        self.add_op(op);
        self
    }

    pub fn z(&mut self, target: usize) -> &mut Self {
        if target > self.n_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.n_qubits);
        }
        let op = GateOp::new(QuantumGate::z(), &[target], &[]);
        self.add_op(op);
        self
    }

    pub fn h(&mut self, target: usize) -> &mut Self {
        if target > self.n_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.n_qubits);
        }
        let op = GateOp::new(QuantumGate::h(), &[target], &[]);
        self.add_op(op);
        self
    }

    pub fn s(&mut self, target: usize) -> &mut Self {
        if target > self.n_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.n_qubits);
        }
        let op = GateOp::new(QuantumGate::s(), &[target], &[]);
        self.add_op(op);
        self
    }

    pub fn t(&mut self, target: usize) -> &mut Self {
        if target > self.n_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.n_qubits);
        }
        let op = GateOp::new(QuantumGate::t(), &[target], &[]);
        self.add_op(op);
        self
    }

    // Parameterized rotation gates
    pub fn rx(&mut self, target: usize, theta: f64) -> &mut Self {
        if target > self.n_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.n_qubits);
        }
        let op = GateOp::new(QuantumGate::rx(theta), &[target], &[]);
        self.add_op(op);
        self
    }

    pub fn ry(&mut self, target: usize, theta: f64) -> &mut Self {
        if target > self.n_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.n_qubits);
        }
        let op = GateOp::new(QuantumGate::ry(theta), &[target], &[]);
        self.add_op(op);
        self
    }

    pub fn rz(&mut self, target: usize, theta: f64) -> &mut Self {
        if target > self.n_qubits-1 {
            panic!("Target qubit out of index (The circuit has only {} qubits)", self.n_qubits);
        }
        let op = GateOp::new(QuantumGate::rz(theta), &[target], &[]);
        self.add_op(op);
        self
    }

    // Two-qubit controlled gates
    pub fn cx(&mut self, control: usize, target: usize) -> &mut Self {
        if control > self.n_qubits-1 || target > self.n_qubits-1 {
            panic!("Qubit index out of bounds (The circuit has only {} qubits)", self.n_qubits);
        }
        if control == target {
            panic!("Control and target qubits must be different");
        }
        let op = GateOp::new(QuantumGate::cx(), &[target], &[control]);
        self.add_op(op);
        self
    }

    pub fn cz(&mut self, control: usize, target: usize) -> &mut Self {
        if control > self.n_qubits-1 || target > self.n_qubits-1 {
            panic!("Qubit index out of bounds (The circuit has only {} qubits)", self.n_qubits);
        }
        if control == target {
            panic!("Control and target qubits must be different");
        }
        let op = GateOp::new(QuantumGate::cz(), &[target], &[control]);
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
            self.n_qubits,
            self.num_gates()
        )?;

        if self.num_gates() == 0 {
            writeln!(f, "   (empty circuit)")?;
            return Ok(())
        }

        for (idx, op) in self.operations.iter().enumerate() {
            let name = op.gate().name().clone().unwrap_or("Custom".to_string());
            let qubits = if op.targets().len() == 1 {
                format!("q[{}]", op.targets()[0])
            } else {
                format!("q{:?}", op.targets())
            };

            writeln!(
                f, 
                "   Gate {}: {} -> {}",
                idx+1,
                name,
                qubits,
            )?;
        }

        Ok(())
    }
}