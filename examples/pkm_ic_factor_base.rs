//! EXP-PKM-IC-001: one cell of the factor-base comparison in the prime-field
//! pair-sum index calculus (`research/pkm_ic_factor_base_20261119/PROTOCOL.md`).
//!
//! ```text
//! pkm_ic_factor_base --curves research/pkm_ic_factor_base_20261119/curves.json \
//!     --bits 20 --t 8 --targets 6 [--arms kummer,small-x,random-x] [--out rows.jsonl]
//! ```
//! For each target `i` (scalar from `StdRng(20261119 + i)`, relation seed
//! `7 + i`) and each arm, one full staged solve with the pair-sum table forced
//! (`IC_SUM_TABLE=1`). One JSON row per solve. The arms share the curve, the
//! targets and the relation sampler; only the factor base differs, and all
//! three have the same number of points `M` (the liftable part of the coset).

use crypto_lib::cryptanalysis::ec_index_calculus::{
    build_factor_base, build_factor_base_from_xs,
    ec_index_calculus_dlp_staged_seeded_with_factor_base, FactorBaseEntry,
};
use crypto_lib::cryptanalysis::pkm_smooth_tower::kummer_tower;
use crypto_lib::ecc::curve::CurveParams;
use crypto_lib::ecc::point::Point;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut o: HashMap<String, String> = HashMap::new();
    let mut i = 1;
    while i + 1 < args.len() {
        o.insert(
            args[i].trim_start_matches("--").to_string(),
            args[i + 1].clone(),
        );
        i += 2;
    }
    let get = |k: &str, d: &str| o.get(k).cloned().unwrap_or_else(|| d.to_string());
    let bits: u64 = get("bits", "20").parse().unwrap();
    let t: usize = get("t", "8").parse().unwrap();
    let targets: u64 = get("targets", "6").parse().unwrap();
    let arms: Vec<String> = get("arms", "kummer,small-x,random-x")
        .split(',')
        .map(String::from)
        .collect();
    let curves: Vec<Value> = serde_json::from_str(
        &std::fs::read_to_string(get(
            "curves",
            "research/pkm_ic_factor_base_20261119/curves.json",
        ))
        .unwrap(),
    )
    .unwrap();
    let c = curves
        .iter()
        .find(|c| c["bits"].as_u64() == Some(bits))
        .expect("curve for --bits");
    let u = |k: &str| BigUint::from(c[k].as_u64().unwrap());
    let curve = CurveParams {
        name: Box::leak(c["name"].as_str().unwrap().to_string().into_boxed_str()),
        p: u("p"),
        a: u("a"),
        b: u("b"),
        gx: u("gx"),
        gy: u("gy"),
        n: u("n"),
        h: 1,
    };
    let p = c["p"].as_u64().unwrap();
    let a_fe = curve.a_fe();
    let g = curve.generator();
    assert!(
        matches!(g.scalar_mul(&curve.n, &a_fe), Point::Infinity),
        "[n]G must be O"
    );
    std::env::set_var("IC_SUM_TABLE", "1");

    // Factor bases. The Kummer coset fixes M; the other arms match it.
    let v = kummer_tower(p, &vec![2; t], None, 5)
        .expect("Kummer tower: 2^t must divide p - 1")
        .v;
    let xs: Vec<BigUint> = v.iter().map(|&x| BigUint::from(x)).collect();
    let kummer = build_factor_base_from_xs(&curve, &xs);
    let m = kummer.len();
    let small = build_factor_base(&curve, m);
    let random = {
        let mut rng = StdRng::seed_from_u64(0x5eed ^ bits ^ (t as u64) << 8);
        let mut xs: Vec<BigUint> = vec![];
        let mut fb: Vec<FactorBaseEntry> = vec![];
        while fb.len() < m {
            xs.push(BigUint::from(rng.gen_range(1..p)));
            fb = build_factor_base_from_xs(&curve, &xs);
        }
        fb.truncate(m);
        fb
    };
    assert_eq!(
        (small.len(), random.len()),
        (m, m),
        "arms must have equal size"
    );

    let mut out = o.get("out").map(|f| {
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(f)
            .unwrap()
    });
    let rho_ref = (std::f64::consts::PI * c["n"].as_u64().unwrap() as f64 / 2.0).sqrt();
    for i in 0..targets {
        let mut trng = StdRng::seed_from_u64(20261119 + i);
        let k = BigUint::from(trng.gen_range(1..c["n"].as_u64().unwrap()));
        let q = g.scalar_mul(&k, &a_fe);
        for arm in &arms {
            let fb = match arm.as_str() {
                "kummer" => kummer.clone(),
                "small-x" => small.clone(),
                "random-x" => random.clone(),
                other => panic!("arm {other}"),
            };
            let t0 = Instant::now();
            let res = ec_index_calculus_dlp_staged_seeded_with_factor_base(
                &curve,
                &g,
                &q,
                fb,
                8,
                1_000_000,
                64,
                7 + i,
            );
            let wall_ms = t0.elapsed().as_secs_f64() * 1e3;
            let row = match res {
                Some((log, rep)) => {
                    let charged = rep.counters.get("group_additions").copied().unwrap_or(0)
                        + rep.counters.get("group_doublings").copied().unwrap_or(0);
                    let ok = log == k && g.scalar_mul(&log, &a_fe) == q;
                    json!({
                        "schema": "pkm-ic-factor-base.row/v1", "curve": curve.name, "bits": bits,
                        "p": p, "n": c["n"], "t": t, "S": v.len(), "M": m, "arm": arm, "target": i,
                        "solved": true, "correct": ok, "charged_group_ops": charged,
                        "rho_reference": rho_ref, "ratio_over_rho": charged as f64 / rho_ref,
                        "trials_total": rep.trials_total, "relations": rep.relations_collected,
                        "trials_per_relation_median": rep.trials_per_relation_median,
                        "factor_base_ms": rep.factor_base_ms, "relations_ms": rep.relations_ms,
                        "linear_algebra_ms": rep.linear_algebra_ms, "verify_ms": rep.verify_ms,
                        "wall_ms": wall_ms, "counters": rep.counters,
                        "counters_unemitted": rep.counters_unemitted,
                    })
                }
                None => json!({
                    "schema": "pkm-ic-factor-base.row/v1", "curve": curve.name, "bits": bits, "p": p,
                    "t": t, "S": v.len(), "M": m, "arm": arm, "target": i, "solved": false,
                    "correct": false, "wall_ms": wall_ms,
                }),
            };
            let line = row.to_string();
            println!("{line}");
            if let Some(f) = out.as_mut() {
                writeln!(f, "{line}").unwrap();
            }
        }
    }
}
