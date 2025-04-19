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
intrico = "0.4.0"
```

## Quick Start

```rust
use intrico::qsim_core::{Qubit, QuantumGate};

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

### Quantum Bits (Qubits)
Comprehensive support for quantum state representation and manipulation

#### Example

```rust
use intrico::qsim_core::{Qubit, QuantumGate};
use rusticle::complex::Complex;

// Create basis states
let zero = Qubit::zero();  // |0⟩ state
let one = Qubit::one();    // |1⟩ state

// Create superposition states
let alpha = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
let beta = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
let superposition = Qubit::new(alpha, beta);  // |+⟩ state

// Apply quantum gates
let mut qubit = Qubit::zero();
qubit.apply(QuantumGate::X);  // Apply Pauli-X (NOT) gate
qubit.apply(QuantumGate::H);  // Apply Hadamard gate

// Measure probabilities
let p0 = qubit.probability_zero();
let p1 = qubit.probability_one();
```

### Quantum Gates
Implementation of fundamental quantum gates with their matrix representations

```rust
use intrico::qsim_core::QuantumGate;

// Pauli gates
let x_gate = QuantumGate::X;  // Pauli-X (quantum NOT)
let y_gate = QuantumGate::Y;  // Pauli-Y
let z_gate = QuantumGate::Z;  // Pauli-Z

// Phase gates
let s_gate = QuantumGate::S;  // S gate (√Z)
let t_gate = QuantumGate::T;  // T gate (π/4 phase)

// Hadamard gate
let h_gate = QuantumGate::H;  // Creates superposition
```

### State Visualization
Dirac notation formatting for quantum states

```rust
use intrico::qsim_core::Qubit;

let qubit = Qubit::zero();
println!("{}", qubit);  // Prints: |ψ⟩ = |0⟩

let qubit = Qubit::one();
println!("{}", qubit);  // Prints: |ψ⟩ = |1⟩

// For superposition states
let alpha = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
let beta = Complex::new(1.0/2.0_f64.sqrt(), 0.0);
let superposition = Qubit::new(alpha, beta);
println!("{}", superposition);  // Prints: |ψ⟩ = (0.7071067811865475+0i)|0⟩ + (0.7071067811865475+0i)|1⟩
```

## License

This project is licensed under the Apache-2.0 License - see the LICENSE file for details.