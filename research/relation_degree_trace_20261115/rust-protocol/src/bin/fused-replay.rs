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
    time::Instant,
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
    run(false)
}
fn elapsed(start: Option<Instant>) -> u64 {
    start.map_or(0, |s| s.elapsed().as_nanos().min(u64::MAX as u128) as u64)
}
pub(crate) fn run(profile: bool) -> Result<()> {
    run_with(profile, false, false)
}
pub(crate) fn run_with(profile: bool, rust_consumer: bool, native_product: bool) -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 6 {
        return Err("usage: fused-replay INPUT compact|expanded|compact-sparse|std|slimgb|msolve-lift SINGULAR MSOLVE NEW_OUTPUT".into());
    }
    let system_bytes = read(Path::new(&args[1]), 4 * 1024 * 1024)?;
    let system: System = serde_json::from_slice(&system_bytes)?;
    let out = Path::new(&args[5]);
    fs::create_dir(out)?;
    let kind = args[2].as_str();
    let mut constructor = serde_json::Value::Null;
    let produce_start = profile.then(Instant::now);
    let witness = if native_product && ["compact", "expanded", "compact-sparse"].contains(&kind) {
        let produced = replay::produce_native_product(&system, kind)?;
        constructor = produced.constructor;
        produced.witness
    } else {
        match kind {
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
        }
    };
    let produce_ns = elapsed(produce_start);
    let wire_start = profile.then(Instant::now);
    let wire_bytes = serde_json::to_vec(&witness)?;
    save(&out.join("witness.json"), &wire_bytes)?;
    let wire_ns = elapsed(wire_start);
    let script_start = profile.then(Instant::now);
    let script = out.join("check.sing");
    if !rust_consumer {
        save(
            &script,
            replay::singular_script(&system, &witness)?.as_bytes(),
        )?;
    }
    let script_ns = elapsed(script_start);
    let check_start = profile.then(Instant::now);
    let mut consumer_stats = serde_json::Value::Null;
    let checked_basis = if rust_consumer {
        let checked = relation_replay_checker::verify_json(&system_bytes, &wire_bytes)?;
        consumer_stats = serde_json::to_value(&checked.stats)?;
        Some(checked.basis)
    } else {
        let checked = process(
            &args[3],
            &["-q".into(), script.display().to_string()],
            out,
            "check",
        )?;
        if std::str::from_utf8(&checked)?.trim() != "CERTIFICATE_REPLAY_OK" {
            return Err("fused replay marker missing/polluted".into());
        }
        None
    };
    let check_ns = elapsed(check_start);
    let basis: Vec<Vec<_>> = checked_basis.unwrap_or_else(|| {
        witness
            .basis
            .iter()
            .map(|row| {
                row.polynomial
                    .iter()
                    .map(|t| (t.exponents.clone(), t.coefficient))
                    .collect()
            })
            .collect()
    });
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
    let mut answer = json!({"basis":basis,"constructor":constructor,"explicit_cofactor_terms":terms,"coprime_pairs":witness.pair_reductions.iter().filter(|r|r.product_criterion).count(),"total_pairs":witness.pair_reductions.len(),"validation":"complete exact replay, checked product criterion; unsigned"});
    if profile {
        answer["profile_ns"] = json!({"construction_and_completion":produce_ns,"wire_write":wire_ns,"script_export_write":script_ns,"checker_process_consumption":check_ns});
    }
    if rust_consumer || native_product {
        answer["consumer"] = json!({"kind":if rust_consumer{"separate-rust-arithmetic"}else{"singular-identity"},"stats":consumer_stats});
        answer["native_product_criterion"] = json!(native_product);
    }
    save(&out.join("answer.json"), &serde_json::to_vec(&answer)?)?;
    println!("{}", serde_json::to_string(&answer)?);
    Ok(())
}
