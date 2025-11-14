use crate::core::gate::Gate;

pub struct GateOp {
    gate: Gate,
    targets: Vec<usize>,
}