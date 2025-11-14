use rusticle::{Complex, Matrix};

use crate::core::Amplitude;

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
pub enum Param1QGate {
    Rz,
    Rx,
    Ry,
    Phase,
    U3
}

pub struct QuantumGate {
    gate: Gate,
    arity: usize
}

impl QuantumGate {
    // new gate
    pub fn new(gate: Gate, arity: usize) -> Self {
        Self {
            gate,
            arity
        }
    }

    // Getters
    pub fn gate(&self) -> &Gate {
        &self.gate
    }

    pub fn arity(&self) -> usize {
        self.arity
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
            arity: 1
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
            arity: 1
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
            arity: 1
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
            arity: 1
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
            arity: 1
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
            arity: 1
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
            arity: 1
        }
    }

    pub fn ry(theta: f64) -> Self {
        let gate = Gate::Param1Q {
            gate: Param1QGate::Ry,
            params: [theta, 0.0, 0.0]
        };

        Self {
            gate,
            arity: 1
        }
    }

    pub fn rz(theta: f64) -> Self {
        let gate = Gate::Param1Q {
            gate: Param1QGate::Rz,
            params: [theta, 0.0, 0.0]
        };

        Self {
            gate,
            arity: 1
        }
    }

    // controlled gates
    pub fn cx() -> Self {
        let gate = Gate::TwoQubit { matrix: [
            Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
        ] };

        Self {
            gate,
            arity: 2
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
            arity: 2
        }
    }
}