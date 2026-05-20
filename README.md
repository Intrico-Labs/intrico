<div align="center">

# 🚀 Intrico

**High-performance quantum computing library for Rust**

_Simulate quantum circuits with precision and speed_

[![Crates.io](https://img.shields.io/crates/v/intrico?style=flat-square)](https://crates.io/crates/intrico)
[![Documentation](https://img.shields.io/docsrs/intrico?style=flat-square)](https://docs.rs/intrico)
[![License](https://img.shields.io/crates/l/intrico?style=flat-square)](LICENSE)

</div>

## Installation

Add Intrico to your `Cargo.toml`:

```toml
[dependencies]
intrico = "2.1.1"
```

Or install using cargo:

```bash
cargo add intrico
```

## Workspace

This repository is a Cargo workspace with three crates:

| Crate | Description |
| --- | --- |
| [`intrico-sdk`](intrico-sdk) | User-facing SDK. Re-exports all core types and adds higher-level helpers, built-in algorithms (QFT), terminal visualisations, and QISA bytecode encoding. This is the crate users depend on (`intrico` on crates.io). |
| [`intrico-core`](intrico-core) | Internal execution engine. Statevector simulation, gate definitions, DAG circuit representation, and `ExecutionContext`. Not intended for direct use. |
| [`intrico-node`](intrico-node) | Standalone CLI executor. Reads a `.qisa` bytecode file produced by `intrico-sdk`'s encoder and runs it through the simulation engine. |

## Examples

| Example | Description | Link |
| --- | --- | --- |
| Bell Pair | Create and measure an entangled Bell state | [bell_pair.rs](intrico-sdk/examples/bell_pair.rs) |
| Quantum Fourier Transform | Prepare and evolve a state using QFT | [qft.rs](intrico-sdk/examples/qft.rs) |
| QISA Encoder | Encode a quantum circuit into QISA bytecode | [qisa_encode.rs](intrico-sdk/examples/qisa_encode.rs) |
