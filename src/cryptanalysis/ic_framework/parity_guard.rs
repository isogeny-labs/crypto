//! Coset-parity admission check: the K1 odd-`m` parity trap.
//!
//! On an ordinary binary curve `y² + xy = x³ + ax² + b` the 2-part of
//! `E(F_{2^n})` is cyclic, so `E / 2E ≅ Z/2`, and a point lies in `2E`
//! exactly when `Tr(x) = Tr(a)` ([`CountedGroup::coset_parity`]).  The
//! class map is a homomorphism that commutes with negation and with the
//! Frobenius, so a relation `Σ_{i=1..m} ±P_i = R` forces
//! `Σ class(P_i) = class(R)` in `Z/2`.
//!
//! The targets a pipeline decomposes are `[a]G + [b]Q` in the odd prime
//! order subgroup, hence class 0.  If every base point has the same class
//! `c`, a relation with `m` unit summands has class `m·c`; when that is
//! not 0 the base yields **exactly zero relations**, however long the run.
//!
//! The case that bites is K1 (`a = 1`, cofactor 2) with an `(x + 1)`-free
//! Frobenius-invariant divisor base (`koblitz-orbit:divisor=i`, `i ≠ 0`):
//! the subspace lies in `ker Tr`, `Tr(a) = 1`, so every base point is
//! outside `2E` and any odd `m` collects nothing, while `m = 2` is safe
//! (TZ-1, `docs/notes/trace-frobenius-primer-20261127/tz1/README.md` in
//! the trunk: at n = 13, m = 3 gave 0 relations and m = 2 twice the
//! generic yield).  The same argument covers K0 with a base inside the
//! translate `Tr(x) = 1`.
//!
//! Only the mod-2 class is read.  On K0 (`E(F_2) ≅ Z/4`) the finer class
//! in `E / 4E` would also catch a base entirely in the class of order 2,
//! but reading it needs a point halving per base point; it is not done
//! here.
//!
//! The check is an admission test, like the factor-base index checks
//! beside it in the runner: at most `|base| + 2` trace evaluations
//! (`2(n − 1)` field squarings each), stopping at the first second
//! class, outside the counted ledger.  It changes no result of a run it
//! admits.

use crate::cryptanalysis::ic_boundary::{CountedGroup, FactorBase};

use super::stages::InstanceCtx;

/// What the check read from a base.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParityVerdict {
    /// The group cannot read the class without a group operation, the
    /// generator or target is outside `2E`, or the base is empty: no
    /// statement is made.
    NotApplicable,
    /// The base meets both classes of `E / 2E`.
    Mixed,
    /// Every base point has class `class`, and `m · class` equals the
    /// targets' class: relations are possible.
    SingleClassReachable { class: u8 },
    /// Every base point has class `class`, and `m · class` differs from
    /// the targets' class: the base yields no relation at this `m`.
    Trapped { class: u8, points: usize },
}

/// Read the `E / 2E` classes of the base and decide whether a relation
/// of `m` unit summands can reach a target in `⟨G, Q⟩`.
pub fn coset_parity_verdict<G: CountedGroup>(
    ctx: &InstanceCtx<G>,
    fb: &FactorBase<G::Elt>,
    m: u32,
) -> ParityVerdict {
    let group = ctx.group;
    // The targets are `[a]G + [b]Q`; their class is fixed only when both
    // generator and target are in `2E` (always, for `r` odd and G, Q in
    // the order-r subgroup).
    match (
        group.coset_parity(&ctx.generator),
        group.coset_parity(&ctx.target),
    ) {
        (Some(0), Some(0)) => {}
        _ => return ParityVerdict::NotApplicable,
    }
    let mut class = None;
    for p in &fb.points {
        let Some(c) = group.coset_parity(p) else {
            return ParityVerdict::NotApplicable;
        };
        match class {
            None => class = Some(c),
            Some(seen) if seen != c => return ParityVerdict::Mixed,
            Some(_) => {}
        }
    }
    let Some(class) = class else {
        return ParityVerdict::NotApplicable;
    };
    if (u64::from(m) * u64::from(class)) % 2 == 0 {
        ParityVerdict::SingleClassReachable { class }
    } else {
        ParityVerdict::Trapped {
            class,
            points: fb.points.len(),
        }
    }
}

/// [`coset_parity_verdict`] as the runner's admission check: `Err` with
/// the trap and its fixes when the base cannot yield a relation.
pub fn check_coset_parity<G: CountedGroup>(
    ctx: &InstanceCtx<G>,
    fb: &FactorBase<G::Elt>,
    m: u32,
    base_name: &str,
    oracle_name: &str,
) -> Result<ParityVerdict, String> {
    let verdict = coset_parity_verdict(ctx, fb, m);
    if let ParityVerdict::Trapped { class, points } = verdict {
        return Err(format!(
            "coset-parity trap: all {points} points of factor base `{base_name}` lie in class \
             {class} of E/2E (Tr(x) != Tr(a)), the targets lie in 2E (class 0), and every \
             relation of `{oracle_name}` has m = {m} unit summands, so its class is \
             {m}·{class} = 1 != 0: this base yields exactly zero relations for every odd m. \
             Fix: use the affine translate W0 + 1 of the abscissa subspace (Tr(x) = Tr(a)), \
             an even m, or a mixed divisor that includes index 0 (the factor x + 1). On K1 an \
             (x + 1)-free koblitz-orbit divisor is the usual cause."
        ));
    }
    Ok(verdict)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ic_boundary::{
        koblitz_instance, BinaryGroup, BinaryInstance, Calibration, GroupOps, OracleCounters,
    };
    use crate::cryptanalysis::ic_framework::plugins::{FrobeniusMitmOracle, KoblitzOrbitBase};
    use crate::cryptanalysis::ic_framework::stages::{
        DecompositionOracle, FactorBaseBuilder, Params, Targets,
    };
    use crate::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec};
    use crate::cryptanalysis::koblitz_fast::FastPoint;

    fn ctx<'a>(
        inst: &BinaryInstance,
        g: &'a BinaryGroup<'a>,
        target: FastPoint,
    ) -> InstanceCtx<'a, BinaryGroup<'a>> {
        InstanceCtx {
            group: g,
            generator: inst.generator,
            target,
            r: inst.r,
            cofactor: inst.cofactor,
            group_order: inst.group_order,
            name: inst.name.clone(),
            field_degree: Some(inst.n),
        }
    }

    fn orbit_base(
        inst: &BinaryInstance,
        c: &InstanceCtx<'_, BinaryGroup<'_>>,
        divisor: &str,
    ) -> FactorBase<FastPoint> {
        let (_, params) = Params::parse_spec(&format!("koblitz-orbit:divisor={divisor}")).unwrap();
        KoblitzOrbitBase { instance: inst }
            .build(c, &params, &mut GroupOps::default())
            .unwrap()
    }

    /// K1, n = 11 (r = 991; x¹¹ − 1 = (x + 1)·Φ₁₁ over F₂): `divisor=1` is
    /// the (x + 1)-free invariant subspace, i.e. ker Tr, so with Tr(a) = 1
    /// every base point is outside 2E.  Odd m is trapped, even m is not.
    /// (TZ-1 measured the
    /// trap at n = 13, where K1's r = 53·79 is not prime and the library
    /// registers no instance.)  K1 n = 17 repeats it on a proper invariant
    /// subspace (dimension 8) and checks that
    /// the mixed divisors `0;1`, `0;2` are not single-class.
    #[test]
    fn k1_n11_x_plus_1_free_divisor_traps_odd_m_only() {
        let inst = koblitz_instance(1, 11).expect("K1 n11");
        assert_eq!(inst.cofactor, 2);
        let g = BinaryGroup(&inst.fast);
        let c = ctx(&inst, &g, inst.generator);
        let pure = orbit_base(&inst, &c, "1");
        assert!(!pure.points.is_empty());
        for m in [3u32, 5, 7, 9] {
            assert_eq!(
                coset_parity_verdict(&c, &pure, m),
                ParityVerdict::Trapped {
                    class: 1,
                    points: pure.points.len()
                },
                "m = {m}"
            );
            let err =
                check_coset_parity(&c, &pure, m, "koblitz-orbit", "mitm-frobenius").unwrap_err();
            assert!(
                err.contains("coset-parity trap") && err.contains("W0 + 1"),
                "{err}"
            );
        }
        for m in [2u32, 4, 6, 8] {
            assert_eq!(
                coset_parity_verdict(&c, &pure, m),
                ParityVerdict::SingleClassReachable { class: 1 },
                "m = {m}"
            );
        }
        // n = 11 has no mixed proper divisor: x^11 - 1 = (x + 1) * (degree 10),
        // so `0;1` would be the whole field. The mixed case is checked at n = 17.

        let inst17 = koblitz_instance(1, 17).expect("K1 n17");
        let g17 = BinaryGroup(&inst17.fast);
        let c17 = ctx(&inst17, &g17, inst17.generator);
        for divisor in ["1", "2"] {
            let fb = orbit_base(&inst17, &c17, divisor);
            assert!(matches!(
                coset_parity_verdict(&c17, &fb, 3),
                ParityVerdict::Trapped { class: 1, .. }
            ));
            assert_eq!(
                coset_parity_verdict(&c17, &fb, 2),
                ParityVerdict::SingleClassReachable { class: 1 }
            );
            let mixed = orbit_base(&inst17, &c17, &format!("0;{divisor}"));
            assert_eq!(coset_parity_verdict(&c17, &mixed, 3), ParityVerdict::Mixed);
        }
    }

    /// The trap is real, not only asserted: on the same base the
    /// three-summand Frobenius probe finds no decomposition of any of 64
    /// targets in G, while the two-summand probe finds many.
    #[test]
    fn k1_n11_pure_base_m3_finds_no_relation_m2_does() {
        let inst = koblitz_instance(1, 11).expect("K1 n11");
        let g = BinaryGroup(&inst.fast);
        let c = ctx(&inst, &g, inst.generator);
        let fb = orbit_base(&inst, &c, "1");
        let mut found = [0u32; 2];
        for (slot, m) in [(0usize, 2u32), (1, 3)] {
            let mut oracle = FrobeniusMitmOracle::new(m, &inst);
            let mut ops = GroupOps::default();
            oracle
                .prepare(&c, &fb, &Params::default(), &mut ops)
                .unwrap();
            let mut counters = OracleCounters::default();
            for k in 1..=64u64 {
                let target = g.mul(&mut ops, inst.generator, 37 * k + 5);
                if let Some(idx) = oracle.decompose(&c, &fb, &mut ops, &mut counters, target) {
                    assert_eq!(idx.len(), m as usize);
                    found[slot] += 1;
                }
            }
        }
        assert!(found[0] > 0, "m = 2 found no relation on the pure base");
        assert_eq!(found[1], 0, "m = 3 found a relation on a trapped base");
    }

    /// The runner refuses the trapped configuration before the oracle's
    /// table is built, and admits (and solves) the same base at m = 2.
    #[test]
    fn k1_n11_pipeline_refuses_m3_and_solves_m2() {
        let inst = koblitz_instance(1, 11).expect("K1 n11");
        let g = BinaryGroup(&inst.fast);
        let planted = 1234 % inst.r;
        let q = g.mul(&mut GroupOps::default(), inst.generator, planted);
        let c = ctx(&inst, &g, q);
        let builder = KoblitzOrbitBase { instance: &inst };
        let run = |m: u32| {
            let (_, fb_params) = Params::parse_spec("koblitz-orbit:divisor=1").unwrap();
            let spec = PipelineSpec {
                factor_base: "koblitz-orbit".into(),
                factor_base_params: fb_params,
                oracle: "mitm-frobenius".into(),
                targets: Targets::Random,
                linalg: "incremental-gauss-full-rank-checked".into(),
                max_trials: 200_000,
                seed: 11,
                ..Default::default()
            };
            let mut oracle = FrobeniusMitmOracle::new(m, &inst);
            run_pipeline(
                &c,
                &spec,
                &builder,
                &mut oracle,
                planted,
                &Calibration::default(),
                None,
            )
        };
        let err = run(3).unwrap_err();
        assert!(err.contains("coset-parity trap"), "{err}");
        let report = run(2).expect("m = 2 is admitted");
        assert!(report.verified, "m = 2 did not recover the planted log");
    }
}
