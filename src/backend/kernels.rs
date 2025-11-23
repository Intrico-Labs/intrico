use crate::core::Amplitude;

pub const KERNEL_RX: usize = 1;
pub const KERNEL_RY: usize = 2;
pub const KERNEL_RZ: usize = 3;
pub const KERNEL_U3: usize = 4;
pub struct KernelDef {
    pub arity: usize,
    pub eval: fn(&[f64]) -> Vec<Amplitude>,
}