//! **E40 Stage 0** (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.50): where
//! the native work of the chain solve's Macaulay matrix goes, counted on
//! real calls ([`macaulay_profile::stage0`]; nothing here is charged to a
//! solve).
//!
//! Curves: E25's ten GLS curves (factor base and precompute from E25's
//! stream) and E24's twelve curves (E24's stream).  Calls: the first
//! `--targets` targets of a stream of Stage 0's own (generator
//! `StdRng(seed ^ 0xE40D_0000)` after the precompute, per-target seeds as
//! E37's Stage A).  Per curve: the mean, minimum and maximum of every count,
//! the per-lead-degree sums, and the number of distinct lead and support
//! digests.
//!
//! ```text
//! ./e40_stage0 --targets 200 --json experiments/47_macaulay_entries/stage0.json
//! ```

use std::collections::BTreeSet;

use crypto_lib::cryptanalysis::klein_v4::{v4_system, SymmetrisedV4, MACAULAY_BOUND};
use crypto_lib::cryptanalysis::macaulay_profile::{stage0, Stage0};
use crypto_lib::cryptanalysis::quartic_folds::{
    generate_gls_klein_instance, generate_klein_instance, klein_base, klein_base_with,
};
use crypto_lib::cryptanalysis::residual_walk::mix64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Map, Value};

const SALT: u64 = 0xE40D_0000;
const GLS: [u64; 5] = [101, 149, 197, 269, 397];
const E24: [u64; 6] = [101, 149, 197, 269, 397, 521];

fn arg<T: std::str::FromStr>(args: &[String], k: &str, d: T) -> T {
    args.iter()
        .position(|a| a == k)
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(d)
}

fn summarise(set: &str, p: u64, seed: u64, calls: &[Stage0]) -> Value {
    let vals: Vec<Map<String, Value>> = calls
        .iter()
        .map(|c| match serde_json::to_value(c).unwrap() {
            Value::Object(m) => m,
            _ => unreachable!(),
        })
        .collect();
    let mut fields = Map::new();
    for k in vals[0].keys() {
        let xs: Vec<f64> = vals.iter().filter_map(|m| m[k].as_f64()).collect();
        if xs.len() != vals.len() {
            continue;
        }
        let mean = xs.iter().sum::<f64>() / xs.len() as f64;
        let lo = xs.iter().copied().fold(f64::INFINITY, f64::min);
        let hi = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        fields.insert(k.clone(), json!({"mean": mean, "min": lo, "max": hi}));
    }
    let deg = calls[0].by_lead_degree.len();
    let mut by = vec![[0u64; 3]; deg];
    for c in calls {
        for (t, v) in by.iter_mut().zip(&c.by_lead_degree) {
            for k in 0..3 {
                t[k] += v[k];
            }
        }
    }
    let leads: BTreeSet<u64> = calls.iter().map(|c| c.leads_digest).collect();
    let sups: BTreeSet<u64> = calls.iter().map(|c| c.support_digest).collect();
    json!({
        "set": set, "p": p, "seed": seed, "calls": calls.len(),
        "fields": fields,
        "by_lead_degree_sum": by,
        "distinct_leads_digests": leads.len(),
        "distinct_support_digests": sups.len(),
        "echelon_reproduced_all": calls.iter().all(|c| c.echelon_reproduced),
        "unreachable_calls": calls.iter().filter(|c| c.unreachable).count(),
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let targets: usize = arg(&args, "--targets", 200);
    let threads: u64 = arg(&args, "--threads", 4);
    let json_out: String = arg(&args, "--json", String::new());
    let mut out: Vec<Value> = Vec::new();
    for (set, sizes) in [("gls", &GLS[..]), ("e24", &E24[..])] {
        for &p in sizes {
            for seed in [1u64, 2] {
                let (ki, mut rng) = if set == "gls" {
                    let ki = generate_gls_klein_instance(p, seed, 16).expect("instance");
                    let mut rng = StdRng::seed_from_u64(seed ^ 0xE25A_6175);
                    let _ = klein_base_with(&ki, false, &mut rng).expect("base");
                    (ki, rng)
                } else {
                    let ki = generate_klein_instance(p, seed, 16).expect("instance");
                    let mut rng = StdRng::seed_from_u64(seed ^ 0xE24A_4B1E);
                    let _ = klein_base(&ki, &mut rng).expect("base");
                    (ki, rng)
                };
                let inst = ki.inst;
                let pcurve = inst.curve();
                let pre = SymmetrisedV4::precompute(&ki, &pcurve, &mut rng, 8).expect("precompute");
                let mut rng = StdRng::seed_from_u64(seed ^ SALT);
                let sseed = seed ^ SALT;
                let n = inst.n;
                let mut calls: Vec<Stage0> = Vec::new();
                let mut residuals = 0u64;
                'outer: loop {
                    let batch: Vec<(u64, u64, u64)> = (0..threads * 2)
                        .map(|k| {
                            (
                                rng.gen_range(0..n),
                                rng.gen_range(1..n),
                                mix64(sseed ^ (residuals + k).wrapping_mul(0x9E37_79B9_7F4A_7C15)),
                            )
                        })
                        .collect();
                    for (a, b, _tseed) in batch {
                        if calls.len() >= targets {
                            break 'outer;
                        }
                        residuals += 1;
                        let c = inst.curve();
                        let r = c.add(&c.mul(&inst.g, a), &c.mul(&inst.q, b));
                        let Some(z_r) = ki.z_of_x(&c.f, &r.x) else {
                            continue;
                        };
                        let mut fp = 0u64;
                        let comps = v4_system(&pre, &c.f, &z_r, &mut fp);
                        if let Some(s) = stage0(&comps, MACAULAY_BOUND, p) {
                            calls.push(s);
                        }
                    }
                }
                let v = summarise(set, p, seed, &calls);
                let fm = |k: &str| v["fields"][k]["mean"].as_f64().unwrap_or(f64::NAN);
                eprintln!(
                    "{set} p={p} s={seed}: rows {:.1} cols {:.0} dense {:.0} tail {:.0} ({:.3}) prefix {:.0} | terms {:.0} lead0 {:.0} lead {:.0} | zero rows {:.2} dup {:.2} untouched cols {:.2} | dim {:.1} chains {:.1} reached {:.1} dep rows {:.1} | digests {} {} | ok {}",
                    fm("rows"), fm("cols"), fm("dense_entries"), fm("tail_entries"),
                    fm("tail_entries") / fm("dense_entries"), fm("prefix_entries"),
                    fm("term_writes"), fm("initial_lead_scan"), fm("lead_scan"),
                    fm("zero_rows"), fm("duplicate_rows"), fm("untouched_columns"),
                    fm("dim"), fm("chains"), fm("reached"), fm("dependency_rows"),
                    v["distinct_leads_digests"], v["distinct_support_digests"],
                    v["echelon_reproduced_all"]
                );
                out.push(v);
                if !json_out.is_empty() {
                    std::fs::write(&json_out, serde_json::to_string_pretty(&out).unwrap()).unwrap();
                }
            }
        }
    }
}
