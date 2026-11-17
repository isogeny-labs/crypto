//! Blind-spot item 19: exact finite checks behind `papers/ic-negative-results/`.
//!
//! Standard library only. Build and run (no cargo target directory needed):
//!   rustc --edition 2021 -C opt-level=3 -o <scratch>/b19 examples/blindspots_19_checks.rs
//!   <scratch>/b19 all
//!
//! Subcommands: `lemma4` | `affine` | `yield` | `all` (see PROTOCOL.md).
//!
//! * `lemma4` computes the true first fall degree d_F (field equations outside
//!   or inside the generating set) and the fake first fall degree d'_F of
//!   Nagao, ePrint 2015/984, Definitions 5 and 6, over F_2, under the EQUALITY
//!   reading of condition (1): the fall occurs at degree d when some tuple
//!   (g_i) has max_i deg(g_i f_i) = d, deg(sum g_i f_i) < d, sum g_i f_i != 0.
//!   Degrees in d'_F are taken after reduction modulo X_i^2 + X_i. The search
//!   is exact linear algebra over F_2: the tuples with deg(g_i f_i) <= d and
//!   top part of the sum zero form a subspace K; a fall exists iff K is not
//!   inside the subspace of tuples with sum zero and K is not inside the
//!   subspace of tuples with max degree <= d - 1 (a vector space is never the
//!   union of two proper subspaces).
//! * `affine` checks the Lean statements of `lean/IcNegative/AffineCore.lean`
//!   on Z/p: two distinct affine forms agree at most once, and the number of
//!   secrets on which a set of n forms is inconsistent is at most n^2 - n.
//!   Z/8 and Z/9 are the negative controls (not fields).
//! * `yield` checks the conservation identity on Z/N: the m = 3 multiset sums
//!   of any base of size B land on exactly C(B+2, 3) targets in total.

use std::collections::BTreeSet;

// ------------------------------------------------------------ F_2 polynomials

const NV_MAX: usize = 8;

/// Monomial packed as 4 bits per variable (exponent <= 15), variable i at bits 4i.
type Mono = u32;

fn mono_exp(m: Mono, i: usize) -> u32 {
    ((m >> (4 * i)) & 0xF) as u32
}

fn mono_deg(m: Mono) -> u32 {
    (0..NV_MAX).map(|i| mono_exp(m, i)).sum()
}

fn mono_mul(a: Mono, b: Mono) -> Mono {
    let mut r: Mono = 0;
    for i in 0..NV_MAX {
        let e = mono_exp(a, i) + mono_exp(b, i);
        assert!(e <= 15, "exponent overflow");
        r |= (e as Mono) << (4 * i);
    }
    r
}

/// Clamp every exponent to at most 1 (X^2 = X, the reduction mod X_i^2 + X_i over F_2).
fn mono_reduce(m: Mono) -> Mono {
    let mut r: Mono = 0;
    for i in 0..NV_MAX {
        if mono_exp(m, i) > 0 {
            r |= 1 << (4 * i);
        }
    }
    r
}

type Poly = BTreeSet<Mono>;

fn padd(a: &Poly, b: &Poly) -> Poly {
    a.symmetric_difference(b).copied().collect()
}

fn pmul_mono(m: Mono, p: &Poly) -> Poly {
    let mut out = Poly::new();
    for &t in p {
        let r = mono_mul(m, t);
        if !out.remove(&r) {
            out.insert(r);
        }
    }
    out
}

fn preduce(p: &Poly) -> Poly {
    let mut out = Poly::new();
    for &t in p {
        let r = mono_reduce(t);
        if !out.remove(&r) {
            out.insert(r);
        }
    }
    out
}

/// Total degree; -1 for the zero polynomial.
fn pdeg(p: &Poly) -> i32 {
    p.iter().map(|&m| mono_deg(m) as i32).max().unwrap_or(-1)
}

fn var(i: usize) -> Mono {
    1 << (4 * i)
}

fn mono_of(exps: &[u32]) -> Mono {
    let mut m: Mono = 0;
    for (i, &e) in exps.iter().enumerate() {
        m |= (e as Mono) << (4 * i);
    }
    m
}

fn poly_of(terms: &[&[u32]]) -> Poly {
    let mut p = Poly::new();
    for t in terms {
        let m = mono_of(t);
        if !p.remove(&m) {
            p.insert(m);
        }
    }
    p
}

macro_rules! poly {
    ($([$($e:expr),*]),*) => {
        poly_of(&[$(&[$($e as u32),*][..]),*])
    };
}

/// All monomials in `n` variables of total degree <= `k`.
fn monos_upto(n: usize, k: u32) -> Vec<Mono> {
    fn rec(i: usize, n: usize, left: u32, cur: Mono, out: &mut Vec<Mono>) {
        if i == n {
            out.push(cur);
            return;
        }
        for e in 0..=left {
            rec(i + 1, n, left - e, cur | ((e as Mono) << (4 * i)), out);
        }
    }
    let mut out = Vec::new();
    rec(0, n, k, 0, &mut out);
    out
}

fn monos_exact(n: usize, k: u32) -> Vec<Mono> {
    monos_upto(n, k).into_iter().filter(|&m| mono_deg(m) == k).collect()
}

/// Multilinear monomials (exponents 0 or 1) in `n` variables.
fn monos_multilinear(n: usize) -> Vec<Mono> {
    (0..(1u32 << n))
        .map(|mask| {
            let mut m: Mono = 0;
            for i in 0..n {
                if mask >> i & 1 == 1 {
                    m |= 1 << (4 * i);
                }
            }
            m
        })
        .collect()
}

// ------------------------------------------------------------ F_2 bit vectors

#[derive(Clone)]
struct Bits(Vec<u64>);

impl Bits {
    fn new(n: usize) -> Bits {
        Bits(vec![0; n.div_ceil(64).max(1)])
    }
    fn set(&mut self, i: usize) {
        self.0[i / 64] |= 1u64 << (i % 64);
    }
    fn get(&self, i: usize) -> bool {
        self.0[i / 64] >> (i % 64) & 1 == 1
    }
    fn xor(&mut self, o: &Bits) {
        for (a, b) in self.0.iter_mut().zip(&o.0) {
            *a ^= *b;
        }
    }
    fn lowest(&self) -> Option<usize> {
        for (w, &x) in self.0.iter().enumerate() {
            if x != 0 {
                return Some(w * 64 + x.trailing_zeros() as usize);
            }
        }
        None
    }
}

/// Kernel basis of the linear map whose columns are `cols` (each of length `rows`
/// bits). Returns kernel vectors as lists of column indices.
fn kernel(cols: &[Bits]) -> Vec<Vec<usize>> {
    let nc = cols.len();
    let mut pivots: Vec<(Bits, Bits)> = Vec::new(); // (image, combination)
    let mut ker = Vec::new();
    for (b, c) in cols.iter().enumerate() {
        let mut v = c.clone();
        let mut comb = Bits::new(nc);
        comb.set(b);
        loop {
            match v.lowest() {
                None => {
                    let idx: Vec<usize> = (0..nc).filter(|&i| comb.get(i)).collect();
                    ker.push(idx);
                    break;
                }
                Some(low) => {
                    if let Some((pv, pc)) = pivots.iter().find(|(pv, _)| pv.lowest() == Some(low)) {
                        v.xor(pv);
                        comb.xor(pc);
                    } else {
                        pivots.push((v, comb));
                        break;
                    }
                }
            }
        }
    }
    ker
}

// ---------------------------------------------------- first fall degrees (F_2)

/// True first fall degree at degree `d` for generators `gens` (unreduced) in `n`
/// variables: does a fall exist exactly at `d`?
fn true_fall_at(gens: &[Poly], n: usize, d: u32) -> bool {
    // Basis elements (j, mono) with deg(mono) + deg(f_j) <= d.
    let mut basis: Vec<(usize, Mono, Poly, u32)> = Vec::new();
    for (j, f) in gens.iter().enumerate() {
        let df = pdeg(f);
        if df < 0 || df as u32 > d {
            continue;
        }
        for m in monos_upto(n, d - df as u32) {
            let img = pmul_mono(m, f);
            let dg = mono_deg(m) + df as u32;
            basis.push((j, m, img, dg));
        }
    }
    let top = monos_exact(n, d);
    let cols: Vec<Bits> = basis
        .iter()
        .map(|(_, _, img, _)| {
            let mut c = Bits::new(top.len());
            for (r, t) in top.iter().enumerate() {
                if img.contains(t) {
                    c.set(r);
                }
            }
            c
        })
        .collect();
    let ker = kernel(&cols);
    let mut not_in_z = false;
    let mut not_in_w = false;
    for t in &ker {
        let mut sum = Poly::new();
        let mut uses_top = false;
        for &b in t {
            sum = padd(&sum, &basis[b].2);
            if basis[b].3 == d {
                uses_top = true;
            }
        }
        if !sum.is_empty() {
            not_in_z = true;
        }
        if uses_top {
            not_in_w = true;
        }
    }
    not_in_z && not_in_w
}

fn true_first_fall(gens: &[Poly], n: usize, dmax: u32) -> Option<u32> {
    (1..=dmax).find(|&d| true_fall_at(gens, n, d))
}

/// Fake first fall at degree `d`: multipliers multilinear, degrees after reduction.
fn fake_fall_at(gens: &[Poly], n: usize, d: u32) -> bool {
    let ml = monos_multilinear(n);
    let reduced: Vec<Poly> = gens.iter().map(preduce).collect();
    // basis: (j, image)
    let mut basis: Vec<(usize, Poly)> = Vec::new();
    for (j, f) in reduced.iter().enumerate() {
        for &m in &ml {
            basis.push((j, preduce(&pmul_mono(m, f))));
        }
    }
    // rows: (a) per generator j, every multilinear monomial of degree > d;
    //       (b) the sum's monomials of degree >= d.
    let high: Vec<Mono> = ml.iter().copied().filter(|&m| mono_deg(m) > d).collect();
    let topd: Vec<Mono> = ml.iter().copied().filter(|&m| mono_deg(m) >= d).collect();
    let rows_a = gens.len() * high.len();
    let rows = rows_a + topd.len();
    let cols: Vec<Bits> = basis
        .iter()
        .map(|(j, img)| {
            let mut c = Bits::new(rows);
            for (r, t) in high.iter().enumerate() {
                if img.contains(t) {
                    c.set(j * high.len() + r);
                }
            }
            for (r, t) in topd.iter().enumerate() {
                if img.contains(t) {
                    c.set(rows_a + r);
                }
            }
            c
        })
        .collect();
    let ker = kernel(&cols);
    let mut not_in_z = false;
    let mut not_in_w = false;
    for t in &ker {
        let mut sum = Poly::new();
        let mut per_gen: Vec<Poly> = vec![Poly::new(); gens.len()];
        for &b in t {
            sum = padd(&sum, &basis[b].1);
            let j = basis[b].0;
            per_gen[j] = padd(&per_gen[j], &basis[b].1);
        }
        if !sum.is_empty() {
            not_in_z = true;
        }
        if per_gen.iter().any(|p| pdeg(p) >= d as i32) {
            not_in_w = true;
        }
    }
    not_in_z && not_in_w
}

fn fake_first_fall(gens: &[Poly], n: usize, dmax: u32) -> Option<u32> {
    (1..=dmax).find(|&d| fake_fall_at(gens, n, d))
}

fn field_eqs(n: usize) -> Vec<Poly> {
    (0..n)
        .map(|i| {
            let mut p = Poly::new();
            p.insert((2 as Mono) << (4 * i));
            p.insert(var(i));
            p
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Falls {
    d_out: Option<u32>,
    d_in: Option<u32>,
    d_fake: Option<u32>,
}

fn falls(gens: &[Poly], n: usize, dmax: u32) -> Falls {
    let mut with_fe = gens.to_vec();
    with_fe.extend(field_eqs(n));
    Falls {
        d_out: true_first_fall(gens, n, dmax),
        d_in: true_first_fall(&with_fe, n, dmax),
        d_fake: fake_first_fall(gens, n, dmax),
    }
}

fn show(o: Option<u32>, dmax: u32) -> String {
    match o {
        Some(d) => d.to_string(),
        None => format!(">{dmax}"),
    }
}

/// Violation of d_F <= d'_F: true degree strictly above the fake one.
fn violates(t: Option<u32>, f: Option<u32>) -> Option<bool> {
    match (t, f) {
        (Some(t), Some(f)) => Some(t > f),
        (None, Some(_)) => Some(true), // d_F > dmax >= d'_F
        (Some(_), None) => Some(false),
        (None, None) => None, // undetermined at this cap
    }
}

// ===================================================================== family F(A, B, v)
// Lines 1..384 above are lines 1..384 of
// research/blindspots_20261018/instruments/blindspots_19_checks.rs
// (sha256 751277a8c075483cb0d1edcff9ef44e76f21164e989e289c8c6a54950c158ec5) with exactly
// two changes: line 31 `NV_MAX` 4 -> 8 and line 34 `Mono` u16 -> u32 (4 bits per variable,
// 8 variables). Nothing else in the core differs.
// Driver: research/inside_gap_n4_20261102/PROTOCOL.md, Amendment 1. Variable 0 is v;
// F(A, B, v) = {A v + 1, B v + B + 1}.

fn mono_vars(vars: &[usize]) -> Mono {
    vars.iter().fold(0 as Mono, |m, &i| m | (1 << (4 * i)))
}

fn system(a: &[usize], b: &[usize]) -> Vec<Poly> {
    let v = mono_vars(&[0]);
    let am = mono_vars(a);
    let bm = mono_vars(b);
    let f1: Poly = [mono_mul(am, v), 0].into_iter().collect();
    let f2: Poly = [mono_mul(bm, v), bm, 0].into_iter().collect();
    vec![f1, f2]
}

fn main() {
    println!("# family F(A,B,v) = {{A v + 1, B v + B + 1}}; v = x0; conjecture d_F(inside) = min(deg A, deg B) + 2, d'_F = 1");
    let mut all_ok = true;
    let mut cases: Vec<(String, Vec<usize>, Vec<usize>, usize)> = Vec::new();
    for k in 1..=5usize {
        // shared: A = x1..xk, B = x1..x(k-1) x(k+1); n = k + 2
        let a: Vec<usize> = (1..=k).collect();
        let mut b: Vec<usize> = (1..k).collect();
        b.push(k + 1);
        cases.push((format!("shared k={k}"), a, b, k + 2));
    }
    for k in 1..=3usize {
        // disjoint: A = x1..xk, B = x(k+1)..x(2k); n = 2k + 1
        let a: Vec<usize> = (1..=k).collect();
        let b: Vec<usize> = (k + 1..=2 * k).collect();
        cases.push((format!("disjoint k={k}"), a, b, 2 * k + 1));
    }
    for (a, b, label) in [(vec![1usize, 2], vec![3usize], "unequal 2,1"), (vec![1, 2, 3], vec![1, 4], "unequal 3,2")] {
        let n = 1 + *a.iter().chain(b.iter()).max().unwrap();
        cases.push((label.to_string(), a, b, n));
    }
    for (label, a, b, n) in cases {
        let g = system(&a, &b);
        let mut with_fe = g.clone();
        with_fe.extend(field_eqs(n));
        let want = a.len().min(b.len()) as u32 + 2;
        let din = true_first_fall(&with_fe, n, want + 1);
        let dfk = fake_first_fall(&g, n, n as u32);
        let ok = din == Some(want) && dfk == Some(1);
        all_ok &= ok;
        println!("FAMILY {label}: n={n} deg A={} deg B={} d_F(inside)={} d'_F={} predicted ({want}, 1) -> {}",
            a.len(), b.len(), show(din, want + 1), show(dfk, n as u32), if ok { "MATCH" } else { "MISMATCH" });
    }
    println!("FAMILY RESULT -> {}", if all_ok { "ALL MATCH" } else { "SOME MISMATCH" });
    // Amendment 2: field equations alone (n >= 2) fall at degree 4.
    let mut fe_ok = true;
    for n in 2..=8usize {
        let d = true_first_fall(&field_eqs(n), n, 6);
        fe_ok &= d == Some(4);
        println!("FIELD-EQUATIONS-ONLY n={n}: d_F(S_fe)={} -> {}", show(d, 6), if d == Some(4) { "= 4" } else { "!= 4" });
    }
    println!("FIELD-EQUATIONS-ONLY RESULT (d_F = 4 for n = 2..8) -> {}", if fe_ok { "PASS" } else { "FAIL" });
    // Amendment 2: n = 1 family {X^k, X^k + 1}: d_F(inside) = k, d'_F = 1.
    let mut n1_ok = true;
    for k in 2u32..=8 {
        let f1: Poly = [k as Mono].into_iter().collect();
        let f2: Poly = [k as Mono, 0].into_iter().collect();
        let g = vec![f1, f2];
        let mut with_fe = g.clone();
        with_fe.extend(field_eqs(1));
        let din = true_first_fall(&with_fe, 1, k + 1);
        let dfk = fake_first_fall(&g, 1, 1);
        let ok = din == Some(k) && dfk == Some(1);
        n1_ok &= ok;
        println!("N1-FAMILY {{X^{k}, X^{k}+1}}: d_F(inside)={} d'_F={} gap={} -> {}", show(din, k + 1), show(dfk, 1),
            match (din, dfk) { (Some(a), Some(b)) => (a as i32 - b as i32).to_string(), _ => "?".into() }, if ok { "MATCH" } else { "MISMATCH" });
    }
    println!("N1-FAMILY RESULT (d_F(inside) = k, d'_F = 1 for k = 2..8) -> {}", if n1_ok { "PASS" } else { "FAIL" });
}
