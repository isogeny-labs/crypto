//! Exact algebraic prediction for a Frobenius CHAIN factor base (PROTOCOL.md section 4.4, claim S6).
//!
//! If the slots are S_k = {T, +-sigma^k P} for k < L (every other slot is {T}), then since sigma acts on the group as the Koblitz
//! endomorphism tau with tau^2 - t tau + 2 = 0 (t = -1 for a = 0, +1 for a = 1), every sum is
//! R = (sum_k e_k tau^k) P + (T-legs), e_k in {-1, 0, 1} (e_k = 0 is the T leg). The parity of the number of T legs is a function of
//! the Z[tau] value (checked against the group law in tests/digits.rs), so the number of distinct R is the number of distinct values
//! of D = sum e_k tau^k in Z[tau], and the multiplicity of R is the multiplicity of D, provided no nonzero difference of two digit
//! vectors annihilates P. A nonzero delta in the annihilator has norm >= |Z[tau] P| >= ord(P), and norm(delta) <= (2 sum_k 2^(k/2))^2,
//! so the model is exact whenever ord(P) exceeds that bound (`model_applies`).
//!
//! Consequences, derived not measured: p = distinct / E, mu = 3^L / E, disp = (sum_R H^2) / 3^L - mu. Colliding digit strings such as
//! tau^2 + tau + 2 = 0 (a = 0) make `distinct` far smaller than 3^L for short chains of size-3 slots.

use crate::curve::{Curve, Pt};
use std::collections::HashMap;

/// A detected chain: L size-3 slots that are consecutive Frobenius images of each other, all remaining slots equal to {T}.
#[derive(Clone, Copy, Debug)]
pub struct Chain {
    pub l: usize,
    pub p0: Pt,
}

/// Exact statistics predicted by the digit model.
#[derive(Clone, Copy, Debug)]
pub struct Prediction {
    pub l: usize,
    pub tuples: f64,
    pub distinct: u64,
    pub p: f64,
    pub mu: f64,
    pub disp: f64,
    /// sum_R H(R)^2 / sum_R H(R): the size-biased multiplicity (equals disp + mu)
    pub size_biased: f64,
}

/// Value multiplicities of sum_{k<l} e_k tau^k over all 3^l digit strings (dynamic programming over the distinct values).
/// Returns (distinct values, sum of squared multiplicities).
pub fn digit_counts(l: usize, a: u32) -> (u64, f64) {
    let t: i64 = if a == 0 { -1 } else { 1 };
    let mut cur: HashMap<(i64, i64), u64> = HashMap::new();
    cur.insert((0, 0), 1);
    let mut pw = (1i64, 0i64); // tau^k = pw.0 + pw.1 tau
    for _ in 0..l {
        let mut next: HashMap<(i64, i64), u64> = HashMap::with_capacity(cur.len() * 3);
        for (&(u, v), &c) in cur.iter() {
            for d in [-1i64, 0, 1] {
                *next.entry((u + d * pw.0, v + d * pw.1)).or_insert(0) += c;
            }
        }
        cur = next;
        // tau^(k+1) = tau (a + b tau) = a tau + b (t tau - 2) = -2b + (a + t b) tau
        pw = (-2 * pw.1, pw.0 + t * pw.1);
    }
    let s2: f64 = cur.values().map(|&c| (c as f64) * (c as f64)).sum();
    (cur.len() as u64, s2)
}

/// The prediction for a chain of `l` size-3 slots on a curve with parameter `a` and `ne` points.
pub fn predict(l: usize, a: u32, ne: u32) -> Prediction {
    let (distinct, s2) = digit_counts(l, a);
    let tuples = 3f64.powi(l as i32);
    let e = ne as f64;
    let mu = tuples / e;
    Prediction {
        l,
        tuples,
        distinct,
        p: distinct as f64 / e,
        mu,
        disp: s2 / tuples - mu,
        size_biased: s2 / tuples,
    }
}

/// Upper bound on the norm of a difference of two digit vectors (coefficients in -2..2).
pub fn norm_bound(l: usize) -> f64 {
    let s: f64 = (0..l).map(|k| 2f64.powf(k as f64 / 2.0)).sum();
    (2.0 * s) * (2.0 * s)
}

/// Order of P in a group of known exponent `ne` (trial factoring of ne; ne < 2^26 here).
pub fn point_order(c: &Curve, p: Pt, ne: u32) -> u64 {
    let mut k = ne as u64;
    let mut primes = vec![];
    let mut x = ne as u64;
    let mut d = 2u64;
    while d * d <= x {
        if x % d == 0 {
            primes.push(d);
            while x % d == 0 {
                x /= d;
            }
        }
        d += 1;
    }
    if x > 1 {
        primes.push(x);
    }
    for q in primes {
        while k % q == 0 && c.mul(p, k / q) == Pt::Inf {
            k /= q;
        }
    }
    k
}

/// Detect a chain in a slot system. None unless every slot is {T} or {T, +-Q} and the size-3 slots (in index order, at least 2)
/// satisfy S_{k+1} = sigma(S_k) as point sets.
pub fn detect_chain(c: &Curve, slots: &[Vec<Pt>]) -> Option<Chain> {
    let t = c.t2();
    let mut prev: Option<Pt> = None; // sigma^k P of the previous size-3 slot
    let (mut l, mut p0) = (0usize, Pt::Inf);
    for s in slots {
        match s.len() {
            1 => {
                if s[0] != t {
                    return None;
                }
            }
            3 => {
                if !s.contains(&t) {
                    return None;
                }
                let q: Vec<Pt> = s.iter().cloned().filter(|&x| x != t).collect();
                if q.len() != 2 || q[1] != c.neg(q[0]) {
                    return None;
                }
                match prev {
                    None => {
                        p0 = q[0];
                        prev = Some(q[0]);
                    }
                    Some(pp) => {
                        let img = c.frob_pt(pp);
                        if q[0] != img && q[0] != c.neg(img) {
                            return None;
                        }
                        prev = Some(img);
                    }
                }
                l += 1;
            }
            _ => return None,
        }
    }
    if l >= 2 {
        Some(Chain { l, p0 })
    } else {
        None
    }
}

/// Whether the digit model is exact for this chain: ord(P) must exceed the norm bound.
pub fn model_applies(c: &Curve, ch: &Chain, ne: u32) -> bool {
    point_order(c, ch.p0, ne) as f64 > norm_bound(ch.l)
}

/// The prediction for a slot system, or None if it is not a chain or the model's order condition fails.
pub fn predict_slots(c: &Curve, slots: &[Vec<Pt>], ne: u32) -> Option<Prediction> {
    let ch = detect_chain(c, slots)?;
    if !model_applies(c, &ch, ne) {
        return None;
    }
    Some(predict(ch.l, c.a, ne))
}
