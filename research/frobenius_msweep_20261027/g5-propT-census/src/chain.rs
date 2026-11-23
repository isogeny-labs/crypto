//! Chained S3 (Semaev) system, exactly as the repository's chained-S3 encodings build it, with every
//! auxiliary partial-sum coordinate a free element of F_q:
//!   S3(x1,x2,u2) = 0, S3(u2,x3,u3) = 0, ..., S3(u_{m-1}, x_m, x_R) = 0     (m - 1 equations, m - 2 auxiliaries)
//! S3(x1,x2,x3) = (x1x2+x1x3+x2x3)^2 + x1x2x3 + b for y^2+xy = x^3 + a x^2 + b (independent of a).
//! All coefficients lie in F_2 iff b in F_2: this is the whole content of Proposition T.

use crate::field::Fq;

#[inline]
pub fn s3(f: &Fq, b: u32, x1: u32, x2: u32, x3: u32) -> u32 {
    let p = f.mul(x1, x2) ^ f.mul(x1, x3) ^ f.mul(x2, x3);
    f.sq(p) ^ f.mul(f.mul(x1, x2), x3) ^ b
}

/// Residual vector (m-1 field elements) of the chained system at (xs, aux).
pub fn residuals(f: &Fq, b: u32, xr: u32, xs: &[u32], aux: &[u32]) -> Vec<u32> {
    let m = xs.len();
    assert!(m >= 2 && aux.len() == m - 2);
    let mut out = Vec::with_capacity(m - 1);
    let mut prev = xs[0];
    for i in 1..m {
        let next = if i + 1 == m { xr } else { aux[i - 1] };
        out.push(s3(f, b, prev, xs[i], next));
        prev = next;
    }
    out
}

#[derive(Debug, PartialEq, Eq)]
pub enum Roots {
    None,
    One(u32),
    Two(u32, u32),
    All,
}

/// All x3 in F_q with S3(x1,x2,x3) = 0: s^2 x3^2 + p x3 + (p^2 + b) = 0, s = x1+x2, p = x1 x2.
pub fn roots(f: &Fq, b: u32, x1: u32, x2: u32) -> Roots {
    let s = x1 ^ x2;
    let p = f.mul(x1, x2);
    let qa = f.sq(s);
    let qb = p;
    let qc = f.sq(p) ^ b;
    if qa != 0 {
        if qb == 0 {
            return Roots::One(f.mul(f.sqrt(qc), f.inv(s)));
        }
        let c = f.mul(f.mul(qa, qc), f.inv(f.sq(qb)));
        if f.tr(c) == 1 {
            return Roots::None;
        }
        let z = f.ht(c);
        let sc = f.mul(qb, f.inv(qa));
        let (r1, r2) = (f.mul(sc, z), f.mul(sc, z ^ 1));
        return Roots::Two(r1, r2);
    }
    if qb != 0 {
        return Roots::One(f.mul(qc, f.inv(qb)));
    }
    if qc == 0 {
        Roots::All
    } else {
        Roots::None
    }
}

/// Does some choice of auxiliaries in F_q satisfy the whole chain? Requires b != 0 (then `All` cannot occur).
pub fn member(f: &Fq, b: u32, xr: u32, xs: &[u32]) -> bool {
    assert!(b != 0);
    let m = xs.len();
    assert!(m >= 2);
    let mut front: Vec<u32> = vec![xs[0]];
    for &x in &xs[1..m - 1] {
        let mut next: Vec<u32> = Vec::with_capacity(front.len() * 2);
        for &u in &front {
            match roots(f, b, u, x) {
                Roots::None => {}
                Roots::One(v) => next.push(v),
                Roots::Two(v, w) => {
                    next.push(v);
                    next.push(w);
                }
                Roots::All => unreachable!(),
            }
        }
        next.sort_unstable();
        next.dedup();
        front = next;
        if front.is_empty() {
            return false;
        }
    }
    front.iter().any(|&u| s3(f, b, u, xs[m - 1], xr) == 0)
}

/// Plant a chain solution: choose the auxiliaries along the chain (depth-first with random root order and
/// backtracking) and return (x_R, aux) with the chain satisfied, or None if no branch survives. Any b != 0.
pub fn plant(f: &Fq, b: u32, xs: &[u32], rng: &mut crate::rng::Rng) -> Option<(u32, Vec<u32>)> {
    fn rec(
        f: &Fq,
        b: u32,
        xs: &[u32],
        i: usize,
        prev: u32,
        aux: &mut Vec<u32>,
        rng: &mut crate::rng::Rng,
    ) -> Option<u32> {
        let m = xs.len();
        let cands: Vec<u32> = match roots(f, b, prev, xs[i]) {
            Roots::None | Roots::All => return None,
            Roots::One(v) => vec![v],
            Roots::Two(v, w) => {
                if rng.below(2) == 0 {
                    vec![v, w]
                } else {
                    vec![w, v]
                }
            }
        };
        for v in cands {
            if i + 1 == m {
                return Some(v);
            }
            aux.push(v);
            if let Some(x) = rec(f, b, xs, i + 1, v, aux, rng) {
                return Some(x);
            }
            aux.pop();
        }
        None
    }
    let mut aux = Vec::with_capacity(xs.len().saturating_sub(2));
    rec(f, b, xs, 1, xs[0], &mut aux, rng).map(|x| (x, aux))
}
