//! Exact S3-root intersection pilot on the registered ordinary Prime31 curve.
//!
//! Same public targets, factor base, and Rust group implementation for both arms:
//! (1) signed point-pair table; (2) x-only S3 quadratic-root index.
//! The S3 arm saves unconditional pair additions and batches all nonzero denominators into one Fp inversion per setup/query panel.
//! This is a relation-*oracle* comparison, not a full ECDLP speed claim.
//!
//! cargo test --release --example chained_s3_prime31_pilot
//! cargo run --release --example chained_s3_prime31_pilot

use std::collections::{BTreeSet, HashMap, HashSet};
use std::time::Instant;

use crypto_lib::cryptanalysis::ic_boundary::{
    CountedGroup, GroupOps, PrimeCurve, PrimePoint, RelationSolver,
};
use crypto_lib::cryptanalysis::ic_framework::linalg::StructuredGauss;
use crypto_lib::cryptanalysis::semaev_prime_poly::Curve as SemaevCurve;

const P: u64 = 2_147_483_647;
const A: u64 = 847_514_846;
const B: u64 = 2_087_444_724;
const N: u64 = 2_147_446_831;
const GX: u64 = 625_037_715;
const GY: u64 = 2_100_627_251;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum XKey {
    Infinity,
    Finite(u64),
}

fn key(p: PrimePoint) -> XKey {
    if p.infinity {
        XKey::Infinity
    } else {
        XKey::Finite(p.x)
    }
}
fn full_key(p: PrimePoint) -> (bool, u64, u64) {
    (p.infinity, p.x, p.y)
}
fn signed(c: &PrimeCurve, p: PrimePoint, negative: bool) -> PrimePoint {
    if negative {
        c.neg(p)
    } else {
        p
    }
}

#[derive(Default, Debug)]
struct Stats {
    s3_root_calls: u64,
    s3_denominator_inversions: u64,
    exceptional_x_equal: u64,
    probes: u64,
    lift_candidates: u64,
    ops: GroupOps,
}

/// Exact roots in P^1(Fp) of S3(x(P),x(Q),Z) for rational P,Q.
/// An infinity key is the missing affine group-law branch P +/- Q = O.
/// The two roots have abscissae x(P+Q), x(P-Q); the S3 polynomial
/// by itself does NOT encode the infinity branch.
fn s3_roots(
    c: &PrimeCurve,
    sem: &SemaevCurve,
    lhs: PrimePoint,
    rhs: PrimePoint,
    st: &mut Stats,
) -> Vec<XKey> {
    assert!(!lhs.infinity && !rhs.infinity);
    assert!(c.is_on_curve(lhs) && c.is_on_curve(rhs));
    st.s3_root_calls += 1;
    if lhs.x == rhs.x {
        st.exceptional_x_equal += 1;
        let doubled = c.double(&mut st.ops, lhs);
        let mut out = vec![XKey::Infinity, key(doubled)];
        out.sort_unstable();
        out.dedup();
        return out;
    }
    let f = sem.f;
    let x = lhs.x;
    let y = rhs.x;
    let e1 = f.add(x, y);
    let e2 = f.mul(x, y);
    let mid = f.add(f.mul(e1, f.add(e2, sem.a)), f.mul(2, sem.b));
    let d = f.sub(x, y);
    let inv_d2 = f.inv(f.mul(d, d));
    st.s3_denominator_inversions += 1;
    let split = f.mul(2, f.mul(lhs.y, rhs.y));
    let z1 = f.mul(f.add(mid, split), inv_d2);
    let z2 = f.mul(f.sub(mid, split), inv_d2);
    assert_eq!(sem.s3(x, y, z1), 0, "first S3 root");
    assert_eq!(sem.s3(x, y, z2), 0, "second S3 root");
    let mut out = vec![XKey::Finite(z1), XKey::Finite(z2)];
    out.sort_unstable();
    out.dedup();
    out
}

/// Compute a whole panel of S3 pairs with one inversion, rather than one
/// Fermat inversion per nonexceptional pair. This is Montgomery's batch
/// inversion on (x_i-x_j)^2, *not* a new Semaev identity.
///
/// Each returned vector is the exact projective x-root set of one input pair;
/// equal x, rational two-torsion and O are handled without division.
fn s3_roots_many(
    c: &PrimeCurve,
    sem: &SemaevCurve,
    pairs: &[(PrimePoint, PrimePoint)],
    st: &mut Stats,
) -> Vec<Vec<XKey>> {
    let f = sem.f;
    let mut outputs = vec![Vec::<XKey>::new(); pairs.len()];
    let mut denoms = Vec::<u64>::with_capacity(pairs.len());
    let mut pending = Vec::<(usize, u64, u64)>::with_capacity(pairs.len());

    for (index, &(lhs, rhs)) in pairs.iter().enumerate() {
        assert!(!lhs.infinity && !rhs.infinity);
        assert!(c.is_on_curve(lhs) && c.is_on_curve(rhs));
        st.s3_root_calls += 1;
        if lhs.x == rhs.x {
            st.exceptional_x_equal += 1;
            let doubled = c.double(&mut st.ops, lhs);
            let roots = &mut outputs[index];
            roots.extend([XKey::Infinity, key(doubled)]);
            roots.sort_unstable();
            roots.dedup();
            continue;
        }
        let e1 = f.add(lhs.x, rhs.x);
        let e2 = f.mul(lhs.x, rhs.x);
        let middle = f.add(f.mul(e1, f.add(e2, sem.a)), f.mul(2, sem.b));
        let split = f.mul(2, f.mul(lhs.y, rhs.y));
        let dx = f.sub(lhs.x, rhs.x);
        denoms.push(f.mul(dx, dx));
        pending.push((index, middle, split));
    }

    if !denoms.is_empty() {
        // pref[k] = product of denoms[0..k], pref[0] = 1.
        // Every denominator is nonzero by the unequal-abscissa guard.
        let mut pref = Vec::<u64>::with_capacity(denoms.len() + 1);
        pref.push(1);
        for &denom in &denoms {
            pref.push(f.mul(*pref.last().expect("prefix nonempty"), denom));
        }
        let mut inverse = f.inv(*pref.last().expect("product nonempty"));
        st.s3_denominator_inversions += 1;
        for t in (0..denoms.len()).rev() {
            let (index, middle, split) = pending[t];
            let inv_d2 = f.mul(inverse, pref[t]);
            inverse = f.mul(inverse, denoms[t]);

            let lhs = pairs[index].0;
            let rhs = pairs[index].1;
            let z_plus = f.mul(f.add(middle, split), inv_d2);
            let z_minus = f.mul(f.sub(middle, split), inv_d2);
            assert_eq!(sem.s3(lhs.x, rhs.x, z_plus), 0);
            assert_eq!(sem.s3(lhs.x, rhs.x, z_minus), 0);
            let roots = &mut outputs[index];
            roots.extend([XKey::Finite(z_plus), XKey::Finite(z_minus)]);
            roots.sort_unstable();
            roots.dedup();
        }
    }
    outputs
}

fn factor_base(c: &PrimeCurve, size: usize) -> Vec<PrimePoint> {
    let mut out = Vec::new();
    for x in 0..c.p {
        if let Some(y) = c.sqrt(c.rhs(x)) {
            out.push(PrimePoint::affine(x, y.min((c.p - y) % c.p)));
            if out.len() == size {
                break;
            }
        }
    }
    assert_eq!(out.len(), size, "factor base too small");
    out
}

type GroupPairs = HashMap<(bool, u64, u64), (usize, usize, bool, bool)>;

fn build_group_pairs(c: &PrimeCurve, base: &[PrimePoint], st: &mut Stats) -> GroupPairs {
    let mut table = GroupPairs::new();
    for i in 0..base.len() {
        for j in i..base.len() {
            for si in [false, true] {
                for sj in [false, true] {
                    let sum = c.add(&mut st.ops, signed(c, base[i], si), signed(c, base[j], sj));
                    table.entry(full_key(sum)).or_insert((i, j, si, sj));
                }
            }
        }
    }
    table
}

fn build_s3_pairs(
    c: &PrimeCurve,
    sem: &SemaevCurve,
    base: &[PrimePoint],
    st: &mut Stats,
) -> HashMap<XKey, Vec<(usize, usize)>> {
    let mut table: HashMap<XKey, Vec<(usize, usize)>> = HashMap::new();
    let mut pairs = Vec::with_capacity(base.len() * (base.len() + 1) / 2);
    let mut indices = Vec::with_capacity(pairs.capacity());
    for i in 0..base.len() {
        for j in i..base.len() {
            indices.push((i, j));
            pairs.push((base[i], base[j]));
        }
    }
    for ((i, j), roots) in indices.into_iter().zip(s3_roots_many(c, sem, &pairs, st)) {
        for k in roots {
            table.entry(k).or_default().push((i, j));
        }
    }
    table
}

fn check_witness(
    c: &PrimeCurve,
    sem: &SemaevCurve,
    pts: [PrimePoint; 3],
    target: PrimePoint,
    st: &mut Stats,
) -> bool {
    if pts.iter().any(|&v| !c.is_on_curve(v)) || !c.is_on_curve(target) {
        return false;
    }
    let sum12 = c.add(&mut st.ops, pts[0], pts[1]);
    let sum123 = c.add(&mut st.ops, sum12, pts[2]);
    if sum123 != target {
        return false;
    }
    if !sum12.infinity {
        assert_eq!(sem.s3(pts[0].x, pts[1].x, sum12.x), 0);
        if !target.infinity {
            assert_eq!(sem.s3(sum12.x, pts[2].x, target.x), 0);
        }
    }
    true
}

fn query_group(
    c: &PrimeCurve,
    sem: &SemaevCurve,
    base: &[PrimePoint],
    table: &GroupPairs,
    target: PrimePoint,
    st: &mut Stats,
) -> Option<[PrimePoint; 3]> {
    for &pt in base {
        for negative in [false, true] {
            let leaf = signed(c, pt, negative);
            let residue = c.add(&mut st.ops, target, c.neg(leaf));
            st.probes += 1;
            if let Some(&(i, j, si, sj)) = table.get(&full_key(residue)) {
                let w = [signed(c, base[i], si), signed(c, base[j], sj), leaf];
                if check_witness(c, sem, w, target, st) {
                    return Some(w);
                }
            }
        }
    }
    None
}

fn query_s3(
    c: &PrimeCurve,
    sem: &SemaevCurve,
    base: &[PrimePoint],
    table: &HashMap<XKey, Vec<(usize, usize)>>,
    target: PrimePoint,
    st: &mut Stats,
) -> Option<[PrimePoint; 3]> {
    // The entire query batch shares the target. The one nonzero product
    // inversion is charged even if the first matching bucket exits early.
    let mut roots_by_leaf: Vec<Vec<XKey>> = if target.infinity {
        base.iter().map(|leaf| vec![XKey::Finite(leaf.x)]).collect()
    } else {
        let pairs: Vec<_> = base.iter().map(|&leaf| (target, leaf)).collect();
        s3_roots_many(c, sem, &pairs, st)
    };
    for (k, _) in base.iter().enumerate() {
        for root in std::mem::take(&mut roots_by_leaf[k]) {
            st.probes += 1;
            if let Some(candidates) = table.get(&root) {
                for &(i, j) in candidates {
                    // The x-only root forgets all three signs. Recover
                    // actual rational points and reject false candidates.
                    st.lift_candidates += 1;
                    for si in [false, true] {
                        for sj in [false, true] {
                            for sk in [false, true] {
                                let w = [
                                    signed(c, base[i], si),
                                    signed(c, base[j], sj),
                                    signed(c, base[k], sk),
                                ];
                                if check_witness(c, sem, w, target, st) {
                                    return Some(w);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn scalar(c: &PrimeCurve, mut point: PrimePoint, mut k: u64) -> PrimePoint {
    let mut r = PrimePoint::INFINITY;
    let mut ops = GroupOps::default();
    while k != 0 {
        if k & 1 != 0 {
            r = c.add(&mut ops, r, point);
        }
        k >>= 1;
        if k != 0 {
            point = c.double(&mut ops, point);
        }
    }
    r
}

// A row records the signed multiplicity of every factor-base coordinate.
// Keep the zero row: shorter decompositions and cancellation are real cases.
type RelationRow = Vec<i8>;
type PairWitness = (usize, usize, bool, bool);

fn row(base: &[PrimePoint], witness: &[(usize, bool)]) -> RelationRow {
    let mut result = vec![0; base.len()];
    for &(index, negative) in witness {
        result[index] += if negative { -1 } else { 1 };
    }
    result
}

fn signed_pairs(
    c: &PrimeCurve,
    base: &[PrimePoint],
    st: &mut Stats,
) -> HashMap<(bool, u64, u64), Vec<PairWitness>> {
    let mut table = HashMap::<_, Vec<_>>::new();
    for i in 0..base.len() {
        for j in i..base.len() {
            for si in [false, true] {
                for sj in [false, true] {
                    let sum = c.add(&mut st.ops, signed(c, base[i], si), signed(c, base[j], sj));
                    table.entry(full_key(sum)).or_default().push((i, j, si, sj));
                }
            }
        }
    }
    table
}

fn pair_point(
    c: &PrimeCurve,
    base: &[PrimePoint],
    pair: PairWitness,
    st: &mut Stats,
) -> PrimePoint {
    c.add(
        &mut st.ops,
        signed(c, base[pair.0], pair.2),
        signed(c, base[pair.1], pair.3),
    )
}

fn verify_row(
    c: &PrimeCurve,
    base: &[PrimePoint],
    target: PrimePoint,
    coefficients: &[i8],
    st: &mut Stats,
) {
    let mut sum = PrimePoint::INFINITY;
    for (index, &coefficient) in coefficients.iter().enumerate() {
        for _ in 0..coefficient.unsigned_abs() {
            let term = signed(c, base[index], coefficient < 0);
            sum = c.add(&mut st.ops, sum, term);
        }
    }
    assert_eq!(sum, target, "accepted row fails group verification");
}

fn right_summands(
    c: &PrimeCurve,
    base: &[PrimePoint],
    m: usize,
    st: &mut Stats,
) -> Vec<(Vec<(usize, bool)>, PrimePoint)> {
    let mut right = Vec::new();
    if m == 3 {
        for k in 0..base.len() {
            for sk in [false, true] {
                right.push((vec![(k, sk)], signed(c, base[k], sk)));
            }
        }
    } else {
        assert_eq!(m, 4);
        for k in 0..base.len() {
            for l in k..base.len() {
                for sk in [false, true] {
                    for sl in [false, true] {
                        let pair = (k, l, sk, sl);
                        right.push((vec![(k, sk), (l, sl)], pair_point(c, base, pair, st)));
                    }
                }
            }
        }
    }
    right
}

fn all_group_rows(
    c: &PrimeCurve,
    base: &[PrimePoint],
    target: PrimePoint,
    right: &[(Vec<(usize, bool)>, PrimePoint)],
    table: &HashMap<(bool, u64, u64), Vec<PairWitness>>,
    st: &mut Stats,
) -> BTreeSet<RelationRow> {
    let mut rows = BTreeSet::new();
    for (suffix, sum) in right {
        let residue = c.add(&mut st.ops, target, c.neg(*sum));
        st.probes += 1;
        if let Some(matches) = table.get(&full_key(residue)) {
            for &(i, j, si, sj) in matches {
                let mut terms = vec![(i, si), (j, sj)];
                terms.extend(suffix);
                let coefficients = row(base, &terms);
                verify_row(c, base, target, &coefficients, st);
                rows.insert(coefficients);
            }
        }
    }
    rows
}

fn all_s3_rows(
    c: &PrimeCurve,
    base: &[PrimePoint],
    target: PrimePoint,
    right: &[(Vec<(usize, bool)>, PrimePoint)],
    table: &HashMap<XKey, Vec<(usize, usize)>>,
    st: &mut Stats,
) -> BTreeSet<RelationRow> {
    let mut rows = BTreeSet::new();
    for (suffix, sum) in right {
        let residue = c.add(&mut st.ops, target, c.neg(*sum));
        st.probes += 1;
        if let Some(matches) = table.get(&key(residue)) {
            for &(i, j) in matches {
                st.lift_candidates += 1;
                for si in [false, true] {
                    for sj in [false, true] {
                        let pair = (i, j, si, sj);
                        if pair_point(c, base, pair, st) != residue {
                            continue;
                        }
                        let mut terms = vec![(i, si), (j, sj)];
                        terms.extend(suffix);
                        let coefficients = row(base, &terms);
                        verify_row(c, base, target, &coefficients, st);
                        rows.insert(coefficients);
                    }
                }
            }
        }
    }
    rows
}

fn small_field_panel(arm: &str, m: usize, size: usize) {
    assert!([3, 4].contains(&m));
    let c = PrimeCurve { p: 101, a: 7, b: 4 };
    let sem = SemaevCurve::new(c.p, c.a, c.b);
    let base = factor_base(&c, size);
    let mut targets = vec![PrimePoint::INFINITY];
    for x in 0..c.p {
        if let Some(y) = c.sqrt(c.rhs(x)) {
            targets.push(PrimePoint::affine(x, y));
            if y != 0 {
                targets.push(PrimePoint::affine(x, c.p - y));
            }
        }
    }
    let start = Instant::now();
    let mut stats = Stats::default();
    let mut produced = 0usize;
    let mut successful_targets = 0usize;
    let (keys, entries, max_bucket, right_states) = match arm {
        "group" => {
            let table = signed_pairs(&c, &base, &mut stats);
            let right = right_summands(&c, &base, m, &mut stats);
            for target in targets.iter().copied() {
                let rows = all_group_rows(&c, &base, target, &right, &table, &mut stats);
                produced += rows.len();
                successful_targets += usize::from(!rows.is_empty());
            }
            (
                table.len(),
                table.values().map(Vec::len).sum::<usize>(),
                table.values().map(Vec::len).max().unwrap_or(0),
                right.len(),
            )
        }
        "s3" => {
            let table = build_s3_pairs(&c, &sem, &base, &mut stats);
            let right = right_summands(&c, &base, m, &mut stats);
            for target in targets.iter().copied() {
                let rows = all_s3_rows(&c, &base, target, &right, &table, &mut stats);
                produced += rows.len();
                successful_targets += usize::from(!rows.is_empty());
            }
            (
                table.len(),
                table.values().map(Vec::len).sum::<usize>(),
                table.values().map(Vec::len).max().unwrap_or(0),
                right.len(),
            )
        }
        _ => panic!("arm must be group or s3"),
    };
    println!(
        "arm={arm} p={} a={} b={} m={m} B={size} targets={} successful_targets={} verified_rows={} table_keys={keys} table_entries={entries} max_bucket={max_bucket} right_states={right_states} probes={} lift_candidates={} roots={} s3_inversions={} adds={} doubles={} group_field_ops={:?} elapsed_ns={}",
        c.p, c.a, c.b, targets.len(), successful_targets, produced,
        stats.probes, stats.lift_candidates, stats.s3_root_calls,
        stats.s3_denominator_inversions, stats.ops.adds, stats.ops.doubles,
        stats.ops.field_ops, start.elapsed().as_nanos()
    );
}

fn recover_from_signed_rows(
    c: &PrimeCurve,
    generator: PrimePoint,
    target: PrimePoint,
    order: u64,
    base: &[PrimePoint],
    relations: impl IntoIterator<Item = (RelationRow, u64, u64)>,
) -> Option<(u64, usize, u64)> {
    let mut solver = StructuredGauss::new(base.len() + 1, order);
    let mut seen = HashSet::new();
    let mut accepted = 0usize;
    for (coefficients, u, v) in relations {
        // Sum(coefficients_i * base_i) = [u]G + [v]Q.
        // Thus A*ell - v*z = u, with the last column reserved for z.
        if !seen.insert((coefficients.clone(), u % order, v % order)) {
            continue;
        }
        let mut matrix_row: Vec<u64> = coefficients
            .into_iter()
            .map(|value| (value as i64).rem_euclid(order as i64) as u64)
            .collect();
        matrix_row.push((order - v % order) % order);
        solver.add_row(matrix_row, u % order);
        accepted += 1;
        if let Some(z) = solver.pinned(base.len()) {
            assert_eq!(
                scalar(c, generator, z),
                target,
                "recovered log fails verification"
            );
            return Some((z, accepted, solver.work().0));
        }
    }
    None
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.get(1).is_some_and(|value| value == "--small-field") {
        assert_eq!(args.len(), 5, "usage: --small-field group|s3 3|4 base_size");
        small_field_panel(&args[2], args[3].parse().unwrap(), args[4].parse().unwrap());
        return;
    }
    let c = PrimeCurve { p: P, a: A, b: B };
    let sem = SemaevCurve::new(P, A, B);
    let g = PrimePoint::affine(GX, GY);
    assert!(c.is_on_curve(g));
    assert_eq!(scalar(&c, g, N), PrimePoint::INFINITY);
    let base = factor_base(&c, 32);

    let mut group_stats = Stats::default();
    let group_table = build_group_pairs(&c, &base, &mut group_stats);
    let mut s3_stats = Stats::default();
    let s3_table = build_s3_pairs(&c, &sem, &base, &mut s3_stats);

    let mut targets = Vec::new();
    for k in 0..32usize {
        let mut ops = GroupOps::default();
        let t = c.add(
            &mut ops,
            base[k % base.len()],
            base[(3 * k + 5) % base.len()],
        );
        targets.push(c.add(&mut ops, t, base[(7 * k + 11) % base.len()]));
        targets.push(scalar(&c, g, (k as u64 + 13) * 97_531));
    }
    let mut hits = 0usize;
    for target in targets.iter().copied() {
        let group = query_group(&c, &sem, &base, &group_table, target, &mut group_stats);
        let algebraic = query_s3(&c, &sem, &base, &s3_table, target, &mut s3_stats);
        assert_eq!(algebraic.is_some(), group.is_some(), "oracle disagreement");
        if let Some(w) = algebraic {
            assert!(check_witness(&c, &sem, w, target, &mut s3_stats));
            hits += 1;
        }
    }
    println!(
        "field={} curve_a={} curve_b={} order={} B={} targets={} matches={}",
        P,
        A,
        B,
        N,
        base.len(),
        targets.len(),
        hits
    );
    println!(
        "group: entries={} probes={} adds={} doubles={} field_ops={:?}",
        group_table.len(),
        group_stats.probes,
        group_stats.ops.adds,
        group_stats.ops.doubles,
        group_stats.ops.field_ops
    );
    println!("S3: xkeys={} probes={} roots={} inversions={} diagonal={} lift_candidates={} adds={} doubles={} field_ops={:?}", s3_table.len(), s3_stats.probes, s3_stats.s3_root_calls, s3_stats.s3_denominator_inversions, s3_stats.exceptional_x_equal, s3_stats.lift_candidates, s3_stats.ops.adds, s3_stats.ops.doubles, s3_stats.ops.field_ops);
    println!("NOTE: S3 inversions are separate from group field_ops; counts are not comparable work units until calibrated. No ECDLP solved.");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exhaustive_rows(
        c: &PrimeCurve,
        base: &[PrimePoint],
        target: PrimePoint,
        m: usize,
    ) -> BTreeSet<RelationRow> {
        fn visit(
            c: &PrimeCurve,
            base: &[PrimePoint],
            target: PrimePoint,
            m: usize,
            terms: &mut Vec<(usize, bool)>,
            sum: PrimePoint,
            rows: &mut BTreeSet<RelationRow>,
            ops: &mut GroupOps,
        ) {
            if terms.len() == m {
                if sum == target {
                    rows.insert(row(base, terms));
                }
                return;
            }
            for index in 0..base.len() {
                for negative in [false, true] {
                    terms.push((index, negative));
                    let next = c.add(ops, sum, signed(c, base[index], negative));
                    visit(c, base, target, m, terms, next, rows, ops);
                    terms.pop();
                }
            }
        }
        let mut rows = BTreeSet::new();
        visit(
            c,
            base,
            target,
            m,
            &mut Vec::new(),
            PrimePoint::INFINITY,
            &mut rows,
            &mut GroupOps::default(),
        );
        rows
    }

    #[test]
    fn complete_rows_match_exhaustive_oracle_for_three_and_four_terms() {
        let c = PrimeCurve { p: 101, a: 7, b: 4 };
        let sem = SemaevCurve::new(c.p, c.a, c.b);
        let base = factor_base(&c, 4);
        let mut targets = vec![PrimePoint::INFINITY];
        for x in 0..c.p {
            if let Some(y) = c.sqrt(c.rhs(x)) {
                targets.push(PrimePoint::affine(x, y));
                if y != 0 {
                    targets.push(PrimePoint::affine(x, c.p - y));
                }
            }
        }
        let group_table = signed_pairs(&c, &base, &mut Stats::default());
        let s3_table = build_s3_pairs(&c, &sem, &base, &mut Stats::default());
        for m in [3, 4] {
            let right = right_summands(&c, &base, m, &mut Stats::default());
            for target in targets.iter().copied() {
                let expected = exhaustive_rows(&c, &base, target, m);
                let group = all_group_rows(
                    &c,
                    &base,
                    target,
                    &right,
                    &group_table,
                    &mut Stats::default(),
                );
                let s3 = all_s3_rows(&c, &base, target, &right, &s3_table, &mut Stats::default());
                assert_eq!(
                    group, expected,
                    "point-pair method, m={m}, target={target:?}"
                );
                assert_eq!(s3, expected, "S3 method, m={m}, target={target:?}");
            }
        }
    }

    #[test]
    fn independent_integer_oracle_aggregate_counts() {
        let c = PrimeCurve { p: 101, a: 7, b: 4 };
        let sem = SemaevCurve::new(c.p, c.a, c.b);
        let mut targets = vec![PrimePoint::INFINITY];
        for x in 0..c.p {
            if let Some(y) = c.sqrt(c.rhs(x)) {
                targets.push(PrimePoint::affine(x, y));
                if y != 0 {
                    targets.push(PrimePoint::affine(x, c.p - y));
                }
            }
        }
        assert_eq!(targets.len(), 97);
        for (m, size, expected_successes, expected_rows) in [
            (3, 4, 58, 96),
            (3, 8, 97, 704),
            (4, 4, 85, 225),
            (4, 8, 97, 2945),
        ] {
            let base = factor_base(&c, size);
            let table = build_s3_pairs(&c, &sem, &base, &mut Stats::default());
            let right = right_summands(&c, &base, m, &mut Stats::default());
            let rows: Vec<_> = targets
                .iter()
                .copied()
                .map(|target| all_s3_rows(&c, &base, target, &right, &table, &mut Stats::default()))
                .collect();
            assert_eq!(
                rows.iter().filter(|result| !result.is_empty()).count(),
                expected_successes
            );
            assert_eq!(rows.iter().map(BTreeSet::len).sum::<usize>(), expected_rows);
        }
    }

    #[test]
    fn target_dependency_recovers_log_with_repeated_factor_rows() {
        let c = PrimeCurve { p: 101, a: 7, b: 4 };
        let sem = SemaevCurve::new(c.p, c.a, c.b);
        let base = factor_base(&c, 4);
        let generator = base[0];
        let known_log = 37;
        let target = scalar(&c, generator, known_log);
        assert_eq!(scalar(&c, generator, 97), PrimePoint::INFINITY);
        for m in [3, 4] {
            let table = build_s3_pairs(&c, &sem, &base, &mut Stats::default());
            let right = right_summands(&c, &base, m, &mut Stats::default());
            let mut relations = Vec::new();
            for v in [0, 1] {
                for u in 0..97 {
                    let public_point = c.add(
                        &mut GroupOps::default(),
                        scalar(&c, generator, u),
                        scalar(&c, target, v),
                    );
                    for coefficients in all_s3_rows(
                        &c,
                        &base,
                        public_point,
                        &right,
                        &table,
                        &mut Stats::default(),
                    ) {
                        relations.push((coefficients, u, v));
                    }
                }
            }
            assert!(relations.iter().any(|(a, u, v)| {
                relations
                    .iter()
                    .any(|(b, w, t)| a == b && (*u, *v) != (*w, *t))
            }));
            let result = recover_from_signed_rows(&c, generator, target, 97, &base, relations);
            assert_eq!(result.map(|value| value.0), Some(known_log));
        }
    }

    fn brute(c: &PrimeCurve, base: &[PrimePoint], target: PrimePoint) -> bool {
        let mut ops = GroupOps::default();
        for &p in base {
            for &q in base {
                for &r in base {
                    for signs in 0..8usize {
                        let a = signed(c, p, signs & 1 != 0);
                        let b = signed(c, q, signs & 2 != 0);
                        let d = signed(c, r, signs & 4 != 0);
                        let ab = c.add(&mut ops, a, b);
                        if c.add(&mut ops, ab, d) == target {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    #[test]
    fn small_curve_exhaustive_group_and_s3_agree() {
        let c = PrimeCurve { p: 101, a: 2, b: 3 };
        let sem = SemaevCurve::new(c.p, c.a, c.b);
        let base = factor_base(&c, 4);
        let mut st_group = Stats::default();
        let group = build_group_pairs(&c, &base, &mut st_group);
        let mut st_s3 = Stats::default();
        let s3 = build_s3_pairs(&c, &sem, &base, &mut st_s3);
        let mut targets = vec![PrimePoint::INFINITY];
        for x in 0..c.p {
            if let Some(y) = c.sqrt(c.rhs(x)) {
                targets.push(PrimePoint::affine(x, y));
                if y != 0 {
                    targets.push(PrimePoint::affine(x, c.p - y));
                }
            }
        }
        for t in targets {
            let b = brute(&c, &base, t);
            assert_eq!(
                query_group(&c, &sem, &base, &group, t, &mut st_group).is_some(),
                b
            );
            assert_eq!(query_s3(&c, &sem, &base, &s3, t, &mut st_s3).is_some(), b);
        }
    }

    #[test]
    fn s3_roots_equal_signed_group_sums_including_diagonals() {
        for (p, a, b) in [(101, 2, 3), (P, A, B)] {
            let c = PrimeCurve { p, a, b };
            let sem = SemaevCurve::new(p, a, b);
            let base = factor_base(&c, 12);
            let mut st = Stats::default();
            let pair_panel: Vec<_> = base
                .iter()
                .flat_map(|&v| base.iter().map(move |&w| (v, w)))
                .collect();
            let mut batched = Stats::default();
            let batch = s3_roots_many(&c, &sem, &pair_panel, &mut batched);
            assert_eq!(batched.s3_denominator_inversions, 1, "one batched inverse");
            for ((v, w), batch_roots) in pair_panel.into_iter().zip(batch) {
                let actual = s3_roots(&c, &sem, v, w, &mut st);
                let mut ops = GroupOps::default();
                let mut expected = vec![
                    key(c.add(&mut ops, v, w)),
                    key(c.add(&mut ops, v, c.neg(w))),
                ];
                expected.sort_unstable();
                expected.dedup();
                assert_eq!(actual, expected, "S3 roots vs group law");
                assert_eq!(batch_roots, expected, "batched S3 roots vs group law");
            }
        }
    }
}
