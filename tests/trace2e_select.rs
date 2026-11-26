//! `binary-subspace select=trace2e`: the prefix (or explicit) subspace cut to
//! the abscissae with `Tr(x) = Tr(a)`, i.e. the points of `2E`
//! (`research/trace2e_builder_20261125/`).  An integration test so it links
//! against the library build instead of compiling the library's own test
//! harness.

use std::collections::BTreeSet;

use crypto_lib::cryptanalysis::ic_boundary::{
    koblitz_instance, random_binary_instance, BinaryGroup, BinaryInstance, Calibration,
    CountedGroup, FactorBase, GroupOps,
};
use crypto_lib::cryptanalysis::ic_framework::plugins::{
    trace_cut, trace_kernel_basis, BinarySubspaceBase, MitmOracle,
};
use crypto_lib::cryptanalysis::ic_framework::stages::{
    FactorBaseBuilder, InstanceCtx, Params, Targets,
};
use crypto_lib::cryptanalysis::ic_framework::{run_pipeline, PipelineSpec};
use crypto_lib::cryptanalysis::koblitz_fast::FastPoint;

fn ctx<'a>(inst: &'a BinaryInstance, g: &'a BinaryGroup<'a>) -> InstanceCtx<'a, BinaryGroup<'a>> {
    InstanceCtx {
        group: g,
        generator: inst.generator,
        target: inst.generator,
        r: inst.r,
        cofactor: inst.cofactor,
        group_order: inst.group_order,
        name: inst.name.clone(),
        field_degree: Some(inst.n),
    }
}

fn build(inst: &BinaryInstance, recipe: &str) -> Result<FactorBase<FastPoint>, String> {
    let g = BinaryGroup(&inst.fast);
    let (_, params) = Params::parse_spec(recipe).unwrap();
    BinarySubspaceBase { instance: inst }.build(&ctx(inst, &g), &params, &mut GroupOps::default())
}

/// The cut by brute force: every non-zero `x` in `span(basis)` with a
/// curve point and `Tr(x) = Tr(a)`.
fn brute_cut(inst: &BinaryInstance, basis: &[u64]) -> BTreeSet<u64> {
    let tr = |v: u64| inst.artin_schreier.trace(v);
    (1u64..1 << basis.len())
        .map(|idx| {
            basis
                .iter()
                .enumerate()
                .filter(|(j, _)| idx >> j & 1 == 1)
                .fold(0, |x, (_, &e)| x ^ e)
        })
        .filter(|&x| x != 0 && tr(x) == tr(inst.a) && !inst.points_with_x(x).is_empty())
        .collect()
}

/// **`select=trace2e` is the prefix subspace cut to `Tr x = Tr a`**:
/// on the panel Koblitz curves of the workspace C1 measurement (and at
/// the D it froze), the base's abscissae are exactly the brute-force
/// cut, i.e. exactly the `explicit-x` list that measurement used; each
/// abscissa is one column with both signs, every point lies in 2E,
/// and the builder enumerates only the cut (no excluded abscissa is
/// solved for).
#[test]
fn trace2e_is_the_prefix_subspace_cut_to_two_e() {
    for (a, n, d) in [
        (1u8, 17u32, 6u32),
        (0, 19, 7),
        (1, 19, 7),
        (0, 23, 8),
        (1, 23, 8),
        (0, 13, 5),
    ] {
        let inst = koblitz_instance(a, n).expect("a Koblitz curve");
        let fb = build(
            &inst,
            &format!("binary-subspace:dimension={d},select=trace2e"),
        )
        .unwrap();
        let prefix: Vec<u64> = (0..d).map(|k| 1u64 << k).collect();
        let want = brute_cut(&inst, &prefix);
        let got: BTreeSet<u64> = fb.points.iter().map(|p| p.x).collect();
        assert_eq!(got, want, "K{a}-{n} D={d}");
        assert_eq!(fb.abscissae, want.len(), "one column per abscissa");
        assert_eq!(fb.columns, fb.abscissae);
        assert_eq!(fb.dimension, Some(d - 1), "the cut has dimension D - 1");
        // Only the coset is enumerated: 2^(D-1) abscissae, less the
        // zero one when the coset is the linear kernel (K0).
        let coset = (1u64 << (d - 1)) - u64::from(a == 0);
        assert_eq!(
            fb.cost.get("as_solves"),
            coset,
            "K{a}-{n}: scanned outside the cut"
        );
        assert_eq!(fb.cost.get("abscissae_scanned"), coset);
        let g = BinaryGroup(&inst.fast);
        let mut ops = GroupOps::default();
        let half = inst.group_order / 2;
        assert!(
            fb.points.iter().all(|&p| g.mul(&mut ops, p, half).infinity),
            "K{a}-{n}: a base point outside 2E"
        );
        // A K1 curve records an affine coset, a K0 curve a linear one.
        assert_eq!(fb.affine_subspace.is_some(), a == 1, "K{a}-{n}");
    }
}

/// The cut works on a non-Koblitz curve, on an explicit basis, and at
/// an even degree, where `Tr(1) = 0` and the pivot is not `1`.
#[test]
fn trace2e_handles_explicit_bases_and_even_degree() {
    for (n, seed) in [(13u32, 3u64), (12, 5), (14, 2)] {
        let inst = (seed..seed + 64)
            .find_map(|s| random_binary_instance(n, s, 8))
            .expect("a binary curve");
        let prefix: Vec<u64> = (0..6).map(|k| 1u64 << k).collect();
        let fb = build(&inst, "binary-subspace:dimension=6,select=trace2e").unwrap();
        let got: BTreeSet<u64> = fb.points.iter().map(|p| p.x).collect();
        assert_eq!(got, brute_cut(&inst, &prefix), "n = {n} prefix");
        let basis = [0x3u64, 0x5, 0x18, 0x21, 0x40];
        let fb = build(
            &inst,
            "binary-subspace:dimension=5,basis=0x3;0x5;0x18;0x21;0x40,select=trace2e",
        )
        .unwrap();
        let got: BTreeSet<u64> = fb.points.iter().map(|p| p.x).collect();
        assert_eq!(got, brute_cut(&inst, &basis), "n = {n} explicit");
    }
}

/// `trace_cut` alone: a basis on which the trace vanishes keeps the
/// whole span when `Tr(a) = 0` and is refused when `Tr(a) = 1`; the
/// builder refuses dimension 1 and unknown selections.
#[test]
fn trace_cut_edge_cases_and_refusals() {
    let k0 = koblitz_instance(0, 13).unwrap();
    let k1 = koblitz_instance(1, 17).expect("a = 1 over F_2^17 (n = 13 has no a = 1 instance)");
    // Four independent elements of ker Tr on K1-17's field (monomials of
    // trace 0, or sums of two monomials of trace 1).
    let tr = |v: u64| k1.artin_schreier.trace(v);
    let ones: Vec<u64> = (0..17).map(|k| 1u64 << k).filter(|&e| tr(e) == 1).collect();
    let k1_kernel: Vec<u64> = (0..17)
        .map(|k| 1u64 << k)
        .map(|e| if tr(e) == 1 { e ^ ones[0] } else { e })
        .filter(|&e| e != 0)
        .take(4)
        .collect();
    assert!(k1_kernel.iter().all(|&e| tr(e) == 0));
    let kernel = trace_kernel_basis(&k0, 4).unwrap();
    assert_eq!(trace_cut(&k0, &kernel).unwrap(), (kernel.clone(), 0));
    assert!(
        trace_cut(&k1, &k1_kernel).is_err(),
        "Tr = 0 on the span and Tr(a) = 1"
    );
    assert!(build(&k0, "binary-subspace:dimension=1,select=trace2e").is_err());
    assert!(build(&k0, "binary-subspace:dimension=4,select=trace4e").is_err());
    assert!(build(
        &k0,
        "binary-subspace:dimension=3,basis=1;2;4,select=strict4e"
    )
    .is_err());
}

/// **End to end**: the pipeline with a `trace2e` base and the plain
/// MITM oracle recovers a planted logarithm on cofactor-2 and
/// cofactor-4 Koblitz curves.
#[test]
fn trace2e_base_recovers_a_planted_logarithm() {
    for (a, n, d) in [(1u8, 17u32, 6u32), (0, 19, 7)] {
        let inst = koblitz_instance(a, n).unwrap();
        let g = BinaryGroup(&inst.fast);
        let planted = 1 + 123_457 % (inst.r - 1);
        let mut ops = GroupOps::default();
        let mut c = ctx(&inst, &g);
        c.target = g.mul(&mut ops, inst.generator, planted);
        let mut spec = PipelineSpec {
            factor_base: "binary-subspace".into(),
            oracle: "mitm".into(),
            targets: Targets::Walk,
            max_trials: 2_000_000,
            seed: 7,
            ..Default::default()
        };
        spec.factor_base_params.set("dimension", &d.to_string());
        spec.factor_base_params.set("select", "trace2e");
        let report = run_pipeline(
            &c,
            &spec,
            &BinarySubspaceBase { instance: &inst },
            &mut MitmOracle::new(2),
            planted,
            &Calibration::default(),
            None,
        )
        .expect("the configuration should run");
        assert!(
            report.verified,
            "K{a}-{n}: did not recover the planted logarithm"
        );
        assert_eq!(report.recovered, Some(planted));
    }
}
