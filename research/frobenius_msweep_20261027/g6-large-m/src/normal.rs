//! Normal bases (Frobenius = cyclic coordinate shift), the type-II optimal normal basis, random bases, spans.

use crate::field::Fq;
use crate::rng::Rng;

pub fn conjugates(f: &Fq, beta: u32) -> Vec<u32> {
    let mut v = Vec::with_capacity(f.n as usize);
    let mut z = beta;
    for _ in 0..f.n {
        v.push(z);
        z = f.sq(z);
    }
    v
}

/// Rank over F_2 of a list of field elements viewed as bit vectors.
pub fn rank_f2(vs: &[u32]) -> usize {
    let mut basis = [0u32; 32];
    let mut r = 0;
    for &v0 in vs {
        let mut v = v0;
        for b in (0..32).rev() {
            if (v >> b) & 1 == 0 {
                continue;
            }
            if basis[b] == 0 {
                basis[b] = v;
                r += 1;
                break;
            }
            v ^= basis[b];
        }
    }
    r
}

pub fn is_normal(f: &Fq, beta: u32) -> bool {
    beta != 0 && rank_f2(&conjugates(f, beta)) == f.n as usize
}

/// Conjugates of a uniformly random normal element (retry until normal).
pub fn random_normal(f: &Fq, rng: &mut Rng) -> Vec<u32> {
    loop {
        let beta = 1 + rng.below(f.q as u64 - 1) as u32;
        if is_normal(f, beta) {
            return conjugates(f, beta);
        }
    }
}

/// n random F_2-independent field elements (a generic basis with no Frobenius structure).
pub fn random_basis(f: &Fq, rng: &mut Rng) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    while v.len() < f.n as usize {
        let x = rng.below(f.q as u64) as u32;
        let mut w = v.clone();
        w.push(x);
        if rank_f2(&w) == w.len() {
            v.push(x);
        }
    }
    v
}

fn is_prime(p: u64) -> bool {
    if p < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= p {
        if p % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

fn pow_mod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = r * b % m;
        }
        b = b * b % m;
        e >>= 1;
    }
    r
}

/// Type-II optimal normal basis {gamma^(2^i) + gamma^-(2^i)} when p = 2n+1 is prime and ord_p(2) = n, so that a
/// primitive p-th root of unity gamma lies in F_{2^n} itself (n = 3, 11, 23, ...). Returns the n conjugates in
/// Frobenius order, or None when this construction does not apply or the result is not a normal basis.
pub fn type2_onb(f: &Fq) -> Option<Vec<u32>> {
    let n = f.n as u64;
    let p = 2 * n + 1;
    if !is_prime(p) || pow_mod(2, n, p) != 1 {
        return None;
    }
    let e = (f.q1 as u64) / p;
    assert_eq!(e * p, f.q1 as u64);
    let mut z = 2u32;
    let gamma = loop {
        let w = f.pow(z, e);
        if w != 1 {
            break w;
        }
        z += 1;
    };
    let beta = gamma ^ f.inv(gamma);
    if !is_normal(f, beta) {
        return None;
    }
    Some(conjugates(f, beta))
}

/// All 2^k F_2-combinations of the vectors (a subspace as a list of x-codes, 0 first).
pub fn span(vs: &[u32]) -> Vec<u32> {
    let mut out = vec![0u32];
    for &v in vs {
        let len = out.len();
        for i in 0..len {
            out.push(out[i] ^ v);
        }
    }
    out
}
