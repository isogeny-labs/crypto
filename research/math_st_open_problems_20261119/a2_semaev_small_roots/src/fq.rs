//! Univariate polynomials over F_q, q = 2^61 - 1, used for root extraction:
//! resultants, interpolation, gcd and root finding (Cantor-Zassenhaus).
use rand::Rng;

pub const Q: u64 = (1u64 << 61) - 1;

#[inline]
pub fn mq(a: u64, b: u64) -> u64 {
    let t = a as u128 * b as u128;
    let lo = (t as u64) & Q;
    let hi = (t >> 61) as u64;
    let s = lo + hi;
    if s >= Q { s - Q } else { s }
}
#[inline]
pub fn aq(a: u64, b: u64) -> u64 { let s = a + b; if s >= Q { s - Q } else { s } }
#[inline]
pub fn sq(a: u64, b: u64) -> u64 { if a >= b { a - b } else { a + Q - b } }
pub fn pq(mut a: u64, mut e: u64) -> u64 {
    let mut r = 1;
    while e > 0 { if e & 1 == 1 { r = mq(r, a); } a = mq(a, a); e >>= 1; }
    r
}
pub fn iq(a: u64) -> u64 { assert!(a != 0); pq(a, Q - 2) }
pub fn from_i128(x: i128) -> u64 { let r = x.rem_euclid(Q as i128); r as u64 }
pub fn lift(x: u64) -> i64 { if x <= Q / 2 { x as i64 } else { -((Q - x) as i64) } }

pub type P = Vec<u64>; // coefficients, low degree first, trimmed

pub fn trim(a: &mut P) { while let Some(&0) = a.last() { a.pop(); } }
pub fn deg(a: &P) -> isize { a.len() as isize - 1 }

pub fn eval(a: &P, x: u64) -> u64 {
    let mut s = 0;
    for &c in a.iter().rev() { s = aq(mq(s, x), c); }
    s
}
pub fn pmul(a: &P, b: &P) -> P {
    if a.is_empty() || b.is_empty() { return vec![]; }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 { continue; }
        for (j, &y) in b.iter().enumerate() { r[i + j] = aq(r[i + j], mq(x, y)); }
    }
    trim(&mut r);
    r
}
pub fn prem(a: &P, m: &P) -> P {
    let mut r = a.clone();
    trim(&mut r);
    let dm = m.len();
    assert!(dm > 0);
    let inv = iq(*m.last().unwrap());
    while r.len() >= dm {
        let c = mq(*r.last().unwrap(), inv);
        let sh = r.len() - dm;
        for i in 0..dm { r[sh + i] = sq(r[sh + i], mq(c, m[i])); }
        r.pop();
        trim(&mut r);
    }
    r
}
pub fn monic(a: &P) -> P {
    let inv = iq(*a.last().unwrap());
    a.iter().map(|&c| mq(c, inv)).collect()
}
pub fn gcd(a: &P, b: &P) -> P {
    let (mut x, mut y) = (a.clone(), b.clone());
    trim(&mut x); trim(&mut y);
    while !y.is_empty() { let r = prem(&x, &y); x = y; y = r; }
    if x.is_empty() { x } else { monic(&x) }
}
pub fn psub(a: &P, b: &P) -> P {
    let n = a.len().max(b.len());
    let mut r = vec![0; n];
    for i in 0..n { r[i] = sq(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)); }
    trim(&mut r);
    r
}
/// base^e mod m
pub fn powmod(base: &P, mut e: u64, m: &P) -> P {
    let mut r: P = vec![1];
    let mut b = prem(base, m);
    while e > 0 {
        if e & 1 == 1 { r = prem(&pmul(&r, &b), m); }
        b = prem(&pmul(&b, &b), m);
        e >>= 1;
    }
    r
}

/// Resultant with the given formal degrees (actual degree must equal them).
pub fn resultant(a: &P, b: &P) -> u64 {
    let (mut a, mut b) = (a.clone(), b.clone());
    trim(&mut a); trim(&mut b);
    if a.is_empty() || b.is_empty() { return 0; }
    let mut res = 1u64;
    loop {
        let da = a.len() - 1;
        let db = b.len() - 1;
        if db == 0 { return mq(res, pq(b[0], da as u64)); }
        let r = prem(&a, &b);
        if r.is_empty() { return 0; }
        let dr = r.len() - 1;
        // Res(a,b) = (-1)^{da db} lc(b)^{da-dr} Res(b, r)
        if (da * db) % 2 == 1 { res = sq(0, res); }
        res = mq(res, pq(*b.last().unwrap(), (da - dr) as u64));
        a = b; b = r;
    }
}

/// Interpolate a polynomial of degree < xs.len() through (xs[i], ys[i]), xs in
/// arithmetic progression x_i = s + i t.
pub fn interp_ap(s: u64, t: u64, ys: &[u64]) -> P {
    let n = ys.len();
    let xs: Vec<u64> = (0..n).map(|i| aq(s, mq(t, i as u64))).collect();
    let tinv = iq(t);
    let invs: Vec<u64> = (0..n as u64).map(|i| if i == 0 { 0 } else { mq(iq(i), tinv) }).collect();
    // Newton divided differences
    let mut c = ys.to_vec();
    for j in 1..n {
        for i in (j..n).rev() {
            c[i] = mq(sq(c[i], c[i - 1]), invs[j]);
        }
    }
    // Horner to monomial form
    let mut r: P = vec![0; n];
    let mut len = 0usize;
    for i in (0..n).rev() {
        // r = r * (x - xs[i]) + c[i]
        let mut nr = vec![0u64; len + 1];
        for k in 0..len {
            nr[k + 1] = aq(nr[k + 1], r[k]);
            nr[k] = sq(nr[k], mq(r[k], xs[i]));
        }
        nr[0] = aq(nr[0], c[i]);
        len += 1;
        r[..len].copy_from_slice(&nr[..len]);
    }
    r.truncate(len);
    trim(&mut r);
    r
}

/// Distinct roots in F_q.
pub fn roots<R: Rng>(a: &P, rng: &mut R) -> Vec<u64> {
    let mut a = a.clone();
    trim(&mut a);
    if a.len() <= 1 { return vec![]; }
    let mut out = vec![];
    if a[0] == 0 { out.push(0); }
    let a = monic(&a);
    let x: P = vec![0, 1];
    let xq = powmod(&x, Q, &a);
    let g = gcd(&a, &psub(&xq, &x));
    if g.len() <= 1 { out.sort(); out.dedup(); return out; }
    split(&g, rng, &mut out);
    out.sort(); out.dedup();
    out
}
fn split<R: Rng>(g: &P, rng: &mut R, out: &mut Vec<u64>) {
    let dg = g.len() - 1;
    if dg == 0 { return; }
    if dg == 1 { out.push(sq(0, mq(g[0], iq(g[1])))); return; }
    for _ in 0..200 {
        let dlt = rng.gen_range(0..Q);
        let w = powmod(&vec![dlt, 1], (Q - 1) / 2, g);
        let h = gcd(g, &psub(&w, &vec![1]));
        let dh = h.len() as isize - 1;
        if dh > 0 && (dh as usize) < dg {
            split(&h, rng, out);
            let (qq, _) = pdiv(g, &h);
            split(&qq, rng, out);
            return;
        }
    }
}
pub fn pdiv(a: &P, m: &P) -> (P, P) {
    let mut r = a.clone();
    trim(&mut r);
    let dm = m.len();
    if r.len() < dm { return (vec![], r); }
    let mut qv = vec![0u64; r.len() - dm + 1];
    let inv = iq(*m.last().unwrap());
    while r.len() >= dm {
        let c = mq(*r.last().unwrap(), inv);
        let sh = r.len() - dm;
        qv[sh] = c;
        for i in 0..dm { r[sh + i] = sq(r[sh + i], mq(c, m[i])); }
        r.pop();
    }
    trim(&mut r);
    trim(&mut qv);
    (qv, r)
}
