//! Independent validator for `research/cheon_ecbench_wide_20261106/`.
//!
//! It reads only the raw ecbench chunk sessions
//! (`sessions/{minus,plus}-b{36..56}/records.jsonl` and their `*.audit.json`
//! receipts of `ecbench verify --replay-all`) and the claims in
//! `gap-pooled.jsonl` and REPORT.md. It shares no code with the analysis
//! (`compare.sh`, `fit.sh`, `ecbench compare`). By its own arithmetic it
//! recomputes:
//!
//! * the pooled paired ratio over 36 to 56 bits, comb over rho, of summed
//!   `total_gae`, for each family. Each pair is the same workload in the same
//!   chunk session. The ratio is checked against `gap-pooled.jsonl` and
//!   against the three-decimal values REPORT.md prints;
//! * the same pooled sums with unpriced native work charged at w = 0.1 and
//!   1 GAE per `*_uncharged` operation, on both arms (AGENTS.md §5a), giving
//!   the gae band and whether the order against rho holds at every weight.
//!
//! It also derives the replay certificate from the audit receipts. Each
//! receipt row is one fresh-process re-execution made by `ecbench verify
//! --replay-all`.
//!
//! ```text
//! cheon_wide_validate --study research/cheon_ecbench_wide_20261106 \
//!     --out validator-report.json --cert-out replay-certificate.json --band-out band.json
//! ```
//!
//! Exit status is non-zero if any recomputed number disagrees with its claim.

use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

const BITS: [u64; 11] = [36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56];
const FAMILIES: [(&str, &str, f64); 2] = [
    // (family, label, the pooled w = 0 ratio REPORT.md prints)
    ("minus", "comb n-1", 0.525),
    ("plus", "comb n+1", 0.367),
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

fn lines(path: &str) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .collect()
}

/// One measured run that recovered its planted logarithm.
struct Run {
    arm: String,
    workload: String,
    ops: f64,
    unpriced: f64,
}

fn runs(path: &str) -> Vec<Run> {
    lines(path)
        .into_iter()
        .filter(|r| {
            r["warmup"] == false
                && r["outcome"]["status"] == "verified"
                && r["outcome"]["matches_planted"] == true
        })
        .map(|r| {
            let unpriced = r["counters"]
                .as_object()
                .unwrap()
                .iter()
                .filter(|(k, _)| k.ends_with("_uncharged"))
                .map(|(_, v)| v.as_f64().unwrap())
                .sum();
            Run {
                arm: r["arm"].as_str().unwrap().into(),
                workload: r["workload"]["workload_id"].as_str().unwrap().into(),
                ops: r["cost"]["total_gae"].as_f64().unwrap(),
                unpriced,
            }
        })
        .collect()
}

/// Sums of comb and rho cost, w GAE per unpriced operation on both arms,
/// over the workloads both arms ran in one session.
fn pair_sums(rs: &[Run], w: f64) -> (f64, f64, usize) {
    let rho: BTreeMap<&str, &Run> = rs
        .iter()
        .filter(|r| r.arm == "rho-neg")
        .map(|r| (r.workload.as_str(), r))
        .collect();
    let (mut num, mut den, mut n) = (0.0, 0.0, 0);
    for c in rs.iter().filter(|r| r.arm == "cheon-comb") {
        if let Some(h) = rho.get(c.workload.as_str()) {
            num += c.ops + w * c.unpriced;
            den += h.ops + w * h.unpriced;
            n += 1;
        }
    }
    (num, den, n)
}

fn main() {
    let study = arg("--study");
    let sess = format!("{study}/sessions");
    let out_p = arg("--out");
    let cert_p = arg("--cert-out");
    let band_p = arg("--band-out");
    let rel = |p: &str| format!("../../{p}");

    let claims: BTreeMap<String, f64> = lines(&format!("{study}/gap-pooled.jsonl"))
        .into_iter()
        .map(|c| {
            (
                c["family"].as_str().unwrap().to_string(),
                c["pooled_w0"].as_f64().unwrap(),
            )
        })
        .collect();

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

    let mut bands = serde_json::Map::new();
    let mut order_all = true;
    for (fam, label, printed) in FAMILIES {
        // Pairs are formed within each chunk session, then summed over sizes.
        let mut sums = [(0.0, 0.0, 0usize); 3];
        for b in BITS {
            let rs = runs(&format!("{sess}/{fam}-b{b}/records.jsonl"));
            for (i, w) in [0.0, 0.1, 1.0].into_iter().enumerate() {
                let (n, d, k) = pair_sums(&rs, w);
                sums[i].0 += n;
                sums[i].1 += d;
                sums[i].2 += k;
            }
        }
        let (n0, d0, k0) = sums[0];
        let name = format!("pooled paired ratio {label} 36-56 bits w=0 ({k0} pairs)");
        push(
            format!("{name} [gap-pooled.jsonl]"),
            claims[fam],
            n0 / d0,
            1e-9,
        );
        // REPORT.md prints three decimals: half a unit of the last place.
        push(
            format!("{name} [REPORT.md]"),
            printed,
            n0 / d0,
            0.0005 / printed * 1.0001,
        );
        let mut band = serde_json::Map::new();
        let mut order = true;
        for (i, key) in ["0", "0.1", "1"].into_iter().enumerate() {
            let (n, d, k) = sums[i];
            order &= n < d;
            band.insert(
                key.into(),
                json!({"ic": n, "rho": d, "ratio": n / d, "pairs": k}),
            );
        }
        order_all &= order;
        bands.insert(
            fam.into(),
            json!({"arm": format!("{label} vs rho.negation, pooled 36-56 bits, oracle queries unpriced"),
                   "gae_band": band, "order_holds_across_band": order}),
        );
    }
    std::fs::write(
        &band_p,
        serde_json::to_string_pretty(
            &json!({"families": bands, "order_holds_across_band": order_all}),
        )
        .unwrap()
            + "\n",
    )
    .unwrap();

    // Replay certificate from the audit receipts.
    let mut cert_runs = vec![];
    let mut sessions_meta = vec![];
    let mut raw: Vec<Value> = vec![];
    let mut all_scalars = true;
    for (fam, _, _) in FAMILIES {
        for b in BITS {
            let d = format!("{fam}-b{b}");
            let rec_p = format!("{sess}/{d}/records.jsonl");
            let aud_p = format!("{sess}/{d}.audit.json");
            let audit = read_json(&aud_p);
            let by_id: BTreeMap<String, Value> = lines(&rec_p)
                .into_iter()
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
                    "session": d, "seq": rp["seq"], "record_id": rp["record_id"],
                    "arm": r["arm"], "method": r["method"]["id"],
                    "bits": r["workload"]["curve"]["field_bits"],
                    "target_index": r["workload"]["target_index"],
                    "workload_id": r["workload"]["workload_id"],
                    "ops_recorded": r["cost"]["total_gae"],
                    "recovered": r["outcome"]["recovered"],
                    "identical": identical, "scalar_verified": scalar,
                }));
            }
            sessions_meta.push(json!({
                "session": d, "session_id": audit["session_id"], "audit_ok": audit["ok"],
                "records": audit["records"], "replays": audit["replays"].as_array().unwrap().len(),
                "audit_sha256": sha256_hex(&aud_p), "records_sha256": sha256_hex(&rec_p),
                "auditor_binary_sha256": audit["auditor_binary_sha256"],
            }));
            raw.push(json!({"path": rel(&rec_p), "sha256": sha256_hex(&rec_p)}));
            raw.push(json!({"path": rel(&aud_p), "sha256": sha256_hex(&aud_p)}));
        }
    }
    let n_runs = cert_runs.len();
    let cert = json!({
        "schema": "cheon-ecbench.replay/v1",
        "status": if all_scalars && n_runs > 0 { "verified" } else { "failed" },
        "independent": true,
        "scalars_verified": all_scalars,
        "independence": "each row is one re-execution made by `ecbench verify --replay-all` (src/cryptanalysis/ecbench/audit.rs): a fresh child process rebuilds the curve, the planted target and the method from the session plan and the recorded seed, and the receipt marks it reproduced only when the recovered scalar, total_gae, every phase count, every counter and the unpriced list all equal the recorded run. This file only joins the receipts to the records and re-checks that each recorded outcome was verified against the planted scalar; it did not re-execute anything itself.",
        "scope": "every run of the 22 chunk sessions (11 sizes x 2 families x 48 runs); receipts committed under research/cheon_ecbench_wide_20261106/sessions/*.audit.json, made on the host class that ran the sessions (two hosts, see sessions/BINARY.txt), not on a second machine",
        "sessions": sessions_meta,
        "runs": cert_runs,
    });
    std::fs::write(&cert_p, serde_json::to_string_pretty(&cert).unwrap() + "\n").unwrap();

    let gp = format!("{study}/gap-pooled.jsonl");
    let rp = format!("{study}/REPORT.md");
    let mut raw_data = vec![
        json!({"path": rel(&gp), "sha256": sha256_hex(&gp)}),
        json!({"path": rel(&rp), "sha256": sha256_hex(&rp)}),
    ];
    raw_data.extend(raw);
    let doc = json!({
        "schema": "cheon-ecbench.validator/v1",
        "status": if pass { "pass" } else { "fail" },
        "independent": true,
        "independence": "separate program (examples/cheon_wide_validate.rs) sharing no code with the study's analysis (compare.sh, fit.sh, ecbench compare); it reads the raw chunk-session run records, pairs each comb run with the rho.negation run of the same workload in the same session, and recomputes the pooled ratios by plain sums. Point estimates only; intervals and the fitted exponents are not recomputed.",
        "raw_data": raw_data,
        "headline": headline,
    });
    std::fs::write(&out_p, serde_json::to_string_pretty(&doc).unwrap() + "\n").unwrap();
    println!(
        "validator: {} ({} headline numbers), certificate: {} runs, order_holds={}",
        if pass { "pass" } else { "FAIL" },
        doc["headline"].as_array().unwrap().len(),
        n_runs,
        order_all
    );
    if !pass {
        std::process::exit(1);
    }
}
