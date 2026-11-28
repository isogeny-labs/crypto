//! Exact descriptive aggregation of saved coordinate records. No solver calls.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const ORIGINAL_SHA: &str = "6e1db64631838ebe3bdc918a88b160ced5c8362b70aea8814d16aca83368286d";
const FIVE_SHA: &str = "43f4d8bbc9c524063b37c44e2df85bd521b8223704e0f53b645e49cb0caf2328";
const SIX_SHA: &str = "f9bd2377cfb7dccf073d6bd9932bb7149b095efa0bf7f7027ab80d6e50d5f54d";
const FIVE_PROOF: &str = include_str!("validation/unit-trace-recorded-proof.json");
const SIX_PROOF: &str = include_str!("validation/six-trace-recorded-proof.json");

fn bind_proof(text: &str, attempts_sha: &str, arity: u64) -> Result<String> {
    let proof: Value = serde_json::from_str(text)?;
    ensure(
        proof["input_sha256"] == ORIGINAL_SHA
            && proof["attempts_sha256"] == attempts_sha
            && proof["systems"] == 3415
            && proof["roots"] == 3419
            && proof["failures"] == 0
            && proof["cold_trace_scope"] == arity
            && proof["cache_cleared_before_first"] == true
            && proof["new_natural_yield_calls"] == 0,
        "embedded replay proof binding",
    )?;
    Ok(format!("{:x}", Sha256::digest(text.as_bytes())))
}

fn ensure(ok: bool, why: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(why.into())
    }
}
fn file_hash(path: &Path) -> Result<String> {
    let mut r = fs::File::open(path)?;
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = r.read(&mut b)?;
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    Ok(format!("{:x}", h.finalize()))
}
fn roots(v: &Value) -> Result<Vec<Vec<u64>>> {
    let mut roots: Vec<Vec<u64>> = serde_json::from_value(v.clone())?;
    ensure(
        roots
            .iter()
            .all(|x| x.len() == 6 && x.iter().all(|a| *a < 1009)),
        "canonical root shape",
    )?;
    roots.sort();
    let unique: BTreeSet<_> = roots.iter().collect();
    ensure(unique.len() == roots.len(), "duplicate roots")?;
    Ok(roots)
}
#[derive(Default, Clone, Debug, PartialEq, Eq)]
struct Count {
    systems: u64,
    roots: u64,
    f4: u128,
    linear: u128,
}
impl Count {
    fn add(&mut self, stats: &Value, n: usize) -> Result<()> {
        for key in ["timed_out", "incomplete", "replayed", "trace_mismatch"] {
            ensure(stats[key].is_boolean(), "explicit status flags")?;
        }
        ensure(
            stats["timed_out"] == false
                && stats["incomplete"] == false
                && stats["nonseparating"].as_u64() == Some(0),
            "complete captured solve",
        )?;
        let f = stats["f4_muls"].as_u64().ok_or("integer F4 counter")? as u128;
        let l = stats["lin_muls"].as_u64().ok_or("integer linear counter")? as u128;
        let f4 = self.f4.checked_add(f).ok_or("F4 total overflow")?;
        let linear = self.linear.checked_add(l).ok_or("linear total overflow")?;
        ensure(f4.checked_add(linear).is_some(), "combined total overflow")?;
        self.f4 = f4;
        self.linear = linear;
        self.systems = self.systems.checked_add(1).ok_or("system count overflow")?;
        self.roots = self
            .roots
            .checked_add(n.try_into()?)
            .ok_or("root count overflow")?;
        Ok(())
    }
    fn json(&self) -> Value {
        json!({"systems":self.systems,"roots":self.roots,"f4_muls":self.f4.to_string(),"lin_muls":self.linear.to_string(),"reported_sum":(self.f4+self.linear).to_string()})
    }
}
#[derive(Default)]
struct Arm {
    total: Count,
    first32: Count,
    replay: Count,
    mismatch: Count,
    recording: Count,
}
impl Arm {
    fn push(&mut self, index: usize, stats: &Value, n: usize, traced: bool) -> Result<()> {
        let rp = stats["replayed"] == true;
        let mm = stats["trace_mismatch"] == true;
        ensure(!(rp && mm), "exclusive trace flags")?;
        if !traced {
            ensure(!rp && !mm, "untraced flags")?;
        }
        self.total.add(stats, n)?;
        if index < 32 {
            self.first32.add(stats, n)?;
        }
        if traced {
            if rp {
                self.replay.add(stats, n)?;
            } else if mm {
                self.mismatch.add(stats, n)?;
            } else {
                self.recording.add(stats, n)?;
            }
        }
        Ok(())
    }
    fn json(&self, id: &str, sha: &str) -> Value {
        json!({"arm":id,"input_sha256":sha,"total":self.total.json(),"first32":self.first32.json(),"replayed":self.replay.json(),"shape_mismatch":self.mismatch.json(),"recording":self.recording.json()})
    }
}
fn traced(path: &Path, expected: &[Vec<Vec<u64>>], expected_counts: (u64, u64)) -> Result<Arm> {
    let mut arm = Arm::default();
    for (i, line) in BufReader::new(fs::File::open(path)?).lines().enumerate() {
        ensure(i < expected.len(), "extra saved attempt")?;
        let line = line?;
        ensure(!line.is_empty(), "empty saved attempt")?;
        let r: Value = serde_json::from_str(&line)?;
        ensure(
            r["residual_index"].as_u64() == Some(i as u64) && r["passed"] == true,
            "ordered passing attempt",
        )?;
        let actual = roots(&r["roots"])?;
        ensure(
            actual == roots(&r["expected"])? && actual == expected[i],
            "all three coordinate multisets",
        )?;
        arm.push(i, &r["stats"], actual.len(), true)?;
    }
    ensure(
        arm.total.systems == expected.len() as u64,
        "missing saved attempt",
    )?;
    ensure(
        arm.total.roots == 3419
            && arm.replay.systems == expected_counts.0
            && arm.mismatch.systems == expected_counts.1
            && arm.recording.systems == 1,
        "full trace tallies",
    )?;
    Ok(arm)
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    ensure(
        args.len() == 4,
        "usage: COHORT ORIGINAL_UNIT FIVE_JSONL SIX_JSONL NEW_RESULT",
    )?;
    ensure(!Path::new(&args[3]).exists(), "exclusive new output")?;
    let replay_proofs = [
        bind_proof(FIVE_PROOF, FIVE_SHA, 5)?,
        bind_proof(SIX_PROOF, SIX_SHA, 6)?,
    ];
    let mut actual_hashes = Vec::new();
    for (path, expected) in args[..3].iter().zip([ORIGINAL_SHA, FIVE_SHA, SIX_SHA]) {
        let h = file_hash(Path::new(path))?;
        ensure(h == expected, "immutable captured input hash")?;
        actual_hashes.push(h);
    }
    let raw: Value = serde_json::from_reader(fs::File::open(&args[0])?)?;
    ensure(
        raw["public"]["p"] == 1009 && raw["seed"] == 389,
        "original domain and seed",
    )?;
    let attempts = raw["work"]["nagao_attempts"]
        .as_array()
        .ok_or("unit attempts")?;
    ensure(attempts.len() == 3415, "all original attempts")?;
    let mut original = Arm::default();
    let mut expected = Vec::new();
    for (i, a) in attempts.iter().enumerate() {
        ensure(
            a["residual_index"].as_u64() == Some(i as u64),
            "ordered original attempts",
        )?;
        let rr = roots(&a["audit"]["lifted_roots"])?;
        original.push(i, &a["audit"]["stats"], rr.len(), false)?;
        expected.push(rr);
    }
    drop(raw);
    let five = traced(Path::new(&args[1]), &expected, (3348, 66))?;
    let six = traced(Path::new(&args[2]), &expected, (3381, 33))?;
    ensure(original.total.roots == 3419, "complete original roots")?;
    let out = json!({"schema":"cover.unit-trace.saved-cohort-counters/v1","selection":"all3415 saved Run51 systems; no exclusion",
        "arms":[original.json("unit-untraced",&actual_hashes[0]),five.json("unit-traced",&actual_hashes[1]),six.json("six-traced",&actual_hashes[2])],
        "every_coordinate_multiset_equal":true,"systems":3415,"roots":3419,"new_solver_calls":0,"new_natural_yield_calls":0,
        "new_timing_calls":0,"complete_cost":false,"ratios":null,"rho_parity":false,"independent_signed_verdict":false,
        "units":"solver-reported Fp multiplication counters; decimal strings accumulated in checked u128",
        "replay_summary_sha256":replay_proofs,
        "missing_costs":["tracked RREF of trace learning outside field_ops","reduction and lifting","collection and graph elimination","other native control, setup, RNG, hashing and serialization"],
        "no_cross_study_repricing":true,"recorded_mathematics_only":true});
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[3])?;
    serde_json::to_writer_pretty(&mut file, &out)?;
    file.write_all(b"\n")?;
    println!("All3415 saved systems and3419roots bound; native partial counter totals written.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn stats() -> Value {
        json!({"timed_out":false,"incomplete":false,"replayed":false,"trace_mismatch":false,"nonseparating":0,"f4_muls":17,"lin_muls":23})
    }
    #[test]
    fn missing_or_fractional_or_failed_records_are_refused() {
        for bad in [json!(null), json!(0.5), json!("17")] {
            let mut s = stats();
            s["f4_muls"] = bad;
            assert!(Count::default().add(&s, 0).is_err());
        }
        let mut s = stats();
        s.as_object_mut().unwrap().remove("incomplete");
        assert!(Count::default().add(&s, 0).is_err());
        for key in ["timed_out", "incomplete"] {
            let mut s = stats();
            s[key] = json!(true);
            assert!(Count::default().add(&s, 0).is_err());
        }
    }
    #[test]
    fn overflow_is_refused_and_large_integers_stay_exact() {
        let mut c = Count {
            f4: u128::MAX - 1,
            ..Default::default()
        };
        assert!(c.add(&stats(), 0).is_err());
        let mut combined = Count {
            f4: u128::MAX - 25,
            ..Default::default()
        };
        assert!(combined.add(&stats(), 0).is_err());
        let mut s = stats();
        s["f4_muls"] = json!(u64::MAX);
        let mut c = Count::default();
        c.add(&s, 0).unwrap();
        c.add(&s, 0).unwrap();
        assert_eq!(c.f4, 2 * u64::MAX as u128);
        assert_eq!(c.json()["f4_muls"], (2 * u64::MAX as u128).to_string());
    }
    #[test]
    fn multiset_and_trace_contracts_reject_duplication_or_crossed_flags() {
        assert!(roots(&json!([[0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]])).is_err());
        let mut s = stats();
        s["replayed"] = json!(true);
        s["trace_mismatch"] = json!(true);
        assert!(Arm::default().push(0, &s, 0, true).is_err());
        s["trace_mismatch"] = json!(false);
        assert!(Arm::default().push(0, &s, 0, false).is_err());
    }
}
