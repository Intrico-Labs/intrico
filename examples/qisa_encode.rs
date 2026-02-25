use std::error::Error;

use intrico::{QuantumCircuit, encoder::qisa::QisaEncoder};

fn main() -> Result<(), Box<dyn Error>> {
    let mut qc = QuantumCircuit::new(2);

    // Bell pair circuit
    qc.h(0).cx(0, 1).measure_all();

    // Encode to QISA bytes
    let bytes = QisaEncoder::encode(&qc)?;

    println!("==== QISA Program Bytes ====\n{:0x?}", bytes);

    Ok(())
}

/*
Output bytes breakdown:

Header:
4d, 54, 55, 41, (magic bytes)
1, 0, (version)
0, 0, (flags)

2, 0, 0, 0, (logical qubit count)
2, 0, 0, 0, (classical reg count)

7, 0, 0, 0, 0, 0, 0, 0, (instr count)

40, 0, 0, 0, 0, 0, 0, 0, (const pool offset)
40, 0, 0, 0, 0, 0, 0, 0, (instr pool offset)

0, 0, 0, 0, 0, 0, 0, 0, (const pool size)
20, 0, 0, 0, 0, 0, 0, 0, (instr pool size)
fb, e9, 5b, e3, 9e, 45, d0, a9 (checksum)

Instructions:
1, 0, 0, 0, 0, qinit(0)
1, 1, 0, 0, 0, qinit(1)
10, 0, 0, 0, 0, h(0)
20, 0, 0, 0, 0, 1, 0, 0, 0, cx(0, 1)
40, 0, 0, 0, 0, 0, 0, 0, 0, meas(0, 0)
40, 1, 0, 0, 0, 1, 0, 0, 0, meas(1, 1)
f0, qend

Footer: 6f, 52, d8, 9f, 35, 20, 54, 3f, 0, 0, 0, 0, 0, 0, 0, 0
*/
