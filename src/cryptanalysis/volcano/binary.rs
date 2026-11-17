//! Binary-field side of the volcano tools: irreducible polynomials of any
//! degree, roots of `F_2`-irreducible factors inside `F_{2^n}`, and curves
//! `y² + xy = x³ + a₂x² + 1/j` with a group-order test.

use super::f2poly::F2Poly;
use crate::binary_ecc::curve::{point_add, scalar_mul, BinaryCurve, BinaryPoint};
use crate::binary_ecc::f2m::{F2mElement, IrreduciblePoly};
use crate::binary_ecc::poly_f2m::F2mPoly;
use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

/// Rabin's irreducibility test for `f` of degree `n`.
pub fn is_irreducible(f: &F2Poly, n: usize) -> bool {
    if f.deg() != Some(n) {
        return false;
    }
    let mut h = F2Poly::x();
    let mut sq: Vec<F2Poly> = vec![h.clone()]; // h_k = x^(2^k) mod f
    for _ in 0..n {
        h = h.square().rem(f);
        sq.push(h.clone());
    }
    if sq[n] != F2Poly::x().rem(f) {
        return false;
    }
    let mut m = n;
    let mut p = 2;
    while m > 1 {
        if m.is_multiple_of(p) {
            let g = f.gcd(&sq[n / p].add(&F2Poly::x()));
            if !g.is_one() {
                return false;
            }
            while m.is_multiple_of(p) {
                m /= p;
            }
        }
        p += 1;
    }
    true
}

/// Lowest-weight irreducible trinomial, else pentanomial, of degree `n`.
pub fn find_irreducible(n: u32) -> IrreduciblePoly {
    let nn = n as usize;
    let mk = |terms: &[usize]| {
        let mut f = F2Poly::monomial(nn);
        for &t in terms {
            f = f.add(&F2Poly::monomial(t));
        }
        f
    };
    for k in 1..nn {
        if is_irreducible(&mk(&[k, 0]), nn) {
            return IrreduciblePoly {
                degree: n,
                low_terms: vec![0, k as u32],
            };
        }
    }
    for a in 3..nn {
        for b in 2..a {
            for c in 1..b {
                if is_irreducible(&mk(&[a, b, c, 0]), nn) {
                    return IrreduciblePoly {
                        degree: n,
                        low_terms: vec![0, c as u32, b as u32, a as u32],
                    };
                }
            }
        }
    }
    panic!("no irreducible tri/pentanomial of degree {}", n);
}

fn rand_elt(n: u32, rng: &mut u64) -> F2mElement {
    let words = (n as usize).div_ceil(64);
    let mut w = vec![0u64; words];
    for v in w.iter_mut() {
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let hi = *rng;
        *rng = rng
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *v = (hi & 0xFFFF_FFFF_0000_0000) | (*rng >> 32);
    }
    F2mElement::from_words(&w, n)
}

/// Embed an `F_2` polynomial as a polynomial over `F_{2^n}`.
pub fn lift_poly(g: &F2Poly, n: u32) -> F2mPoly {
    let d = g.deg().unwrap_or(0);
    let coeffs: Vec<F2mElement> = (0..=d)
        .map(|i| {
            if g.coeff(i) {
                F2mElement::one(n)
            } else {
                F2mElement::zero(n)
            }
        })
        .collect();
    F2mPoly::from_coeffs(coeffs, n)
}

/// One root in `F_{2^n}` of a polynomial that splits into linear factors there,
/// by trace-map splitting with random multipliers.
fn one_root(f: &F2mPoly, n: u32, irr: &IrreduciblePoly, rng: &mut u64) -> F2mElement {
    let d = f.degree().expect("zero polynomial");
    if d == 1 {
        let f = f.monic(irr);
        return f.coeff(0);
    }
    loop {
        let c = rand_elt(n, rng);
        if c.is_zero() {
            continue;
        }
        let mut acc = F2mPoly::x(n).scalar_mul(&c, irr).divrem(f, irr).1;
        let mut sum = acc.clone();
        for _ in 1..n {
            acc = acc.mul(&acc, irr).divrem(f, irr).1;
            sum = sum.add(&acc);
        }
        let g = f.gcd(&sum, irr);
        let dg = g.degree().unwrap_or(0);
        if dg > 0 && dg < d {
            let (other, _) = f.divrem(&g, irr);
            let pick = if dg <= d - dg { g } else { other };
            return one_root(&pick, n, irr, rng);
        }
    }
}

/// The whole Frobenius orbit (`d` roots) of an `F_2`-irreducible `g` of degree `d | n`.
pub fn orbit_of(g: &F2Poly, n: u32, irr: &IrreduciblePoly, rng: &mut u64) -> Vec<F2mElement> {
    let d = g.deg().unwrap();
    let alpha = if d == 1 {
        if g.coeff(0) {
            F2mElement::one(n)
        } else {
            F2mElement::zero(n)
        }
    } else {
        one_root(&lift_poly(g, n), n, irr, rng)
    };
    let mut out = Vec::with_capacity(d);
    let mut cur = alpha;
    for _ in 0..d {
        out.push(cur.clone());
        cur = cur.square(irr);
    }
    out
}

pub fn trace(c: &F2mElement, n: u32, irr: &IrreduciblePoly) -> bool {
    let mut acc = c.clone();
    let mut cur = c.clone();
    for _ in 1..n {
        cur = cur.square(irr);
        acc = acc.add(&cur);
    }
    !acc.is_zero()
}

/// `z` with `z² + z = c`, for odd `n` and `Tr(c) = 0`.
pub fn half_trace(c: &F2mElement, n: u32, irr: &IrreduciblePoly) -> F2mElement {
    let mut acc = c.clone();
    let mut cur = c.clone();
    for _ in 0..(n - 1) / 2 {
        cur = cur.square(irr).square(irr);
        acc = acc.add(&cur);
    }
    acc
}

/// `y² + xy = x³ + a₂x² + 1/j` (a₂ ∈ {0, 1}); `j ≠ 0`.
pub fn curve_for_j(j: &F2mElement, a2: bool, n: u32, irr: &IrreduciblePoly) -> BinaryCurve {
    let a6 = j.flt_inverse(irr).expect("j must be nonzero");
    BinaryCurve {
        m: n,
        irreducible: irr.clone(),
        a: if a2 {
            F2mElement::one(n)
        } else {
            F2mElement::zero(n)
        },
        b: a6,
        generator: BinaryPoint::Infinity,
        order: BigUint::one(),
        cofactor: BigUint::one(),
    }
}

pub fn random_point(curve: &BinaryCurve, rng: &mut u64) -> BinaryPoint {
    let n = curve.m;
    let irr = &curve.irreducible;
    loop {
        let x = rand_elt(n, rng);
        if x.is_zero() {
            continue;
        }
        // z^2 + z = x + a2 + b / x^2
        let xinv = x.flt_inverse(irr).unwrap();
        let c = x.add(&curve.a).add(&curve.b.mul(&xinv.square(irr), irr));
        if trace(&c, n, irr) {
            continue;
        }
        let z = half_trace(&c, n, irr);
        let y = x.mul(&z, irr);
        let p = BinaryPoint::Affine { x, y };
        debug_assert!(curve.is_on_curve(&p));
        return p;
    }
}

/// True when `[order]P = O` for `tries` random points (a necessary condition for
/// `#E = order`; a wrong twist fails with overwhelming probability).
pub fn order_test(curve: &BinaryCurve, order: &BigUint, tries: usize, rng: &mut u64) -> bool {
    for _ in 0..tries {
        let p = random_point(curve, rng);
        if scalar_mul(curve, &p, order) != BinaryPoint::Infinity {
            return false;
        }
    }
    true
}

/// `(t_n, u_n)` for the Koblitz curve `y² + xy = x³ + a₂x² + 1` over `F_{2^n}`:
/// `t_n = t₁t_{n-1} − 2t_{n-2}`, `u_n` the Lucas companion, `t_n² − 4·2^n = −7u_n²`.
pub fn koblitz_trace(n: u32, a2: bool) -> (BigInt, BigInt) {
    let t1 = BigInt::from(if a2 { 1 } else { -1 });
    let (mut t0, mut t) = (BigInt::from(2), t1.clone());
    let (mut u0, mut u) = (BigInt::zero(), BigInt::one());
    for _ in 2..=n {
        let tn = &t1 * &t - BigInt::from(2) * &t0;
        t0 = std::mem::replace(&mut t, tn);
        let un = &t1 * &u - BigInt::from(2) * &u0;
        u0 = std::mem::replace(&mut u, un);
    }
    (t, u)
}

/// Group order `2^n + 1 − t_n` of the Koblitz curve.
pub fn koblitz_order(n: u32, a2: bool) -> BigUint {
    let (t, _) = koblitz_trace(n, a2);
    let v = (BigInt::one() << n) + BigInt::one() - t;
    v.to_biguint().unwrap()
}

/// Add two points (used by callers that want to cross-check group laws).
pub fn add_points(c: &BinaryCurve, a: &BinaryPoint, b: &BinaryPoint) -> BinaryPoint {
    point_add(c, a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn irreducible_search_matches_known_degrees() {
        // x^83 + x^7 + x^4 + x^2 + 1 is the standard pentanomial; a lower-weight one may exist,
        // so only check that whatever is found is irreducible and has the right degree.
        for n in [8u32, 13, 53, 83] {
            let irr = find_irreducible(n);
            let mut f = F2Poly::monomial(n as usize);
            for &t in &irr.low_terms {
                f = f.add(&F2Poly::monomial(t as usize));
            }
            assert!(is_irreducible(&f, n as usize), "degree {n}");
        }
        // x^8 + x^4 + x^3 + x + 1 (AES) is irreducible, x^8 + x^4 + x^3 + x^2 + 1 is not... check AES.
        let aes = F2Poly::monomial(8)
            .add(&F2Poly::monomial(4))
            .add(&F2Poly::monomial(3))
            .add(&F2Poly::monomial(1))
            .add(&F2Poly::one());
        assert!(is_irreducible(&aes, 8));
        let red = F2Poly::monomial(4)
            .add(&F2Poly::monomial(2))
            .add(&F2Poly::one()); // (x^2+x+1)^2
        assert!(!is_irreducible(&red, 4));
    }

    #[test]
    fn koblitz_traces_satisfy_the_cm_identity() {
        for n in [1u32, 2, 3, 5, 13, 53, 83, 131] {
            for a2 in [false, true] {
                let (t, u) = koblitz_trace(n, a2);
                let lhs = &t * &t - (BigInt::from(4) << n);
                assert_eq!(lhs, BigInt::from(-7) * &u * &u, "n={n} a2={a2}");
            }
        }
        // E(F_2) for a2 = 0 has 4 points, for a2 = 1 has 2.
        assert_eq!(koblitz_order(1, false), BigUint::from(4u32));
        assert_eq!(koblitz_order(1, true), BigUint::from(2u32));
    }

    #[test]
    fn koblitz_order_test_distinguishes_twists() {
        let n = 13u32;
        let irr = find_irreducible(n);
        let one = F2mElement::one(n);
        let mut rng = 99u64;
        let nn = koblitz_order(n, false);
        // j = 1: a2 = 0 is K_0 with order nn; a2 = 1 is its twist.
        let c0 = curve_for_j(&one, false, n, &irr);
        let c1 = curve_for_j(&one, true, n, &irr);
        assert!(order_test(&c0, &nn, 6, &mut rng));
        assert!(!order_test(&c1, &nn, 6, &mut rng));
    }
}
