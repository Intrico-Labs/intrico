use rusticle::{Complex, Matrix};

use crate::core::Amplitude;

#[derive(Clone, Debug)]
pub enum Gate {
    // single qubit gates
    OneQubit {
        matrix: [Amplitude; 4],
    },

    // two qubit gates
    TwoQubit {
        matrix: [Amplitude; 16],
    },

    // parameterized gates
    Param1Q {
        gate: Param1QGate,
        params: [f64; 3]
    },

    // controlled gates
    // TODO: convert CX, CZ to control gates for faster execution
    // add specific functions that execute these gates faster than
    // normal matrix multiplication
    Controlled {
        controls: usize,
        gate: Box<Gate>
    },

    // custom gates
    Custom {
        arity: usize,
        matrix: Matrix<Amplitude>
    }
}

// single qubit parameterized gates
#[derive(Clone, Debug)]
pub enum Param1QGate {
    Rz,
    Rx,
    Ry,
    Phase,
    U3
}

#[derive(Clone, Debug)]
pub struct QuantumGate {
    gate: Gate,
    arity: usize,
    name: Option<String>
}

impl QuantumGate {
    // new gate
    pub fn new(gate: Gate, arity: usize, name: Option<String>) -> Self {
        Self {
            gate,
            arity,
            name
        }
    }

    // Getters
    pub fn gate(&self) -> &Gate {
        &self.gate
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn name(&self) -> &Option<String> {
        &self.name
    }

    pub fn params(&self) -> Option<&[f64; 3]> {
        match &self.gate {
            Gate::Param1Q { params, .. } => Some(params),
            _ => None
        }
    }

    // pauli-x gate
    pub fn x() -> Self {
        let gate = Gate::OneQubit { matrix: [
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
        ] };

        Self {
            gate,
            arity: 1,
            name: Some("X".to_string())
        }
    }

    // pauli-y gate
    pub fn y() -> Self {
        let gate = Gate::OneQubit { matrix: [
            Complex::new(0.0, 0.0),
            Complex::new(0.0, -1.0),
            Complex::new(0.0, 1.0),
            Complex::new(0.0, 0.0),
        ] };

        Self {
            gate,
            arity: 1,
            name: Some("Y".to_string())
        }
    }

    // pauli-z gate
    pub fn z() -> Self {
        let gate = Gate::OneQubit { matrix: [
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(-1.0, 0.0),
        ] };

        Self {
            gate,
            arity: 1,
            name: Some("Z".to_string())
        }
    }

    // hadamard gate
    pub fn h() -> Self {
        let inv_sqrt2 = 1.0 / 2.0_f64.sqrt();
        let gate = Gate::OneQubit { matrix: [
            Complex::new(inv_sqrt2, 0.0),
            Complex::new(inv_sqrt2, 0.0),
            Complex::new(inv_sqrt2, 0.0),
            Complex::new(-inv_sqrt2, 0.0),
        ] };

        Self {
            gate,
            arity: 1,
            name: Some("H".to_string())
        }
    }

    // s gate (phase gate)
    pub fn s() -> Self {
        let gate = Gate::OneQubit { matrix: [
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 1.0),
        ] };

        Self {
            gate,
            arity: 1,
            name: Some("S".to_string())
        }
    }

    // t gate (π/8 gate)
    pub fn t() -> Self {
        let inv_sqrt2 = 1.0 / 2.0_f64.sqrt();
        let gate = Gate::OneQubit { matrix: [
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(inv_sqrt2, inv_sqrt2),
        ] };

        Self {
            gate,
            arity: 1,
            name: Some("T".to_string())
        }
    }

    // rotation gates (parameterized)
    pub fn rx(theta: f64) -> Self {
        let gate = Gate::Param1Q {
            gate: Param1QGate::Rx,
            params: [theta, 0.0, 0.0]
        };

        Self {
            gate,
            arity: 1,
            name: Some("RX".to_string())
        }
    }

    pub fn ry(theta: f64) -> Self {
        let gate = Gate::Param1Q {
            gate: Param1QGate::Ry,
            params: [theta, 0.0, 0.0]
        };

        Self {
            gate,
            arity: 1,
            name: Some("RY".to_string())
        }
    }

    pub fn rz(theta: f64) -> Self {
        let gate = Gate::Param1Q {
            gate: Param1QGate::Rz,
            params: [theta, 0.0, 0.0]
        };

        Self {
            gate,
            arity: 1,
            name: Some("RZ".to_string())
        }
    }

    // u3 gate (universal single-qubit gate)
    pub fn u3(theta: f64, phi: f64, lambda: f64) -> Self {
        let gate = Gate::Param1Q {
            gate: Param1QGate::U3,
            params: [theta, phi, lambda]
        };

        Self {
            gate,
            arity: 1,
            name: Some("U3".to_string())
        }
    }

    // controlled gates
    pub fn cx() -> Self {
        let gate = Gate::TwoQubit { matrix: [
            Complex::new(1.0, 0.0), 
            Complex::new(0.0, 0.0), 
            Complex::new(0.0, 0.0), 
            Complex::new(0.0, 0.0),

            Complex::new(0.0, 0.0), 
            Complex::new(1.0, 0.0), 
            Complex::new(0.0, 0.0), 
            Complex::new(0.0, 0.0),

            Complex::new(0.0, 0.0), 
            Complex::new(0.0, 0.0), 
            Complex::new(0.0, 0.0), 
            Complex::new(1.0, 0.0),

            Complex::new(0.0, 0.0), 
            Complex::new(0.0, 0.0), 
            Complex::new(1.0, 0.0), 
            Complex::new(0.0, 0.0),
        ] };

        Self {
            gate,
            arity: 2,
            name: Some("CX".to_string())
        }
    }

    pub fn cz() -> Self {
        let gate = Gate::TwoQubit { matrix: [
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0),
        ] };

        Self {
            gate,
            arity: 2,
            name: Some("CZ".to_string())
        }
    }
}