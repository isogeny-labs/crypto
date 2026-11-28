//! Independent scalar verification with opt-in complete arithmetic Ir framing.
#[path = "../tools/aps-export/src/value.rs"]
mod value;
pub use value::*;
#[path = "../tools/aps-export/src/audit.rs"]
pub mod audit;
use crypto_lib::cryptanalysis::ic_measurement::{
    callgrind_solve_frame_begin, callgrind_solve_frame_end,
};

fn main() {
    let result = (|| -> Result<serde_json::Value> {
        let args: Vec<_> = std::env::args().collect();
        if args.len() != 2 {
            return Err("usage: aps_scalar_check RECORD".into());
        }
        let bytes = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
        let record = audit::record_line(
            std::str::from_utf8(&bytes)
                .map_err(|e| e.to_string())?
                .trim_end_matches('\n'),
        )?;
        let verifier_binary = hash(
            &std::fs::read(std::env::current_exe().map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?,
        );
        let profile = std::env::var("ECBENCH_CALLGRIND_SOLVE").as_deref() == Ok("1");
        if profile {
            callgrind_solve_frame_begin();
        }
        let mut result = audit::scalar_record(&record);
        if profile {
            callgrind_solve_frame_end();
        }
        if let Ok(value) = &mut result {
            value["binary_sha256"] = serde_json::json!(verifier_binary);
        }
        result
    })();
    match result {
        Ok(value) => {
            println!("{value}");
            if value["matches_target"] != true {
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("aps-scalar-check: {error}");
            std::process::exit(1);
        }
    }
}
