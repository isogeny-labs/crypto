use p739_cross_session_audit::{execute, read_request, write_outputs};
use serde_json::json;
use std::path::Path;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: p739-cross-session-audit REQUEST.json OUTPUT_DIR".into());
    }
    let request = read_request(Path::new(&args[1]))?;
    let (result, rows) = execute(&request)?;
    write_outputs(Path::new(&args[2]), &result, &rows)?;
    println!(
        "{}",
        json!({
            "schema": result.schema,
            "exact_deterministic_solver_accounting_equivalence": result.exact_deterministic_solver_accounting_equivalence,
            "mapped_records": result.mapped_records,
            "projection_sha256": result.deterministic_projection_sha256,
            "new_solver_executions": result.new_solver_executions,
            "new_measurements": result.new_measurements
        })
    );
    Ok(())
}
