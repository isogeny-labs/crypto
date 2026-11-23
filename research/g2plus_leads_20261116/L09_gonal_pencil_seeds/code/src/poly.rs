//! Polynomials over counted F_p (SPEC section 4).
use crate::field::Fp;
use crate::rng::{hash_poly, SplitMix64};

pub type Poly = Vec<u64>;

pub fn trim(v: &mut Poly) {
    while let Some(&0) = v.last() {
        v.pop();
    }
}

pub fn padd(fp: &Fp, a: &Poly, b: &Poly) -> Poly {
    let n = a.len().max(b.len());
    let mut r = vec![0u64; n];
    for i in 0..n {
        let x = if i < a.len() { a[i] } else { 0 };
        let y = if i < b.len() { b[i] } else { 0 };
        r[i] = fp.add(x, y);
    }
    trim(&mut r);
    r
}

pub fn pmul(fp: &Fp, a: &Poly, b: &Poly) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for i in 0..a.len() {
        for j in 0..b.len() {
            let m = fp.mul(a[i], b[j]);
            r[i + j] = fp.add(r[i + j], m);
        }
    }
    trim(&mut r);
    r
}

pub fn prem_monic(fp: &Fp, a: &Poly, f: &Poly) -> Poly {
    let d = f.len() - 1;
    let mut a = a.clone();
    if a.len() > d {
        let mut k = a.len() - 1;
        loop {
            if k < d {
                break;
            }
            let t = a[k];
            for i in 0..d {
                let m = fp.mul(t, f[i]);
                a[k - d + i] = fp.sub(a[k - d + i], m);
            }
            a[k] = 0;
            if k == 0 {
                break;
            }
            k -= 1;
        }
    }
    trim(&mut a);
    a
}

pub fn mulmod(fp: &Fp, a: &Poly, b: &Poly, f: &Poly) -> Poly {
    prem_monic(fp, &pmul(fp, a, b), f)
}

pub fn powmod(fp: &Fp, base: &Poly, e: u64, f: &Poly) -> Poly {
    assert!(e >= 1);
    let nbits = 64 - e.leading_zeros();
    let mut h = base.clone();
    for i in (0..nbits - 1).rev() {
        h = mulmod(fp, &h, &h, f);
        if (e >> i) & 1 == 1 {
            h = mulmod(fp, &h, base, f);
        }
    }
    h
}

pub fn make_monic(fp: &Fp, c: &Poly) -> Poly {
    let n = c.len();
    let l = fp.inv(c[n - 1]);
    let mut r = vec![0u64; n];
    for i in 0..n - 1 {
        r[i] = fp.mul(c[i], l);
    }
    r[n - 1] = 1;
    r
}

pub fn split_test(fp: &Fp, f: &Poly) -> bool {
    let base = prem_monic(fp, &vec![0, 1], f);
    let h = powmod(fp, &base, fp.p, f);
    h == base
}

pub fn gcd_monic(fp: &Fp, a: &Poly, b: &Poly) -> Poly {
    let mut a = a.clone();
    let mut b = b.clone();
    while !b.is_empty() {
        b = make_monic(fp, &b);
        let r = prem_monic(fp, &a, &b);
        a = b;
        b = r;
    }
    make_monic(fp, &a)
}

pub fn divexact_monic(fp: &Fp, a: &Poly, g: &Poly) -> Poly {
    let d = g.len() - 1;
    let mut a = a.clone();
    let mut q = vec![0u64; a.len() - d];
    let mut k = a.len() - 1;
    loop {
        if k < d {
            break;
        }
        let t = a[k];
        q[k - d] = t;
        for i in 0..d {
            let m = fp.mul(t, g[i]);
            a[k - d + i] = fp.sub(a[k - d + i], m);
        }
        a[k] = 0;
        if k == 0 {
            break;
        }
        k -= 1;
    }
    for i in 0..d {
        assert!(a[i] == 0, "divexact_monic: non-zero remainder");
    }
    trim(&mut q);
    q
}

pub fn roots_split(fp: &Fp, f: &Poly) -> Vec<u64> {
    let deg = f.len() - 1;
    if deg == 1 {
        return vec![fp.neg(f[0])];
    }
    assert!(deg == 2 || deg == 3);
    let mut rng = SplitMix64::new(hash_poly(f));
    loop {
        let d = rng.below(fp.p);
        let base = prem_monic(fp, &vec![d, 1], f);
        let mut h = powmod(fp, &base, (fp.p - 1) / 2, f);
        if h.is_empty() {
            h.push(0);
        }
        h[0] = fp.sub(h[0], 1);
        trim(&mut h);
        if h.is_empty() {
            continue;
        }
        let g = gcd_monic(fp, f, &h);
        let dg = g.len() - 1;
        if dg >= 1 && dg < deg {
            let mut r = roots_split(fp, &g);
            r.extend(roots_split(fp, &divexact_monic(fp, f, &g)));
            r.sort();
            return r;
        }
    }
}

pub fn roots_all(fp: &Fp, c: &Poly) -> Vec<u64> {
    let mut w = powmod(fp, &vec![0, 1], fp.p, c);
    while w.len() < 2 {
        w.push(0);
    }
    w[1] = fp.sub(w[1], 1);
    trim(&mut w);
    if w.is_empty() {
        return roots_split(fp, c);
    }
    let g = gcd_monic(fp, c, &w);
    if g.len() == 1 {
        return vec![];
    }
    roots_split(fp, &g)
}

/// Divide by (x - r): returns (quotient, remainder). Costs len(a) multiplications.
pub fn synth_div(fp: &Fp, a: &Poly, r: u64) -> (Poly, u64) {
    let n = a.len() - 1;
    let mut q = vec![0u64; n];
    let mut acc = 0u64;
    for k in (0..=n).rev() {
        acc = fp.add(fp.mul(acc, r), a[k]);
        if k > 0 {
            q[k - 1] = acc;
        }
    }
    (q, acc)
}
