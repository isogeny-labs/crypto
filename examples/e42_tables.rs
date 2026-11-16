//! **E42 tables** (`RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` §9.54–§9.55):
//! Stage 0, controls, Stage A and end to end for W (`chain-tails`) against V
//! (`chain-profile`, E40's records on stream `E37E0001`), A (`chain-counted`,
//! E37's records), E25 (`macaulay`, E37's records) and the matched rho (walk
//! (iii)), at `0`, `0.1` and `1` GAE, with σ1 (lead scans priced, every arm)
//! and σ4 (per-call structural work priced, every arm, E40's Stage 0
//! counts); the negative control V′/W′ on stream `E42E0002`.  Reads
//! recorded JSON only; writes `tables.md`, `summary.json` (the values the
//! report plots) and the verdicts of E42b (§9.56, G2.244–G2.257; E42's
//! G2.230–G2.243 stopped at its controls kill, §9.55, recorded here from
//! `checks.json`).  E42b's controls are `checks_b.json`.
//!
//! ```text
//! ./e42_tables experiments/49_macaulay_tails
//! ```

#![recursion_limit = "256"]

use std::env;
use std::fmt::Write as _;
use std::fs;

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
const E37: &str = "experiments/44_pivot_rules_confirm/e2e";
const E40: &str = "experiments/47_macaulay_entries";
const WEAK: &str = "equal at 0, ahead at 0.1 and 1";

fn load(path: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}")))
        .unwrap()
}

fn f(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap_or_else(|| panic!("field {k}"))
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

/// Fields a W record may change against V's on the same row and stream
/// (and, with `profile_offset_ops`, against A's).
fn may_differ(k: &str, vs_a: bool) -> bool {
    k.ends_with("_ms")
        || k == "ms"
        || k.contains("wall")
        || matches!(
            k,
            "solver" | "lead_scan_reads" | "macaulay_stored_entries" | "tail_extent_ops"
        )
        || (vs_a && k == "profile_offset_ops")
}

fn strip(v: Option<&Value>, vs_a: bool) -> Option<Value> {
    fn go(v: &Value, vs_a: bool) -> Value {
        match v {
            Value::Object(m) => Value::Object(
                m.iter()
                    .filter(|(k, _)| !may_differ(k, vs_a))
                    .map(|(k, x)| (k.clone(), go(x, vs_a)))
                    .collect(),
            ),
            Value::Array(a) => Value::Array(a.iter().map(|x| go(x, vs_a)).collect()),
            x => x.clone(),
        }
    }
    v.map(|x| go(x, vs_a))
}

fn diff_fields(a: &Value, b: &Value, vs_a: bool) -> Vec<String> {
    let (ao, bo) = (a.as_object().unwrap(), b.as_object().unwrap());
    let mut keys: Vec<&String> = ao.keys().chain(bo.keys()).collect();
    keys.sort();
    keys.dedup();
    keys.into_iter()
        .filter(|k| !may_differ(k, vs_a) && strip(ao.get(*k), vs_a) != strip(bo.get(*k), vs_a))
        .cloned()
        .collect()
}

/// `S(g)` of a record: `total_muls + 97 g (cells + extra)`; cells are the
/// stored cells plus offset and tail operations for V and W,
/// `macaulay_entries` (+ any selection) otherwise.
fn s_of(x: &Value, extra: f64) -> Vec<f64> {
    let cells = match x.get("macaulay_stored_entries").and_then(Value::as_f64) {
        Some(st) => st + f(x, "profile_offset_ops") + x["tail_extent_ops"].as_f64().unwrap_or(0.0),
        None => f(x, "macaulay_entries") + x["pivot_selection_ops"].as_f64().unwrap_or(0.0),
    };
    GS.iter()
        .map(|g| f(x, "total_muls") + GAE * g * (cells + extra))
        .collect()
}

/// Order of arm `x` against reference `a` (`ahead`: x cheaper).
fn order(a: &[f64], x: &[f64]) -> &'static str {
    let ahead = (0..3).all(|g| x[g] < a[g]);
    let behind = (0..3).all(|g| x[g] > a[g]);
    let weak = x[0] == a[0] && x[1] < a[1] && x[2] < a[2];
    let weak_behind = x[0] == a[0] && x[1] > a[1] && x[2] > a[2];
    if ahead {
        "ahead"
    } else if weak {
        WEAK
    } else if behind {
        "behind"
    } else if weak_behind {
        "equal at 0, behind at 0.1 and 1"
    } else {
        "unresolved"
    }
}

fn rat(r: &[f64], x: &[f64]) -> Vec<f64> {
    (0..3).map(|g| r[g] / x[g]).collect()
}

fn fmt3(b: &[(f64, f64)]) -> String {
    format!(
        "`{:.4}`–`{:.4}` / `{:.4}`–`{:.4}` / `{:.4}`–`{:.4}`",
        b[0].0, b[0].1, b[1].0, b[1].1, b[2].0, b[2].1
    )
}

fn main() {
    let dir = env::args()
        .nth(1)
        .unwrap_or("experiments/49_macaulay_tails".into());
    let mut md = String::new();
    let mut verdict = Map::new();
    writeln!(md, "# E42 / E42b tables (§9.55, §9.57)\n\nGenerated by `examples/e42_tables.rs` from recorded JSON. `S(g) = total_muls + 97 g · cells`; cells = `macaulay_entries` for A and E25, `macaulay_stored_entries + profile_offset_ops` for V, `macaulay_stored_entries + profile_offset_ops + tail_extent_ops` for W; rho(g) = rho + 97 g · steps (walk (iii)).\n").unwrap();

    // ── Stage 0 (v2) ──
    let s0 = load(&format!("{dir}/stage0_v2.json"));
    let s0a = s0.as_array().unwrap();
    writeln!(md, "## Stage 0 (per call, range of the 22 curve means; 200 calls each)\n\n| count | min | max |\n|:--|--:|--:|").unwrap();
    for k in [
        "tail_cells",
        "build_extent_cells",
        "final_extent_cells",
        "ever_extent_cells",
        "ever_nonzero_cells",
        "final_nonzero_cells",
        "touched_v",
        "touched_g",
        "updates",
        "dense_updates",
        "growth_events",
        "growth_cells",
        "growth_copy_keep_lead",
        "sparse_merge_lower_bound",
        "support_terms",
        "overflow_copy_cells",
        "overflow_split_updates",
        "outside_dependency_final_cells",
        "dense_update_cells_v",
        "dense_update_cells_g",
    ] {
        let (lo, hi) = range(
            &s0a.iter()
                .map(|c| f(&c["fields"][k], "mean"))
                .collect::<Vec<_>>(),
        );
        writeln!(md, "| `{k}` | {lo:.1} | {hi:.1} |").unwrap();
    }
    for (k, name) in [
        ("reserve_cells_mean", "reserve cells"),
        ("reserve_reallocs_mean", "re-allocations"),
        ("reserve_copy_cells_mean", "copied cells"),
    ] {
        for i in 0..3 {
            let (lo, hi) = range(
                &s0a.iter()
                    .map(|c| c[k][i].as_f64().unwrap())
                    .collect::<Vec<_>>(),
            );
            writeln!(md, "| {name}, k = {i} | {lo:.1} | {hi:.1} |").unwrap();
        }
    }
    let s0_ok = s0a.iter().all(|c| c["echelon_reproduced_all"] == true);
    // σ4: E40's per-call structural work (common to every arm).
    let e40s0 = load(&format!("{E40}/stage0.json"));
    let struct_of = |set: &str, p: u64, seed: u64| -> f64 {
        let c = e40s0
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["set"] == set && c["p"] == p && c["seed"] == seed)
            .unwrap();
        [
            "enumerated_monomials",
            "cols",
            "selection_tests",
            "term_writes",
        ]
        .iter()
        .map(|k| f(&c["fields"][k], "mean"))
        .sum()
    };

    // ── Existing configurations ──
    let ide = load(&format!("{dir}/identity_existing_solvers.json"));
    let ide_ok = ide["runs"]
        .as_array()
        .unwrap()
        .iter()
        .all(|r| r["identical"] == true);
    writeln!(md, "\n## Existing configurations under the E42 binary\n\n| config | reference | identical |\n|:--|:--|:--|").unwrap();
    for r in ide["runs"].as_array().unwrap() {
        writeln!(
            md,
            "| `{}` | {} | {} |",
            r["config"].as_str().unwrap(),
            r["reference"].as_str().unwrap(),
            r["identical"]
        )
        .unwrap();
    }

    // ── Controls ──
    let ck42 = load(&format!("{dir}/checks.json"));
    writeln!(md, "\n## E42's controls (§9.55, stream `0xE42C_0001`)\n\nG2.231 as registered: **{}** (G0 re-allocations {:.1} per call against `[900, 960]`); every per-call check: **{}**. E42 stopped here.\n",
        ck42["g2_231_holds"], f(&ck42, "g0_reallocs_per_call"), ck42["g2_245_holds"].as_bool().unwrap_or(
            ck42["w_equals_a_counted"] == ck42["calls"] && ck42["p1_detected"] == ck42["calls"] && ck42["p2_detected"] == ck42["calls"]
            && ck42["g0_equals_a_counted"] == ck42["calls"] && ck42["g0_counter_ok"] == ck42["calls"]
            && ck42["w_counter_ok"] == ck42["calls"] && ck42["perturbed_changed"] == ck42["nonempty"])).unwrap();
    let ck = load(&format!("{dir}/checks_b.json"));
    let ck_ok = ck["g2_245_holds"] == true;
    let g0_desc = ck["g2_246_holds"] == true;
    writeln!(md, "\n## E42b's controls (G2.245–G2.246, stream `0xE42C_0002`)\n\n{} calls: W = A counted on {}; W counter ok {} (re-allocations {}); P1 detected {}; P2 detected {}; G0 = A counted {}, counter ok {}, re-allocations {:.1} per call; perturbed coefficient changed {}/{} non-empty sets.\n",
        ck["calls"], ck["w_equals_a_counted"], ck["w_counter_ok"], ck["w_reallocs"], ck["p1_detected"], ck["p2_detected"],
        ck["g0_equals_a_counted"], ck["g0_counter_ok"], f(&ck, "g0_reallocs_per_call"), ck["perturbed_changed"], ck["nonempty"]).unwrap();

    // ── Stage A ──
    let sa = load(&format!("{dir}/stage_a.json"));
    let mut sa_ok = true;
    let mut lead_ok = true;
    let mut sa_rows = Vec::new();
    writeln!(md, "## Stage A (10 curves × 1,000 calls, stream `seed ^ 0xE42A_0001`)\n\n| p | seed | agree (ex, A, V, W) | W = A | V = A | W ops = V | counter | lead W ≤ V | W cells clean [min, max] | V cells clean | reallocs/call | W tail ops | W ops | fb A/V/W | slack | S_V/S_W 0 | 0.1 | 1 | S_A/S_W 0.1 | 1 | σ1 V/W 0.1 | 1 |\n|--:|--:|:--|--:|--:|--:|--:|--:|:--|--:|--:|--:|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    for c in sa.as_array().unwrap() {
        let calls = c["calls"].as_u64().unwrap();
        let agree: Vec<u64> = c["agree"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap())
            .collect();
        let ok = agree.iter().all(|&a| a == calls)
            && c["w_equals_a_counted"].as_u64() == Some(calls)
            && c["v_equals_a_counted"].as_u64() == Some(calls)
            && c["w_offsets_equal_v"].as_u64() == Some(calls)
            && c["stored_counter_ok"].as_u64() == Some(calls);
        sa_ok &= ok;
        lead_ok &= c["lead_scan_le_v"].as_u64() == Some(calls);
        let s = &c["summaries"];
        let (a, v, w) = (&s[1], &s[2], &s[3]);
        let r = &c["ratio_v_over_w"];
        let ra = &c["ratio_a_over_w"];
        let r1 = &c["ratio_v_over_w_sigma1"];
        let g = |x: &Value, i: usize| x[i].as_f64().unwrap();
        let reallocs = f(c, "w_reallocs") / calls as f64;
        writeln!(md, "| {} | {} | {:?} | {} | {} | {} | {} | {} | {:.0} [{}, {}] | {:.0} | {:.2} | {:.0} | {:.0} | {} / {} / {} | {:.0} | {:.6} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} |",
            c["p"], c["seed"], agree, c["w_equals_a_counted"], c["v_equals_a_counted"], c["w_offsets_equal_v"],
            c["stored_counter_ok"], c["lead_scan_le_v"], f(w, "cells_clean_mean"), w["cells_clean_min"], w["cells_clean_max"],
            f(v, "cells_clean_mean"), reallocs, f(w, "tail_ops_mean"), f(w, "offset_ops_mean"),
            a["fallbacks"], v["fallbacks"], w["fallbacks"], f(c, "w_slack_clean_mean"),
            g(r, 0), g(r, 1), g(r, 2), g(ra, 1), g(ra, 2), g(r1, 1), g(r1, 2)).unwrap();
        sa_rows.push(json!({"p": c["p"], "seed": c["seed"], "identity_ok": ok,
            "w_cells_clean": f(w, "cells_clean_mean"), "v_cells_clean": f(v, "cells_clean_mean"),
            "w_cells_clean_min": w["cells_clean_min"], "w_cells_clean_max": w["cells_clean_max"],
            "reallocs_per_call": reallocs, "w_tail_ops": f(w, "tail_ops_mean"), "w_offset_ops": f(w, "offset_ops_mean"),
            "v_offset_ops": f(v, "offset_ops_mean"), "w_lead": f(w, "lead_scan_mean"), "v_lead": f(v, "lead_scan_mean"),
            "fill_extent": f(c, "fill_extent_clean_mean"), "slack": f(c, "w_slack_clean_mean"),
            "axpy_v": f(c, "axpy_cells_v_mean"), "axpy_g": f(c, "axpy_cells_g_mean"),
            "c_mean": f(w, "call_mean"), "s_v": v["s_mean"], "s_w": w["s_mean"], "s_a": a["s_mean"],
            "v_over_w": r, "a_over_w": ra, "a_over_v": c["ratio_a_over_v"], "v_over_w_sigma1": r1,
            "fb_a": a["fallbacks"], "fb_v": v["fallbacks"], "fb_w": w["fallbacks"]}));
    }
    let col = |k: &str, i: Option<usize>| -> (f64, f64) {
        range(
            &sa_rows
                .iter()
                .map(|r| match i {
                    Some(i) => r[k][i].as_f64().unwrap(),
                    None => f(r, k),
                })
                .collect::<Vec<_>>(),
        )
    };
    let (wc0, wc1) = col("w_cells_clean", None);
    let (rl0, rl1) = col("reallocs_per_call", None);
    let (to0, to1) = col("w_tail_ops", None);
    let (r00, r01) = col("v_over_w", Some(0));
    let (r10, r11) = col("v_over_w", Some(1));
    let (r20, r21) = col("v_over_w", Some(2));
    let (sl0, sl1) = col("slack", None);
    let (axv0, axv1) = col("axpy_v", None);
    let (axg0, axg1) = col("axpy_g", None);
    let sa_fb_same = sa_rows
        .iter()
        .all(|r| r["fb_a"] == r["fb_w"] && r["fb_v"] == r["fb_w"]);
    let sa_behind = sa_rows
        .iter()
        .any(|r| (0..3).any(|g| r["s_w"][g].as_f64().unwrap() > r["s_v"][g].as_f64().unwrap()));
    writeln!(md, "\nRanges over curves: W clean cells `{wc0:.0}`–`{wc1:.0}`, re-allocations per call `{rl0:.2}`–`{rl1:.2}`, tail ops `{to0:.0}`–`{to1:.0}`, slack over the fill extent `{sl0:.0}`–`{sl1:.0}`; S_V/S_W `{r00:.6}`–`{r01:.6}` / `{r10:.4}`–`{r11:.4}` / `{r20:.4}`–`{r21:.4}`; axpy cells (Stage 0 replay) V `{axv0:.0}`–`{axv1:.0}`, G `{axg0:.0}`–`{axg1:.0}`.").unwrap();

    // ── End to end ──
    let s27 = load("experiments/32_walk_karatsuba/summary.json");
    let w24 = load("experiments/44_pivot_rules_confirm/walk_e24.json");
    let mut rows: Vec<Value> = Vec::new();
    let mut identity = Vec::new();
    let mut health_ok = true;
    for (set, curves) in [("gls", &GLS[..]), ("e24", &E24[..])] {
        for &(p, seed) in curves {
            let ra = load(&format!("{E37}/{set}_chain-counted_p{p}s{seed}.json"));
            let re = load(&format!("{E37}/{set}_macaulay_p{p}s{seed}.json"));
            let rv = load(&format!("{E40}/e2e/{set}_chain-profile_p{p}s{seed}.json"));
            let rw = load(&format!("{dir}/e2e/{set}_chain-tails_p{p}s{seed}.json"));
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
                let (xa, xe, xv, xw) = (
                    arm_of(&ra, a),
                    arm_of(&re, a),
                    arm_of(&rv, a),
                    arm_of(&rw, a),
                );
                let calls = f(&xa, "residuals");
                let st = struct_of(set, p, seed) * calls;
                let (la, lv, lw) = (
                    f(&xa, "lead_scan_reads"),
                    f(&xv, "lead_scan_reads"),
                    f(&xw, "lead_scan_reads"),
                );
                let (sa_, se, sv, sw) = (
                    s_of(&xa, 0.0),
                    s_of(&xe, 0.0),
                    s_of(&xv, 0.0),
                    s_of(&xw, 0.0),
                );
                let (sa1, se1, sv1, sw1) =
                    (s_of(&xa, la), s_of(&xe, la), s_of(&xv, lv), s_of(&xw, lw));
                let (sa4, se4, sv4, sw4) =
                    (s_of(&xa, st), s_of(&xe, st), s_of(&xv, st), s_of(&xw, st));
                let dv = diff_fields(&xv, &xw, false);
                let da = diff_fields(&xa, &xw, true);
                identity.push(
                    json!({"set": set, "p": p, "seed": seed, "arm": a.unwrap_or("—"),
                    "fields_differing_from_v": dv, "fields_differing_from_a": da}),
                );
                for x in [&xa, &xe, &xv, &xw] {
                    health_ok &= x["folded"]["correct"] == true
                        && x["rho"]
                            .as_array()
                            .is_some_and(|w| w.iter().all(|w| w["correct"] == true));
                }
                rows.push(json!({"set": set, "p": p, "seed": seed, "arm": a.unwrap_or("—"),
                    "calls": calls, "fallbacks_w": xw["solver_fallbacks"].clone(),
                    "fallbacks_v": xv["solver_fallbacks"].clone(), "fallbacks_a": xa["solver_fallbacks"].clone(),
                    "rho": rho, "walk_steps": steps, "rho_g": rho_g,
                    "S_A": sa_, "S_E25": se, "S_V": sv, "S_W": sw,
                    "v_over_w": rat(&sv, &sw), "a_over_w": rat(&sa_, &sw), "e25_over_w": rat(&se, &sw),
                    "a_over_v": rat(&sa_, &sv), "e25_over_v": rat(&se, &sv),
                    "v_over_w_sigma1": rat(&sv1, &sw1), "a_over_w_sigma1": rat(&sa1, &sw1), "e25_over_w_sigma1": rat(&se1, &sw1),
                    "v_over_w_sigma4": rat(&sv4, &sw4), "a_over_w_sigma4": rat(&sa4, &sw4), "e25_over_w_sigma4": rat(&se4, &sw4),
                    "w_over_rho": rat(&sw, &rho_g), "v_over_rho": rat(&sv, &rho_g), "a_over_rho": rat(&sa_, &rho_g),
                    "e25_over_rho": rat(&se, &rho_g),
                    "order_w_vs_v": order(&sv, &sw), "order_w_vs_a": order(&sa_, &sw), "order_w_vs_e25": order(&se, &sw),
                    "order_w_vs_v_sigma1": order(&sv1, &sw1), "order_w_vs_a_sigma1": order(&sa1, &sw1), "order_w_vs_e25_sigma1": order(&se1, &sw1),
                    "order_w_vs_v_sigma4": order(&sv4, &sw4), "order_w_vs_a_sigma4": order(&sa4, &sw4), "order_w_vs_e25_sigma4": order(&se4, &sw4),
                    "w_cells_per_call": f(&xw, "macaulay_stored_entries") / calls,
                    "v_cells_per_call": f(&xv, "macaulay_stored_entries") / calls,
                    "w_offset_ops_per_call": f(&xw, "profile_offset_ops") / calls,
                    "w_tail_ops_per_call": f(&xw, "tail_extent_ops") / calls,
                    "w_lead_per_call": lw / calls, "v_lead_per_call": lv / calls}));
            }
        }
    }
    let ident_ok = identity.iter().all(|i| {
        i["fields_differing_from_v"].as_array().unwrap().is_empty()
            && i["fields_differing_from_a"].as_array().unwrap().is_empty()
    });
    let rr = |rows: &[Value], k: &str, g: usize| -> (f64, f64) {
        range(
            &rows
                .iter()
                .map(|r| r[k][g].as_f64().unwrap())
                .collect::<Vec<_>>(),
        )
    };
    let band =
        |rows: &[Value], k: &str| -> Vec<(f64, f64)> { (0..3).map(|g| rr(rows, k, g)).collect() };
    let all_order = |k: &str, want: &str| rows.iter().all(|r| r[k] == want);
    writeln!(md, "\n## End to end (32 rows, E37's stream `E37E0001`, Karatsuba)\n\nIdentity (W record = E40's V record outside tag, timing, lead scans, stored cells and tail ops; = E37's A record outside those and offset ops): **{ident_ok}**; every log recovered and every walk correct (A, E25, V, W): **{health_ok}**.\n").unwrap();
    writeln!(md, "| set | p | seed | arm | calls | fb W/V | W cells/call | W tail ops/call | V/W 0 | 0.1 | 1 | A/W 0.1 | 1 | E25/W 0 | 0.1 | 1 | W/rho 0 | 0.1 | 1 | V/rho 1 |\n|:--|--:|--:|:--|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|").unwrap();
    for r in &rows {
        let g = |k: &str, i: usize| r[k][i].as_f64().unwrap();
        writeln!(md, "| {} | {} | {} | {} | {} | {}/{} | {:.0} | {:.0} | {:.6} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {} | {} | {} | {} |",
            r["set"].as_str().unwrap(), r["p"], r["seed"], r["arm"].as_str().unwrap(), r["calls"],
            r["fallbacks_w"], r["fallbacks_v"], f(r, "w_cells_per_call"), f(r, "w_tail_ops_per_call"),
            g("v_over_w", 0), g("v_over_w", 1), g("v_over_w", 2), g("a_over_w", 1), g("a_over_w", 2),
            g("e25_over_w", 0), g("e25_over_w", 1), g("e25_over_w", 2),
            sci(g("w_over_rho", 0)), sci(g("w_over_rho", 1)), sci(g("w_over_rho", 2)), sci(g("v_over_rho", 2))).unwrap();
    }
    let b_vw = band(&rows, "v_over_w");
    let b_aw = band(&rows, "a_over_w");
    let b_ew = band(&rows, "e25_over_w");
    let b_vw1 = band(&rows, "v_over_w_sigma1");
    let b_vw4 = band(&rows, "v_over_w_sigma4");
    let b_aw1 = band(&rows, "a_over_w_sigma1");
    let b_aw4 = band(&rows, "a_over_w_sigma4");
    let b_ew1 = band(&rows, "e25_over_w_sigma1");
    let b_ew4 = band(&rows, "e25_over_w_sigma4");
    let b_wr = band(&rows, "w_over_rho");
    let b_vr = band(&rows, "v_over_rho");
    let w_vs_v = all_order("order_w_vs_v", WEAK);
    let w_vs_a = all_order("order_w_vs_a", WEAK);
    let w_vs_e25 = all_order("order_w_vs_e25", "ahead");
    let s1 = (all_order("order_w_vs_v_sigma1", WEAK) || all_order("order_w_vs_v_sigma1", "ahead"))
        && (all_order("order_w_vs_a_sigma1", WEAK) || all_order("order_w_vs_a_sigma1", "ahead"))
        && all_order("order_w_vs_e25_sigma1", "ahead");
    let s4 = all_order("order_w_vs_v_sigma4", WEAK)
        && all_order("order_w_vs_a_sigma4", WEAK)
        && all_order("order_w_vs_e25_sigma4", "ahead");
    let below_rho = rows
        .iter()
        .filter(|r| (0..3).any(|g| r["w_over_rho"][g].as_f64().unwrap() < 1.0))
        .count();
    let e25_at0_equal = rows
        .iter()
        .all(|r| r["e25_over_w"][0] == r["e25_over_v"][0] && r["S_W"][0] == r["S_V"][0]);
    let fb_e2e_same = rows
        .iter()
        .all(|r| r["fallbacks_w"] == r["fallbacks_v"] && r["fallbacks_w"] == r["fallbacks_a"]);
    writeln!(md, "\n### Band summary (0 / 0.1 / 1 GAE)\n\n| quantity | 0 / 0.1 / 1 GAE | order on all 32 rows |\n|:--|:--|:--|").unwrap();
    for (name, b, ok, txt) in [
        (
            "`S_V / S_W`",
            &b_vw,
            w_vs_v,
            "W equal at 0, ahead at 0.1 and 1",
        ),
        (
            "`S_A / S_W`",
            &b_aw,
            w_vs_a,
            "W equal at 0, ahead at 0.1 and 1",
        ),
        ("`S_E25 / S_W`", &b_ew, w_vs_e25, "W ahead at every weight"),
        ("σ1 `S_V / S_W`", &b_vw1, s1, "same order (σ1, every arm)"),
        ("σ1 `S_A / S_W`", &b_aw1, s1, "same order (σ1, every arm)"),
        ("σ1 `S_E25 / S_W`", &b_ew1, s1, "same order (σ1, every arm)"),
        ("σ4 `S_V / S_W`", &b_vw4, s4, "same order (σ4, every arm)"),
        ("σ4 `S_A / S_W`", &b_aw4, s4, "same order (σ4, every arm)"),
        ("σ4 `S_E25 / S_W`", &b_ew4, s4, "same order (σ4, every arm)"),
    ] {
        writeln!(
            md,
            "| {name} | {} | {} |",
            fmt3(b),
            if ok { txt } else { "NOT as registered" }
        )
        .unwrap();
    }
    for (name, b) in [("`S_W / rho`", &b_wr), ("`S_V / rho`", &b_vr)] {
        writeln!(
            md,
            "| {name} | `{}`–`{}` / `{}`–`{}` / `{}`–`{}` | rows with W below rho at any weight: {below_rho} |",
            sci(b[0].0),
            sci(b[0].1),
            sci(b[1].0),
            sci(b[1].1),
            sci(b[2].0),
            sci(b[2].1)
        )
        .unwrap();
    }

    // ── Negative control: V′ and W′ on E42E0002 (GLS) ──
    let mut neg = Vec::new();
    for &(p, seed) in &GLS {
        let rv = load(&format!("{E40}/e2e/gls_chain-profile_p{p}s{seed}.json"));
        let rv2 = load(&format!(
            "{dir}/e2e/gls_chain-profile_E42E0002_p{p}s{seed}.json"
        ));
        let rw2 = load(&format!(
            "{dir}/e2e/gls_chain-tails_E42E0002_p{p}s{seed}.json"
        ));
        for a in ["A", "B"] {
            let (xv, xv2, xw2) = (
                arm_of(&rv, Some(a)),
                arm_of(&rv2, Some(a)),
                arm_of(&rw2, Some(a)),
            );
            let (sv, sv2, sw2) = (s_of(&xv, 0.0), s_of(&xv2, 0.0), s_of(&xw2, 0.0));
            neg.push(json!({"p": p, "seed": seed, "arm": a,
                "order_v2_vs_v": order(&sv, &sv2), "v_over_v2": rat(&sv, &sv2),
                "v2_over_w2": rat(&sv2, &sw2), "order_w2_vs_v2": order(&sv2, &sw2),
                "w2_identity_fields": diff_fields(&xv2, &xw2, false)}));
        }
    }
    let v2_orders: Vec<&str> = neg
        .iter()
        .map(|r| r["order_v2_vs_v"].as_str().unwrap())
        .collect();
    let v2_ranked = v2_orders.iter().all(|&o| o == "ahead" || o == WEAK)
        || v2_orders.iter().all(|&o| o == "behind");
    let w2_band = band(&neg, "v2_over_w2");
    let vv_band = band(&neg, "v_over_v2");
    let w2_ident = neg
        .iter()
        .all(|r| r["w2_identity_fields"].as_array().unwrap().is_empty());
    let w2_order = neg.iter().all(|r| r["order_w2_vs_v2"] == WEAK);
    writeln!(md, "\n## Negative control (stream `E42E0002`, 20 GLS rows)\n\nV′ against V (unpaired across streams): orders {v2_orders:?}; V′ ranked against V: **{v2_ranked}**; `S_V / S_V′` {}.\nW′ against V′ (paired): {}; order as registered on all rows: **{w2_order}**; W′ records = V′'s outside the allowed fields: **{w2_ident}**.\n",
        fmt3(&vv_band), fmt3(&w2_band)).unwrap();

    // ── Verdicts ──
    let inside = |b: (f64, f64), lo: f64, hi: f64| b.0 >= lo && b.1 <= hi;
    let g230 = sa_ok && ident_ok && ide_ok;
    let g232 = wc0 >= 9.25e4 && wc1 <= 9.35e4 && rl0 >= 4.0 && rl1 <= 10.0;
    let g233 = to0 >= 5.3e3 && to1 <= 5.7e3 && sa_ok;
    let g234_kill = r21 <= 1.005 || r20 <= 1.005 || sa_behind;
    let g234 = r00 == 1.0
        && r01 == 1.0
        && inside((r10, r11), 1.015, 1.035)
        && inside((r20, r21), 1.02, 1.05)
        && !g234_kill;
    let g235 = lead_ok;
    let g236 = w_vs_v
        && b_vw[0].0 == 1.0
        && b_vw[0].1 == 1.0
        && inside(b_vw[1], 1.015, 1.035)
        && inside(b_vw[2], 1.02, 1.05);
    let g237 = w_vs_a
        && w_vs_e25
        && e25_at0_equal
        && inside(b_aw[1], 1.27, 1.31)
        && inside(b_aw[2], 1.38, 1.45)
        && inside(b_ew[1], 1.77, 1.87)
        && inside(b_ew[2], 1.43, 1.53);
    let g238 = below_rho == 0 && inside(b_vw[2], 1.02, 1.05);
    let g239 = s1 && s4;
    let g240 = !v2_ranked
        && w2_order
        && w2_ident
        && inside(w2_band[1], 1.015, 1.035)
        && inside(w2_band[2], 1.02, 1.05);
    let g241 = sa_fb_same && fb_e2e_same;
    let g242 = sl0 >= 9.0e3 && sl1 <= 11.0e3;
    let g243 = axg0 >= 5.9e5 && axg1 <= 6.3e5 && axv0 >= 7.3e5 && axv1 <= 7.7e5;
    for (k, v) in [
        ("G2.244_identity", g230),
        ("G2.245_controls", ck_ok),
        ("G2.246_g0_reallocs_descriptive", g0_desc),
        ("G2.247_stored_cells", g232),
        ("G2.248_tail_ops", g233),
        ("G2.249_stage_a_band", g234),
        ("G2.249_kill_fired", g234_kill),
        ("G2.250_lead_scans", g235),
        ("G2.251_e2e_vs_v", g236),
        ("G2.252_e2e_vs_a_e25", g237),
        ("G2.253_rho", g238),
        ("G2.254_sensitivities", g239),
        ("G2.255_negative", g240),
        ("G2.256_fallbacks", g241),
        ("G2.257_slack", g242),
        ("G2.257_axpy_cells", g243),
        ("stage0_echelon_reproduced", s0_ok),
        ("stage_a_identity", sa_ok),
        ("e2e_identity", ident_ok),
        ("existing_configs_identical", ide_ok),
        ("e2e_logs_and_walks", health_ok),
    ] {
        verdict.insert(k.into(), json!(v));
    }
    let gate = g230 && ck_ok && !g234_kill;
    verdict.insert(
        "W_ranked_vs_V_registered_rule".into(),
        json!(gate && w_vs_v),
    );
    verdict.insert(
        "W_ranked_vs_A_registered_rule".into(),
        json!(gate && w_vs_a),
    );
    verdict.insert("W_ranked_vs_E25".into(), json!(gate && w_vs_e25));
    writeln!(
        md,
        "## Verdicts\n\n```\n{}\n```",
        serde_json::to_string_pretty(&verdict).unwrap()
    )
    .unwrap();
    let summary = json!({
        "verdict": verdict, "stage_a": sa_rows, "rows": rows, "negative": neg,
        "e2e_identity": identity,
        "bands": {"v_over_w": b_vw, "a_over_w": b_aw, "e25_over_w": b_ew,
                  "v_over_w_sigma1": b_vw1, "a_over_w_sigma1": b_aw1, "e25_over_w_sigma1": b_ew1,
                  "v_over_w_sigma4": b_vw4, "a_over_w_sigma4": b_aw4, "e25_over_w_sigma4": b_ew4,
                  "w_over_rho": b_wr, "v_over_rho": b_vr,
                  "negative_v_over_v2": vv_band, "negative_v2_over_w2": w2_band},
    });
    fs::write(format!("{dir}/tables.md"), md).unwrap();
    fs::write(
        format!("{dir}/summary.json"),
        serde_json::to_string_pretty(&summary).unwrap(),
    )
    .unwrap();
    eprintln!("{}", serde_json::to_string_pretty(&verdict).unwrap());
}
