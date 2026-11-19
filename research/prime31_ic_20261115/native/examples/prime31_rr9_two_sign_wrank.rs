//! Prime31 RR9: exact rank of a two-sign public rational-list interpolant.
//!
//! This is a NO-ENUMERATION negative gate for one proposed *non-generic*
//! relation extractor, not a discrete-log solver or a timing benchmark.
//! The field, curve, generator, and factor bases reuse the original native
//! Prime31 Rust adapter. No signed group-sum table or secret scalar is used.
//!
//! Run under the pinned original source:
//! cargo +1.100.0 test --locked --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_rr9_two_sign_wrank
//! cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_rr9_two_sign_wrank
use prime31_algebraic_ic::{check_curve, Arithmetic, Base, PRIME31};
use serde::Serialize;

const P: u64 = 2_147_483_647;
const DEGREES: [usize; 3] = [7, 27, 28];

#[inline]
fn mul(x: u64, y: u64) -> u64 {
    ((x as u128 * y as u128) % (P as u128)) as u64
}
#[inline]
fn add(x: u64, y: u64) -> u64 {
    (x + y) % P
}
#[inline]
fn sub(x: u64, y: u64) -> u64 {
    (x + P - y) % P
}
fn pow(mut x: u64, mut k: u64) -> u64 {
    let mut z = 1;
    while k != 0 {
        if k & 1 != 0 { z = mul(z, x); }
        x = mul(x, x);
        k >>= 1;
    }
    z
}
fn inv(x: u64) -> u64 {
    assert_ne!(x, 0);
    pow(x, P - 2)
}
fn monomials(d: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    // wt(X) = 1; wt(Z) = 3, matching deg(A),deg(V) <= 3.
    for j in 0..=d / 3 {
        for i in 0..=d - 3 * j { out.push((i, j)); }
    }
    out
}
fn modular_rank(rows: &[Vec<u64>]) -> usize {
    let mut a = rows.to_vec();
    let h = a.len();
    let w = a[0].len();
    let mut pivot_row = 0;
    for j in 0..w {
        let Some(k) = (pivot_row..h).find(|&i| a[i][j] != 0) else {
            continue;
        };
        a.swap(pivot_row, k);
        let scale = inv(a[pivot_row][j]);
        for c in j..w { a[pivot_row][c] = mul(a[pivot_row][c], scale); }
        for i in pivot_row + 1..h {
            let alpha = a[i][j];
            if alpha == 0 { continue; }
            for c in j..w {
                a[i][c] = sub(a[i][c], mul(alpha, a[pivot_row][c]));
            }
        }
        pivot_row += 1;
        if pivot_row == h { break; }
    }
    pivot_row
}
fn first_x31(ar: &mut Arithmetic) -> Vec<(u64, u64)> {
    let mut points = Vec::new();
    for x in 0..10_000 {
        let rhs = ar.rhs(PRIME31, x);
        if let Some(y) = ar.sqrt(rhs, P) {
            if y != 0 { points.push((x, y.min(P - y))); }
        }
        if points.len() == 31 { break; }
    }
    assert_eq!(points.len(), 31, "original first-rational-x base");
    points
}
fn signed_slope_points(base: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let (r, s) = PRIME31.g;
    assert_eq!(base.len(), 31);
    let mut out = Vec::with_capacity(62);
    for &(x, y) in base {
        assert!(x < P && y > 0 && y < P && x != r);
        let inv_dx = inv(sub(x, r));
        for ordinate in [y, P - y] {
            // z = A(x)/V(x) = -(s +/- y)/(x-r).
            let z = mul(sub(0, add(s, ordinate)), inv_dx);
            let d = add(add(mul(x, x), mul(r, x)), add(mul(r, r), PRIME31.a));
            // Q0 = (x-r)z^2 + 2*s*z - (x^2+rx+r^2+a).
            let q0 = sub(add(mul(sub(x, r), mul(z, z)), mul(add(s, s), z)), d);
            assert_eq!(q0, 0, "universal two-sign curve component");
            out.push((x, z));
        }
    }
    out
}
fn evaluation_rank(points: &[(u64, u64)], degree: usize) -> (usize, usize) {
    let mons = monomials(degree);
    let mut rows = Vec::new();
    for &(x, z) in points {
        let mut xp = vec![1u64; degree + 1];
        let mut zp = vec![1u64; degree / 3 + 1];
        for i in 1..xp.len() { xp[i] = mul(xp[i - 1], x); }
        for j in 1..zp.len() { zp[j] = mul(zp[j - 1], z); }
        rows.push(mons.iter().map(|&(i, j)| mul(xp[i], zp[j])).collect());
    }
    (mons.len(), modular_rank(&rows))
}
#[derive(Serialize)]
struct RankCase {
    weighted_degree: usize,
    monomials: usize,
    public_signed_points: usize,
    evaluation_rank: usize,
    right_nullity: usize,
    universal_q0_multiples: usize,
    extra_kernel_dimension: usize,
}
#[derive(Serialize)]
struct BaseReport {
    base: &'static str,
    x_coordinates: Vec<u64>,
    ranks: Vec<RankCase>,
}
fn report_base(name: &'static str, base: Vec<(u64, u64)>) -> BaseReport {
    let signed = signed_slope_points(&base);
    let ranks = DEGREES.iter().map(|&d| {
        let (n, rank) = evaluation_rank(&signed, d);
        let automatic = if d < 7 { 0 } else { monomials(d - 7).len() };
        let nullity = n - rank;
        assert!(nullity >= automatic, "Q0 multiples must be in the kernel");
        RankCase {
            weighted_degree: d,
            monomials: n,
            public_signed_points: signed.len(),
            evaluation_rank: rank,
            right_nullity: nullity,
            universal_q0_multiples: automatic,
            extra_kernel_dimension: nullity - automatic,
        }
    }).collect();
    BaseReport {
        base: name,
        x_coordinates: base.into_iter().map(|(x, _)| x).collect(),
        ranks,
    }
}
fn calculate() -> Vec<BaseReport> {
    assert_eq!(P, PRIME31.p);
    let mut ar = Arithmetic::default();
    check_curve(PRIME31, &mut ar).expect("unchanged original cofactor-one curve");
    let first = first_x31(&mut ar);
    let dickson = Base::build(PRIME31, 6, &mut ar)
        .expect("original Dickson depth-6 base").points;
    assert_eq!(dickson.len(), 31);
    vec![report_base("first-x31", first), report_base("dickson-depth6-31", dickson)]
}
fn main() {
    let reports = calculate();
    println!("{}", serde_json::json!({
        "schema": "prime31.rr9.two-sign-weighted-rank.v1",
        "status": "native exact finite-field rank; not an ECDLP solve",
        "p": PRIME31.p,
        "a": PRIME31.a,
        "b": PRIME31.b,
        "n": PRIME31.n,
        "R": [PRIME31.g.0, PRIME31.g.1],
        "weight_x": 1,
        "weight_z": 3,
        "reports": reports,
        "group_sum_enumeration_used": false,
        "unknown_scalar_recovered": false,
        "new_solver_timing_measured": false
    }));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_prime31_public_two_sign_ranks() {
        let reports = calculate();
        for report in reports {
            let got: Vec<_> = report.ranks.iter().map(|s|
                (s.weighted_degree, s.monomials, s.evaluation_rank,
                 s.right_nullity, s.universal_q0_multiples,
                 s.extra_kernel_dimension)).collect();
            assert_eq!(got, vec![
                (7, 15, 14, 1, 1, 0),
                (27, 145, 61, 84, 84, 0),
                (28, 155, 62, 93, 92, 1),
            ], "base {}", report.base);
        }
    }
    #[test]
    fn dimension_and_universal_weight() {
        assert_eq!(monomials(7).len(), 15);
        assert_eq!(monomials(20).len(), 84);
        assert_eq!(monomials(21).len(), 92);
        assert_eq!(monomials(27).len(), 145);
        assert_eq!(monomials(28).len(), 155);
        // The leading weighted term of Q0 is X Z^2, of weight 7.
        // Therefore its multiples within degree D have dimension N(D-7).
    }
}
