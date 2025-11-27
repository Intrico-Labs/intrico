//! Kernel definitions for parameterized quantum gates.
//!
//! This module defines kernel IDs and the kernel evaluation structure for
//! parameterized gates like rotation gates (RX, RY, RZ) and the universal U3 gate.

use std::collections::HashMap;

use crate::core::Amplitude;

/// Kernel ID for RX (X-axis rotation) gate.
pub const KERNEL_RX: usize = 1;

/// Kernel ID for RY (Y-axis rotation) gate.
pub const KERNEL_RY: usize = 2;

/// Kernel ID for RZ (Z-axis rotation) gate.
pub const KERNEL_RZ: usize = 3;

/// Kernel ID for U3 (universal single-qubit) gate.
pub const KERNEL_U3: usize = 4;

/// Kernel ID for CP (controlled phase) gate.
pub const KERNEL_CP: usize = 5;

/// Kernel definition for parameterized gates.
///
/// Contains the gate arity and an evaluation function that takes parameters
/// and returns the gate matrix as a vector of amplitudes.
pub struct KernelDef {
    /// Number of qubits the gate operates on.
    pub arity: usize,
    /// Function that evaluates the gate matrix given parameters.
    pub eval: fn(&[f64]) -> Vec<Amplitude>,
}

pub(crate) fn kernel_map() -> HashMap<usize, KernelDef> {
    let mut kernels: HashMap<usize, KernelDef> = HashMap::new();

    kernels.insert(KERNEL_RX, KernelDef {
            arity: 1,
            eval: |params| {
                let th = params[0] / 2.0;
                vec![
                    Amplitude::new(th.cos(), 0.0),
                    Amplitude::new(0.0, -th.sin()),
                    Amplitude::new(0.0, -th.sin()),
                    Amplitude::new(th.cos(), 0.0),
                ]
            }
        });

        kernels.insert(KERNEL_RY, KernelDef {
            arity: 1,
            eval: |params| {
                let th = params[0] / 2.0;
                vec![
                    Amplitude::new(th.cos(), 0.0),
                    Amplitude::new(-th.sin(), 0.0),
                    Amplitude::new(th.sin(), 0.0),
                    Amplitude::new(th.cos(), 0.0),
                ]
            }
        });

        kernels.insert(KERNEL_RZ, KernelDef {
            arity: 1,
            eval: |params| {
                let th = params[0] / 2.0;
                vec![
                    Amplitude::new(th.cos(), -th.sin()),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(th.cos(), th.sin()),
                ]
            }
        });

        kernels.insert(KERNEL_U3, KernelDef {
            arity: 1,
            eval: |params| {
                let theta = params[0];
                let phi = params[1];
                let lambda = params[2];
                let th = theta / 2.0;
                vec![
                    Amplitude::new(th.cos(), 0.0),
                    Amplitude::new(-lambda.cos() * th.sin(), -lambda.sin() * th.sin()),
                    Amplitude::new(phi.cos() * th.sin(), phi.sin() * th.sin()),
                    Amplitude::new((phi + lambda).cos() * th.cos(), (phi + lambda).sin() * th.cos()),
                ]
            }
        });

        kernels.insert(KERNEL_CP, KernelDef { 
            arity: 2, 
            eval: |params| {
                let theta = params[0];

                vec![
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),

                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),

                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(1.0, 0.0),
                    Amplitude::new(0.0, 0.0),

                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(0.0, 0.0),
                    Amplitude::new(theta.cos(), theta.sin())
                ]
            } 
        });

    kernels
}