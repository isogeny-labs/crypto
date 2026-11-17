//! Exact-count falsifier for lever 2 of `RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md`
//! §4 item 2: *count the solutions of the combined system over the closure for a toy
//! prime; if the count is at least `B^{m-1}`, the lever is dead.*
//!
//! For a base `X ⊂ F_p` of size `B` and a target `r ∈ F_p`, the closure count is
//!
//! ```text
//!     c_m(r) = #{ (x_1, …, x_m) ∈ X^m  (ordered) :  S_{m+1}(x_1, …, x_m, r) = 0 }.
//! ```
//!
//! Because every `x_i` is in `F_p` and the membership polynomial `∏(X − x)` splits, this is
//! the whole closure solution set (no solution lies outside `F_p^m`); `r` ranges over all of
//! `F_p`, twist targets included. `S_3(x,y,z) = (x−y)² z² − 2((x+y)(xy+a)+2b) z +
//! ((xy−a)² − 4b(x+y))` and `S_4(x_1,x_2,x_3,r) = Res_u(S_3(x_1,x_2,u), S_3(u,x_3,r))`, the
//! resultant of the two quadratics, `(A·F−C·D)² − (A·E−B·D)(B·F−C·E)`.
//!
//! Counting is exact and uses no sampling: `m = 2` solves the quadratic `S_3(x_i,x_j,·) = 0`
//! for every pair, `m = 3` finds the `F_p`-roots of the quartic `S_4(x_i,x_j,x_k,·)` for every
//! multiset (Cantor–Zassenhaus on degree ≤ 4), each weighted by its orderings. A tuple whose
//! polynomial in `r` vanishes identically counts for every `r` (`all_add`).
//!
//! Base families (§4.2 candidates): `subgroup` (a coset `g·μ_B`), `ap` (`a + j·d`),
//! `interval` (`0 … B−1`), `image` (a random cubic over `t = 0, 1, …`), `dickson`
//! (`w + w⁻¹`, `w ∈ μ_{2B}`); the null `null` (a uniform `B`-subset of `F_p`); and the
//! positive control `smallmult` (`x(kP)`, `1 ≤ k ≤ B`), whose count is known to reach `B`.
//!
//! `--selftest` runs the gates: the fast counts against brute-force evaluation at every
//! `r`, and `S_3`, `S_4` against the group law.
//!
//! Study: `research/lever2_exact_count_20261101/` (PROTOCOL.md). Usage:
//!
//! ```text
//! exact_count_membership --selftest
//! exact_count_membership --m 2 --sizes 128,256,512,1024 --nf 40 --nnull 400 --seed S --out DIR
//! ```
use rand::{rngs::StdRng, Rng, SeedableRng};
use rayon::prelude::*;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fs;
use std::io::Write;

const NONE: u32 = u32::MAX;

fn mulm(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}
fn addm(a: u64, b: u64, p: u64) -> u64 {
    let s = a + b;
    if s >= p {
        s - p
    } else {
        s
    }
}
fn subm(a: u64, b: u64, p: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + p - b
    }
}
fn powm(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulm(r, a, p);
        }
        a = mulm(a, a, p);
        e >>= 1;
    }
    r
}
fn invm(a: u64, p: u64) -> u64 {
    powm(a, p - 2, p)
}
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

/// `F_p` with a square-root table (`sqrt[y] = s`, `s² = y`, or `NONE`).
struct Field {
    p: u64,
    sqrt: Vec<u32>,
}
impl Field {
    fn new(p: u64) -> Field {
        let mut sqrt = vec![NONE; p as usize];
        for s in 0..p {
            let y = (s * s % p) as usize;
            if sqrt[y] == NONE {
                sqrt[y] = s as u32;
            }
        }
        Field { p, sqrt }
    }
}

enum Roots {
    All,
    Some(usize, [u64; 2]),
}

/// Roots in `F_p` of `a z² + b z + c`.
fn quad_roots(f: &Field, a: u64, b: u64, c: u64) -> Roots {
    let p = f.p;
    if a == 0 {
        if b == 0 {
            return if c == 0 {
                Roots::All
            } else {
                Roots::Some(0, [0, 0])
            };
        }
        return Roots::Some(1, [mulm(subm(0, c, p), invm(b, p), p), 0]);
    }
    let disc = subm(mulm(b, b, p), mulm(4 % p, mulm(a, c, p), p), p);
    let s = f.sqrt[disc as usize];
    if s == NONE {
        return Roots::Some(0, [0, 0]);
    }
    let inv2a = invm(mulm(2, a, p), p);
    let nb = subm(0, b, p);
    if s == 0 {
        return Roots::Some(1, [mulm(nb, inv2a, p), 0]);
    }
    let s = s as u64;
    Roots::Some(
        2,
        [
            mulm(addm(nb, s, p), inv2a, p),
            mulm(subm(nb, s, p), inv2a, p),
        ],
    )
}

/// `S_3(x, y, ·)` as `(A, B, C)`: `A z² + B z + C`.
fn s3_coeffs(x: u64, y: u64, a: u64, b: u64, p: u64) -> (u64, u64, u64) {
    let d = subm(x, y, p);
    let s = addm(x, y, p);
    let pr = mulm(x, y, p);
    let aa = mulm(d, d, p);
    let inner = addm(mulm(s, addm(pr, a, p), p), mulm(2, b, p), p);
    let bb = subm(0, mulm(2, inner, p), p);
    let q = subm(pr, a, p);
    let cc = subm(mulm(q, q, p), mulm(mulm(4, b, p), s, p), p);
    (aa, bb, cc)
}

/// Numeric `S_3(x, y, z)`.
fn s3_eval(x: u64, y: u64, z: u64, a: u64, b: u64, p: u64) -> u64 {
    let (aa, bb, cc) = s3_coeffs(x, y, a, b, p);
    addm(addm(mulm(aa, mulm(z, z, p), p), mulm(bb, z, p), p), cc, p)
}

/// `Res_u(A u² + B u + C, D u² + E u + F)`.
fn res2(aa: u64, bb: u64, cc: u64, dd: u64, ee: u64, ff: u64, p: u64) -> u64 {
    let t1 = subm(mulm(aa, ff, p), mulm(cc, dd, p), p);
    let t2 = subm(mulm(aa, ee, p), mulm(bb, dd, p), p);
    let t3 = subm(mulm(bb, ff, p), mulm(cc, ee, p), p);
    subm(mulm(t1, t1, p), mulm(t2, t3, p), p)
}

/// Numeric `S_4(x1, x2, x3, r)`.
fn s4_eval(x1: u64, x2: u64, x3: u64, r: u64, a: u64, b: u64, p: u64) -> u64 {
    let (aa, bb, cc) = s3_coeffs(x1, x2, a, b, p);
    let (dd, ee, ff) = s3_coeffs(x3, r, a, b, p);
    // S_3(u, x3, r) as a quadratic in u is S_3(x3, r, u).
    res2(aa, bb, cc, dd, ee, ff, p)
}

// ── small polynomials over F_p, little-endian ───────────────────────

fn trim(v: &mut Vec<u64>) {
    while v.last() == Some(&0) {
        v.pop();
    }
}
fn pmul(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            r[i + j] = addm(r[i + j], mulm(x, y, p), p);
        }
    }
    trim(&mut r);
    r
}
/// `(quotient, remainder)`; `b` non-empty with last coefficient non-zero.
fn pdivmod(a: &[u64], b: &[u64], p: u64) -> (Vec<u64>, Vec<u64>) {
    let mut r = a.to_vec();
    trim(&mut r);
    let db = b.len() - 1;
    if r.len() <= db {
        return (vec![], r);
    }
    let inv = invm(*b.last().unwrap(), p);
    let mut q = vec![0u64; r.len() - db];
    for i in (0..=r.len() - 1 - db).rev() {
        let c = mulm(r[i + db], inv, p);
        q[i] = c;
        if c != 0 {
            for j in 0..=db {
                r[i + j] = subm(r[i + j], mulm(c, b[j], p), p);
            }
        }
    }
    r.truncate(db);
    trim(&mut r);
    trim(&mut q);
    (q, r)
}
fn pmulmod(a: &[u64], b: &[u64], m: &[u64], p: u64) -> Vec<u64> {
    pdivmod(&pmul(a, b, p), m, p).1
}
/// `base^e mod m`.
fn ppowmod(base: &[u64], mut e: u64, m: &[u64], p: u64) -> Vec<u64> {
    let mut result = vec![1u64];
    let mut b = pdivmod(base, m, p).1;
    while e > 0 {
        if e & 1 == 1 {
            result = pmulmod(&result, &b, m, p);
        }
        b = pmulmod(&b, &b, m, p);
        e >>= 1;
    }
    result
}
fn pgcd(a: &[u64], b: &[u64], p: u64) -> Vec<u64> {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    trim(&mut a);
    trim(&mut b);
    while !b.is_empty() {
        let r = pdivmod(&a, &b, p).1;
        a = b;
        b = r;
    }
    if let Some(&l) = a.last() {
        let inv = invm(l, p);
        for c in a.iter_mut() {
            *c = mulm(*c, inv, p);
        }
    }
    a
}
/// Split a squarefree product of distinct linear factors into its roots.
fn split_linear(g: &[u64], p: u64, st: &mut u64, out: &mut Vec<u64>) {
    let d = g.len() - 1;
    if d == 0 {
        return;
    }
    if d == 1 {
        out.push(mulm(subm(0, g[0], p), invm(g[1], p), p));
        return;
    }
    loop {
        *st ^= *st << 13;
        *st ^= *st >> 7;
        *st ^= *st << 17;
        let delta = *st % p;
        let mut h = ppowmod(&[delta, 1], (p - 1) / 2, g, p);
        if h.is_empty() {
            h = vec![p - 1];
        } else {
            h[0] = subm(h[0], 1, p);
        }
        trim(&mut h);
        let gg = pgcd(g, &h, p);
        let dg = gg.len() - 1;
        if dg > 0 && dg < d {
            let other = pdivmod(g, &gg, p).0;
            split_linear(&gg, p, st, out);
            split_linear(&other, p, st, out);
            return;
        }
    }
}
/// Distinct `F_p`-roots of `f`; `None` when `f ≡ 0` (every `r`).
fn roots_of(f: &[u64], p: u64, st: &mut u64, out: &mut Vec<u64>) -> bool {
    out.clear();
    let mut f = f.to_vec();
    trim(&mut f);
    if f.is_empty() {
        return false;
    }
    if f.len() == 1 {
        return true;
    }
    let inv = invm(*f.last().unwrap(), p);
    for c in f.iter_mut() {
        *c = mulm(*c, inv, p);
    }
    let mut h = ppowmod(&[0, 1], p, &f, p);
    while h.len() < 2 {
        h.push(0);
    }
    h[1] = subm(h[1], 1, p);
    trim(&mut h);
    let g = pgcd(&f, &h, p);
    split_linear(&g, p, st, out);
    true
}

/// `S_4(x1,x2,x3,r)` as a polynomial in `r` (degree ≤ 4, little-endian).
fn s4_poly(x1: u64, x2: u64, x3: u64, a: u64, b: u64, p: u64) -> Vec<u64> {
    let (aa, bb, cc) = s3_coeffs(x1, x2, a, b, p);
    s4_poly_pair(aa, bb, cc, x3, a, b, p)
}
fn s4_poly_pair(aa: u64, bb: u64, cc: u64, x3: u64, a: u64, b: u64, p: u64) -> Vec<u64> {
    let x3s = mulm(x3, x3, p);
    // D, E, F of S_3(x3, r, u) in u, as polynomials in r.
    let d = [x3s, subm(0, mulm(2, x3, p), p), 1];
    let e = [
        subm(0, mulm(2, addm(mulm(a, x3, p), mulm(2, b, p), p), p), p),
        subm(0, mulm(2, addm(x3s, a, p), p), p),
        subm(0, mulm(2, x3, p), p),
    ];
    let f = [
        subm(mulm(a, a, p), mulm(mulm(4, b, p), x3, p), p),
        subm(0, addm(mulm(mulm(2, a, p), x3, p), mulm(4, b, p), p), p),
        x3s,
    ];
    let comb = |u: u64, v: &[u64; 3], w: u64, z: &[u64; 3]| -> Vec<u64> {
        (0..3)
            .map(|i| subm(mulm(u, v[i], p), mulm(w, z[i], p), p))
            .collect()
    };
    let mut p1 = comb(aa, &f, cc, &d);
    let mut p2 = comb(aa, &e, bb, &d);
    let mut p3 = comb(bb, &f, cc, &e);
    trim(&mut p1);
    trim(&mut p2);
    trim(&mut p3);
    let sq = pmul(&p1, &p1, p);
    let pr = pmul(&p2, &p3, p);
    let n = sq.len().max(pr.len());
    let mut r = vec![0u64; n];
    for (i, c) in r.iter_mut().enumerate() {
        *c = subm(*sq.get(i).unwrap_or(&0), *pr.get(i).unwrap_or(&0), p);
    }
    trim(&mut r);
    r
}

// ── the exact count ─────────────────────────────────────────────────

struct Counts {
    c: Vec<u32>,
    all_add: u64,
    total_ordered_solutions: u64,
}

fn count_m2(f: &Field, xs: &[u64], a: u64, b: u64) -> Counts {
    let p = f.p;
    let mut c = vec![0u32; p as usize];
    let (mut all_add, mut total) = (0u64, 0u64);
    for i in 0..xs.len() {
        for j in i..xs.len() {
            let w: u32 = if i == j { 1 } else { 2 };
            let (aa, bb, cc) = s3_coeffs(xs[i], xs[j], a, b, p);
            match quad_roots(f, aa, bb, cc) {
                Roots::All => all_add += w as u64,
                Roots::Some(n, r) => {
                    for &root in r.iter().take(n) {
                        c[root as usize] += w;
                        total += w as u64;
                    }
                }
            }
        }
    }
    Counts {
        c,
        all_add,
        total_ordered_solutions: total,
    }
}

fn count_m3(f: &Field, xs: &[u64], a: u64, b: u64) -> Counts {
    let p = f.p;
    let mut c = vec![0u32; p as usize];
    let (mut all_add, mut total) = (0u64, 0u64);
    let mut st = 0x9E37_79B9_7F4A_7C15u64;
    let mut roots = Vec::with_capacity(4);
    for i in 0..xs.len() {
        for j in i..xs.len() {
            let (aa, bb, cc) = s3_coeffs(xs[i], xs[j], a, b, p);
            for k in j..xs.len() {
                let w: u32 = if i == j && j == k {
                    1
                } else if i == j || j == k {
                    3
                } else {
                    6
                };
                let poly = s4_poly_pair(aa, bb, cc, xs[k], a, b, p);
                if !roots_of(&poly, p, &mut st, &mut roots) {
                    all_add += w as u64;
                    continue;
                }
                for &root in &roots {
                    c[root as usize] += w;
                    total += w as u64;
                }
            }
        }
    }
    Counts {
        c,
        all_add,
        total_ordered_solutions: total,
    }
}

/// Brute force: every ordered tuple, every `r`, by numeric evaluation.
fn count_brute(m: usize, xs: &[u64], a: u64, b: u64, p: u64) -> Vec<u64> {
    let mut c = vec![0u64; p as usize];
    for r in 0..p {
        let mut n = 0u64;
        if m == 2 {
            for &x1 in xs {
                for &x2 in xs {
                    if s3_eval(x1, x2, r, a, b, p) == 0 {
                        n += 1;
                    }
                }
            }
        } else {
            for &x1 in xs {
                for &x2 in xs {
                    for &x3 in xs {
                        if s4_eval(x1, x2, x3, r, a, b, p) == 0 {
                            n += 1;
                        }
                    }
                }
            }
        }
        c[r as usize] = n;
    }
    c
}

// ── bases ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Family {
    Subgroup,
    Ap,
    Interval,
    Image,
    Dickson,
    Null,
    SmallMult,
}
impl Family {
    fn name(self) -> &'static str {
        match self {
            Family::Subgroup => "subgroup",
            Family::Ap => "ap",
            Family::Interval => "interval",
            Family::Image => "image",
            Family::Dickson => "dickson",
            Family::Null => "null",
            Family::SmallMult => "smallmult",
        }
    }
    fn all() -> [Family; 7] {
        [
            Family::Subgroup,
            Family::Ap,
            Family::Interval,
            Family::Image,
            Family::Dickson,
            Family::Null,
            Family::SmallMult,
        ]
    }
}

fn nonsingular(a: u64, b: u64, p: u64) -> bool {
    let a3 = mulm(mulm(a, a, p), a, p);
    addm(mulm(4, a3, p), mulm(27, mulm(b, b, p), p), p) != 0
}

fn root_of_unity(p: u64, order: u64, rng: &mut StdRng) -> u64 {
    // `order` is a power of two dividing p − 1.
    loop {
        let h = rng.gen_range(2..p);
        let z = powm(h, (p - 1) / order, p);
        if order == 1 || powm(z, order / 2, p) != 1 {
            return z;
        }
    }
}

fn affine_add(p: u64, a: u64, pt: (u64, u64), q: (u64, u64)) -> Option<(u64, u64)> {
    if pt.0 == q.0 {
        if addm(pt.1, q.1, p) == 0 {
            return None;
        }
        let lam = mulm(
            addm(mulm(3, mulm(pt.0, pt.0, p), p), a, p),
            invm(mulm(2, pt.1, p), p),
            p,
        );
        let x3 = subm(mulm(lam, lam, p), mulm(2, pt.0, p), p);
        let y3 = subm(mulm(lam, subm(pt.0, x3, p), p), pt.1, p);
        return Some((x3, y3));
    }
    let lam = mulm(subm(q.1, pt.1, p), invm(subm(q.0, pt.0, p), p), p);
    let x3 = subm(subm(mulm(lam, lam, p), pt.0, p), q.0, p);
    let y3 = subm(mulm(lam, subm(pt.0, x3, p), p), pt.1, p);
    Some((x3, y3))
}

/// A base of `bsize` distinct elements of `F_p` and a note of its parameters.
fn make_base(
    f: &Field,
    fam: Family,
    bsize: usize,
    a: u64,
    b: u64,
    rng: &mut StdRng,
) -> (Vec<u64>, Value) {
    let p = f.p;
    match fam {
        Family::Subgroup => {
            let z = root_of_unity(p, bsize as u64, rng);
            let g = rng.gen_range(1..p);
            let xs = (0..bsize as u64)
                .map(|k| mulm(g, powm(z, k, p), p))
                .collect();
            (xs, json!({"g": g, "zeta": z}))
        }
        Family::Ap => {
            let a0 = rng.gen_range(0..p);
            let d = rng.gen_range(1..p);
            let xs = (0..bsize as u64)
                .map(|j| addm(a0, mulm(j, d, p), p))
                .collect();
            (xs, json!({"a0": a0, "d": d}))
        }
        Family::Interval => ((0..bsize as u64).collect(), json!({})),
        Family::Image => {
            let (c3, c2, c1, c0) = (
                rng.gen_range(1..p),
                rng.gen_range(0..p),
                rng.gen_range(0..p),
                rng.gen_range(0..p),
            );
            let mut seen = HashSet::new();
            let mut xs = Vec::with_capacity(bsize);
            let mut t = 0u64;
            while xs.len() < bsize {
                let v = addm(
                    addm(
                        mulm(c3, mulm(t, mulm(t, t, p), p), p),
                        mulm(c2, mulm(t, t, p), p),
                        p,
                    ),
                    addm(mulm(c1, t, p), c0, p),
                    p,
                );
                if seen.insert(v) {
                    xs.push(v);
                }
                t += 1;
            }
            (
                xs,
                json!({"c3": c3, "c2": c2, "c1": c1, "c0": c0, "t_max": t}),
            )
        }
        Family::Dickson => {
            let z = root_of_unity(p, 2 * bsize as u64, rng);
            let zi = invm(z, p);
            let xs = (1..=bsize as u64)
                .map(|k| addm(powm(z, k, p), powm(zi, k, p), p))
                .collect();
            (xs, json!({"zeta": z}))
        }
        Family::Null => {
            let mut seen = HashSet::new();
            let mut xs = Vec::with_capacity(bsize);
            while xs.len() < bsize {
                let v = rng.gen_range(0..p);
                if seen.insert(v) {
                    xs.push(v);
                }
            }
            (xs, json!({}))
        }
        Family::SmallMult => loop {
            // x(kP), 1 ≤ k ≤ B, for a random point P of order > 2B.
            let x0 = rng.gen_range(0..p);
            let rhs = addm(addm(mulm(x0, mulm(x0, x0, p), p), mulm(a, x0, p), p), b, p);
            let y0 = f.sqrt[rhs as usize];
            if y0 == NONE || y0 == 0 {
                continue;
            }
            let p0 = (x0, y0 as u64);
            let mut acc = p0;
            let mut xs = vec![acc.0];
            let mut ok = true;
            for _ in 1..bsize {
                match affine_add(p, a, acc, p0) {
                    Some(n) => {
                        acc = n;
                        xs.push(acc.0);
                    }
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if !ok {
                continue;
            }
            let set: HashSet<_> = xs.iter().copied().collect();
            if set.len() == bsize {
                break (xs, json!({"x0": x0}));
            }
        },
    }
}

// ── one instance ────────────────────────────────────────────────────

fn seed_of(base: u64, m: usize, bsize: usize, fam: Family, idx: u64) -> u64 {
    let mut z = base ^ ((m as u64) << 56) ^ ((bsize as u64) << 36) ^ ((fam as u64) << 28) ^ idx;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn run_instance(f: &Field, m: usize, bsize: usize, fam: Family, idx: u64, seed: u64) -> Value {
    let p = f.p;
    let mut rng = StdRng::seed_from_u64(seed_of(seed, m, bsize, fam, idx));
    let (a, b) = loop {
        let (a, b) = (rng.gen_range(0..p), rng.gen_range(1..p));
        if nonsingular(a, b, p) {
            break (a, b);
        }
    };
    let (xs, params) = make_base(f, fam, bsize, a, b, &mut rng);
    let counts = if m == 2 {
        count_m2(f, &xs, a, b)
    } else {
        count_m3(f, &xs, a, b)
    };
    let (mut cmax, mut cmax_curve, mut cmax_nt) = (0u64, 0u64, 0u64);
    let (mut cov, mut cov_curve, mut n_curve) = (0u64, 0u64, 0u64);
    // m = 3 has the trivial solutions (x, x, r) and permutations for every r in the base
    // (P - P + R = R): 3B - 2 ordered tuples. `cmax_nt` removes them.
    let mut in_x = vec![false; p as usize];
    for &x in &xs {
        in_x[x as usize] = true;
    }
    let trivial: u64 = if m == 3 { 3 * bsize as u64 - 2 } else { 0 };
    let mut trivial_violations = 0u64;
    for r in 0..p {
        let c = counts.c[r as usize] as u64 + counts.all_add;
        let c_nt = if in_x[r as usize] {
            if c < trivial {
                trivial_violations += 1;
            }
            c.saturating_sub(trivial)
        } else {
            c
        };
        cmax_nt = cmax_nt.max(c_nt);
        let rhs = addm(addm(mulm(r, mulm(r, r, p), p), mulm(a, r, p), p), b, p);
        let on_curve = f.sqrt[rhs as usize] != NONE;
        cmax = cmax.max(c);
        if c > 0 {
            cov += 1;
        }
        if on_curve {
            n_curve += 1;
            cmax_curve = cmax_curve.max(c);
            if c > 0 {
                cov_curve += 1;
            }
        }
    }
    let thr = (bsize as u64).pow(m as u32 - 1);
    json!({
        "m": m, "B": bsize, "p": p, "family": fam.name(), "instance": idx,
        "a": a, "b": b, "params": params,
        "cmax": cmax, "cmax_curve": cmax_curve, "cmax_nt": cmax_nt,
        "trivial_per_target": trivial, "trivial_violations": trivial_violations,
        "coverage": cov as f64 / p as f64,
        "coverage_curve": cov_curve as f64 / n_curve as f64,
        "targets_on_curve": n_curve,
        "total_ordered_solutions": counts.total_ordered_solutions,
        "all_add": counts.all_add,
        "threshold": thr,
        "fires": cmax >= thr,
        "fires_curve": cmax_curve >= thr,
    })
}

/// `p = k·2B + 1`, prime, `p ≥ B^m`: the smallest such, so `μ_{2B} ⊂ F_p^*`.
fn prime_for(bsize: usize, m: usize) -> u64 {
    let step = 2 * bsize as u64;
    let lo = (bsize as u64).pow(m as u32);
    let mut k = lo / step + 1;
    loop {
        let p = k * step + 1;
        if p >= lo && is_prime(p) {
            return p;
        }
        k += 1;
    }
}

// ── statistics (Mann–Whitney with tie correction) ───────────────────

fn erfc(x: f64) -> f64 {
    let z = x.abs();
    let t = 1.0 / (1.0 + 0.5 * z);
    let r = t
        * (-z * z - 1.26551223
            + t * (1.00002368
                + t * (0.37409196
                    + t * (0.09678418
                        + t * (-0.18628806
                            + t * (0.27886807
                                + t * (-1.13520398
                                    + t * (1.48851587 + t * (-0.82215223 + t * 0.17087277)))))))))
            .exp();
    if x >= 0.0 {
        r
    } else {
        2.0 - r
    }
}

/// `(two-sided p, Cliff's delta)` of `fam` against `null` (delta > 0: `fam` larger).
fn mann_whitney(fam: &[f64], null: &[f64]) -> (f64, f64) {
    let (n1, n2) = (fam.len() as f64, null.len() as f64);
    let mut all: Vec<(f64, bool)> = fam
        .iter()
        .map(|&v| (v, true))
        .chain(null.iter().map(|&v| (v, false)))
        .collect();
    all.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());
    let n = all.len();
    let (mut r1, mut tie_term) = (0.0, 0.0);
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n && all[j].0 == all[i].0 {
            j += 1;
        }
        let rank = (i + 1 + j) as f64 / 2.0;
        let t = (j - i) as f64;
        tie_term += t * t * t - t;
        for item in &all[i..j] {
            if item.1 {
                r1 += rank;
            }
        }
        i = j;
    }
    let u1 = r1 - n1 * (n1 + 1.0) / 2.0;
    let mean = n1 * n2 / 2.0;
    let nn = n as f64;
    let var = n1 * n2 / 12.0 * ((nn + 1.0) - tie_term / (nn * (nn - 1.0)));
    let delta = 2.0 * u1 / (n1 * n2) - 1.0;
    if var <= 0.0 {
        return (1.0, delta);
    }
    let z = (u1 - mean) / var.sqrt();
    (erfc(z.abs() / std::f64::consts::SQRT_2), delta)
}

fn median(v: &mut [f64]) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

// ── self-test gates ─────────────────────────────────────────────────

fn selftest() -> bool {
    let mut ok = true;
    // G1: fast counts equal brute-force evaluation at every r.
    for (m, p, bsize) in [
        (2usize, 193u64, 8usize),
        (2, 193, 16),
        (3, 97, 8),
        (3, 113, 8),
    ] {
        let f = Field::new(p);
        for fam in Family::all() {
            if matches!(fam, Family::Subgroup | Family::Dickson)
                && (p - 1) % (2 * bsize as u64) != 0
            {
                continue;
            }
            for idx in 0..6u64 {
                let mut rng = StdRng::seed_from_u64(seed_of(7, m, bsize, fam, idx));
                let (a, b) = loop {
                    let (a, b) = (rng.gen_range(0..p), rng.gen_range(1..p));
                    if nonsingular(a, b, p) {
                        break (a, b);
                    }
                };
                let (xs, _) = make_base(&f, fam, bsize, a, b, &mut rng);
                let fast = if m == 2 {
                    count_m2(&f, &xs, a, b)
                } else {
                    count_m3(&f, &xs, a, b)
                };
                let brute = count_brute(m, &xs, a, b, p);
                let same = (0..p as usize).all(|r| fast.c[r] as u64 + fast.all_add == brute[r]);
                if !same {
                    ok = false;
                    eprintln!("G1 FAIL m={m} p={p} B={bsize} {} idx={idx}", fam.name());
                }
            }
        }
    }
    println!(
        "G1 (fast count = brute force at every r, m = 2, 3): {}",
        if ok { "pass" } else { "FAIL" }
    );
    // G2: S_3 and S_4 vanish at the group-law targets.
    let mut g2 = true;
    let (p, a, b) = (101u64, 7u64, 11u64);
    assert!(nonsingular(a, b, p));
    let f = Field::new(p);
    let mut pts = vec![];
    for x in 0..p {
        let rhs = addm(addm(mulm(x, mulm(x, x, p), p), mulm(a, x, p), p), b, p);
        let y = f.sqrt[rhs as usize];
        if y != NONE && y != 0 {
            pts.push((x, y as u64));
            pts.push((x, p - y as u64));
        }
    }
    let (mut n2, mut n3) = (0u64, 0u64);
    for &p1 in &pts {
        for &p2 in &pts {
            if let Some(s) = affine_add(p, a, p1, p2) {
                n2 += 1;
                if s3_eval(p1.0, p2.0, s.0, a, b, p) != 0 {
                    g2 = false;
                }
            }
        }
    }
    for &p1 in pts.iter().step_by(3) {
        for &p2 in pts.iter().step_by(2) {
            let Some(s12) = affine_add(p, a, p1, p2) else {
                continue;
            };
            for &p3 in pts.iter().step_by(2) {
                if let Some(s) = affine_add(p, a, s12, p3) {
                    n3 += 1;
                    if s4_eval(p1.0, p2.0, p3.0, s.0, a, b, p) != 0 {
                        g2 = false;
                    }
                }
            }
        }
    }
    println!(
        "G2 (S3 vanishes at x(P1+P2): {n2} cases; S4 at x(P1+P2+P3): {n3} cases): {}",
        if g2 { "pass" } else { "FAIL" }
    );
    ok &= g2;
    // G3: the polynomial-in-r form of S_4 equals numeric evaluation.
    let mut g3 = true;
    let mut rng = StdRng::seed_from_u64(3);
    for _ in 0..2000 {
        let (x1, x2, x3, r) = (
            rng.gen_range(0..p),
            rng.gen_range(0..p),
            rng.gen_range(0..p),
            rng.gen_range(0..p),
        );
        let poly = s4_poly(x1, x2, x3, a, b, p);
        let mut v = 0u64;
        for &c in poly.iter().rev() {
            v = addm(mulm(v, r, p), c, p);
        }
        if v != s4_eval(x1, x2, x3, r, a, b, p) {
            g3 = false;
        }
        // symmetry of S_4 in its first three arguments
        if s4_eval(x1, x2, x3, r, a, b, p) != s4_eval(x3, x1, x2, r, a, b, p) {
            g3 = false;
        }
    }
    println!(
        "G3 (S4 polynomial form = numeric; S4 symmetric): {}",
        if g3 { "pass" } else { "FAIL" }
    );
    ok &= g3;
    // G5: at m = 3 every base element r has at least the 3B - 2 trivial tuples.
    let mut g5 = true;
    for (p, bsize) in [(97u64, 8usize), (113, 8)] {
        let f = Field::new(p);
        for fam in Family::all() {
            for idx in 0..6u64 {
                let row = run_instance(&f, 3, bsize, fam, idx, 5);
                if row["trivial_violations"].as_u64().unwrap() != 0 {
                    g5 = false;
                    eprintln!("G5 FAIL p={p} {} idx={idx}", fam.name());
                }
            }
        }
    }
    println!(
        "G5 (m = 3: every r in the base has >= 3B - 2 trivial tuples): {}",
        if g5 { "pass" } else { "FAIL" }
    );
    ok &= g5;
    // G4: the control fires: x(kP) has a target with B ordered pairs at m = 2.
    let mut g4 = true;
    let p = prime_for(32, 2);
    let f = Field::new(p);
    for idx in 0..5u64 {
        let row = run_instance(&f, 2, 32, Family::SmallMult, idx, 11);
        if row["cmax"].as_u64().unwrap() < 32 {
            g4 = false;
        }
    }
    println!(
        "G4 (positive control x(kP) reaches B at m = 2, p = {p}, B = 32): {}",
        if g4 { "pass" } else { "FAIL" }
    );
    ok &= g4;
    ok
}

// ── main ────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut m = 2usize;
    let mut sizes: Vec<usize> = vec![];
    let (mut nf, mut nnull, mut seed) = (40usize, 400usize, 20261101u64);
    let mut out = String::new();
    let mut do_selftest = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--selftest" => do_selftest = true,
            "--m" => {
                i += 1;
                m = args[i].parse().unwrap();
            }
            "--sizes" => {
                i += 1;
                sizes = args[i].split(',').map(|s| s.parse().unwrap()).collect();
            }
            "--nf" => {
                i += 1;
                nf = args[i].parse().unwrap();
            }
            "--nnull" => {
                i += 1;
                nnull = args[i].parse().unwrap();
            }
            "--seed" => {
                i += 1;
                seed = args[i].parse().unwrap();
            }
            "--out" => {
                i += 1;
                out = args[i].clone();
            }
            other => panic!("unknown option {other}"),
        }
        i += 1;
    }
    if do_selftest {
        let ok = selftest();
        println!("selftest: {}", if ok { "ALL PASS" } else { "FAILED" });
        std::process::exit(if ok { 0 } else { 1 });
    }
    assert!(m == 2 || m == 3, "--m is 2 or 3");
    assert!(
        !sizes.is_empty() && !out.is_empty(),
        "--sizes and --out are required"
    );
    fs::create_dir_all(&out).unwrap();
    let mut rows_file = fs::File::create(format!("{out}/rows.jsonl")).unwrap();
    let mut summary = vec![];
    let started = std::time::Instant::now();
    for &bsize in &sizes {
        let p = prime_for(bsize, m);
        let f = Field::new(p);
        let thr = (bsize as u64).pow(m as u32 - 1);
        let mut jobs: Vec<(Family, u64)> = vec![];
        for fam in Family::all() {
            let n = match fam {
                Family::Null => nnull,
                _ => nf,
            };
            for idx in 0..n as u64 {
                jobs.push((fam, idx));
            }
        }
        let rows: Vec<Value> = jobs
            .par_iter()
            .map(|&(fam, idx)| run_instance(&f, m, bsize, fam, idx, seed))
            .collect();
        for r in &rows {
            writeln!(rows_file, "{r}").unwrap();
        }
        let get = |fam: Family, key: &str| -> Vec<f64> {
            rows.iter()
                .filter(|r| r["family"] == fam.name())
                .map(|r| r[key].as_f64().unwrap())
                .collect()
        };
        let mut fam_summ = vec![];
        for fam in Family::all() {
            let mut entry = json!({"family": fam.name()});
            for key in [
                "cmax",
                "cmax_nt",
                "cmax_curve",
                "coverage",
                "coverage_curve",
            ] {
                let v = get(fam, key);
                let nv = get(Family::Null, key);
                let (pv, delta) = if fam == Family::Null {
                    (1.0, 0.0)
                } else {
                    mann_whitney(&v, &nv)
                };
                let mut s = v.clone();
                let mut ns = nv.clone();
                entry[key] = json!({
                    "n": v.len(), "median": median(&mut s),
                    "min": v.iter().cloned().fold(f64::INFINITY, f64::min),
                    "max": v.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                    "null_median": median(&mut ns), "mw_p": pv, "cliff_delta": delta,
                });
            }
            let fires = rows
                .iter()
                .filter(|r| r["family"] == fam.name() && r["fires"] == true)
                .count();
            let n_f = rows.iter().filter(|r| r["family"] == fam.name()).count();
            entry["fires"] = json!(fires);
            entry["instances"] = json!(n_f);
            fam_summ.push(entry);
        }
        // A/A: the null split by instance index into two halves.
        let half = |key: &str, upper: bool| -> Vec<f64> {
            rows.iter()
                .filter(|r| {
                    r["family"] == "null"
                        && (r["instance"].as_u64().unwrap() >= (nnull / 2) as u64) == upper
                })
                .map(|r| r[key].as_f64().unwrap())
                .collect()
        };
        let mut aa = json!({});
        for key in ["cmax", "coverage"] {
            let (pv, delta) = mann_whitney(&half(key, true), &half(key, false));
            aa[key] = json!({"mw_p": pv, "cliff_delta": delta});
        }
        let trivial_violations: u64 = rows
            .iter()
            .map(|r| r["trivial_violations"].as_u64().unwrap())
            .sum();
        eprintln!(
            "m={m} B={bsize} p={p}: done, {:.0}s",
            started.elapsed().as_secs_f64()
        );
        summary.push(
            json!({"m": m, "B": bsize, "p": p, "threshold": thr, "aa_null_halves": aa,
                            "trivial_violations": trivial_violations, "families": fam_summ}),
        );
    }
    let doc = json!({"seed": seed, "nf": nf, "nnull": nnull, "m": m, "sizes": summary,
                     "wall_seconds": started.elapsed().as_secs_f64()});
    fs::write(
        format!("{out}/summary.json"),
        serde_json::to_string_pretty(&doc).unwrap(),
    )
    .unwrap();
}
