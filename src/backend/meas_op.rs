#[derive(Debug)]
pub struct MeasurementOp {
    creg_index: usize,
    qubit_index: usize,
    
}

impl MeasurementOp {
    pub fn new(creg_index: usize, qubit_index: usize) -> Self {
        Self {
            creg_index,
            qubit_index
        }
    }

    pub fn qubit_index(&self) -> usize {
        self.qubit_index
    }

    pub fn creg_index(&self) -> usize {
        self.creg_index
    }
}