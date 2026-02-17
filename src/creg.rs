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
        if index >= self.bits.len() {
            panic!("Classical register index {} out of bounds for register of size {}.", index, self.bits.len());
        }
        self.bits[index] = value;
    }

    pub fn get(&self, index: usize) -> Option<usize> {
        if index >= self.bits.len() {
            panic!("Classical register index {} out of bounds for register of size {}.", index, self.bits.len());
        }
        self.bits[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classical_register_set_get() {
        let mut creg = ClassicalRegister::new(4);
        assert_eq!(creg.get(0), None);
        creg.set(0, Some(1));
        assert_eq!(creg.get(0), Some(1));
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_classical_register_get_out_of_bounds() {
        let creg = ClassicalRegister::new(2);
        creg.get(5);
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_classical_register_set_out_of_bounds() {
        let mut creg = ClassicalRegister::new(2);
        creg.set(5, Some(1));
    }
}