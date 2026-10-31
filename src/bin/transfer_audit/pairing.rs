//! Candidate family D: pairing transfer (MOV / Frey-Ruck).
//!
//! The Tate pairing sends `<G>` into the order-`r` subgroup of
//! `F_(q^k)^*` with `k = ord_r(q)` the embedding degree, and the discrete
//! logarithm of the image is the logarithm of the source.  The transfer helps
//! only if `k` is small enough that (pairing evaluation) + (the best
//! discrete-logarithm algorithm in `F_(q^k)`) beats rho on `<G>`.
//!
//! The embedding degree is exact when `r - 1` factors completely or `k` is
//! at most the search limit, and a verified lower bound otherwise.  The
//! price uses the smallest published `L[1/3]` constant with `o(1)` dropped,
//! deliberately attacker-favourable, and is not weighted by field-operation
//! cost, so it understates the route.

use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::json;

use crypto_lib::cryptanalysis::curve_traits::arith::{multiplicative_order, order_at_most};
use crypto_lib::cryptanalysis::curve_traits::Factoring;

use crate::certificate::{
    log2_add, map_value, round3, Certificate, Check, CheckResult, Construction, CostRow,
    CostStatus, CostTable, Evidence, SubgroupPreservation, TransferType,
};
use crate::cost::{log2_l13, LFS_C, LNFS_C, MILLER_MULTS_PER_BIT};
use crate::ctx::{jmap, Ctx, PriceBasis, Spec};
use crate::verdict::MapBasis;

/// Largest `k` tried by direct search (`curve_traits` uses the same bound).
pub const SEARCH_LIMIT: u64 = 1000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Embedding {
    /// `ord_r(q)` when known exactly.
    pub exact: Option<BigUint>,
    /// `k > lower_bound` when only a bound is known.
    pub lower_bound: Option<u64>,
    pub method: String,
}

/// `ord_r(q)` for a prime `r` not dividing `q`.  The least `k <= SEARCH_LIMIT`
/// with `r | q^k - 1` is exact (the scan is ascending); beyond it the order
/// comes from a complete factorization of `r - 1`, else only a lower bound.
pub fn embedding_degree(q: &BigUint, r: &BigUint, fx: &Factoring) -> Embedding {
    // `ord_r(q)` does not exist when r divides q.  Without this guard the
    // scan below finds nothing, the order routine is handed `q mod r = 0`
    // and the result is the bogus k = r - 1 (a "complete" answer).
    if (q % r).is_zero() {
        return Embedding {
            exact: None,
            lower_bound: None,
            method:
                "undefined: r divides q (the anomalous case r = q), so q is not a unit modulo r"
                    .into(),
        };
    }
    if let Some(k) = order_at_most(q, r, SEARCH_LIMIT) {
        return Embedding {
            exact: Some(BigUint::from(k)),
            lower_bound: None,
            method: format!("least k <= {SEARCH_LIMIT} with r | q^k - 1 (ascending scan)"),
        };
    }
    let rm1 = r - BigUint::one();
    let fac = fx.factor(&rm1);
    if fac.complete() {
        return Embedding {
            exact: Some(multiplicative_order(q, r, &fac)),
            lower_bound: None,
            method: "order of q modulo r from the complete factorization of r - 1".into(),
        };
    }
    Embedding {
        exact: None,
        lower_bound: Some(SEARCH_LIMIT),
        method: format!(
            "no k <= {SEARCH_LIMIT}; r - 1 is not completely factored within the budget and the committed hints, so only k > {SEARCH_LIMIT} is known"
        ),
    }
}

pub fn candidates(ctx: &Ctx) -> Result<Vec<Certificate>, String> {
    let inst = ctx.inst;
    let q = inst.q();
    let r = &inst.r;
    let log2_q = inst.field_bits();
    let log2_r = inst.log2_r();
    // The antecedent comes first: the embedding degree `ord_r(q)` is
    // undefined when r divides q (r = q is Smart's anomalous case), and the
    // pairing route does not apply.
    let r_coprime_q = !(&q % r).is_zero();
    if !r_coprime_q {
        return Ok(vec![undefined_embedding(ctx)]);
    }
    let emb = embedding_degree(&q, r, ctx.fx);
    // Cross-check against the repository's trait record.
    let trait_degree = ctx.traits.embedding.degree.clone();
    let trait_lb = ctx.traits.embedding.lower_bound;
    let agree = match (&emb.exact, &trait_degree, trait_lb) {
        (Some(k), Some(t), _) => k.to_string() == *t,
        (None, None, Some(lb)) => lb == SEARCH_LIMIT,
        _ => false,
    };

    // Price at the exact degree, or at the smallest degree consistent with
    // the bound (the cost is increasing in k, so this is a lower bound).
    let k_priced: f64 = match (&emb.exact, emb.lower_bound) {
        (Some(k), _) => k.to_f64().unwrap_or(f64::MAX),
        (None, Some(lb)) => (lb + 1) as f64,
        _ => 1.0,
    };
    let target_bits = k_priced * log2_q;
    let miller =
        (log2_r).log2() + MILLER_MULTS_PER_BIT.log2() + ctx.model.log2_field_mult(target_bits);
    let pairing_eval = miller + 1.0; // two evaluations: G and Q
                                     // A price band: the attacker-favourable end (smallest published constant,
                                     // o(1) dropped, operations unweighted) can only exclude a route; the
                                     // conservative end (generic NFS constant, bit operations) is the one
                                     // that may support a weakness verdict.
    let ic_fav = log2_l13(target_bits, LFS_C);
    let ic = log2_l13(target_bits, LNFS_C) + ctx.model.log2_field_mult(target_bits);
    let rho_f = ctx.model.log2_rho_elliptic(log2_r, log2_q)
        + ctx.model.log2_field_mult(target_bits)
        - ctx.model.log2_elliptic_op(log2_q);
    let dlp = ic.min(rho_f);
    let dlp_fav = ic_fav.min(rho_f);
    let verify = (1.5 * log2_r).log2() + ctx.model.log2_elliptic_op(log2_q);
    let total_fav = log2_add(log2_add(pairing_eval, dlp_fav), verify);
    let fav_ratio = total_fav - ctx.base.log2_total;

    let mut rows = vec![
        CostRow::absent(
            "map_construction",
            CostStatus::NotApplicable,
            "the pairing is defined by the curve; no auxiliary curve or cover is constructed (the auxiliary point of the Weil pairing is not needed for the Tate pairing)",
        ),
        CostRow::modeled(
            "transfer_evaluation",
            pairing_eval,
            "two pairing evaluations: log2(2 * log2(r) * 12 * M(k log2 q)); MILLER_MULTS_PER_BIT is a placeholder that never decides a verdict",
        ),
        CostRow::absent(
            "relation_collection",
            CostStatus::NotApplicable,
            "balanced with the linear algebra in L[1/3] algorithms; priced together below",
        ),
        CostRow::absent("linear_algebra", CostStatus::NotApplicable, "see relation_collection"),
        CostRow::modeled(
            "destination_solve_unsplit",
            dlp,
            format!(
                "conservative end of the band: min(L_N[1/3, {LNFS_C:.4}] with o(1) dropped, weighted by M(k log2 q) = 2^{ic:.1}; generic rho/Pohlig-Hellman in the order-r subgroup of F_(q^k)^* = 2^{rho_f:.1}); N = q^k. The attacker-favourable end (c = {LFS_C:.4}, unweighted) is 2^{ic_fav:.1}"
            ),
        ),
        CostRow::modeled(
            "descent_back_mapping",
            verify,
            "the logarithm of e(G,S)^d to base e(G,S) is d mod r: recovery is free; one scalar multiplication on E verifies it",
        ),
    ];
    if emb.exact.is_none() {
        rows.push(CostRow::absent(
            "exact_embedding_degree",
            CostStatus::NotEvaluated,
            format!("k > {SEARCH_LIMIT} only; priced at k = {}, and every cost term increases with k, so the total is a lower bound", SEARCH_LIMIT + 1),
        ));
    }
    let mut costs = CostTable::with_rows(rows, ctx.base.log2_total, ctx.base.log2_no_automorphisms);
    costs.destination_solver = Some(
        if ic <= rho_f {
            "IndexCalculusInFqk_L13_conservative"
        } else {
            "RhoInSubgroupOfFqk"
        }
        .into(),
    );
    costs.solvers_considered = vec![
        ("IndexCalculusInFqk_L13_conservative".into(), round3(ic)),
        ("IndexCalculusInFqk_L13_favourable".into(), round3(ic_fav)),
        ("RhoInSubgroupOfFqk".into(), round3(rho_f)),
    ];

    let antecedents = vec![
        Check::new("r_prime", CheckResult::Pass, "validated"),
        Check::new(
            "r_does_not_divide_q",
            if r_coprime_q {
                CheckResult::Pass
            } else {
                CheckResult::Fail
            },
            "gcd(q, r) = 1, so the embedding degree is defined",
        ),
        Check::new(
            "embedding_degree_exact",
            if emb.exact.is_some() {
                CheckResult::Pass
            } else {
                CheckResult::Indeterminate
            },
            emb.method.clone(),
        ),
        Check::new(
            "agrees_with_curve_traits_record",
            if agree {
                CheckResult::Pass
            } else {
                CheckResult::Fail
            },
            "curve_traits::compute reports the same embedding degree or the same lower bound",
        ),
    ];
    let status = if emb.exact.is_some() && r_coprime_q {
        Evidence::Arithmetic
    } else {
        Evidence::NotEvaluated
    };
    let (cons_total, cons_ratio) = (costs.log2_total, costs.log2_cost_ratio);
    let mut s = Spec::new(
        TransferType::PairingTransfer,
        "pairing:tate_mov_frey_ruck",
        "Tate/Weil pairing into F_(q^k)^*, k = embedding degree of r",
        costs,
    );
    s.map = map_value(
        "e_r(G, -): <G> -> mu_r subset F_(q^k)^*",
        "E(F_q)[r]",
        "F_(q^k)^*",
        json!({
            "embedding_degree": emb.exact.as_ref().map(|k| k.to_string()),
            "embedding_degree_lower_bound": emb.lower_bound,
        }),
        "F_(q^k)",
        Construction::NotEvaluated,
        "Miller's algorithm; no generic pairing is implemented in the repository (mov_attack ships k = 2 supersingular only)",
    );
    s.subgroup = SubgroupPreservation {
        status,
        method: "arithmetic: Frey-Ruck non-degeneracy of the Tate pairing at k = ord_r(q)".into(),
        argument: "for k the embedding degree the Tate-Lichtenbaum pairing is non-degenerate (Frey-Ruck 1994), so some S makes e(G, S) a primitive r-th root of unity: the image of <G> has order r. Not executed".into(),
        image_order: (status == Evidence::Arithmetic).then(|| r.to_string()),
        kernel_meets_subgroup: (status == Evidence::Arithmetic).then_some(false),
        antecedents,
        executed: None,
    };
    s.carried = emb.exact.is_some() && r_coprime_q;
    s.map_basis = if emb.exact.is_some() {
        MapBasis::Theorem
    } else {
        MapBasis::Existence
    };
    s.class_invariant = true;
    if fav_ratio < 0.0 && cons_ratio.is_some_and(|r| r >= 0.0) {
        s.lead_override = Some(crate::certificate::LeadClass::FormulaMarginal);
        // The band has ends on both sides of the baseline: the formula
        // cannot decide, so the route is neither refuted nor a weakness.
        s.price_basis = PriceBasis::Straddles;
    }
    s.existence = if emb.exact.is_some() {
        "supported"
    } else {
        "unknown"
    };
    s.explicit_construction = "unknown";
    s.map_correctness = "unknown";
    s.scaling_support = "unknown";
    // One element of F_(q^k) must be stored to evaluate a pairing at all.
    const TIB_BITS: f64 = 8.0 * 1.0995e12;
    s.checks = vec![Check::new(
        "pairing_target_element_storable",
        if target_bits <= TIB_BITS { CheckResult::Pass } else { CheckResult::Fail },
        format!(
            "one element of F_(q^k) has {target_bits:.3e} bits (1 TiB = {TIB_BITS:.2e} bits): above that the pairing cannot be evaluated and the price below is a formal extrapolation of the cost model"
        ),
    ), Check::new(
        "anomalous_curve",
        if ctx.traits.anomalous { CheckResult::Fail } else { CheckResult::Pass },
        "#E = q would put the curve in the scope of Smart's attack, which this tool does not cover",
    )];
    s.invariants = jmap(vec![
        ("embedding_degree", json!(emb.exact.as_ref().map(|k| k.to_string()))),
        ("embedding_degree_lower_bound", json!(emb.lower_bound)),
        ("embedding_method", json!(emb.method)),
        ("target_element_bits", json!(target_bits)),
        ("log2_ic_in_fqk_L13_conservative", json!(round3(ic))),
        ("log2_ic_in_fqk_L13_favourable", json!(round3(ic_fav))),
        ("price_band", json!({
            "favourable_log2_total": round3(total_fav),
            "favourable_log2_cost_ratio": round3(fav_ratio),
            "conservative_log2_total": cons_total,
            "conservative_log2_cost_ratio": cons_ratio,
            "reading": if fav_ratio >= 0.0 { "robustly above the matched rho: even the attacker-favourable price exceeds it" }
                else if cons_ratio.is_some_and(|r| r < 0.0) { "robustly below: even the conservative price is under the matched rho" }
                else { "straddles the matched rho: the formula band cannot decide; recorded as a marginal lead" },
        })),
        ("log2_rho_in_subgroup_of_fqk", json!(round3(rho_f))),
        ("class_invariant", json!("the embedding degree depends only on q and r, which every curve of the isogeny class shares")),
        ("lfs_c", json!(LFS_C)),
    ]);
    s.notes.push(format!(
        "reading of the verdict: it is relative. It compares the conservative price of a discrete logarithm in F_(q^k) (2^{:.1} operations: generic NFS constant, o(1) dropped, weighted by the field-multiplication cost) with the matched rho on <G> (2^{:.1}). `class-weak` means the first is below the second by at least the {:.0}-bit model margin, i.e. a conservative pairing price below rho under this model; it does not say the curve is weak at any absolute security level, and `no weakness found in scope` does not say it is strong",
        cons_total.unwrap_or(f64::NAN),
        ctx.base.log2_total,
        ctx.model.advantage_margin_bits
    ));
    s.open_obligations = vec![
        "implement a generic Tate pairing and execute it on the generator (only k = 2 supersingular exists in mov_attack.rs)".into(),
        "replace the L[1/3] lower bound by a priced index-calculus pipeline for F_(q^k) (quasi-polynomial algorithms for small characteristic are not modeled)".into(),
    ];
    if emb.exact.is_none() {
        s.open_obligations.push(
            "complete the factorization of r - 1 to fix k exactly; the price above is for k = 1001"
                .into(),
        );
    }
    s.references = vec![
        "Menezes-Okamoto-Vanstone 1993; Frey-Ruck 1994 (as cited in src/cryptanalysis/mov_attack.rs)".into(),
        "src/cryptanalysis/curve_traits (embedding degree record)".into(),
    ];
    Ok(vec![ctx.finish(s)])
}

/// `r` divides `q` (the anomalous case `r = q`): the embedding degree
/// `ord_r(q)` is undefined, the Tate pairing route does not apply, and the
/// certificate says so without pricing a route that does not exist.
fn undefined_embedding(ctx: &Ctx) -> Certificate {
    let costs = CostTable::unpriced(
        ctx.base.log2_total,
        "embedding degree undefined: r divides q, so there is no Tate-pairing route into F_(q^k)^*",
    );
    let mut s = Spec::new(
        TransferType::PairingTransfer,
        "pairing:tate_mov_frey_ruck",
        "Tate/Weil pairing into F_(q^k)^*, k = embedding degree of r",
        costs,
    );
    s.map = map_value(
        "e_r(G, -): <G> -> mu_r subset F_(q^k)^*",
        "E(F_q)[r]",
        "none",
        json!({"embedding_degree": null, "embedding_degree_lower_bound": null}),
        "n/a",
        Construction::NotApplicable,
        "none: r divides q, so mu_r is trivial in characteristic dividing r",
    );
    s.subgroup = SubgroupPreservation {
        status: Evidence::NotApplicable,
        method: "none".into(),
        argument: "r divides q: the order of q modulo r is undefined and there are no r-th roots of unity to map into; the anomalous case is the domain of Smart's p-adic attack, which this tool does not cover".into(),
        image_order: None,
        kernel_meets_subgroup: None,
        antecedents: vec![Check::new(
            "r_does_not_divide_q",
            CheckResult::Fail,
            "r divides q, so the embedding degree is undefined",
        )],
        executed: None,
    };
    s.existence = "not_applicable";
    s.explicit_construction = "not_applicable";
    s.map_correctness = "not_applicable";
    s.scaling_support = "not_applicable";
    s.advantage = Some("not_applicable");
    s.checks = vec![Check::new(
        "anomalous_curve",
        if ctx.traits.anomalous {
            CheckResult::Fail
        } else {
            CheckResult::NotRun
        },
        "#E = q puts the curve in the scope of Smart's attack, which this tool does not cover",
    )];
    s.invariants = jmap(vec![
        ("embedding_degree", json!(null)),
        ("embedding_degree_lower_bound", json!(null)),
        (
            "embedding_method",
            json!("undefined: r divides q (checked before ord_r(q) is computed)"),
        ),
    ]);
    s.references = vec!["Smart 1999; Semaev 1998; Satoh-Araki 1998 (anomalous curves)".into()];
    ctx.finish(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::bls12_381::fq::{modulus, scalar_modulus};

    fn fx() -> Factoring {
        Factoring::new(1 << 18, Vec::new())
    }

    /// The supersingular curve y^2 = x^3 + x over F_p, p = 3 (mod 4), has
    /// #E = p + 1.  For p = 10007 (prime, 10007 = 3 mod 4), #E = 10008 =
    /// 2^3 * 3^2 * 139 and r = 139 divides p + 1, so p = -1 (mod r) and
    /// the embedding degree is exactly 2: 139 does not divide p - 1 = 10006
    /// but divides p^2 - 1.
    #[test]
    fn embedding_degree_of_a_supersingular_toy_curve_is_two() {
        let p = BigUint::from(10007u32);
        // Naive point count with Euler's criterion, independent of the code under test.
        let mut count = 1u32; // infinity
        for x in 0u32..10007 {
            let rhs = (u64::from(x) * u64::from(x) % 10007 * u64::from(x) + u64::from(x)) % 10007;
            if rhs == 0 {
                count += 1;
            } else {
                let e = BigUint::from(rhs).modpow(&BigUint::from(5003u32), &p);
                if e.is_one() {
                    count += 2;
                }
            }
        }
        assert_eq!(count, 10008);
        let r = BigUint::from(139u32);
        assert_eq!(10008 % 139, 0);
        let emb = embedding_degree(&p, &r, &fx());
        assert_eq!(emb.exact, Some(BigUint::from(2u32)));
        assert!(!((&p - 1u8) % &r).is_zero());
        assert!(((&p * &p - 1u8) % &r).is_zero());
    }

    /// BLS12-381 is the standard pairing-friendly curve: k = 12.
    #[test]
    fn embedding_degree_of_bls12_381_is_twelve() {
        let emb = embedding_degree(&modulus(), &scalar_modulus(), &fx());
        assert_eq!(emb.exact, Some(BigUint::from(12u32)));
    }

    /// A subgroup whose order divides q - 1 has k = 1 (the anomalous-like
    /// "trivial pairing" case): r = 5 divides 11 - 1.
    #[test]
    fn embedding_degree_one_when_r_divides_q_minus_one() {
        let emb = embedding_degree(&BigUint::from(11u32), &BigUint::from(5u32), &fx());
        assert_eq!(emb.exact, Some(BigUint::from(1u32)));
    }

    /// Past the search limit with r - 1 not completely factored, only a lower
    /// bound is reported; a committed factor hint completes it.  r = 2 * 65537 *
    /// p2 + 1 with both factors above the trial-division bound 2^16, so a
    /// zero rho budget leaves r - 1 unsplit.  The order of 2 divides r - 1 and
    /// is not 1 or 2, hence at least 65537 > 1000.
    #[test]
    fn unfactored_large_order_gives_a_lower_bound_then_an_exact_value_with_hints() {
        use crypto_lib::cryptanalysis::curve_traits::arith::is_prime;
        let p1 = BigUint::from(65537u32);
        let (p2, r) = (65539u32..70000)
            .filter_map(|c| {
                let p2 = BigUint::from(c);
                let r = BigUint::from(2u8) * &p1 * &p2 + 1u8;
                (is_prime(&p2) && is_prime(&r)).then_some((p2, r))
            })
            .next()
            .expect("a prime r = 2 p1 p2 + 1 exists in the range");
        let q = BigUint::from(2u8);
        let blind = Factoring::new(0, Vec::new());
        let emb = embedding_degree(&q, &r, &blind);
        assert_eq!(emb.exact, None);
        assert_eq!(emb.lower_bound, Some(SEARCH_LIMIT));
        let hinted = Factoring::new(0, vec![p1.clone(), p2.clone()]);
        let emb = embedding_degree(&q, &r, &hinted);
        let k = emb.exact.expect("complete factorization gives the order");
        let rm1 = &r - 1u8;
        assert!((&rm1 % &k).is_zero());
        assert!(q.modpow(&k, &r).is_one());
        assert!(k > BigUint::from(SEARCH_LIMIT));
    }

    /// `ord_r(q)` does not exist when r divides q.  The guard returns "no
    /// answer" instead of the bogus k = r - 1 the order routine produced
    /// from `q mod r = 0`.
    #[test]
    fn embedding_degree_is_undefined_when_r_divides_q() {
        for (q, r) in [(11u32, 11u32), (25, 5), (1009, 1009)] {
            let emb = embedding_degree(&BigUint::from(q), &BigUint::from(r), &fx());
            assert_eq!((&emb.exact, emb.lower_bound), (&None, None), "q={q} r={r}");
            assert!(emb.method.starts_with("undefined"), "{}", emb.method);
        }
    }

    /// An anomalous curve (#E = p, so r = q = p) over a small prime field:
    /// the whole pairing candidate is the unpriced `not_applicable`
    /// certificate, decided before any embedding degree is computed.
    #[test]
    fn an_anomalous_curve_gets_the_not_applicable_pairing_certificate() {
        use crate::certificate::{RouteStatus, Verdict};
        use crate::spec::Instance;
        use crypto_lib::cryptanalysis::curve_traits::{
            GeneratorCoords, Model, RegistryCurve, Representation, Source,
        };
        use num_bigint::BigInt;
        use std::collections::BTreeMap;

        let p = 1009u64;
        let pb = BigUint::from(p);
        let legendre = |v: u64| BigUint::from(v).modpow(&BigUint::from((p - 1) / 2), &pb);
        let rhs = |a: u64, b: u64, x: u64| (x * x % p * x + a * x + b) % p;
        let order = |a: u64, b: u64| {
            1 + (0..p)
                .map(|x| {
                    let v = rhs(a, b, x);
                    if v == 0 {
                        1
                    } else if legendre(v).is_one() {
                        2
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        };
        let (a, b) = (0..p)
            .flat_map(|a| (1..p).map(move |b| (a, b)))
            .find(|&(a, b)| !(4 * a * a % p * a + 27 * b * b).is_multiple_of(p) && order(a, b) == p)
            .expect("a curve with #E = p exists over F_1009");
        let (x, y) = (0..p)
            .find_map(|x| {
                let v = rhs(a, b, x);
                (0..p).find(|y| y * y % p == v).map(|y| (x, y))
            })
            .unwrap();
        let rc = RegistryCurve {
            slug: "toy-anomalous-p1009".into(),
            family: "weierstrass".into(),
            model: Model::Prime {
                p: pb.clone(),
                a: BigUint::from(a),
                b: BigUint::from(b),
            },
            trace: BigInt::from(1),
            order: pb.clone(),
            end: None,
            representations: vec![Representation {
                generator: GeneratorCoords::Affine(BigUint::from(x), BigUint::from(y)),
                subgroup_order: pb.clone(),
                cofactor: BigUint::one(),
            }],
            levels: BTreeMap::new(),
            source: Source {
                kind: "registry".into(),
                path: None,
                curve_uid: None,
                registered: false,
            },
        };
        let inst = Instance::from_registry_curve("toy-anomalous", rc).unwrap();
        let model = crypto_lib::cryptanalysis::jacobian_transfer::cost::CostModel::default();
        let f = fx();
        let ctx = Ctx::new(&inst, &model, &f, 7).unwrap();
        assert!(ctx.traits.anomalous);
        let certs = candidates(&ctx).unwrap();
        assert_eq!(certs.len(), 1);
        let c = &certs[0];
        assert_eq!(c.subgroup_preservation.status, Evidence::NotApplicable);
        assert_eq!(c.verdict, Verdict::NoWeaknessFoundInScope);
        assert_eq!(c.evaluation.status, RouteStatus::Excluded);
        assert_eq!(c.obligations.computational_advantage, "not_applicable");
        assert!(c.costs.log2_total.is_none());
        assert!(!c.carried);
        assert_eq!(c.invariants["embedding_degree"], serde_json::Value::Null);
    }
}
