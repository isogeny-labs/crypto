//! Exact structure facts for curves over GF(2^31 - 1). Pure integer arithmetic
//! (u64/u128), no CAS. Each fact has two independent methods where one exists.

use crate::{kdf, mulmod, powmod, splitmix, P};

pub type Pt = Option<(u64, u64)>;

#[inline]
fn fm(a: u64, b: u64) -> u64 {
    ((a as u128 * b as u128) % P as u128) as u64
}
#[inline]
fn fa(a: u64, b: u64) -> u64 {
    (a + b) % P
}
#[inline]
fn fs(a: u64, b: u64) -> u64 {
    (a + P - b % P) % P
}
fn finv(a: u64) -> u64 {
    assert!(a % P != 0);
    powmod(a, P - 2, P)
}

/// Affine group law (method 1; independent of the counted engine in lib.rs).
pub fn add(p1: Pt, p2: Pt, a: u64) -> Pt {
    let (u, v) = match (p1, p2) {
        (None, q) => return q,
        (q, None) => return q,
        (Some(u), Some(v)) => (u, v),
    };
    let l = if u.0 == v.0 {
        if fa(u.1, v.1) == 0 {
            return None;
        }
        fm(fa(fm(3, fm(u.0, u.0)), a), finv(fm(2, u.1)))
    } else {
        fm(fs(v.1, u.1), finv(fs(v.0, u.0)))
    };
    let x = fs(fs(fm(l, l), u.0), v.0);
    Some((x, fs(fm(l, fs(u.0, x)), u.1)))
}
pub fn mul(mut k: u64, mut pt: Pt, a: u64) -> Pt {
    let mut r = None;
    while k > 0 {
        if k & 1 == 1 {
            r = add(r, pt, a);
        }
        pt = add(pt, pt, a);
        k >>= 1;
    }
    r
}

/// Jacobian-coordinate scalar multiplication, left-to-right (method 2 verifier;
/// shares no group-law code with `add` or with lib.rs).
pub fn mul_jacobian(k: u64, pt: Pt, a: u64) -> Pt {
    let (gx, gy) = match pt {
        None => return None,
        Some(v) => v,
    };
    // (X, Y, Z), Z = 0 is infinity
    let dbl = |(x, y, z): (u64, u64, u64)| -> (u64, u64, u64) {
        if z == 0 || y == 0 {
            return (1, 1, 0);
        }
        let yy = fm(y, y);
        let s = fm(4, fm(x, yy));
        let z2 = fm(z, z);
        let m = fa(fm(3, fm(x, x)), fm(a, fm(z2, z2)));
        let x3 = fs(fm(m, m), fm(2, s));
        let y3 = fs(fm(m, fs(s, x3)), fm(8, fm(yy, yy)));
        (x3, y3, fm(2, fm(y, z)))
    };
    let addm = |pj: (u64, u64, u64)| -> (u64, u64, u64) {
        let (x1, y1, z1) = pj;
        if z1 == 0 {
            return (gx, gy, 1);
        }
        let z1z1 = fm(z1, z1);
        let u2 = fm(gx, z1z1);
        let s2 = fm(gy, fm(z1, z1z1));
        let h = fs(u2, x1);
        let r = fs(s2, y1);
        if h == 0 {
            return if r == 0 { dbl(pj) } else { (1, 1, 0) };
        }
        let hh = fm(h, h);
        let hhh = fm(h, hh);
        let v = fm(x1, hh);
        let x3 = fs(fs(fm(r, r), hhh), fm(2, v));
        let y3 = fs(fm(r, fs(v, x3)), fm(y1, hhh));
        (x3, y3, fm(z1, h))
    };
    let mut acc = (1u64, 1u64, 0u64);
    for i in (0..64).rev() {
        acc = dbl(acc);
        if (k >> i) & 1 == 1 {
            acc = addm(acc);
        }
    }
    let (x, y, z) = acc;
    if z == 0 {
        return None;
    }
    let zi = finv(z);
    let zi2 = fm(zi, zi);
    Some((fm(x, zi2), fm(y, fm(zi, zi2))))
}

pub fn is_prime(m: u64) -> bool {
    if m < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if m % q == 0 {
            return m == q;
        }
    }
    let (mut d, mut s) = (m - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    'w: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, m);
        if x == 1 || x == m - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulmod(x, x, m);
            if x == m - 1 {
                continue 'w;
            }
        }
        return false;
    }
    true
}

pub fn factor(mut m: u64) -> Vec<(u64, u32)> {
    let mut f = vec![];
    let mut d = 2u64;
    while d * d <= m {
        let mut e = 0;
        while m % d == 0 {
            m /= d;
            e += 1;
        }
        if e > 0 {
            f.push((d, e));
        }
        d += if d == 2 { 1 } else { 2 };
    }
    if m > 1 {
        f.push((m, 1));
    }
    f
}
pub fn fac_json(f: &[(u64, u32)]) -> String {
    let v: Vec<String> = f.iter().map(|(q, e)| format!("\"{}\":{}", q, e)).collect();
    format!("{{{}}}", v.join(","))
}

pub fn legendre(x: u64) -> i32 {
    let x = x % P;
    if x == 0 {
        0
    } else if powmod(x, (P - 1) / 2, P) == 1 {
        1
    } else {
        -1
    }
}
fn sqrt_p(x: u64) -> Option<u64> {
    let r = powmod(x % P, (P + 1) / 4, P); // P = 3 mod 4
    if fm(r, r) == x % P {
        Some(r)
    } else {
        None
    }
}
fn rhs(a: u64, b: u64, x: u64) -> u64 {
    fa(fa(fm(fm(x, x), x), fm(a, x)), b)
}

pub struct Rng(u64);
impl Rng {
    pub fn new(stream: &str) -> Rng {
        Rng(kdf::key(MASTER, &[stream]))
    }
    pub fn next(&mut self) -> u64 {
        splitmix(&mut self.0)
    }
}
pub const MASTER: &str = "4d33315f73747275"; // "M31_stru"

pub fn rand_point(a: u64, b: u64, rng: &mut Rng) -> (u64, u64) {
    loop {
        let x = rng.next() % P;
        let f = rhs(a, b, x);
        if f == 0 {
            continue;
        }
        if let Some(y) = sqrt_p(f) {
            return (x, y);
        }
    }
}

fn isqrt(n: u64) -> u64 {
    let mut r = (n as f64).sqrt() as u64;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

/// All m in the (widened) Hasse window with mP = O; never uses the claimed order.
pub fn bsgs_hasse(pt: (u64, u64), a: u64) -> Vec<u64> {
    let s = isqrt(P);
    let lo = P + 1 - 2 * s - 2;
    let hi = P + 1 + 2 * s + 2;
    let m = isqrt(hi - lo) + 1;
    let mut baby = std::collections::HashMap::new();
    let mut r: Pt = None;
    for j in 0..m {
        baby.entry(r).or_insert(j);
        r = add(r, Some(pt), a);
    }
    let mp = mul(m, Some(pt), a);
    let neg = |q: Pt| q.map(|(x, y)| (x, fs(0, y)));
    let negmp = neg(mp);
    let mut sgiant = neg(mul(lo, Some(pt), a));
    let mut hits = vec![];
    for i in 0..=m {
        if let Some(&j) = baby.get(&sgiant) {
            let c = lo + i * m + j;
            if c <= hi {
                hits.push(c);
            }
        }
        sgiant = add(sgiant, negmp, a);
    }
    hits.sort();
    hits.dedup();
    hits
}

// ---- polynomials over F_p, coefficient vectors low -> high ----
fn trim(f: &mut Vec<u64>) {
    while f.last() == Some(&0) {
        f.pop();
    }
}
fn pmod(f: &[u64], g: &[u64]) -> Vec<u64> {
    let mut f = f.to_vec();
    trim(&mut f);
    let inv = finv(*g.last().unwrap());
    while f.len() >= g.len() {
        let c = fm(*f.last().unwrap(), inv);
        let sh = f.len() - g.len();
        for (i, &gi) in g.iter().enumerate() {
            f[sh + i] = fs(f[sh + i], fm(c, gi));
        }
        trim(&mut f);
    }
    f
}
fn pmulmod(f: &[u64], g: &[u64], m: &[u64]) -> Vec<u64> {
    if f.is_empty() || g.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; f.len() + g.len() - 1];
    for (i, &x) in f.iter().enumerate() {
        for (j, &y) in g.iter().enumerate() {
            r[i + j] = fa(r[i + j], fm(x, y));
        }
    }
    pmod(&r, m)
}
fn pgcd(f: &[u64], g: &[u64]) -> Vec<u64> {
    let (mut f, mut g) = (f.to_vec(), g.to_vec());
    trim(&mut f);
    trim(&mut g);
    while !g.is_empty() {
        let r = pmod(&f, &g);
        f = g;
        g = r;
    }
    f
}
/// Number of distinct F_p-roots of f = deg gcd(f, x^p - x).
pub fn n_roots(f: &[u64]) -> usize {
    let mut r = vec![1u64];
    let mut base = pmod(&[0, 1], f);
    let mut e = P;
    while e > 0 {
        if e & 1 == 1 {
            r = pmulmod(&r, &base, f);
        }
        base = pmulmod(&base, &base, f);
        e >>= 1;
    }
    r.resize(r.len().max(2), 0);
    r[1] = fs(r[1], 1);
    trim(&mut r);
    if r.is_empty() {
        return f.len() - 1;
    }
    pgcd(f, &r).len() - 1
}
/// Method 2 for root counts: direct search is impossible at 2^31, so use
/// distinct roots of f by brute force over a *check*: count of x in F_p with f(x)=0
/// is instead recomputed via gcd with x^p - x evaluated by a different exponent
/// path (x^(p-1) - 1 on f / gcd(f, x)), i.e. Fermat instead of x^p - x.
pub fn n_roots_fermat(f: &[u64]) -> usize {
    let zero_root = if f[0] == 0 { 1 } else { 0 };
    // remove factor x
    let mut g = f.to_vec();
    while g[0] == 0 {
        g.remove(0);
    }
    if g.len() == 1 {
        return zero_root;
    }
    let mut r = vec![1u64];
    let mut base = pmod(&[0, 1], &g);
    let mut e = P - 1;
    while e > 0 {
        if e & 1 == 1 {
            r = pmulmod(&r, &base, &g);
        }
        base = pmulmod(&base, &base, &g);
        e >>= 1;
    }
    r.resize(r.len().max(1), 0);
    r[0] = fs(r[0], 1);
    trim(&mut r);
    let d = if r.is_empty() { g.len() - 1 } else { pgcd(&g, &r).len() - 1 };
    d + zero_root
}

pub fn mult_order(x: u64, n: u64) -> u64 {
    let mut k = n - 1;
    for (q, e) in factor(n - 1) {
        for _ in 0..e {
            if powmod(x, k / q, n) == 1 {
                k /= q;
            }
        }
    }
    k
}

pub fn twist(a: u64, b: u64) -> (u64, u64) {
    let c = (2..100).find(|&c| legendre(c) == -1).unwrap();
    (fm(a, fm(c, c)), fm(b, fm(c, fm(c, c))))
}

/// |E(F_p)[3]| from random points (cofactor-cleared), as a group-theoretic second method.
pub fn three_torsion_size(a: u64, b: u64, order: u64, rng: &mut Rng) -> usize {
    let mut m = order;
    while m % 3 == 0 {
        m /= 3;
    }
    let mut pts = std::collections::HashSet::new();
    for _ in 0..40 {
        let mut r = mul(m, Some(rand_point(a, b, rng)), a);
        while r.is_some() && mul(3, r, a).is_some() {
            r = mul(3, r, a);
        }
        if let Some(v) = r {
            pts.insert(v);
            pts.insert((v.0, fs(0, v.1)));
        }
    }
    // close under addition (subgroup generated)
    let mut grp: std::collections::HashSet<Pt> = pts.iter().map(|&v| Some(v)).collect();
    grp.insert(None);
    loop {
        let v: Vec<Pt> = grp.iter().cloned().collect();
        let mut grew = false;
        for &x in &v {
            for &y in &v {
                if grp.insert(add(x, y, a)) {
                    grew = true;
                }
            }
        }
        if !grew {
            break;
        }
    }
    grp.len()
}
