//! Controls of the protocol.
//!
//! * F10, positive control: a synthetic GHS instance priced with the frozen
//!   model must be classed `formula_advantage`.
//! * F6, subfield-subgroup control: a subgroup of `E(F_(2^l))` survives the
//!   trace (the descent is injective on it) and the tool labels the result
//!   as a constant-factor effect on a subfield subgroup, not a transfer to
//!   an easier object.
//! * F9, measured index-calculus control in odd characteristic: the
//!   repository's own `hyperelliptic_ic_bench::head_to_head` on thread
//!   curves of genus 2 and 3, compared with the modelled rho cost of a
//!   source subgroup of order about `p`.  Wall-clock calibration enters the
//!   group-operation equivalents, so this record is not byte-reproducible
//!   and `check` skips it.

use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::{json, Value};

use crypto_lib::binary_ecc::curve::scalar_mul;
use crypto_lib::binary_ecc::BinaryPoint;
use crypto_lib::cryptanalysis::curve_traits::arith::factor;
use crypto_lib::cryptanalysis::curve_traits::Model;
use crypto_lib::cryptanalysis::hyperelliptic_ic_bench::{
    full_factor_base_size, head_to_head, thread_instance, HeadToHeadTrials, RhoVariant,
};
use crypto_lib::cryptanalysis::hyperelliptic_index_calculus::{
    HecIndexCalculusParams, LinearAlgebra, RelationSearch, SmoothnessTest,
};
use crypto_lib::cryptanalysis::jacobian_transfer::cost::CostModel;

use crate::certificate::{round3, Evidence, LeadClass, TransferType, Verdict};
use crate::cost::{log2_rho_with_class, price_jacobian_route, ConstructionCost, Route};
use crate::ctx::{judge, Ctx, Judgement, Spec};
use crate::ghs::subfield_order;
use crate::spec::{random_binary_point, Instance};
use crate::verdict::MapBasis;

/// A synthetic descent priced like a certificate and judged by the same
/// [`judge`] function `Ctx::finish` uses (the production path), twice: as
/// the tool would record it (no map, no subgroup certificate), and with the
/// map and the subgroup granted as a counterfactual.
fn synthetic_judgements(
    model: &CostModel,
    (log2_genus, field_bits, source_bits, log2_r): (f64, f64, f64, f64),
) -> (Spec, Judgement, Judgement) {
    let baseline = log2_rho_with_class(model, log2_r, source_bits, 2);
    let rt = Route {
        log2_genus,
        field_bits,
        source_field_bits: source_bits,
        log2_r,
        construction: ConstructionCost::Priced(
            0.0,
            "synthetic: construction priced at zero".into(),
        ),
        transfer_eval_override: None,
        baseline_log2: baseline,
        reference_no_aut_log2: model.log2_rho_elliptic(log2_r, source_bits),
        image_class: 2,
    };
    let costs = price_jacobian_route(model, &rt);
    let mut spec = Spec::new(
        TransferType::GhsWeilDescent,
        "control:synthetic_formula_advantage",
        "synthetic GHS instance (formulas only)",
        costs,
    );
    // No map and no subgroup certificate: the defaults of `Spec::new`; the
    // route may carry the subgroup.
    spec.carried = true;
    let as_recorded = judge(model, &spec);
    let mut granted = spec.clone();
    granted.subgroup.status = Evidence::Verified;
    granted.map_basis = MapBasis::Executed;
    let counterfactual = judge(model, &granted);
    (spec, as_recorded, counterfactual)
}

/// F10, positive control, through the production decision path.
pub fn synthetic_ghs_control(model: &CostModel) -> Value {
    let (spec, j, jg) = synthetic_judgements(model, (2.0, 40.0, 160.0, 158.0));
    let pass = j.lead == LeadClass::FormulaAdvantage
        && j.verdict == Verdict::NoWeaknessFoundInScope
        && jg.verdict == Verdict::SourceTransferWeak;
    json!({
        "schema": "transfer-audit/control/v1",
        "id": "control:synthetic_formula_advantage",
        "protocol": "F10",
        "path": "Spec -> ctx::judge, the function Ctx::finish uses for every certificate (verdict::decide, cost::classify, route evaluation)",
        "description": "synthetic GHS instance: genus 4 over F_(2^40) carrying a 158-bit subgroup of a curve over a 160-bit field; no curve, no map, formulas only",
        "costs": spec.costs,
        "lead_class": j.lead,
        "verdict": j.verdict,
        "verdict_basis": j.basis,
        "evaluation": j.evaluation,
        "expected_lead_class": "formula_advantage",
        "expected_verdict": "no weakness found in scope",
        "counterfactual_with_map_and_subgroup_granted": {
            "note": "the same priced route with the map executed and the subgroup verified, inputs that do not exist for this synthetic instance: shows the decision rule and the margin turn the lead into a weakness verdict once the missing ingredients are supplied",
            "lead_class": jg.lead,
            "verdict": jg.verdict,
            "verdict_basis": jg.basis,
            "expected_verdict": "source-transfer-weak",
        },
        "pass": pass,
        "note": "shows the classifier flags a lead and the verdict rule withholds a weakness verdict without a map and a subgroup certificate; the lead is not a weakness",
    })
}
/// Build the instance of a subgroup contained in `E(F_(2^l))`: the largest
/// prime `r' > n/l` of `#E(F_(2^l))` other than the instance's own `r`, with
/// a generator found as a seeded point times the cofactor and required to
/// be fixed by the `2^l`-Frobenius.
pub fn subfield_subgroup_instance(ctx: &Ctx, l: u32) -> Option<Instance> {
    let inst = ctx.inst;
    let c = inst.binary()?;
    let n = c.m;
    let n_rel = n / l;
    let nl = subfield_order(ctx, l)?.to_biguint()?;
    let fac = factor(&nl, 1 << 20);
    let r2 = fac
        .primes
        .iter()
        .map(|(p, _)| p.clone())
        .filter(|p| *p != inst.r && *p > BigUint::from(n_rel))
        .max()?;
    if !fac.complete() {
        return None;
    }
    let order = inst.rc.order.clone();
    if !(&order % &r2).eq(&BigUint::default()) {
        return None;
    }
    let cof = &order / &r2;
    let mut rng = StdRng::seed_from_u64(ctx.seed ^ (u64::from(l) << 8) ^ 0x5eed);
    for _ in 0..64 {
        let p = random_binary_point(c, &mut rng);
        let g = scalar_mul(c, &p, &cof);
        if let BinaryPoint::Affine { x, y } = &g {
            // Fixed by the 2^l-Frobenius, so it lies in E(F_(2^l)).
            let fx = x.square_k_times(l, &c.irreducible);
            let fy = y.square_k_times(l, &c.irreducible);
            if fx == *x && fy == *y {
                let mut rc = inst.rc.clone();
                rc.slug = format!("{}-subfield-l{l}", inst.rc.slug);
                let Model::Binary { .. } = &rc.model else {
                    return None;
                };
                rc.representations =
                    vec![crypto_lib::cryptanalysis::curve_traits::Representation {
                        generator: crypto_lib::cryptanalysis::curve_traits::GeneratorCoords::Affine(
                            x.to_biguint(),
                            y.to_biguint(),
                        ),
                        subgroup_order: r2.clone(),
                        cofactor: cof.clone(),
                    }];
                let name = format!("{}#subfield-subgroup-l{l}", inst.name);
                return Instance::from_registry_curve(&name, rc).ok();
            }
        }
    }
    None
}

/// Least-squares slope with its standard error (needs three points).
fn slope(points: &[(f64, f64)]) -> Option<(f64, Option<f64>)> {
    if points.len() < 2 {
        return None;
    }
    let n = points.len() as f64;
    let (sx, sy): (f64, f64) = points
        .iter()
        .fold((0.0, 0.0), |a, p| (a.0 + p.0, a.1 + p.1));
    let (mx, my) = (sx / n, sy / n);
    let num: f64 = points.iter().map(|p| (p.0 - mx) * (p.1 - my)).sum();
    let den: f64 = points.iter().map(|p| (p.0 - mx) * (p.0 - mx)).sum();
    if den <= 0.0 {
        return None;
    }
    let b = num / den;
    let se = (points.len() >= 3).then(|| {
        let a = my - b * mx;
        let rss: f64 = points.iter().map(|p| (p.1 - a - b * p.0).powi(2)).sum();
        (rss / (n - 2.0) / den).sqrt()
    });
    Some((b, se))
}

/// The protocol's plan for F9.
pub const PROTOCOL_PLAN: [(u32, &[u64]); 2] = [(2, &[41, 61, 101, 151]), (3, &[23, 31, 41, 61])];

/// A longer plan run after the protocol's, labelled post hoc: it is not
/// pre-registered and does not replace the F9 result.
pub const EXTENDED_PLAN: [(u32, &[u64]); 2] = [
    (2, &[41, 61, 101, 151, 211, 251, 401]),
    (3, &[23, 31, 41, 61, 101, 151]),
];

/// Measured control F9.  Primes are the protocol's; trials 3 and 9.
pub fn hec_ic_control(plan: &[(u32, &[u64])], id: &str, post_hoc: bool) -> Value {
    let trials = HeadToHeadTrials { ic: 3, rho: 9 };
    let mut rows = Vec::new();
    let mut summary = Vec::new();
    for &(genus, primes) in plan {
        let mut pts_ic = Vec::new();
        for &p in primes {
            let Some((curve, d1, n, c)) = thread_instance(p, genus) else {
                rows.push(
                    json!({"genus": genus, "p": p, "skipped": "no usable prime-order subgroup"}),
                );
                continue;
            };
            let k = &n / BigUint::from(3u32) + BigUint::from(7u32);
            let d2 = d1.scalar_mul(&k, &curve);
            let params = HecIndexCalculusParams {
                fb_size: usize::MAX,
                extra_relations: 8,
                max_trials: 5_000_000,
                seed: 20260916,
                search: RelationSearch::factor_base_walk(),
                linear_algebra: LinearAlgebra::Sparse,
                smoothness: SmoothnessTest::Gcd,
                large_primes: false,
            };
            let row = head_to_head(
                &curve,
                &d1,
                &d2,
                &n,
                &params,
                0xC0FFEE,
                200_000_000,
                &k,
                &trials,
                &RhoVariant::DistinguishedPoints,
            );
            // Modelled rho on a source subgroup of prime order about p:
            // sqrt(pi p / 4) group operations.
            let source_rho_ops = (std::f64::consts::PI * p as f64 / 4.0).sqrt();
            if row.ic_correct {
                pts_ic.push(((p as f64).ln(), row.ic_total_group_ops.ln()));
            }
            rows.push(json!({
                "genus": genus,
                "p": p,
                "c": c,
                "jacobian_prime_order_N": row.n.to_string(),
                "factor_base_size": full_factor_base_size(&curve),
                "ic_correct": row.ic_correct,
                "rho_correct": row.rho_correct,
                "ic_total_group_ops": round3(row.ic_total_group_ops),
                "rho_on_same_jacobian_group_ops": round3(row.rho_group_ops),
                "s_ic": round3(row.ic_s),
                "s_rho": round3(row.rho_s),
                "ic_over_rho_same_jacobian": round3(row.ic_s / row.rho_s),
                "modelled_rho_on_source_subgroup_of_order_p_ops": round3(source_rho_ops),
                "ic_over_modelled_source_rho": round3(row.ic_total_group_ops / source_rho_ops),
                "unit_note": "numerator in genus-g Jacobian group-operation equivalents, denominator in elliptic group operations; a Jacobian operation costs more than an elliptic one, so the ratio understates the gap",
            }));
        }
        let sl = slope(&pts_ic).map(|(b, _)| b);
        let se = slope(&pts_ic).and_then(|(_, e)| e);
        summary.push(json!({
            "genus": genus,
            "points_used": pts_ic.len(),
            "fitted_exponent_of_ic_total_ops_in_p": sl.map(round3),
            "standard_error_of_exponent": se.map(round3),
            "asymptotic_exponent_2_minus_2_over_g": round3(2.0 - 2.0 / f64::from(genus)),
            "protocol_f9_threshold": 1.0,
            "pass": sl.is_some_and(|s| s >= 1.0),
        }));
    }
    let all_above = rows
        .iter()
        .filter_map(|r| r["ic_over_modelled_source_rho"].as_f64())
        .all(|x| x > 1.0);
    json!({
        "schema": "transfer-audit/control/v1",
        "id": id,
        "protocol": if post_hoc { "F9 (extended plan, post hoc: not pre-registered)" } else { "F9" },
        "post_hoc": post_hoc,
        "nondeterministic": true,
        "description": "repository index calculus (hyperelliptic_index_calculus, fb-walk, sparse, gcd) on thread curves of genus 2 and 3 over F_p with a prime-order subgroup, vs rho on the same Jacobian (measured) and vs modelled rho on a source subgroup of order about p; 3 index-calculus and 9 rho trials per instance",
        "caveat": "the relations are those of a generic genus-g Jacobian, not of a certified image of E: the repository cannot produce the odd-characteristic cover image (two points at infinity) as input to its index calculus. The group-operation equivalents include wall-clock calibration, so the record varies between runs.",
        "rows": rows,
        "fits": summary,
        "all_rows_ic_above_modelled_source_rho": all_above,
        "pass": all_above && summary.iter().all(|s| s["pass"] == json!(true)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::certificate::RouteStatus;

    /// F10 goes through `judge`, the production decision function: the
    /// recorded certificate has a `formula_advantage` lead and the verdict
    /// `no weakness found in scope` (no map, no subgroup certificate), and
    /// granting those two ingredients turns the same price into a weakness.
    #[test]
    fn f10_runs_the_production_decision_path() {
        let v = synthetic_ghs_control(&CostModel::default());
        assert_eq!(v["pass"], true, "{v}");
        assert_eq!(v["lead_class"], "formula_advantage");
        assert_eq!(v["verdict"], "no weakness found in scope");
        assert_eq!(v["evaluation"]["status"], "not_evaluable");
        assert_eq!(
            v["counterfactual_with_map_and_subgroup_granted"]["verdict"],
            "source-transfer-weak"
        );
        assert!(v["path"].as_str().unwrap().contains("ctx::judge"));
    }

    /// The control can fail: a route over a field where index calculus loses
    /// to rho is not flagged, even with the map and subgroup granted.
    #[test]
    fn f10_negative_control_is_not_flagged() {
        let m = CostModel::default();
        let (_, j, jg) = synthetic_judgements(&m, (2.0, 80.0, 160.0, 158.0));
        assert_ne!(j.lead, LeadClass::FormulaAdvantage);
        assert_eq!(j.verdict, Verdict::NoWeaknessFoundInScope);
        assert_eq!(jg.verdict, Verdict::NoWeaknessFoundInScope);
        assert_eq!(jg.evaluation.status, RouteStatus::Excluded);
    }
}
