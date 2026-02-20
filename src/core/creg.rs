//! Classical Register Representation
//! 

pub struct ClassicalRegister {
    bits: Vec<Option<u8>>,
}

impl ClassicalRegister {
    pub fn new(size: usize) -> Self {
        Self { bits: vec![None; size] }
    }

    pub fn set(&mut self, index: usize, value: u8) {
        if index >= self.bits.len() {
            panic!("Classical register index {} out of bounds for register of size {}.", index, self.bits.len());
        }
        if value > 1 {
            panic!("Classical bit value must be 0 or 1, got {}.", value);
        }
        self.bits[index] = Some(value);
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index >= self.bits.len() {
            panic!("Classical register index {} out of bounds for register of size {}.", index, self.bits.len());
        }
        self.bits[index]
    }

    pub fn size(&self) -> usize {
        self.bits.len()
    }

    /// Returns the bitstring representation of the register (e.g. "101")
    /// Panics if any bit has not been measured.
    pub fn bitstring(&self) -> String {
        self.bits.iter()
            .map(|b| match b {
                Some(v) => (b'0' + v) as char,
                None => panic!("Cannot produce bitstring: register contains unmeasured bits."),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classical_register_set_get() {
        let mut creg = ClassicalRegister::new(4);
        assert_eq!(creg.get(0), None);
        creg.set(0, 1);
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
        creg.set(5, 1);
    }

    #[test]
    #[should_panic(expected = "must be 0 or 1")]
    fn test_classical_register_set_invalid_value() {
        let mut creg = ClassicalRegister::new(2);
        creg.set(0, 42);
    }

    #[test]
    fn test_bitstring() {
        let mut creg = ClassicalRegister::new(3);
        creg.set(0, 1);
        creg.set(1, 0);
        creg.set(2, 1);
        assert_eq!(creg.bitstring(), "101");
    }

    #[test]
    #[should_panic(expected = "unmeasured bits")]
    fn test_bitstring_unmeasured() {
        let mut creg = ClassicalRegister::new(3);
        creg.set(0, 1);
        creg.bitstring();
    }
}