//! Classical Register Representation
//! 

pub struct ClassicalRegister {
    bits: Vec<Option<usize>>,
}

impl ClassicalRegister {
    pub fn new(size: usize) -> Self {
        Self { bits: vec![None; size] }
    }

    pub fn set(&mut self, index: usize, value: Option<usize>) {
        self.bits[index] = value;
    }

    pub fn get(&self, index: usize) -> Option<usize> {
        self.bits[index]
    }
}