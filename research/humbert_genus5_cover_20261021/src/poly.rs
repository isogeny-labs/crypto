//! Dense univariate polynomials over F_p (coefficients low to high).
use crate::field::{addm, invm, mulm, subm};

pub fn trim(a: &mut Vec<u64>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}
pub fn deg(a: &[u64]) -> isize {
    let mut d = a.len() as isize - 1;
    while d >= 0 && a[d as usize] == 0 {
        d -= 1;
    }
    d
}
pub fn mul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut c = vec![0u64; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        if ai == 0 {
            continue;
        }
        for (j, &bj) in b.iter().enumerate() {
            c[i + j] = addm(c[i + j], mulm(ai, bj, p), p);
        }
    }
    trim(&mut c);
    c
}
pub fn sub(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let n = a.len().max(b.len());
    let mut c = vec![0u64; n];
    for i in 0..n {
        let x = if i < a.len() { a[i] } else { 0 };
        let y = if i < b.len() { b[i] } else { 0 };
        c[i] = subm(x, y, p);
    }
    trim(&mut c);
    c
}
/// (quotient, remainder)
pub fn divrem(a: &[u64], b: &[u64], p: u64) -> (Vec<u64>, Vec<u64>) {
    let db = deg(b);
    assert!(db >= 0, "division by zero polynomial");
    let db = db as usize;
    let mut r: Vec<u64> = a.to_vec();
    trim(&mut r);
    if r.len() < db + 1 {
        return (vec![], r);
    }
    let lc_inv = invm(b[db], p);
    let mut q = vec![0u64; r.len() - db];
    for i in (0..q.len()).rev() {
        let c = mulm(r[i + db], lc_inv, p);
        q[i] = c;
        if c == 0 {
            continue;
        }
        for j in 0..=db {
            r[i + j] = subm(r[i + j], mulm(c, b[j], p), p);
        }
    }
    trim(&mut r);
    trim(&mut q);
    (q, r)
}
pub fn rem(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    divrem(a, b, p).1
}
pub fn monic(a: &[u64], p: u64) -> Vec<u64> {
    let mut a = a.to_vec();
    trim(&mut a);
    if a.is_empty() {
        return a;
    }
    let li = invm(*a.last().unwrap(), p);
    for c in a.iter_mut() {
        *c = mulm(*c, li, p);
    }
    a
}
pub fn gcd(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let mut x = a.to_vec();
    let mut y = b.to_vec();
    trim(&mut x);
    trim(&mut y);
    while !y.is_empty() {
        let r = rem(&x, &y, p);
        x = y;
        y = r;
    }
    monic(&x, p)
}
pub fn mulmod(a: &[u64], b: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    rem(&mul(a, b, p), m, p)
}
pub fn powmod(base: &[u64], mut e: u128, m: &[u64], p: u64) -> Vec<u64> {
    let mut r = vec![1u64];
    let mut b = rem(base, m, p);
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(&r, &b, m, p);
        }
        b = mulmod(&b, &b, m, p);
        e >>= 1;
    }
    r
}
pub fn eval(a: &[u64], x: u64, p: u64) -> u64 {
    let mut r = 0;
    for &c in a.iter().rev() {
        r = addm(mulm(r, x, p), c, p);
    }
    r
}
/// True iff a (degree >= 1) is a product of distinct linear factors over F_p.
pub fn splits_distinct(a: &[u64], p: u64) -> bool {
    if deg(a) < 1 {
        return deg(a) == 0;
    }
    let xp = powmod(&[0, 1], p as u128, a, p);
    let d = sub(&xp, &[0, 1], p);
    d.is_empty()
}
/// All roots in F_p of a split squarefree polynomial (brute force is fine at toy p
/// and is only reached on candidates that already passed `splits_distinct`).
pub fn roots(a: &[u64], p: u64) -> Vec<u64> {
    let d = deg(a) as usize;
    let mut out = Vec::with_capacity(d);
    let mut f = a.to_vec();
    for x in 0..p {
        if eval(&f, x, p) == 0 {
            out.push(x);
            f = divrem(&f, &[(p - x) % p, 1], p).0;
            if out.len() == d {
                break;
            }
        }
    }
    out
}
/// Lagrange basis for interpolation at nodes 0..=n.
pub struct Interp {
    pub n: usize,
    pub basis: Vec<Vec<u64>>,
}
impl Interp {
    pub fn new(n: usize, p: u64) -> Interp {
        assert!((n as u64) < p, "need p > interpolation degree");
        let mut basis = Vec::with_capacity(n + 1);
        for t in 0..=n {
            let mut num = vec![1u64];
            let mut den = 1u64;
            for s in 0..=n {
                if s == t {
                    continue;
                }
                num = mul(&num, &[(p - s as u64 % p) % p, 1], p);
                den = mulm(den, subm(t as u64, s as u64, p), p);
            }
            let di = invm(den, p);
            for c in num.iter_mut() {
                *c = mulm(*c, di, p);
            }
            basis.push(num);
        }
        Interp { n, basis }
    }
    pub fn interpolate(&self, vals: &[u64], p: u64) -> Vec<u64> {
        let mut r = vec![0u64; self.n + 1];
        for (t, &v) in vals.iter().enumerate() {
            if v == 0 {
                continue;
            }
            for (i, &c) in self.basis[t].iter().enumerate() {
                r[i] = addm(r[i], mulm(v, c, p), p);
            }
        }
        trim(&mut r);
        r
    }
}
/// Rabin test for an irreducible monic quartic over F_p.
pub fn is_irreducible_quartic(m: &[u64], p: u64) -> bool {
    let x = vec![0u64, 1];
    let xp = powmod(&x, p as u128, m, p);
    let xp2 = powmod(&xp, p as u128, m, p);
    let xp4 = powmod(&powmod(&xp2, p as u128, m, p), p as u128, m, p);
    if sub(&xp4, &x, p) != Vec::<u64>::new() {
        return false;
    }
    gcd(&sub(&xp2, &x, p), m, p) == vec![1]
}
/// Rabin test for irreducible monic of prime degree k.
pub fn is_irreducible_prime_deg(m: &[u64], k: usize, p: u64) -> bool {
    let x = vec![0u64, 1];
    let mut xpk = x.clone();
    for _ in 0..k {
        xpk = powmod(&xpk, p as u128, m, p);
    }
    if !sub(&xpk, &x, p).is_empty() {
        return false;
    }
    let xp = powmod(&x, p as u128, m, p);
    gcd(&sub(&xp, &x, p), m, p) == vec![1]
}
