use std::{env, fs, process, time::Instant};

use intrico_core::ExecutionContext;
use qisa::core::program::Program;

mod decoder;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: intrico-node <file.qisa>");
        process::exit(1);
    }

    let path = &args[1];

    let bytes = fs::read(path).unwrap_or_else(|e| {
        eprintln!("Error reading '{}': {}", path, e);
        process::exit(1);
    });

    let program = Program::parse_from_bytes(&bytes).unwrap_or_else(|e| {
        eprintln!("Failed to parse QISA program: {}", e);
        process::exit(1);
    });

    let num_qubits = program.header.logical_qubit_count as usize;
    let num_clbits = program.header.classical_register_count as usize;

    println!("=== intrico-node ===");
    println!("File:    {}", path);
    println!("Qubits:  {}", num_qubits);
    println!("Clbits:  {}", num_clbits);
    println!("Instrs:  {}", program.instructions.len());

    let circuit = decoder::decode_program(&program).unwrap_or_else(|e| {
        eprintln!("Decode error: {}", e);
        process::exit(1);
    });

    let start = Instant::now();
    let mut ctx = ExecutionContext::new(num_qubits, num_clbits);
    ctx.run(&circuit);
    let elapsed = start.elapsed();

    let (state, creg) = ctx.into_inner();

    println!("\n=== Result ===");
    println!("Statevector:  {:.4}", state);
    if num_clbits > 0 {
        println!("Measurement:  {}", creg.bitstring());
    }
    println!("Time:         {:.4}ms", elapsed.as_secs_f64() * 1000.0);
}
