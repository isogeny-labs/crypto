//! Pre-registered yield predictions for experiment E2 of the
//! generator-effect study (research/generator_effect_ic_20261019/E2.md):
//! the cofactor ladder, where the abscissa base and a known-log base
//! should separate by their cofactor-class counts.
//!
//! `cargo run --release --example generator_effect_e2_n0 -- CURVES.jsonl SEED SEED_AA > PREREG.jsonl`
//!
//! `CURVES.jsonl` holds one `{"h": H, "curve": <prime_explicit>}` per line.
//! For each curve, at `B = ⌈#E^{1/3}⌉` (`family_optimum_base` with no
//! folds), the three bases of the E2 arms are built exactly as the
//! pipeline builds them: `prime-abscissa:size=B`,
//! `known-log:size=B,scalars=random,seed=SEED` and the A/A base with
//! `seed=SEED_AA`.  For each base it prints
//!
//! - `n0`: the class-cancelling count `N₀` of `exact_decomposition_ceiling`
//!   (`m = 2`, the signed points), the primary prediction: online yield
//!   ratio of two bases = ratio of their `N₀`;
//! - `sumset`: the distinct nonzero pair sums `±P_i ± P_j` (i ≤ j) that lie
//!   in `⟨G⟩`, which is what the negation-folded pair table can match, and
//!   `sumset / r`, the predicted relations per target tried.
//!
//! `#E` is re-counted by `point_count` (`O(p)` Legendre symbols) and must
//! equal the curve's `group_order`.  Nothing here reads a measurement.
use crypto_lib::cryptanalysis::ic_boundary::{
    exact_decomposition_ceiling, family_optimum_base, prime_factor_base, CountedGroup, GroupOps,
    PrimeCurve, PrimeInstance, PrimePoint,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::KnownLogBase;
use crypto_lib::cryptanalysis::ic_framework::stages::{FactorBaseBuilder, InstanceCtx, Params};
use serde_json::{json, Value};
use std::collections::HashSet;

fn sumset_in_subgroup(curve: &PrimeCurve, points: &[PrimePoint], r: u64) -> usize {
    let mut ops = GroupOps::default();
    let mut sums = HashSet::new();
    for i in 0..points.len() {
        for j in i..points.len() {
            let s = curve.add(&mut ops, points[i], points[j]);
            if curve.is_identity(&s) {
                continue;
            }
            if curve.is_identity(&curve.mul(&mut ops, s, r)) {
                sums.insert(curve.key(&s));
            }
        }
    }
    sums.len()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let seeds = [args[1].clone(), args[2].clone()];
    for line in std::fs::read_to_string(&args[0]).unwrap().lines() {
        let v: Value = serde_json::from_str(line).unwrap();
        let c = &v["curve"];
        let n = |k: &str| c[k].as_u64().unwrap();
        let (p, r, order) = (n("p"), n("r"), n("group_order"));
        let inst = PrimeInstance {
            name: "e2".into(),
            curve: PrimeCurve {
                p,
                a: n("a"),
                b: n("b"),
            },
            group_order: order,
            r,
            cofactor: order / r,
            generator: (n("gx"), n("gy")),
        };
        assert_eq!(inst.curve.point_count(), order, "#E of p = {p}");
        let size = family_optimum_base(order as f64, 1.0, 1.0).ceil() as u64;
        let g = inst.generator_point();
        let ctx = InstanceCtx {
            group: &inst.curve,
            generator: g,
            target: g,
            r,
            cofactor: order / r,
            group_order: order,
            name: inst.name.clone(),
            field_degree: None,
        };
        let mut bases = vec![(
            "ic-abscissa".to_string(),
            format!("prime-abscissa:size={size}"),
            prime_factor_base(&inst, size as usize).points,
        )];
        for (arm, seed) in [
            ("ic-knownlog-random", &seeds[0]),
            ("ic-knownlog-random-aa", &seeds[1]),
        ] {
            let spec = format!("known-log:size={size},scalars=random,seed={seed}");
            let (_, params) = Params::parse_spec(&spec).unwrap();
            let fb = KnownLogBase
                .build(&ctx, &params, &mut GroupOps::default())
                .unwrap();
            bases.push((arm.to_string(), spec, fb.points));
        }
        let mut out = Vec::new();
        for (arm, spec, points) in &bases {
            let (classes, n0_over_r) = exact_decomposition_ceiling(&inst.curve, points, r, 2)
                .expect("N0 computable at m = 2");
            let sumset = sumset_in_subgroup(&inst.curve, points, r);
            out.push(json!({
                "arm": arm, "factor_base": spec, "signed_points": points.len(),
                "classes": classes, "n0_over_r": n0_over_r, "n0": n0_over_r * r as f64,
                "sumset": sumset, "predicted_hit_rate": sumset as f64 / r as f64,
            }));
        }
        let ratio =
            |k: &str, a: usize, b: usize| out[a][k].as_f64().unwrap() / out[b][k].as_f64().unwrap();
        let ratio_n0 = ratio("n0", 1, 0);
        let ratio_sum = ratio("sumset", 1, 0);
        let aa_n0 = ratio("n0", 2, 1);
        let aa_sum = ratio("sumset", 2, 1);
        println!(
            "{}",
            json!({
                "h": v["h"], "p": p, "point_count_checked": true, "r": r, "group_order": order, "size": size, "curve": c,
                "bases": out,
                "predicted_yield_ratio_knownlog_over_abscissa_n0": ratio_n0,
                "predicted_yield_ratio_knownlog_over_abscissa_sumset": ratio_sum,
                "predicted_yield_ratio_aa_n0": aa_n0,
                "predicted_yield_ratio_aa_sumset": aa_sum,
            })
        );
    }
}
