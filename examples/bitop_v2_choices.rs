//! What each open `bitop/v2` definition choice (`docs/units/BITOPS.md`,
//! "bitop/v2: memory", the eight choices PR #2639 left to the owner) changes
//! on committed records.  Rust, per AGENTS.md "Implementation language".
//!
//! ```text
//! cargo run --release --example bitop_v2_choices -- --out choices.json DIR...
//! ```
//!
//! Each `DIR` is a session directory (`records.jsonl`).  Every verified,
//! non-warm-up record is priced with `ecbench::work::price_record_v2` (the
//! frozen rule) and with the alternative each choice names:
//!
//! 1. `memory_bits` as the **max** of the terms instead of their sum;
//! 2. the relation matrix **sparse** — its non-zeros at construction
//!    (`row_build_modr_adds`: one per summand and one target entry a row)
//!    times `w + ⌈log₂ columns⌉` (value and column index) — instead of
//!    dense `rows · columns · w`;
//! 3. oracle-table **insert attempts** against the entries a table can hold
//!    (its key space: `2^κ` field-element keys): where attempts exceed it,
//!    attempts overstate the entries kept;
//! 4. the kangaroo's distinguished-point entry at `2κ + w + 1` (an affine
//!    point, one distance and the herd bit) instead of `2κ + 2w`;
//! 5. the ranking rule: within a session (one curve) the arms' mean
//!    `bitops`, mean `memory_bits` and mean `at`; a pair ranks under the
//!    frozen rule only where the `bitops` and `at` orders agree;
//! 6. `bound fit`: the slope of `log₂ memory_bits` against `log₂ r` across
//!    sessions, per method (the memory exponent `bound fit --unit bitop/v2`
//!    does not fit);
//! 7. walk state: the jump table and the walks in flight at the
//!    distinguished-point width, against the stored table;
//! 8. records whose memory is incomplete, by method and missing field.
//!
//! Nothing here changes a `bitop/v1` or `bitop/v2` figure: the alternatives
//! are computed beside the frozen rule, for the owner's decision.

use std::collections::BTreeMap;
use std::path::PathBuf;

use crypto_lib::cryptanalysis::bitops::{Entry, Widths};
use crypto_lib::cryptanalysis::ecbench::{record::Record, runner, work};
use serde_json::{json, Value};

#[derive(Default, Clone)]
struct Row {
    session: String,
    arm: String,
    method: String,
    log2_r: f64,
    bitops: f64,
    complete: bool,
    mem_sum: f64,
    mem_max: f64,
    mem_complete: bool,
    matrix_dense: Option<f64>,
    matrix_sparse: Option<f64>,
    table_attempts: Option<f64>,
    table_key_space: Option<f64>,
    kangaroo_half: Option<f64>,
    walk_state: Option<f64>,
    entry_table: f64,
    unpriced: Vec<String>,
}

fn f(v: &str) -> f64 {
    v.parse::<f64>().unwrap_or(f64::NAN)
}

fn phase_native(r: &Record, phase: &str, name: &str) -> Option<u64> {
    r.phases
        .iter()
        .find(|p| p.name == phase)
        .and_then(|p| p.native.get(name).or_else(|| p.work_native.get(name)))
        .copied()
}

fn row(session: &str, r: &Record) -> Option<Row> {
    let v2 = work::price_record_v2(r).ok()?;
    let w: Widths = v2.widths;
    let terms: BTreeMap<String, f64> = v2
        .memory_terms
        .iter()
        .map(|(k, v)| (k.clone(), f(v)))
        .collect();
    let mut out = Row {
        session: session.into(),
        arm: r.arm.clone(),
        method: r.method.id.clone(),
        log2_r: (r.workload.curve.r as f64).log2(),
        bitops: f(&v2.bitops),
        complete: v2.complete,
        mem_sum: f(&v2.memory_bits),
        mem_max: terms.values().cloned().fold(0.0, f64::max),
        mem_complete: v2.memory_complete,
        unpriced: v2.memory_unpriced.clone(),
        ..Default::default()
    };
    let wb = w.r_bits.max(1) as f64;
    out.entry_table = w.entry_bits(Entry::TableEntry) as f64;
    if let Some(&dense) = terms.get("matrix") {
        out.matrix_dense = Some(dense);
        let columns = r
            .factor_base
            .as_ref()
            .map(|fb| fb.columns + 1)
            .or_else(|| phase_native(r, "linear_algebra", "columns"));
        if let (Some(nnz), Some(c)) = (
            phase_native(r, "linear_algebra", "row_build_modr_adds"),
            columns,
        ) {
            let index_bits = (c as f64).log2().ceil().max(1.0);
            out.matrix_sparse = Some(nnz as f64 * (wb + index_bits));
        }
    }
    let attempts: u64 = r
        .phases
        .iter()
        .filter_map(|p| p.work_native.get("table_insert_attempts"))
        .sum();
    if attempts > 0 {
        out.table_attempts = Some(attempts as f64);
        // A table keyed by a field element holds at most 2^κ distinct keys.
        out.table_key_space = Some(2f64.powi(w.element_bits() as i32));
    }
    let dp = w.entry_bits(Entry::DistinguishedPoint) as f64;
    if r.method.id.starts_with("kangaroo") {
        if let Some(&t) = terms.get("table") {
            let k = w.element_bits() as f64;
            let wr = w.r_bits.max(1) as f64;
            out.kangaroo_half = Some(t * (2.0 * k + wr + 1.0) / dp);
        }
    }
    // Walk state: the jump table and the walks in flight, each at the
    // distinguished-point width.
    let c = |k: &str| r.counters.get(k).copied();
    let walk_entries = if r.method.id.starts_with("rho") {
        c("jumps").map(|j| j + 1)
    } else if r.method.id.starts_with("kangaroo") {
        c("jump_count").map(|j| j + c("starts").unwrap_or(2))
    } else if r.method.id.starts_with("ic.") {
        phase_native(r, "relations", "walk_jumps")
            .map(|j| j + phase_native(r, "relations", "walk_pool_offsets").unwrap_or(0) + 1)
    } else {
        None
    };
    out.walk_state = walk_entries.map(|e| e as f64 * dp);
    Some(out)
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn quantiles(mut v: Vec<f64>) -> Value {
    if v.is_empty() {
        return Value::Null;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |x: f64| v[((v.len() - 1) as f64 * x).round() as usize];
    json!({"n": v.len(), "min": q(0.0), "median": q(0.5), "max": q(1.0)})
}

/// OLS slope and intercept of y on x.
fn fit(xs: &[f64], ys: &[f64]) -> Option<(f64, f64)> {
    if xs.len() < 3 {
        return None;
    }
    let (mx, my) = (mean(xs), mean(ys));
    let sxx: f64 = xs.iter().map(|x| (x - mx).powi(2)).sum();
    let sxy: f64 = xs.iter().zip(ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    (sxx > 0.0).then(|| (sxy / sxx, my - sxy / sxx * mx))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut out_path = None;
    let mut dirs = Vec::new();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--out" {
            out_path = args.get(i + 1).cloned();
            i += 2;
        } else {
            dirs.push(PathBuf::from(&args[i]));
            i += 1;
        }
    }
    let mut rows = Vec::new();
    for d in &dirs {
        let name = d
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        for r in runner::read_records(d).unwrap_or_else(|e| panic!("{}: {e}", d.display())) {
            if r.counts() {
                if let Some(x) = row(&name, &r) {
                    rows.push(x);
                }
            }
        }
    }

    // 1. Sum against max, per method.
    let mut by_method: BTreeMap<String, Vec<&Row>> = BTreeMap::new();
    for r in &rows {
        by_method.entry(r.method.clone()).or_default().push(r);
    }
    let sum_vs_max: Value = by_method
        .iter()
        .map(|(m, rs)| {
            (
                m.clone(),
                quantiles(rs.iter().map(|r| r.mem_sum / r.mem_max).collect()),
            )
        })
        .collect::<serde_json::Map<_, _>>()
        .into();
    // 2. Dense against sparse.
    let dense_vs_sparse = quantiles(
        rows.iter()
            .filter_map(|r| Some(r.matrix_dense? / r.matrix_sparse?))
            .collect(),
    );
    let matrix_share = quantiles(
        rows.iter()
            .filter_map(|r| Some(r.matrix_dense? / r.mem_sum))
            .collect(),
    );
    let sparse_total_ratio = quantiles(
        rows.iter()
            .filter_map(|r| {
                let s = r.mem_sum - r.matrix_dense? + r.matrix_sparse?;
                Some(r.mem_sum / s)
            })
            .collect(),
    );
    // 3. Attempts against the key space.
    let attempts_vs_keys = quantiles(
        rows.iter()
            .filter_map(|r| Some(r.table_attempts? / r.table_key_space?))
            .collect(),
    );
    let attempts_over_key_space_records = rows
        .iter()
        .filter(|r| matches!((r.table_attempts, r.table_key_space), (Some(a), Some(k)) if a > k))
        .count();
    // The tables term's share of the declared sum.
    let tables_share = quantiles(
        rows.iter()
            .filter_map(|r| Some(r.table_attempts? * r.entry_table / r.mem_sum))
            .collect(),
    );
    // 7. Walk state against the declared total.
    let walk_by_method: Value = by_method
        .iter()
        .map(|(m, rs)| {
            (
                m.clone(),
                quantiles(
                    rs.iter()
                        .filter_map(|r| Some(r.walk_state? / r.mem_sum))
                        .collect(),
                ),
            )
        })
        .collect::<serde_json::Map<_, _>>()
        .into();
    // 8. Incomplete memory.
    let mut incomplete: BTreeMap<String, BTreeMap<String, u64>> = BTreeMap::new();
    for r in rows.iter().filter(|r| !r.mem_complete) {
        let e = incomplete.entry(r.method.clone()).or_default();
        for u in &r.unpriced {
            *e.entry(u.clone()).or_default() += 1;
        }
    }

    // 5. Ranking, per session: arm means, then each pair's orders.
    #[derive(Clone)]
    struct Arm {
        method: String,
        log2_r: f64,
        bitops: f64,
        sum: f64,
        max: f64,
        kangaroo_half: Option<f64>,
        walk: f64,
        complete: bool,
    }
    let mut arms: BTreeMap<(String, String), Vec<&Row>> = BTreeMap::new();
    for r in &rows {
        arms.entry((r.session.clone(), r.arm.clone()))
            .or_default()
            .push(r);
    }
    let arm_means: BTreeMap<(String, String), Arm> = arms
        .iter()
        .map(|(k, rs)| {
            let g = |f: fn(&Row) -> f64| mean(&rs.iter().map(|r| f(r)).collect::<Vec<_>>());
            (
                k.clone(),
                Arm {
                    method: rs[0].method.clone(),
                    log2_r: rs[0].log2_r,
                    bitops: g(|r| r.bitops),
                    sum: g(|r| r.mem_sum),
                    max: g(|r| r.mem_max),
                    kangaroo_half: rs[0]
                        .kangaroo_half
                        .map(|_| g(|r| r.kangaroo_half.unwrap_or(f64::NAN))),
                    walk: g(|r| r.walk_state.unwrap_or(0.0)),
                    complete: rs.iter().all(|r| r.complete && r.mem_complete),
                },
            )
        })
        .collect();
    let mut pairs = Vec::new();
    let sessions: std::collections::BTreeSet<&String> = arm_means.keys().map(|(s, _)| s).collect();
    let mut tally: BTreeMap<String, [u64; 5]> = BTreeMap::new();
    for s in sessions {
        let here: Vec<(&String, &Arm)> = arm_means
            .iter()
            .filter(|((ss, _), _)| ss == s)
            .map(|((_, a), m)| (a, m))
            .collect();
        for (i, (an, a)) in here.iter().enumerate() {
            for (bn, b) in here.iter().skip(i + 1) {
                // Each arm's mean `at`, from mean bitops × mean memory: the
                // product of the arm's expected gates and declared memory.
                let order = |x: f64, y: f64| x.partial_cmp(&y).map(|o| o as i8).unwrap_or(0);
                let g = order(a.bitops, b.bitops);
                let at_sum = order(a.bitops * a.sum, b.bitops * b.sum);
                let at_max = order(a.bitops * a.max, b.bitops * b.max);
                let ka = a.kangaroo_half.unwrap_or(a.sum);
                let kb = b.kangaroo_half.unwrap_or(b.sum);
                let at_kh = order(a.bitops * ka, b.bitops * kb);
                let at_walk = order(a.bitops * (a.sum + a.walk), b.bitops * (b.sum + b.walk));
                let key = {
                    let mut v = [a.method.as_str(), b.method.as_str()];
                    v.sort();
                    format!("{} vs {}", v[0], v[1])
                };
                let t = tally.entry(key.clone()).or_default();
                t[0] += 1;
                t[1] += u64::from(g == at_sum);
                t[2] += u64::from(g == at_max);
                t[3] += u64::from(g == at_kh);
                t[4] += u64::from(g == at_walk);
                pairs.push(json!({
                    "session": s, "a": an, "b": bn, "log2_r": a.log2_r,
                    "both_complete": a.complete && b.complete,
                    "bitops_a": a.bitops, "bitops_b": b.bitops,
                    "mem_sum_a": a.sum, "mem_sum_b": b.sum,
                    "mem_max_a": a.max, "mem_max_b": b.max,
                    "ranks_sum": g == at_sum, "ranks_max": g == at_max,
                    "ranks_kangaroo_half": g == at_kh,
                    "ranks_with_walk_state": g == at_walk,
                }));
            }
        }
    }
    let ranking: Value = tally
        .iter()
        .map(|(k, t)| {
            (
                k.clone(),
                json!({"pairs": t[0], "ranked_sum": t[1], "ranked_max": t[2], "ranked_kangaroo_point_distance": t[3], "ranked_with_walk_state": t[4]}),
            )
        })
        .collect::<serde_json::Map<_, _>>()
        .into();
    // 6. Memory exponent per method, over the sessions' arm means.
    let mut per_method: BTreeMap<String, (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> =
        BTreeMap::new();
    for a in arm_means.values() {
        let e = per_method.entry(a.method.clone()).or_default();
        e.0.push(a.log2_r);
        e.1.push(a.sum.log2());
        e.2.push(a.max.log2());
        e.3.push(a.bitops.log2());
    }
    let exponents: Value = per_method
        .iter()
        .map(|(m, (x, s, mx, g))| {
            (
                m.clone(),
                json!({
                    "sessions": x.len(),
                    "gates_slope": fit(x, g).map(|f| f.0),
                    "memory_sum_slope": fit(x, s).map(|f| f.0),
                    "memory_max_slope": fit(x, mx).map(|f| f.0),
                    "at_sum_slope": fit(x, &g.iter().zip(s).map(|(a, b)| a + b).collect::<Vec<_>>()).map(|f| f.0),
                }),
            )
        })
        .collect::<serde_json::Map<_, _>>()
        .into();

    let doc = json!({
        "schema": "bitop-v2.choices/v1",
        "sessions": dirs.iter().map(|d| d.display().to_string()).collect::<Vec<_>>(),
        "records": rows.len(),
        "memory_complete_records": rows.iter().filter(|r| r.mem_complete).count(),
        "c1_sum_over_max": sum_vs_max,
        "c2_dense_over_sparse_matrix": dense_vs_sparse,
        "c2_matrix_share_of_sum": matrix_share,
        "c2_sum_dense_over_sum_sparse": sparse_total_ratio,
        "c3_attempts_over_key_space": attempts_vs_keys,
        "c3_records_attempts_exceed_key_space": attempts_over_key_space_records,
        "c3_tables_share_of_sum": tables_share,
        "c5_ranking_by_method_pair": ranking,
        "c6_exponents": exponents,
        "c7_walk_state_over_declared": walk_by_method,
        "c8_memory_incomplete": incomplete,
        "pairs": pairs,
    });
    let text = serde_json::to_string_pretty(&doc).unwrap() + "\n";
    match out_path {
        Some(p) => std::fs::write(&p, &text).unwrap_or_else(|e| panic!("{p}: {e}")),
        None => print!("{text}"),
    }
    let mut summary = doc.clone();
    summary.as_object_mut().unwrap().remove("pairs");
    eprintln!("{}", serde_json::to_string_pretty(&summary).unwrap());
}
