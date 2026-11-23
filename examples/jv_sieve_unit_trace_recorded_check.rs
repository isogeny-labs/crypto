//! Cold-cache correctness replay on all captured systems; no new curve/yield.
pub use crypto_lib::cryptanalysis;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_nagao_unit_fast_elimination.rs"]
mod fast;
#[allow(dead_code)]
#[path = "../src/cryptanalysis/jv_nagao_unit_elimination.rs"]
mod nagao_unit_elimination;
use crypto_lib::cryptanalysis::{
    f4_fp::{self, Poly},
    jv_bitops as work, jv_cover as cover,
};
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{BufWriter, Write},
};
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 3 {
        return Err("usage: TRACE_REPLAY ORIGINAL_UNIT_RAW NEW_ATTEMPTS NEW_SUMMARY".into());
    }
    let bytes = fs::read(&a[0])?;
    let sha = hash(&bytes);
    if sha != "6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d" {
        return Err("original recorded input".into());
    }
    let w: Value = serde_json::from_slice(&bytes)?;
    let p = w["public"]["p"].as_u64().ok_or("p")?;
    let l = w["public"]["l"].as_u64().ok_or("l")?;
    let records = w["work"]["nagao_attempts"].as_array().ok_or("attempts")?;
    if records.len() != 3415 {
        return Err("complete captured cohort".into());
    }
    let mut out = BufWriter::new(
        fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&a[1])?,
    );
    cover::clear_traces();
    cover::set_trace_scope(5);
    work::claim();
    work::set_moduli(p, l);
    work::enable(true);
    let _phase = work::enter(work::Phase::Descent);
    let opts = cover::opts_for_mode(24, 600.0, cover::StopMode::Staircase(64));
    let (mut roots_checked, mut replayed, mut fallback, mut failures) = (0, 0, 0, 0);
    for (i, t) in records.iter().enumerate() {
        if t["residual_index"] != json!(i) {
            return Err("ordered complete prefix".into());
        }
        let input: Vec<Poly> = serde_json::from_value(t["audit"]["original"].clone())?;
        let mut expected: Vec<Vec<u64>> =
            serde_json::from_value(t["audit"]["lifted_roots"].clone())?;
        expected.sort();
        let mut rng =
            StdRng::seed_from_u64(389u64 ^ (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let record = match fast::solve_audited_traced(&input, p, &opts, &mut rng) {
            Err(e) => {
                failures += 1;
                json!({"residual_index":i,"error":e,"expected":expected,"passed":false})
            }
            Ok((mut roots, stats, audit)) => {
                roots.sort();
                roots_checked += roots.len();
                replayed += usize::from(stats.replayed);
                fallback += usize::from(stats.trace_mismatch);
                let passed = roots == expected
                    && !stats.incomplete
                    && !stats.timed_out
                    && stats.nonseparating == 0
                    && audit.reduced_sha256
                        == t["audit"]["reduced_sha256"]
                            .as_str()
                            .ok_or("reduced digest")?
                    && roots
                        .iter()
                        .all(|x| x.len() == 6 && input.iter().all(|f| f4_fp::eval(f, x, p) == 0));
                failures += usize::from(!passed);
                json!({"residual_index":i,"roots":roots,"expected":expected,"stats":stats,"reduced_sha256":audit.reduced_sha256,"passed":passed})
            }
        };
        serde_json::to_writer(&mut out, &record)?;
        out.write_all(b"\n")?;
    }
    out.flush()?;
    cover::set_trace_scope(0);
    let result = json!({"schema":"cover.unit-trace.recorded-correctness/v1","input_sha256":sha,"systems":records.len(),"roots":roots_checked,
        "replayed":replayed,"shape_fallbacks":fallback,"failures":failures,"cold_trace_scope":5,"cache_cleared_before_first":true,
        "every_captured_coordinate_set_checked":true,"every_lifted_original_equation_checked":true,"attempts_sha256":hash(&fs::read(&a[1])?),
        "new_natural_yield_calls":0,"elapsed_claim":false,"complete_cost":false,"rho_parity":false});
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&a[2])?;
    serde_json::to_writer_pretty(&mut file, &result)?;
    file.write_all(b"\n")?;
    println!("Recorded trace replay: {} systems, {roots_checked} roots, {replayed} replays, {fallback} shape fallbacks, {failures} failures",records.len());
    if failures != 0 {
        return Err("traced coordinate mismatch; every attempt retained".into());
    }
    Ok(())
}
