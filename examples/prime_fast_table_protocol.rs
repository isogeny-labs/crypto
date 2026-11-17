//! Tune/eval protocol: the strongest implemented prime-field IC (`prime_fast::ic_solve`, summands 2 and 3)
//! against the table-lookup collector (`prime_fast::ic_solve_table`), on native Montgomery arithmetic.
//!
//! phase `tune`: curves of 20 and 22 bits (residue 3 mod 4), targets seeds 0..9; per arm the constant
//! c in fb = round(c * p^(1/3)) with the lowest median total_ms (from a wide grid) is written to FROZEN.json.
//! phase `eval`: fresh curves (21, 23 and 24 bits residue 3; 22 and 24 bits residue 1), targets seeds 100..139,
//! frozen c per arm, interleaved seeded order.  Every solve is verified (log equals the planted one).
//!
//! usage: prime_fast_table_protocol <tune|eval> <out_dir>   (env GIT_COMMIT, BIN_SHA256 recorded in rows)
use crypto_lib::cryptanalysis::prime_fast::{
    find_a3_curve, ic_solve, ic_solve_table, FastCurve, Pt,
};
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use serde_json::json;
use std::time::Instant;

const EXTRA: usize = 6; // the repository's bench policy (+6 extra relations), same for all arms
const ARMS: [&str; 3] = ["s2", "s3", "tb"];
const GRID: [f64; 10] = [0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 4.0, 6.0, 8.0];

fn run_arm(
    arm: &str,
    c: &FastCurve,
    g: Pt,
    q: Pt,
    fb: usize,
    seed: u64,
) -> Option<(u64, f64, serde_json::Value)> {
    let t = Instant::now();
    let res = match arm {
        "s2" => ic_solve(c, g, q, fb, EXTRA, 2, u64::MAX, seed),
        "s3" => ic_solve(c, g, q, fb, EXTRA, 3, u64::MAX, seed),
        _ => ic_solve_table(c, g, q, fb, EXTRA, u64::MAX, seed),
    };
    let wall = t.elapsed().as_secs_f64() * 1e3;
    res.map(|(x, r)| {
        (
            x,
            wall,
            json!({"total_ms": r.total_ms, "relations_ms": r.relations_ms, "pair_table_ms": r.pair_table_ms,
                   "la_ms": r.linear_algebra_ms, "verify_ms": r.verify_ms, "targets": r.targets, "relations": r.relations,
                   "rows_two": r.rows_two, "rows_three": r.rows_three, "pair_entries": r.pair_table_entries}),
        )
    })
}

fn fb_for(c: f64, p: u64) -> usize {
    ((c * (p as f64).cbrt()).round() as usize).max(8)
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let phase = args[1].as_str();
    let out = std::path::Path::new(&args[2]);
    std::fs::create_dir_all(out).unwrap();
    let commit = std::env::var("GIT_COMMIT").unwrap_or_default();
    let bin = std::env::var("BIN_SHA256").unwrap_or_default();
    let planted = |c: &FastCurve, g: Pt, seed: u64| -> (u64, Pt) {
        let mut rng = StdRng::seed_from_u64(0xA5A5 ^ seed ^ c.n);
        let k = rng.gen_range(2..c.n);
        (k, c.mul(Some(g), k).expect("planted target"))
    };
    if phase == "tune" {
        let curves: Vec<(u32, u64)> = vec![(20, 3), (22, 3)];
        let mut best: Vec<(String, f64)> = Vec::new();
        let mut rows = Vec::new();
        for arm in ARMS {
            let mut med = Vec::new();
            for &cc in &GRID {
                let mut times = Vec::new();
                for &(bits, res) in &curves {
                    let (fc, g) = find_a3_curve(bits, res).expect("curve");
                    for s in 0..10u64 {
                        let (k, q) = planted(&fc, g, s);
                        let fb = fb_for(cc, fc.f.p);
                        match run_arm(arm, &fc, g, q, fb, 1) {
                            Some((x, wall, rep)) if x == k => {
                                times.push(rep["total_ms"].as_f64().unwrap());
                                rows.push(json!({"arm": arm, "c": cc, "bits": bits, "res": res, "tseed": s, "fb": fb, "ok": true, "wall_ms": wall, "rep": rep}));
                            }
                            _ => rows.push(json!({"arm": arm, "c": cc, "bits": bits, "res": res, "tseed": s, "fb": fb, "ok": false})),
                        }
                    }
                }
                let m = if times.len() == 20 {
                    median(times)
                } else {
                    f64::INFINITY
                };
                med.push((cc, m));
                eprintln!("{arm} c={cc}: median {m:.3} ms");
            }
            let (cb, mb) =
                med.iter()
                    .cloned()
                    .fold((0.0, f64::INFINITY), |a, b| if b.1 < a.1 { b } else { a });
            eprintln!(
                "{arm} -> c = {cb} ({mb:.3} ms)  edge: {}",
                cb == GRID[0] || cb == GRID[GRID.len() - 1]
            );
            best.push((arm.to_string(), cb));
        }
        let frozen: serde_json::Map<String, serde_json::Value> =
            best.iter().map(|(a, c)| (a.clone(), json!(c))).collect();
        std::fs::write(
            out.join("frozen.json"),
            serde_json::to_string(&frozen).unwrap(),
        )
        .unwrap();
        std::fs::write(
            out.join("tune_rows.json"),
            serde_json::to_string(&rows).unwrap(),
        )
        .unwrap();
    } else {
        let frozen: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(out.join("frozen.json")).unwrap())
                .unwrap();
        let curves: Vec<(u32, u64)> = vec![(21, 3), (23, 3), (24, 3), (22, 1), (24, 1)];
        let mut jobs: Vec<(usize, u64, &str)> = Vec::new();
        for ci in 0..curves.len() {
            for s in 100..140u64 {
                for arm in ARMS {
                    jobs.push((ci, s, arm));
                }
            }
        }
        jobs.shuffle(&mut StdRng::seed_from_u64(0xE7A1));
        let built: Vec<(FastCurve, Pt)> = curves
            .iter()
            .map(|&(b, r)| find_a3_curve(b, r).expect("curve"))
            .collect();
        let mut rows = Vec::new();
        for (ci, s, arm) in jobs {
            let (fc, g) = &built[ci];
            let (k, q) = planted(fc, *g, s);
            let cc = frozen[arm].as_f64().unwrap();
            let fb = fb_for(cc, fc.f.p);
            let r = run_arm(arm, fc, *g, q, fb, 1);
            let (ok, wall, rep) = match r {
                Some((x, w, rep)) => (x == k, w, rep),
                None => (false, f64::NAN, json!(null)),
            };
            rows.push(json!({"arm": arm, "c": cc, "bits": curves[ci].0, "res": curves[ci].1, "p": fc.f.p, "n": fc.n, "tseed": s, "fb": fb,
                             "ok": ok, "wall_ms": wall, "rep": rep, "commit": commit, "binary_sha256": bin, "extra": EXTRA}));
        }
        std::fs::write(out.join("rows.json"), serde_json::to_string(&rows).unwrap()).unwrap();
        println!(
            "{} rows; {} verified",
            rows.len(),
            rows.iter().filter(|r| r["ok"] == true).count()
        );
    }
}
