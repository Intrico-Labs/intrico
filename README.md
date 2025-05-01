<h1 align="center">Intrico - A new <i>intuitive</i> era of Quantum Computing</h1>
<p align="center">High-performance Quantum Computing Rust library to simulate, and visualize quantum circuits with precision and power.</p>

## Installation

Run this in your terminal
```bash
cargo add intrico
```

OR add this to your `Cargo.toml`:

```toml
[dependencies]
intrico = "1.0.0"
```

## Quick Start

```rust
// Example: Evolving a single qubit state
use intrico::{Qubit, QuantumGate};

fn main() {
    // Create a qubit in the |0⟩ state
    let mut ket_0 = Qubit::zero();
    println!("ket_0 = {:?}\t{:?}", ket_0, ket_0.state_vector());

    // Apply a Hadamard gate to create a superposition
    ket_0.apply(QuantumGate::H);

    // This should print the |+⟩ state
    println!("psi = {:?}", ket_0);
}
```

## Features
| Feature | Description |
|---------|-------------|
| `core` | Core Quantum definitions like qubits, quantum gates, gate operations, etc |
| `circuit` | Quantum Circuit functionality including visualisations |
| `simulator` | Quantum Simulation functionality |

## Examples
Checkout the [examples](./examples/) directory for all the examples. For convenience here's a list of some significant ones:

| Example | Description |
|---------|-------------|
| [`quantum-circuit`](./examples/quantum_circuit.rs) | A bell state quantum circuit visualisation |
| [`statevector-simulator`](./examples/statevector_simulator.rs) | A bell state simulation using statevector simulator |
| [`grovers-algorithm`](./examples/grovers_algorithm.rs) | A two-qubit grover's algorithm simulation |


## License

This project is licensed under the Apache-2.0 License - see the LICENSE file for details.