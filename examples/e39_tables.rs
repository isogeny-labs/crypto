//! **E39 (§9.48) tables**: the pilot (G2.190), the controls (G2.192,
//! G2.193), Stage A (G2.191, G2.194–G2.196) and end to end on E25's 20 GLS
//! rows and E24's 12 curves against A, E25, B″ and the matched rho (walk
//! (iii)), with the negative-control arm L, at `0`, `0.1` and `1` GAE and
//! σ1–σ3 (G2.197–G2.203).  `e38_tables.rs` with C_F^post (`P_post`) in place
//! of C_F^mid, E39's thresholds, the per-row fallback ceiling of §9.48 and the
//! distance to rho of every arm on every row; reads recorded JSON only and
//! skips a stage whose records do not exist (a kill stops the run).  Writes
//! `tables.md` and `summary.json` (the values the report plots).
//!
//! ```text
//! ./e39_tables experiments/46_pivot_rules_confirm3
//! ```

use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use serde_json::{json, Map, Value};

const GAE: f64 = 97.0;
const GS: [f64; 3] = [0.0, 0.1, 1.0];
const GLS: [(u64, u64); 10] = [
    (101, 1),
    (101, 2),
    (149, 1),
    (149, 2),
    (197, 1),
    (197, 2),
    (269, 1),
    (269, 2),
    (397, 1),
    (397, 2),
];
const E24: [(u64, u64); 12] = [
    (101, 1),
    (101, 2),
    (149, 1),
    (149, 2),
    (197, 1),
    (197, 2),
    (269, 1),
    (269, 2),
    (397, 1),
    (397, 2),
    (521, 1),
    (521, 2),
];
const RULES: [&str; 3] = ["chain-fifo", "chain-static", "chain-fifo-static"];
/// End-to-end arms: A (run as `chain-counted`), E25 (`macaulay`), the
/// rules, B″, L.
const ARMS: [&str; 7] = [
    "chain-counted",
    "macaulay",
    "chain-fifo",
    "chain-static",
    "chain-fifo-static",
    "chain-sparsest-est",
    "chain-lifo",
];
/// Chain arms whose fallbacks must coincide (G2.198).
const CHAIN_ARMS: [&str; 6] = [
    "chain-counted",
    "chain-fifo",
    "chain-static",
    "chain-fifo-static",
    "chain-sparsest-est",
    "chain-lifo",
];

fn load(path: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}")))
        .unwrap()
}

fn load_opt(path: &str) -> Option<Value> {
    Path::new(path).exists().then(|| load(path))
}

fn f(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or_else(|| panic!("field {k}"))
}

fn short(s: &str) -> &'static str {
    match s {
        "chain-fifo" => "F",
        "chain-static" => "S",
        "chain-fifo-static" => "H",
        "chain-lifo" => "L",
        "chain-sparsest-est" => "B″",
        "chain-counted" | "chain" => "A",
        "macaulay" | "existing" => "E25",
        _ => "?",
    }
}

fn sci(v: f64) -> String {
    if v == 0.0 {
        return "0".into();
    }
    let e = v.abs().log10().floor() as i32;
    format!("{:.2}·10^{}", v / 10f64.powi(e), e)
}

fn range(v: &[f64]) -> (f64, f64) {
    v.iter()
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), &x| {
            (lo.min(x), hi.max(x))
        })
}

fn arm_of(recs: &Value, a: Option<&str>) -> Value {
    let arr = recs.as_array().unwrap();
    match a {
        Some(a) => arr
            .iter()
            .find(|r| r["arm"] == a)
            .unwrap_or_else(|| panic!("arm {a}"))
            .clone(),
        None => arr[0].clone(),
    }
}

/// Fields a rule's (or B″'s, L's) record may change against A's on the same
/// stream: the solve's multiplications and what derives from them, the
/// selection and lead-scan fields, wall times and the tag.
fn may_differ(k: &str) -> bool {
    k.ends_with("_ms")
        || matches!(
            k,
            "solve_muls"
                | "total_muls"
                | "call"
                | "ratio"
                | "solver"
                | "pivot_selection"
                | "pivot_selection_ops"
                | "lead_scan_reads"
        )
}

fn diff_fields(a: &Value, b: &Value, allow: fn(&str) -> bool) -> Vec<String> {
    let (ao, bo) = (a.as_object().unwrap(), b.as_object().unwrap());
    let mut keys: Vec<&String> = ao.keys().chain(bo.keys()).collect();
    keys.sort();
    keys.dedup();
    keys.into_iter()
        .filter(|k| !allow(k) && ao.get(*k) != bo.get(*k))
        .cloned()
        .collect()
}

fn within(v: f64, lo: f64, hi: f64) -> bool {
    (lo..=hi).contains(&v)
}

/// One control tally against its thresholds: `(positive rate, detection
/// rate on all calls, detection rate on non-empty calls)`.
fn rates(t: &Value) -> (f64, f64, f64) {
    let n = f(t, "calls");
    let ne = f(t, "nonempty").max(1.0);
    (
        f(t, "positive") / n,
        f(t, "detected_positive") / n,
        f(t, "nonempty_detected_positive") / ne,
    )
}

/// The gating controls of §9.48: `(key, rule, label)`.
const GATING: [(&str, &str, &str); 4] = [
    ("c_f", "F", "C_F (i), P_ext"),
    ("c_f_post", "F", "C_F^post (ii), P_post"),
    ("c_sh", "S", "C_SH (S), strict"),
    ("c_sh", "H", "C_SH (H), strict"),
];

fn control_table(
    md: &mut String,
    ck: &Value,
    title: &str,
    pos_min: f64,
    ne_min: f64,
) -> (bool, Vec<Value>) {
    writeln!(
        md,
        "\n### {title}: {} calls (salt `{}`), {} with a non-empty clean set; fallbacks used: {}\n",
        ck["calls"],
        ck["salt"].as_str().unwrap_or("?"),
        ck["nonempty"],
        ck["fallbacks_used"]
    )
    .unwrap();
    writeln!(md, "| control | rule | check | aimed | effective | positive | strict | pivots changed | targets changed | intact | first diff at r* | NF read of r* altered | detected raw | detected (positive) | rate | non-empty detected | rate | via fallback | thresholds met |").unwrap();
    writeln!(
        md,
        "|:--|:--|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|"
    )
    .unwrap();
    let mut all_ok = true;
    let mut out = Vec::new();
    let mut rows: Vec<(&str, &str, bool)> = GATING.iter().map(|(k, r, _)| (*k, *r, true)).collect();
    for r in ["F", "S", "H", "L"] {
        rows.push(("c_0", r, false));
    }
    for (key, rule, gating) in rows {
        let t = &ck[key][rule];
        let (pos, det, ned) = rates(t);
        let ok = pos >= pos_min && ned >= ne_min;
        if gating {
            all_ok &= ok;
        }
        writeln!(
            md,
            "| {} | {rule} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {det:.3} | {}/{} | {ned:.3} | {} | {ok}{} |",
            key.to_uppercase(), t["check"].as_str().unwrap_or("?"), t["aimed"], t["effective"], t["positive"], t["strict"],
            t["pivots_changed"], t["targets_changed"], t["structure_intact"], t["first_at_target"], t["nf_read_altered"],
            t["detected_raw"], t["detected_positive"],
            t["nonempty_detected_positive"], t["nonempty"], t["via_fallback"], if gating { "" } else { " (calibration)" }
        )
        .unwrap();
        out.push(
            json!({"control": key, "rule": rule, "gating": gating, "calls": t["calls"],
            "positive_rate": pos, "detection_rate": det, "nonempty_detection_rate": ned,
            "thresholds_met": ok, "tally": t}),
        );
    }
    (all_ok, out)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let dir = args
        .first()
        .cloned()
        .unwrap_or("experiments/46_pivot_rules_confirm3".into());
    let e27 = "experiments/32_walk_karatsuba";
    let e37 = "experiments/44_pivot_rules_confirm";
    let mut md = String::new();
    let mut verdict = Map::new();
    let mut summary = Map::new();
    writeln!(md, "# E39 tables (generated by `examples/e39_tables.rs`)\n").unwrap();

    // ── Identity of existing configurations (G2.191, binary clauses) ──────
    if let Some(id) = load_opt(&format!("{dir}/identity_existing.json")) {
        let runs = id["runs"].as_array().unwrap();
        let ok = runs.iter().all(|r| r["identical"] == true);
        writeln!(
            md,
            "## Existing configurations under the E39 binaries (G2.191)\n"
        )
        .unwrap();
        writeln!(md, "| config | reference | identical |\n|:--|:--|:--|").unwrap();
        for r in runs {
            writeln!(
                md,
                "| {} | {} | {} |",
                r["config"].as_str().unwrap(),
                r["reference"].as_str().unwrap(),
                r["identical"]
            )
            .unwrap();
        }
        writeln!(
            md,
            "\n{} / {} byte-identical: **{ok}**\n",
            runs.iter().filter(|r| r["identical"] == true).count(),
            runs.len()
        )
        .unwrap();
        verdict.insert("G2.191_existing_configs".into(), json!(ok));
    }

    // ── Pilot (G2.190) and controls (G2.192, G2.193) ─────────────────────
    writeln!(md, "## Controls\n\nPositive checks: strict = the control's read log first differs from the clean run's at the same phase and column with a different hash; `P_ext` = strict or pivot set or border targets changed; `P_post` = pivot set and border targets unchanged, the first differing entry is the read of the altered row `r*` at `l*` (same phase, other hash), and a phase-1 (normal-form) read of `r*` at `l*` has another hash.  Detected: solutions, degree, decomposition set or fallback flag differ; a call without its registered positive check counts as not detected.").unwrap();
    let mut pilots = Vec::new();
    for (name, file) in [
        ("pilot P1", "pilot.json"),
        ("fallback pilot P2", "pilot_p2.json"),
    ] {
        if let Some(ck) = load_opt(&format!("{dir}/{file}")) {
            let (_, ctl) = control_table(&mut md, &ck, name, 76.0 / 80.0, 0.90);
            let pass: Vec<(String, bool)> = ctl
                .iter()
                .filter(|c| c["gating"] == true)
                .map(|c| {
                    let ok = c["positive_rate"].as_f64().unwrap() >= 76.0 / 80.0
                        && c["nonempty_detection_rate"].as_f64().unwrap() >= 0.90;
                    (
                        format!(
                            "{} {}",
                            c["control"].as_str().unwrap(),
                            c["rule"].as_str().unwrap()
                        ),
                        ok,
                    )
                })
                .collect();
            writeln!(
                md,
                "\nPilot pass (positive `≥ 76/80`, non-empty detection `≥ 0.90`): {pass:?}"
            )
            .unwrap();
            pilots.push(json!({"pilot": name, "controls": ctl, "pass": pass,
                "perturbed_detected": ck["perturbed_detected"], "nonempty": ck["nonempty"],
                "identity_failures": ck["identity_failures"]}));
        }
    }
    if let Some(p1) = pilots.first() {
        let all_pass = p1["pass"].as_array().unwrap().iter().all(|x| x[1] == true);
        verdict.insert("G2.190_pilot_all_primaries_pass".into(), json!(all_pass));
    }
    summary.insert("pilots".into(), json!(pilots));
    let ck = load_opt(&format!("{dir}/checks.json"));
    let mut controls_ok = None;
    if let Some(ck) = &ck {
        let (gate, ctl) = control_table(&mut md, ck, "main control run", 0.95, 0.96);
        let nonempty = ck["nonempty"].as_u64().unwrap();
        let pert_ok = ["F", "S", "H", "L"]
            .iter()
            .all(|r| ck["perturbed_detected"][r].as_u64() == Some(nonempty));
        let dig_ok = ["F", "L"]
            .iter()
            .all(|r| ck["digest_differs_from_a"][r].as_u64() == ck["contested_calls_f"].as_u64());
        let g192 = gate && pert_ok && dig_ok && ck["identity_failures"].as_u64() == Some(0);
        writeln!(md, "\nPerturbed coefficient detected (non-empty calls {nonempty}): {}; digests ≠ A: {} (contested calls {}); clean-run identity failures: {}.  **G2.192 thresholds met: {g192}**\n",
            ck["perturbed_detected"], ck["digest_differs_from_a"], ck["contested_calls_f"], ck["identity_failures"]).unwrap();
        // G2.193: mechanics of the F controls.
        let tf = &ck["c_f"]["F"];
        let tp = &ck["c_f_post"]["F"];
        let n = f(tf, "calls");
        let g193 = f(tf, "positive") / n >= 0.99
            && f(tf, "strict") / n <= 0.05
            && f(tp, "aimed") / n >= 0.99
            && f(tp, "positive") == f(tp, "aimed")
            && f(tp, "structure_intact") == f(tp, "aimed")
            && f(tp, "first_at_target") == f(tp, "aimed");
        // Aim positions of C_F^post and C_F.
        let aims = ck["aims"].as_array().unwrap();
        let vs: Vec<f64> = aims.iter().filter_map(|a| a["v"].as_f64()).collect();
        let ls: Vec<f64> = aims.iter().filter_map(|a| a["l"].as_f64()).collect();
        let cs: Vec<f64> = aims.iter().filter_map(|a| a["c_star"].as_f64()).collect();
        let nr: Vec<f64> = aims
            .iter()
            .filter_map(|a| a["nf_reads_at_l"].as_f64())
            .collect();
        let (v0, v1) = range(&vs);
        let (l0, l1) = range(&ls);
        let (cs0, cs1) = range(&cs);
        let (n0, n1) = range(&nr);
        writeln!(md, "Aims: C_F `c*` in `[{cs0}, {cs1}]`; C_F^post `|V|` in `[{v0}, {v1}]`, `l*` in `[{l0}, {l1}]`, phase-1 reads at `l*` in `[{n0}, {n1}]` ({} aimed of {}).  **G2.193 (mechanics) met: {g193}**\n", ls.len(), aims.len()).unwrap();
        verdict.insert("G2.192_controls".into(), json!(g192));
        verdict.insert("G2.193_f_control_mechanics".into(), json!(g193));
        summary.insert("controls".into(), json!(ctl));
        summary.insert(
            "control_aims".into(),
            json!({"c_star": [cs0, cs1], "v": [v0, v1], "l_star": [l0, l1], "nf_reads_at_l": [n0, n1]}),
        );
        controls_ok = Some(g192);
    }

    // ── Stage A ──────────────────────────────────────────────────────────
    let mut stage = Vec::new();
    let mut pooled: Map<String, Value> = Map::new();
    if let Some(sa) = load_opt(&format!("{dir}/stage_a.json")) {
        writeln!(md, "\n## Stage A (fresh calls: salt `0xE39A_0001`; per call, means over all calls, fallbacks charged)\n").unwrap();
        writeln!(md, "| p | seed | calls | agree (existing, A, A counted, B″, A-rule, F, S, H, L) | A-rule = A counted | same fallback calls (chain solvers) | fallbacks | digest ≠ A | contested calls |").unwrap();
        writeln!(md, "|--:|--:|--:|:--|--:|:--|--:|:--|--:|").unwrap();
        let mut sa_identity = true;
        let mut pool_acc: BTreeMap<String, (f64, f64, f64)> = Default::default();
        for c in sa.as_array().unwrap() {
            let calls = c["calls"].as_u64().unwrap() as usize;
            let agree: Vec<usize> = c["agree"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap() as usize)
                .collect();
            let low = c["lowest_equals_a"].as_u64().unwrap() as usize;
            let arms = c["arms"].as_array().unwrap();
            let fb0 = &arms[1]["fallback_calls"];
            let same_fb = arms[1..].iter().all(|a| &a["fallback_calls"] == fb0);
            if agree.iter().any(|&a| a != calls) || low != calls || !same_fb {
                sa_identity = false;
            }
            writeln!(
                md,
                "| {} | {} | {calls} | {:?} | {low} | {same_fb} | {} | {} | {} |",
                c["p"],
                c["seed"],
                agree,
                fb0.as_array().map_or(0, |v| v.len()),
                c["digest_differs_from_a"],
                c["calls_contested"]
            )
            .unwrap();
            let sums = c["summaries"].as_array().unwrap();
            let by = |n: &str| sums.iter().find(|s| s["solver"] == n).unwrap().clone();
            let sv = |n: &str, g: usize| by(n)["s_mean"][g].as_f64().unwrap();
            let a_lead = f(&by("chain-counted"), "lead_scan_mean");
            let a_ord = f(&by("chain-lowest-e36"), "ordered_candidates_mean");
            let sens = |n: &str, g: usize, which: usize| -> f64 {
                let w = GAE * GS[g];
                let extra = match (which, n) {
                    (1, "existing" | "chain") => a_lead,
                    (1, _) => f(&by(n), "lead_scan_mean"),
                    (2, "existing" | "chain") => a_ord,
                    (2, "chain-sparsest-est") => 0.0,
                    (2, _) => f(&by(n), "ordered_candidates_mean"),
                    (3, "chain-fifo" | "chain-static" | "chain-fifo-static" | "chain-lifo") => {
                        f(&by(n), "contested_mean")
                    }
                    _ => 0.0,
                };
                sv(n, g) + w * extra
            };
            for n in [
                "chain-lowest-e36",
                "chain-fifo",
                "chain-static",
                "chain-fifo-static",
                "chain-lifo",
            ] {
                let e = pool_acc.entry(n.to_string()).or_default();
                e.0 += f(&by(n), "ordered_candidates_mean");
                e.1 += f(&by(n), "contested_mean");
                e.2 += 1.0;
            }
            let mut per = Map::new();
            for n in [
                "existing",
                "chain",
                "chain-lowest-e36",
                "chain-sparsest-est",
                "chain-fifo",
                "chain-static",
                "chain-fifo-static",
                "chain-lifo",
            ] {
                let b = by(n);
                per.insert(
                    n.into(),
                    json!({
                        "c_mean": f(&b, "call_mean"), "c_median": b["call_median"].clone(),
                        "echelon_mean": f(&b, "echelon_mean"), "entries_mean": f(&b, "entries_mean"),
                        "sel_mean": f(&b, "sel_mean"), "lead_scan_mean": f(&b, "lead_scan_mean"),
                        "contested_mean": f(&b, "contested_mean"),
                        "ordered_candidates_mean": f(&b, "ordered_candidates_mean"),
                        "fallbacks": b["fallbacks"].clone(),
                        "s": [sens(n, 0, 0), sens(n, 1, 0), sens(n, 2, 0)],
                        "s_sigma1": [sens(n, 0, 1), sens(n, 1, 1), sens(n, 2, 1)],
                        "s_sigma2": [sens(n, 0, 2), sens(n, 1, 2), sens(n, 2, 2)],
                        "s_sigma3": [sens(n, 0, 3), sens(n, 1, 3), sens(n, 2, 3)],
                    }),
                );
            }
            stage.push(json!({"p": c["p"], "seed": c["seed"], "calls": calls, "per": per}));
        }
        for (n, (o, ct, k)) in &pool_acc {
            pooled.insert(
                n.clone(),
                json!({"ordered_candidates_mean": o / k, "contested_mean": ct / k}),
            );
        }
        writeln!(md, "\nStage A identity (every solver agrees with the existing solver on every call; A's rule through E36's echelon = A counted; the same calls fall back for every chain solver): **{sa_identity}**\n").unwrap();
        writeln!(md, "| p | seed | X | C mean | sel / call | C_A/C_X | S_A/S_X 0 | 0.1 | 1 | σ1 1 | σ2 1 | σ3 1 | S_E25/S_X 1 | S_B″/S_X 0 | 0.1 | 1 |").unwrap();
        writeln!(
            md,
            "|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|"
        )
        .unwrap();
        let (mut g194, mut g195, mut g196) = (true, true, true);
        let mut sa_ranges: BTreeMap<String, Vec<Vec<f64>>> = Default::default();
        for r in &stage {
            let per = &r["per"];
            let s = |n: &str, k: &str, g: usize| per[n][k][g].as_f64().unwrap();
            let ca = f(&per["chain"], "c_mean");
            let sel_s = f(&per["chain-static"], "sel_mean");
            let sel_h = f(&per["chain-fifo-static"], "sel_mean");
            g194 &= f(&per["chain-fifo"], "sel_mean") == 0.0
                && f(&per["chain-lifo"], "sel_mean") == 0.0
                && within(sel_s, 500.0, 525.0)
                && within(sel_h - sel_s, 325.0, 345.0);
            let cf = f(&per["chain-fifo"], "c_mean");
            let cb = f(&per["chain-sparsest-est"], "c_mean");
            g195 &= within(ca / cf, 1.44, 1.58)
                && within(ca / f(&per["chain-static"], "c_mean"), 1.38, 1.49)
                && within(ca / f(&per["chain-fifo-static"], "c_mean"), 1.48, 1.59)
                && within(f(&per["chain-lifo"], "c_mean") / ca, 1.36, 1.44)
                && within(cf / cb, 1.00, 1.08);
            for x in [
                "chain-fifo",
                "chain-static",
                "chain-fifo-static",
                "chain-lifo",
                "chain-sparsest-est",
            ] {
                let rat = |k: &str, g: usize| s("chain", k, g) / s(x, k, g);
                let v = vec![
                    ca / f(&per[x], "c_mean"),
                    rat("s", 0),
                    rat("s", 1),
                    rat("s", 2),
                    rat("s_sigma1", 2),
                    rat("s_sigma2", 2),
                    rat("s_sigma3", 2),
                    s("existing", "s", 2) / s(x, "s", 2),
                    s("chain-sparsest-est", "s", 0) / s(x, "s", 0),
                    s("chain-sparsest-est", "s", 1) / s(x, "s", 1),
                    s("chain-sparsest-est", "s", 2) / s(x, "s", 2),
                ];
                if RULES.contains(&x) {
                    let all_ahead = ["s", "s_sigma1", "s_sigma2", "s_sigma3"]
                        .iter()
                        .all(|k| (0..3).all(|g| rat(k, g) > 1.0));
                    let band = match x {
                        "chain-fifo" => within(v[3], 1.009, 1.016),
                        _ => within(v[3], 1.006, 1.011),
                    };
                    g196 &= all_ahead && band;
                }
                if x == "chain-lifo" {
                    g196 &= (0..3).all(|g| rat("s", g) < 1.0);
                }
                writeln!(
                    md,
                    "| {} | {} | {} | {:.0} | {:.1} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} |",
                    r["p"], r["seed"], short(x), f(&per[x], "c_mean"), f(&per[x], "sel_mean"),
                    v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10]
                )
                .unwrap();
                sa_ranges.entry(x.to_string()).or_default().push(v);
            }
        }
        writeln!(md, "\nStage A ranges over the ten curves (`[min, max]`):\n").unwrap();
        writeln!(md, "| X | C_A/C_X | S_A/S_X 0 | 0.1 | 1 | σ1 1 | σ2 1 | σ3 1 | S_E25/S_X 1 | S_B″/S_X 0 | 0.1 | 1 |").unwrap();
        writeln!(md, "|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|").unwrap();
        let mut sa_range_json = Map::new();
        for (x, rows) in &sa_ranges {
            let cols: Vec<(f64, f64)> = (0..rows[0].len())
                .map(|i| range(&rows.iter().map(|r| r[i]).collect::<Vec<_>>()))
                .collect();
            let cells: Vec<String> = cols.iter().map(|(a, b)| format!("{a:.4}–{b:.4}")).collect();
            writeln!(md, "| {} | {} |", short(x), cells.join(" | ")).unwrap();
            sa_range_json.insert(
                x.clone(),
                json!(cols.iter().map(|(a, b)| json!([a, b])).collect::<Vec<_>>()),
            );
        }
        verdict.insert("G2.191_stage_a_identity".into(), json!(sa_identity));
        verdict.insert("G2.194_selection".into(), json!(g194));
        verdict.insert("G2.195_quality_bands".into(), json!(g195));
        verdict.insert("G2.196_stage_a_band".into(), json!(g196));
        summary.insert("stage_a".into(), json!(stage));
        summary.insert("stage_a_ranges".into(), json!(sa_range_json));
        summary.insert("stage_a_pooled_tallies".into(), json!(pooled));
    }

    // ── End to end ───────────────────────────────────────────────────────
    let have_e2e = GLS.iter().all(|(p, s)| {
        ARMS.iter()
            .all(|a| Path::new(&format!("{dir}/e2e/gls_{a}_p{p}s{s}.json")).exists())
    }) && E24.iter().all(|(p, s)| {
        ARMS.iter()
            .all(|a| Path::new(&format!("{dir}/e2e/e24_{a}_p{p}s{s}.json")).exists())
    });
    if have_e2e && !stage.is_empty() {
        let s27 = load(&format!("{e27}/summary.json"));
        let w24 = load(&format!("{e37}/walk_e24.json"));
        let stage_by = |p: u64, seed: u64| -> Option<&Value> {
            stage
                .iter()
                .find(|r| r["p"].as_u64() == Some(p) && r["seed"].as_u64() == Some(seed))
        };
        let tally = |set: &str, p: u64, seed: u64, arm: &str| -> (f64, f64) {
            let ordered = matches!(
                arm,
                "chain-counted" | "macaulay" | "chain-static" | "chain-fifo-static"
            );
            let contested = matches!(
                arm,
                "chain-fifo" | "chain-static" | "chain-fifo-static" | "chain-lifo"
            );
            if !ordered && !contested {
                return (0.0, 0.0);
            }
            let key = match arm {
                "chain-counted" | "macaulay" => "chain-lowest-e36",
                x => x,
            };
            let src = if set == "gls" {
                stage_by(p, seed).unwrap()["per"][key].clone()
            } else {
                pooled[key].clone()
            };
            (
                if ordered {
                    f(&src, "ordered_candidates_mean")
                } else {
                    0.0
                },
                if contested {
                    f(&src, "contested_mean")
                } else {
                    0.0
                },
            )
        };
        let mut rows: Vec<Value> = Vec::new();
        let mut identity = Vec::new();
        let mut health = Vec::new();
        let mut fb_same = true;
        for (set, curves) in [("gls", &GLS[..]), ("e24", &E24[..])] {
            for &(p, seed) in curves {
                let recs: Vec<(&str, Value)> = ARMS
                    .iter()
                    .map(|s| (*s, load(&format!("{dir}/e2e/{set}_{s}_p{p}s{seed}.json"))))
                    .collect();
                let arms: Vec<Option<&str>> = if set == "gls" {
                    vec![Some("A"), Some("B")]
                } else {
                    vec![None]
                };
                for a in arms {
                    let (rho, steps) = if set == "gls" {
                        let r = s27["rows"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|r| r["p"] == p && r["seed"] == seed && r["arm"] == a.unwrap())
                            .unwrap();
                        (f(r, "rho_e27"), f(r, "walk_steps"))
                    } else {
                        let c = w24["curves"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .find(|c| c["p"] == p && c["seed"] == seed)
                            .unwrap();
                        (f(&c["mean_fp_muls"], "batched"), f(c, "mean_steps"))
                    };
                    let rho_g: Vec<f64> = GS.iter().map(|g| rho + GAE * g * steps).collect();
                    let xa = arm_of(&recs[0].1, a);
                    let a_lead = xa["lead_scan_reads"].as_f64().unwrap_or(0.0);
                    let mut row = json!({"set": set, "p": p, "seed": seed, "arm": a.unwrap_or("—"),
                        "rho": rho, "walk_steps": steps, "rho_g": rho_g});
                    let fb_a = xa["solver_fallbacks"].as_f64().unwrap_or(-1.0);
                    // §9.48: E37's fallback rate on this row (chain-fifo) + 0.50 points.
                    let e37f = arm_of(
                        &load(&format!("{e37}/e2e/{set}_chain-fifo_p{p}s{seed}.json")),
                        a,
                    );
                    let fb_ceiling = f(&e37f, "solver_fallbacks") / f(&e37f, "residuals") + 0.005;
                    for (s, rec) in &recs {
                        let x = arm_of(rec, a);
                        let calls = f(&x, "residuals");
                        let sel = x["pivot_selection_ops"].as_f64().unwrap_or(0.0);
                        let lead = if *s == "macaulay" {
                            a_lead
                        } else {
                            x["lead_scan_reads"].as_f64().unwrap_or(0.0)
                        };
                        let (ord, cont) = tally(set, p, seed, s);
                        let base: Vec<f64> = GS
                            .iter()
                            .map(|g| {
                                f(&x, "total_muls") + GAE * g * (f(&x, "macaulay_entries") + sel)
                            })
                            .collect();
                        let plus = |extra: f64| -> Vec<f64> {
                            (0..3).map(|g| base[g] + GAE * GS[g] * extra).collect()
                        };
                        row[*s] = json!({
                            "S": base, "S_sigma1": plus(lead), "S_sigma2": plus(ord * calls),
                            "S_sigma3": plus(cont * calls), "calls": calls,
                            "selection_per_call": sel / calls, "c_mean": x["call"]["mean"].clone(),
                            "over_rho": (0..3).map(|g| base[g] / rho_g[g]).collect::<Vec<_>>(),
                        });
                        let unsolved = f(&x, "unsolved");
                        let fb = x["solver_fallbacks"].as_f64().unwrap_or(0.0);
                        if CHAIN_ARMS.contains(s) && fb != fb_a {
                            fb_same = false;
                        }
                        let rho_ok = x["rho"]
                            .as_array()
                            .is_some_and(|w| w.iter().all(|w| w["correct"] == true));
                        health.push(json!({"set": set, "p": p, "seed": seed, "arm": a.unwrap_or("—"), "solver": s,
                            "log_recovered": x["folded"]["correct"] == true, "rho_walks_correct": rho_ok,
                            "unsolved_share": unsolved / calls, "fallback_share": fb / calls,
                            "fallback_ceiling": fb_ceiling,
                            "stream_salt": x["stream_salt"].clone()}));
                        if *s != "chain-counted" && *s != "macaulay" {
                            let d = diff_fields(&xa, &x, may_differ);
                            identity.push(
                                json!({"set": set, "p": p, "seed": seed, "arm": a.unwrap_or("—"),
                                "solver": s, "fields_differing_from_a": d}),
                            );
                        } else if *s == "macaulay" {
                            identity.push(
                                json!({"set": set, "p": p, "seed": seed, "arm": a.unwrap_or("—"),
                                "solver": s, "informational": true,
                                "fields_differing_from_a": diff_fields(&xa, &x, may_differ)}),
                            );
                        }
                    }
                    rows.push(row);
                }
            }
        }
        let ident_ok = identity
            .iter()
            .filter(|i| i["informational"] != true)
            .all(|i| i["fields_differing_from_a"].as_array().unwrap().is_empty());
        let salt_ok = health.iter().all(|h| h["stream_salt"] == "0xe39e0001");
        verdict.insert("G2.191_e2e_identity".into(), json!(ident_ok));
        verdict.insert("e2e_stream_salt_0xe39e0001".into(), json!(salt_ok));
        let (u_max, fb_max) = (
            health
                .iter()
                .map(|h| f(h, "unsolved_share"))
                .fold(0.0, f64::max),
            health
                .iter()
                .map(|h| f(h, "fallback_share"))
                .fold(0.0, f64::max),
        );
        let g198 = fb_same
            && health.iter().all(|h| {
                h["log_recovered"] == true
                    && h["rho_walks_correct"] == true
                    && f(h, "unsolved_share") <= 0.001
                    && f(h, "fallback_share") <= f(h, "fallback_ceiling")
            });

        let refs = [
            ("chain-counted", "S_A / S_X"),
            ("macaulay", "S_E25 / S_X"),
            ("chain-sparsest-est", "S_B″ / S_X"),
        ];
        writeln!(md, "\n## End to end (paired on the fresh stream `0xE39E_0001`; Karatsuba products for every arm)\n").unwrap();
        writeln!(md, "Health: logs recovered and walks correct on every record; max unsolved share `{u_max:.5}`, max fallback share `{fb_max:.5}`; same fallback count for every chain arm on every row: **{fb_same}**; fallback ceiling per row = E37's rate on the row + 0.50 points.  G2.198: **{g198}**\n").unwrap();
        writeln!(md, "Per row: `S_ref / S_X` at `0 / 0.1 / 1` GAE (`>1`: X cheaper) and `S_X / rho` (walk (iii)).\n").unwrap();
        for x in RULES.iter().copied().chain(["chain-lifo"]) {
            writeln!(md, "### {} (`{x}`)\n", short(x)).unwrap();
            writeln!(md, "| set | p | seed | arm | sel/call | A/X 0 | 0.1 | 1 | E25/X 0 | 0.1 | 1 | B″/X 0 | 0.1 | 1 | X/rho 0 | 0.1 | 1 |").unwrap();
            writeln!(
                md,
                "|:--|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|"
            )
            .unwrap();
            for r in &rows {
                let v = |n: &str, g: usize| {
                    r[n]["S"][g].as_f64().unwrap() / r[x]["S"][g].as_f64().unwrap()
                };
                let o = |g: usize| r[x]["over_rho"][g].as_f64().unwrap();
                writeln!(
                    md,
                    "| {} | {} | {} | {} | {:.1} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {} | {} | {} |",
                    r["set"].as_str().unwrap(), r["p"], r["seed"], r["arm"].as_str().unwrap(),
                    f(&r[x], "selection_per_call"),
                    v("chain-counted", 0), v("chain-counted", 1), v("chain-counted", 2),
                    v("macaulay", 0), v("macaulay", 1), v("macaulay", 2),
                    v("chain-sparsest-est", 0), v("chain-sparsest-est", 1), v("chain-sparsest-est", 2),
                    sci(o(0)), sci(o(1)), sci(o(2))
                )
                .unwrap();
            }
            writeln!(md).unwrap();
        }
        writeln!(md, "### Ranges and orders (an order is stated only where it holds at every weight on every row of the set)\n").unwrap();
        writeln!(md, "| X | reference | rows | 0 GAE | 0.1 GAE | 1 GAE | rows X ahead (0/0.1/1) | primary order | σ1 | σ2 | σ3 |").unwrap();
        writeln!(md, "|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|:--|").unwrap();
        let mut ranks = Vec::new();
        let order_of = |ahead: &[usize], n: usize| -> &'static str {
            if ahead.iter().all(|&a| a == n) {
                "X ahead across the band"
            } else if ahead.iter().all(|&a| a == 0) {
                "X behind across the band"
            } else {
                "unresolved"
            }
        };
        for x in RULES
            .iter()
            .copied()
            .chain(["chain-sparsest-est", "chain-lifo"])
        {
            for (rf, what) in refs {
                if rf == x {
                    continue;
                }
                for set in ["gls", "e24", "all"] {
                    let sel: Vec<&Value> = rows
                        .iter()
                        .filter(|r| set == "all" || r["set"] == set)
                        .collect();
                    let n = sel.len();
                    let mut cells = Vec::new();
                    let mut ahead = Vec::new();
                    let mut rr = Vec::new();
                    for g in 0..3 {
                        let v: Vec<f64> = sel
                            .iter()
                            .map(|r| {
                                r[rf]["S"][g].as_f64().unwrap() / r[x]["S"][g].as_f64().unwrap()
                            })
                            .collect();
                        let (lo, hi) = range(&v);
                        rr.push(json!([lo, hi]));
                        cells.push(format!("{lo:.4}–{hi:.4}"));
                        ahead.push(v.iter().filter(|&&q| q > 1.0).count());
                    }
                    let mut sig = Vec::new();
                    let mut sig_ahead = Vec::new();
                    for k in ["S_sigma1", "S_sigma2", "S_sigma3"] {
                        let ah: Vec<usize> = (0..3)
                            .map(|g| {
                                sel.iter()
                                    .filter(|r| {
                                        r[rf][k][g].as_f64().unwrap() / r[x][k][g].as_f64().unwrap()
                                            > 1.0
                                    })
                                    .count()
                            })
                            .collect();
                        sig.push(order_of(&ah, n));
                        sig_ahead.push(ah);
                    }
                    let prim = order_of(&ahead, n);
                    writeln!(
                        md,
                        "| {} | {what} | {set} ({n}) | {} | {} | {} | {}/{}/{} | {prim} | {} | {} | {} |",
                        short(x), cells[0], cells[1], cells[2], ahead[0], ahead[1], ahead[2], sig[0], sig[1], sig[2]
                    )
                    .unwrap();
                    ranks.push(json!({"x": x, "reference": rf, "set": set, "rows": n, "ranges": rr,
                        "rows_x_ahead": ahead, "order": prim, "order_sigma": sig, "rows_x_ahead_sigma": sig_ahead}));
                }
            }
        }
        // Distance to rho, every arm.
        writeln!(
            md,
            "\n### Distance to the matched rho (`S_X / rho`, walk (iii))\n"
        )
        .unwrap();
        writeln!(
            md,
            "| X | rows | 0 GAE | 0.1 GAE | 1 GAE | rows below rho |"
        )
        .unwrap();
        writeln!(md, "|:--|:--|:--|:--|:--|--:|").unwrap();
        let mut rho_ranges = Vec::new();
        let mut any_below = false;
        for x in ARMS {
            for set in ["gls", "e24"] {
                let sel: Vec<&Value> = rows.iter().filter(|r| r["set"] == set).collect();
                let mut cells = Vec::new();
                let mut rr = Vec::new();
                let mut below = 0;
                for g in 0..3 {
                    let v: Vec<f64> = sel
                        .iter()
                        .map(|r| r[x]["over_rho"][g].as_f64().unwrap())
                        .collect();
                    below += v.iter().filter(|&&q| q < 1.0).count();
                    let (lo, hi) = range(&v);
                    rr.push(json!([lo, hi]));
                    cells.push(format!("{}–{}", sci(lo), sci(hi)));
                }
                any_below |= below > 0;
                writeln!(
                    md,
                    "| {} | {set} ({}) | {} | {} | {} | {below} |",
                    short(x),
                    sel.len(),
                    cells[0],
                    cells[1],
                    cells[2]
                )
                .unwrap();
                rho_ranges.push(json!({"x": x, "set": set, "ranges": rr, "rows_below_rho": below}));
            }
        }
        for (g, gname) in ["0", "0.1", "1"].iter().enumerate() {
            writeln!(
                md,
                "\n### Distance to the matched rho per row at {gname} GAE (`S_X / rho`)\n"
            )
            .unwrap();
            writeln!(md, "| set | p | seed | arm | A | E25 | F | S | H | B″ | L |\n|:--|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|").unwrap();
            for r in &rows {
                let o = |x: &str| sci(r[x]["over_rho"][g].as_f64().unwrap());
                writeln!(
                    md,
                    "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                    r["set"].as_str().unwrap(),
                    r["p"],
                    r["seed"],
                    r["arm"].as_str().unwrap(),
                    o("chain-counted"),
                    o("macaulay"),
                    o("chain-fifo"),
                    o("chain-static"),
                    o("chain-fifo-static"),
                    o("chain-sparsest-est"),
                    o("chain-lifo")
                )
                .unwrap();
            }
        }

        // A/A across fresh streams: S_A / S_F here vs E37's stream.
        writeln!(md, "\n### A/A across fresh streams (32 rows): `S_A / S_F` on E37's stream `0xE37E0001` and here\n").unwrap();
        writeln!(
            md,
            "| set | p | seed | arm | E37 0 | 0.1 | 1 | E39 0 | 0.1 | 1 | Δ 0 | Δ 0.1 | Δ 1 |"
        )
        .unwrap();
        writeln!(md, "|:--|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
        let mut aa = Vec::new();
        let mut g202 = true;
        let sx = |x: &Value, g: f64| {
            f(x, "total_muls")
                + GAE
                    * g
                    * (f(x, "macaulay_entries") + x["pivot_selection_ops"].as_f64().unwrap_or(0.0))
        };
        for r in &rows {
            let (set, p, seed) = (
                r["set"].as_str().unwrap(),
                r["p"].as_u64().unwrap(),
                r["seed"].as_u64().unwrap(),
            );
            let a = (set == "gls").then(|| r["arm"].as_str().unwrap());
            let ra = arm_of(
                &load(&format!("{e37}/e2e/{set}_chain-counted_p{p}s{seed}.json")),
                a,
            );
            let rf = arm_of(
                &load(&format!("{e37}/e2e/{set}_chain-fifo_p{p}s{seed}.json")),
                a,
            );
            let old: Vec<f64> = GS.iter().map(|&g| sx(&ra, g) / sx(&rf, g)).collect();
            let new: Vec<f64> = (0..3)
                .map(|g| {
                    r["chain-counted"]["S"][g].as_f64().unwrap()
                        / r["chain-fifo"]["S"][g].as_f64().unwrap()
                })
                .collect();
            let d: Vec<f64> = (0..3).map(|g| new[g] - old[g]).collect();
            g202 &= d[0].abs() <= 0.05 && d[1].abs() <= 0.01 && d[2].abs() <= 0.002;
            writeln!(md, "| {set} | {p} | {seed} | {} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:+.4} | {:+.4} | {:+.5} |",
                a.unwrap_or("—"), old[0], old[1], old[2], new[0], new[1], new[2], d[0], d[1], d[2]).unwrap();
            aa.push(json!({"set": set, "p": p, "seed": seed, "arm": a.unwrap_or("—"), "e37": old, "e39": new, "delta": d}));
        }
        verdict.insert("G2.202_aa".into(), json!(g202));

        // Registered bands and orders.
        let rng_of = |x: &str, rf: &str, g: usize| -> (f64, f64) {
            let r = ranks
                .iter()
                .find(|r| r["x"] == x && r["reference"] == rf && r["set"] == "all")
                .unwrap();
            (
                r["ranges"][g][0].as_f64().unwrap(),
                r["ranges"][g][1].as_f64().unwrap(),
            )
        };
        let inside = |x: &str, g: usize, lo: f64, hi: f64| {
            let (a, b) = rng_of(x, "chain-counted", g);
            a >= lo && b <= hi
        };
        let rank_of = |x: &str, rf: &str| -> String {
            ranks
                .iter()
                .find(|r| r["x"] == x && r["reference"] == rf && r["set"] == "all")
                .map(|r| r["order"].as_str().unwrap().to_string())
                .unwrap()
        };
        let ahead = |x: &str| {
            rank_of(x, "chain-counted") == "X ahead across the band"
                && rank_of(x, "macaulay") == "X ahead across the band"
        };
        let (e0, e1) = rng_of("chain-fifo", "macaulay", 0);
        let g197_bands = inside("chain-fifo", 0, 1.43, 1.57)
            && inside("chain-fifo", 1, 1.085, 1.115)
            && inside("chain-fifo", 2, 1.010, 1.015)
            && inside("chain-static", 0, 1.38, 1.48)
            && inside("chain-static", 1, 1.075, 1.100)
            && inside("chain-static", 2, 1.006, 1.011)
            && inside("chain-fifo-static", 0, 1.47, 1.57)
            && inside("chain-fifo-static", 1, 1.087, 1.110)
            && inside("chain-fifo-static", 2, 1.006, 1.011)
            && e0 >= 3.3
            && e1 <= 4.2;
        let g197_order: Vec<(String, bool)> = RULES
            .iter()
            .map(|x| (short(x).to_string(), ahead(x)))
            .collect();
        let g199 = RULES
            .iter()
            .all(|x| rank_of(x, "chain-sparsest-est") == "unresolved")
            && {
                let (lo, hi) = rng_of("chain-fifo", "chain-sparsest-est", 0);
                lo >= 0.92 && hi < 1.0
            };
        let l_sigma2_ahead_1gae = ranks
            .iter()
            .find(|r| {
                r["x"] == "chain-lifo" && r["reference"] == "chain-counted" && r["set"] == "all"
            })
            .map(|r| r["rows_x_ahead_sigma"][1][2].as_u64() == r["rows"].as_u64())
            .unwrap();
        let g200 = rank_of("chain-lifo", "chain-counted") == "X behind across the band"
            && inside("chain-lifo", 0, 0.68, 0.75)
            && inside("chain-lifo", 1, 0.88, 0.92)
            && inside("chain-lifo", 2, 0.980, 0.990)
            && l_sigma2_ahead_1gae;
        let gates = controls_ok == Some(true)
            && verdict.get("G2.191_existing_configs") == Some(&json!(true))
            && verdict.get("G2.191_stage_a_identity") == Some(&json!(true))
            && ident_ok;
        let g201: Vec<(String, bool)> = RULES
            .iter()
            .map(|x| (short(x).to_string(), gates && ahead(x)))
            .collect();
        let f_rho = |g: usize| -> (f64, f64) {
            let v: Vec<f64> = rows
                .iter()
                .map(|r| r["chain-fifo"]["over_rho"][g].as_f64().unwrap())
                .collect();
            range(&v)
        };
        let (a0, b0) = f_rho(0);
        let (a1, b1) = f_rho(2);
        let g203 = !any_below && a0 >= 2e3 && b0 <= 4e5 && a1 >= 5e4 && b1 <= 1e7;
        verdict.insert("G2.197_bands".into(), json!(g197_bands));
        verdict.insert("G2.197_order_ahead_of_A_and_E25".into(), json!(g197_order));
        verdict.insert("G2.198_health".into(), json!(g198));
        verdict.insert("G2.199_b2_unresolved".into(), json!(g199));
        verdict.insert("G2.200_l_arm".into(), json!(g200));
        verdict.insert("G2.201_rank_with_gates".into(), json!(g201));
        verdict.insert("G2.203_rho".into(), json!(g203));
        writeln!(md, "\nE25/F at 0 GAE: `{e0:.3}`–`{e1:.3}`; F/rho at 0 GAE `{}`–`{}`, at 1 GAE `{}`–`{}`; L under σ2 ahead of A at 1 GAE on every row: {l_sigma2_ahead_1gae}.",
            sci(a0), sci(b0), sci(a1), sci(b1)).unwrap();
        writeln!(md, "\nEnd-to-end identity (F, S, H, B″, L records equal A's on the same row and stream apart from the solve's multiplications, selection, lead scans, wall times, tag): **{ident_ok}**").unwrap();
        summary.insert("e2e".into(), json!(rows));
        summary.insert("ranks".into(), json!(ranks));
        summary.insert("rho_ranges".into(), json!(rho_ranges));
        summary.insert("aa".into(), json!(aa));
        summary.insert("e2e_identity".into(), json!(identity));
        summary.insert("e2e_identity_ok".into(), json!(ident_ok));
        summary.insert("health".into(), json!(health));
    }
    writeln!(md, "\n## Automatic checks against the registered numbers (the verdicts are written in §9.49)\n").unwrap();
    for (k, v) in &verdict {
        writeln!(md, "- `{k}`: {v}").unwrap();
    }
    fs::write(format!("{dir}/tables.md"), &md).unwrap();
    summary.insert("experiment".into(), json!("e39"));
    summary.insert("gae".into(), json!(GAE));
    summary.insert("verdict_checks".into(), json!(verdict));
    fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_string_pretty(&Value::Object(summary)).unwrap(),
    )
    .unwrap();
    print!("{md}");
}
