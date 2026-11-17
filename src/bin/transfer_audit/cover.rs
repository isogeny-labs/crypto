//! Candidate family C: genus-g covers `H -> E` over the curve's own field.
//!
//! Binary fields: the same-field genus-3, degree-3 cover of
//! `binary_ecc::cover_transfer`, executed on the generator (the pullback is
//! explicit and `Jac(H)` arithmetic exists).  Prime fields: the same-field
//! genus-2, degree-2 cover `v^2 = F(u^2 + c)` of `docs/curves/COVERS.md`;
//! its Jacobian has two points at infinity and the repository has no
//! arithmetic for it, so the subgroup obligation is an arithmetic argument
//! only.  Both are priced with the destination solvers of the frozen
//! model, and a genus sweep `g = 2..8` reproduces the recorded negative
//! result `IDEA-20261009-275a14` (index calculus on a genus-`g` Jacobian
//! over `F_q` costs `q^(2 - 2/g)`, never below rho on a subgroup of
//! size about `q`).

use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::rngs::StdRng;
use rand::SeedableRng;
use serde_json::json;

use crypto_lib::binary_ecc::curve::point_add;
use crypto_lib::binary_ecc::{
    BinaryPoint, F2mElement, OrdinaryBinaryCover, SubgroupTransferStatus,
};
use crypto_lib::cryptanalysis::curve_traits::Model;

use crate::certificate::{
    map_value, round3, Certificate, Check, CheckResult, Construction, CostTable, Evidence,
    SubgroupPreservation, TransferType,
};
use crate::cost::{
    class_discount_bits, ic_leading_exponent, price_jacobian_route, ConstructionCost,
};
use crate::ctx::{jmap, Ctx, PriceBasis, Spec};
use crate::spec::{planted_scalar, random_binary_point};
use crate::subgroup::{check_subgroup_preservation, BinaryEc, BinaryJac, SubgroupInputs};
use crate::verdict::MapBasis;

pub fn candidates(ctx: &Ctx) -> Result<Vec<Certificate>, String> {
    let mut out = Vec::new();
    if ctx.inst.is_binary() {
        out.push(binary_cover(ctx)?);
    } else {
        out.push(prime_cover(ctx)?);
    }
    out.push(genus_sweep(ctx));
    Ok(out)
}

fn binary_cover(ctx: &Ctx) -> Result<Certificate, String> {
    let inst = ctx.inst;
    let c = inst.binary().ok_or("binary curve required")?;
    let Model::Binary { n, irr, a, b, .. } = &inst.rc.model else {
        return Err("binary model required".into());
    };
    let n = *n;
    let cover = OrdinaryBinaryCover::new(n, irr.clone(), a.clone(), b.clone())
        .map_err(|e| format!("cover construction: {e}"))?;
    let mut checks = vec![Check::new(
        "cover_certificate_replay",
        match cover.verify_certificate() {
            Ok(()) => CheckResult::Pass,
            Err(_) => CheckResult::Fail,
        },
        "OrdinaryBinaryCover::verify_certificate: d^2 = b, defining-equation substitution, genus 3, degree 3, separable, ramification at infinity and at (0,0)",
    )];
    let h = cover.source_curve();
    let g = c.generator.clone();
    let mut rng = StdRng::seed_from_u64(ctx.seed ^ 0xc07e);
    let planted = planted_scalar(&inst.r, &mut rng);
    let mut scalars = vec![planted.clone()];
    scalars.extend((0..2).map(|_| planted_scalar(&inst.r, &mut rng)));
    let samples: Vec<BinaryPoint> = (0..4).map(|_| random_binary_point(c, &mut rng)).collect();
    let f = |p: &BinaryPoint| -> Result<_, String> {
        cover.pullback_point_class(p).map_err(|e| e.to_string())
    };
    let src = BinaryEc(c);
    let dst = BinaryJac(h);
    let rep = check_subgroup_preservation(&SubgroupInputs {
        src: &src,
        dst: &dst,
        map: &f,
        g: &g,
        r: &inst.r,
        r_is_prime: true,
        samples: &samples,
        scalars: &scalars,
    });
    // phi_* phi^* = [3] on the generator (typed witness).
    let comp = cover.pull_push_composition_certificate(&g);
    checks.push(Check::new(
        "pull_push_equals_multiplication_by_three_on_G",
        match &comp {
            Ok(c) if c.identity_holds => CheckResult::Pass,
            _ => CheckResult::Fail,
        },
        "PullPushCompositionCertificate: formal pushforward of the pullback of [G]-[O] equals [3]G (a typed witness, not a general Jacobian norm)",
    ));
    let transfer_status = cover.subgroup_transfer_status(&inst.r);
    let degree_coprime = matches!(
        transfer_status,
        Ok(SubgroupTransferStatus::InjectiveIfExactSubgroupOrder { .. })
    );
    checks.push(Check::new(
        "degree_coprime_to_r",
        if degree_coprime { CheckResult::Pass } else { CheckResult::Fail },
        "gcd(3, r) = 1: with phi_* phi^* = [3], the pullback is injective on <G> once the exact order of <G> is established (it is: validation)",
    ));
    // The unique rational 2-torsion point T = (0, sqrt b).
    let t = BinaryPoint::Affine {
        x: F2mElement::zero(n),
        y: cover.square_root_b().clone(),
    };
    let t_ok = c.is_on_curve(&t) && point_add(c, &t, &t) == BinaryPoint::Infinity;
    checks.push(Check::new(
        "rational_two_torsion_point",
        if t_ok { CheckResult::Pass } else { CheckResult::Fail },
        "T = (0, sqrt b) is on E and 2T = O; it is the branch point of the cover over x = 0 (ramification index 3)",
    ));

    let status = rep.status;
    let image_order = (status == Evidence::Verified).then(|| inst.r.to_string());
    let mut rt = ctx.route(3f64.log2(), f64::from(n));
    rt.construction = ConstructionCost::Priced(
        f64::from(n).log2() + ctx.model.log2_field_mult(f64::from(n)),
        "d = sqrt(b) = b^(2^(n-1)): at most n field multiplications".into(),
    );
    // H is defined over the subfield F_(2^d) that gives E its Frobenius class
    // (a, b and d = sqrt(b) all lie there), and phi^* commutes with that
    // Frobenius, so rho on the image walks the same class as the baseline.
    rt.image_class = ctx.base.class_size;
    let costs = price_jacobian_route(ctx.model, &rt);
    let mut s = Spec::new(
        TransferType::GenusCover,
        "cover:binary_g3_deg3",
        &format!(
            "same-field genus-3 cover H -> E of degree 3 over F_(2^{n}), v^2+u^2 v = u^7+a u^4+d u"
        ),
        costs,
    );
    s.map = map_value(
        "pullback phi^*: E(F_q) -> Jac(H)(F_q), P -> phi^*([P]-[O])",
        &format!("E(F_(2^{n}))"),
        &format!("Jac(H)(F_(2^{n})), genus 3"),
        json!({"cover_degree": 3, "genus": 3, "ramification": "index 3 at infinity and at (0,0)"}),
        &format!("F_(2^{n})"),
        Construction::Executed,
        "(u,v) -> (u^3, u v + d); pullback of (x,y), x != 0: U = t^3 + x, V = ((y+d)/x) t^2",
    );
    s.subgroup = SubgroupPreservation {
        status,
        method: "executed: pullback of the generator, Cantor arithmetic in Jac(H), generic checker".into(),
        argument: "phi^*(G) has exact order r in Jac(H)(F_q) (r prime, phi^*(G) != O, [r]phi^*(G) = O), so phi^* is injective on <G>; additivity and scalar equivariance sampled".into(),
        image_order,
        kernel_meets_subgroup: Some(rep.kernel_meets_subgroup),
        antecedents: vec![Check::new(
            "gcd(3,r)=1",
            if degree_coprime { CheckResult::Pass } else { CheckResult::Fail },
            "consistency with phi_* phi^* = [3]",
        )],
        executed: Some(json!({"subgroup_report": rep, "planted_scalar_bits": planted.bits()})),
    };
    s.executed = Some(json!({
        "api": "binary_ecc::OrdinaryBinaryCover::{pullback_point_class, pull_push_composition_certificate, subgroup_transfer_status}",
        "subgroup_report": rep,
    }));
    s.carried = status == Evidence::Verified;
    s.map_basis = MapBasis::Executed;
    s.existence = "supported";
    s.explicit_construction = "supported";
    s.map_correctness = "supported";
    s.scaling_support = "unknown";
    s.checks = checks;
    s.invariants = jmap(vec![
        ("genus", json!(3)),
        ("degree", json!(3)),
        (
            "rational_two_torsion",
            json!({"points": ["O", "(0, sqrt b)"], "count": 2}),
        ),
        (
            "destination",
            json!("Jac(H)(F_q), q = 2^n, genus 3, one rational point at infinity"),
        ),
        (
            "ic_leading_exponent_in_log2_q",
            json!(round3(ic_leading_exponent(3.0))),
        ),
        (
            "rho_leading_exponent_in_log2_q",
            json!(round3(0.5 * inst.log2_r() / inst.field_bits())),
        ),
    ]);
    s.open_obligations = vec![
        "pushforward of an arbitrary Jacobian class is implemented only for one fixed F_8 fixture (problems/binary-cover-arbitrary-jacobian-norm-missing.md); recovery needs only the logarithm of the image, so this does not block the cost row".into(),
        "no index calculus exists in the repository for a genus-3 binary Jacobian; the destination price is the frozen formula (hyperelliptic_ic_bench measures odd-characteristic Jacobians only)".into(),
    ];
    s.notes.push(
        "structure exists and the subgroup is carried (verified), but the destination is a genus-3 Jacobian over the same field: the exponent is (4/3) log2 q against rho's 0.5 log2 r".into(),
    );
    s.notes.push(if ctx.base.class_size > 2 {
        format!(
            "rho on the Jacobian image is priced with the same equivalence class as the baseline (c = {}: the cover is defined over the subfield that gives E its Frobenius class, which commutes with phi^*); the total under the frozen model alone, without that discount, is costs.log2_total_frozen_model and is what the cross-check against docs/curves/jacobian-transfers.json compares",
            ctx.base.class_size
        )
    } else {
        "the baseline has no equivalence class beyond negation (c = 2), so rho on the image gets no discount and the total equals the frozen model's".to_string()
    });
    s.references = vec![
        "docs/curves/COVERS.md (ordinary binary models)".into(),
        "src/binary_ecc/cover_transfer.rs".into(),
        "crypto-autoresearcher ledger IDEA-20261009-275a14".into(),
    ];
    Ok(ctx.finish(s))
}

// ── Odd characteristic ───────────────────────────────────────────────

type Poly = Vec<BigUint>;

fn trim(mut v: Poly) -> Poly {
    while v.last().is_some_and(|c| c.is_zero()) {
        v.pop();
    }
    v
}

fn sub_mod(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
    (a + p - (b % p)) % p
}

fn poly_mul(a: &Poly, b: &Poly, p: &BigUint) -> Poly {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![BigUint::zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            out[i + j] = (&out[i + j] + x * y) % p;
        }
    }
    trim(out)
}

fn poly_sub(a: &Poly, b: &Poly, p: &BigUint) -> Poly {
    let len = a.len().max(b.len());
    let zero = BigUint::zero();
    trim(
        (0..len)
            .map(|i| sub_mod(a.get(i).unwrap_or(&zero), b.get(i).unwrap_or(&zero), p))
            .collect(),
    )
}

/// Remainder of `a` by `f` over `F_p` (`f` nonzero).
fn poly_rem(a: &Poly, f: &Poly, p: &BigUint) -> Poly {
    let mut r = trim(a.clone());
    let df = f.len() - 1;
    let lead_inv = f[df].modpow(&(p - 2u8), p);
    while r.len() > df {
        let shift = r.len() - 1 - df;
        let coef = &r[r.len() - 1] * &lead_inv % p;
        for (i, fc) in f.iter().enumerate() {
            let sub = &coef * fc % p;
            r[shift + i] = sub_mod(&r[shift + i], &sub, p);
        }
        r = trim(r);
    }
    r
}

fn poly_gcd(mut a: Poly, mut b: Poly, p: &BigUint) -> Poly {
    while !b.is_empty() {
        let r = poly_rem(&a, &b, p);
        a = b;
        b = r;
    }
    a
}

/// Number of roots in `F_p` of the squarefree cubic `x^3 + a x + b`:
/// the degree of `gcd(x^p - x, f)`.
pub fn cubic_root_count(p: &BigUint, a: &BigUint, b: &BigUint) -> usize {
    let f: Poly = trim(vec![b % p, a % p, BigUint::zero(), BigUint::one()]);
    let x: Poly = vec![BigUint::zero(), BigUint::one()];
    // x^p mod f by square-and-multiply.
    let mut result: Poly = vec![BigUint::one()];
    let mut base = poly_rem(&x, &f, p);
    for i in 0..p.bits() {
        if p.bit(i) {
            result = poly_rem(&poly_mul(&result, &base, p), &f, p);
        }
        base = poly_rem(&poly_mul(&base, &base, p), &f, p);
    }
    let h = poly_sub(&result, &x, p);
    if h.is_empty() {
        return 3;
    }
    poly_gcd(f, h, p).len() - 1
}

fn prime_cover(ctx: &Ctx) -> Result<Certificate, String> {
    let inst = ctx.inst;
    let Model::Prime { p, a, b } = &inst.rc.model else {
        return Err("prime model required".into());
    };
    let eval = |x: u32| -> BigUint {
        let x = BigUint::from(x);
        (&x * &x * &x + a * &x + b) % p
    };
    let c_choice = (0u32..4).find(|c| !eval(*c).is_zero());
    let mut checks = vec![Check::new(
        "construction_point_c_exists",
        if c_choice.is_some() { CheckResult::Pass } else { CheckResult::Fail },
        "some c in {0,1,2,3} has F(c) != 0 (a cubic has at most 3 roots), so H: v^2 = F(u^2 + c) is a genus-2 curve with a degree-2 map to E",
    )];
    let roots = cubic_root_count(p, a, b);
    let parity_even = (&inst.rc.order % 2u8).is_zero();
    checks.push(Check::new(
        "two_torsion_matches_order_parity",
        if (roots > 0) == parity_even {
            CheckResult::Pass
        } else {
            CheckResult::Fail
        },
        format!(
            "x^3+ax+b has {roots} root(s) in F_p, so #E(F_p)[2] = {}; #E is {}",
            roots + 1,
            if parity_even { "even" } else { "odd" }
        ),
    ));
    checks.push(Check::new(
        "executable_pullback_and_jacobian_arithmetic",
        CheckResult::NotRun,
        "H has two rational points at infinity (even-degree model); the repository's Mumford arithmetic is for one point at infinity (problems/prime-cover-balanced-jacobian-and-induced-maps-missing.md)",
    ));
    let coprime = (&inst.r % 2u8) != BigUint::zero();
    let mut rt = ctx.route(1.0, ctx.inst.field_bits());
    rt.construction = ConstructionCost::Priced(
        (12f64).log2() + ctx.model.log2_field_mult(inst.field_bits()),
        "choose c: at most 4 evaluations of a cubic, about 12 field multiplications".into(),
    );
    let costs = price_jacobian_route(ctx.model, &rt);
    let mut s = Spec::new(
        TransferType::GenusCover,
        "cover:prime_g2_deg2",
        "same-field genus-2 cover H -> E of degree 2 over F_p, v^2 = F(u^2 + c)",
        costs,
    );
    s.map = map_value(
        "pullback phi^*: E(F_p) -> Jac(H)(F_p)",
        "E(F_p)",
        "Jac(H)(F_p), genus 2, two points at infinity",
        json!({"cover_degree": 2, "genus": 2, "c": c_choice}),
        "F_p",
        Construction::ArithmeticNotExecuted,
        "(x, y) = (u^2 + c, v)",
    );
    s.subgroup = SubgroupPreservation {
        status: if coprime && c_choice.is_some() { Evidence::Arithmetic } else { Evidence::NotEvaluated },
        method: "arithmetic: phi_* phi^* = [2] on Pic^0(E) and gcd(2, r) = 1".into(),
        argument: "for a degree-2 morphism phi: H -> E the composite phi_* phi^* is multiplication by 2; r is an odd prime, so [2] is invertible on <G> and phi^* is injective on <G>. The identity is a theorem; the induced maps are not executed here".into(),
        image_order: None,
        kernel_meets_subgroup: coprime.then_some(false),
        antecedents: vec![
            Check::new("gcd(2,r)=1", if coprime { CheckResult::Pass } else { CheckResult::Fail }, "r is odd"),
            Check::new("cover_exists", if c_choice.is_some() { CheckResult::Pass } else { CheckResult::Fail }, "see construction_point_c_exists"),
        ],
        executed: None,
    };
    s.carried = coprime && c_choice.is_some();
    s.map_basis = MapBasis::Existence;
    s.existence = "supported";
    s.explicit_construction = "supported";
    s.map_correctness = "unknown";
    s.scaling_support = "unknown";
    s.checks = checks;
    s.invariants = jmap(vec![
        ("genus", json!(2)),
        ("degree", json!(2)),
        ("c", json!(c_choice)),
        (
            "rational_two_torsion",
            json!({
                "cubic_roots_in_Fp": roots,
                "E(Fp)[2]_order": roots + 1,
                "galois_action_on_E[2]": match roots { 0 => "cyclic of order 3 (irreducible cubic)", 1 => "transposition (one root, one conjugate pair)", _ => "trivial (full rational 2-torsion)" },
                "gluing_note": "a (2,2)-gluing of E with E' needs an anti-isometry E[2] ~ E'[2] of Galois modules; with irreducible cubic it needs E' with the same splitting field. Not constructed here.",
            }),
        ),
        ("destination", json!("Jac(H)(F_p), genus 2")),
        (
            "ic_leading_exponent_in_log2_q",
            json!(round3(ic_leading_exponent(2.0))),
        ),
        (
            "rho_leading_exponent_in_log2_q",
            json!(round3(0.5 * inst.log2_r() / inst.field_bits())),
        ),
    ]);
    s.open_obligations = vec![
        "checked arithmetic and pullback/norm maps for the two-infinity genus-2 model, with composition and independent replay (problems/prime-cover-balanced-jacobian-and-induced-maps-missing.md)".into(),
        "exact-order and image-order certificates for the catalog binding (problems/catalog-cover-subgroup-certificates-incomplete.md)".into(),
        "(2,2)-correspondence (gluing) candidates from the E[2] Galois structure: not evaluated".into(),
    ];
    s.references = vec![
        "docs/curves/COVERS.md (prime characteristic greater than 3)".into(),
        "crypto-autoresearcher ledger IDEA-20261009-275a14".into(),
    ];
    Ok(ctx.finish(s))
}

// ── Genus sweep: the recorded negative, as a check ───────────────────

fn genus_sweep(ctx: &Ctx) -> Certificate {
    let inst = ctx.inst;
    let fb = inst.field_bits();
    let log2_r = inst.log2_r();
    let mut rows = Vec::new();
    let mut all_exponent_gap = true;
    let mut all_above = true;
    let mut all_ic_above = true;
    let mut best: Option<(u32, CostTable)> = None;
    for g in 2u32..=8 {
        let lg = f64::from(g).log2();
        let mut rt = ctx.route(lg, fb);
        rt.construction =
            ConstructionCost::Priced(0.0, "best case: construction priced at zero".into());
        // Best case for the attacker: a cover defined over the subfield that
        // gives E its class keeps that class on the image.
        rt.image_class = ctx.base.class_size;
        let t = price_jacobian_route(ctx.model, &rt);
        let exp = ic_leading_exponent(f64::from(g));
        let ic_leading = exp * fb;
        let rho_leading = 0.5 * log2_r;
        all_exponent_gap &= ic_leading > rho_leading;
        let ratio = t.log2_cost_ratio.unwrap_or(f64::NAN);
        all_above &= ratio > 0.0;
        let ic_total = ctx.model.log2_hyperelliptic_ic(lg, fb);
        all_ic_above &= ic_total.is_some_and(|v| v > ctx.base.log2_total);
        rows.push(json!({
            "genus": g,
            "ic_exponent_2_minus_2_over_g": round3(exp),
            "ic_leading_log2": round3(ic_leading),
            "rho_leading_log2": round3(rho_leading),
            "ic_total_log2": ic_total.map(round3),
            "rho_on_jacobian_log2": round3(
                ctx.model.log2_rho_jacobian(log2_r, lg, fb) - class_discount_bits(ctx.base.class_size)
            ),
            "log2_total_best_solver": t.log2_total,
            "best_solver": t.destination_solver,
            "log2_cost_ratio_to_baseline": t.log2_cost_ratio,
        }));
        if best
            .as_ref()
            .is_none_or(|(_, b)| t.log2_total < b.log2_total)
        {
            best = Some((g, t));
        }
    }
    let (g_best, costs) = best.expect("sweep is nonempty");
    let mut s = Spec::new(
        TransferType::GenusCover,
        "cover:genus_sweep",
        "hypothetical genus-g covers over the curve's own field, g = 2..8 (pricing check)",
        costs,
    );
    s.map = map_value(
        "pullback into a genus-g Jacobian over the same field",
        "E(F_q)",
        "Jac(C)(F_q), g = 2..8",
        json!({"genus_range": [2, 8]}),
        "F_q",
        Construction::NotEvaluated,
        "none: formula sweep, not a construction",
    );
    s.subgroup = SubgroupPreservation::not_evaluated(
        "none",
        "no cover is constructed for g other than 2 (prime) or 3 (binary); the sweep prices the destination only",
    );
    s.checks = vec![
        Check::new(
            "ic_exponent_exceeds_rho_exponent_for_every_genus",
            if all_exponent_gap { CheckResult::Pass } else { CheckResult::Fail },
            "(2 - 2/g) log2 q > 0.5 log2 r for g = 2..8: the leading term of double-large-prime index calculus is at least log2 q while rho needs 0.5 log2 r <= 0.5 log2 q + 1",
        ),
        Check::new(
            "index_calculus_total_above_baseline_for_every_genus",
            if all_ic_above { CheckResult::Pass } else { CheckResult::Fail },
            "the double-large-prime total g! q + g^2 q^(2-2/g) Jacobian operations (frozen model) exceeds the matched rho for g = 2..8",
        ),
        Check::new(
            "priced_route_above_baseline_for_every_genus",
            if all_above { CheckResult::Pass } else { CheckResult::Fail },
            "the best destination solver over each genus costs more than the matched baseline",
        ),
    ];
    s.invariants = jmap(vec![
        ("sweep", json!(rows)),
        ("cheapest_genus", json!(g_best)),
        ("reproduces", json!("IDEA-20261009-275a14: genus-g covers of E over F_p cost p^(2 - 2/g) >= p, worse than rho; this is a check of that statement under the frozen model, not a new result")),
    ]);
    s.notes.push(
        "price = best case: construction priced at zero and rho on the image granted the curve's own equivalence class, so the true cost is higher".into(),
    );
    s.price_basis = PriceBasis::Hypothetical;
    s.references = vec![
        "crypto-autoresearcher ledger/proposals/IDEA-20261009-275a14.yaml (dominated by KR-IC-a4dd54, KR-IC-5ea2f8)".into(),
        "src/cryptanalysis/jacobian_transfer/cost.rs (Gaudry; Gaudry-Thome-Theriault-Diem)".into(),
    ];
    ctx.finish(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cubic_root_counts_match_hand_factorizations() {
        let p = BigUint::from(7u8);
        // x^3 - x = x(x-1)(x+1): three roots mod 7 (a = -1 = 6, b = 0).
        assert_eq!(
            cubic_root_count(&p, &BigUint::from(6u8), &BigUint::zero()),
            3
        );
        // x^3 + 1 = (x+1)(x^2 - x + 1); mod 7, x^2 - x + 1 has roots 3 and 5.
        assert_eq!(cubic_root_count(&p, &BigUint::zero(), &BigUint::one()), 3);
        // x^3 + x + 1 mod 5: values 1,3,11=1,31=1,69=4 -> no root.
        assert_eq!(
            cubic_root_count(&BigUint::from(5u8), &BigUint::one(), &BigUint::one()),
            0
        );
        // x^3 + 2 mod 7: cubes mod 7 are 0,1,6, so x^3 = 5 has no solution.
        assert_eq!(
            cubic_root_count(&p, &BigUint::zero(), &BigUint::from(2u8)),
            0
        );
        // x^3 + x mod 3 = x(x^2+1): one root (x^2 + 1 has none mod 3).
        assert_eq!(
            cubic_root_count(&BigUint::from(3u8), &BigUint::one(), &BigUint::zero()),
            1
        );
    }
}
