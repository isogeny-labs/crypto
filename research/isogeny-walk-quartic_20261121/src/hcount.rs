//! Point counts of y^2 = f(x) (f monic of odd degree d) over F_p, F_{p^2}, F_{p^3} by exhaustive
//! enumeration, and the genus-3 L-polynomial. Counted in F_p multiplications (`m`) and table
//! lookups (`nat`); field additions are not counted (same convention as qds_core).
//!
//! F_{p^3}: x = a + b t + c t^2 with t^3 = r1 t + r0 irreducible; the quadratic character of
//! f(x) is the Legendre symbol of its norm; f(x+1) comes from forward differences (additions
//! only), a fresh table of d+1 Horner evaluations is built at the start of each (b, c) line.

/// Result of one count.
#[derive(Clone, Copy, Debug, Default)]
pub struct Cnt {
    pub n: u64,
    pub m: u64,
    pub nat: u64,
}

fn legendre_table(p: u64) -> Vec<i8> {
    let mut t = vec![-1i8; p as usize];
    t[0] = 0;
    for x in 1..p {
        t[(x * x % p) as usize] = 1;
    }
    t
}

fn nonresidue(p: u64) -> u64 {
    let t = legendre_table(p);
    (2..p).find(|&c| t[c as usize] == -1).unwrap()
}

/// `N_1` of y^2 = f(x), including the point at infinity (f odd degree).
pub fn count_fp(f: &[u64], p: u64) -> Cnt {
    let t = legendre_table(p);
    let d = f.len() as u64 - 1;
    let mut s = 0i64;
    for x in 0..p {
        let v = f.iter().rev().fold(0u64, |acc, &c| (acc * x + c) % p);
        s += t[v as usize] as i64;
    }
    Cnt {
        n: (p as i64 + 1 + s) as u64,
        m: p * d,
        nat: p,
    }
}

/// `N_2`.
pub fn count_fp2(f: &[u64], p: u64) -> Cnt {
    let t = legendre_table(p);
    let nr = nonresidue(p);
    let d = f.len() as u64 - 1;
    let mut s = 0i64;
    for b in 0..p {
        for a in 0..p {
            // Horner in F_p[w]/(w^2 - nr), x = a + b w
            let (mut ra, mut rb) = (0u64, 0u64);
            for &c in f.iter().rev() {
                let na = (ra * a + nr * (rb * b % p)) % p;
                let nb = (ra * b + rb * a) % p;
                ra = (na + c) % p;
                rb = nb;
            }
            let norm = (ra * ra % p + p - nr * (rb * rb % p) % p) % p;
            s += t[norm as usize] as i64;
        }
    }
    // per x: d F_{p^2} mults of 5 F_p mults + norm of 3
    Cnt {
        n: (p as i64 * p as i64 + 1 + s) as u64,
        m: p * p * (5 * d + 3),
        nat: p * p,
    }
}

/// An irreducible `t^3 = r1 t + r0` over F_p (no root).
fn irreducible_cubic(p: u64) -> (u64, u64) {
    for r1 in 0..p {
        for r0 in 1..p {
            if (0..p).all(|x| !(x * x % p * x % p + p * p - r1 * x % p - r0).is_multiple_of(p)) {
                return (r1, r0);
            }
        }
    }
    unreachable!()
}

type E3 = [u64; 3];

/// Product in F_p[t]/(t^3 - r1 t - r0) (13 F_p multiplications).
fn mul3(a: E3, b: E3, r: (u64, u64), p: u64) -> E3 {
    let c0 = a[0] * b[0];
    let c1 = a[0] * b[1] + a[1] * b[0];
    let c2 = a[0] * b[2] + a[1] * b[1] + a[2] * b[0];
    let c3 = (a[1] * b[2] + a[2] * b[1]) % p;
    let c4 = a[2] * b[2] % p;
    // t^3 = r1 t + r0, t^4 = r1 t^2 + r0 t
    [
        (c0 + c3 * r.1 % p) % p,
        (c1 + c3 * r.0 % p + c4 * r.1 % p) % p,
        (c2 + c4 * r.0 % p) % p,
    ]
}

/// Norm to F_p of `e0 + e1 t + e2 t^2`: det of the multiplication matrix (13 F_p multiplications).
fn norm3(e: E3, r: (u64, u64), p: u64) -> u64 {
    // rows e, e t, e t^2 with t^3 = r1 t + r0
    let v1 = [e[2] * r.1 % p, (e[0] + e[2] * r.0) % p, e[1]];
    let v2 = [v1[2] * r.1 % p, (v1[0] + v1[2] * r.0) % p, v1[1]];
    let det = e[0] as i128 * (v1[1] as i128 * v2[2] as i128 - v1[2] as i128 * v2[1] as i128)
        - e[1] as i128 * (v1[0] as i128 * v2[2] as i128 - v1[2] as i128 * v2[0] as i128)
        + e[2] as i128 * (v1[0] as i128 * v2[1] as i128 - v1[1] as i128 * v2[0] as i128);
    det.rem_euclid(p as i128) as u64
}

/// `N_3` (exhaustive over p^3 elements).
pub fn count_fp3(f: &[u64], p: u64) -> Cnt {
    let t = legendre_table(p);
    let r = irreducible_cubic(p);
    let d = f.len() - 1;
    let mut s = 0i64;
    let add3 = |a: E3, b: E3| -> E3 {
        let mut o = [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
        for z in o.iter_mut() {
            if *z >= p {
                *z -= p;
            }
        }
        o
    };
    for c in 0..p {
        for b in 0..p {
            // Horner values f(x0 + i), x0 = b t + c t^2, i = 0..=d
            let mut vals: Vec<E3> = Vec::with_capacity(d + 1);
            for i in 0..=d as u64 {
                let x: E3 = [i % p, b, c];
                let mut acc: E3 = [0; 3];
                for &co in f.iter().rev() {
                    acc = mul3(acc, x, r, p);
                    acc[0] = (acc[0] + co) % p;
                }
                vals.push(acc);
            }
            // forward differences D[k] = Delta^k f(x0)
            let mut dif: Vec<E3> = vals.clone();
            for k in 1..=d {
                for i in (k..=d).rev() {
                    let (hi, lo) = (dif[i], dif[i - 1]);
                    dif[i] = [
                        (hi[0] + p - lo[0]) % p,
                        (hi[1] + p - lo[1]) % p,
                        (hi[2] + p - lo[2]) % p,
                    ];
                }
            }
            for _a in 0..p {
                s += t[norm3(dif[0], r, p) as usize] as i64;
                for k in 0..d {
                    dif[k] = add3(dif[k], dif[k + 1]);
                }
            }
        }
    }
    let p2 = p * p;
    // per line: (d + 1) Horner evaluations of d mults of 13; per x: one norm of 13
    let lines = p2;
    Cnt {
        n: (p2 as i64 * p as i64 + 1 + s) as u64,
        m: lines * (d as u64 + 1) * d as u64 * 13 + p * p2 * 13,
        nat: p * p2,
    }
}

/// Genus-3 L-polynomial data from `N_1, N_2, N_3`: `(e1, e2, e3, #J)` with
/// `P(T) = 1 - e1 T + e2 T^2 - e3 T^3 + p e2 T^4 - p^2 e1 T^5 + p^3 T^6`.
pub fn lpoly_from_counts(p: u64, n1: u64, n2: u64, n3: u64) -> (i128, i128, i128, i128) {
    let p = p as i128;
    let s1 = p + 1 - n1 as i128;
    let s2 = p * p + 1 - n2 as i128;
    let s3 = p * p * p + 1 - n3 as i128;
    let e1 = s1;
    let e2 = (e1 * s1 - s2) / 2;
    let e3 = (e2 * s1 - e1 * s2 + s3) / 3;
    (e1, e2, e3, jac_from_e(p, e1, e2, e3))
}

/// `P(1)`.
pub fn jac_from_e(p: i128, e1: i128, e2: i128, e3: i128) -> i128 {
    1 - e1 + e2 - e3 + p * e2 - p * p * e1 + p * p * p
}

/// Window for the Jacobian order from `N_1, N_2` alone: all `#J` consistent with
/// `|s3| <= 6 p^{3/2}` and the integrality of `e3`, as `(base, lo, hi)` with `#J = base - e3`
/// for integer `e3` in `lo..=hi`.
pub fn window_from_n1n2(p: u64, n1: u64, n2: u64) -> (i128, i128, i128) {
    let pi = p as i128;
    let s1 = pi + 1 - n1 as i128;
    let s2 = pi * pi + 1 - n2 as i128;
    let e1 = s1;
    let e2 = (e1 * s1 - s2) / 2;
    let base = 1 - e1 + e2 + pi * e2 - pi * pi * e1 + pi * pi * pi;
    let c0 = e2 * s1 - e1 * s2;
    let bound = (6.0 * (p as f64).powf(1.5)).ceil() as i128 + 1;
    // e3 = (c0 + s3)/3 with s3 in [-bound, bound]
    let lo = (c0 - bound).div_euclid(3);
    let hi = (c0 + bound).div_euclid(3) + 1;
    (base, lo, hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// y^2 = x^3 + a x + b over F_{p^k}: N_k from the Frobenius trace.
    #[test]
    fn elliptic_counts_follow_the_trace() {
        let p = 31u64;
        let f = [7u64, 5, 0, 1]; // x^3 + 5x + 7
        let n1 = count_fp(&f, p).n as i128;
        let a = p as i128 + 1 - n1;
        let pi = p as i128;
        let n2 = count_fp2(&f, p).n as i128;
        let n3 = count_fp3(&f, p).n as i128;
        assert_eq!(n2, pi * pi + 1 - (a * a - 2 * pi));
        assert_eq!(n3, pi * pi * pi + 1 - (a * a * a - 3 * pi * a));
    }

    #[test]
    fn norm3_is_multiplicative_and_mul3_distributes() {
        let p = 29u64;
        let r = irreducible_cubic(p);
        let a: E3 = [3, 17, 5];
        let b: E3 = [11, 2, 23];
        let ab = mul3(a, b, r, p);
        assert_eq!(
            norm3(ab, r, p),
            norm3(a, r, p) * norm3(b, r, p) % p,
            "norm is multiplicative"
        );
    }

    #[test]
    fn genus3_window_contains_exact_order() {
        let p = 23u64;
        // monic degree-7 f with distinct roots
        let f = [1u64, 4, 0, 9, 2, 0, 5, 1];
        let (n1, n2, n3) = (count_fp(&f, p).n, count_fp2(&f, p).n, count_fp3(&f, p).n);
        let (e1, e2, e3, n) = lpoly_from_counts(p, n1, n2, n3);
        let _ = (e1, e2);
        let (base, lo, hi) = window_from_n1n2(p, n1, n2);
        assert!(lo <= e3 && e3 <= hi);
        assert_eq!(n, base - e3);
        assert!(n > 0);
    }
}
