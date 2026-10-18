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

const NV_MAX: usize = 4;

/// Monomial packed as 4 bits per variable (exponent <= 15), variable i at bits 4i.
type Mono = u16;

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

fn run_lemma4() -> bool {
    let dmax = 6;
    let mut ok = true;

    // Positive control: f1 = X, f2 = X + 1 (one variable), d_F = d'_F = 1.
    let f1 = poly!([1]);
    let f2 = poly!([1], [0]);
    let c = falls(&[f1, f2], 1, dmax);
    let pass = c.d_out == Some(1) && c.d_in == Some(1) && c.d_fake == Some(1);
    println!(
        "CONTROL positive X, X+1: d_F(out)={} d_F(in)={} d'_F={} -> {}",
        show(c.d_out, dmax),
        show(c.d_in, dmax),
        show(c.d_fake, dmax),
        if pass { "PASS" } else { "FAIL" }
    );
    ok &= pass;

    // Witness A (n = 2): f1 = X^2 Y, f2 = XY + X.
    let a = [poly!([2, 1]), poly!([1, 1], [1, 0])];
    // Witness B (n = 3, multilinear): f1 = X1 X2 + X3, f2 = X1 X3 + X2.
    let b = [poly!([1, 1, 0], [0, 0, 1]), poly!([1, 0, 1], [0, 1, 0])];
    // Witness C (n = 3): f1 = X1 X2 + X3, f2 = X1 + X2 (the ledger's first, outside-form example).
    let cw = [poly!([1, 1, 0], [0, 0, 1]), poly!([1, 0, 0], [0, 1, 0])];
    for (name, g, n) in [("A (X^2Y, XY+X)", &a[..], 2), ("B (X1X2+X3, X1X3+X2)", &b[..], 3), ("C (X1X2+X3, X1+X2)", &cw[..], 3)] {
        let w = falls(g, n, dmax);
        println!(
            "WITNESS {name}: d_F(outside)={} d_F(inside)={} d'_F={}",
            show(w.d_out, dmax),
            show(w.d_in, dmax),
            show(w.d_fake, dmax)
        );
    }
    let wa = falls(&a, 2, dmax);
    let wb = falls(&b, 3, dmax);
    let pa = wa.d_fake == Some(2) && wa.d_out == Some(3) && wa.d_in == Some(3);
    let pb = wb.d_fake == Some(2) && wb.d_out == Some(3) && wb.d_in == Some(3);
    println!("CHECK witness A d'_F=2, d_F=3 outside and inside -> {}", if pa { "PASS" } else { "FAIL" });
    println!("CHECK witness B d'_F=2, d_F=3 outside and inside -> {}", if pb { "PASS" } else { "FAIL" });
    ok &= pa && pb;

    // Exhaustive census: unordered pairs of distinct multilinear polynomials of
    // degree 1..2 in three variables (126 polynomials, 7875 pairs).
    let n = 3;
    let ml: Vec<Mono> = monos_multilinear(n).into_iter().filter(|&m| mono_deg(m) <= 2).collect();
    let mut polys: Vec<Poly> = Vec::new();
    for mask in 1u32..(1 << ml.len()) {
        let p: Poly = ml.iter().enumerate().filter(|(i, _)| mask >> i & 1 == 1).map(|(_, &m)| m).collect();
        if pdeg(&p) >= 1 {
            polys.push(p);
        }
    }
    let mut total = 0u32;
    let (mut v_out, mut v_in, mut lt_out, mut lt_in, mut und_out, mut und_in) = (0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
    for i in 0..polys.len() {
        for j in (i + 1)..polys.len() {
            let w = falls(&[polys[i].clone(), polys[j].clone()], n, dmax);
            total += 1;
            match violates(w.d_out, w.d_fake) {
                Some(true) => v_out += 1,
                None => und_out += 1,
                _ => {}
            }
            match violates(w.d_in, w.d_fake) {
                Some(true) => v_in += 1,
                None => und_in += 1,
                _ => {}
            }
            if let (Some(t), Some(f)) = (w.d_out, w.d_fake) {
                if t < f {
                    lt_out += 1;
                }
            }
            if let (Some(t), Some(f)) = (w.d_in, w.d_fake) {
                if t < f {
                    lt_in += 1;
                }
            }
        }
    }
    println!(
        "CENSUS pairs={} (126 polys, n=3, dmax={dmax}): d_F>d'_F outside={} inside={}; d_F<d'_F outside={} inside={}; undetermined outside={} inside={}",
        total, v_out, v_in, lt_out, lt_in, und_out, und_in
    );
    // The census must contain at least the two witnesses as violations, in both placements.
    let pass = v_out > 0 && v_in > 0;
    println!("CHECK census finds violations of d_F <= d'_F in both placements -> {}", if pass { "PASS" } else { "FAIL" });
    ok &= pass;
    ok
}

// ------------------------------------------------------- affine core (Z/modulus)

/// Advance `idx` (a strictly increasing selection from 0..total) to the next
/// combination; false when it was the last one.
fn next_comb(idx: &mut [usize], total: usize) -> bool {
    let n = idx.len();
    let mut k = n;
    while k > 0 {
        k -= 1;
        if idx[k] != total - n + k {
            idx[k] += 1;
            for t in (k + 1)..n {
                idx[t] = idx[t - 1] + 1;
            }
            return true;
        }
    }
    false
}

fn agree_count(m: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    (0..m).filter(|&x| (a + b * x) % m == (c + d * x) % m).count() as u32
}

fn run_affine() -> bool {
    let mut ok = true;
    for &p in &[5u32, 7, 11] {
        let mut max_agree = 0;
        let mut pairs = 0u64;
        for a in 0..p {
            for b in 0..p {
                for c in 0..p {
                    for d in 0..p {
                        if (a, b) != (c, d) {
                            pairs += 1;
                            max_agree = max_agree.max(agree_count(p, a, b, c, d));
                        }
                    }
                }
            }
        }
        let pass = max_agree <= 1;
        println!("CHECK Z/{p}: {pairs} ordered pairs of distinct forms, max agreements {max_agree} <= 1 -> {}", if pass { "PASS" } else { "FAIL" });
        ok &= pass;
    }
    // Union bound: bad secrets of a set of n forms is at most n^2 - n.
    for &(p, n) in &[(5u32, 4usize), (7, 3), (7, 4), (11, 3)] {
        let forms: Vec<(u32, u32)> = (0..p).flat_map(|a| (0..p).map(move |b| (a, b))).collect();
        let mut max_bad = 0usize;
        let mut sets = 0u64;
        let mut idx: Vec<usize> = (0..n).collect();
        loop {
            sets += 1;
            let bad = (0..p)
                .filter(|&x| {
                    (0..n).any(|i| {
                        (0..n).any(|j| {
                            i != j && {
                                let (a, b) = forms[idx[i]];
                                let (c, d) = forms[idx[j]];
                                (a + b * x) % p == (c + d * x) % p
                            }
                        })
                    })
                })
                .count();
            max_bad = max_bad.max(bad);
            if !next_comb(&mut idx, forms.len()) {
                break;
            }
        }
        let bound = n * n - n;
        let unordered = n * (n - 1) / 2;
        let pass = max_bad <= unordered && unordered <= bound;
        println!(
            "CHECK Z/{p}: {sets} sets of n={n} forms, max bad secrets {max_bad} <= n(n-1)/2={unordered} <= n^2-n={bound} -> {}",
            if pass { "PASS" } else { "FAIL" }
        );
        ok &= pass;
    }
    // Negative control: composite moduli break the lemma.
    for &m in &[8u32, 9] {
        let mut max_agree = 0;
        let mut witness = (0, 0, 0, 0);
        for a in 0..m {
            for b in 0..m {
                for c in 0..m {
                    for d in 0..m {
                        if (a, b) != (c, d) {
                            let k = agree_count(m, a, b, c, d);
                            if k > max_agree {
                                max_agree = k;
                                witness = (a, b, c, d);
                            }
                        }
                    }
                }
            }
        }
        let pass = max_agree > 1;
        println!(
            "CONTROL negative Z/{m}: max agreements {max_agree} at forms {:?} (>1 expected, not a field) -> {}",
            witness,
            if pass { "PASS" } else { "FAIL" }
        );
        ok &= pass;
    }
    ok
}

// ------------------------------------------------------------ yield (Z/N)

fn choose(n: u64, k: u64) -> u64 {
    let mut r = 1u64;
    for i in 0..k {
        r = r * (n - i) / (i + 1);
    }
    r
}

fn run_yield() -> bool {
    let mut ok = true;
    let m = 3usize;
    for &n in &[11u32, 12, 13] {
        for &bsz in &[3usize, 4] {
            let nonzero: Vec<u32> = (1..n).collect();
            let expect = choose((bsz + m - 1) as u64, m as u64);
            let mut idx: Vec<usize> = (0..bsz).collect();
            let mut bases = 0u64;
            let (mut cov_min, mut cov_max) = (usize::MAX, 0usize);
            let mut all_total = true;
            loop {
                bases += 1;
                let base: Vec<u32> = idx.iter().map(|&i| nonzero[i]).collect();
                let mut c = vec![0u64; n as usize];
                for i in 0..bsz {
                    for j in i..bsz {
                        for k in j..bsz {
                            c[((base[i] + base[j] + base[k]) % n) as usize] += 1;
                        }
                    }
                }
                let total: u64 = c.iter().sum();
                all_total &= total == expect;
                let cov = c.iter().filter(|&&x| x > 0).count();
                cov_min = cov_min.min(cov);
                cov_max = cov_max.max(cov);
                if !next_comb(&mut idx, nonzero.len()) {
                    break;
                }
            }
            // Mirror control: an arithmetic progression has the same total, lower coverage.
            let ap: Vec<u32> = (1..=bsz as u32).collect();
            let mut c = vec![0u64; n as usize];
            for i in 0..bsz {
                for j in i..bsz {
                    for k in j..bsz {
                        c[((ap[i] + ap[j] + ap[k]) % n) as usize] += 1;
                    }
                }
            }
            let ap_total: u64 = c.iter().sum();
            let ap_cov = c.iter().filter(|&&x| x > 0).count();
            let pass = all_total && ap_total == expect && ap_cov < cov_max;
            println!(
                "CHECK Z/{n} B={bsz} m=3: {bases} bases, every total = C(B+2,3) = {expect}; coverage min {cov_min} max {cov_max}; AP base total {ap_total} coverage {ap_cov} -> {}",
                if pass { "PASS" } else { "FAIL" }
            );
            ok &= pass;
        }
    }
    ok
}

fn main() {
    let cmd = std::env::args().nth(1).unwrap_or_else(|| "all".to_string());
    let ok = match cmd.as_str() {
        "lemma4" => run_lemma4(),
        "affine" => run_affine(),
        "yield" => run_yield(),
        "all" => {
            let a = run_lemma4();
            let b = run_affine();
            let c = run_yield();
            a && b && c
        }
        other => {
            eprintln!("unknown subcommand {other}; use lemma4 | affine | yield | all");
            std::process::exit(2);
        }
    };
    println!("{}", if ok { "ALL CHECKS PASS" } else { "SOME CHECK FAILED" });
    if !ok {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn witnesses_and_controls() {
        assert!(run_lemma4());
    }

    #[test]
    fn affine_core() {
        assert!(run_affine());
    }

    #[test]
    fn yield_identity() {
        assert!(run_yield());
    }
}
