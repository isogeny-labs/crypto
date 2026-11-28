//! Public instance generation (not charged to any arm): field, curve,
//! point count, generator, subfield factor-base support.
use crate::binary_ecc::IrreduciblePoly;
use crate::cryptanalysis::binary_radical_root::ArtinSchreier;
use crate::cryptanalysis::semaev_decomp::Gf2;
use crate::field::{Curve, Pt, CF};
use crate::seeds::Sm64;
use std::collections::HashMap;

fn pmulmod(a: u128, b: u128, f: u128, n: u32) -> u128 {
    // a, b < 2^n, f of degree n.
    let mut r = 0u128;
    let mut a = a;
    let mut b = b;
    while b != 0 {
        if b & 1 == 1 {
            r ^= a;
        }
        b >>= 1;
        a <<= 1;
        if (a >> n) & 1 == 1 {
            a ^= f;
        }
    }
    r
}

fn pgcd(mut a: u128, mut b: u128) -> u128 {
    let deg = |x: u128| 127 - x.leading_zeros() as i32;
    while b != 0 {
        while a != 0 && deg(a) >= deg(b) {
            a ^= b << (deg(a) - deg(b));
        }
        std::mem::swap(&mut a, &mut b);
    }
    a
}

/// Rabin's irreducibility test over F₂.
pub fn is_irreducible(f: u128, n: u32) -> bool {
    let x = 2u128;
    // x^(2^k) mod f
    let pow2k = |k: u32| {
        let mut t = x;
        for _ in 0..k {
            t = pmulmod(t, t, f, n);
        }
        t
    };
    if pow2k(n) != x {
        return false;
    }
    let mut m = n;
    let mut primes = vec![];
    let mut d = 2;
    while d * d <= m {
        if m % d == 0 {
            primes.push(d);
            while m % d == 0 {
                m /= d;
            }
        }
        d += 1;
    }
    if m > 1 {
        primes.push(m);
    }
    for p in primes {
        let t = pow2k(n / p) ^ x;
        if pgcd(f, t) != 1 {
            return false;
        }
    }
    true
}

/// Lowest-weight, then lexicographically smallest, irreducible
/// trinomial or pentanomial of degree n (same search order as
/// `koblitz_index_calculus::find_irreducible_sparse`).
pub fn irreducible(n: u32) -> IrreduciblePoly {
    let mut cands: Vec<u64> = vec![1];
    for i in 1..n {
        cands.push(1 | (1 << i));
        for j in (i + 1)..n {
            cands.push(1 | (1 << i) | (1 << j));
            for k in (j + 1)..n {
                cands.push(1 | (1 << i) | (1 << j) | (1 << k));
            }
        }
    }
    cands.sort_unstable();
    for low in cands {
        let f = (1u128 << n) | low as u128;
        if is_irreducible(f, n) {
            return IrreduciblePoly { degree: n, low_terms: (0..n).filter(|i| (low >> i) & 1 == 1).collect() };
        }
    }
    panic!("no sparse irreducible");
}

fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}
pub fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let (mut d, mut s) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

/// `b` lies in no proper subfield of `F_{2^n}`.
fn generic_b(gf: &Gf2, b: u64) -> bool {
    let n = gf.n;
    (1..n).filter(|d| n % d == 0).all(|d| gf.sqr_k(b, d) != b)
}

/// Group order by baby-step giant-step on one point, Hasse interval.
/// Returns all `N` in the interval with `[N]P = O`.
fn orders_killing(c: &Curve, cf: &CF, p: Pt) -> Vec<u64> {
    let q = 1u64 << c.n;
    let w = (2.0 * (q as f64).sqrt()).ceil() as i64 + 1; // |t| <= w
    let m = ((w as f64).sqrt().ceil() as i64).max(2);
    // baby: x(jP) -> j, j = 1..m
    let mut baby: HashMap<u64, i64> = HashMap::new();
    let mut jp = Pt::Inf;
    for j in 1..=m {
        jp = c.add(cf, jp, p);
        if let Pt::A(x, _) = jp {
            baby.entry(x).or_insert(j);
        }
    }
    let step = c.mul(cf, (2 * m + 1) as u64, p);
    let nstep = c.neg(step);
    let base = c.mul(cf, q + 1, p);
    let imax = w / (2 * m + 1) + 2;
    let mut out = vec![];
    // Q_i = (q+1)P - i*(2m+1)P for i in -imax..=imax
    let mut qi = base;
    for _ in 0..imax {
        qi = c.add(cf, qi, step);
    }
    let mut i = -imax;
    while i <= imax {
        let mut es: Vec<i64> = vec![];
        match qi {
            Pt::Inf => es.push(0),
            Pt::A(x, _) => {
                if let Some(&j) = baby.get(&x) {
                    es.push(j);
                    es.push(-j);
                }
            }
        }
        for e in es {
            let t = i * (2 * m + 1) + e;
            if t.abs() <= w {
                let nn = (q as i64 + 1 - t) as u64;
                if c.mul(cf, nn, p) == Pt::Inf && !out.contains(&nn) {
                    out.push(nn);
                }
            }
        }
        qi = c.add(cf, qi, nstep);
        i += 1;
    }
    out
}

pub struct Instance {
    pub curve: Curve,
    pub g: Pt,
    pub attempts: u32,
}

/// A random curve over `F_{2^n}` with `b` in no proper subfield and order
/// `h·ℓ`, `h ∈ {2, 4}`, ℓ prime; generator `G = [h]P` of order ℓ.
pub fn random_instance(n: u32, rng: &mut Sm64) -> Instance {
    let irr = irreducible(n);
    let mut attempts = 0;
    loop {
        attempts += 1;
        let gf = Gf2::new(&irr);
        let b = rng.bits(n);
        let a = rng.bits(1);
        if b == 0 || !generic_b(&gf, b) {
            continue;
        }
        let asv = ArtinSchreier::new(&gf).expect("AS solver");
        let sqrt_b = asv.sqrt(b);
        let mut c = Curve { gf, n, a, b, sqrt_b, asv, order: 0, h: 0, ell: 0, irr_low: irr.low_terms.clone() };
        let gfc = c.gf.clone();
        let cf = CF::new(&gfc);
        // random point
        let rp = |rng: &mut Sm64, c: &Curve, cf: &CF| loop {
            let x = rng.bits(n);
            if x == 0 {
                continue;
            }
            let ix2 = cf.inv(cf.sqr(x));
            if let Some(p) = c.lift_with(cf, x, ix2) {
                return p;
            }
        };
        let p = rp(rng, &c, &cf);
        assert!(c.on_curve(p));
        let cands = orders_killing(&c, &cf, p);
        let mut found = None;
        for &nn in &cands {
            for h in [2u64, 4] {
                if nn % h == 0 && is_prime(nn / h) {
                    let ell = nn / h;
                    if (ell as f64) > 4.0 * ((1u64 << n) as f64).sqrt() && c.mul(&cf, h, p) != Pt::Inf {
                        found = Some((nn, h, ell));
                    }
                }
            }
        }
        let Some((nn, h, ell)) = found else { continue };
        // Cross-check with a second random point.
        let p2 = rp(rng, &c, &cf);
        if c.mul(&cf, nn, p2) != Pt::Inf {
            continue;
        }
        c.order = nn;
        c.h = h;
        c.ell = ell;
        let g = loop {
            let pp = rp(rng, &c, &cf);
            let g = c.mul(&cf, h, pp);
            if g != Pt::Inf {
                break g;
            }
        };
        assert_eq!(c.mul(&cf, ell, g), Pt::Inf);
        return Instance { curve: c, g, attempts };
    }
}

/// F₂-basis of the subfield `F_{2^{n'}}` ⊂ `F_{2^n}`: the kernel of
/// `x ↦ x^{2^{n'}} + x`.
pub fn subfield_basis(gf: &Gf2, np: u32) -> Vec<u64> {
    let n = gf.n;
    assert_eq!(n % np, 0);
    let mut rows: Vec<(u64, u64)> = vec![]; // (reduced image, combination)
    let mut kernel = vec![];
    for i in 0..n {
        let e = 1u64 << i;
        let mut img = gf.sqr_k(e, np) ^ e;
        let mut comb = e;
        for &(ri, rc) in &rows {
            let p = 63 - ri.leading_zeros();
            if (img >> p) & 1 == 1 {
                img ^= ri;
                comb ^= rc;
            }
        }
        if img == 0 {
            kernel.push(comb);
        } else {
            let p = 63 - img.leading_zeros();
            for r in rows.iter_mut() {
                if (r.0 >> p) & 1 == 1 {
                    r.0 ^= img;
                    r.1 ^= comb;
                }
            }
            rows.push((img, comb));
        }
    }
    assert_eq!(kernel.len() as u32, np, "subfield dimension");
    kernel
}
