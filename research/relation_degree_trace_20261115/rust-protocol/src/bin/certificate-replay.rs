//! Construct and export proof witnesses. Timing orchestration is a separate task.
use certified_relations::{replay, System};
use std::{error::Error, fs, io::Write, path::Path};
#[path = "../external_certificate.rs"]
mod external_certificate;
use compact_speed_analysis::cas_formats;
fn read(path: &str, cap: u64) -> Result<Vec<u8>, Box<dyn Error>> {
    if fs::metadata(path)?.len() > cap {
        return Err("file byte cap".into());
    }
    Ok(fs::read(path)?)
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 5 {
        return Err("usage: certificate-replay produce|script|program|program-lift|import INPUT MODE_OR_FILE NEW_OUTPUT".into());
    }
    let system: System = serde_json::from_slice(&read(&args[2], 4 * 1024 * 1024)?)?;
    let bytes = match args[1].as_str() {
        "produce" => {
            if args[3] == "compact-sparse" {
                serde_json::to_vec(&replay::produce_sparse(&system)?.witness)?
            } else if args[3] == "compact-product" || args[3] == "expanded-product" {
                serde_json::to_vec(&replay::produce_product(
                    &system,
                    args[3] == "compact-product",
                )?)?
            } else {
                let compact = match args[3].as_str() {
                    "compact" => true,
                    "expanded" => false,
                    _ => return Err("unknown constructor".into()),
                };
                serde_json::to_vec_pretty(&replay::produce(&system, compact)?)?
            }
        }
        "script" => replay::singular_script(
            &system,
            &serde_json::from_slice(&read(&args[3], 32 * 1024 * 1024)?)?,
        )?
        .into_bytes(),
        "program" => if args[3] == "msolve" {
            external_certificate::msolve_input(&system)?
        } else {
            external_certificate::program(&system, &args[3], None)?
        }
        .into_bytes(),
        "program-lift" => {
            let basis = cas_formats::msolve_basis(
                std::str::from_utf8(&read(&args[3], 16 * 1024 * 1024)?)?,
                &system.variables,
            )?;
            external_certificate::program(
                &system,
                "slimgb",
                Some(&basis.into_iter().collect::<Vec<_>>()),
            )?
            .into_bytes()
        }
        "import" | "import-product" => {
            let members = external_certificate::parse(
                std::str::from_utf8(&read(&args[3], 16 * 1024 * 1024)?)?,
                system.variables.len(),
                system.prime,
                system.polynomials.len(),
            )?;
            if args[1] == "import-product" {
                serde_json::to_vec(&replay::complete_product(&system, Vec::new(), members)?)?
            } else {
                serde_json::to_vec_pretty(&replay::complete(&system, Vec::new(), members)?)?
            }
        }
        _ => return Err("unknown mode".into()),
    };
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(Path::new(&args[4]))?
        .write_all(&bytes)?;
    Ok(())
}
