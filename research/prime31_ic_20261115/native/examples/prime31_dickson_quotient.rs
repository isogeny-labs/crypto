//! Division-free Dickson/S3 resultant gate on the existing ordinary Prime31 curve.
//!
//! This is a correctness instrument, not a timing benchmark or a complete DLP
//! solver. It preserves every old source file. All emitted evidence is unsigned.
//!
//! For q(Y)=dY^2-TY+U and F_k(Y)=D_{2^k}(Y)-c, the homogeneous trace/norm
//! recurrence evaluates Res_Y(F_k,q) using four quotient products per level.
//! It also covers a drop of q from degree two to one or zero. The independent
//! comparator multiplies q(y) over the actual full split Dickson fibre.
//! Fewer quotient products do not imply less work: the product comparator's
//! factors have degree at most two, while recurrence products become dense.

use prime31_algebraic_ic::{check_curve, Arithmetic, Base, Point, Relation, PRIME31};
use serde::Serialize;
use serde_json::json;
use std::cell::Cell;

const P: u64 = 2_147_483_647;
type Poly = Vec<u64>;

fn add(a: u64, b: u64) -> u64 {
    let s = a + b;
    if s >= P {
        s - P
    } else {
        s
    }
}
fn sub(a: u64, b: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + P - b
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
struct Counts {
    field_multiplications: u64,
    quotient_products: u64,
    largest_single_polynomial_slots: usize,
}

#[derive(Default)]
struct Polys {
    fm: Cell<u64>,
    qm: Cell<u64>,
    slots: Cell<usize>,
}

impl Polys {
    fn counts(&self) -> Counts {
        Counts {
            field_multiplications: self.fm.get(),
            quotient_products: self.qm.get(),
            largest_single_polynomial_slots: self.slots.get(),
        }
    }
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.fm.set(self.fm.get() + 1);
        ((a as u128 * b as u128) % P as u128) as u64
    }
    fn power(&self, mut a: u64, mut n: u64) -> u64 {
        let mut z = 1;
        while n != 0 {
            if n & 1 != 0 {
                z = self.mul(z, a);
            }
            n >>= 1;
            if n != 0 {
                a = self.mul(a, a);
            }
        }
        z
    }
    fn trim(&self, mut a: Poly) -> Poly {
        self.slots.set(self.slots.get().max(a.len()));
        while a.last() == Some(&0) {
            a.pop();
        }
        a
    }
    fn sum(&self, a: &[u64], b: &[u64]) -> Poly {
        let mut r = vec![0; a.len().max(b.len())];
        for (i, &v) in a.iter().enumerate() {
            r[i] = add(r[i], v);
        }
        for (i, &v) in b.iter().enumerate() {
            r[i] = add(r[i], v);
        }
        self.trim(r)
    }
    fn diff(&self, a: &[u64], b: &[u64]) -> Poly {
        let mut r = vec![0; a.len().max(b.len())];
        for (i, &v) in a.iter().enumerate() {
            r[i] = v;
        }
        for (i, &v) in b.iter().enumerate() {
            r[i] = sub(r[i], v);
        }
        self.trim(r)
    }
    fn scale(&self, a: &[u64], c: u64) -> Poly {
        if c == 0 {
            return vec![];
        }
        if c == 1 {
            return a.to_vec();
        }
        self.trim(a.iter().map(|&v| self.mul(v, c)).collect())
    }
    fn prod(&self, a: &[u64], b: &[u64]) -> Poly {
        if a.is_empty() || b.is_empty() {
            return vec![];
        }
        let mut r = vec![0; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            if x == 0 {
                continue;
            }
            for (j, &y) in b.iter().enumerate() {
                if y != 0 {
                    r[i + j] = add(r[i + j], self.mul(x, y));
                }
            }
        }
        self.trim(r)
    }
    fn divrem(&self, f: &[u64], d: &[u64]) -> (Poly, Poly) {
        assert!(!d.is_empty(), "polynomial division by zero");
        let mut r = self.trim(f.to_vec());
        let mut q = vec![0; f.len().saturating_sub(d.len()) + 1];
        let lead = *d.last().unwrap();
        let inv = if lead == 1 {
            1
        } else {
            self.power(lead, P - 2)
        };
        while r.len() >= d.len() {
            let j = r.len() - d.len();
            let c = if inv == 1 {
                *r.last().unwrap()
            } else {
                self.mul(*r.last().unwrap(), inv)
            };
            q[j] = c;
            for (i, &v) in d.iter().enumerate() {
                if v != 0 {
                    r[i + j] = sub(r[i + j], self.mul(c, v));
                }
            }
            r = self.trim(r);
        }
        (self.trim(q), r)
    }
    fn rem(&self, a: &[u64], h: &[u64]) -> Poly {
        self.divrem(a, h).1
    }
    fn mprod(&self, a: &[u64], b: &[u64], h: &[u64]) -> Poly {
        self.qm.set(self.qm.get() + 1);
        self.rem(&self.prod(a, b), h)
    }
    fn gcd(&self, mut a: Poly, mut b: Poly) -> Poly {
        while !b.is_empty() {
            let r = self.rem(&a, &b);
            a = b;
            b = r;
        }
        if a.is_empty() {
            return a;
        }
        self.scale(&a, self.power(*a.last().unwrap(), P - 2))
    }
    fn invert(&self, f: &[u64], h: &[u64]) -> Result<Poly, String> {
        let (mut r0, mut r1) = (h.to_vec(), self.rem(f, h));
        let (mut t0, mut t1) = (vec![], vec![1]);
        while !r1.is_empty() {
            let (q, r) = self.divrem(&r0, &r1);
            let t = self.diff(&t0, &self.prod(&q, &t1));
            r0 = r1;
            r1 = r;
            t0 = t1;
            t1 = t;
        }
        if r0.len() != 1 {
            return Err("nonunit normalization denominator".into());
        }
        Ok(self.rem(&self.scale(&t0, self.power(r0[0], P - 2)), h))
    }
    fn powmod(&self, mut a: Poly, mut n: usize, h: &[u64]) -> Poly {
        let mut z = vec![1];
        while n != 0 {
            if n & 1 != 0 {
                z = self.mprod(&z, &a, h);
            }
            n >>= 1;
            if n != 0 {
                a = self.mprod(&a, &a, h);
            }
        }
        z
    }
    fn eval(&self, a: &[u64], x: u64) -> u64 {
        a.iter().rev().fold(0, |z, &v| add(self.mul(z, x), v))
    }
    fn roots_poly(&self, xs: &[u64]) -> Poly {
        xs.iter()
            .fold(vec![1], |z, &x| self.prod(&z, &[sub(0, x), 1]))
    }
    fn fibre_poly(&self, depth: usize, c: u64) -> Poly {
        let mut f = vec![0, 1];
        for _ in 0..depth {
            f = self.diff(&self.prod(&f, &f), &[2]);
        }
        self.diff(&f, &[c])
    }
}

// q(Y)=d(X)Y^2-T(X)Y+U(X). All coordinates are public; no ordinates
// or point additions enter the three algebraic kernels.
fn s3_coefficients(k: &Polys, t: u64) -> (Poly, Poly, Poly) {
    let tt = k.mul(t, t);
    let at = k.mul(PRIME31.a, t);
    let four_b = k.mul(4, PRIME31.b);
    let d = vec![tt, sub(0, add(t, t)), 1];
    let tr = vec![
        add(add(at, at), four_b),
        add(add(PRIME31.a, tt), add(PRIME31.a, tt)),
        add(t, t),
    ];
    let u = vec![
        sub(k.mul(PRIME31.a, PRIME31.a), k.mul(four_b, t)),
        sub(0, add(add(at, at), four_b)),
        tt,
    ];
    (k.trim(d), k.trim(tr), k.trim(u))
}

#[derive(Clone, Debug, Serialize)]
struct Layer {
    level: usize,
    degrees: [Option<usize>; 3],
    nonzero_coefficients: [usize; 3],
    state_coefficient_slots: usize,
}

impl Layer {
    fn new(level: usize, den: &[u64], tr: &[u64], norm: &[u64]) -> Self {
        let states = [den, tr, norm];
        Self {
            level,
            degrees: states.map(|v| v.len().checked_sub(1)),
            nonzero_coefficients: states.map(|v| v.iter().filter(|&&x| x != 0).count()),
            state_coefficient_slots: states.iter().map(|v| v.len()).sum(),
        }
    }
}

fn dickson_resultant(k: &Polys, q: &(Poly, Poly, Poly), depth: usize, c: u64, h: &[u64]) -> Poly {
    dickson_resultant_trace(k, q, depth, c, h).0
}

fn dickson_resultant_trace(
    k: &Polys,
    q: &(Poly, Poly, Poly),
    depth: usize,
    c: u64,
    h: &[u64],
) -> (Poly, Vec<Layer>) {
    let (mut den, mut tr, mut norm) = (k.rem(&q.0, h), k.rem(&q.1, h), k.rem(&q.2, h));
    let mut layers = vec![Layer::new(0, &den, &tr, &norm)];
    for level in 1..=depth {
        let den2 = k.mprod(&den, &den, h);
        let tr2 = k.mprod(&tr, &tr, h);
        let norm2 = k.mprod(&norm, &norm, h);
        let norm_den = k.mprod(&norm, &den, h);
        let tr_next = k.diff(&k.diff(&tr2, &k.scale(&norm_den, 2)), &k.scale(&den2, 4));
        let norm_next = k.sum(
            &k.diff(&norm2, &k.scale(&tr2, 2)),
            &k.sum(&k.scale(&norm_den, 4), &k.scale(&den2, 4)),
        );
        den = den2;
        tr = tr_next;
        norm = norm_next;
        layers.push(Layer::new(level, &den, &tr, &norm));
    }
    let result = k.rem(
        &k.sum(
            &k.diff(&norm, &k.scale(&tr, c)),
            &k.scale(&den, k.mul(c, c)),
        ),
        h,
    );
    (result, layers)
}

// This is the strongest simple polynomial control here: sparse quadratic
// factors, not fictitious dense degree-B factors. Reported kernel counters
// exclude base setup, extraction and validation; no complete-cost conclusion
// follows. A balanced fast product tree is not implemented.
fn fibre_product(k: &Polys, q: &(Poly, Poly, Poly), values: &[u64], h: &[u64]) -> Poly {
    let mut z = vec![1];
    for &y in values {
        let factor = k.sum(
            &k.diff(&k.scale(&q.0, k.mul(y, y)), &k.scale(&q.1, y)),
            &q.2,
        );
        z = k.mprod(&z, &factor, h);
    }
    z
}

// Independent expanded-polynomial Horner control in the regular quadratic
// quotient. Rescaling by d^N compares the exact resultant coefficient vector.
fn expanded_horner(
    k: &Polys,
    q: &(Poly, Poly, Poly),
    f: &[u64],
    h: &[u64],
) -> Result<Poly, String> {
    let inv = k.invert(&q.0, h)?;
    let beta = k.mprod(&k.scale(&q.1, P - 1), &inv, h);
    let gamma = k.mprod(&q.2, &inv, h);
    let (mut r0, mut r1) = (vec![], vec![]);
    for &c in f.iter().rev() {
        let next0 = k.diff(&[c], &k.mprod(&gamma, &r1, h));
        let next1 = k.diff(&r0, &k.mprod(&beta, &r1, h));
        r0 = k.rem(&next0, h);
        r1 = k.rem(&next1, h);
    }
    let r01 = k.mprod(&r0, &r1, h);
    let n = k.sum(
        &k.diff(&k.mprod(&r0, &r0, h), &k.mprod(&beta, &r01, h)),
        &k.mprod(&gamma, &k.mprod(&r1, &r1, h), h),
    );
    let scale = k.powmod(q.0.clone(), f.len() - 1, h);
    Ok(k.mprod(&n, &scale, h))
}

fn signed_rows(
    base: &Base,
    target: Point,
    pairs: &[(usize, usize)],
    ar: &mut Arithmetic,
) -> Vec<Relation> {
    let mut out = vec![];
    for &(i, j) in pairs {
        for si in [-1, 1] {
            for sj in [-1, 1] {
                let u = Some(base.points[i]);
                let v = Some(base.points[j]);
                let u = if si == 1 { u } else { ar.neg(PRIME31, u) };
                let v = if sj == 1 { v } else { ar.neg(PRIME31, v) };
                if ar.add(PRIME31, u, v) == target {
                    let mut entries = [(i, si), (j, sj)];
                    entries.sort_unstable();
                    out.push(Relation { entries });
                }
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}

// The line through -R recovers the unique signed pair for a public target
// with nonzero ordinate. This is a post-resultant extraction control, not
// free group information supplied to the discovery kernel.
fn oriented_rows(
    base: &Base,
    target: Point,
    pairs: &[(usize, usize)],
    ar: &mut Arithmetic,
) -> Result<Vec<Relation>, String> {
    let (r, s) = target.ok_or("oriented affine recovery excludes identity")?;
    if s == 0 {
        return Err("oriented recovery requires nonzero target ordinate".into());
    }
    let k = Polys::default();
    let inv_2s = k.power(add(s, s), P - 2);
    let derivative = add(k.mul(3, k.mul(r, r)), PRIME31.a);
    let mut rows = vec![];
    for &(i, j) in pairs {
        let x = base.points[i].0;
        let z = base.points[j].0;
        let lambda = k.mul(sub(k.mul(sub(x, r), sub(z, r)), derivative), inv_2s);
        let y = sub(k.mul(lambda, sub(x, r)), s);
        let w = sub(k.mul(lambda, sub(z, r)), s);
        let sign = |index: usize, ordinate: u64| -> Result<i8, String> {
            if ordinate == base.points[index].1 {
                Ok(1)
            } else if ordinate == sub(0, base.points[index].1) {
                Ok(-1)
            } else {
                Err("oriented ordinate does not match the factor-base point".into())
            }
        };
        let mut entries = [(i, sign(i, y)?), (j, sign(j, w)?)];
        entries.sort_unstable();
        if !ar.on_curve(PRIME31, Some((x, y)))
            || !ar.on_curve(PRIME31, Some((z, w)))
            || ar.add(PRIME31, Some((x, y)), Some((z, w))) != target
        {
            return Err("oriented relation failed original-curve group replay".into());
        }
        rows.push(Relation { entries });
    }
    rows.sort_unstable();
    rows.dedup();
    Ok(rows)
}

fn pairs_from_resultant(
    k: &Polys,
    base: &Base,
    f: &[u64],
    h: &[u64],
    resultant: &[u64],
    target_x: u64,
) -> Vec<(usize, usize)> {
    let g = k.gcd(h.to_vec(), resultant.to_vec());
    let q = s3_coefficients(k, target_x);
    let mut pairs = vec![];
    for (i, &(x, _)) in base.points.iter().enumerate() {
        if k.eval(&g, x) != 0 {
            continue;
        }
        let local = k.trim(vec![
            k.eval(&q.2, x),
            sub(0, k.eval(&q.1, x)),
            k.eval(&q.0, x),
        ]);
        let allowed = k.gcd(f.to_vec(), local);
        for (j, &(y, _)) in base.points.iter().enumerate() {
            if i <= j && k.eval(&allowed, y) == 0 {
                pairs.push((i, j));
            }
        }
    }
    pairs
}

#[derive(Serialize)]
struct Case {
    depth: usize,
    fibre_size: usize,
    factor_base_size: usize,
    kind: String,
    target: Point,
    tangent_x_in_base: bool,
    candidate_pairs: usize,
    signed_relations: usize,
    recurrence: Counts,
    recurrence_layers: Vec<Layer>,
    sparse_product: Counts,
    regular_full_fibre_horner: Option<Counts>,
    regular_filtered_base_horner: Option<Counts>,
    coefficient_vectors_equal: bool,
    full_fibre_and_liftable_outer_support_equal: bool,
    exhaustive_signed_group_rows_equal: bool,
    oriented_recovery_rows_equal: bool,
}

fn check_case(base: &Base, target: Point, kind: &str, ar: &mut Arithmetic) -> Result<Case, String> {
    let (t, yr) = target.ok_or("identity uses the separate inverse-pair rule")?;
    if !ar.on_curve(PRIME31, target) || yr == 0 {
        return Err("expected a non-2-torsion rational public target".into());
    }
    let setup = Polys::default();
    let xs: Vec<u64> = base.points.iter().map(|q| q.0).collect();
    let h = setup.roots_poly(&xs);
    let f = setup.fibre_poly(base.depth, base.terminal);
    if f != setup.roots_poly(&base.values) {
        return Err("full Dickson fibre polynomial mismatch".into());
    }
    let q = s3_coefficients(&setup, t);
    let rec = Polys::default();
    let (g, recurrence_layers) = dickson_resultant_trace(&rec, &q, base.depth, base.terminal, &h);
    let rec_counts = rec.counts();
    if rec_counts.quotient_products != (4 * base.depth) as u64 {
        return Err("unexpected recurrence product schedule".into());
    }
    let product = Polys::default();
    let gp = fibre_product(&product, &q, &base.values, &h);
    let product_counts = product.counts();
    if g != gp {
        return Err(format!(
            "resultant coefficient mismatch at depth {}",
            base.depth
        ));
    }
    let tangent = xs.binary_search(&t).is_ok();
    let hreg = if tangent {
        let (r, rem) = setup.divrem(&h, &[sub(0, t), 1]);
        if !rem.is_empty() {
            return Err("tangent factor removal failed".into());
        }
        r
    } else {
        h.clone()
    };
    let (horner_counts, filtered_horner_counts) = if hreg.len() > 1 {
        let horner = Polys::default();
        let gh = expanded_horner(&horner, &q, &f, &hreg)?;
        if gh != setup.rem(&g, &hreg) {
            return Err("expanded Horner coefficient mismatch".into());
        }
        // The existing repository oracle uses H(Y), not the larger F(Y).
        // Its support must agree, although its resultant coefficients differ.
        let filtered_horner = Polys::default();
        let gh_filtered = expanded_horner(&filtered_horner, &q, &h, &hreg)?;
        if setup.gcd(hreg.clone(), gh_filtered) != setup.gcd(hreg.clone(), gh) {
            return Err("filtered-base Horner support mismatch".into());
        }
        (Some(horner.counts()), Some(filtered_horner.counts()))
    } else {
        (None, None)
    };
    let full = Polys::default();
    let gf = dickson_resultant(&full, &q, base.depth, base.terminal, &f);
    let full_support = setup.gcd(f.clone(), gf.clone());
    let filtered_support = setup.gcd(h.clone(), g.clone());
    if full_support != filtered_support {
        return Err("nonliftable full-fibre projection survived".into());
    }
    let pairs = pairs_from_resultant(&setup, base, &f, &h, &g, t);
    let rows = signed_rows(base, target, &pairs, ar);
    let all_pairs: Vec<_> = (0..base.points.len())
        .flat_map(|i| (i..base.points.len()).map(move |j| (i, j)))
        .collect();
    let truth = signed_rows(base, target, &all_pairs, ar);
    if rows != truth {
        return Err("signed relation set differs from independent group enumeration".into());
    }
    if oriented_rows(base, target, &pairs, ar)? != rows {
        return Err("oriented recovery differs from exhaustive signed lifts".into());
    }
    Ok(Case {
        depth: base.depth,
        fibre_size: base.values.len(),
        factor_base_size: base.points.len(),
        kind: kind.into(),
        target,
        tangent_x_in_base: tangent,
        candidate_pairs: pairs.len(),
        signed_relations: rows.len(),
        recurrence: rec_counts,
        recurrence_layers,
        sparse_product: product_counts,
        regular_full_fibre_horner: horner_counts,
        regular_filtered_base_horner: filtered_horner_counts,
        coefficient_vectors_equal: true,
        full_fibre_and_liftable_outer_support_equal: true,
        exhaustive_signed_group_rows_equal: true,
        oriented_recovery_rows_equal: true,
    })
}

fn main() -> Result<(), String> {
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar)?;
    let mut cases = vec![];
    for depth in [2, 4, 6] {
        let base = Base::build(PRIME31, depth, &mut ar)?;
        let sum = ar.add(PRIME31, Some(base.points[0]), Some(base.points[1]));
        let dbl = ar.add(PRIME31, Some(base.points[0]), Some(base.points[0]));
        let public = ar.scalar(PRIME31, 17, Some(PRIME31.g));
        for (kind, target) in [
            ("positive distinct pair", sum),
            ("positive repeated point", dbl),
            ("public target", public),
            ("tangent target-x", Some(base.points[0])),
        ] {
            cases.push(check_case(&base, target, kind, &mut ar)?);
        }
    }
    println!(
        "{}",
        json!({
            "schema":"prime31-dickson-quotient-correctness-v1",
            "standing":"[pcc:unverified]",
            "scope":"source-reusing development correctness and deterministic operation counts; no wall times",
            "curve":PRIME31,
            "method":"division-free homogeneous Dickson trace/norm resultant modulo the factor-base polynomial",
            "resultant_orientation":"Res_Y(D_(2^depth)(Y)-c, S3(X,Y,x_R))",
            "performance_warning":"sparse-product factors have degree at most two; dense quotient-call counts are not equal-work units",
        "counter_scope":"algebraic kernels only; excludes base setup, extraction, all verification and complete logarithm recovery",
        "horner_count_qualification":"both Horner controls include denominator clearing used for exact resultant comparison; the existing normalized gcd oracle can omit that unit rescaling",
        "layer_state_order":["denominator","trace","norm"],
            "full_ECDLP_recovery":false,
            "asymptotic_ECDLP_speedup":false,
            "cases":cases,
        })
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_relation_sets_and_tangent_targets_match() {
        let mut ar = Arithmetic::default();
        check_curve(PRIME31, &mut ar).unwrap();
        for depth in [2, 3, 4, 5, 6] {
            let base = Base::build(PRIME31, depth, &mut ar).unwrap();
            let sum = ar.add(PRIME31, Some(base.points[0]), Some(base.points[1]));
            let dbl = ar.add(PRIME31, Some(base.points[1]), Some(base.points[1]));
            let neg = ar.neg(PRIME31, Some(base.points[1]));
            let difference = ar.add(PRIME31, Some(base.points[0]), neg);
            let public = ar.scalar(PRIME31, 137, Some(PRIME31.g));
            for (name, q) in [
                ("sum", sum),
                ("double", dbl),
                ("difference", difference),
                ("public", public),
                ("tangent", Some(base.points[0])),
            ] {
                check_case(&base, q, name, &mut ar).unwrap();
            }
        }
    }

    #[test]
    fn degree_drop_and_nonreduced_outer_ring_preserve_resultant() {
        let mut ar = Arithmetic::default();
        let base = Base::build(PRIME31, 4, &mut ar).unwrap();
        let k = Polys::default();
        let h = k.prod(&k.prod(&[P - 1, 1], &[P - 1, 1]), &[P - 1, 1]);
        for q in [
            (vec![], vec![1, 1], vec![2, 0, 1]),
            (vec![], vec![], vec![7, 2]),
            (vec![1, 2], vec![4, 5], vec![6, 7]),
            (vec![], vec![], vec![]),
        ] {
            assert_eq!(
                dickson_resultant(&k, &q, base.depth, base.terminal, &h),
                fibre_product(&k, &q, &base.values, &h)
            );
        }
    }

    #[test]
    fn zero_depth_is_evaluation_with_correct_resultant_orientation() {
        let k = Polys::default();
        let h = vec![3, 0, 1];
        let q = (vec![7, 1], vec![9], vec![11, 2]);
        assert_eq!(
            dickson_resultant(&k, &q, 0, 13, &h),
            fibre_product(&k, &q, &[13], &h)
        );
    }

    #[test]
    fn identity_target_uses_complete_inverse_pair_rule() {
        let mut ar = Arithmetic::default();
        let base = Base::build(PRIME31, 4, &mut ar).unwrap();
        let inverse_pairs: Vec<_> = (0..base.points.len()).map(|i| (i, i)).collect();
        let all_pairs: Vec<_> = (0..base.points.len())
            .flat_map(|i| (i..base.points.len()).map(move |j| (i, j)))
            .collect();
        assert_eq!(
            signed_rows(&base, None, &inverse_pairs, &mut ar),
            signed_rows(&base, None, &all_pairs, &mut ar)
        );
        assert!(check_case(&base, None, "invalid affine identity", &mut ar).is_err());
    }

    #[test]
    fn altered_resultant_is_rejected_by_coefficient_comparison() {
        let mut ar = Arithmetic::default();
        let base = Base::build(PRIME31, 3, &mut ar).unwrap();
        let k = Polys::default();
        let h = k.roots_poly(&base.points.iter().map(|p| p.0).collect::<Vec<_>>());
        let q = s3_coefficients(&k, PRIME31.g.0);
        let mut g = dickson_resultant(&k, &q, base.depth, base.terminal, &h);
        if g.is_empty() {
            g.push(1);
        } else {
            g[0] = add(g[0], 1);
        }
        assert_ne!(g, fibre_product(&k, &q, &base.values, &h));
    }
}
