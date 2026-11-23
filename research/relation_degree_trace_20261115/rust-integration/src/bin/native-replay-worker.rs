//! Native constructor plus the actual root crate's exact replay API.
//! The fused CLI adapter retains complete proof bytes; no external checker.
use certified_relations::{replay, System};
use crypto_lib::cryptanalysis::f4_fp::certified::verify_replay_certificate_json;
use serde_json::json;
use std::{
    error::Error,
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
fn read(path: &str, cap: u64) -> Result<Vec<u8>, Box<dyn Error>> {
    if fs::metadata(path)?.len() > cap {
        return Err("file byte cap".into());
    }
    Ok(fs::read(path)?)
}
fn save(path: &Path, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    fs::File::options()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(bytes)?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    run(true, true)
}
pub(crate) fn run(native_product: bool, rust_consumer: bool) -> Result<(), Box<dyn Error>> {
    run_with_exports(native_product, rust_consumer, 2, None)
}
#[allow(dead_code)] // Called by the two explicit variant wrapper binaries.
pub(crate) fn run_completion_variant(exports: usize) -> Result<(), Box<dyn Error>> {
    run_with_exports(true, true, exports, None)
}
#[allow(dead_code)]
pub(crate) fn run_kernel_variant(which: usize) -> Result<(), Box<dyn Error>> {
    let kernels = match which {
        0 => replay::NativeKernels::Legacy,
        1 => replay::NativeKernels::Ordered,
        2 => replay::NativeKernels::Cached,
        3 => replay::NativeKernels::Both,
        4 => replay::NativeKernels::BothUpdates,
        5 => replay::NativeKernels::BothUpdatesSum,
        6 => replay::NativeKernels::BothUpdatesLead,
        7 => replay::NativeKernels::BothUpdatesFused,
        8 => replay::NativeKernels::BothUpdatesFrontier,
        _ => return Err("unknown native kernel variant".into()),
    };
    run_with_exports(true, true, 0, Some(kernels))
}
fn run_with_exports(
    native_product: bool,
    rust_consumer: bool,
    exports: usize,
    kernels: Option<replay::NativeKernels>,
) -> Result<(), Box<dyn Error>> {
    if exports > 2 || (exports < 2 && (!native_product || !rust_consumer)) {
        return Err("completion variants require native product and full Rust consumer".into());
    }
    if kernels.is_some() && exports != 0 {
        return Err("kernel variants require pending wire and full Rust replay".into());
    }
    let a: Vec<_> = std::env::args().collect();
    if a.len() == 4 && a[1] == "verify" {
        let checked = verify_replay_certificate_json(
            &read(&a[2], 4 * 1024 * 1024)?,
            &read(&a[3], 16 * 1024 * 1024)?,
        )?;
        println!("{}", serde_json::to_string(&checked)?);
        return Ok(());
    }
    if a.len() != 6 {
        return Err("usage: native-replay-worker INPUT compact|expanded|compact-sparse SINGULAR_UNUSED MSOLVE_UNUSED NEW_OUTPUT; or verify SYSTEM WITNESS".into());
    }
    let original = read(&a[1], 4 * 1024 * 1024)?;
    let system: System = serde_json::from_slice(&original)?;
    let out = Path::new(&a[5]);
    fs::create_dir(out)?;
    let (wire, constructor, witness) = if exports == 0 {
        // Opaque pending evidence, not an accepted basis. Retain bytes and invoke
        // the actual root checker below before any basis reaches the answer.
        let pending = if let Some(kernels) = kernels {
            replay::produce_native_for_rust_with_kernels(&system, &a[2], kernels)?
        } else {
            replay::produce_native_for_rust(&system, &a[2])?
        };
        let (wire, constructor) = pending.into_parts();
        (wire, constructor, None)
    } else {
        let (witness, constructor) = if native_product {
            let produced = if exports == 1 {
                replay::produce_native_single_export(&system, &a[2])?
            } else {
                replay::produce_native_product(&system, &a[2])?
            };
            (produced.witness, produced.constructor)
        } else if a[2] == "compact-sparse" {
            let produced = replay::produce_sparse(&system)?;
            (
                produced.witness,
                json!({"info":produced.constructor,"subset_succeeded":produced.subset_succeeded,"subset_error":produced.subset_error}),
            )
        } else if a[2] == "compact" || a[2] == "expanded" {
            (
                replay::produce_product(&system, a[2] == "compact")?,
                serde_json::Value::Null,
            )
        } else {
            return Err("unknown native constructor".into());
        };
        (serde_json::to_vec(&witness)?, constructor, Some(witness))
    };
    save(&out.join("witness.json"), &wire)?;
    let (basis, stats) = if rust_consumer {
        let checked = verify_replay_certificate_json(&original, &wire)?;
        (checked.basis, serde_json::to_value(checked.stats)?)
    } else {
        let witness = witness
            .as_ref()
            .ok_or("CAS checker requires exported witness")?;
        let script = out.join("check.sing");
        save(
            &script,
            replay::singular_script(&system, witness)?.as_bytes(),
        )?;
        let stdout = out.join("check.stdout");
        let stderr = out.join("check.stderr");
        let status = Command::new(&a[3])
            .args(["-q", &script.display().to_string()])
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
            &out.join("check.process.json"),
            &serde_json::to_vec(
                &json!({"program":a[3],"script":script,"exit_code":status.code(),"success":status.success()}),
            )?,
        )?;
        if !status.success()
            || !fs::read(stderr)?.is_empty()
            || fs::read_to_string(stdout)?.trim() != "CERTIFICATE_REPLAY_OK"
        {
            return Err("native CAS consumer failed/diagnostics/marker".into());
        }
        (
            witness
                .basis
                .iter()
                .map(|r| {
                    r.polynomial
                        .iter()
                        .map(|t| (t.exponents.clone(), t.coefficient))
                        .collect()
                })
                .collect(),
            serde_json::Value::Null,
        )
    };
    let answer = json!({"basis":basis,"constructor":constructor,"consumer":{"kind":if rust_consumer{"separate-rust-arithmetic"}else{"singular-identity"},"stats":stats},"native_product_criterion":native_product,"root_api":rust_consumer,"completion_exports":exports,"kernel_mode":kernels,"validation":"native root adapter, exact consumer selected; unsigned"});
    save(&out.join("answer.json"), &serde_json::to_vec(&answer)?)?;
    println!("{}", serde_json::to_string(&answer)?);
    Ok(())
}
