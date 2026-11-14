//! The slides' relation search, measured: the algebraic decomposition oracle
//! (`pkm_smooth_tower::algebraic_decompose`: F4 + substitution on slide 17's
//! low-degree system) against enumeration over the factor base
//! (`enumerate_decompositions`), on the same targets, both counted in `F_p`
//! multiplications (`research/pkm_odd_tower_f4_20261114/PROTOCOL.md`, part B).
//!
//! ```text
//! pkm_algebraic_oracle --p P --degrees 3,3 [--form kummer|isogeny] [--m 2|3]
//!     [--planted K] [--random K] [--seed S] [--budget SECS] [--dmax D]
//!     [--cell NAME] [--out FILE.jsonl]
//! ```
//! One JSON row per target: both relation sets must be equal (`agree`).
//! The auxiliary-curve search of the isogeny form is not charged.

use crypto_lib::cryptanalysis::pkm_smooth_tower::{
    algebraic_decompose, enumerate_decompositions, isogeny_tower_on, kummer_tower,
    search_aux_exact, sqrtm, Curve, Rng,
};
use serde_json::json;
use std::collections::HashMap;
use std::io::Write;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut o: HashMap<String, String> = HashMap::new();
    let mut i = 0;
    while i + 1 < args.len() {
        o.insert(
            args[i].trim_start_matches("--").to_string(),
            args[i + 1].clone(),
        );
        i += 2;
    }
    let get = |k: &str, d: &str| o.get(k).cloned().unwrap_or_else(|| d.to_string());
    let p: u64 = get("p", "1399680001").parse().unwrap();
    let degrees: Vec<u64> = get("degrees", "3,3")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let form = get("form", "kummer");
    let m: usize = get("m", "2").parse().unwrap();
    let planted: u64 = get("planted", "2").parse().unwrap();
    let random: u64 = get("random", "2").parse().unwrap();
    let seed: u64 = get("seed", "20261114").parse().unwrap();
    let budget: u64 = get("budget", "1800").parse().unwrap();
    let dmax: u32 = get("dmax", "64").parse().unwrap();
    let cell = get("cell", "adhoc");

    let (tower, aux_tried) = if form == "kummer" {
        (kummer_tower(p, &degrees, None, seed).expect("kummer"), 0)
    } else {
        let s: u64 = degrees.iter().product();
        let f = search_aux_exact(p, s, seed ^ 0xa0c5, 1_000_000).expect("aux");
        (
            isogeny_tower_on(f.aux, f.order, f.q, &degrees, seed, f.curves_tried).expect("iso"),
            f.curves_tried,
        )
    };
    tower.verify().expect("verify");
    let mut crng = Rng(seed ^ 0xe11);
    let e = loop {
        let c = Curve {
            p,
            a: crng.below(p),
            b: crng.below(p),
        };
        if !c.is_singular() {
            break c;
        }
    };
    let base: Vec<(u64, u64)> = tower
        .v
        .iter()
        .filter_map(|&x| sqrtm(e.rhs(x), p).map(|y| (x, y)))
        .collect();
    let mut out = o.get("out").map(|f| {
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(f)
            .unwrap()
    });
    let mut trng = Rng(seed ^ 0x7a6e7 ^ (m as u64) << 8);
    let threads = std::env::var("RAYON_NUM_THREADS").unwrap_or_default();
    let mut k = 0;
    for (kind, count) in [("planted", planted), ("random", random)] {
        for draw in 0..count {
            let r = if kind == "planted" {
                let pts: Vec<(u64, u64)> = (0..m)
                    .map(|_| base[trng.below(base.len() as u64) as usize])
                    .collect();
                match pts.iter().fold(None, |a, &q| e.add(a, Some(q))) {
                    Some(r) => r,
                    None => continue,
                }
            } else {
                e.random_point(&mut trng).unwrap()
            };
            let alg = algebraic_decompose(&e, &tower, m, r, dmax, Duration::from_secs(budget));
            let (en, en_ops) = enumerate_decompositions(&e, &tower.v, m, r);
            let row = json!({
                "schema": "pkm-algebraic-oracle.row/v1", "cell": cell, "p": p, "m": m,
                "form": form, "degrees": degrees, "S": tower.size(), "log2S": (tower.size() as f64).log2(),
                "factor_base_points": base.len(), "aux_curves_tried_uncharged": aux_tried,
                "kind": kind, "draw": draw, "index": k, "seed": seed, "target": [r.0, r.1],
                "curve": [e.a, e.b],
                "alg_relations": alg.relations.len(), "enum_relations": en.len(),
                "agree": !alg.undetermined && alg.relations == en,
                "alg_undetermined": alg.undetermined, "alg_solutions": alg.solutions,
                "alg_unlifted": alg.unlifted, "alg_field_ops": alg.field_ops,
                "alg_solving_degree": alg.solving_degree, "alg_f4_runs": alg.f4_runs,
                "alg_max_cols": alg.max_cols, "alg_ms": alg.ms,
                "enum_field_ops": en_ops,
                "ratio_alg_over_enum": alg.field_ops as f64 / en_ops.max(1) as f64,
                "dmax": dmax, "budget_s": budget, "threads": threads,
            });
            k += 1;
            let line = row.to_string();
            println!("{line}");
            if let Some(f) = out.as_mut() {
                writeln!(f, "{line}").unwrap();
            }
        }
    }
}
