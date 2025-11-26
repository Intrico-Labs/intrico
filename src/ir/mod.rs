//! Intermediate representation (IR) layer.
//!
//! This module provides normalized circuit representations that sit between
//! the user-facing API and backend implementations, enabling circuit portability
//! and transformation.

pub mod circuit;
pub mod metadata;
pub mod sequential;

pub use circuit::CircuitIR;
pub use metadata::IRMetadata;
pub use sequential::SequentialIR;