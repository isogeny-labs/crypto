//! The group law of a long Weierstrass model over any `Fq` (every
//! characteristic), division polynomials from the b-invariants, and the
//! modular helpers GLV needs. Catalogue checks, not a performance path.

use crate::field::{Elt, Fq};
use anyhow::{bail, Result};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

/// y^2 + a1 xy + a3 y = x^3 + a2 x^2 + a4 x + a6 over `k`.
pub struct Curve<'a> {
    pub k: &'a Fq,
    pub a: [Elt; 5],
}

/// An affine point; `None` is the point at infinity.
pub type Point = Option<(Elt, Elt)>;

impl Curve<'_> {
    pub fn on_curve(&self, p: &Point) -> bool {
        let Some((x, y)) = p else { return true };
        let k = self.k;
        let [a1, a2, a3, a4, a6] = &self.a;
        let lhs = k.add(&k.mul(y, y), &k.add(&k.mul(&k.mul(a1, x), y), &k.mul(a3, y)));
        let x2 = k.mul(x, x);
        let rhs = k.add(&k.add(&k.mul(&x2, x), &k.mul(a2, &x2)), &k.add(&k.mul(a4, x), a6));
        lhs == rhs
    }

    pub fn neg(&self, p: &Point) -> Point {
        let k = self.k;
        p.as_ref().map(|(x, y)| (x.clone(), k.sub(&k.neg(y), &k.add(&k.mul(&self.a[0], x), &self.a[2]))))
    }

    pub fn add(&self, p: &Point, q: &Point) -> Result<Point> {
        let k = self.k;
        let [a1, a2, a3, a4, a6] = &self.a;
        let (Some((x1, y1)), Some((x2, y2))) = (p, q) else {
            return Ok(if p.is_none() { q.clone() } else { p.clone() });
        };
        if self.neg(p) == *q {
            return Ok(None);
        }
        let (lambda, nu) = if x1 != x2 {
            let d = k.inv(&k.sub(x2, x1))?;
            let l = k.mul(&k.sub(y2, y1), &d);
            let n = k.mul(&k.sub(&k.mul(y1, x2), &k.mul(y2, x1)), &d);
            (l, n)
        } else {
            // doubling: denominator 2y + a1 x + a3 (nonzero, since P != -P)
            let den = k.add(&k.add(&k.scale(y1, 2), &k.mul(a1, x1)), a3);
            let d = k.inv(&den)?;
            let x1sq = k.mul(x1, x1);
            let num_l = k.sub(&k.add(&k.add(&k.scale(&x1sq, 3), &k.scale(&k.mul(a2, x1), 2)), a4), &k.mul(a1, y1));
            let num_n =
                k.sub(&k.add(&k.add(&k.neg(&k.mul(&x1sq, x1)), &k.mul(a4, x1)), &k.scale(a6, 2)), &k.mul(a3, y1));
            (k.mul(&num_l, &d), k.mul(&num_n, &d))
        };
        let x3 = k.sub(&k.sub(&k.sub(&k.add(&k.mul(&lambda, &lambda), &k.mul(a1, &lambda)), a2), x1), x2);
        let y3 = k.sub(&k.sub(&k.neg(&k.mul(&k.add(&lambda, a1), &x3)), &nu), a3);
        Ok(Some((x3, y3)))
    }

    pub fn mul(&self, n: &BigInt, p: &Point) -> Result<Point> {
        if n.is_negative() {
            return self.mul(&-n, &self.neg(p));
        }
        let mut acc: Point = None;
        for i in (0..n.bits()).rev() {
            acc = self.add(&acc, &acc)?;
            if n.bit(i) {
                acc = self.add(&acc, p)?;
            }
        }
        Ok(acc)
    }
}

// ------------------------------------------------------------ polynomials over Fq

fn padd(k: &Fq, a: &[Elt], b: &[Elt]) -> Vec<Elt> {
    let n = a.len().max(b.len());
    (0..n)
        .map(|i| match (a.get(i), b.get(i)) {
            (Some(x), Some(y)) => k.add(x, y),
            (Some(x), None) | (None, Some(x)) => x.clone(),
            _ => unreachable!(),
        })
        .collect()
}

fn pneg(k: &Fq, a: &[Elt]) -> Vec<Elt> {
    a.iter().map(|x| k.neg(x)).collect()
}

fn pmul(k: &Fq, a: &[Elt], b: &[Elt]) -> Vec<Elt> {
    let mut c = vec![k.zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        if k.is_zero(x) {
            continue;
        }
        for (j, y) in b.iter().enumerate() {
            c[i + j] = k.add(&c[i + j], &k.mul(x, y));
        }
    }
    c
}

fn ptrim(k: &Fq, mut a: Vec<Elt>) -> Vec<Elt> {
    while a.len() > 1 && a.last().is_some_and(|c| k.is_zero(c)) {
        a.pop();
    }
    a
}

/// Evaluate an ascending polynomial at x.
pub fn peval(k: &Fq, a: &[Elt], x: &Elt) -> Elt {
    a.iter().rev().fold(k.zero(), |acc, c| k.add(&k.mul(&acc, x), c))
}

/// Division polynomials in x alone, from b2, b4, b6, b8 (Silverman, AEC,
/// Exercise 3.7): `F = psi_2^2 = 4x^3 + b2 x^2 + 2 b4 x + b6`, `psi_3`,
/// `psi_4 / psi_2`, and `psi_5 = (psi_4/psi_2) F^2 - psi_3^3`. Odd n: psi_n;
/// even n: psi_n / psi_2. Ascending coefficients, trailing zeros trimmed.
pub struct DivisionPolynomials {
    pub psi2_squared: Vec<Elt>,
    pub psi3: Vec<Elt>,
    pub psi4_over_psi2: Vec<Elt>,
    pub psi5: Vec<Elt>,
}

pub fn division_polynomials(k: &Fq, b2: &Elt, b4: &Elt, b6: &Elt, b8: &Elt) -> DivisionPolynomials {
    let c = |n: i64| k.from_int(n);
    let f = vec![b6.clone(), k.scale(b4, 2), b2.clone(), c(4)];
    let psi3 = vec![b8.clone(), k.scale(b6, 3), k.scale(b4, 3), b2.clone(), c(3)];
    let psi4 = vec![
        k.sub(&k.mul(b4, b8), &k.mul(b6, b6)),
        k.sub(&k.mul(b2, b8), &k.mul(b4, b6)),
        k.scale(b8, 10),
        k.scale(b6, 10),
        k.scale(b4, 5),
        b2.clone(),
        c(2),
    ];
    let f2 = pmul(k, &f, &f);
    let psi3_cubed = pmul(k, &pmul(k, &psi3, &psi3), &psi3);
    let psi5 = padd(k, &pmul(k, &psi4, &f2), &pneg(k, &psi3_cubed));
    DivisionPolynomials {
        psi2_squared: ptrim(k, f),
        psi3: ptrim(k, psi3),
        psi4_over_psi2: ptrim(k, psi4),
        psi5: ptrim(k, psi5),
    }
}

// ------------------------------------------------------------ integers mod a prime

pub fn modpow(b: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {
    b.mod_floor(m).modpow(e, m)
}

/// A square root of `a` modulo the odd prime `p` (Tonelli–Shanks), if one exists.
pub fn sqrt_mod(a: &BigInt, p: &BigInt) -> Option<BigInt> {
    let a = a.mod_floor(p);
    if a.is_zero() {
        return Some(BigInt::zero());
    }
    let one = BigInt::one();
    let pm1 = p - &one;
    if modpow(&a, &(&pm1 >> 1), p) != one {
        return None;
    }
    let (mut q, mut s) = (pm1.clone(), 0u32);
    while q.is_even() {
        q >>= 1;
        s += 1;
    }
    let mut z = BigInt::from(2);
    while modpow(&z, &(&pm1 >> 1), p) == one {
        z += 1;
    }
    let (mut m, mut c) = (s, modpow(&z, &q, p));
    let mut t = modpow(&a, &q, p);
    let mut r = modpow(&a, &((&q + &one) >> 1), p);
    while t != one {
        let (mut i, mut t2) = (0u32, t.clone());
        while t2 != one {
            t2 = (&t2 * &t2).mod_floor(p);
            i += 1;
            if i == m {
                return None;
            }
        }
        let b = modpow(&c, &(BigInt::one() << (m - i - 1)), p);
        m = i;
        c = (&b * &b).mod_floor(p);
        t = (&t * &c).mod_floor(p);
        r = (&r * &b).mod_floor(p);
    }
    Some(r)
}

/// A primitive k-th root of unity modulo the prime p (k | p - 1, k in {3, 4}).
pub fn root_of_unity(k: u32, p: &BigInt) -> Result<BigInt> {
    let pm1 = p - BigInt::one();
    if !(&pm1 % k).is_zero() {
        bail!("{k} does not divide p - 1");
    }
    let e = &pm1 / k;
    let mut g = BigInt::from(2);
    loop {
        let r = modpow(&g, &e, p);
        // order exactly k: r != 1 and, for k = 4, r^2 != 1
        if !r.is_one() && (k != 4 || !modpow(&r, &BigInt::from(2), p).is_one()) {
            return Ok(r);
        }
        g += 1;
    }
}

/// Short basis of the GLV lattice {(a, b) : a + b lambda = 0 mod r}
/// (Gallant–Lambert–Vanstone, extended Euclid stopped at sqrt r).
pub fn glv_basis(r: &BigInt, lambda: &BigInt) -> [(BigInt, BigInt); 2] {
    let sqrt_r = r.sqrt();
    // remainders r_i and coefficients t_i with s_i r + t_i lambda = r_i
    let (mut r0, mut r1) = (r.clone(), lambda.mod_floor(r));
    let (mut t0, mut t1) = (BigInt::zero(), BigInt::one());
    while r1 >= sqrt_r {
        let q = &r0 / &r1;
        let r2 = &r0 - &q * &r1;
        let t2 = &t0 - &q * &t1;
        r0 = std::mem::replace(&mut r1, r2);
        t0 = std::mem::replace(&mut t1, t2);
    }
    // now r0 >= sqrt r > r1: v1 = (r1, -t1); v2 the shorter of (r0, -t0) and the next pair
    let v1 = (r1.clone(), -t1.clone());
    let q = &r0 / &r1;
    let (r2, t2) = (&r0 - &q * &r1, &t0 - &q * &t1);
    let norm = |a: &BigInt, b: &BigInt| a * a + b * b;
    let v2 = if norm(&r0, &t0) <= norm(&r2, &t2) { (r0, -t0) } else { (r2, -t2) };
    [v1, v2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{a_invariants, invariants_from_a};

    fn bi(s: &str) -> BigInt {
        s.parse().unwrap()
    }

    fn small_curve(p: i64, a: i64, b: i64) -> (Fq, [Elt; 5]) {
        let k = Fq::prime(BigInt::from(p));
        let mut c = serde_json::Map::new();
        c.insert("a".into(), a.to_string().into());
        c.insert("b".into(), b.to_string().into());
        let (ai, _) = a_invariants(&k, "short_weierstrass", &c).unwrap();
        (k, ai)
    }

    fn all_points(e: &Curve) -> Vec<Point> {
        let p: i64 = e.k.p.to_string().parse().unwrap();
        let mut out = vec![None];
        for x in 0..p {
            for y in 0..p {
                let pt = Some((e.k.from_int(x), e.k.from_int(y)));
                if e.on_curve(&pt) {
                    out.push(pt);
                }
            }
        }
        out
    }

    fn order(e: &Curve, p: &Point) -> u64 {
        let mut q = p.clone();
        let mut n = 1;
        while q.is_some() {
            q = e.add(&q, p).unwrap();
            n += 1;
        }
        n
    }

    #[test]
    fn division_polynomials_vanish_on_torsion() {
        // y^2 = x^3 + 2x + 3 over GF(97) and y^2 = x^3 + x + 1 over GF(101):
        // psi_n(x(P)) = 0 exactly for P of order n (n odd), psi_4/psi_2 for order 4.
        for (p, a, b) in [(97, 2, 3), (101, 1, 1), (103, 5, 7)] {
            let (k, ai) = small_curve(p, a, b);
            let e = Curve { k: &k, a: ai.clone() };
            let inv = invariants_from_a(&k, ai).unwrap();
            let d = division_polynomials(&k, &inv.b2, &inv.b4, &inv.b6, &inv.b8);
            for pt in all_points(&e).iter().skip(1) {
                let (x, _) = pt.as_ref().unwrap();
                let n = order(&e, pt);
                assert_eq!(k.is_zero(&peval(&k, &d.psi3, x)), n == 3, "p={p} psi3 n={n}");
                assert_eq!(k.is_zero(&peval(&k, &d.psi5, x)), n == 5, "p={p} psi5 n={n}");
                assert_eq!(k.is_zero(&peval(&k, &d.psi4_over_psi2, x)), n == 4, "p={p} psi4 n={n}");
                assert_eq!(k.is_zero(&peval(&k, &d.psi2_squared, x)), n == 2, "p={p} psi2 n={n}");
            }
        }
    }

    #[test]
    fn summation_polynomial_s3_vanishes_on_sums() {
        // S_3(x1, x2, x3) = 0 when P1 + P2 + P3 = O (Semaev 2004).
        let (k, ai) = small_curve(97, 2, 3);
        let e = Curve { k: &k, a: ai };
        let pts = all_points(&e);
        let (a, b) = (k.from_int(2), k.from_int(3));
        let mut checked = 0;
        for p1 in pts.iter().skip(1).take(12) {
            for p2 in pts.iter().skip(1).take(12) {
                let p3 = e.neg(&e.add(p1, p2).unwrap());
                let (Some((x1, _)), Some((x2, _)), Some((x3, _))) = (p1, p2, &p3) else { continue };
                let v = crate::import::enrich::eval_s3(&k, x1, x2, x3, &a, &b);
                assert!(k.is_zero(&v));
                checked += 1;
            }
        }
        assert!(checked > 50);
    }

    #[test]
    fn secp256k1_glv_constants() {
        // beta and lambda of secp256k1 as published with the GLV method (SEC 2 / libsecp256k1).
        let p = bi("115792089237316195423570985008687907853269984665640564039457584007908834671663");
        let r = bi("115792089237316195423570985008687907852837564279074904382605163141518161494337");
        let k = Fq::prime(p.clone());
        let a = [k.zero(), k.zero(), k.zero(), k.zero(), k.from_int(7)];
        let e = Curve { k: &k, a };
        let g: Point = Some((
            k.parse("55066263022277343669578718895168534326250603453777594175500187360389116729240").unwrap(),
            k.parse("32670510020758816978083085130507043184471273380659243275938904335757337482424").unwrap(),
        ));
        let (beta, lambda) = crate::import::enrich::glv_j0(&e, &p, &r, &g).unwrap();
        let h = |s: &str| BigInt::parse_bytes(s.as_bytes(), 16).unwrap();
        let beta0 = h("7ae96a2b657c07106e64479eac3434e99cf0497512f58995c1396c28719501ee");
        let lambda0 = h("5363ad4cc05c30e0a5261c028812645a122e22ea20816678df02967c1b23bd72");
        // the conjugate pair (beta^2, lambda^2) = (p - 1 - beta, r - 1 - lambda) is equally valid
        let pairs = [(beta0.clone(), lambda0.clone()), (&p - 1 - &beta0, &r - 1 - &lambda0)];
        assert!(pairs.contains(&(beta.clone(), lambda.clone())), "beta {beta:x} lambda {lambda:x}");
        let [v1, v2] = glv_basis(&r, &lambda);
        for (a, b) in [v1, v2] {
            assert!((&a + &b * &lambda).mod_floor(&r).is_zero());
            assert!(a.bits() <= 130 && b.bits() <= 130);
        }
    }

    #[test]
    fn sqrt_mod_works() {
        let p = bi("1000003");
        for a in [4u32, 9, 2, 5, 123456] {
            if let Some(s) = sqrt_mod(&BigInt::from(a), &p) {
                assert_eq!((&s * &s).mod_floor(&p), BigInt::from(a).mod_floor(&p));
            }
        }
        assert!(sqrt_mod(&BigInt::from(5), &BigInt::from(13)).is_none());
    }
}
