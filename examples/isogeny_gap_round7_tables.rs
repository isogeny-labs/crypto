//! # Tables for research/isogeny_conductor_gap_round7_20261019 (E10, E11a, E12)
//!
//! - **E10** reads `e10/records.jsonl` (`f4-f2-early` at free ∈ {0, 2, 4})
//!   and round 6's `e8/records.jsonl` (full F4 and free = 8).  Per
//!   configuration, the saving is `1 − Σ early / Σ full`, where work is
//!   word XORs plus monomial tests.  Each early run's solution count must
//!   equal E8's full-run count (the gate).
//! - **E11a** reads `e11a/records.jsonl`: default base at d against the
//!   half base (trace coset at d − 1).  It reports the maximum solving
//!   degree, lifted decompositions, and estimated F4 work for a relation
//!   set: columns × Σ work ÷ Σ lifted.
//! - **E12** reads E8 only.  The hit rate is the fraction of targets with
//!   ≥ 1 lifted decomposition.  It is compared, coset ÷ default, with the
//!   decomposition-count ratio (Poisson intervals).
//!
//! ```bash
//! cargo run --release --example isogeny_gap_round7_tables -- \
//!   --dir research/isogeny_conductor_gap_round7_20261019 \
//!   --e8 research/isogeny_conductor_gap_round6_20261018/e8/records.jsonl \
//!   --out research/isogeny_conductor_gap_round7_20261019/TABLES.md \
//!   --json research/isogeny_conductor_gap_round7_20261019/tables.json
//! ```

use std::collections::BTreeMap;
use std::fmt::Write as _;

use serde_json::{json, Value};

fn arg(args: &[String], name: &str) -> String {
    let i = args
        .iter()
        .position(|a| a == name)
        .unwrap_or_else(|| panic!("missing {name}"));
    args[i + 1].clone()
}

fn lines(path: &str) -> Vec<Value> {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{path}: {e}"))
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

/// System records keyed (config, target), last copy winning, restricted to
/// finished configurations; and the `config_done` lines.
fn systems(recs: &[Value]) -> (BTreeMap<String, Vec<Value>>, BTreeMap<String, Value>) {
    let mut by: BTreeMap<(String, u64), Value> = BTreeMap::new();
    let mut cfgs = BTreeMap::new();
    for r in recs {
        let c = r["config"].as_str().unwrap().to_string();
        if r["kind"] == "config_done" {
            cfgs.insert(c, r.clone());
        } else {
            by.insert((c, r["target"].as_u64().unwrap()), r.clone());
        }
    }
    let mut out: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for ((c, _), r) in by {
        if cfgs.contains_key(&c) {
            out.entry(c).or_default().push(r);
        }
    }
    (out, cfgs)
}

fn u(v: &Value, k: &str) -> f64 {
    v[k].as_u64().unwrap() as f64
}

fn poisson(a: f64, b: f64) -> (f64, f64, f64) {
    if a == 0.0 || b == 0.0 {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    let r = a / b;
    let se = (1.0 / a + 1.0 / b).sqrt();
    (r, r * (-1.96 * se).exp(), r * (1.96 * se).exp())
}

fn f(x: f64) -> String {
    if x.is_nan() {
        "–".into()
    } else {
        format!("{x:.2}")
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dir = arg(&args, "--dir");
    let (e8, e8cfg) = systems(&lines(&arg(&args, "--e8")));
    let mut md = String::from("# Round 7 tables (E10, E11a, E12)\n");
    let mut js = serde_json::Map::new();

    // ── E10 ──
    if let Ok(text) = std::fs::read_to_string(format!("{dir}/e10/records.jsonl")) {
        let recs: Vec<Value> = text
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let (e10, _) = systems(&recs);
        let mut mismatches = 0u64;
        writeln!(md, "\n## E10. Early-stop threshold\n\nSaving = 1 − Σ early / Σ full (word XORs + monomial tests); free = 8 is round 6's.  P-E10a: free = 0 saving ≥ −1 % wherever a system is solvable.  P-E10b: free = 0 keeps ≥ 80 % of the free = 8 saving where that is ≥ 20 %.\n\n| config | solvable | free 0 | free 2 | free 4 | free 8 | P-E10a | P-E10b |\n|:--|--:|--:|--:|--:|--:|:--|:--|").unwrap();
        let mut rows = Vec::new();
        for (c, v) in &e10 {
            let Some(full) = e8.get(c) else { continue };
            let by_t: BTreeMap<u64, &Value> = full
                .iter()
                .map(|r| (r["target"].as_u64().unwrap(), r))
                .collect();
            let mut tot_full = 0.0;
            let mut tot8 = 0.0;
            let mut tot = [0.0f64; 3];
            let mut solvable = 0;
            for r in v {
                let e = by_t[&r["target"].as_u64().unwrap()];
                tot_full += u(&e["full"], "word_xors") + u(&e["full"], "extraction_tests");
                tot8 += u(&e["early"], "word_xors")
                    + u(&e["early"], "extraction_tests")
                    + u(&e["early"], "extraction_xors");
                if e["solutions"].as_u64().unwrap() > 0 {
                    solvable += 1;
                }
                for (k, run) in r["early"].as_array().unwrap().iter().enumerate() {
                    if run["solutions"] != e["solutions"] {
                        mismatches += 1;
                    }
                    tot[k] += u(run, "word_xors")
                        + u(run, "extraction_tests")
                        + u(run, "extraction_xors");
                }
            }
            let s: Vec<f64> = tot.iter().map(|t| 1.0 - t / tot_full).collect();
            let s8 = 1.0 - tot8 / tot_full;
            let a = (solvable > 0).then(|| s[0] >= -0.01);
            let b = (s8 >= 0.2).then(|| s[0] >= 0.8 * s8);
            let yn = |x: Option<bool>| match x {
                Some(true) => "holds",
                Some(false) => "**fails**",
                None => "–",
            };
            writeln!(
                md,
                "| `{c}` | {solvable} | {:.1} % | {:.1} % | {:.1} % | {:.1} % | {} | {} |",
                100.0 * s[0],
                100.0 * s[1],
                100.0 * s[2],
                100.0 * s8,
                yn(a),
                yn(b)
            )
            .unwrap();
            rows.push(
                json!({"config": c, "solvable": solvable, "saving_free0": s[0],
                "saving_free2": s[1], "saving_free4": s[2], "saving_free8": s8,
                "p_e10a": a, "p_e10b": b}),
            );
        }
        writeln!(
            md,
            "\nGate: {mismatches} solution-count mismatches against E8's full runs."
        )
        .unwrap();
        js.insert(
            "e10".into(),
            json!({"rows": rows, "mismatches": mismatches}),
        );
    }

    // ── E11a ──
    if let Ok(text) = std::fs::read_to_string(format!("{dir}/e11a/records.jsonl")) {
        let recs: Vec<Value> = text
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let (sys, cfgs) = systems(&recs);
        writeln!(md, "\n## E11a. Half base (coset at d − 1) against the default base at d, random binary curves\n\nEstimated relation-set work = columns × Σ work ÷ Σ lifted.  P-E11a: half-base max solving degree ≤ default's.  P-E11b: half-base estimate lower on ≥ 75 % of pairs with ≥ 10 default decompositions.\n\n| curve | a | h | d | max degree default / half | columns default / half | lifted default / half | work per system, half ÷ default | estimate, half ÷ default |\n|:--|--:|--:|--:|:--|:--|:--|--:|--:|").unwrap();
        let mut rows = Vec::new();
        let (mut eligible, mut wins, mut deg_ok, mut deg_n) = (0, 0, 0, 0);
        for (c, cfg) in &cfgs {
            if cfg["base"] != "default" {
                continue;
            }
            let hk = c.replace(":default", ":half");
            let (Some(hcfg), Some(dv), Some(hv)) = (cfgs.get(&hk), sys.get(c), sys.get(&hk)) else {
                continue;
            };
            let deg = |v: &[Value]| {
                v.iter()
                    .map(|r| r["full"]["solving_degree"].as_u64().unwrap())
                    .max()
                    .unwrap_or(0)
            };
            let work = |v: &[Value]| -> f64 {
                v.iter()
                    .map(|r| u(&r["full"], "word_xors") + u(&r["full"], "extraction_tests"))
                    .sum()
            };
            let lifted = |v: &[Value]| -> f64 { v.iter().map(|r| u(r, "lifted")).sum() };
            let (dd, hd) = (deg(dv), deg(hv));
            deg_n += 1;
            if hd <= dd {
                deg_ok += 1;
            }
            let (dc, hc) = (u(cfg, "base_abscissae"), u(hcfg, "base_abscissae"));
            let (dl, hl) = (lifted(dv), lifted(hv));
            let (dw, hw) = (work(dv), work(hv));
            let est = |cols: f64, w: f64, l: f64| if l > 0.0 { cols * w / l } else { f64::NAN };
            let ratio = est(hc, hw, hl) / est(dc, dw, dl);
            if dl >= 10.0 && !ratio.is_nan() {
                eligible += 1;
                if ratio < 1.0 {
                    wins += 1;
                }
            }
            writeln!(
                md,
                "| `{}` | {} | {} | {} | {dd} / {hd} | {dc:.0} / {hc:.0} | {dl:.0} / {hl:.0} | {} | {} |",
                cfg["curve"].as_str().unwrap(),
                cfg["a"],
                cfg["cofactor"],
                cfg["d"],
                f((hw / hv.len() as f64) / (dw / dv.len() as f64)),
                f(ratio)
            )
            .unwrap();
            rows.push(
                json!({"curve": cfg["curve"], "a": cfg["a"], "cofactor": cfg["cofactor"],
                "d": cfg["d"], "degree_default": dd, "degree_half": hd, "columns_default": dc,
                "columns_half": hc, "lifted_default": dl, "lifted_half": hl,
                "estimate_half_over_default": ratio}),
            );
        }
        writeln!(md, "\nP-E11a: {deg_ok} of {deg_n} pairs.  P-E11b: half base lower on {wins} of {eligible} eligible pairs.").unwrap();
        // Post hoc (not preregistered): the half base at d has dimension
        // d − 1, so the frozen comparison mixes the 2E lever with a smaller
        // base.  Matched columns: half at d = 10 against default at d = 9,
        // both of dimension 9.
        writeln!(md, "\n### E11a post hoc: matched dimension (half at d = 10 against default at d = 9)\n\nNot preregistered.  Both bases have dimension 9, so the columns match; what differs is that every half-base point is in 2E and its span reaches z⁹.\n\n| curve | columns default / half | max degree default / half | lifted default / half | work per system, half ÷ default | estimate, half ÷ default |\n|:--|:--|:--|:--|--:|--:|").unwrap();
        let mut post = Vec::new();
        for (c, cfg) in &cfgs {
            if !(cfg["base"] == "default" && cfg["d"] == 9) {
                continue;
            }
            let hk = c.replace(":d9:default", ":d10:half");
            let (Some(hcfg), Some(dv), Some(hv)) = (cfgs.get(&hk), sys.get(c), sys.get(&hk)) else {
                continue;
            };
            let deg = |v: &[Value]| {
                v.iter()
                    .map(|r| r["full"]["solving_degree"].as_u64().unwrap())
                    .max()
                    .unwrap_or(0)
            };
            let work = |v: &[Value]| -> f64 {
                v.iter()
                    .map(|r| u(&r["full"], "word_xors") + u(&r["full"], "extraction_tests"))
                    .sum()
            };
            let lifted = |v: &[Value]| -> f64 { v.iter().map(|r| u(r, "lifted")).sum() };
            let (dc, hc) = (u(cfg, "base_abscissae"), u(hcfg, "base_abscissae"));
            let (dl, hl, dw, hw) = (lifted(dv), lifted(hv), work(dv), work(hv));
            let ratio = if dl > 0.0 && hl > 0.0 {
                (hc * hw / hl) / (dc * dw / dl)
            } else {
                f64::NAN
            };
            writeln!(
                md,
                "| `{}` | {dc:.0} / {hc:.0} | {} / {} | {dl:.0} / {hl:.0} | {} | {} |",
                cfg["curve"].as_str().unwrap(),
                deg(dv),
                deg(hv),
                f((hw / hv.len() as f64) / (dw / dv.len() as f64)),
                f(ratio)
            )
            .unwrap();
            post.push(
                json!({"curve": cfg["curve"], "estimate_half10_over_default9": ratio,
                "lifted_default9": dl, "lifted_half10": hl}),
            );
        }
        js.insert("e11a_matched_dimension_post_hoc".into(), json!(post));
        js.insert(
            "e11a".into(),
            json!({"rows": rows, "p_e11a": [deg_ok, deg_n], "p_e11b": [wins, eligible]}),
        );
    }

    // ── E12 ──
    writeln!(md, "\n## E12. Hit rate against decomposition count (E8 records)\n\nHit rate = targets with ≥ 1 lifted decomposition ÷ 200.  P-E12a: at F_2^19, a = 0, d = 9 the coset ÷ default hit-rate interval contains 1.48.  P-E12b: the decomposition ratio exceeds the hit-rate ratio wherever the default hit rate is ≥ 0.3.\n\n| config | default hit rate | hits default / coset | hit-rate ratio | decompositions default / coset | decomposition ratio | decompositions per hit, coset |\n|:--|--:|:--|:--|:--|--:|--:|").unwrap();
    let mut rows = Vec::new();
    for (c, cfg) in &e8cfg {
        if cfg["base"] != "default" {
            continue;
        }
        let ck = c.replace(":default", ":coset");
        let (Some(dv), Some(cv)) = (e8.get(c), e8.get(&ck)) else {
            continue;
        };
        let hits = |v: &[Value]| v.iter().filter(|r| u(r, "lifted") > 0.0).count() as f64;
        let decs = |v: &[Value]| -> f64 { v.iter().map(|r| u(r, "lifted")).sum() };
        let (dh, ch, dd, cd) = (hits(dv), hits(cv), decs(dv), decs(cv));
        let h = poisson(ch, dh);
        let dr = poisson(cd, dd);
        let rate = dh / dv.len() as f64;
        writeln!(
            md,
            "| `{c}` | {rate:.2} | {dh:.0} / {ch:.0} | {} [{}, {}] | {dd:.0} / {cd:.0} | {} | {} |",
            f(h.0),
            f(h.1),
            f(h.2),
            f(dr.0),
            f(if ch > 0.0 { cd / ch } else { f64::NAN })
        )
        .unwrap();
        rows.push(
            json!({"config": c, "default_hit_rate": rate, "hits_default": dh, "hits_coset": ch,
            "hit_ratio": [h.0, h.1, h.2], "decompositions_default": dd, "decompositions_coset": cd,
            "decomposition_ratio": dr.0}),
        );
    }
    js.insert("e12".into(), json!({"rows": rows}));
    std::fs::write(arg(&args, "--out"), &md).unwrap();
    std::fs::write(
        arg(&args, "--json"),
        serde_json::to_string_pretty(&Value::Object(js)).unwrap() + "\n",
    )
    .unwrap();
    print!("{md}");
}
