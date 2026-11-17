//! Fixed-oracle accounting and exact106-row scientific regression, all native.
use super::*;
use crate::prime_cost16::{assert_idle, capture, Budget, Snapshot};
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::panic::{catch_unwind, AssertUnwindSafe};

const REFERENCE: &str = include_str!("../original-complete15B-scientific-snapshot.stdout");

fn poly(coefficients: &[u64], p: u64) -> FpPoly {
    FpPoly::from_coeffs(
        coefficients.iter().copied().map(BigUint::from).collect(),
        BigUint::from(p),
    )
}

fn boundary_reset(field: &Field) {
    field.k.mul_calls15.set(0);
    field.k.nonzero_mul_calls15.set(0);
    field.k.inverse_calls15.set(0);
    field.shortcut_calls15.set(0);
    field.general_calls15.set(0);
}

fn boundary(field: &Field) -> [u64; 5] {
    [
        field.k.mul_calls15.get(),
        field.k.nonzero_mul_calls15.get(),
        field.k.inverse_calls15.get(),
        field.shortcut_calls15.get(),
        field.general_calls15.get(),
    ]
}

fn encoding(field: &Field, value: &Element) -> [u64; 2] {
    [field.k.encoding(&value.a), field.k.encoding(&value.b)]
}

struct Calibrations {
    count: usize,
    sites: u32,
}

impl Calibrations {
    fn check<T: Debug>(
        &mut self,
        label: &str,
        value: &T,
        actual: Snapshot,
        expected: [u64; 6],
        histogram: &[(usize, u64)],
    ) {
        actual.validate();
        assert_eq!(actual.all.ops, expected, "fixed oracle {label}");
        assert_eq!(actual.all.sparse_inverses(), histogram);
        assert_eq!(actual.inverse, Budget::zero());
        for (i, n) in actual.all.sites.iter().enumerate() {
            if *n != 0 {
                self.sites |= 1 << i;
            }
        }
        self.count += 1;
        println!("CT16|calibration|label={label}|value={value:?}|ops={:?}|inv_inputs={:?}|sites={:?}|inverse_scope_empty=true",actual.all.ops,actual.all.sparse_inverses(),actual.all.sites);
    }
}

fn calibrate(cubic: &CubicField) {
    let p = BigUint::from(7_u64);
    let x = poly(&[2, 3], 7);
    let y = poly(&[4, 1], 7);
    let one = FpPoly::one(p.clone());
    let two = BigUint::from(2_u64);
    let mut c = Calibrations { count: 0, sites: 0 };
    let (out, stats) = capture(false, || FpPoly::constant(BigUint::from(9_u64), p.clone()));
    assert_eq!(out, poly(&[2], 7));
    c.check("constant9", &out, stats, [0, 0, 0, 1, 0, 0], &[]);
    let (out, stats) = capture(false, || poly(&[8, 0, 9], 7));
    assert_eq!(out, poly(&[1, 0, 2], 7));
    c.check("coefficients", &out, stats, [0, 0, 0, 3, 0, 0], &[]);
    let (out, stats) = capture(false, || x.add(&y));
    assert_eq!(out, poly(&[6, 4], 7));
    c.check("add", &out, stats, [0, 2, 0, 2, 0, 0], &[]);
    let (out, stats) = capture(false, || x.sub(&y));
    assert_eq!(out, poly(&[5, 2], 7));
    c.check("subtract", &out, stats, [0, 2, 2, 2, 0, 0], &[]);
    let (out, stats) = capture(false, || x.neg());
    assert_eq!(out, poly(&[5, 4], 7));
    c.check("negate", &out, stats, [0, 0, 2, 2, 0, 0], &[]);
    let (out, stats) = capture(false, || x.mul(&y));
    assert_eq!(out, poly(&[1, 0, 3], 7));
    c.check("multiply", &out, stats, [4, 4, 0, 8, 0, 0], &[]);
    let (out, stats) = capture(false, || x.scalar_mul(&two));
    assert_eq!(out, poly(&[4, 6], 7));
    c.check("scalar", &out, stats, [2, 0, 0, 3, 0, 0], &[]);
    let (out, stats) = capture(false, || x.eval(&two));
    assert_eq!(out, BigUint::from(1_u64));
    c.check("Horner", &out, stats, [2, 2, 0, 2, 0, 0], &[]);
    let (out, stats) = capture(false, || crate::fp_poly::fp_inv(&two, &p));
    assert_eq!(out, Some(BigUint::from(4_u64)));
    c.check("inverse2", &out, stats, [0, 0, 1, 1, 1, 0], &[(2, 1)]);
    let (out, stats) = capture(false, || crate::fp_poly::fp_inv(&BigUint::from(0_u64), &p));
    assert_eq!(out, None);
    c.check("inverse0", &out, stats, [0, 0, 0, 1, 0, 0], &[]);
    let (out, stats) = capture(false, || x.divrem(&one));
    assert_eq!(out, (x.clone(), FpPoly::zero(p.clone())));
    c.check(
        "division_by_one",
        &out,
        stats,
        [4, 5, 4, 10, 1, 0],
        &[(1, 1)],
    );
    let (out, stats) = capture(false, || cubic.c(203));
    assert_eq!(out, poly(&[1], 101));
    c.check("cubic101_c203", &out, stats, [0, 0, 0, 1, 0, 1], &[]);
    assert_eq!(c.count, 12);
    assert_eq!(c.sites, (1 << 17) - 1);
    let (out, bad) = capture(true, || x.mul(&y));
    assert_eq!(out, poly(&[1, 0, 3], 7));
    assert_ne!(bad.all.ops, [4, 4, 0, 8, 0, 0]);
    println!("CT16|control|corrupt_product_counter|detected=true|math_unchanged=true|bad_ops={:?}|sites={:?}",bad.all.ops,bad.all.sites);
    let (out, restored) = capture(false, || x.mul(&y));
    assert_eq!(out, poly(&[1, 0, 3], 7));
    restored.validate();
    assert_eq!(restored.all.ops, [4, 4, 0, 8, 0, 0]);
    assert_idle();
    println!(
        "CT16|calibrations_accepted|positive=12|corrupt_control=1|restored_positive=true|sites=17"
    );
}

struct Totals {
    pairs: usize,
    baseline: Snapshot,
    candidate: Snapshot,
    all_relations: [usize; 4],
    inverse_relations: [usize; 4],
    all_expanded: [usize; 4],
    inverse_expanded: [usize; 4],
}

fn budget_line(kind: &str, key: &str, scope: &str, arm: &str, budget: &Budget) {
    println!("CT16|budget|kind={kind}|key={key}|scope={scope}|arm={arm}|ops={:?}|inv_inputs={:?}|sites={:?}",budget.ops,budget.sparse_inverses(),budget.sites);
}

impl Totals {
    fn new() -> Self {
        Self {
            pairs: 0,
            baseline: Snapshot::zero(),
            candidate: Snapshot::zero(),
            all_relations: [0; 4],
            inverse_relations: [0; 4],
            all_expanded: [0; 4],
            inverse_expanded: [0; 4],
        }
    }

    fn pair(&mut self, kind: &str, key: &str, base: Snapshot, fast: Snapshot) {
        base.validate();
        fast.validate();
        let a = base.all.comparison(&fast.all, false);
        let b = base.inverse.comparison(&fast.inverse, false);
        let c = base.all.comparison(&fast.all, true);
        let d = base.inverse.comparison(&fast.inverse, true);
        self.all_relations[a] += 1;
        self.inverse_relations[b] += 1;
        self.all_expanded[c] += 1;
        self.inverse_expanded[d] += 1;
        self.baseline.add(&base);
        self.candidate.add(&fast);
        self.pairs += 1;
        budget_line(kind, key, "all_backend", "baseline", &base.all);
        budget_line(kind, key, "all_backend", "candidate", &fast.all);
        budget_line(kind, key, "inverse_only", "baseline", &base.inverse);
        budget_line(kind, key, "inverse_only", "candidate", &fast.inverse);
        println!("CT16|comparison|kind={kind}|key={key}|indices_equal_candidate_baseline_incomparable|all={a}|inverse={b}|expanded_all={c}|expanded_inverse={d}");
    }

    fn finish(&self, kind: &str, pairs: usize) {
        assert_eq!(self.pairs, pairs);
        self.baseline.validate();
        self.candidate.validate();
        budget_line(kind, "TOTAL", "all_backend", "baseline", &self.baseline.all);
        budget_line(
            kind,
            "TOTAL",
            "all_backend",
            "candidate",
            &self.candidate.all,
        );
        budget_line(
            kind,
            "TOTAL",
            "inverse_only",
            "baseline",
            &self.baseline.inverse,
        );
        budget_line(
            kind,
            "TOTAL",
            "inverse_only",
            "candidate",
            &self.candidate.inverse,
        );
        println!("CT16|totals|kind={kind}|pairs={pairs}|order=equal,candidate_lower,baseline_lower,incomparable|all={:?}|inverse={:?}|expanded_all={:?}|expanded_inverse={:?}",self.all_relations,self.inverse_relations,self.all_expanded,self.inverse_expanded);
    }
}

fn snapshot_match(remaining: &mut BTreeSet<String>, line: String) {
    assert!(
        remaining.remove(&line),
        "unmatched or repeated prior15 row: {line}"
    );
}

fn paired_map(
    baseline: &RichelotMap,
    candidate: &RichelotMap,
    input: &Class,
    reverse: bool,
    label: &str,
    remaining: &mut BTreeSet<String>,
    totals: &mut Totals,
) -> Result<(Class, closed_fibres::ClosedRoute, u64, u64)> {
    println!("CT16|attempt|map|label={label}|reverse={reverse}");
    boundary_reset(&baseline.k);
    boundary_reset(&candidate.k);
    let (out, base) = capture(false, || baseline.evaluate_closed(input, reverse));
    let (expected, old_route) = out?;
    let old = boundary(&baseline.k);
    let (out, fast) = capture(false, || candidate.evaluate_closed(input, reverse));
    let (actual, new_route) = out?;
    let new = boundary(&candidate.k);
    assert_eq!(expected, actual);
    assert_eq!(old_route, new_route);
    assert_eq!(old[2], new[2]);
    assert_eq!(old[0], new[0] + 5 * new[3]);
    assert_eq!(old[1], new[1] + 2 * new[3]);
    assert_eq!(old[4], new[3] + new[4]);
    snapshot_match(remaining,format!("CT15|map|label={label}|reverse={reverse}|route={old_route:?}|input={:?}|baseline={:?}|candidate={:?}|base_counts={old:?}|fast_counts={new:?}",input.components(),expected.components(),actual.components()));
    println!("CT16|map|label={label}|reverse={reverse}|route={old_route:?}|input={:?}|output={:?}|snapshot_equal=true|base_boundary={old:?}|candidate_boundary={new:?}",input.components(),actual.components());
    totals.pair("maps", &format!("{label}/{reverse}"), base, fast);
    Ok((actual, old_route, new[3], new[4]))
}

pub(super) fn run() -> Result<()> {
    assert_idle();
    let baseline = Field::new(CubicField::with_cubic(P14, [3, 26, 3])?);
    let mut candidate = Field::new(CubicField::with_cubic(P14, [3, 26, 3])?);
    candidate.cubic_first15 = true;
    assert_eq!(baseline.nonresidue, candidate.nonresidue);
    println!("CT16|scope|p=101|cubic=[3,26,3,1]|nonresidue=2|ops=M,A,S,R,I,R64|I=atomic_actual_modpow|two_budgets=all_backend,inverse_only|cost_win_not_required|no_timing_ratio|unsigned");
    calibrate(&baseline.k);
    let mut remaining: BTreeSet<String> = REFERENCE
        .lines()
        .filter(|line| line.starts_with("CT15|primitive|") || line.starts_with("CT15|map|"))
        .map(str::to_owned)
        .collect();
    assert_eq!(remaining.len(), 106);
    let mut encodings = BTreeSet::new();
    let mut cubic = Totals::new();
    let mut general = Totals::new();
    let mut signs = 0;
    for i in 0_u64..64 {
        let n = 1 + (65537 * i % 1_030_300);
        assert!(encodings.insert(n));
        let x = baseline.lift3(&baseline.k.element(n));
        assert!(!baseline.zero(&x) && x.b.is_zero());
        boundary_reset(&baseline);
        boundary_reset(&candidate);
        let (old, base) = capture(false, || baseline.inv(&x));
        let old_boundary = boundary(&baseline);
        let (new, fast) = capture(false, || candidate.inv(&x));
        let new_boundary = boundary(&candidate);
        assert_eq!(old, new);
        assert_eq!(old_boundary, [5, 2, 1, 0, 1]);
        assert_eq!(new_boundary, [0, 0, 1, 1, 0]);
        assert_eq!(base.all, base.inverse);
        assert_eq!(fast.all, fast.inverse);
        assert_eq!(baseline.mul(&x, &old), baseline.c(1));
        assert_eq!(candidate.mul(&x, &new), candidate.c(1));
        snapshot_match(&mut remaining,format!("CT15|primitive|kind=cubic|i={i}|input={:?}|baseline={:?}|candidate={:?}|base_counts={old_boundary:?}|fast_counts={new_boundary:?}",encoding(&baseline,&x),encoding(&baseline,&old),encoding(&candidate,&new)));
        println!(
            "CT16|primitive|kind=cubic|i={i}|input={:?}|inverse={:?}|snapshot_equal=true",
            encoding(&baseline, &x),
            encoding(&baseline, &old)
        );
        cubic.pair("cubic", &i.to_string(), base, fast);
        if i < 8 {
            let wrong = candidate.neg(&new);
            assert_ne!(candidate.mul(&x, &wrong), candidate.c(1));
            signs += 1;
            println!(
                "CT16|control|wrong_sign|i={i}|inverse={:?}|detected=true",
                encoding(&candidate, &wrong)
            );
        }
    }
    assert_eq!(encodings.len(), 64);
    assert_eq!(signs, 8);
    cubic.finish("cubic", 64);
    for i in 0_u64..8 {
        let x = Element {
            a: baseline.k.element(i + 1),
            b: baseline.k.element(3 * i + 1),
        };
        assert!(!x.b.is_zero());
        boundary_reset(&baseline);
        boundary_reset(&candidate);
        let (old, base) = capture(false, || baseline.inv(&x));
        let old_boundary = boundary(&baseline);
        let (new, fast) = capture(false, || candidate.inv(&x));
        let new_boundary = boundary(&candidate);
        assert_eq!(old, new);
        assert_eq!(old_boundary, [5, 5, 1, 0, 1]);
        assert_eq!(old_boundary, new_boundary);
        assert_eq!(base, fast);
        assert_eq!(base.all, base.inverse);
        assert_eq!(fast.all, fast.inverse);
        assert_eq!(baseline.mul(&x, &old), baseline.c(1));
        assert_eq!(candidate.mul(&x, &new), candidate.c(1));
        snapshot_match(&mut remaining,format!("CT15|primitive|kind=noncubic|i={i}|input={:?}|baseline={:?}|candidate={:?}|base_counts={old_boundary:?}|fast_counts={new_boundary:?}",encoding(&baseline,&x),encoding(&baseline,&old),encoding(&candidate,&new)));
        println!(
            "CT16|primitive|kind=noncubic|i={i}|input={:?}|inverse={:?}|snapshot_equal=true",
            encoding(&baseline, &x),
            encoding(&baseline, &old)
        );
        general.pair("noncubic", &i.to_string(), base, fast);
    }
    general.finish("noncubic", 8);
    let zero = baseline.c(0);
    for field in [&baseline, &candidate] {
        assert!(catch_unwind(AssertUnwindSafe(|| capture(false, || field.inv(&zero)))).is_err());
        assert_idle();
    }
    println!("CT16|control|zero|rejections=2|panic_hook_not_suppressed=true|inverse_and_capture_scopes_closed=true");
    let mut constructors = 0;
    for (label, p, coefficients) in [
        ("reducible_zero", 101, [0, 0, 0]),
        ("reducible_unit", 101, [100, 0, 0]),
        ("composite_modulus", 15, [1, 1, 1]),
    ] {
        match CubicField::with_cubic(p, coefficients) {
            Ok(_) => return Err(format!("invalid field control accepted: {label}")),
            Err(error) => {
                println!("CT16|control|constructor|label={label}|detected=true|error={error}")
            }
        }
        constructors += 1;
    }
    assert_eq!(constructors, 3);
    let old_map =
        RichelotMap::from_selected_pairing(assessment14(false, Mutation14::None)?, PAIRING14)?;
    let mut new_map =
        RichelotMap::from_selected_pairing(assessment14(false, Mutation14::None)?, PAIRING14)?;
    assert_model14(&old_map.source, &SOURCE14);
    assert_model14(&old_map.target, &TARGET14);
    assert_model14(&new_map.source, &SOURCE14);
    assert_model14(&new_map.target, &TARGET14);
    assert_eq!(old_map.target_scale, 13);
    assert_eq!(new_map.target_scale, 13);
    new_map.k.cubic_first15 = true;
    let witnesses = crate::balanced::witnesses14(&old_map.source)?;
    assert_eq!(witnesses.len(), 17);
    let mut maps = Totals::new();
    let mut routes = [0; 2];
    let mut shortcuts = 0;
    let mut fallbacks = 0;
    let mut closed = 0;
    for (label, input) in &witnesses {
        let (image, route, fast, fallback) = paired_map(
            &old_map,
            &new_map,
            input,
            false,
            label,
            &mut remaining,
            &mut maps,
        )?;
        shortcuts += fast;
        fallbacks += fallback;
        match route {
            closed_fibres::ClosedRoute::LowerDegree => routes[0] += 1,
            closed_fibres::ClosedRoute::Quadratic => routes[1] += 1,
            other => return Err(format!("unexpected route {label}: {other:?}")),
        }
        if label == "quadratic_orbit" || label == "opposite_quadratic_orbit" {
            assert_eq!(route, closed_fibres::ClosedRoute::Quadratic);
            closed += 1;
        }
        let (_, _, fast, fallback) = paired_map(
            &old_map,
            &new_map,
            &image,
            true,
            label,
            &mut remaining,
            &mut maps,
        )?;
        shortcuts += fast;
        fallbacks += fallback;
    }
    assert_eq!(routes, [9, 8]);
    assert_eq!(closed, 2);
    assert_eq!(shortcuts, 1054);
    assert_eq!(fallbacks, 0);
    assert!(remaining.is_empty(), "all106 prior rows must be consumed");
    maps.finish("maps", 34);
    assert_idle();
    println!("CT16|accepted|calibrations=12|counter_negative=1|sites=17|cubic=64|noncubic=8|zero=2|sign=8|constructors=3|forward=17|dual=17|closed=2|snapshot_rows=106|shortcuts={shortcuts}|general={fallbacks}|all_costs_retained|no_timing_ratio|unsigned");
    Ok(())
}
