use rusticle::complex::Complex;
use rand::{distr::weighted::WeightedIndex, prelude::*, rng};

use crate::QuantumCircuit;

/// Represents the available simulation backends
#[derive(Debug, Clone, PartialEq)]
pub enum Backend {
    /// Statevector simulation backend
    StateVector,
}

impl Default for Backend {
    fn default() -> Self {
        Backend::StateVector
    }
}

/// Simulator result that stores all the necessary counts
/// and states after running the simulation
#[derive(Debug)]
pub struct SimulationResult {
    /// Number of shots executed
    pub shots: usize,
    /// Final state of the qubits after simulation
    pub final_state: Vec<Complex>,
    /// Measurement counts for each basis state
    pub counts: std::collections::HashMap<String, usize>,
}

/// A quantum circuit simulator that executes quantum circuits
/// using various simulation backends
#[derive(Debug)]
pub struct Simulator {
    /// Name of the simulator
    pub name: String,
    /// The simulation backend to use
    pub backend: Backend,
    /// Quantum circuit
    pub circuit: Option<QuantumCircuit>,
    
}

impl Default for Simulator {
    fn default() -> Self {
        Simulator {
            name: "Simulator".to_string(),
            backend: Backend::default(),
            circuit: None,
        }
    }
}

impl Simulator {
    /// Creates a new simulator with the default backend
    /// 
    /// # Examples
    /// ```
    /// use intrico::Simulator;
    /// 
    /// let sim = Simulator::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new simulator with a specific backend
    /// 
    /// # Examples
    /// ```
    /// use intrico::Simulator;
    /// 
    /// let sim = Simulator::with_backend(Backend::StateVector);
    /// ```
    pub fn with_backend(backend: Backend) -> Self {
        Simulator {
            name: "Simulator".to_string(),
            backend,
            circuit: None,
        }
    }

    /// Sets the name for the simulator
    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    /// Sets the name for the simulator 
    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    /// Sets a circuit to the simulator
    pub fn set_circuit(&mut self, circuit: QuantumCircuit) {
        self.circuit = Some(circuit);
    }

    /// Add a circuit to the simulator 
    /// 
    /// # Examples
    /// ```
    /// use intrico::Simulator;
    /// 
    /// let sim = Simulator::new()
    ///     .with_circuit(QuantumCircuit::new(2));
    /// ```
    pub fn with_circuit(mut self, circuit: QuantumCircuit) -> Self {
        self.circuit = Some(circuit);
        self
    }

    /// Run the simulator with the specified number of shots
    /// 
    /// # Examples
    /// ```
    /// use intrico::Simulator;
    /// 
    /// let mut qc = QuantumCircuit::new(2);
    /// qc.h(0);
    /// qc.cnot(0, 1);
    /// 
    /// let sim = Simulator::new()
    ///     .with_circuit(qc);
    /// let result = sim.run(1000);
    /// ```
    pub fn run(&self, shots: usize) -> SimulationResult {
        let circuit = self.circuit.as_ref()
            .expect("No circuit provided to simulator. Use with_circuit() or set_circuit() to add a circuit.");
        
        let final_state = circuit.execute();

        // Calculate probabilities
        let probabilities: Vec<f64> = final_state.iter().map(|amp| amp.norm_squared()).collect();

        // Sample measurements
        let dist = WeightedIndex::new(&probabilities).unwrap();
        let mut rng = rng();
        let mut counts = std::collections::HashMap::new();

        let num_qubits = circuit.num_qubits();
        for _ in 0..shots {
            let idx = dist.sample(&mut rng);
            let bitstring = format!("{:0width$b}", idx, width = num_qubits);

            *counts.entry(bitstring).or_insert(0) += 1;
        }

        SimulationResult { shots, final_state, counts }
    }
}
