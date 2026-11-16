//! pcc `recompute` proof with numeric headlines only (written after the run,
//! because pcc requires numeric claimed/recomputed values). It reuses the
//! frozen scorer (`src/score.rs`, unchanged) and encodes each registered
//! verdict as a number: PASS = 1, FAIL = 0; interpretation outcome
//! inconclusive = 0, A = 1, B = 2, C = 3.
//!
//! usage: validate <run_dir> <claimed.json> <file=sha256>...
#![allow(dead_code)]
#[path = "../../../c1_twisted_line_ladder_20261027/src/json.rs"]
mod json;
#[path = "../rng.rs"]
mod rng;
#[path = "../score.rs"]
mod score;

pub const ARMS: [&str; 6] = ["std", "tz", "tw", "twh", "hz", "mix"];

fn code(v: &str) -> f64 {
    match v {
        "PASS" | "A" => 1.0,
        "FAIL" | "inconclusive" => 0.0,
        "B" => 2.0,
        "C" => 3.0,
        _ => f64::NAN,
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (dir, claimed_path, pins) = (&a[1], &a[2], &a[3..]);
    let s = score::compute(dir);
    let mut rec = s.headline.clone();
    for (k, v) in &s.verdicts {
        rec.insert(format!("verdict.{k}"), code(v));
    }
    let claimed = json::parse(&std::fs::read_to_string(claimed_path).unwrap());
    let json::J::Obj(entries) = &claimed else { panic!("claimed.json must be an object") };
    let mut ok = true;
    let mut heads = vec![];
    for (name, c) in entries {
        let x = c.get("value").num().expect("numeric claimed value");
        let tol = c.get("rel_tol").num().unwrap_or(0.0);
        let rv = *rec.get(name.as_str()).unwrap_or(&f64::NAN);
        let good = if tol == 0.0 { x == rv } else { ((x - rv) / x).abs() <= tol };
        ok &= good;
        heads.push(format!("    {{\"name\": \"{name}\", \"claimed\": {x}, \"recomputed\": {rv:.6}, \"rel_tol\": {tol}}}"));
    }
    let raw: Vec<String> = pins.iter().map(|p| {
        let (f, h) = p.split_once('=').unwrap();
        format!("    {{\"path\": \"{f}\", \"sha256\": \"{h}\"}}")
    }).collect();
    println!(
        "{{\n  \"schema\": \"validator-report\",\n  \"status\": \"{}\",\n  \"study\": \"research/odd_n_trace_accounting_20261107\",\n  \"recomputed_by\": {{\"tool\": \"research/odd_n_trace_accounting_20261107 (validate binary, frozen src/score.rs)\", \"command\": \"validate <run> claimed.json <file=sha256>...\"}},\n  \"note\": \"Producer-run recompute of README headline numbers and registered verdicts (PASS=1, FAIL=0; interpretation inconclusive=0, A=1, B=2, C=3) from the pinned raw rows; unsigned, so independence is not established.\",\n  \"raw_data\": [\n{}\n  ],\n  \"headline\": [\n{}\n  ]\n}}",
        if ok { "pass" } else { "fail" }, raw.join(",\n"), heads.join(",\n"));
}
