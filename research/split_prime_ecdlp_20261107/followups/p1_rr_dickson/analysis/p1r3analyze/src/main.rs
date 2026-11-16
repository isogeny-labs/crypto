//! p1r3analyze: P1 round 3 (PROTOCOL-R3.md). One cell (T24 m5 t4, seed 2), systems rr and t.
//! Reads msolve parse records (p1tool parse JSON, one per line or one per file) from round 2b and
//! round 3, and applies the frozen reading: a completed job gives exact D and widest; a capped or
//! timed-out job gives lower bounds, and bounds from several attempts combine by max (PROTOCOL-R3,
//! "Bounds come from all of R2b and R3"). Then rule 3 of PROTOCOL-R2, word for word.
//! Usage: p1r3analyze OUT_DIR FILE...   (each FILE: parse.json or parsed.jsonl; a record's system is
//! the file-name prefix before "_s" of its "file" field)
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
#[derive(Default, Debug)]
struct B { d: i64, w: i64, exact: Option<(i64, i64, i64)>, attempts: Vec<Value> }
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let out = &a[1];
    fs::create_dir_all(out).unwrap();
    let mut m: BTreeMap<String, B> = BTreeMap::new();
    for f in &a[2..] {
        for line in fs::read_to_string(f).unwrap().lines().filter(|l| !l.trim().is_empty()) {
            let v: Value = serde_json::from_str(line).unwrap();
            let base = v["file"].as_str().unwrap().rsplit('/').next().unwrap().to_string();
            let sys = base.split_once("_s").unwrap().0.to_string();
            let d = v["D"].as_i64().unwrap();
            let w = v["widest_cols"].as_i64().unwrap();
            let st = v["status"].as_str().unwrap().to_string();
            let e = m.entry(sys).or_default();
            if st == "ok" {
                assert!(e.exact.is_none(), "two completed jobs for one system");
                e.exact = Some((d, w, v["quotient_dim"].as_i64().unwrap_or(-1)));
            }
            e.d = e.d.max(d);
            e.w = e.w.max(w);
            e.attempts.push(json!({"source": f, "status": st, "D": d, "widest": w, "exit": v["exit"], "wall_s": v["wall_s"]}));
        }
    }
    let rr = m.get("rr").expect("rr records");
    let t = m.get("t").expect("t records");
    // frozen reading: "RR below X" decided only when RR completed and RR < X's value or lower bound.
    let (below_d, below_w) = match rr.exact {
        Some((d, w, _)) => (Some(d < t.d), Some(w < t.w)),
        None => (None, None),
    };
    let verdict = match (below_d, below_w) {
        (Some(true), Some(true)) => "RR advantage holds at m = 5 with a tower",
        (Some(true), Some(false)) => "trade",
        (Some(false), _) => "fails at m = 5",
        _ => "undecided",
    };
    let row = |s: &str, b: &B| json!({"system": s, "completed": b.exact.is_some(),
        "D": b.exact.map(|x| x.0).unwrap_or(b.d), "D_is_lower_bound": b.exact.is_none(),
        "widest": b.exact.map(|x| x.1).unwrap_or(b.w), "widest_is_lower_bound": b.exact.is_none(),
        "quotient_dim": b.exact.map(|x| x.2), "attempts": b.attempts});
    let dec = json!({"cell": "D24_m5_t4", "seed": 2, "rule": "PROTOCOL-R2 rule 3 with the bound reading; bounds are the max over R2b and R3 attempts (PROTOCOL-R3)",
        "rr": row("rr", rr), "t": row("t", t), "rr_below_t_on_D": below_d, "rr_below_t_on_widest": below_w, "verdict": verdict});
    fs::write(format!("{out}/decision_r3.json"), serde_json::to_string_pretty(&dec).unwrap() + "\n").unwrap();
    let mut tsv = String::from("system\tcompleted\tD\twidest\tn_attempts\n");
    for (s, b) in [("rr", rr), ("t", t)] {
        tsv += &format!("{s}\t{}\t{}\t{}\t{}\n", b.exact.is_some(), b.exact.map(|x| x.0).unwrap_or(b.d), b.exact.map(|x| x.1).unwrap_or(b.w), b.attempts.len());
    }
    fs::write(format!("{out}/p1_r3_bounds.tsv"), tsv).unwrap();
    println!("{verdict}");
}
