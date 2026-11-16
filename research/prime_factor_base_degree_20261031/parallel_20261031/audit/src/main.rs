use secant_complete_audit::{audit, digest, export_f4, export_inputs, Spec};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 5 || !["run", "export-inputs", "export-f4"].contains(&args[1].as_str()) {
        return Err("usage: secant-complete-audit run|export-inputs|export-f4 SPEC.json FRESH_OUTPUT_DIRECTORY FULL_SOURCE_COMMIT".into());
    }
    let bytes = fs::read(&args[2])?;
    if bytes.len() > 1_048_576 {
        return Err("spec exceeds 1 MiB".into());
    }
    let spec: Spec = serde_json::from_slice(&bytes)?;
    if args[1] == "export-inputs" {
        export_inputs(&spec, Path::new(&args[3]), &args[4], &digest(&bytes))?;
    } else if args[1] == "export-f4" {
        export_f4(&spec, Path::new(&args[3]), &args[4], &digest(&bytes))?;
    } else if !audit(&spec, Path::new(&args[3]), &args[4], &digest(&bytes))? {
        return Err("complete audit found mismatches; all raw records retained".into());
    }
    Ok(())
}
