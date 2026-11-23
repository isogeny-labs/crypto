//! Radical symmetric pair-image membership for Semaev S3 over odd prime fields.
//!
//! For a squarefree split factor-base polynomial h(T)=prod_(w in W)(T-w),
//! A(s,t)=h(x)+h(y), K(s,t)=(x-y)(h(x)-h(y)), s=x+y, t=xy.
//! A=K=0 has exactly the unordered multiset images of W, INCLUDING the
//! diagonal. Unlike a norm-product presentation, the complete affine
//! zero-dimensional ideal <A,K> is radical. See the adjacent research note.
//!
//! Run:
//! cargo test --release --locked --example prime31_trace_skew_pair
//! cargo run  --release --locked --example prime31_trace_skew_pair
//! Uses the original crate Fp and Curve implementations; no external CAS.

use crypto_lib::cryptanalysis::semaev_prime_poly::{Curve, Fp};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug)]
struct Jet {
    v: u64,
    ds: u64,
    dt: u64,
}

impl Jet {
    const fn new(v: u64, ds: u64, dt: u64) -> Self {
        Self { v, ds, dt }
    }
}

fn scaled_add(f: Fp, a: Jet, k: u64, b: Jet) -> Jet {
    Jet::new(
        f.add(a.v, f.mul(k, b.v)),
        f.add(a.ds, f.mul(k, b.ds)),
        f.add(a.dt, f.mul(k, b.dt)),
    )
}

/// If U_k is the trace or skew trace, U_(k+1) = s*U_k - t*U_(k-1).
/// The derivative recurrence is differentiated exactly in Fp, not by
/// finite differences.
fn step(f: Fp, s: u64, t: u64, before: Jet, current: Jet) -> Jet {
    Jet::new(
        f.sub(f.mul(s, current.v), f.mul(t, before.v)),
        f.sub(
            f.add(current.v, f.mul(s, current.ds)),
            f.mul(t, before.ds),
        ),
        f.sub(
            f.mul(s, current.dt),
            f.add(before.v, f.mul(t, before.dt)),
        ),
    )
}

/// Return A=h(x)+h(y), K=(x-y)(h(x)-h(y)), and both Jacobian rows.
/// Fully division-free, O(deg h) field operations and constant storage.
/// Works for all (s,t) in Fp^2, including nonsplit quadratics and Delta=0.
fn pair_equations(f: Fp, h: &[u64], s: u64, t: u64) -> (Jet, Jet) {
    assert!(!h.is_empty());
    assert!(s < f.p && t < f.p);
    let delta = f.sub(f.mul(s, s), f.mul(4 % f.p, t));
    let mut trace0 = Jet::new(2 % f.p, 0, 0);
    let mut trace1 = Jet::new(s, 1, 0);
    let mut skew0 = Jet::new(0, 0, 0);
    let mut skew1 = Jet::new(delta, f.mul(2, s), f.neg(4 % f.p));

    let mut a = Jet::new(f.mul(h[0], 2 % f.p), 0, 0);
    let mut k = Jet::new(0, 0, 0);
    if h.len() >= 2 {
        a = scaled_add(f, a, h[1], trace1);
        k = scaled_add(f, k, h[1], skew1);
    }
    for &c in h.iter().skip(2) {
        let next_trace = step(f, s, t, trace0, trace1);
        let next_skew = step(f, s, t, skew0, skew1);
        trace0 = trace1;
        trace1 = next_trace;
        skew0 = skew1;
        skew1 = next_skew;
        a = scaled_add(f, a, c, trace1);
        k = scaled_add(f, k, c, skew1);
    }
    (a, k)
}

fn polynomial_from_roots(f: Fp, roots: &[u64]) -> Vec<u64> {
    let mut h = vec![1];
    for &w in roots {
        assert!(w < f.p);
        let mut next = vec![0; h.len() + 1];
        for (i, &c) in h.iter().enumerate() {
            next[i] = f.sub(next[i], f.mul(c, w));
            next[i + 1] = f.add(next[i + 1], c);
        }
        h = next;
    }
    h
}

/// Horner evaluation of h'(x) without factorials.
fn derivative_horner(f: Fp, h: &[u64], x: u64) -> u64 {
    let mut value = 0u64;
    for i in (1..h.len()).rev() {
        value = f.add(f.mul(value, x), f.mul((i as u64) % f.p, h[i]));
    }
    value
}

fn jacobian(f: Fp, a: Jet, k: Jet) -> u64 {
    f.sub(f.mul(a.ds, k.dt), f.mul(a.dt, k.ds))
}

fn point_image(xs: &[u64], p: u64) -> BTreeSet<(u64, u64)> {
    let mut result = BTreeSet::new();
    for (i, &x) in xs.iter().enumerate() {
        for &y in &xs[i..] {
            result.insert(((x + y) % p, ((x as u128 * y as u128) % p as u128) as u64));
        }
    }
    result
}

const P31: u64 = 2_147_483_647;
const A31: u64 = 847_514_846;
const B31: u64 = 2_087_444_724;
const GX: u64 = 625_037_715;
const GY: u64 = 2_100_627_251;

fn prime31_factor_base() -> Vec<u64> {
    let f = Fp::new(P31);
    let mut roots = Vec::new();
    for x in 0..1000 {
        let rhs = f.add(
            f.add(f.mul(f.mul(x, x), x), f.mul(A31, x)),
            B31,
        );
        if rhs == 0 || f.pow(rhs, (P31 - 1) / 2) == 1 {
            roots.push(x);
        }
        if roots.len() == 32 {
            break;
        }
    }
    assert_eq!(roots.len(), 32);
    roots
}

fn symmetric_s3(curve: Curve, x: u64, y: u64, z: u64) -> u64 {
    let f = curve.f;
    let s = f.add(x, y);
    let t = f.mul(x, y);
    let delta = f.sub(f.mul(s, s), f.mul(4, t));
    let m = f.add(f.mul(s, f.add(t, curve.a)), f.mul(2, curve.b));
    let c = f.sub(t, curve.a);
    let c0 = f.sub(f.mul(c, c), f.mul(f.mul(4, curve.b), s));
    f.add(f.sub(f.mul(delta, f.mul(z, z)), f.mul(f.mul(2, m), z)), c0)
}

fn main() {
    let f = Fp::new(P31);
    let curve = Curve::new(P31, A31, B31);
    let roots = prime31_factor_base();
    let h = polynomial_from_roots(f, &roots);
    let mut valid = 0usize;
    for (i, &x) in roots.iter().enumerate() {
        for &y in &roots[i..] {
            let s = f.add(x, y);
            let t = f.mul(x, y);
            let (a, k) = pair_equations(f, &h, s, t);
            assert_eq!((a.v, k.v), (0, 0));
            assert_ne!(jacobian(f, a, k), 0);
            assert_eq!(symmetric_s3(curve, x, y, GX), curve.s3(x, y, GX));
            valid += 1;
        }
    }
    let rhs = f.add(f.add(f.mul(f.mul(GX, GX), GX), f.mul(A31, GX)), B31);
    assert_eq!(f.mul(GY, GY), rhs);
    println!("Prime31 p={P31} B={} unordered-pairs={} radical-Jacobians={} S3-parity=pass", roots.len(), valid, valid);
    println!("Exact membership/polynomial stage only; NO Rust cost advantage or DLP recovery claim.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exhaustive_small_field_scheme_and_jacobian() {
        let p = 101u64;
        let f = Fp::new(p);
        for roots in [
            vec![0],
            vec![0, 1],
            vec![0, 1, 3, 5],
            vec![0, 1, 3, 5, 6, 12],
        ] {
            let h = polynomial_from_roots(f, &roots);
            let expected = point_image(&roots, p);
            let mut observed = BTreeSet::new();
            for s in 0..p {
                for t in 0..p {
                    let (a, k) = pair_equations(f, &h, s, t);
                    if a.v == 0 && k.v == 0 {
                        observed.insert((s, t));
                        assert_ne!(jacobian(f, a, k), 0);
                    }
                }
            }
            assert_eq!(observed, expected);
            assert_eq!(observed.len(), roots.len() * (roots.len() + 1) / 2);
            for (i, &x) in roots.iter().enumerate() {
                for &y in &roots[i..] {
                    let (a, k) = pair_equations(f, &h, f.add(x, y), f.mul(x, y));
                    let c = if x == y { f.neg(4) } else { f.neg(2) };
                    let predicted = f.mul(
                        c,
                        f.mul(derivative_horner(f, &h, x), derivative_horner(f, &h, y)),
                    );
                    assert_eq!(jacobian(f, a, k), predicted);
                }
            }
        }
    }

    #[test]
    fn prime31_pair_membership_s3_and_outside_negative() {
        main();
        let f = Fp::new(P31);
        let roots = prime31_factor_base();
        let h = polynomial_from_roots(f, &roots);
        for i in 0..5000u64 {
            let s = 1000 + i;
            let t = 10000 + 3 * i;
            let (a, k) = pair_equations(f, &h, s, t);
            assert_ne!((a.v, k.v), (0, 0));
        }
    }
}
