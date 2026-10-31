//! Candidate family B: isogeny moves.
//!
//! An isogeny of degree coprime to `r` carries `<G>` injectively, so the
//! subgroup is never the obstacle.  What an isogeny cannot change is the
//! point of the audit: over `F_q` isogenous curves share `#E`, the trace,
//! `r`, the embedding degree and the twist order, hence every generic
//! attack and the pairing route cost the same on every class member.  A
//! move can help only through per-representative structure: a smaller GHS
//! magic number (binary composite degree), extra automorphisms for rho, or
//! a structure the destination solver exploits.  This module records the
//! class invariants and local volcano structure from `curve_traits`,
//! prices the identity path against a priced move, prices the Galbraith-
//! Hess-Smart isogeny extension (a heuristic of
//! `jacobian_transfer::binary`), and replays the repository's explicit
//! degree-5 isogenies for sect113r1.

use num_bigint::BigInt;
use serde_json::{json, Value};

use crypto_lib::cryptanalysis::curve_traits::arith::log2_abs;
use crypto_lib::cryptanalysis::curve_traits::Model;
use crypto_lib::cryptanalysis::jacobian_transfer::binary::{
    generic_genus, ghs_magic, isogeny_extension_estimates, IsogenyExtensionEstimate,
};
use crypto_lib::cryptanalysis::jacobian_transfer::log2_biguint;
use crypto_lib::cryptanalysis::sect113r1_audit;

use crate::certificate::{
    map_value, round3, Certificate, Check, CheckResult, Construction, CostRow, CostStatus,
    CostTable, Evidence, SubgroupPreservation, TransferType,
};
use crate::cost::{
    jacobian_can_carry, price_jacobian_route, ConstructionCost, VELU_MULTS_PER_KERNEL_POINT,
};
use crate::ctx::{jmap, Ctx, PriceBasis, Spec};
use crate::verdict::MapBasis;

const SECT113R1_SLUG: &str = "icv1-f2m113-tm122610772499221213-97df4ac6";

pub fn candidates(ctx: &Ctx) -> Result<Vec<Certificate>, String> {
    let mut out = vec![class_invariants(ctx)];
    if ctx.inst.is_binary() {
        out.extend(ghs_extension(ctx)?);
    }
    if ctx.inst.rc.slug == SECT113R1_SLUG {
        out.push(explicit_degree5_replay(ctx)?);
    }
    Ok(out)
}

/// Cost table of "move along one isogeny edge, then attack the codomain":
/// the codomain has the same order, so the destination is the baseline
/// (identity path).  The edge costs one Velu evaluation at degree `ell`.
fn identity_path_costs(ctx: &Ctx, ell: Option<u64>, construction_note: &str) -> CostTable {
    let fb = ctx.inst.field_bits();
    let velu = ell.map(|l| {
        ((VELU_MULTS_PER_KERNEL_POINT * (l as f64 - 1.0) / 2.0).max(1.0)).log2()
            + ctx.model.log2_field_mult(fb)
    });
    let rows = vec![
        CostRow::absent("map_construction", CostStatus::NotEvaluated, construction_note),
        match velu {
            Some(v) => CostRow::modeled(
                "transfer_evaluation",
                v,
                format!("one Velu evaluation at degree {}: log2(4 (l-1)/2) + log2 M(n); VELU_MULTS_PER_KERNEL_POINT is a placeholder", ell.unwrap_or(0)),
            ),
            None => CostRow::absent("transfer_evaluation", CostStatus::NotApplicable, "no walkable small degree"),
        },
        CostRow::absent("relation_collection", CostStatus::NotApplicable, "destination solver is rho, as on E"),
        CostRow::absent("linear_algebra", CostStatus::NotApplicable, "see relation_collection"),
        CostRow::modeled(
            "destination_solve_unsplit",
            ctx.base.log2_total,
            "Pollard rho on the same r over the same field: every class member has the same order, so the matched baseline is unchanged (identity path)",
        ),
        CostRow::modeled(
            "descent_back_mapping",
            0.0,
            "the dual isogeny composition is [deg]; the logarithm transfers by a known multiple of d, recovery is one modular inversion of deg modulo r",
        ),
    ];
    let mut costs = CostTable::with_rows(rows, ctx.base.log2_total, ctx.base.log2_no_automorphisms);
    costs.destination_solver = Some("RhoOnIsogenousCurve".into());
    costs.solvers_considered = vec![("RhoOnIsogenousCurve".into(), round3(ctx.base.log2_total))];
    costs
}

fn class_invariants(ctx: &Ctx) -> Certificate {
    let inst = ctx.inst;
    let t = &ctx.traits;
    let fb = inst.field_bits();
    let walkable: Vec<u64> = t
        .small_primes
        .iter()
        .filter(|s| s.depth > 0 || s.rational_isogenies.is_some_and(|n| n > 0))
        .map(|s| s.ell)
        .filter(|l| *l != if inst.is_binary() { 2 } else { 0 })
        .collect();
    let all_coprime = walkable
        .iter()
        .all(|l| !(&inst.r % *l).eq(&num_bigint::BigUint::default()));
    let cm = t.frobenius.cm_disc.clone();
    use crypto_lib::cryptanalysis::curve_traits::Status;
    let end_status = t.endomorphism.status;
    let end_maximal = t.endomorphism.maximal == Some(true);
    // `Probable` is exact only if every Miller-Rabin probable prime in the
    // discriminant's factorization is prime; only `Proved` earns a PASS.
    let end_proved = end_maximal && end_status == Status::Proved;
    let end_probable = end_maximal && end_status == Status::Probable;
    let cm_small = cm
        .as_deref()
        .and_then(|d| d.parse::<i64>().ok())
        .is_some_and(|d| d == -3 || d == -4);
    let ell = walkable.iter().copied().min();
    let costs = identity_path_costs(
        ctx,
        ell,
        "finding a path to a representative with a cheaper destination: no such representative is identified, so the walk is not priced",
    );

    let mut s = Spec::new(
        TransferType::IsogenyMove,
        "isogeny:class_invariants",
        "isogeny moves: class invariants, volcano structure and the identity-path comparison",
        costs,
    );
    s.map = map_value(
        "F_q-rational isogenies of degree l coprime to r",
        "E",
        "E' in the F_q-isogeny class of E",
        json!({"small_degrees_walkable": walkable}),
        &format!(
            "F_q (q = {})",
            if inst.is_binary() {
                format!("2^{}", fb as u32)
            } else {
                "p".into()
            }
        ),
        Construction::NotEvaluated,
        "Velu formulas for kernel polynomials of degree l; no destination curve is selected",
    );
    s.subgroup = SubgroupPreservation {
        status: if all_coprime { Evidence::Arithmetic } else { Evidence::NotEvaluated },
        method: "arithmetic: gcd(deg phi, r) = 1".into(),
        argument: "a separable isogeny of degree l has a kernel of order l; if gcd(l, r) = 1 the kernel meets <G> trivially and phi is injective on <G>, with image of order r. For an inseparable Frobenius step the degree is a power of the characteristic, which is coprime to r".into(),
        image_order: all_coprime.then(|| inst.r.to_string()),
        kernel_meets_subgroup: all_coprime.then_some(false),
        antecedents: vec![Check::new(
            "gcd(l, r) = 1 for every walkable l <= 31",
            if all_coprime { CheckResult::Pass } else { CheckResult::Fail },
            format!("walkable degrees {walkable:?}; r has {} bits", inst.r.bits()),
        )],
        executed: None,
    };
    s.carried = all_coprime;
    s.map_basis = MapBasis::Existence;
    s.at_isogenous_representative = true;
    s.price_basis = PriceBasis::IdentityPath;
    s.existence = "supported";
    s.explicit_construction = "unknown";
    s.map_correctness = "unknown";
    s.scaling_support = "unknown";
    s.checks = vec![
        Check::new(
            "class_invariants_are_shared",
            CheckResult::Pass,
            "over F_q isogenous curves have equal trace, #E, r, embedding degree and twist order (audit-curve, section 2): no generic attack changes along an isogeny",
        ),
        Check::new(
            "class_members_have_only_negation_as_automorphism",
            match (&cm, cm_small) {
                (None, _) => CheckResult::Indeterminate,
                (Some(_), true) => CheckResult::Fail,
                (Some(_), false) => CheckResult::Pass,
            },
            match &cm {
                None => "d_K is not determined (the discriminant did not factor within the budget)".to_string(),
                Some(d) => format!(
                    "d_K = {d}: units of O_K are +-1 unless d_K is -3 or -4{}",
                    if cm_small { "; members with End = O_K have extra units, and the source's baseline class already uses them where it has them" } else { "" }
                ),
            },
        ),
        Check::new(
            "end_ring_conductor_f_E",
            if end_proved { CheckResult::Pass } else { CheckResult::Indeterminate },
            if end_proved {
                format!(
                    "End(E) = O_K is proved (trait record status Proved): f_E = 1, so E is on the crater of every volcano and no isogeny can lower f_E; conductor gap g = f_pi / f_E = f_pi ({} bits). Moves go down the volcano, away from the maximal order",
                    t.frobenius.conductor_bits
                )
            } else if end_probable {
                format!(
                    "End(E) = O_K holds under trait record status Probable only: it is exact if every Miller-Rabin probable prime (bases 2..53, above 3.3e24) in the factorization of the discriminant is prime, and it is not proved here. If it holds, f_E = 1 and no isogeny can lower f_E (conductor gap g = f_pi, {} bits); the statement is conditional on it",
                    t.frobenius.conductor_bits
                )
            } else {
                "no End(E) certificate: f_E, D_E and the vertical degree are INDETERMINATE, never copied from f_pi (audit-curve)".to_string()
            },
        ),
    ];
    s.invariants = jmap(vec![
        (
            "frobenius",
            serde_json::to_value(&t.frobenius).unwrap_or(Value::Null),
        ),
        (
            "endomorphism",
            serde_json::to_value(&t.endomorphism).unwrap_or(Value::Null),
        ),
        (
            "small_primes",
            serde_json::to_value(&t.small_primes).unwrap_or(Value::Null),
        ),
        (
            "subfield",
            serde_json::to_value(&t.subfield).unwrap_or(Value::Null),
        ),
        (
            "embedding",
            serde_json::to_value(&t.embedding).unwrap_or(Value::Null),
        ),
        (
            "twist",
            serde_json::to_value(&t.twist).unwrap_or(Value::Null),
        ),
        ("ordinary", json!(t.ordinary)),
        ("anomalous", json!(t.anomalous)),
        ("end_ring_status", json!(end_status)),
        (
            "order_check",
            serde_json::to_value(&t.order_check).unwrap_or(Value::Null),
        ),
    ]);
    s.open_obligations = vec![
        "End(E) certificate (f_E, D_E) to place E on its volcano; the conductor-gap mechanism has no priced advantage here (problems/conductor-gap-upper-floor-and-cost.md)".into(),
        "a destination representative with a concrete cheaper attack; none is identified, so the path search is not priced".into(),
    ];
    s.notes.push(
        "C*(E) = min over representatives of (path + transfer + best attack): the identity path gives the baseline, and a priced move adds a transfer term to the same destination cost".into(),
    );
    s.references = vec![
        "audit-curve skill, sections 2-4 (crypto-autoresearcher)".into(),
        "docs/endomorphism-rules.md".into(),
        "src/cryptanalysis/curve_traits (Frobenius discriminant, conductor, volcano levels)".into(),
    ];
    ctx.finish(s)
}

fn ghs_extension(ctx: &Ctx) -> Result<Vec<Certificate>, String> {
    let inst = ctx.inst;
    let Model::Binary { n, irr, a, b, .. } = &inst.rc.model else {
        return Ok(Vec::new());
    };
    let n = *n;
    let q = inst.q();
    let disc = &inst.rc.trace * &inst.rc.trace - BigInt::from(q) * 4;
    let log2_abs_disc = log2_abs(&disc);
    let log2_r = inst.log2_r();
    let mut out = Vec::new();
    let mut ls: Vec<u32> = (1..n).filter(|l| n % l == 0).collect();
    ls.sort_unstable();
    for l in ls {
        let n_rel = n / l;
        let magic = ghs_magic(n, l, irr, a, b);
        let needs_unit = magic.trace_a == 1 && n_rel % 2 == 0;
        let cap = if magic.descent_exists {
            magic.magic
        } else {
            n_rel + 1
        };
        let estimates =
            isogeny_extension_estimates(ctx.model, n, l, n_rel, cap, log2_abs_disc, needs_unit);
        if estimates.is_empty() {
            continue;
        }
        // The repository's selection: the first estimate and the cheapest
        // plausible one.
        let price = |e: &IsogenyExtensionEstimate| {
            let genus = generic_genus(e.magic_bound);
            let lg = log2_biguint(&genus).max(0.0);
            let mut rt = ctx.route(lg, f64::from(l));
            rt.construction = ConstructionCost::Priced(
                e.log2_search_cost,
                "heuristic walk until a curve with the target annihilator degree is hit: reciprocal of the weak fraction times one isogeny step".into(),
            );
            (lg, price_jacobian_route(ctx.model, &rt))
        };
        let mut chosen: Vec<&IsogenyExtensionEstimate> = vec![&estimates[0]];
        if let Some(bp) = estimates
            .iter()
            .filter(|e| e.log2_expected_isogenous >= 0.0)
            .min_by(|x, y| {
                let tx = price(x).1.log2_total.unwrap_or(f64::MAX);
                let ty = price(y).1.log2_total.unwrap_or(f64::MAX);
                tx.partial_cmp(&ty).unwrap_or(std::cmp::Ordering::Equal)
            })
        {
            if bp.annihilator_degree != estimates[0].annihilator_degree {
                chosen.push(bp);
            }
        }
        let best = chosen
            .iter()
            .map(|e| (*e, price(e)))
            .min_by(|x, y| {
                x.1 .1
                    .log2_total
                    .unwrap_or(f64::MAX)
                    .partial_cmp(&y.1 .1.log2_total.unwrap_or(f64::MAX))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .expect("chosen is nonempty");
        let (e, (lg, costs)) = best;
        let plausible = e.log2_expected_isogenous >= 0.0;
        let carries = jacobian_can_carry(lg, f64::from(l), log2_r);
        let rows: Vec<Value> = chosen
            .iter()
            .map(|x| {
                json!({
                    "annihilator_degree": x.annihilator_degree,
                    "magic_bound": x.magic_bound,
                    "log2_candidate_curves": round3(x.log2_candidate_curves),
                    "log2_expected_isogenous": round3(x.log2_expected_isogenous),
                    "log2_search_cost": round3(x.log2_search_cost),
                    "expected_in_class": x.log2_expected_isogenous >= 0.0,
                })
            })
            .collect();
        let mut s = Spec::new(
            TransferType::IsogenyMove,
            &format!("isogeny:ghs_extension:l={l}"),
            &format!("isogeny extension of GHS (Galbraith-Hess-Smart): walk to a curve with magic number <= {} then descend to F_(2^{l})", e.magic_bound),
            costs,
        );
        s.map = map_value(
            "isogeny to E' with small magic number, then GHS conorm-norm map",
            &format!("E(F_(2^{n}))"),
            &format!("Jac(C')(F_(2^{l})), genus <= 2^({}-1)", e.magic_bound),
            json!({"relative_degree": n_rel, "magic_bound": e.magic_bound}),
            &format!("F_(2^{l})"),
            Construction::NotEvaluated,
            "none: no isogenous curve with a small magic number is exhibited",
        );
        s.subgroup = SubgroupPreservation::not_evaluated(
            "none",
            "no isogenous curve is exhibited, so neither the isogeny nor the descent map exists to certify; the class-wide bound on achievable magic numbers is in the GHS candidate of the same tower",
        );
        s.carried = plausible && carries;
        s.map_basis = MapBasis::Existence;
        s.at_isogenous_representative = true;
        s.price_basis = PriceBasis::Hypothetical;
        s.existence = "unknown";
        s.explicit_construction = "unknown";
        s.map_correctness = "unknown";
        s.scaling_support = "unknown";
        s.invariants = jmap(vec![
            ("relative_degree", json!(n_rel)),
            ("base_degree", json!(l)),
            ("direct_magic_number", json!(magic.magic)),
            ("direct_descent_exists", json!(magic.descent_exists)),
            ("target_needs_mmt_condition_8", json!(needs_unit)),
            (
                "isogeny_class_size_log2_heuristic",
                json!(round3(0.5 * log2_abs_disc - std::f64::consts::PI.log2())),
            ),
            ("estimates", json!(rows)),
            ("destination_can_carry_r", json!(carries)),
        ]);
        s.checks = vec![Check::new(
            "heuristic_not_evidence",
            CheckResult::NotRun,
            "counts curves with a small annihilator degree and assumes the class is a random set of size sqrt(|t^2-4q|)/pi; no curve is exhibited",
        )];
        s.open_obligations = vec![
            "exhibit an isogenous curve with a small magic number (the walk needs kernel polynomials of the conductor-gap degrees)".into(),
            "construct the smooth GHS curve and norm-conorm map for that curve".into(),
        ];
        s.notes.push("heuristic row, priced for the record; the totals use the repository's isogeny_extension_estimates".into());
        s.references = vec![
            "jacobian_transfer::binary::isogeny_extension_estimates (Galbraith-Hess-Smart)".into(),
        ];
        out.push(ctx.finish(s));
    }
    Ok(out)
}

/// Replay the repository's own explicit degree-5 Velu isogenies of
/// sect113r1 (`sect113r1_audit`) and certify what they establish: the
/// generator's image has exact order r on a valid codomain and the planted
/// target is carried forward and back.  This is `verified` for those edges
/// and says nothing about a cheaper destination.
fn explicit_degree5_replay(ctx: &Ctx) -> Result<Certificate, String> {
    let report = sect113r1_audit::run_audit()?;
    let reps = &report.valid_isogenies.representatives;
    let edge_ok = |c: &sect113r1_audit::IsogenyCertificate| {
        c.kernel_certificate_exact
            && c.codomain_nonsingular
            && c.generator_image_on_codomain
            && c.generator_image_exact_order_n
            && c.planted_target_forward_homomorphism
            && c.dual_composition_on_generator_is_times_five
            && c.dual_composition_on_planted_target_is_times_five
            && c.planted_target_pullback_to_source
    };
    let all_ok = !reps.is_empty() && reps.iter().all(edge_ok);
    let edges: Vec<Value> = reps
        .iter()
        .map(|c| {
            json!({
                "degree": c.degree,
                "codomain_b": c.codomain_b,
                "codomain_icv1_slug": c.codomain_identity.icv1_slug,
                "generator_image_exact_order_n": c.generator_image_exact_order_n,
                "planted_target_forward_homomorphism": c.planted_target_forward_homomorphism,
                "dual_composition_is_times_five": c.dual_composition_on_generator_is_times_five && c.dual_composition_on_planted_target_is_times_five,
                "verdict_in_source_module": c.verdict,
                "all_checks_pass": edge_ok(c),
            })
        })
        .collect();
    let costs = identity_path_costs(
        ctx,
        Some(5),
        "kernel-polynomial search for the degree-5 edges is not priced here",
    );
    let mut s = Spec::new(
        TransferType::IsogenyMove,
        "isogeny:explicit_degree5",
        "explicit Velu degree-5 isogenies of sect113r1 to the two rational codomains (replay of sect113r1_audit)",
        costs,
    );
    s.map = map_value(
        "Velu isogeny of degree 5 over F_(2^113)",
        "sect113r1",
        "two isogenous codomains (the class's rational 5-kernels)",
        json!({"degree": 5, "edges": edges}),
        "F_(2^113)",
        Construction::Executed,
        "binary_velu::velu_codomain, velu_point_map_binary with the division-polynomial kernel",
    );
    s.subgroup = SubgroupPreservation {
        status: if all_ok { Evidence::Verified } else { Evidence::Refuted },
        method: "replay: sect113r1_audit::run_audit (executed in this process)".into(),
        argument: "for each degree-5 edge the generator's image lies on the nonsingular codomain with exact order r, the planted target is carried forward and pulled back, and the dual composition is [5]".into(),
        image_order: all_ok.then(|| ctx.inst.r.to_string()),
        kernel_meets_subgroup: Some(!all_ok),
        antecedents: Vec::new(),
        executed: Some(json!({
            "report_schema": report.schema,
            "report_certificate_sha256": report.certificate_sha256,
            "source_module_gates_passed": report.implemented_diagnostic_gates_passed,
            "edges": edges,
        })),
    };
    s.executed = s.subgroup.executed.clone();
    s.carried = all_ok;
    s.map_basis = MapBasis::Executed;
    s.at_isogenous_representative = true;
    s.price_basis = PriceBasis::IdentityPath;
    s.existence = "supported";
    s.explicit_construction = "supported";
    s.map_correctness = "supported";
    s.scaling_support = "not_applicable";
    s.checks = vec![Check::new(
        "replayed_isogeny_certificates",
        if all_ok { CheckResult::Pass } else { CheckResult::Fail },
        "sect113r1_audit::run_audit returned certificates for the degree-5 representatives; every flag checked above is true",
    )];
    s.notes.push(
        "transport along these edges is verified; the destination is rho on a curve of the same order, so the cost row equals the baseline: structure exists, no speedup".into(),
    );
    s.references = vec![
        "src/cryptanalysis/sect113r1_audit.rs (the repository's separate, size-based sect113r1 verdict is outside this tool's scope)".into(),
        "tests/sect113r1_ghs_evidence.rs".into(),
    ];
    Ok(ctx.finish(s))
}
