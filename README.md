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
intrico = "2.0.1"
```

Or install using cargo:

```bash
cargo add intrico
```

## Examples

| Example                   | Description                                 | Link                                      |
| ------------------------- | ------------------------------------------- | ----------------------------------------- |
| Bell Pair                 | Create and measure an entangled Bell state  | [bell_pair.rs](examples/bell_pair.rs)     |
| Quantum Fourier Transform | Prepare and evolve a state using QFT        | [qft.rs](examples/qft.rs)                 |
| QISA Encoder              | Encode a Quantum Circuit into QISA Bytecode | [qisa_encode.rs](examples/qisa_encode.rs) |
