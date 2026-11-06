//! Analysis of `research/ic_acct1_cover_bitop_20261021` (PROTOCOL.md
//! "Measures", "Controls", "Decision rule"), from the JSON lines
//! `jv_cover_ic_vs_rho_bitops` writes: per `(p, moves)` the `bitop/v1` ratio
//! of means IC / rho against the executed R3 (primary) and R0 rho of the
//! same `p`, with percentile-bootstrap intervals (instances and rho runs
//! resampled independently), the §5a band (each arm's `word_op` and
//! `hash_probe` work re-priced at 0, 0.1 and 1 GAE per operation), the
//! controls, and the verdict the protocol's rule gives.
//!
//! ```sh
//! cargo run --release --example jv_cover_bitop_analyse -- \
//!     --committed experiments/40_jv_cover_e2e_fast.json \
//!     --aa raw/ic_aa.jsonl --out analysis.json raw/ic_*.jsonl raw/rho_*.jsonl
//! ```
//!
//! Everything printed is computed here from the recorded per-run counts.

use std::collections::BTreeMap;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};

const RESAMPLES: usize = 10_000;
const SEED: u64 = 20261021;
const BAND: [f64; 3] = [0.0, 0.1, 1.0];
/// Protocol: positive-control windows for the mean of `steps / √ℓ`.
const R0_WINDOW: (f64, f64) = (0.9, 1.7);
const R3_WINDOW: (f64, f64) = (0.55, 1.25);

fn num(v: &Value) -> f64 {
    match v {
        Value::String(s) => s.parse().expect("numeric string"),
        Value::Number(n) => n.as_f64().expect("number"),
        _ => panic!("not a number: {v}"),
    }
}

/// Bitops of `word_op` and `hash_probe` in a tally, and their counts.
fn native(total: &Value, w_bits: u64) -> (f64, f64) {
    let c = &total["counts"];
    let words = num(&c["word_op"]);
    let probes = num(&c["hash_probe"]);
    // bitop/v1: word_op = 64, hash_probe = 4·bits(p)
    (words * 64.0 + probes * 4.0 * w_bits as f64, words + probes)
}

#[derive(Clone, Debug)]
struct Run {
    bitops: f64,
    native_bitops: f64,
    native_ops: f64,
    ok: bool,
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len().max(1) as f64
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    let i = ((sorted.len() - 1) as f64 * q).round() as usize;
    sorted[i]
}

/// Ratio of means `a/b` with a percentile-bootstrap 95% interval; `f`
/// prices a run.
fn ratio_ci(a: &[Run], b: &[Run], f: &dyn Fn(&Run) -> f64, rng: &mut StdRng) -> (f64, f64, f64) {
    let av: Vec<f64> = a.iter().map(f).collect();
    let bv: Vec<f64> = b.iter().map(f).collect();
    let point = mean(&av) / mean(&bv);
    let mut boots: Vec<f64> = (0..RESAMPLES)
        .map(|_| {
            let ma = (0..av.len())
                .map(|_| av[rng.gen_range(0..av.len())])
                .sum::<f64>()
                / av.len() as f64;
            let mb = (0..bv.len())
                .map(|_| bv[rng.gen_range(0..bv.len())])
                .sum::<f64>()
                / bv.len() as f64;
            ma / mb
        })
        .collect();
    boots.sort_by(|x, y| x.partial_cmp(y).unwrap());
    (point, quantile(&boots, 0.025), quantile(&boots, 0.975))
}

fn read_lines(path: &str) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{path}: {e}"))
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).expect("json line"))
        .collect()
}

fn verdict(hi: f64, lo: f64) -> &'static str {
    if hi < 1.0 {
        "below rho"
    } else if lo > 1.0 {
        "above rho"
    } else {
        "unresolved"
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out_path: Option<String> = None;
    let mut committed: Option<String> = None;
    let mut aa: Option<String> = None;
    let mut files: Vec<String> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = Some(args[i + 1].clone());
                i += 2;
            }
            "--committed" => {
                committed = Some(args[i + 1].clone());
                i += 2;
            }
            "--aa" => {
                aa = Some(args[i + 1].clone());
                i += 2;
            }
            f => {
                files.push(f.to_string());
                i += 1;
            }
        }
    }
    let rows: Vec<Value> = files.iter().flat_map(|f| read_lines(f)).collect();
    let mut rng = StdRng::seed_from_u64(SEED);

    // IC runs by (p, moves); rho runs by (p, variant); GAE by p
    let mut ic: BTreeMap<(u64, u64), Vec<(u64, Run, Value)>> = BTreeMap::new();
    let mut rho: BTreeMap<(u64, String), Vec<(Run, Value)>> = BTreeMap::new();
    let mut gae: BTreeMap<u64, f64> = BTreeMap::new();
    let mut completeness = Vec::new();
    for r in &rows {
        let p = r["p"].as_u64().unwrap();
        let bits = 64 - p.leading_zeros() as u64;
        let (nb, no) = native(&r["total"], bits);
        let run = Run {
            bitops: num(&r["total_bitops"]),
            native_bitops: nb,
            native_ops: no,
            ok: false,
        };
        match r["arm"].as_str().unwrap() {
            "ic" => {
                let ok = r["solved"].as_bool() == Some(true)
                    && r["correct"].as_bool() == Some(true)
                    && r["verified_on_challenge"].as_bool() == Some(true);
                let foreign = r["foreign_hook_calls"].as_u64().unwrap();
                let other = r["other_modulus_ops"].as_u64().unwrap();
                completeness.push(json!({
                    "p": p, "seed": r["seed"], "moves": r["moves"],
                    "foreign_hook_calls": foreign, "other_modulus_ops": other,
                    "complete": foreign == 0 && other == 0,
                }));
                ic.entry((p, r["moves"].as_u64().unwrap()))
                    .or_default()
                    .push((r["seed"].as_u64().unwrap(), Run { ok, ..run }, r.clone()));
            }
            "rho" => {
                let ok = r["correct"].as_bool() == Some(true)
                    && r["verified_on_challenge"].as_bool() == Some(true)
                    && r["capped"].as_bool() == Some(false);
                let pr = &r["fq3_prices"];
                // one affine addition of the walk: 2 products, a squaring,
                // an inversion and 42 F_p additions (bitop/v1 at bits(p))
                let add_bits = 13.0 * bits as f64;
                let g = 2.0 * num(&pr["mul"]["bitops"])
                    + num(&pr["sq"]["bitops"])
                    + num(&pr["inv"]["bitops"])
                    + 42.0 * add_bits;
                gae.insert(p, g);
                rho.entry((p, r["variant"].as_str().unwrap().to_string()))
                    .or_default()
                    .push((Run { ok, ..run }, r.clone()));
            }
            a => panic!("unknown arm {a}"),
        }
    }

    // ── controls
    let mut controls = serde_json::Map::new();
    let window = |variant: &str, (lo, hi): (f64, f64)| -> Value {
        let v: Vec<f64> = rho
            .iter()
            .filter(|((_, var), _)| var == variant)
            .flat_map(|(_, runs)| runs.iter().map(|(_, r)| num(&r["steps_over_sqrt_l"])))
            .collect();
        let m = mean(&v);
        json!({"runs": v.len(), "mean_steps_over_sqrt_l": m, "window": [lo, hi],
               "pass": !v.is_empty() && m >= lo && m <= hi})
    };
    controls.insert("positive_R0".into(), window("R0", R0_WINDOW));
    controls.insert("positive_R3".into(), window("R3", R3_WINDOW));
    // rho A/A: R3 at each p, run index 0 against run index 1
    let mut rho_aa = Vec::new();
    for ((p, var), runs) in &rho {
        if var != "R3" {
            continue;
        }
        let half = |k: u64| -> Vec<Run> {
            runs.iter()
                .filter(|(_, r)| r["run"].as_u64() == Some(k))
                .map(|(x, _)| x.clone())
                .collect()
        };
        let (a, b) = (half(0), half(1));
        if a.is_empty() || b.is_empty() {
            continue;
        }
        let (pt, lo, hi) = ratio_ci(&a, &b, &|r| r.bitops, &mut rng);
        rho_aa.push(
            json!({"p": p, "runs": [a.len(), b.len()], "ratio": pt, "ci95": [lo, hi],
                           "pass": lo <= 1.0 && hi >= 1.0}),
        );
    }
    controls.insert("rho_aa".into(), Value::Array(rho_aa.clone()));
    // IC A/A: a re-run of an instance must give identical counts
    if let Some(path) = &aa {
        let again = read_lines(path);
        let mut checks = Vec::new();
        for r in &again {
            let key = (r["p"].as_u64().unwrap(), r["moves"].as_u64().unwrap());
            let seed = r["seed"].as_u64().unwrap();
            let first = ic
                .get(&key)
                .and_then(|v| v.iter().find(|(s, _, _)| *s == seed))
                .map(|(_, _, row)| row.clone());
            let same = first.as_ref().is_some_and(|f| {
                f["phases"] == r["phases"] && f["legacy"]["total_muls"] == r["legacy"]["total_muls"]
            });
            checks.push(json!({"p": key.0, "moves": key.1, "seed": seed, "identical": same}));
        }
        controls.insert("ic_aa".into(), Value::Array(checks));
    }
    // IC reproduction: walk, transport and model legacy counts equal the
    // committed §18.4 rows of the same (p, seed, moves)
    if let Some(path) = &committed {
        let old: Vec<Value> =
            serde_json::from_str(&std::fs::read_to_string(path).expect("committed")).expect("json");
        let mut checks = Vec::new();
        for runs in ic.values() {
            for (seed, _, r) in runs {
                let found = old.iter().find(|o| {
                    o["p"] == r["p"]
                        && o["seed"].as_u64() == Some(*seed)
                        && o["moves"] == r["moves"]
                });
                if let Some(o) = found {
                    let same = ["walk_muls", "transport_muls", "model_muls"]
                        .iter()
                        .all(|k| o[*k] == r["legacy"][*k]);
                    checks.push(json!({"p": r["p"], "seed": seed, "moves": r["moves"], "identical": same,
                        "committed": [o["walk_muls"], o["transport_muls"], o["model_muls"]],
                        "measured": [r["legacy"]["walk_muls"], r["legacy"]["transport_muls"], r["legacy"]["model_muls"]]}));
                }
            }
        }
        controls.insert("ic_reproduces_committed_reach".into(), Value::Array(checks));
    }
    controls.insert("completeness".into(), Value::Array(completeness.clone()));
    let all_pass = |v: &Value| -> bool {
        match v {
            Value::Array(a) => a.iter().all(|x| {
                x.get("pass")
                    .or(x.get("identical"))
                    .or(x.get("complete"))
                    .and_then(Value::as_bool)
                    .unwrap_or(false)
            }),
            Value::Object(_) => v["pass"].as_bool().unwrap_or(false),
            _ => false,
        }
    };
    let controls_pass = controls.values().all(all_pass);

    // ── per (p, moves): IC against R3 and R0 of the same p
    let mut table = Vec::new();
    println!("| p | moves | IC runs (ok) | IC bitops mean | rho | rho runs (ok) | rho bitops mean | IC/rho | 95% CI | band w=0 | w=0.1 | w=1 | legacy row S/rho (pooled 1.3609) | verdict |");
    println!("|--:|--:|:--|--:|:--|:--|--:|--:|:--|:--|:--|:--|--:|:--|");
    for ((p, moves), runs) in &ic {
        let icr: Vec<Run> = runs.iter().map(|(_, r, _)| r.clone()).collect();
        let ic_ok = icr.iter().all(|r| r.ok);
        let legacy = mean(
            &runs
                .iter()
                .map(|(_, _, r)| num(&r["legacy"]["s_over_pooled_rho"]))
                .collect::<Vec<_>>(),
        );
        for var in ["R3", "R0"] {
            let Some(rr) = rho.get(&(*p, var.to_string())) else {
                continue;
            };
            let rhor: Vec<Run> = rr.iter().map(|(r, _)| r.clone()).collect();
            let rho_ok = rhor.iter().all(|r| r.ok);
            let (pt, lo, hi) = ratio_ci(&icr, &rhor, &|r| r.bitops, &mut rng);
            let g = gae[p];
            let mut band = Vec::new();
            for w in BAND {
                let f = move |r: &Run| r.bitops - r.native_bitops + w * g * r.native_ops;
                let (bp, blo, bhi) = ratio_ci(&icr, &rhor, &f, &mut rng);
                band.push(json!({"w_gae": w, "ratio": bp, "ci95": [blo, bhi]}));
            }
            let band_hi = band.iter().map(|b| num(&b["ci95"][1])).fold(hi, f64::max);
            let band_lo = band.iter().map(|b| num(&b["ci95"][0])).fold(lo, f64::min);
            let v = if !(ic_ok && rho_ok) {
                "unresolved (a run failed)"
            } else if !controls_pass {
                "withheld (controls)"
            } else {
                verdict(band_hi, band_lo)
            };
            let bf = |k: usize| {
                format!(
                    "{:.4} [{:.4}, {:.4}]",
                    num(&band[k]["ratio"]),
                    num(&band[k]["ci95"][0]),
                    num(&band[k]["ci95"][1])
                )
            };
            println!(
                "| {p} | {moves} | {} ({}) | {:.3e} | {var} | {} ({}) | {:.3e} | {:.4} | [{:.4}, {:.4}] | {} | {} | {} | {:.4} | {v} |",
                icr.len(),
                icr.iter().filter(|r| r.ok).count(),
                mean(&icr.iter().map(|r| r.bitops).collect::<Vec<_>>()),
                rhor.len(),
                rhor.iter().filter(|r| r.ok).count(),
                mean(&rhor.iter().map(|r| r.bitops).collect::<Vec<_>>()),
                pt, lo, hi, bf(0), bf(1), bf(2), legacy
            );
            // per-phase means of the IC arm (bitops), for the phase table
            let mut phases = serde_json::Map::new();
            if let Some(Value::Object(ph)) = runs.first().map(|(_, _, r)| r["phases"].clone()) {
                for name in ph.keys() {
                    let v: Vec<f64> = runs
                        .iter()
                        .map(|(_, _, r)| num(&r["phases"][name]["bitops"]))
                        .collect();
                    phases.insert(name.clone(), json!(mean(&v)));
                }
            }
            let mut rho_phases = serde_json::Map::new();
            if let Some(Value::Object(ph)) = rr.first().map(|(_, r)| r["phases"].clone()) {
                for name in ph.keys() {
                    let v: Vec<f64> = rr
                        .iter()
                        .map(|(_, r)| num(&r["phases"][name]["bitops"]))
                        .collect();
                    rho_phases.insert(name.clone(), json!(mean(&v)));
                }
            }
            table.push(json!({
                "p": p, "moves": moves, "rho": var,
                "ic_runs": icr.len(), "ic_ok": icr.iter().filter(|r| r.ok).count(),
                "rho_runs": rhor.len(), "rho_ok": rhor.iter().filter(|r| r.ok).count(),
                "ic_bitops_mean": mean(&icr.iter().map(|r| r.bitops).collect::<Vec<_>>()),
                "rho_bitops_mean": mean(&rhor.iter().map(|r| r.bitops).collect::<Vec<_>>()),
                "ratio": pt, "ci95": [lo, hi], "band": band,
                "gae_bitops": g,
                "ic_phase_bitops_mean": phases,
                "rho_phase_bitops_mean": rho_phases,
                "legacy_row_s_over_pooled_rho_mean": legacy,
                "rho_legacy_s_harness_mean": mean(&rr.iter().map(|(_, r)| num(&r["legacy"]["s_harness"])).collect::<Vec<_>>()),
                "ic_legacy_s_mean": mean(&runs.iter().map(|(_, _, r)| num(&r["legacy"]["s"])).collect::<Vec<_>>()),
                "verdict": v,
            }));
        }
    }
    println!();
    println!("controls pass: {controls_pass}");
    println!(
        "{}",
        serde_json::to_string_pretty(&Value::Object(controls.clone())).unwrap()
    );
    let out = json!({
        "schema": "ic_acct1_cover_bitop_analysis/v1",
        "resamples": RESAMPLES,
        "bootstrap_seed": SEED,
        "band_gae": BAND,
        "controls": controls,
        "controls_pass": controls_pass,
        "rows": table,
    });
    if let Some(path) = out_path {
        std::fs::write(&path, serde_json::to_string_pretty(&out).unwrap() + "\n")
            .expect("write --out");
    }
}
