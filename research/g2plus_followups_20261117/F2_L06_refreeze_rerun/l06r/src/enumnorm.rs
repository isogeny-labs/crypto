//! F2: point counts N2, N3 by enumeration with the norm-based quadratic character
//! (SPEC.md section 5.2). No square tables: x in F_{p^k}^* is a square iff its norm to
//! F_p is a square in F_p. Independent of enumchi.rs (which uses square tables) so that
//! gate G1 compares two implementations.
use crate::fp::Fp;
use crate::poly::Poly;

/// N2 = #C(F_{p^2}) for y^2 = f(x), f monic of degree 7 (one point at infinity).
/// F_{p^2} = F_p[t]/(t^2 - nu); norm(a0 + a1 t) = a0^2 - nu a1^2.
pub fn n2_norm(fp: &Fp, f: &Poly) -> i128 {
    let p = fp.p;
    let nu = fp.nu;
    let cf: Vec<u64> = (0..7).map(|i| f.get(i)).collect();
    assert!(f.n == 8 && f.get(7) == 1);
    let mut n: i128 = 1;
    for x1 in 0..p {
        let nx1 = x1 * nu % p;
        for x0 in 0..p {
            let (mut a0, mut a1) = (1u64, 0u64);
            for i in (0..7).rev() {
                let b0 = (a0 * x0 + a1 * nx1) % p;
                let b1 = (a0 * x1 + a1 * x0) % p;
                a0 = (b0 + cf[i]) % p;
                a1 = b1;
            }
            if a0 == 0 && a1 == 0 {
                n += 1;
            } else {
                let nm = (a0 * a0 + (p - a1 * a1 % p) * nu) % p;
                if fp.legendre(nm) == 1 {
                    n += 2;
                }
            }
        }
    }
    n
}

/// The F_{p^3} modulus t^3 + c1 t + c0: first (c1 outer = 0,1,..; c0 inner = 1,2,..)
/// without a root in F_p (same rule as enumchi.rs).
pub fn cubic_modulus(p: u64) -> (u64, u64) {
    for a1 in 0..p {
        for a0 in 1..p {
            let mut root = false;
            for t in 0..p {
                if (t * t % p * t + a1 * t + a0) % p == 0 {
                    root = true;
                    break;
                }
            }
            if !root {
                return (a1, a0);
            }
        }
    }
    panic!("no irreducible cubic");
}

#[inline]
fn mul3(a: [u64; 3], b: [u64; 3], c1: u64, c0: u64, p: u64) -> [u64; 3] {
    // t^3 = -c1 t - c0
    let d0 = a[0] * b[0];
    let d1 = a[0] * b[1] + a[1] * b[0];
    let d2 = a[0] * b[2] + a[1] * b[1] + a[2] * b[0];
    let d3 = (a[1] * b[2] + a[2] * b[1]) % p;
    let d4 = (a[2] * b[2]) % p;
    // t^4 = -c1 t^2 - c0 t
    let r0 = (d0 + (p - c0) * d3) % p;
    let r1 = (d1 + (p - c1) * d3 + (p - c0) * d4) % p;
    let r2 = (d2 + (p - c1) * d4) % p;
    [r0, r1, r2]
}

/// times t: (b0, b1, b2) t = (-c0 b2, b0 - c1 b2, b1)
#[inline]
fn mul_t(b: [u64; 3], c1: u64, c0: u64, p: u64) -> [u64; 3] {
    [(p - c0) * b[2] % p, (b[0] + (p - c1) * b[2]) % p, b[1]]
}

/// Norm F_{p^3} -> F_p = determinant of multiplication by a in the basis 1, t, t^2.
pub fn norm3(a: [u64; 3], c1: u64, c0: u64, p: u64) -> u64 {
    let v0 = a;
    let v1 = mul_t(v0, c1, c0, p);
    let v2 = mul_t(v1, c1, c0, p);
    // columns v0, v1, v2; det = sum over permutations
    let m = |i: usize, j: usize| -> i128 {
        [v0, v1, v2][j][i] as i128
    };
    let det = m(0, 0) * (m(1, 1) * m(2, 2) - m(1, 2) * m(2, 1))
        - m(0, 1) * (m(1, 0) * m(2, 2) - m(1, 2) * m(2, 0))
        + m(0, 2) * (m(1, 0) * m(2, 1) - m(1, 1) * m(2, 0));
    det.rem_euclid(p as i128) as u64
}

/// N3 = #C(F_{p^3}).
pub fn n3_norm(fp: &Fp, f: &Poly) -> i128 {
    let p = fp.p;
    let (c1, c0) = cubic_modulus(p);
    let cf: Vec<u64> = (0..7).map(|i| f.get(i)).collect();
    assert!(f.n == 8 && f.get(7) == 1);
    let mut n: i128 = 1;
    for x2 in 0..p {
        for x1 in 0..p {
            for x0 in 0..p {
                let x = [x0, x1, x2];
                let mut acc = [1u64, 0, 0];
                for i in (0..7).rev() {
                    acc = mul3(acc, x, c1, c0, p);
                    acc[0] = (acc[0] + cf[i]) % p;
                }
                if acc == [0, 0, 0] {
                    n += 1;
                } else if fp.legendre(norm3(acc, c1, c0, p)) == 1 {
                    n += 2;
                }
            }
        }
    }
    n
}

/// s2 from N1 (via s1) and N2: S2 = p^2 + 1 - N2, s2 = (s1^2 - S2) / 2.
pub fn s2_from(p: i128, s1: i128, n2: i128) -> i128 {
    let sk2 = p * p + 1 - n2;
    let t = s1 * s1 - sk2;
    assert!(t % 2 == 0);
    t / 2
}

/// s3 from s1, s2, N2, N3 by Newton: s3 = (s2 S1 - s1 S2 + S3) / 3.
pub fn s3_from(p: i128, s1: i128, s2: i128, n2: i128, n3: i128) -> i128 {
    let sk1 = s1;
    let sk2 = p * p + 1 - n2;
    let sk3 = p * p * p + 1 - n3;
    let t = s2 * sk1 - s1 * sk2 + sk3;
    assert!(t % 3 == 0);
    t / 3
}
