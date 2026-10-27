//! Candidate family A: Weil/GHS descent, one candidate per divisor
//! `l < n` of the binary field degree (including `l = 1`).
//!
//! What is exact: the magic number, GHS type and genus
//! ([`screen_curve`], MMT eq. (5) and Theorem 5(i)), MMT Lemma 6
//! condition (8), the Hasse-Weil capacity of the descent Jacobian, and the
//! set of magic numbers any curve over the field can reach (a class-wide,
//! isogeny-independent bound).  What is executed: the trace branch
//! (`m = 1`, `a` and `b` in `F_(2^l)`) on the actual generator.  What is
//! not: the smooth GHS curve and the norm-conorm map for `m >= 2`, which
//! the repository does not implement (`docs/GHS_TRANSPORT.md`).

use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::{json, Value};

use crypto_lib::binary_ecc::curve::scalar_mul;
use crypto_lib::binary_ecc::{BinaryPoint, F2mElement};
use crypto_lib::cryptanalysis::curve_traits::arith::lucas_v;
use crypto_lib::cryptanalysis::curve_traits::Model;
use crypto_lib::cryptanalysis::ec_trapdoor::FieldTower;
use crypto_lib::cryptanalysis::ghs_descent::{trace_map, ECurve, Pt};
use crypto_lib::cryptanalysis::ghs_screen::{screen_curve, GhsCurveInput};
use crypto_lib::cryptanalysis::ghs_transport::{trace_transport, BinaryPointInput};
use crypto_lib::cryptanalysis::isogeny_degree_search::ball::{
    achievable_magic_numbers, ghs_window_is_empty,
};
use crypto_lib::cryptanalysis::jacobian_transfer::binary::ghs_magic;
use crypto_lib::cryptanalysis::jacobian_transfer::log2_biguint;

use crate::certificate::{
    map_value, round3, Certificate, Check, CheckResult, Construction, CostRow, CostStatus,
    CostTable, Evidence, SubgroupPreservation, TransferType,
};
use crate::cost::{
    jacobian_can_carry, log2_rho_with_class, min_genus_to_carry, price_jacobian_route,
    weil_bits_per_genus, ConstructionCost,
};
use crate::ctx::{jmap, Ctx, Spec};
use crate::spec::{planted_scalar, random_binary_point};
use crate::subgroup::{check_subgroup_preservation, BinaryEc, SubgroupInputs};
use crate::verdict::MapBasis;

fn to_pt(p: &BinaryPoint) -> Pt {
    match p {
        BinaryPoint::Infinity => Pt::Inf,
        BinaryPoint::Affine { x, y } => Pt::Aff {
            x: x.clone(),
            y: y.clone(),
        },
    }
}

fn from_pt(p: &Pt) -> BinaryPoint {
    match p {
        Pt::Inf => BinaryPoint::Infinity,
        Pt::Aff { x, y } => BinaryPoint::Affine {
            x: x.clone(),
            y: y.clone(),
        },
    }
}

fn point_input(p: &BinaryPoint) -> Option<BinaryPointInput> {
    match p {
        BinaryPoint::Infinity => None,
        BinaryPoint::Affine { x, y } => Some(BinaryPointInput {
            x: x.to_biguint(),
            y: y.to_biguint(),
        }),
    }
}

/// `#E(F_(2^l))` for a curve defined over `F_(2^d)` with `d | l`, from the
/// descended trace and the Lucas recurrence; `None` when `l` is not a
/// multiple of `d` or the sign of the trace is ambiguous.
pub fn subfield_order(ctx: &Ctx, l: u32) -> Option<BigInt> {
    let sf = &ctx.traits.subfield;
    let d = sf.definition_degree?;
    let t_d: BigInt = sf.base_trace.as_ref()?.parse().ok()?;
    if d == 0 || !l.is_multiple_of(d) {
        return None;
    }
    let m = l / d;
    if sf.base_trace_sign_free && m % 2 == 1 {
        return None;
    }
    let t_l = lucas_v(&t_d, &(BigInt::one() << d as usize), m);
    Some((BigInt::one() << l as usize) + BigInt::one() - t_l)
}

pub struct TraceOutcome {
    pub evidence: Evidence,
    pub executed: Value,
    pub checks: Vec<Check>,
    pub image_order: Option<String>,
    pub kernel_meets: Option<bool>,
}

/// Execute the trace branch on the actual generator and a planted target,
/// and certify the map with the generic subgroup checker.
pub fn run_trace(
    ctx: &Ctx,
    input: &GhsCurveInput,
    n: u32,
    l: u32,
    n_rel: u32,
) -> Result<TraceOutcome, String> {
    let inst = ctx.inst;
    let c = inst.binary().ok_or("binary curve required")?;
    let Model::Binary { irr, a, b, .. } = &inst.rc.model else {
        return Err("binary model required".into());
    };
    let mut rng = StdRng::seed_from_u64(ctx.seed ^ (u64::from(l) << 32) ^ 0x7ace);
    let d = planted_scalar(&inst.r, &mut rng);
    let g = c.generator.clone();
    let q = scalar_mul(c, &g, &d);
    let gp = point_input(&g).ok_or("generator at infinity")?;
    let qp = point_input(&q).ok_or("planted target at infinity")?;
    let tt = trace_transport(input, n_rel, &gp, &qp, &inst.r)?;
    let ec = ECurve::new(n, irr.clone(), a.clone(), b.clone());
    let tower = FieldTower::new(n, n_rel, l, irr.clone());
    let f = |p: &BinaryPoint| -> Result<BinaryPoint, String> {
        Ok(from_pt(&trace_map(&ec, &tower, &to_pt(p))))
    };
    let samples: Vec<BinaryPoint> = (0..4).map(|_| random_binary_point(c, &mut rng)).collect();
    let scalars: Vec<BigUint> = (0..3).map(|_| planted_scalar(&inst.r, &mut rng)).collect();
    let group = BinaryEc(c);
    let rep = check_subgroup_preservation(&SubgroupInputs {
        src: &group,
        dst: &group,
        map: &f,
        g: &g,
        r: &inst.r,
        r_is_prime: true,
        samples: &samples,
        scalars: &scalars,
    });
    let mut checks = Vec::new();
    // The repository's checked API and the generic map must agree.
    let api_image = tt.generator_image.as_ref().map(from_pt);
    let generic_image = f(&g)?;
    checks.push(Check::new(
        "trace_transport_agrees_with_trace_map",
        if api_image.as_ref() == Some(&generic_image) {
            CheckResult::Pass
        } else {
            CheckResult::Fail
        },
        "ghs_transport::trace_transport and ghs_descent::trace_map return the same image of G",
    ));
    let planted = match (&tt.generator_image, &tt.target_image) {
        (Some(ip), Some(iq)) if *ip != Pt::Inf => {
            let lhs = ec.scalar_mul(ip, &d);
            Some(lhs == *iq)
        }
        _ => None,
    };
    checks.push(Check::new(
        "planted_target_transported",
        match planted {
            Some(true) => CheckResult::Pass,
            Some(false) => CheckResult::Fail,
            None => CheckResult::NotApplicable,
        },
        "Tr(Q) = [d] Tr(G) for the seeded planted scalar d (known-answer fixture); not applicable when Tr(G) = O",
    ));
    let evidence = rep.status;
    let image_order = match (&tt.generator_image, evidence) {
        (_, Evidence::Verified) => Some(inst.r.to_string()),
        (Some(Pt::Inf), _) => Some("1".to_string()),
        _ => None,
    };
    let executed = json!({
        "api": "ghs_transport::trace_transport + ghs_descent::trace_map",
        "relative_degree": n_rel,
        "base_degree": l,
        "status": format!("{:?}", tt.status),
        "generator_image_is_infinity": tt.generator_image == Some(Pt::Inf),
        "planted_scalar_bits": d.bits(),
        "subgroup_report": rep,
    });
    Ok(TraceOutcome {
        evidence,
        executed,
        checks,
        image_order,
        kernel_meets: Some(rep.kernel_meets_subgroup),
    })
}

/// Price the trace route: no construction, $n/l$ Frobenius powers and
/// additions per point, then rho on $E(F_(2^l))$ with that curve's own
/// equivalence class (negation and its Frobenius).
fn trace_costs(ctx: &Ctx, n: u32, l: u32, n_rel: u32) -> CostTable {
    let m = ctx.model;
    let log2_r = ctx.inst.log2_r();
    let eval = f64::from(n_rel).log2() + m.log2_elliptic_op(f64::from(n));
    let d = ctx.traits.subfield.definition_degree.unwrap_or(n);
    let class = if d > 0 && l.is_multiple_of(d) {
        2 * u64::from(l / d)
    } else {
        2
    };
    let dest = log2_rho_with_class(m, log2_r, f64::from(l), class);
    let verify = (1.5 * log2_r).log2() + m.log2_elliptic_op(f64::from(n));
    let rows = vec![
        CostRow::absent("map_construction", CostStatus::NotApplicable, "none: the descent is a Frobenius iteration"),
        CostRow::modeled("transfer_evaluation", eval, "n/l Frobenius powers and n/l elliptic additions in the source field: log2(n/l) + log2(10 * M(n))"),
        CostRow::absent("relation_collection", CostStatus::NotApplicable, "destination is an elliptic curve: rho"),
        CostRow::absent("linear_algebra", CostStatus::NotApplicable, "see relation_collection"),
        CostRow::modeled(
            "destination_solve_unsplit",
            dest,
            format!("rho on E(F_(2^{l})): 0.5 log2 r + log2 sqrt(pi/4) + log2(10 M({l})) - 0.5 log2({class}/2) (equivalence class c = {class} of the destination curve, negation included; the model reference sqrt(pi r/4) is the c = 2 case)"),
        ),
        CostRow::modeled("descent_back_mapping", verify, "the logarithm is unchanged (homomorphism); one scalar multiplication on E verifies it"),
    ];
    let mut t = CostTable::with_rows(rows, ctx.base.log2_total, ctx.base.log2_no_automorphisms);
    t.destination_solver = Some("RhoOnSubfieldCurve".into());
    t.solvers_considered = vec![("RhoOnSubfieldCurve".into(), round3(dest))];
    t
}

pub fn candidates(ctx: &Ctx) -> Result<Vec<Certificate>, String> {
    if !ctx.inst.is_binary() {
        return Ok(vec![prime_not_applicable(ctx)]);
    }
    let inst = ctx.inst;
    let Model::Binary {
        n,
        modulus,
        irr,
        a,
        b,
    } = &inst.rc.model
    else {
        return Err("binary model required".into());
    };
    let n = *n;
    let input = GhsCurveInput {
        absolute_degree: n,
        modulus: modulus.clone(),
        a: a.to_biguint(),
        b: b.to_biguint(),
    };
    let report = screen_curve(&input, &BigUint::from(4u8)).map_err(|e| e.to_string())?;
    let mut rows = report.rows.clone();
    rows.sort_by_key(|r| r.base_degree);
    let proper_subfields: Vec<u32> = (2..n).filter(|d| n % d == 0).collect();
    let mut out = Vec::new();
    if proper_subfields.is_empty() {
        out.push(no_proper_subfield(ctx, n));
    }
    for row in &rows {
        out.push(one_tower(ctx, &input, irr, a, b, &proper_subfields, row)?);
    }
    Ok(out)
}

/// Prime extension degree: there is no proper subfield `F_(2^d)`, `1 < d < n`,
/// so Weil descent to one is out of scope, with the reasons as a certificate:
/// the divisor list of `n`, and the magic numbers any curve over the field
/// can have for the only tower `F_2 <= F_(2^n)`, which are fixed by the
/// factorization of `x^n - 1` over `F_2` (MMT Theorem 5(i)) and do not depend
/// on the curve or its isogeny class.
fn no_proper_subfield(ctx: &Ctx, n: u32) -> Certificate {
    let achievable = achievable_magic_numbers(n);
    let min_ge2 = achievable.iter().copied().find(|v| *v >= 2);
    let window_empty = ghs_window_is_empty(n, 2, 6);
    let costs = CostTable::unpriced(
        ctx.base.log2_total,
        "out of scope: no proper subfield, nothing to price",
    );
    let mut s = Spec::new(
        TransferType::GhsWeilDescent,
        "ghs:proper_subfield",
        "Weil descent to a proper subfield F_(2^d), 1 < d < n",
        costs,
    );
    s.map = map_value(
        "GHS conorm-norm map to a Jacobian over a proper subfield",
        &format!("E(F_(2^{n}))"),
        "none",
        json!({"proper_subfield_degrees": []}),
        "n/a",
        Construction::OutOfScope,
        "none",
    );
    s.subgroup = SubgroupPreservation {
        status: Evidence::NotApplicable,
        method: "none".into(),
        argument: format!("n = {n} is prime: its only subfields are F_2 and F_(2^{n}), so there is no descent to a proper subfield; descent to F_2 is the candidate ghs:l=1"),
        image_order: None,
        kernel_meets_subgroup: None,
        antecedents: vec![Check::new(
            "n_is_prime",
            CheckResult::Pass,
            format!("no divisor d of {n} with 1 < d < {n}"),
        )],
        executed: None,
    };
    s.existence = "not_applicable";
    s.explicit_construction = "not_applicable";
    s.map_correctness = "not_applicable";
    s.scaling_support = "not_applicable";
    s.advantage = Some("not_applicable");
    s.checks = vec![Check::new(
        "ghs_window_empty_for_tower_over_F2",
        if window_empty { CheckResult::Pass } else { CheckResult::Fail },
        format!("no curve over F_(2^{n}), in any isogeny class, has a GHS magic number in 2..=6 for the tower F_2 <= F_(2^{n}); achievable magic numbers: {achievable:?}"),
    )];
    s.invariants = jmap(vec![
        ("absolute_degree", json!(n)),
        ("divisors", json!([1, n])),
        ("achievable_magic_numbers_over_F2", json!(achievable)),
        ("min_magic_number_at_least_2", json!(min_ge2)),
        ("window_2_to_6_empty", json!(window_empty)),
    ]);
    s.notes.push("scope statement, not a security statement: it excludes GHS Weil descent to a proper subfield, nothing else".into());
    s.references = vec![
        "Maurer-Menezes-Teske 2002, Thm 5(i) (as cited in ec_trapdoor.rs)".into(),
        "src/cryptanalysis/isogeny_degree_search/ball.rs (achievable_magic_numbers, ghs_window_is_empty)".into(),
    ];
    ctx.finish(s)
}

fn prime_not_applicable(ctx: &Ctx) -> Certificate {
    let costs = CostTable::unpriced(
        ctx.base.log2_total,
        "no descent exists: the base field has no proper subfield",
    );
    let mut s = Spec::new(
        TransferType::GhsWeilDescent,
        "ghs",
        "Weil/GHS descent (binary fields)",
        costs,
    );
    s.map = map_value(
        "GHS conorm-norm map to a Jacobian over a subfield",
        "E(F_p)",
        "none",
        json!({}),
        "n/a",
        Construction::NotApplicable,
        "none",
    );
    s.subgroup = SubgroupPreservation {
        status: Evidence::NotApplicable,
        method: "none".into(),
        argument: "F_p is a prime field: it has no proper subfield and Weil restriction is trivial"
            .into(),
        image_order: None,
        kernel_meets_subgroup: None,
        antecedents: vec![Check::new(
            "prime_field",
            CheckResult::Pass,
            "the base field is F_p; characteristic is odd",
        )],
        executed: None,
    };
    s.existence = "not_applicable";
    s.explicit_construction = "not_applicable";
    s.map_correctness = "not_applicable";
    s.scaling_support = "not_applicable";
    s.advantage = Some("not_applicable");
    s.notes.push(
        "the odd-characteristic analogues need E over F_(p^k) with k > 1 (Diem; Joux-Vitse); base change to F_(p^k) and descent back to a genus-k cover over F_p is priced in the genus-cover candidate".into(),
    );
    ctx.finish(s)
}

#[allow(clippy::too_many_arguments)]
fn one_tower(
    ctx: &Ctx,
    input: &GhsCurveInput,
    irr: &crypto_lib::binary_ecc::IrreduciblePoly,
    a: &F2mElement,
    b: &F2mElement,
    proper_subfields: &[u32],
    row: &crypto_lib::cryptanalysis::ghs_screen::GhsCoverTower,
) -> Result<Certificate, String> {
    let inst = ctx.inst;
    let n = input.absolute_degree;
    let l = row.base_degree;
    let n_rel = row.relative_degree;
    let m = row.magic_number;
    let log2_r = inst.log2_r();
    let extras = ghs_magic(n, l, irr, a, b);
    let log2_genus = log2_biguint(&row.genus).max(0.0);
    let tower = FieldTower::new(n, n_rel, l, irr.clone());
    let a_in_k = tower.is_in_subfield(a, l);
    let b_in_k = tower.is_in_subfield(b, l);

    let mut checks = vec![
        Check::new(
            "screen_agrees_with_ghs_magic",
            if extras.magic == m && extras.type_i == row.type_i && extras.descent_exists == row.descends {
                CheckResult::Pass
            } else {
                CheckResult::Fail
            },
            "ghs_screen::screen_curve (ec_trapdoor) and jacobian_transfer::binary::ghs_magic return the same magic number, type and condition (8)",
        ),
        Check::new(
            "no_proper_subfield_for_prime_degree",
            if proper_subfields.is_empty() {
                CheckResult::Pass
            } else {
                CheckResult::NotApplicable
            },
            if proper_subfields.is_empty() {
                format!("n = {n} has no divisor d with 1 < d < n: the only tower is F_2 <= F_(2^{n}), so Weil descent to a proper subfield F_(2^d) does not exist")
            } else {
                format!("n = {n} has proper subfield degrees {proper_subfields:?}")
            },
        ),
    ];

    // Class-wide (isogeny-independent) bound: the magic numbers any curve
    // over this field can have for this tower.
    let achievable = achievable_magic_numbers(n_rel);
    let min_ge2 = achievable.iter().copied().find(|v| *v >= 2);
    let window_empty = ghs_window_is_empty(n_rel, 2, 6);
    let class_wide = {
        let mut cw = json!({
            "relative_degree": n_rel,
            "achievable_magic_numbers": achievable,
            "window_2_to_6_empty": window_empty,
            "min_magic_number_at_least_2": min_ge2,
        });
        if let Some(mm) = min_ge2 {
            // Smallest genus any curve over the field reaches with m >= 2:
            // type I at the smallest achievable m.
            let g_low = (BigUint::one() << (mm as usize - 1)) - BigUint::one();
            let g_low = if g_low.is_zero() {
                BigUint::one()
            } else {
                g_low
            };
            let lg = log2_biguint(&g_low).max(0.0);
            let carries = jacobian_can_carry(lg, f64::from(l), log2_r);
            let mut rt = ctx.route(lg, f64::from(l));
            rt.construction =
                ConstructionCost::Priced(0.0, "best case: construction priced at zero".into());
            let t = price_jacobian_route(ctx.model, &rt);
            cw["genus_lower_bound_log2"] = json!(round3(lg));
            cw["best_case_route_log2_total"] = json!(t.log2_total);
            cw["best_case_log2_cost_ratio"] = json!(t.log2_cost_ratio);
            cw["best_case_destination_can_carry_r"] = json!(carries);
            cw["statement"] = json!("every curve over this field, in every isogeny class, has m = 1 or m >= the listed minimum for this tower; the best-case price above is a lower bound on any GHS route through F_(2^l) other than the trace");
        }
        cw
    };

    let mut s;
    let log2_genus_f = log2_genus;
    if !row.descends {
        // MMT Lemma 6 condition (8) fails: no descent exists.
        let mut rt = ctx.route(log2_genus_f, f64::from(l));
        rt.construction = ConstructionCost::Unpriced("no descent exists".into());
        let costs = price_jacobian_route(ctx.model, &rt);
        s = Spec::new(
            TransferType::GhsWeilDescent,
            &format!("ghs:l={l}"),
            &format!("GHS descent to Jac(C)/F_(2^{l}), N = {n_rel} * {l}"),
            costs,
        );
        s.subgroup = SubgroupPreservation {
            status: Evidence::Refuted,
            method: "MMT Lemma 6 condition (8)".into(),
            argument: "the cover does not descend to F_(2^l): every lift of the 2^l-Frobenius has order 2n, so no Weil descent exists and nothing is carried".into(),
            image_order: None,
            kernel_meets_subgroup: None,
            antecedents: vec![Check::new(
                "condition_8_fails",
                CheckResult::Pass,
                "n even, Tr(a) = 1 and Tr_(K/k)(b) = 0",
            )],
            executed: None,
        };
        s.carried = false;
        s.existence = "refuted";
        s.explicit_construction = "not_applicable";
        s.map_correctness = "not_applicable";
    } else if m <= 1 {
        // Subfield curve: the descent is the trace onto E(F_(2^l)).
        let applicable = a_in_k && b_in_k;
        let costs = trace_costs(ctx, n, l, n_rel);
        s = Spec::new(
            TransferType::GhsWeilDescent,
            &format!("ghs:l={l}"),
            &format!("trace descent E(F_(2^{n})) -> E(F_(2^{l})), N = {n_rel} * {l}"),
            costs,
        );
        if applicable {
            let out = run_trace(ctx, input, n, l, n_rel)?;
            checks.extend(out.checks.clone());
            let predicted = subfield_order(ctx, l);
            let (ant, argument) = match &predicted {
                Some(nl) => {
                    let g = BigInt::from(inst.r.clone()).gcd(nl);
                    let killed_predicted = g.is_one();
                    let executed_killed = out.evidence == Evidence::Refuted;
                    let agree = !killed_predicted || executed_killed;
                    (
                        Check::new(
                            "arithmetic_prediction_agrees_with_execution",
                            if agree { CheckResult::Pass } else { CheckResult::Fail },
                            format!(
                                "#E(F_(2^{l})) = {nl}, gcd(r, #E) = {g}: the image of G lies in E(F_(2^{l})), so gcd = 1 forces Tr(G) = O; executed result: {:?}",
                                out.evidence
                            ),
                        ),
                        format!(
                            "Tr_(K/k)(G) lies in E(F_(2^{l})) of order {nl}; gcd(r, #E(F_(2^{l}))) = {g}{}",
                            if killed_predicted { ", so the trace kills <G>" } else { ", so survival is possible and execution decides" }
                        ),
                    )
                }
                None => (
                    Check::new(
                        "arithmetic_prediction_agrees_with_execution",
                        CheckResult::Indeterminate,
                        "the descended trace over F_(2^d) is not available or its sign is ambiguous for this l",
                    ),
                    "the order of E(F_(2^l)) is not available; execution decides".to_string(),
                ),
            };
            let status = out.evidence;
            let mut sp = SubgroupPreservation {
                status,
                method: "executed: ghs_transport::trace_transport + generic checker".into(),
                argument,
                image_order: out.image_order.clone(),
                kernel_meets_subgroup: out.kernel_meets,
                antecedents: vec![ant],
                executed: Some(out.executed.clone()),
            };
            if status == Evidence::Verified {
                sp.argument.push_str("; the trace is injective on <G>, but the destination is E over a subfield: same exponent, constant factor only");
            }
            s.subgroup = sp;
            s.executed = Some(out.executed);
            s.carried = status == Evidence::Verified;
            s.map_basis = MapBasis::Executed;
            s.existence = "supported";
            s.explicit_construction = "supported";
            s.map_correctness = "supported";
        } else {
            s.subgroup = SubgroupPreservation {
                status: Evidence::NotEvaluated,
                method: "none".into(),
                argument: format!(
                    "b lies in F_(2^{l}) (magic number 1) but a does not: the trace branch needs a and b both in the subfield; E may still be K-isomorphic to a model over F_(2^{l}) (a + a' = x^2 + x), and that isomorphism is not applied here"
                ),
                image_order: None,
                kernel_meets_subgroup: None,
                antecedents: Vec::new(),
                executed: None,
            };
            s.carried = false;
            s.existence = "unknown";
        }
        s.scaling_support = "not_applicable";
    } else {
        // m >= 2 and the cover descends.
        let carries = jacobian_can_carry(log2_genus_f, f64::from(l), log2_r);
        let mut rt = ctx.route(log2_genus_f, f64::from(l));
        rt.construction = ConstructionCost::Unpriced(
            "the smooth GHS curve and the norm-conorm map are not implemented in the repository (docs/GHS_TRANSPORT.md, Remaining construction gates)".into(),
        );
        let costs = price_jacobian_route(ctx.model, &rt);
        s = Spec::new(
            TransferType::GhsWeilDescent,
            &format!("ghs:l={l}"),
            &format!("GHS descent to Jac(C)/F_(2^{l}), N = {n_rel} * {l}"),
            costs,
        );
        let per_genus = weil_bits_per_genus(f64::from(l));
        let g_min = min_genus_to_carry(log2_r, f64::from(l));
        if carries {
            s.subgroup = SubgroupPreservation::not_evaluated(
                "none",
                "the Jacobian is large enough to hold r, a necessary condition only; injectivity of the norm-conorm map on <G> needs the explicit cover, which does not exist in the repository",
            );
            s.subgroup.antecedents.push(Check::new(
                "capacity",
                CheckResult::Pass,
                format!("g * 2 log2(2^(l/2) + 1) = 2^{log2_genus_f:.1} * {per_genus:.3} >= log2 r = {log2_r:.2}"),
            ));
            s.carried = true;
            s.open_obligations.push(
                "construct the smooth GHS curve C/F_(2^l) and the norm-conorm map E(F_(2^n)) -> Jac(C)(F_(2^l)) (problems/ghs-higher-magic-descent-not-executable.md)".into(),
            );
            s.open_obligations.push(
                "certify Phi injective on <G>: image of G has order r and the kernel meets <G> trivially".into(),
            );
        } else {
            s.subgroup = SubgroupPreservation {
                status: Evidence::Refuted,
                method: "Hasse-Weil capacity gate".into(),
                argument: format!(
                    "#Jac(C)(F_(2^{l})) <= (2^(l/2) + 1)^(2g) = 2^({per_genus:.3} g); with g = 2^{log2_genus_f:.1} this is below r (log2 r = {log2_r:.2}): the descent Jacobian cannot contain a subgroup of order r, so no map into it is injective on <G>"
                ),
                image_order: None,
                kernel_meets_subgroup: None,
                antecedents: vec![Check::new(
                    "capacity",
                    CheckResult::Pass,
                    format!("least genus that can carry r over F_(2^{l}) is {g_min}; this cover has genus 2^{log2_genus_f:.1}"),
                )],
                executed: None,
            };
            s.carried = false;
        }
        s.existence = "supported";
        s.explicit_construction = "unknown";
        s.map_correctness = "unknown";
        s.scaling_support = "unknown";
        s.notes.push(
            "existence follows from the GHS construction (Galbraith-Hess-Smart 2002; Hess 2004, as cited in ec_trapdoor.rs); the repository carries structural data only".into(),
        );
    }

    let ghs_type = if row.type_i { "I" } else { "II" };
    s.map = map_value(
        "GHS conorm-norm map (trace map when m = 1)",
        &format!("E(F_(2^{n}))"),
        &if m <= 1 {
            format!("E(F_(2^{l}))")
        } else {
            format!("Jac(C)(F_(2^{l})), genus {}", row.genus)
        },
        json!({
            "artin_schreier_cover_degree_log2": m,
            "tower": format!("F_2 <= F_(2^{l}) <= F_(2^{n})"),
            "relative_degree": n_rel,
        }),
        &format!("F_(2^{l})"),
        if m <= 1 && a_in_k && b_in_k && row.descends {
            Construction::Executed
        } else if !row.descends {
            Construction::NotApplicable
        } else {
            Construction::ArithmeticNotExecuted
        },
        if m <= 1 {
            "Tr_(K/k)(P) = P + sigma(P) + ... + sigma^(n-1)(P)"
        } else {
            "none implemented"
        },
    );
    s.checks = checks;
    s.invariants = jmap(vec![
        ("relative_degree", json!(n_rel)),
        ("base_degree", json!(l)),
        ("magic_number", json!(m)),
        ("ghs_type", json!(ghs_type)),
        ("genus", json!(row.genus.to_string())),
        ("log2_genus", json!(round3(log2_genus))),
        ("mmt_condition_8", json!(row.descends)),
        ("trace_a", json!(extras.trace_a)),
        ("relative_trace_b_zero", json!(extras.relative_trace_b_zero)),
        ("orbit_length", json!(extras.orbit_length)),
        ("a_in_subfield", json!(a_in_k)),
        ("b_in_subfield", json!(b_in_k)),
        ("proper_subfield_degrees", json!(proper_subfields)),
        (
            "weil_bits_per_genus",
            json!(round3(weil_bits_per_genus(f64::from(l)))),
        ),
        (
            "min_genus_to_carry_r",
            json!(min_genus_to_carry(log2_r, f64::from(l))),
        ),
        ("class_wide", class_wide),
    ]);
    s.references = vec![
        "Maurer-Menezes-Teske, LMS J. Comput. Math. 5 (2002): eq. (5), Thm 5(i), Lemma 6 (as cited in src/cryptanalysis/ec_trapdoor.rs)".into(),
        "Galbraith-Hess-Smart 2002; Hess, LMS J. Comput. Math. 7 (2004) (as cited in src/cryptanalysis/ghs_descent.rs)".into(),
        "docs/GHS_TRANSPORT.md; tests/sect113r1_ghs_evidence.rs".into(),
    ];
    if let Some(rho) = ctx.traits.subfield.definition_degree {
        s.invariants
            .insert("curve_definition_degree".into(), json!(rho));
    }
    Ok(ctx.finish(s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::cryptanalysis::ec_trapdoor::{ghs_genus_for_type, ghs_type_i, magic_number};

    fn tower(n: u32, l: u32, modulus: u32) -> (FieldTower, F2mElement) {
        let degree = n;
        let irr = crypto_lib::binary_ecc::IrreduciblePoly {
            degree,
            low_terms: (0..degree).filter(|i| (modulus >> i) & 1 == 1).collect(),
        };
        (
            FieldTower::new(n, n / l, l, irr),
            F2mElement::from_bit_positions(&[1], n), // b = z
        )
    }

    /// Hand check 1.  K = F_4 = F_2[z]/(z^2+z+1), k = F_2, sigma = squaring
    /// (order 2), b = z.  sqrt(b) = z^2 (since (z^2)^2 = z^4 = z), and
    /// sigma(z^2) = z^4 = z.  The orbit {z^2, z} is F_2-independent
    /// (z^2 + z = 1 != 0), so the plain dimension is 2 and the sigma
    /// annihilator is x^2 - 1 = (x - 1)^2, divisible by (x - 1): type II.
    /// The vectors (1, z^2), (1, z) sum to (0, 1) != 0, so (1, 0) is not in
    /// the span and m = 2, genus 2^(2-1) = 2.  Condition (8): n = 2 is even
    /// and Tr_(K/k)(b) = z + z^2 = 1 != 0, so the cover descends.
    #[test]
    fn magic_number_and_genus_hand_example_f4() {
        let (t, b) = tower(2, 1, 0b111);
        let a = F2mElement::zero(2);
        assert_eq!(magic_number(&t, &a, &b), 2);
        assert!(!ghs_type_i(&t, &b));
        assert_eq!(ghs_genus_for_type(2, false), BigUint::from(2u8));
        let input = GhsCurveInput {
            absolute_degree: 2,
            modulus: BigUint::from(0b111u8),
            a: BigUint::zero(),
            b: BigUint::from(2u8),
        };
        let rep = screen_curve(&input, &BigUint::from(4u8)).unwrap();
        assert_eq!(rep.rows.len(), 1);
        let r = &rep.rows[0];
        assert_eq!(
            (r.base_degree, r.relative_degree, r.magic_number),
            (1, 2, 2)
        );
        assert_eq!(r.genus, BigUint::from(2u8));
        assert!(!r.type_i && r.descends);
        // The class-wide set for n = 2: x^2 - 1 = (x-1)^2 gives m in {1, 2}.
        assert_eq!(achievable_magic_numbers(2), vec![1, 2]);
    }

    /// Hand check 2.  K = F_8 = F_2[z]/(z^3+z+1), k = F_2, sigma = squaring
    /// (order 3), b = z.  sqrt(b) = z^4 = z^2 + z; the orbit is
    /// s0 = z^4, s1 = z, s2 = z^2 and s0 + s1 + s2 = (z^2+z) + z + z^2 = 0,
    /// so the plain dimension is 2 with annihilator x^2 + x + 1, not
    /// divisible by (x - 1): type I.  The vectors (1, s0), (1, s1), (1, s2)
    /// sum to (1, 0), so (1, 0) lies in the span and m = 2 + 1 = 3, genus
    /// 2^(3-1) - 1 = 3.
    #[test]
    fn magic_number_and_genus_hand_example_f8_type_one() {
        let (t, b) = tower(3, 1, 0b1011);
        let a = F2mElement::zero(3);
        assert_eq!(magic_number(&t, &a, &b), 3);
        assert!(ghs_type_i(&t, &b));
        assert_eq!(ghs_genus_for_type(3, true), BigUint::from(3u8));
        let input = GhsCurveInput {
            absolute_degree: 3,
            modulus: BigUint::from(0b1011u8),
            a: BigUint::zero(),
            b: BigUint::from(2u8),
        };
        let rep = screen_curve(&input, &BigUint::from(4u8)).unwrap();
        let r = &rep.rows[0];
        assert_eq!((r.magic_number, r.type_i), (3, true));
        assert_eq!(r.genus, BigUint::from(3u8));
        // n = 3 is odd, so condition (8) holds.
        assert!(r.descends);
        // x^3 - 1 = (x-1)(x^2+x+1): achievable m are d + e, d in {0, 2}, e = 1.
        assert_eq!(achievable_magic_numbers(3), vec![1, 3]);
    }

    /// A Koblitz curve (b = 1) has m = 1 and genus 1 on every tower.
    #[test]
    fn koblitz_has_magic_number_one_on_every_tower() {
        let modulus = (1u64 << 15) | 3; // z^15 + z + 1
        let input = GhsCurveInput {
            absolute_degree: 15,
            modulus: BigUint::from(modulus),
            a: BigUint::one(),
            b: BigUint::one(),
        };
        let rep = screen_curve(&input, &BigUint::from(4u8)).unwrap();
        let mut ls: Vec<u32> = rep.rows.iter().map(|r| r.base_degree).collect();
        ls.sort_unstable();
        assert_eq!(ls, vec![1, 3, 5]);
        for r in &rep.rows {
            assert_eq!((r.magic_number, r.genus.clone()), (1, BigUint::one()));
        }
    }
}
