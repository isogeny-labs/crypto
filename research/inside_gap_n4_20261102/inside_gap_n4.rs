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

// ===================================================================== inside-gap search, n = 4
// Everything above this line is a verbatim copy of lines 1..384 of
// research/blindspots_20261018/instruments/blindspots_19_checks.rs
// (sha256 751277a8c075483cb0d1edcff9ef44e76f21164e989e289c8c6a54950c158ec5).
// Below: the driver for research/inside_gap_n4_20261102/PROTOCOL.md. For each system it
// computes d'_F (multilinear quotient; degrees <= n so a cap >= n is exhaustive) and the
// true first fall degree with the field equations INSIDE, and records d_F(inside) - d'_F.

use std::sync::{Arc, Mutex};

fn fmt_poly(p: &Poly, n: usize) -> String {
    let names = ["X", "Y", "Z", "W"];
    let mut terms: Vec<(u32, String)> = p
        .iter()
        .map(|&m| {
            let mut s = String::new();
            for i in 0..n {
                let e = mono_exp(m, i);
                if e == 1 {
                    s.push_str(names[i]);
                } else if e > 1 {
                    s.push_str(&format!("{}^{}", names[i], e));
                }
            }
            if s.is_empty() {
                s.push('1');
            }
            (mono_deg(m), s)
        })
        .collect();
    terms.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    terms.into_iter().map(|t| t.1).collect::<Vec<_>>().join("+")
}

fn poly_from_mask(monos: &[Mono], mask: u32) -> Poly {
    monos.iter().enumerate().filter(|(i, _)| mask >> i & 1 == 1).map(|(_, &m)| m).collect()
}

/// splitmix64: a fixed, documented PRNG so the sample is reproducible from the seed.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

#[derive(Default)]
struct Acc {
    total: u64,
    hist: std::collections::BTreeMap<i32, u64>,
    und: u64,                                      // d'_F finite, d_F(inside) > dmax
    und_examples: Vec<String>,
    best: std::collections::BTreeMap<i32, (usize, String)>, // smallest witness per gap
    no_fake: u64,
}

fn eval_pair(g: [Poly; 2], n: usize, dmax: u32, acc: &mut Acc) {
    acc.total += 1;
    let dfk = fake_first_fall(&g, n, n as u32);
    let Some(dfk) = dfk else {
        acc.no_fake += 1;
        return;
    };
    let mut with_fe = g.to_vec();
    with_fe.extend(field_eqs(n));
    let din = true_first_fall(&with_fe, n, dmax);
    let size = g[0].len() + g[1].len();
    match din {
        Some(d) => {
            let gap = d as i32 - dfk as i32;
            *acc.hist.entry(gap).or_default() += 1;
            let desc = format!("{{{}, {}}} d_F(inside)={} d'_F={}", fmt_poly(&g[0], n), fmt_poly(&g[1], n), d, dfk);
            let e = acc.best.entry(gap).or_insert((usize::MAX, String::new()));
            if size < e.0 {
                *e = (size, desc);
            }
        }
        None => {
            acc.und += 1;
            if acc.und_examples.len() < 20 {
                acc.und_examples.push(format!("{{{}, {}}} d'_F={} d_F(inside)>{}", fmt_poly(&g[0], n), fmt_poly(&g[1], n), dfk, dmax));
            }
        }
    }
}

fn merge(into: &mut Acc, from: Acc) {
    into.total += from.total;
    into.und += from.und;
    into.no_fake += from.no_fake;
    for (k, v) in from.hist {
        *into.hist.entry(k).or_default() += v;
    }
    for x in from.und_examples {
        if into.und_examples.len() < 20 {
            into.und_examples.push(x);
        }
    }
    for (k, (s, d)) in from.best {
        let e = into.best.entry(k).or_insert((usize::MAX, String::new()));
        if s < e.0 {
            *e = (s, d);
        }
    }
}

fn report(label: &str, acc: &Acc, dmax: u32) {
    println!("INSIDE-GAP {label}: systems={} no-fake-fall={} histogram d_F(inside)-d'_F {:?} undetermined(d_F(inside)>{dmax})={}",
        acc.total, acc.no_fake, acc.hist, acc.und);
    for (g, (_, d)) in &acc.best {
        println!("  SMALLEST WITNESS gap {g}: {d}");
    }
    for x in &acc.und_examples {
        println!("  UNDETERMINED: {x}");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(String::as_str).unwrap_or("");
    let n = 4usize;
    let dmax: u32 = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(6);
    let threads = 4usize;
    // Monomial sets.
    let ml2: Vec<Mono> = monos_multilinear(n).into_iter().filter(|&m| mono_deg(m) >= 0 && mono_deg(m) <= 2).collect(); // 11
    let all2: Vec<Mono> = monos_upto(n, 2); // 15
    let ml3: Vec<Mono> = monos_multilinear(n).into_iter().filter(|&m| mono_deg(m) <= 3).collect(); // 15
    let acc = Arc::new(Mutex::new(Acc::default()));
    match mode {
        // Q1: exhaustive, all unordered pairs of distinct nonconstant multilinear deg <= 2 polynomials.
        "q1" => {
            let polys: Vec<Poly> = (1u32..(1 << ml2.len())).map(|m| poly_from_mask(&ml2, m)).filter(|p| pdeg(p) >= 1).collect();
            let limit: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(polys.len());
            let polys = Arc::new(polys);
            let mut hs = Vec::new();
            for t in 0..threads {
                let (polys, acc) = (polys.clone(), acc.clone());
                hs.push(std::thread::spawn(move || {
                    let mut local = Acc::default();
                    for i in (t..limit.min(polys.len())).step_by(threads) {
                        for j in (i + 1)..polys.len() {
                            eval_pair([polys[i].clone(), polys[j].clone()], n, dmax, &mut local);
                        }
                    }
                    merge(&mut acc.lock().unwrap(), local);
                }));
            }
            for h in hs { h.join().unwrap(); }
            report(&format!("Q1 multilinear deg<=2 n=4 (first-index rows < {limit})"), &acc.lock().unwrap(), dmax);
        }
        // Q2 / Q3: seeded uniform sample of unordered pairs of distinct nonconstant polynomials.
        "q2" | "q3" => {
            let is_q2 = mode == "q2";
            let monos = if is_q2 { all2.clone() } else { ml3.clone() };
            let samples: u64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(1000);
            let per = samples / threads as u64;
            let monos = Arc::new(monos);
            let mut hs = Vec::new();
            for t in 0..threads {
                let (monos, acc) = (monos.clone(), acc.clone());
                hs.push(std::thread::spawn(move || {
                    let mut rng = Rng(0x5EED_0000_0000_0000u64 ^ (if is_q2 { 2u64 } else { 3u64 }) << 32 ^ t as u64);
                    let mut local = Acc::default();
                    let k = monos.len() as u32;
                    let mut done = 0u64;
                    while done < per {
                        let a = (rng.next() % ((1u64 << k) - 1)) as u32 + 1;
                        let b = (rng.next() % ((1u64 << k) - 1)) as u32 + 1;
                        if a == b { continue; }
                        let (p, q) = (poly_from_mask(&monos, a), poly_from_mask(&monos, b));
                        if pdeg(&p) < 1 || pdeg(&q) < 1 { continue; }
                        eval_pair([p, q], n, dmax, &mut local);
                        done += 1;
                    }
                    merge(&mut acc.lock().unwrap(), local);
                }));
            }
            for h in hs { h.join().unwrap(); }
            let label = if mode == "q2" { "Q2 sample, all deg<=2 n=4" } else { "Q3 sample, multilinear deg<=3 n=4" };
            report(&format!("{label} ({samples} pairs, seed 0x5EED..{})", if mode == "q2" { 2 } else { 3 }), &acc.lock().unwrap(), dmax);
        }
        // Series: exhaustive, all unordered pairs of nonconstant polynomials of degree <= 2 in
        // n = 2 and n = 3 variables (inside gap only), for comparison with n = 4.
        "series" => {
            for nn in [2usize, 3] {
                let monos = monos_upto(nn, 2);
                let polys: Vec<Poly> = (1u32..(1 << monos.len())).map(|m| poly_from_mask(&monos, m)).filter(|p| pdeg(p) >= 1).collect();
                let polys = Arc::new(polys);
                let acc = Arc::new(Mutex::new(Acc::default()));
                let mut hs = Vec::new();
                for t in 0..threads {
                    let (polys, acc) = (polys.clone(), acc.clone());
                    hs.push(std::thread::spawn(move || {
                        let mut local = Acc::default();
                        for i in (t..polys.len()).step_by(threads) {
                            for j in (i + 1)..polys.len() {
                                eval_pair([polys[i].clone(), polys[j].clone()], nn, dmax, &mut local);
                            }
                        }
                        merge(&mut acc.lock().unwrap(), local);
                    }));
                }
                for h in hs { h.join().unwrap(); }
                report(&format!("SERIES all deg<=2 n={nn}"), &acc.lock().unwrap(), dmax);
            }
        }
        // Control: the n = 3 witness embedded in 4 variables must still give gap 2.
        "control" => {
            let mut a = Acc::default();
            let f1 = poly!([0, 1, 1, 0], [0, 0, 0, 0]);
            let f2 = poly!([1, 0, 1, 0], [1, 0, 0, 0], [0, 0, 0, 0]);
            eval_pair([f1, f2], n, dmax, &mut a);
            let pass = a.hist.get(&2) == Some(&1);
            report("CONTROL {YZ+1, XZ+X+1} in 4 variables", &a, dmax);
            println!("CONTROL gap 2 reproduced in n = 4 -> {}", if pass { "PASS" } else { "FAIL" });
        }
        _ => eprintln!("usage: inside_gap_n4 control | q1 [row_limit] [dmax] | q2 <samples> [dmax] | q3 <samples> [dmax]"),
    }
}
