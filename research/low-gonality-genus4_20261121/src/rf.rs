//! Deterministic root finding over F_p for binary forms of small degree, with
//! every field multiplication counted.  Used by the genus-4 (3,3) relation
//! phase: the residual intersection of a plane section with the curve is a
//! binary cubic, and its F_p roots replace the scan of the factor base.
//!
//! Polynomials are `Vec<u64>` low to high.  A binary form of homogeneous degree
//! `deg_h` is stored as its dehomogenisation `f(x)` with `deg f <= deg_h`; the
//! missing degree is the multiplicity of the root at infinity.

use std::cell::Cell;

pub type Poly = Vec<u64>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Root {
    Fin(u64),
    Inf,
}

#[inline]
pub fn mm(a: u64, b: u64, p: u64, ctr: &Cell<u64>) -> u64 {
    ctr.set(ctr.get() + 1);
    a * b % p
}

fn trim(a: &mut Poly) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

pub fn powm(mut a: u64, mut e: u64, p: u64, ctr: &Cell<u64>) -> u64 {
    let mut r = 1u64;
    while e > 0 {
        if e & 1 == 1 {
            r = mm(r, a, p, ctr);
        }
        e >>= 1;
        if e > 0 {
            a = mm(a, a, p, ctr);
        }
    }
    r
}

pub fn invm(a: u64, p: u64, ctr: &Cell<u64>) -> u64 {
    powm(a, p - 2, p, ctr)
}

pub fn pmul(a: &[u64], b: &[u64], p: u64, ctr: &Cell<u64>) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut out = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            out[i + j] = (out[i + j] + mm(x, y, p, ctr)) % p;
        }
    }
    trim(&mut out);
    out
}

/// Quotient and remainder of `a` by `f` (f nonzero, trimmed).
pub fn pdivrem(a: &[u64], f: &[u64], p: u64, ctr: &Cell<u64>) -> (Poly, Poly) {
    let mut r: Poly = a.to_vec();
    trim(&mut r);
    let df = f.len() - 1;
    if r.len() <= df {
        return (vec![], r);
    }
    let il = invm(*f.last().unwrap(), p, ctr);
    let mut q = vec![0u64; r.len() - df];
    while r.len() > df && !r.is_empty() {
        let k = r.len() - 1 - df;
        let c = mm(*r.last().unwrap(), il, p, ctr);
        q[k] = c;
        for (j, &fj) in f.iter().enumerate() {
            let t = mm(c, fj, p, ctr);
            r[k + j] = (r[k + j] + p - t) % p;
        }
        // leading coefficient is now zero
        r.pop();
        trim(&mut r);
    }
    trim(&mut q);
    (q, r)
}

fn pmulmod(a: &[u64], b: &[u64], f: &[u64], p: u64, ctr: &Cell<u64>) -> Poly {
    let prod = pmul(a, b, p, ctr);
    pdivrem(&prod, f, p, ctr).1
}

fn monic(a: &mut Poly, p: u64, ctr: &Cell<u64>) {
    if a.is_empty() {
        return;
    }
    let il = invm(*a.last().unwrap(), p, ctr);
    for x in a.iter_mut() {
        *x = mm(*x, il, p, ctr);
    }
}

pub fn pgcd(a: &[u64], b: &[u64], p: u64, ctr: &Cell<u64>) -> Poly {
    let (mut x, mut y): (Poly, Poly) = (a.to_vec(), b.to_vec());
    trim(&mut x);
    trim(&mut y);
    while !y.is_empty() {
        let r = pdivrem(&x, &y, p, ctr).1;
        x = y;
        y = r;
    }
    monic(&mut x, p, ctr);
    x
}

/// `base^e mod f` by square and multiply.
fn ppow(base: &[u64], e: u64, f: &[u64], p: u64, ctr: &Cell<u64>) -> Poly {
    let mut r: Poly = vec![1];
    let mut b = pdivrem(base, f, p, ctr).1;
    let mut e = e;
    while e > 0 {
        if e & 1 == 1 {
            r = pmulmod(&r, &b, f, p, ctr);
        }
        e >>= 1;
        if e > 0 {
            b = pmulmod(&b, &b, f, p, ctr);
        }
    }
    r
}

/// Split a monic squarefree polynomial that is a product of distinct linear
/// factors over F_p.  Deterministic: gcd with (x+a)^((p-1)/2) - 1 for
/// a = 0, 1, 2, ...
pub fn split_linear(g: &[u64], p: u64, ctr: &Cell<u64>, out: &mut Vec<u64>) {
    let d = g.len() - 1;
    if d == 0 {
        return;
    }
    if d == 1 {
        out.push((p - g[0]) % p);
        return;
    }
    for a in 0..p {
        let t = ppow(&[a, 1], (p - 1) / 2, g, p, ctr);
        let mut t1 = t;
        if t1.is_empty() {
            t1 = vec![p - 1];
        } else {
            t1[0] = (t1[0] + p - 1) % p;
        }
        trim(&mut t1);
        let h = if t1.is_empty() {
            g.to_vec()
        } else {
            pgcd(g, &t1, p, ctr)
        };
        let dh = h.len() - 1;
        if dh > 0 && dh < d {
            let (q, _) = pdivrem(g, &h, p, ctr);
            let mut qm = q;
            monic(&mut qm, p, ctr);
            split_linear(&h, p, ctr, out);
            split_linear(&qm, p, ctr, out);
            return;
        }
    }
    unreachable!("split_linear: no splitting element (not a product of distinct linear factors)");
}

pub struct RootSet {
    /// Distinct F_p-rational roots in P^1 (finite roots, then Inf if present).
    pub roots: Vec<Root>,
    /// All `deg_h` roots are distinct and rational.
    pub complete: bool,
}

/// Distinct rational roots of the binary form with dehomogenisation `f` and
/// homogeneous degree `deg_h`.  `None` for the zero form.
pub fn distinct_roots(f: &[u64], deg_h: usize, p: u64, ctr: &Cell<u64>) -> Option<RootSet> {
    let mut f: Poly = f.to_vec();
    trim(&mut f);
    if f.is_empty() {
        return None;
    }
    let d = f.len() - 1;
    assert!(d <= deg_h);
    let inf = deg_h - d;
    let mut roots: Vec<Root> = vec![];
    let mut gdeg = 0usize;
    if d > 0 {
        // g = gcd(f, x^p - x): the product of the distinct rational linear factors
        let xp = ppow(&[0, 1], p, &f, p, ctr);
        let mut t = xp;
        if t.len() < 2 {
            t.resize(2, 0);
        }
        t[1] = (t[1] + p - 1) % p;
        trim(&mut t);
        let g = if t.is_empty() {
            {
                let mut fm = f.clone();
                monic(&mut fm, p, ctr);
                fm
            }
        } else {
            pgcd(&f, &t, p, ctr)
        };
        gdeg = g.len() - 1;
        let mut r = vec![];
        if gdeg > 0 {
            split_linear(&g, p, ctr, &mut r);
        }
        r.sort_unstable();
        roots.extend(r.into_iter().map(Root::Fin));
    }
    if inf >= 1 {
        roots.push(Root::Inf);
    }
    let complete = gdeg == d && inf <= 1;
    Some(RootSet { roots, complete })
}

/// Divide `f` by (x - x0), requiring zero remainder; returns None if the
/// remainder is nonzero.
pub fn div_linear(f: &[u64], x0: u64, p: u64, ctr: &Cell<u64>) -> Option<Poly> {
    if f.len() < 2 {
        return None;
    }
    let n = f.len() - 1;
    let mut q = vec![0u64; n];
    let mut carry = 0u64;
    for k in (0..=n).rev() {
        let v = (f[k] + mm(carry, x0, p, ctr)) % p;
        if k == 0 {
            if v != 0 {
                return None;
            }
        } else {
            q[k - 1] = v;
            carry = v;
        }
    }
    trim(&mut q);
    Some(q)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lcg(s: &mut u64) -> u64 {
        *s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *s >> 33
    }

    fn eval(f: &[u64], x: u64, p: u64) -> u64 {
        f.iter().rev().fold(0, |a, &c| (a * x + c) % p)
    }

    #[test]
    fn roots_match_brute_force() {
        let ctr = Cell::new(0);
        let mut s = 11u64;
        for &p in &[5u64, 7, 11, 31, 61, 127, 251, 509] {
            for _ in 0..400 {
                let f: Poly = (0..4).map(|_| lcg(&mut s) % p).collect();
                let mut ft = f.clone();
                trim(&mut ft);
                if ft.is_empty() {
                    continue;
                }
                let rs = distinct_roots(&f, 3, p, &ctr).unwrap();
                let mut want: Vec<u64> = (0..p).filter(|&x| eval(&f, x, p) == 0).collect();
                want.sort_unstable();
                let got: Vec<u64> = rs
                    .roots
                    .iter()
                    .filter_map(|r| if let Root::Fin(x) = r { Some(*x) } else { None })
                    .collect();
                assert_eq!(got, want, "p={p} f={f:?}");
                let inf = 3 - ft.len() + 1;
                assert_eq!(rs.roots.contains(&Root::Inf), inf >= 1);
                // multiplicity-free and fully rational iff 3 distinct roots in all
                let total = want.len() + usize::from(inf >= 1);
                let squarefree = inf <= 1 && {
                    // brute-force multiplicity check via derivative gcd
                    want.iter().all(|&x| {
                        let (q, _) = pdivrem(&ft, &[(p - x) % p, 1], p, &ctr);
                        eval(&q, x, p) != 0
                    })
                };
                assert_eq!(rs.complete, total == 3 && squarefree, "p={p} f={f:?}");
            }
        }
    }

    #[test]
    fn division_by_known_roots() {
        let ctr = Cell::new(0);
        let p = 127u64;
        let mut s = 5u64;
        for _ in 0..100 {
            let r1 = lcg(&mut s) % p;
            let r2 = lcg(&mut s) % p;
            let extra: Poly = (0..3).map(|_| lcg(&mut s) % p).collect();
            let f = pmul(
                &pmul(&[(p - r1) % p, 1], &[(p - r2) % p, 1], p, &ctr),
                &extra,
                p,
                &ctr,
            );
            let q = div_linear(&f, r1, p, &ctr).unwrap();
            let q = div_linear(&q, r2, p, &ctr).unwrap();
            let mut e = extra.clone();
            trim(&mut e);
            assert_eq!(q, e);
            let x = (r1 + r2 + 1) % p;
            if eval(&f, x, p) != 0 {
                assert!(div_linear(&f, x, p, &ctr).is_none());
            }
        }
    }

    #[test]
    fn split_three_roots() {
        let ctr = Cell::new(0);
        let p = 2039u64;
        let g = pmul(
            &pmul(&[p - 5, 1], &[p - 700, 1], p, &ctr),
            &[p - 2000, 1],
            p,
            &ctr,
        );
        let rs = distinct_roots(&g, 3, p, &ctr).unwrap();
        assert!(rs.complete);
        assert_eq!(
            rs.roots,
            vec![Root::Fin(5), Root::Fin(700), Root::Fin(2000)]
        );
    }
}
