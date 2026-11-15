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
//!     [--membership tower|dense|both] [--vkind coset|random]
//! ```
//! `--membership dense` writes membership in `V` as one polynomial
//! `∏(x − v)` (`algebraic_decompose_dense`); `both` solves every target both
//! ways (EXP-PKM-DENSE-DEC-001). `--vkind random` replaces the tower's coset
//! by `|V|` random elements of `F_p` (dense only).
//! One JSON row per target: both relation sets must be equal (`agree`).
//! The auxiliary-curve search of the isogeny form is not charged.

use crypto_lib::cryptanalysis::pkm_smooth_tower::{
    algebraic_decompose, algebraic_decompose_dense, enumerate_decompositions, isogeny_tower_on,
    kummer_tower, search_aux_exact, sqrtm, Curve, Rng,
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
    let membership = get("membership", "tower");
    let vkind = get("vkind", "coset");
    assert!(
        ["tower", "dense", "both"].contains(&membership.as_str()),
        "membership"
    );
    assert!(
        vkind == "coset" || membership == "dense",
        "a random V has no tower"
    );

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
    let v: Vec<u64> = if vkind == "random" {
        let mut vr = Rng(seed ^ 0x5eed_0dd);
        let mut v: Vec<u64> = vec![];
        while v.len() < tower.v.len() {
            let x = vr.below(p);
            if !v.contains(&x) {
                v.push(x);
            }
        }
        v
    } else {
        tower.v.clone()
    };
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
    let base: Vec<(u64, u64)> = v
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
            let (en, en_ops) = enumerate_decompositions(&e, &v, m, r);
            let dense = (membership != "tower").then(|| {
                algebraic_decompose_dense(&e, &v, m, r, dmax, Duration::from_secs(budget))
            });
            let alg = (membership != "dense")
                .then(|| algebraic_decompose(&e, &tower, m, r, dmax, Duration::from_secs(budget)));
            let side = |d: &Option<
                crypto_lib::cryptanalysis::pkm_smooth_tower::AlgebraicDecomposition,
            >| {
                d.as_ref().map(|d| {
                    json!({
                        "relations": d.relations.len(), "agree": !d.undetermined && d.relations == en,
                        "undetermined": d.undetermined, "solutions": d.solutions, "unlifted": d.unlifted,
                        "field_ops": d.field_ops, "f4_field_ops": d.f4_field_ops,
                        "linear_stage_ops": d.linear_stage_ops, "staircase": d.staircase,
                        "solving_degree": d.solving_degree, "max_cols": d.max_cols, "ms": d.ms,
                        "stage_error": d.stage_error,
                        "ratio_over_enum": d.field_ops as f64 / en_ops.max(1) as f64,
                    })
                })
            };
            let row = json!({
                "schema": "pkm-algebraic-oracle.row/v2", "cell": cell, "p": p, "m": m,
                "form": form, "degrees": degrees, "S": v.len(), "log2S": (v.len() as f64).log2(),
                "membership": membership, "vkind": vkind,
                "factor_base_points": base.len(), "aux_curves_tried_uncharged": aux_tried,
                "kind": kind, "draw": draw, "index": k, "seed": seed, "target": [r.0, r.1],
                "curve": [e.a, e.b], "enum_relations": en.len(), "enum_field_ops": en_ops,
                "tower": side(&alg), "dense": side(&dense),
                "ratio_dense_over_tower": match (&dense, &alg) {
                    (Some(d), Some(t)) if !d.undetermined && !t.undetermined =>
                        Some(d.field_ops as f64 / t.field_ops.max(1) as f64),
                    _ => None,
                },
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
