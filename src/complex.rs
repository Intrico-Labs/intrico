//! Complex Module
//! 
//! This module contains the type implementation for Complex numbers and also
//! relevant operations that make life easier.
//! 
//! Note: This module is supposed to be abstracted into a separate crate in the future

use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }

    pub fn real(&self) -> f64 {
        self.real
    }

    pub fn imag(&self) -> f64 {
        self.imag
    }
}

// Implement multiplication for Complex numbers (by reference)
impl Mul for &Complex {
    type Output = Complex;

    fn mul(self, rhs: &Complex) -> Complex {
        // (a + bi)(c + di) = (ac - bd) + (ad + bc)i
        Complex {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

// Implement multiplication for Complex numbers (by value)
impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex {
        &self * &rhs
    }
}

// Implement addition for Complex numbers (by reference)
impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: &Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

// Implement addition for Complex numbers (by value)
impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex {
        &self + &rhs
    }
}
