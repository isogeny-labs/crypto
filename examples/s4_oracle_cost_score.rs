//! Scorer for `research/s4_oracle_cost_20261018/` (PREREGISTRATION.md): reads
//! `runs/cell-*.jsonl`, applies the registered verdict and trend rules, prints
//! a markdown table. Computes nothing the records do not hold.
//!
//! `cargo run --release --example s4_oracle_cost_score -- runs/cell-*.jsonl`

use serde_json::Value;
use std::collections::BTreeMap;

fn median(mut v: Vec<f64>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = v.len() / 2;
    Some(if v.len() % 2 == 1 {
        v[m]
    } else {
        (v[m - 1] + v[m]) / 2.0
    })
}

fn num(r: &Value, k: &str) -> f64 {
    r[k].as_f64().unwrap_or(f64::NAN)
}

fn cost(r: &Value, w: f64) -> f64 {
    num(r, "word_xors") + w * (num(r, "unpriced_native") + num(r, "input_terms"))
}

fn main() {
    let mut recs: Vec<Value> = Vec::new();
    for p in std::env::args().skip(1) {
        for l in std::fs::read_to_string(&p).unwrap_or_default().lines() {
            if let Ok(v) = serde_json::from_str::<Value>(l) {
                recs.push(v);
            }
        }
    }
    let cellkey = |r: &Value| -> (u64, u64) {
        match (r["n"].as_u64(), r["ell"].as_u64()) {
            (Some(n), Some(l)) => (l, n),
            _ => {
                let c = r["cell"].as_str().unwrap_or("0:0");
                let (n, l) = c.split_once(':').unwrap();
                (l.parse().unwrap(), n.parse().unwrap())
            }
        }
    };
    let ok = |r: &Value| r["verdict"] == "verified" && r["rep"].as_u64() == Some(1);
    let mut cells: BTreeMap<(u64, u64), Vec<&Value>> = BTreeMap::new();
    for r in &recs {
        cells.entry(cellkey(r)).or_default().push(r);
    }
    println!("| (n,l) | arm | draws run | measured | verified | solving degree | median word_xors | median rows x cols (max) | median f4 s (note) |");
    println!("|---|---|--:|--:|--:|---|--:|---|--:|");
    let mut ratios: Vec<(f64, f64)> = Vec::new();
    let mut verdicts: Vec<String> = Vec::new();
    for ((l, n), rs) in &cells {
        for arm in ["chained", "eliminated"] {
            let a: Vec<&&Value> = rs
                .iter()
                .filter(|r| r["arm"] == arm && r["rep"].as_u64() == Some(1))
                .collect();
            let fin: Vec<&&Value> = a
                .iter()
                .filter(|r| r["finished"] == true)
                .copied()
                .collect();
            let ver: Vec<&&Value> = a.iter().filter(|r| ok(r)).copied().collect();
            let degs: Vec<String> = ver
                .iter()
                .map(|r| r["solving_degree"].to_string())
                .collect();
            let wx = median(ver.iter().map(|r| num(r, "word_xors")).collect());
            let mat = median(
                ver.iter()
                    .map(|r| num(r, "matrix_rows_max") * num(r, "matrix_cols_max"))
                    .collect(),
            );
            let rows = ver
                .first()
                .map(|r| format!("{} x {}", r["matrix_rows_max"], r["matrix_cols_max"]))
                .unwrap_or("-".into());
            let _ = mat;
            let secs = median(ver.iter().map(|r| num(r, "f4_secs")).collect());
            println!(
                "| ({n},{l}) | {arm} | {} | {} | {} | {} | {} | {} | {} |",
                a.len(),
                fin.len(),
                ver.len(),
                degs.join(","),
                wx.map_or("unmeasured".into(), |x| format!("{x:.4e}")),
                rows,
                secs.map_or("-".into(), |x| format!("{x:.2}")),
            );
        }
        // Paired verdict on draws measured and verified on both arms.
        let mut paired: Vec<(&Value, &Value)> = Vec::new();
        for d in 0..4u64 {
            let g = |arm: &str| {
                rs.iter()
                    .find(|r| r["arm"] == arm && r["draw"].as_u64() == Some(d) && ok(r))
                    .copied()
            };
            if let (Some(c), Some(e)) = (g("chained"), g("eliminated")) {
                paired.push((c, e));
            }
        }
        if paired.len() < 3 {
            verdicts.push(format!(
                "({n},{l}): no verdict, {} paired verified draws (< 3)",
                paired.len()
            ));
            continue;
        }
        let mut below = Vec::new();
        for w in [0.0, 0.1, 1.0] {
            let mc = median(paired.iter().map(|p| cost(p.0, w)).collect()).unwrap();
            let me = median(paired.iter().map(|p| cost(p.1, w)).collect()).unwrap();
            below.push((w, mc, me));
        }
        let all_below = below.iter().all(|x| x.2 < x.1);
        let all_above = below.iter().all(|x| x.2 > x.1);
        let tag = if all_below {
            "STAGE IMPROVEMENT"
        } else if all_above {
            "LOSS"
        } else {
            "UNRESOLVED in band"
        };
        let band = below
            .iter()
            .map(|(w, c, e)| format!("w={w}: chained {c:.3e} / elim {e:.3e} = {:.1}x", c / e))
            .collect::<Vec<_>>()
            .join("; ");
        verdicts.push(format!(
            "({n},{l}): {tag} on {} paired draws. {band}",
            paired.len()
        ));
        ratios.push((*l as f64, (below[0].1 / below[0].2).log2()));
    }
    println!();
    for v in &verdicts {
        println!("Verdict {v}");
    }
    // A/A spread at (13,4) draw 0.
    for arm in ["chained", "eliminated"] {
        let v: Vec<&Value> = recs
            .iter()
            .filter(|r| {
                r["arm"] == arm
                    && r["n"] == 13
                    && r["ell"] == 4
                    && r["draw"] == 0
                    && r["finished"] == true
            })
            .collect();
        if v.len() >= 2 {
            let w: Vec<f64> = v.iter().map(|r| num(r, "word_xors")).collect();
            let s: Vec<f64> = v.iter().map(|r| num(r, "f4_secs")).collect();
            println!(
                "A/A {arm} (13,4) draw 0: word_xors {:?} (spread {}), f4 secs {:?}",
                w,
                w.iter().cloned().fold(f64::MIN, f64::max)
                    - w.iter().cloned().fold(f64::MAX, f64::min),
                s
            );
        } else {
            println!("A/A {arm} (13,4): not available");
        }
    }
    // Semantic gap and failures.
    let gap = recs
        .iter()
        .filter(|r| {
            r["arm"] == "eliminated"
                && r["e_size"].as_u64() > r["e_rat_size"].as_u64()
                && r["rep"] == 1
        })
        .count();
    let notver = recs
        .iter()
        .filter(|r| r["finished"] == true && r["verdict"] != "verified")
        .count();
    println!("Records where E > E_rat (eliminated returns non-rational-u zeros): {gap}; finished-but-unverified records: {notver}");
    // Trend.
    if ratios.len() >= 3 {
        let k = ratios.len() as f64;
        let (mx, my) = (
            ratios.iter().map(|r| r.0).sum::<f64>() / k,
            ratios.iter().map(|r| r.1).sum::<f64>() / k,
        );
        let sxy: f64 = ratios.iter().map(|r| (r.0 - mx) * (r.1 - my)).sum();
        let sxx: f64 = ratios.iter().map(|r| (r.0 - mx).powi(2)).sum();
        println!("Trend: slope of log2(chained/eliminated median word_xors) on l = {:.3} over {} cells ({})", sxy / sxx, ratios.len(), if sxy < 0.0 { "advantage shrinking" } else { "advantage growing" });
    } else {
        println!(
            "Trend: not determined ({} cells with a verdict)",
            ratios.len()
        );
    }
}
