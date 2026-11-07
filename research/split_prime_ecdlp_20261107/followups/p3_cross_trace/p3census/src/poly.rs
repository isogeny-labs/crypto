//! Dense polynomials over F_p (low-to-high coefficient vectors), p < 2^32.
use crate::ec::inv_p;
use crate::util::powmod;

pub type Poly = Vec<u64>;

pub fn trim(mut a: Poly) -> Poly {
    while a.last() == Some(&0) {
        a.pop();
    }
    a
}
pub fn padd(a: &Poly, b: &Poly, p: u64) -> Poly {
    let n = a.len().max(b.len());
    trim((0..n).map(|i| (a.get(i).copied().unwrap_or(0) + b.get(i).copied().unwrap_or(0)) % p).collect())
}
pub fn psub(a: &Poly, b: &Poly, p: u64) -> Poly {
    let n = a.len().max(b.len());
    trim((0..n).map(|i| (a.get(i).copied().unwrap_or(0) + p - b.get(i).copied().unwrap_or(0)) % p).collect())
}
pub fn pscale(a: &Poly, k: u64, p: u64) -> Poly {
    trim(a.iter().map(|&c| c * (k % p) % p).collect())
}
pub fn pmul(a: &Poly, b: &Poly, p: u64) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = (r[i + j] + x * y) % p;
        }
    }
    trim(r)
}
/// a mod m (m non-zero, not necessarily monic)
pub fn pmod(a: &Poly, m: &Poly, p: u64) -> Poly {
    let mut r = a.clone();
    let dm = m.len() - 1;
    let il = inv_p(*m.last().unwrap(), p);
    while r.len() > dm {
        let lead = *r.last().unwrap();
        if lead != 0 {
            let f = lead * il % p;
            let shift = r.len() - 1 - dm;
            for (i, &c) in m.iter().enumerate() {
                r[shift + i] = (r[shift + i] + p - f * c % p) % p;
            }
        }
        r.pop();
        r = trim(r);
    }
    trim(r)
}
pub fn pgcd(a: &Poly, b: &Poly, p: u64) -> Poly {
    let (mut x, mut y) = (trim(a.clone()), trim(b.clone()));
    while !y.is_empty() {
        let r = pmod(&x, &y, p);
        x = y;
        y = r;
    }
    x
}
/// x^e mod m
pub fn xpow(e: u64, m: &Poly, p: u64) -> Poly {
    let mut result: Poly = vec![1];
    let mut base = pmod(&vec![0, 1], m, p);
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            result = pmod(&pmul(&result, &base, p), m, p);
        }
        base = pmod(&pmul(&base, &base, p), m, p);
        e >>= 1;
    }
    result
}
pub fn peval(a: &Poly, x: u64, p: u64) -> u64 {
    a.iter().rev().fold(0, |acc, &c| (acc * x + c) % p)
}
pub fn pderiv(a: &Poly, p: u64) -> Poly {
    trim(a.iter().enumerate().skip(1).map(|(i, &c)| c * i as u64 % p).collect())
}

/// ψ3 = 3x^4 + 6a x^2 + 12 b x - a^2
pub fn psi3(a: u64, b: u64, p: u64) -> Poly {
    trim(vec![(p - a * a % p) % p, 12 * b % p, 6 * a % p, 0, 3])
}
/// ψ4 / (4y) = x^6 + 5a x^4 + 20 b x^3 - 5a^2 x^2 - 4ab x - 8b^2 - a^3
fn psi4_over_2y(a: u64, b: u64, p: u64) -> Poly {
    let a2 = a * a % p;
    let a3 = a2 * a % p;
    let b2 = b * b % p;
    trim(vec![
        (p - (8 * b2 % p + a3) % p) % p,
        (p - 4 * (a * b % p) % p) % p,
        (p - 5 * a2 % p) % p,
        20 * b % p,
        5 * a % p,
        0,
        1,
    ])
}
/// ψ5 = ψ4 ψ2^3 - ψ3^3 (recurrence psi_{2m+1}, m = 2), with y^2 = f = x^3 + a x + b,
/// ψ2 = 2y, ψ4 = 4y g: ψ4 ψ2^3 = (4y g)(8 y^3) = 32 y^4 g = 32 f^2 g. Leading
/// coefficient 32 - 27 = 5 (selftest checks degree 12 and 5P = O <=> psi5(x) = 0).
pub fn psi5(a: u64, b: u64, p: u64) -> Poly {
    let f: Poly = trim(vec![b % p, a % p, 0, 1]);
    let g = psi4_over_2y(a, b, p);
    let f2g = pmul(&pmul(&f, &f, p), &g, p);
    let t1 = pscale(&f2g, 32, p);
    let s = psi3(a, b, p);
    let s3 = pmul(&pmul(&s, &s, p), &s, p);
    psub(&t1, &s3, p)
}
/// Number of distinct roots in F_p = deg gcd(f, x^p - x).
pub fn roots_in_fp(f: &Poly, p: u64) -> usize {
    let f = trim(f.clone());
    if f.len() <= 1 {
        return 0;
    }
    let xp = xpow(p, &f, p);
    let diff = psub(&xp, &vec![0, 1], p);
    if diff.is_empty() {
        return f.len() - 1; // x^p = x mod f: all roots in F_p (f splits)
    }
    let g = pgcd(&f, &diff, p);
    g.len() - 1
}
#[allow(dead_code)]
pub fn unused(p: u64) -> u64 {
    powmod(2, 3, p)
}
