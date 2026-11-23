//! Finite polynomial controls and retained-data replay only; no DLP worker.
pub use crypto_lib::cryptanalysis;
#[path = "../research/extension_ic_graph_20261115/quartic_discriminant_check.rs"]
pub mod independent;
#[path = "../src/cryptanalysis/jv_quartic_discriminant.rs"]
pub mod screen;
use crypto_lib::cryptanalysis::jv_sieve::FpRing;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Write, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn main() -> Result<()> {
    let a: Vec<_> = std::env::args().skip(1).collect();
    if a.len() != 3 || a[0] != "--replay" {
        return Err("usage: CONTROLS --replay ORIGINAL48_RAW NEW_OUTPUT".into());
    }
    let bytes = fs::read(Path::new(&a[1]))?;
    let w: Value = serde_json::from_slice(&bytes)?;
    if w["source_revision"] != "f58a7eb96be9fe47c40f5ef5dcaabe74e2392854"
        || w["method"] != "quadratic-pairs"
    {
        return Err("retained48identity".into());
    }
    let p = w["public"]["p"].as_u64().ok_or("prime")?;
    let ring = FpRing::new(p);
    let ctx = screen::Context::new(&ring)?;
    let splits = w["work"]["collection"]["splits"]
        .as_array()
        .ok_or("histories")?;
    if splits.len() != 262144 {
        return Err("complete retained48quartics".into());
    }
    let (mut skipped, mut kept, mut accepted, mut attempts, mut old_zero_attempt_skips) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut histogram = BTreeMap::<i8, u64>::new();
    for split in splits {
        let tail: Vec<u64> = serde_json::from_value(split["tail"].clone())?;
        let actual = ctx.screen(&tail)?;
        let character = independent::check(p, &tail, &serde_json::to_value(actual)?)?;
        *histogram.entry(character).or_default() += 1;
        let report = &split["report"];
        let count = report["attempts"].as_array().ok_or("attempts")?.len() as u64;
        if actual.rejects() {
            skipped += 1;
            if count != 0 || !report["factors"].is_null() {
                return Err("screen skipped attempted/admitted old split".into());
            }
            old_zero_attempt_skips += 1;
        } else {
            kept += 1;
            attempts += count;
        }
        if report["factors"].is_array() {
            accepted += 1;
            if character != 1 {
                return Err("false negative on admitted pair".into());
            }
        }
    }
    let output = json!({"schema":"cover.quartic-discriminant.recorded-check/v1","original_sha256":format!("{:x}",Sha256::digest(&bytes)),"original_source_revision":w["source_revision"],"retained_quartics":splits.len(),"character_histogram":histogram,"would_skip":skipped,"would_keep_old_splitter":kept,"all_skips_originally_used_zero_draws":old_zero_attempt_skips==skipped,"accepted_factorizations_checked":accepted,"retained_random_polynomial_attempts":attempts,"every_resultant_checked_by_independent_sylvester":true,"scientific_worker_invoked":false,"natural_yield_or_performance_measurement":false,"complete_cost":false,"total_bitops":null,"ratios":null});
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[2])?;
    serde_json::to_writer_pretty(&mut f, &output)?;
    f.write_all(b"\n")?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    fn mul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
        let mut out = vec![0; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                out[i + j] = ((out[i + j] as u128 + x as u128 * y as u128) % p as u128) as u64;
            }
        }
        out
    }
    fn eval(f: &[u64], x: u64, p: u64) -> u64 {
        f.iter().rev().fold(0, |v, &c| {
            ((v as u128 * x as u128 + c as u128) % p as u128) as u64
        })
    }
    #[test]
    fn exhaustive_nonmonic_quartics_match_sylvester_and_distinct_pair_condition() {
        let mut cases = 0;
        for p in [5u64, 7] {
            let ring = FpRing::new(p);
            let ctx = screen::Context::new(&ring).unwrap();
            let quads: Vec<_> = (0..p)
                .flat_map(|a| (0..p).map(move |b| vec![a, b, 1]))
                .filter(|q| !(0..p).any(|x| eval(q, x, p) == 0))
                .collect();
            let mut pairs = BTreeSet::new();
            for (i, q) in quads.iter().enumerate() {
                for r in &quads[i + 1..] {
                    pairs.insert(mul(q, r, p));
                }
            }
            for index in 0..p.pow(4) {
                let mut x = index;
                let mut monic = vec![0; 5];
                for c in &mut monic[..4] {
                    *c = x % p;
                    x /= p;
                }
                monic[4] = 1;
                let has_pair = pairs.contains(&monic);
                for leading in 1..p {
                    let tail: Vec<_> = monic.iter().map(|&c| c * leading % p).collect();
                    let result = ctx.screen(&tail).unwrap();
                    independent::check(p, &tail, &serde_json::to_value(result).unwrap()).unwrap();
                    if has_pair {
                        assert!(!result.rejects());
                    }
                    if !(0..p).any(|x| eval(&tail, x, p) == 0) {
                        assert_eq!(result.character == 1, has_pair);
                    }
                    cases += 1;
                }
            }
        }
        assert_eq!(cases, 16906);
    }
    #[test]
    fn malformed_field_or_tail_never_receives_screen_admission() {
        for p in [2, 3, 4, 9, 15, 1001, 1513] {
            let ring = FpRing::new(p);
            assert!(screen::Context::new(&ring).is_err());
        }
        let ring = FpRing::new(101);
        let ctx = screen::Context::new(&ring).unwrap();
        for tail in [
            vec![],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4, 0],
            vec![101, 2, 3, 4, 1],
        ] {
            assert!(ctx.screen(&tail).is_err());
        }
    }
    #[test]
    fn independent_checker_rejects_each_corrupted_resultant_character_field() {
        let ring = FpRing::new(101);
        let ctx = screen::Context::new(&ring).unwrap();
        let tail = mul(&[98, 0, 1], &[99, 0, 1], 101);
        let result = ctx.screen(&tail).unwrap();
        assert_eq!(result.character, 1);
        let raw = serde_json::to_value(result).unwrap();
        for key in ["resultant", "discriminant_class", "character"] {
            let mut bad = raw.clone();
            bad[key] = json!((bad[key].as_i64().unwrap() + 1) % 101);
            assert!(independent::check(101, &tail, &bad).is_err());
        }
        let mut bad = raw;
        bad["unchecked_extra"] = json!(true);
        assert!(independent::check(101, &tail, &bad).is_err());
    }
    #[test]
    fn context_setup_and_screen_arithmetic_stay_in_owned_field_phase() {
        use crypto_lib::cryptanalysis::jv_bitops::{self as work, Phase};
        work::claim();
        work::set_moduli(101, 103);
        work::enable(true);
        let phase = work::enter(Phase::Relations);
        let before = work::snapshot();
        let ring = FpRing::new(101);
        let ctx = screen::Context::new(&ring).unwrap();
        let result = ctx.screen(&[6, 0, 96, 0, 1]).unwrap();
        assert_eq!(result.character, 1);
        let delta = work::snapshot().since(&before);
        let p = work::prims_json(&delta.sum(&Phase::IC));
        assert!(p["field_mul"].as_u64().unwrap() > 0);
        assert!(p["field_inv"].as_u64().unwrap() > 0);
        assert_eq!(work::foreign(), 0);
        work::enable(false);
        drop(phase);
    }
}
