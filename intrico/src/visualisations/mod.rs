//! Terminal visualisation utilities for quantum circuit results.
//!
//! # Functions
//! - [`plot_histogram`] — renders a bar chart of [`SamplingResult`] counts to stdout.

pub mod histogram;

pub use histogram::plot_histogram;
