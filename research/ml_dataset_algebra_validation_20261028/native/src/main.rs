use boolean_matrix_trace_check::{schema::Binding, verify};
use std::{
    fs::File,
    io::{BufReader, Read},
};

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: boolean-matrix-trace-check TRACE.jsonl EXPECTED_BINDING.json".into());
    }
    let mut bytes = Vec::new();
    File::open(&args[2])
        .map_err(|e| e.to_string())?
        .take(4097)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > 4096 {
        return Err("expected binding file exceeds 4096 bytes".into());
    }
    let expected: Binding = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let trace = File::open(&args[1]).map_err(|e| e.to_string())?;
    let result = verify(BufReader::new(trace), &expected)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?
    );
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("trace rejected: {e}");
        std::process::exit(1);
    }
}
