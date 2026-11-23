//! Fixed recorded coordinate solves for native profiling, never a throughput run.
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
use std::{fs, io::Write};
fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 3 || !["six", "unit", "fast"].contains(&a[1].as_str()) {
        return Err("usage: PROFILE_RECORDED ORIGINAL_UNIT_RAW six|unit|fast NEW_OUTPUT".into());
    }
    if std::path::Path::new(&a[2]).exists() {
        return Err("output already exists".into());
    }
    let bytes = fs::read(&a[0])?;
    let input_sha = digest(&bytes);
    if input_sha != "6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d" {
        return Err("frozen recorded input".into());
    }
    let raw: Value = serde_json::from_slice(&bytes)?;
    let p = raw["public"]["p"].as_u64().ok_or("p")?;
    let l = raw["public"]["l"].as_u64().ok_or("l")?;
    let attempts = raw["work"]["nagao_attempts"].as_array().ok_or("attempts")?;
    if attempts.len() != 3415 {
        return Err("recorded cohort".into());
    }
    let opts = cover::opts_for_mode(24, 600.0, cover::StopMode::Staircase(64));
    work::claim();
    work::set_moduli(p, l);
    work::enable(true);
    let _phase = work::enter(work::Phase::Descent);
    let mut fingerprint = Sha256::new();
    let mut total_roots = 0;
    for (i, t) in attempts.iter().take(32).enumerate() {
        let input: Vec<Poly> = serde_json::from_value(t["audit"]["original"].clone())?;
        let mut rng =
            StdRng::seed_from_u64(389u64 ^ (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let (mut roots, stats) = match a[1].as_str() {
            "six" => cover::solve_zero_dim(&input, 6, p, &opts, &mut rng),
            "unit" => {
                let (r, s, _) = nagao_unit_elimination::solve_audited(&input, p, &opts, &mut rng)?;
                (r, s)
            }
            "fast" => {
                let (r, s, _) = fast::solve_audited(&input, p, &opts, &mut rng)?;
                (r, s)
            }
            _ => unreachable!(),
        };
        if stats.incomplete || stats.timed_out || stats.nonseparating != 0 {
            return Err(format!("incomplete profile input{i}").into());
        }
        let mut expected: Vec<Vec<u64>> =
            serde_json::from_value(t["audit"]["lifted_roots"].clone())?;
        roots.sort();
        expected.sort();
        if roots != expected
            || roots
                .iter()
                .any(|r| r.len() != 6 || input.iter().any(|f| f4_fp::eval(f, r, p) != 0))
        {
            return Err(format!("recorded coordinate/root replay at{i}").into());
        }
        total_roots += roots.len();
        fingerprint.update(serde_json::to_vec(&(i, &roots))?);
    }
    let result = json!({"schema":"cover.unit.recorded-coordinate-profile/v1","arm":a[1],"input_sha256":input_sha,
        "systems":32,"selection":"first32 captured Run51 systems, fixed before profiling","roots":total_roots,
        "coordinate_sha256":format!("{:x}",fingerprint.finalize()),"all_expected_roots_and_original_equations_checked":true,
        "new_natural_yield_calls":0,"profile_only":true,"elapsed_claim":false,"complete_cost":false,"rho_parity":false});
    let mut f = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&a[2])?;
    serde_json::to_writer_pretty(&mut f, &result)?;
    f.write_all(b"\n")?;
    Ok(())
}
