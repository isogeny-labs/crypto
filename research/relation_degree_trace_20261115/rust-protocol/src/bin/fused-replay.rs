//! One invocation pays construction, retained proof transfer and exact checking.
//! The bounded parent applies process-group deadlines and resource limits.
use certified_relations::{replay, System};
use compact_speed_analysis::cas_formats;
use serde_json::json;
use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
#[path = "../external_certificate.rs"]
mod external_certificate;
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn save(path: &Path, bytes: &[u8]) -> Result<()> {
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(bytes)?;
    Ok(())
}
fn read(path: &Path, cap: u64) -> Result<Vec<u8>> {
    if fs::metadata(path)?.len() > cap {
        return Err("file byte cap".into());
    }
    Ok(fs::read(path)?)
}
fn process(exe: &str, args: &[String], out: &Path, label: &str) -> Result<Vec<u8>> {
    let stdout = out.join(format!("{label}.stdout"));
    let stderr = out.join(format!("{label}.stderr"));
    let status = Command::new(exe)
        .args(args)
        .stdin(Stdio::null())
        .stdout(
            fs::File::options()
                .write(true)
                .create_new(true)
                .open(&stdout)?,
        )
        .stderr(
            fs::File::options()
                .write(true)
                .create_new(true)
                .open(&stderr)?,
        )
        .status()?;
    save(
        &out.join(format!("{label}.process.json")),
        &serde_json::to_vec(
            &json!({"program":exe,"args":args,"success":status.success(),"exit_code":status.code()}),
        )?,
    )?;
    let bytes = read(&stdout, 16 * 1024 * 1024)?;
    if !status.success() || !read(&stderr, 16 * 1024 * 1024)?.is_empty() {
        return Err(format!("fused child failed/diagnostics: {label}").into());
    }
    Ok(bytes)
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 6 {
        return Err("usage: fused-replay INPUT compact|expanded|compact-sparse|std|slimgb|msolve-lift SINGULAR MSOLVE NEW_OUTPUT".into());
    }
    let system: System = serde_json::from_slice(&read(Path::new(&args[1]), 4 * 1024 * 1024)?)?;
    let out = Path::new(&args[5]);
    fs::create_dir(out)?;
    let kind = args[2].as_str();
    let mut constructor = serde_json::Value::Null;
    let witness = match kind {
        "compact" | "expanded" => replay::produce_product(&system, kind == "compact")?,
        "compact-sparse" => {
            let produced = replay::produce_sparse(&system)?;
            constructor = json!({"info":produced.constructor,"subset_succeeded":produced.subset_succeeded,"subset_error":produced.subset_error});
            produced.witness
        }
        "std" | "slimgb" | "msolve-lift" => {
            let program = if kind == "msolve-lift" {
                let input = out.join("input.ms");
                save(
                    &input,
                    external_certificate::msolve_input(&system)?.as_bytes(),
                )?;
                let basis_path = out.join("msolve-basis.txt");
                process(
                    &args[4],
                    &[
                        "-g".into(),
                        "2".into(),
                        "-t".into(),
                        "1".into(),
                        "-f".into(),
                        input.display().to_string(),
                        "-o".into(),
                        basis_path.display().to_string(),
                    ],
                    out,
                    "msolve",
                )?;
                let basis = cas_formats::msolve_basis(
                    std::str::from_utf8(&read(&basis_path, 16 * 1024 * 1024)?)?,
                    &system.variables,
                )?;
                external_certificate::program(
                    &system,
                    "slimgb",
                    Some(&basis.into_iter().collect::<Vec<_>>()),
                )?
            } else {
                external_certificate::program(&system, kind, None)?
            };
            let script = out.join("producer.sing");
            save(&script, program.as_bytes())?;
            let bytes = process(
                &args[3],
                &["-q".into(), script.display().to_string()],
                out,
                "producer",
            )?;
            let members = external_certificate::parse(
                std::str::from_utf8(&bytes)?,
                system.variables.len(),
                system.prime,
                system.polynomials.len(),
            )?;
            replay::complete_product(&system, Vec::new(), members)?
        }
        _ => return Err("unknown fused producer".into()),
    };
    save(&out.join("witness.json"), &serde_json::to_vec(&witness)?)?;
    let script = out.join("check.sing");
    save(
        &script,
        replay::singular_script(&system, &witness)?.as_bytes(),
    )?;
    let checked = process(
        &args[3],
        &["-q".into(), script.display().to_string()],
        out,
        "check",
    )?;
    if std::str::from_utf8(&checked)?.trim() != "CERTIFICATE_REPLAY_OK" {
        return Err("fused replay marker missing/polluted".into());
    }
    let basis: Vec<Vec<_>> = witness
        .basis
        .iter()
        .map(|row| {
            row.polynomial
                .iter()
                .map(|t| (t.exponents.clone(), t.coefficient))
                .collect()
        })
        .collect();
    let terms: usize = witness
        .definitions
        .iter()
        .chain(&witness.basis)
        .map(|r| r.proof.len())
        .sum::<usize>()
        + witness.input_reductions.iter().map(Vec::len).sum::<usize>()
        + witness
            .pair_reductions
            .iter()
            .map(|r| r.proof.len())
            .sum::<usize>();
    let answer = json!({"basis":basis,"constructor":constructor,"explicit_cofactor_terms":terms,"coprime_pairs":witness.pair_reductions.iter().filter(|r|r.product_criterion).count(),"total_pairs":witness.pair_reductions.len(),"validation":"complete exact replay, checked product criterion; unsigned"});
    save(&out.join("answer.json"), &serde_json::to_vec(&answer)?)?;
    println!("{}", serde_json::to_string(&answer)?);
    Ok(())
}
