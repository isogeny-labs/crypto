//! Exact S3-root intersection pilot on the registered ordinary Prime31 curve.
//!
//! Same public targets, factor base, and Rust group implementation for both arms:
//! (1) signed point-pair table; (2) x-only S3 quadratic-root index.
//! The S3 arm saves unconditional pair additions but pays Fp inversions.
//! This is a relation-*oracle* comparison, not a full ECDLP speed claim.
//!
//! cargo test --release --example chained_s3_prime31_pilot
//! cargo run --release --example chained_s3_prime31_pilot

use std::collections::HashMap;

use crypto_lib::cryptanalysis::ic_boundary::{CountedGroup, GroupOps, PrimeCurve, PrimePoint};
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
    if p.infinity { XKey::Infinity } else { XKey::Finite(p.x) }
}
fn full_key(p: PrimePoint) -> (bool, u64, u64) {
    (p.infinity, p.x, p.y)
}
fn signed(c: &PrimeCurve, p: PrimePoint, negative: bool) -> PrimePoint {
    if negative { c.neg(p) } else { p }
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
    for i in 0..base.len() {
        for j in i..base.len() {
            for k in s3_roots(c, sem, base[i], base[j], st) {
                table.entry(k).or_default().push((i, j));
            }
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
    for (k, &leaf) in base.iter().enumerate() {
        // If R=O, the pair must have the same x as the third leaf.
        let keys = if target.infinity {
            vec![XKey::Finite(leaf.x)]
        } else {
            s3_roots(c, sem, target, leaf, st)
        };
        for root in keys {
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

fn main() {
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
        let t = c.add(&mut ops, base[k % base.len()], base[(3*k+5) % base.len()]);
        targets.push(c.add(&mut ops, t, base[(7*k+11) % base.len()]));
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
    println!("field={} curve_a={} curve_b={} order={} B={} targets={} matches={}", P, A, B, N, base.len(), targets.len(), hits);
    println!("group: entries={} probes={} adds={} doubles={} field_ops={:?}", group_table.len(), group_stats.probes, group_stats.ops.adds, group_stats.ops.doubles, group_stats.ops.field_ops);
    println!("S3: xkeys={} probes={} roots={} inversions={} diagonal={} lift_candidates={} adds={} doubles={} field_ops={:?}", s3_table.len(), s3_stats.probes, s3_stats.s3_root_calls, s3_stats.s3_denominator_inversions, s3_stats.exceptional_x_equal, s3_stats.lift_candidates, s3_stats.ops.adds, s3_stats.ops.doubles, s3_stats.ops.field_ops);
    println!("NOTE: S3 inversions are separate from group field_ops; counts are not comparable work units until calibrated. No ECDLP solved.");
}

#[cfg(test)]
mod tests {
    use super::*;

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
                    targets.push(PrimePoint::affine(x, c.p-y));
                }
            }
        }
        for t in targets {
            let b = brute(&c, &base, t);
            assert_eq!(query_group(&c, &sem, &base, &group, t, &mut st_group).is_some(), b);
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
            for &v in &base {
                for &w in &base {
                    let actual = s3_roots(&c, &sem, v, w, &mut st);
                    let mut ops = GroupOps::default();
                    let mut expected = vec![
                        key(c.add(&mut ops, v, w)),
                        key(c.add(&mut ops, v, c.neg(w))),
                    ];
                    expected.sort_unstable();
                    expected.dedup();
                    assert_eq!(actual, expected, "s3 roots vs point sums");
                }
            }
        }
    }
}
