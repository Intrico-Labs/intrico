//! Execution engine

pub mod context;
pub mod result;

pub use context::ExecutionContext;
pub use result::{ExecutionTime, MeasurementResult, SamplingResult};
