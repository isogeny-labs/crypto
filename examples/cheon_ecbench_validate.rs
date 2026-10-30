//! Independent validator for `research/cheon_ecbench_20261022/`.
//!
//! Shares no code with `cheon_ecbench_analyse`.  It reads only the raw
//! ecbench session files (`sessions/*/records.jsonl` and the `*.audit.json`
//! receipts of `ecbench verify --replay-all`) and the claims in
//! `results/results.json`.  It recomputes by its own arithmetic:
//!
//! * the pooled paired ratios over 36 to 56 bits (sum of Cheon cost over sum of
//!   matched-rho cost, same workload, same session) at w = 0, 0.1 and 1 GAE per
//!   unpriced native operation, for comb n-1, comb n+1 and naive n-1, against
//!   the claim in `results.json` and against the three-decimal values printed
//!   in REPORT.md;
//! * the per-size mean operation counts of Table 1;
//! * the gae band of the n+1 comb arm (sums at w = 0, 0.1, 1) and whether the
//!   order against rho holds in all 12 (w, q) cells.
//!
//! It also derives the replay certificate from the audit receipts: each
//! receipt row is one fresh-process re-execution made by `ecbench verify
//! --replay-all`, which compares the recovered scalar and every count with the
//! recorded run.
//!
//! ```text
//! cheon_ecbench_validate --sessions research/cheon_ecbench_20261022/sessions \
//!     --results research/cheon_ecbench_20261022/results/results.json \
//!     --out validator-report.json --cert-out replay-certificate.json --band-out band.json
//! ```
//!
//! Exit status is non-zero if any recomputed number disagrees with its claim.

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const SESSIONS: [(&str, &str); 3] = [
    ("minus", "minus-20261022"),
    ("minus-naive", "minus-naive-20261022"),
    ("plus", "plus-20261022"),
];

fn arg(name: &str) -> String {
    let a: Vec<String> = std::env::args().collect();
    a.iter()
        .position(|x| x == name)
        .and_then(|i| a.get(i + 1).cloned())
        .unwrap_or_else(|| panic!("missing {name}"))
}

fn sha256_hex(path: &str) -> String {
    let b = std::fs::read(path).unwrap();
    Sha256::digest(&b)
        .iter()
        .map(|x| format!("{x:02x}"))
        .collect()
}

fn read_json(path: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

/// One measured, verified run.
struct Run {
    arm: String,
    bits: u64,
    workload: String,
    ops: f64,
    unpriced: f64,
    queries: f64,
    server: f64,
}

fn load(path: &str) -> Vec<(Value, Run)> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .filter(|r| {
            r["warmup"] == false
                && r["outcome"]["status"] == "verified"
                && r["outcome"]["matches_planted"] == true
        })
        .map(|r| {
            let c = r["counters"].as_object().unwrap();
            let unpriced: f64 = c
                .iter()
                .filter(|(k, _)| k.ends_with("_uncharged"))
                .map(|(_, v)| v.as_f64().unwrap())
                .sum();
            let get = |k: &str| c.get(k).and_then(|v| v.as_f64()).unwrap_or(0.0);
            let run = Run {
                arm: r["arm"].as_str().unwrap().into(),
                bits: r["workload"]["curve"]["field_bits"].as_u64().unwrap(),
                workload: r["workload"]["workload_id"].as_str().unwrap().into(),
                ops: r["cost"]["total_gae"].as_f64().unwrap(),
                unpriced,
                queries: get("oracle_queries"),
                server: get("oracle_server_group_ops"),
            };
            (r, run)
        })
        .collect()
}

/// Pooled ratio sum(cheon + w U + q Q) / sum(rho + w U_rho) over paired workloads.
fn pooled(
    runs: &[Run],
    arm: &str,
    w: f64,
    q: Option<f64>,
    lo: u64,
    hi: u64,
) -> (f64, f64, f64, usize) {
    let rho: BTreeMap<&str, &Run> = runs
        .iter()
        .filter(|r| r.arm == "rho-neg")
        .map(|r| (r.workload.as_str(), r))
        .collect();
    let (mut num, mut den, mut n) = (0.0, 0.0, 0);
    for c in runs
        .iter()
        .filter(|r| r.arm == arm && (lo..=hi).contains(&r.bits))
    {
        let Some(h) = rho.get(c.workload.as_str()) else {
            continue;
        };
        let oracle = match q {
            Some(q) => q * c.queries,
            None => c.server,
        };
        num += c.ops + w * c.unpriced + oracle;
        den += h.ops + w * h.unpriced;
        n += 1;
    }
    (num / den, num, den, n)
}

fn main() {
    let sess_dir = arg("--sessions");
    let results = read_json(&arg("--results"));
    let out_p = arg("--out");
    let cert_p = arg("--cert-out");
    let band_p = arg("--band-out");

    let mut data: BTreeMap<&str, Vec<(Value, Run)>> = BTreeMap::new();
    for (k, d) in SESSIONS {
        data.insert(k, load(&format!("{sess_dir}/{d}/records.jsonl")));
    }
    let runs_of = |k: &str| -> Vec<Run> {
        // Cheap re-materialisation: Run is not Clone, rebuild from the pair list.
        data[k]
            .iter()
            .map(|(_, r)| Run {
                arm: r.arm.clone(),
                bits: r.bits,
                workload: r.workload.clone(),
                ops: r.ops,
                unpriced: r.unpriced,
                queries: r.queries,
                server: r.server,
            })
            .collect()
    };

    let mut headline: Vec<Value> = vec![];
    let mut pass = true;
    let mut push = |name: String, claimed: f64, recomputed: f64, rel_tol: f64| {
        let ok = (claimed - recomputed).abs() <= rel_tol * claimed.abs().max(1e-300);
        if !ok {
            pass = false;
            eprintln!("DISAGREES: {name}: claimed {claimed} recomputed {recomputed}");
        }
        headline.push(
            json!({"name": name, "claimed": claimed, "recomputed": recomputed, "rel_tol": rel_tol}),
        );
    };

    // 1. pooled ratios, 36 to 56 bits. REPORT.md prints three decimals.
    let printed: [(&str, &str, &str, [f64; 3]); 3] = [
        ("minus", "cheon-comb", "comb n-1", [0.552, 0.528, 0.418]),
        ("plus", "cheon-comb", "comb n+1", [0.279, 0.259, 0.165]),
        (
            "minus-naive",
            "cheon-naive",
            "naive n-1",
            [7.660, 6.983, 3.938],
        ),
    ];
    for (sess, arm, label, vals) in printed {
        let runs = runs_of(sess);
        let family = if sess == "plus" { "plus" } else { "minus" };
        for (i, w) in [0.0, 0.1, 1.0].into_iter().enumerate() {
            let (ratio, _, _, pairs) = pooled(&runs, arm, w, Some(0.0), 36, 56);
            let wname = format!("pooled paired ratio {label} 36-56 bits w={w} q=0 ({pairs} pairs)");
            let claimed = results["table2b"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| {
                    r["arm"] == arm
                        && r["family"] == family
                        && r["q"] == "0"
                        && r["w"].as_f64().unwrap() == w
                })
                .map(|r| r["ratio"].as_f64().unwrap())
                .expect("claim missing in results.json");
            push(format!("{wname} [results.json]"), claimed, ratio, 1e-9);
            // REPORT.md prints the value to three decimals: half a unit of the
            // last place is the tolerance.
            push(
                format!("{wname} [REPORT.md]"),
                vals[i],
                ratio,
                0.0005 / vals[i] * 1.0001,
            );
        }
    }

    // 2. Table 1 mean operations per (family, bits, arm).
    for row in results["table1"].as_array().unwrap() {
        let arm = row["arm"].as_str().unwrap();
        let family = row["family"].as_str().unwrap();
        let bits = row["bits"].as_u64().unwrap();
        let sess = match (family, arm) {
            ("plus", _) => "plus",
            (_, "cheon-naive") => "minus-naive",
            _ => "minus",
        };
        let v: Vec<f64> = data[sess]
            .iter()
            .filter(|(_, r)| r.arm == arm && r.bits == bits)
            .map(|(_, r)| r.ops)
            .collect();
        let mean = v.iter().sum::<f64>() / v.len() as f64;
        push(
            format!("mean ops {family} {bits}b {arm} (n={})", v.len()),
            row["mean_ops"].as_f64().unwrap(),
            mean,
            1e-9,
        );
    }

    // 3. gae band for comb n+1 pooled over 36 to 56 bits, all 12 (w, q) cells.
    let plus = runs_of("plus");
    let mut band = serde_json::Map::new();
    for (key, w) in [("0", 0.0), ("0.1", 0.1), ("1", 1.0)] {
        let (_, num, den, _) = pooled(&plus, "cheon-comb", w, Some(0.0), 36, 56);
        band.insert(key.into(), json!({"ic": num, "rho": den}));
    }
    let mut cells = vec![];
    let mut order_holds = true;
    for w in [0.0, 0.1, 1.0] {
        for q in [Some(0.0), Some(0.1), Some(1.0), None] {
            let (ratio, _, _, _) = pooled(&plus, "cheon-comb", w, q, 36, 56);
            order_holds &= ratio < 1.0;
            cells.push(json!({"w": w, "q": q.map(|x| x.to_string()).unwrap_or("server".into()), "ratio": ratio}));
        }
    }
    std::fs::write(
        &band_p,
        serde_json::to_string_pretty(&json!({"arm": "cheon.pplus1_comb vs rho.negation, n+1 curves, pooled 36-56 bits, 48 pairs, oracle priced at q=0", "gae_band": band, "cells": cells, "order_holds_across_band": order_holds})).unwrap() + "\n",
    )
    .unwrap();

    // 4. replay certificate from the audit receipts.
    let mut cert_runs = vec![];
    let mut sessions_meta = vec![];
    let mut all_scalars = true;
    let mut raw: Vec<Value> = vec![];
    for (k, d) in SESSIONS {
        let rec_path = format!("{sess_dir}/{d}/records.jsonl");
        let aud_path = format!("{sess_dir}/{d}.audit.json");
        let audit = read_json(&aud_path);
        let by_id: BTreeMap<String, Value> = std::fs::read_to_string(&rec_path)
            .unwrap()
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str::<Value>(l).unwrap())
            .map(|r| (r["record_id"].as_str().unwrap().to_string(), r))
            .collect();
        for rp in audit["replays"].as_array().unwrap() {
            let r = &by_id[rp["record_id"].as_str().unwrap()];
            let identical = rp["reproduced"] == true && rp["detail"] == "identical";
            let scalar = identical
                && r["outcome"]["status"] == "verified"
                && r["outcome"]["matches_target"] == true
                && r["outcome"]["matches_planted"] == true;
            all_scalars &= scalar;
            cert_runs.push(json!({
                "session": k,
                "seq": rp["seq"],
                "record_id": rp["record_id"],
                "arm": r["arm"],
                "method": r["method"]["id"],
                "bits": r["workload"]["curve"]["field_bits"],
                "target_index": r["workload"]["target_index"],
                "workload_id": r["workload"]["workload_id"],
                "ops_recorded": r["cost"]["total_gae"],
                "recovered": r["outcome"]["recovered"],
                "identical": identical,
                "scalar_verified": scalar,
            }));
        }
        sessions_meta.push(json!({
            "session": k,
            "session_id": audit["session_id"],
            "audit_ok": audit["ok"],
            "records": audit["records"],
            "replays": audit["replays"].as_array().unwrap().len(),
            "audit_sha256": sha256_hex(&aud_path),
            "records_sha256": sha256_hex(&rec_path),
            "auditor_binary_sha256": audit["auditor_binary_sha256"],
        }));
        raw.push(json!({"path": format!("../../research/cheon_ecbench_20261022/sessions/{d}/records.jsonl"), "sha256": sha256_hex(&rec_path)}));
    }
    let n_runs = cert_runs.len();
    let cert = json!({
        "schema": "cheon-ecbench.replay/v1",
        "status": if all_scalars && n_runs > 0 { "verified" } else { "failed" },
        "independent": true,
        "scalars_verified": all_scalars,
        "independence": "each row is one re-execution made by `ecbench verify --replay-all` (src/cryptanalysis/ecbench/audit.rs): a fresh child process rebuilds the curve, the planted target and the method from the session plan and the recorded algorithm seed, and the receipt marks it reproduced only when the recovered scalar, total_gae, every phase count, every counter, the unpriced list and the automorphisms used all equal the recorded run. This file only joins the receipts to the records and re-checks that each recorded outcome was verified against the planted scalar; it did not re-execute anything itself.",
        "scope": "every measured run of the three sessions (minus 112, minus-naive 96, plus 112; no warmups were recorded); the receipts are those committed under research/cheon_ecbench_20261022/sessions/*.audit.json, produced by the same ecbench binary that ran the sessions on the same host class, not on a second machine",
        "sessions": sessions_meta,
        "runs": cert_runs,
    });
    std::fs::write(&cert_p, serde_json::to_string_pretty(&cert).unwrap() + "\n").unwrap();

    let mut raw_data = vec![
        json!({"path": "../../research/cheon_ecbench_20261022/results/results.json", "sha256": sha256_hex(&arg("--results"))}),
    ];
    raw_data.extend(raw);
    for (_, d) in SESSIONS {
        let p = format!("{sess_dir}/{d}.audit.json");
        raw_data.push(json!({"path": format!("../../research/cheon_ecbench_20261022/sessions/{d}.audit.json"), "sha256": sha256_hex(&p)}));
    }
    let doc = json!({
        "schema": "cheon-ecbench.validator/v1",
        "status": if pass { "pass" } else { "fail" },
        "independent": true,
        "independence": "separate program (examples/cheon_ecbench_validate.rs) sharing no code with examples/cheon_ecbench_analyse.rs; it reads the raw session run records, pairs each Cheon run with the rho.negation run of the same workload in the same session, and recomputes the pooled ratios and the per-size means by plain sums. Point estimates only; bootstrap intervals are not recomputed.",
        "raw_data": raw_data,
        "headline": headline,
    });
    std::fs::write(&out_p, serde_json::to_string_pretty(&doc).unwrap() + "\n").unwrap();
    println!(
        "validator: {} ({} headline numbers), certificate: {} runs, order_holds={}",
        if pass { "pass" } else { "FAIL" },
        doc["headline"].as_array().unwrap().len(),
        n_runs,
        order_holds
    );
    if !pass {
        std::process::exit(1);
    }
}
