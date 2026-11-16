//! Diagnostic wrapper only. Original frozen workers remain the timing controls.
use certified_relations::{replay, System};
use crypto_lib::cryptanalysis::f4_fp::certified::verify_replay_certificate_json;
use serde_json::json;
use std::{error::Error, fs, path::Path, time::Instant};
type Result<T> = std::result::Result<T, Box<dyn Error>>;
fn ns(t: Instant) -> u64 {
    u64::try_from(t.elapsed().as_nanos()).expect("bounded duration")
}
fn read(path: &str, cap: u64) -> Result<Vec<u8>> {
    if fs::metadata(path)?.len() > cap {
        return Err("input/file budget".into());
    }
    let bytes = fs::read(path)?;
    if bytes.len() as u64 > cap {
        return Err("input/file budget".into());
    }
    Ok(bytes)
}
pub fn run(frontier: bool) -> Result<()> {
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
        return Err(
            "usage: native-timed INPUT KIND SINGULAR_UNUSED MSOLVE_UNUSED NEW_OUTPUT".into(),
        );
    }
    let start = Instant::now();
    let original = read(&a[1], 4 * 1024 * 1024)?;
    let system: System = serde_json::from_slice(&original)?;
    let input_parse_ns = ns(start);
    let out = Path::new(&a[5]);
    fs::create_dir(out)?;
    let kernel = if frontier {
        replay::NativeKernels::BothUpdatesFrontier
    } else {
        replay::NativeKernels::BothUpdatesFused
    };
    let start = Instant::now();
    let (wire, constructor) =
        replay::produce_native_for_rust_with_kernels(&system, &a[2], kernel)?.into_parts();
    let construction_and_wire_ns = ns(start);
    let start = Instant::now();
    fs::write(out.join("witness.json"), &wire)?;
    let wire_write_ns = ns(start);
    let start = Instant::now();
    let checked = verify_replay_certificate_json(&original, &wire)?;
    let root_check_ns = ns(start);
    let answer = json!({"basis":checked.basis,"constructor":constructor,"consumer":{"kind":"separate-rust-arithmetic","stats":checked.stats},"native_product_criterion":true,"root_api":true,"completion_exports":0,"kernel_mode":kernel,"diagnostic_phase_ns":{"input_parse":input_parse_ns,"construction_and_wire":construction_and_wire_ns,"wire_write":wire_write_ns,"root_check":root_check_ns},"validation":"same root replay; diagnostic clocks only, unsigned"});
    let bytes = serde_json::to_vec(&answer)?;
    fs::write(out.join("answer.json"), &bytes)?;
    println!("{}", std::str::from_utf8(&bytes)?);
    Ok(())
}
// Cargo also discovers this helper as a standalone diagnostic binary.
fn main() -> Result<()> {
    run(true)
}
