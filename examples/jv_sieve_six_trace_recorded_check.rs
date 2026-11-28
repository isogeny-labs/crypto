//! Same captured coordinate cohort as the unit control; no new relation search.
use crypto_lib::cryptanalysis::{f4_fp, jv_cover as cover};
use rand::{rngs::StdRng, SeedableRng};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{BufWriter, Write},
};

fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 3 {
        return Err("usage: SIX_TRACE_REPLAY ORIGINAL_UNIT_RAW NEW_ATTEMPTS NEW_SUMMARY".into());
    }
    let bytes = fs::read(&args[0])?;
    let input_sha = hash(&bytes);
    if input_sha != "6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d" {
        return Err("frozen recorded input".into());
    }
    let raw: Value = serde_json::from_slice(&bytes)?;
    let p = raw["public"]["p"].as_u64().ok_or("p")?;
    let records = raw["work"]["nagao_attempts"].as_array().ok_or("attempts")?;
    if records.len() != 3415 {
        return Err("complete captured cohort".into());
    }
    let mut out = BufWriter::new(
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args[1])?,
    );
    cover::clear_traces();
    cover::set_trace_scope(6);
    let opts = cover::opts_for_mode(24, 600.0, cover::StopMode::Staircase(64));
    let (mut roots_checked, mut replayed, mut fallbacks, mut failures) = (0, 0, 0, 0);
    for (i, t) in records.iter().enumerate() {
        if t["residual_index"] != json!(i) {
            return Err("ordered complete prefix".into());
        }
        let input: Vec<f4_fp::Poly> = serde_json::from_value(t["audit"]["original"].clone())?;
        let mut expected: Vec<Vec<u64>> =
            serde_json::from_value(t["audit"]["lifted_roots"].clone())?;
        let mut rng =
            StdRng::seed_from_u64(389u64 ^ (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let (mut roots, stats) = cover::solve_zero_dim_traced(&input, 6, p, &opts, &mut rng, true);
        roots.sort();
        expected.sort();
        let passed = roots == expected
            && !stats.incomplete
            && !stats.timed_out
            && stats.nonseparating == 0
            && roots
                .iter()
                .all(|x| x.len() == 6 && input.iter().all(|f| f4_fp::eval(f, x, p) == 0));
        roots_checked += roots.len();
        replayed += usize::from(stats.replayed);
        fallbacks += usize::from(stats.trace_mismatch);
        failures += usize::from(!passed);
        serde_json::to_writer(
            &mut out,
            &json!({"residual_index":i,"roots":roots,"expected":expected,"stats":stats,"passed":passed}),
        )?;
        out.write_all(b"\n")?;
    }
    out.flush()?;
    cover::set_trace_scope(0);
    let result = json!({"schema":"cover.six-trace.recorded-correctness/v1","input_sha256":input_sha,
        "systems":records.len(),"roots":roots_checked,"replayed":replayed,"shape_fallbacks":fallbacks,"failures":failures,
        "cache_cleared_before_first":true,"cold_trace_scope":6,"every_captured_coordinate_set_checked":true,
        "every_original_equation_checked":true,"attempts_sha256":hash(&fs::read(&args[1])?),
        "new_natural_yield_calls":0,"elapsed_claim":false,"complete_cost":false,"rho_parity":false});
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    serde_json::to_writer_pretty(&mut file, &result)?;
    file.write_all(b"\n")?;
    println!("Six-coordinate recorded trace replay: {} systems, {roots_checked} roots, {replayed} replays, {fallbacks} shape fallbacks, {failures} failures",records.len());
    if failures != 0 {
        return Err("six-coordinate trace mismatch; all attempts retained".into());
    }
    Ok(())
}
