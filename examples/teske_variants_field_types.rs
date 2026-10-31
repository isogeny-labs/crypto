//! Teske-type hidden Weil descents by field type: exact enumeration.
//!
//! Companion to `research/teske_variants_field_types_20261018/` (protocol,
//! report). The file uses `std` only, so it builds without the crate:
//!
//! ```text
//! rustc --edition 2021 -O examples/teske_variants_field_types.rs -o tvft
//! ./tvft research/teske_variants_field_types_20261018/results [A|B|C ...]
//! # tests: rustc --edition 2021 -O --test examples/teske_variants_field_types.rs -o t && ./t
//! ```
//!
//! **Part A (binary fields).** For odd `n`, the irreducible factors of
//! `x^n - 1` over `F_2` are the cyclotomic cosets of `2` modulo `n`. A curve
//! `y^2 + xy = x^3 + ax^2 + b` over `F_{2^{nl}}` has GHS magic number
//! `m = deg Ord_b + [x+1 does not divide Ord_b]` and descent genus
//! `2^{m-1}` or `2^{m-1} - 1` (GHS 2002 Thm 1; Maurer–Menezes–Teske 2002
//! Thm 5). The number of `b` with `Ord_b | F` is exactly `q^{deg F}`,
//! `q = 2^l`, so a random curve has `Ord_b | F` with probability
//! `q^{-(n - deg F)}`. Hess's generalisation (Hess 2003 §4; Karabina–
//! Menezes–Pomerance–Shparlinski 2010 eq. (4)) writes `b = (alpha beta)^2`
//! and gets genus `2^t - 2^{t-r} - 2^{t-s} + 1` for `r = deg Ord_alpha`,
//! `s = deg Ord_beta`, `t = deg lcm`, from about `q^{r+s-1}` values of `b`.
//! The tool enumerates both and prints, for a few `N = n l`, the exponents
//! of rho, of the owner's index calculus, and of an auditor's isogeny walk.
//!
//! **Part B (odd characteristic).** `E/F_{q^n}`, `q = p` an odd prime, with
//! a degree-2 function whose branch set `B` has `sigma`-orbit union `S`,
//! `r = |S|`, span dimension `m`, GHS genus `1 + 2^{m-2}(r-4)`. The tool
//! instantiates every `sigma`-stable `S` of the allowed orbit sizes inside an
//! ambient field `F_{q^L}`, every weak `B` inside it, the `j`-invariant of
//! `E_B` and (by exact point counting) its trace, together with the trace
//! of every `j` in `F_{q^n}`, and reports how the weak curves sit inside
//! the isogeny classes.
//!
//! **Part C (census).** Every hyperelliptic genus-3 curve `y^2 = c f(x)` over
//! `F_q` whose `L`-polynomial is `1 - tT^3 + q^3T^6` has Jacobian isogenous
//! to `Res_{F_{q^3}/F_q} E` for an `E/F_{q^3}` of trace `t` (Tate). The tool
//! finds all of them, canonicalises under `PGL_2(F_q)`, factors the
//! Weierstrass octic, computes the `F_{q^3}`-rational reduced automorphism
//! group from the roots, and tests the GHS compositum signature: a
//! Frobenius-cyclic Klein four-group of `F_{q^3}`-rational involutions with
//! the eight Weierstrass points as two free orbits.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::time::Instant;

// ===========================================================================
// Integer helpers

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

fn powmod(mut b: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64 % m;
    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = (r as u128 * b as u128 % m as u128) as u64;
        }
        b = (b as u128 * b as u128 % m as u128) as u64;
        e >>= 1;
    }
    r
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

/// Distinct prime factors by trial division (inputs stay below 2^40 here).
fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n.is_multiple_of(d) {
            out.push(d);
            while n.is_multiple_of(d) {
                n /= d;
            }
        }
        d += 1;
    }
    if n > 1 {
        out.push(n);
    }
    out
}

/// Multiplicative order of `a` modulo `n`, `gcd(a, n) = 1`.
fn mult_order(a: u64, n: u64) -> u64 {
    let mut k = 1;
    let mut x = a % n;
    while x != 1 {
        x = x * a % n;
        k += 1;
    }
    k
}

fn log2_f(x: f64) -> f64 {
    x.ln() / std::f64::consts::LN_2
}

// ===========================================================================
// Part A: binary fields

/// One irreducible factor of `x^n - 1` over `F_2`: a cyclotomic coset.
#[derive(Clone, Debug)]
struct Coset {
    /// Smallest element of the coset (`0` for the factor `x + 1`).
    rep: u32,
    /// Degree of the factor.
    deg: u32,
    /// Order of the factor: the order of the root `zeta^rep`.
    order: u32,
}

fn cyclotomic_cosets(n: u32) -> Vec<Coset> {
    assert!(n % 2 == 1, "odd n only");
    let mut seen = vec![false; n as usize];
    let mut out = Vec::new();
    for a in 0..n {
        if seen[a as usize] {
            continue;
        }
        let mut deg = 0;
        let mut x = a;
        loop {
            seen[x as usize] = true;
            deg += 1;
            x = (x * 2) % n;
            if x == a {
                break;
            }
        }
        let order = n / gcd(a as u64, n as u64) as u32;
        out.push(Coset { rep: a, deg, order });
    }
    out
}

/// Genus `2^{m-1} - [type I]` as `(exact if it fits, log2)`.
fn ghs_genus(m: u32, type_i: bool) -> (Option<u128>, f64) {
    if m <= 120 {
        let g = (1u128 << (m - 1)) - if type_i { 1 } else { 0 };
        (Some(g), log2_f(g as f64))
    } else {
        (None, (m - 1) as f64)
    }
}

#[derive(Clone, Debug)]
struct BinRow {
    deg: u32,
    type_i: bool,
    m: u32,
    genus: Option<u128>,
    log2_genus: f64,
    delta: u32,
    /// Number of products `F` with this `(deg, type)` and `lcm(orders) = n`.
    num_f: u64,
}

/// A1: every product of distinct irreducible factors with `lcm(orders) = n`.
fn binary_rows(n: u32) -> Vec<BinRow> {
    let cos = cyclotomic_cosets(n);
    let k = cos.len();
    assert!(k <= 22, "too many cosets for n = {n}");
    let mut by_key: BTreeMap<(u32, bool), u64> = BTreeMap::new();
    for mask in 1u32..(1u32 << k) {
        let mut l = 1u64;
        let mut deg = 0u32;
        let mut has_one = false;
        for (i, c) in cos.iter().enumerate() {
            if mask >> i & 1 == 1 {
                l = lcm(l, c.order as u64);
                deg += c.deg;
                if c.rep == 0 {
                    has_one = true;
                }
            }
        }
        if l != n as u64 {
            continue;
        }
        *by_key.entry((deg, !has_one)).or_insert(0) += 1;
    }
    by_key
        .into_iter()
        .map(|((deg, type_i), num_f)| {
            let m = deg + if type_i { 1 } else { 0 };
            let (genus, log2_genus) = ghs_genus(m, type_i);
            BinRow {
                deg,
                type_i,
                m,
                genus,
                log2_genus,
                delta: n - deg,
                num_f,
            }
        })
        .collect()
}

#[derive(Clone, Debug)]
struct GghsRow {
    r: u32,
    s: u32,
    t: u32,
    genus: Option<u128>,
    log2_genus: f64,
    delta: u32,
    num_pairs: u64,
}

/// A2: pairs `(F_alpha, F_beta)`, each a single factor, `(x+1)` times one,
/// or `(x+1)`, with `lcm(orders of F_alpha F_beta) = n`.
fn gghs_rows(n: u32) -> Vec<GghsRow> {
    let cos = cyclotomic_cosets(n);
    // candidate F: (set of coset indices)
    let mut cands: Vec<Vec<usize>> = Vec::new();
    let one = cos.iter().position(|c| c.rep == 0).unwrap();
    cands.push(vec![one]);
    for (i, _) in cos.iter().enumerate() {
        if i == one {
            continue;
        }
        cands.push(vec![i]);
        cands.push(vec![one, i]);
    }
    let mut by_key: BTreeMap<(u32, u32, u32), u64> = BTreeMap::new();
    for fa in &cands {
        for fb in &cands {
            let mut union: BTreeSet<usize> = BTreeSet::new();
            union.extend(fa.iter().copied());
            union.extend(fb.iter().copied());
            let l = union
                .iter()
                .fold(1u64, |acc, &i| lcm(acc, cos[i].order as u64));
            if l != n as u64 {
                continue;
            }
            let r: u32 = fa.iter().map(|&i| cos[i].deg).sum();
            let s: u32 = fb.iter().map(|&i| cos[i].deg).sum();
            let t: u32 = union.iter().map(|&i| cos[i].deg).sum();
            let (r, s) = if r <= s { (r, s) } else { (s, r) };
            *by_key.entry((r, s, t)).or_insert(0) += 1;
        }
    }
    let mut rows: Vec<GghsRow> = by_key
        .into_iter()
        .map(|((r, s, t), num_pairs)| {
            let (genus, log2_genus) = if t <= 120 {
                let g = (1u128 << t) - (1u128 << (t - r)) - (1u128 << (t - s)) + 1;
                (Some(g), log2_f(g as f64))
            } else {
                (None, t as f64)
            };
            GghsRow {
                r,
                s,
                t,
                genus,
                log2_genus,
                delta: n + 1 - r - s,
                num_pairs,
            }
        })
        .collect();
    rows.sort_by(|a, b| {
        a.delta
            .cmp(&b.delta)
            .then(a.log2_genus.partial_cmp(&b.log2_genus).unwrap())
    });
    rows
}

/// Expected number of weak Frobenius-orbit classes per isogeny class
/// (GHS 2002 §5; Yee 2016 §5.2): `c q^{deg - n/2} / (n l)`, capped at 1.
fn per_class_expectation(c: f64, deg: f64, n: u32, l: u32) -> f64 {
    let q = 2f64.powi(l as i32);
    (c * q.powf(deg - n as f64 / 2.0) / (n as f64 * l as f64)).min(1.0)
}

fn part_a(out: &Path, log: &mut String) {
    let mut types =
        String::from("n,ord_2_mod_n,num_cosets,deg,type,m,genus,log2_genus,delta,num_F\n");
    let mut gg = String::from("n,r,s,t,genus,log2_genus,delta,num_pairs,is_min_genus_for_delta\n");
    let mut summary = String::from(
        "n,ord_2_mod_n,num_cosets,best_typeI_deg,best_typeI_genus,best_typeI_delta,best_typeII_deg,best_typeII_genus,best_typeII_delta,best_gghs_r,best_gghs_s,best_gghs_genus,best_gghs_delta,half_n,margin_typeI,margin_typeII,margin_gghs\n",
    );
    let mut n = 3;
    while n <= 99 {
        let cos = cyclotomic_cosets(n);
        let ord = mult_order(2, n as u64);
        let rows = binary_rows(n);
        for r in &rows {
            let _ = writeln!(
                types,
                "{},{},{},{},{},{},{},{:.3},{},{}",
                n,
                ord,
                cos.len(),
                r.deg,
                if r.type_i { "I" } else { "II" },
                r.m,
                r.genus.map(|g| g.to_string()).unwrap_or_else(|| "-".into()),
                r.log2_genus,
                r.delta,
                r.num_f
            );
        }
        let grows = gghs_rows(n);
        // min genus per delta
        let mut best_per_delta: BTreeMap<u32, f64> = BTreeMap::new();
        for g in &grows {
            let e = best_per_delta.entry(g.delta).or_insert(f64::INFINITY);
            if g.log2_genus < *e {
                *e = g.log2_genus;
            }
        }
        for g in &grows {
            let _ = writeln!(
                gg,
                "{},{},{},{},{},{:.3},{},{},{}",
                n,
                g.r,
                g.s,
                g.t,
                g.genus.map(|x| x.to_string()).unwrap_or_else(|| "-".into()),
                g.log2_genus,
                g.delta,
                g.num_pairs,
                (best_per_delta[&g.delta] - g.log2_genus).abs() < 1e-9
            );
        }
        let best_i = rows
            .iter()
            .filter(|r| r.type_i)
            .min_by(|a, b| a.log2_genus.partial_cmp(&b.log2_genus).unwrap());
        let best_ii = rows
            .iter()
            .filter(|r| !r.type_i)
            .min_by(|a, b| a.log2_genus.partial_cmp(&b.log2_genus).unwrap());
        let best_g = grows
            .iter()
            .min_by(|a, b| a.log2_genus.partial_cmp(&b.log2_genus).unwrap());
        let half = n as f64 / 2.0;
        let fmt_g = |g: Option<u128>, lg: f64| {
            g.map(|x| x.to_string())
                .unwrap_or_else(|| format!("2^{lg:.1}"))
        };
        let _ = writeln!(
            summary,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{:.1},{:.1},{:.1},{:.1}",
            n,
            ord,
            cos.len(),
            best_i.map(|r| r.deg.to_string()).unwrap_or_default(),
            best_i
                .map(|r| fmt_g(r.genus, r.log2_genus))
                .unwrap_or_default(),
            best_i.map(|r| r.delta.to_string()).unwrap_or_default(),
            best_ii.map(|r| r.deg.to_string()).unwrap_or_default(),
            best_ii
                .map(|r| fmt_g(r.genus, r.log2_genus))
                .unwrap_or_default(),
            best_ii.map(|r| r.delta.to_string()).unwrap_or_default(),
            best_g.map(|r| r.r.to_string()).unwrap_or_default(),
            best_g.map(|r| r.s.to_string()).unwrap_or_default(),
            best_g
                .map(|r| fmt_g(r.genus, r.log2_genus))
                .unwrap_or_default(),
            best_g.map(|r| r.delta.to_string()).unwrap_or_default(),
            half,
            best_i.map(|r| r.delta as f64 - half).unwrap_or(f64::NAN),
            best_ii.map(|r| r.delta as f64 - half).unwrap_or(f64::NAN),
            best_g.map(|r| r.delta as f64 - half).unwrap_or(f64::NAN),
        );
        n += 2;
    }
    fs::write(out.join("binary_types.csv"), types).unwrap();
    fs::write(out.join("binary_gghs.csv"), gg).unwrap();
    fs::write(out.join("binary_summary.csv"), summary).unwrap();

    // A3: bit table for frozen N.
    let mut bits = String::from(
        "N,n,l,variant,deg_or_rs,m,genus,delta,rho_bits,owner_bits,walk_bits,margin_bits,log2_weak_curves,per_class_expectation,log2_genus_times_l\n",
    );
    for &nn in &[150u32, 155, 161, 165, 168, 185, 210, 300, 600] {
        let mut n = 3;
        while n <= nn {
            if nn % n == 0 {
                let l = nn / n;
                let rows = binary_rows(n);
                let grows = gghs_rows(n);
                let mut emit = |variant: &str,
                                deg_or_rs: String,
                                m: u32,
                                genus: Option<u128>,
                                lg: f64,
                                delta: u32,
                                deg_count: f64,
                                c: f64| {
                    let owner = (2.0 - 2.0 / 2f64.powf(lg)) * l as f64;
                    let walk = (delta as f64).min(n as f64 / 2.0) * l as f64;
                    let margin = (delta as f64 - n as f64 / 2.0) * l as f64;
                    let log2_weak = 1.0 + l as f64 * deg_count + log2_f(c);
                    let pce = per_class_expectation(c, deg_count, n, l);
                    let _ = writeln!(
                        bits,
                        "{},{},{},{},{},{},{},{},{:.1},{:.1},{:.1},{:.1},{:.1},{:.3e},{:.1}",
                        nn,
                        n,
                        l,
                        variant,
                        deg_or_rs,
                        m,
                        genus
                            .map(|g| g.to_string())
                            .unwrap_or_else(|| format!("2^{lg:.1}")),
                        delta,
                        nn as f64 / 2.0,
                        owner,
                        walk,
                        margin,
                        log2_weak,
                        pce,
                        lg + log2_f(l as f64)
                    );
                };
                for ty in [true, false] {
                    if let Some(r) = rows
                        .iter()
                        .filter(|r| r.type_i == ty)
                        .min_by(|a, b| a.log2_genus.partial_cmp(&b.log2_genus).unwrap())
                    {
                        emit(
                            if ty { "GHS-I" } else { "GHS-II" },
                            r.deg.to_string(),
                            r.m,
                            r.genus,
                            r.log2_genus,
                            r.delta,
                            r.deg as f64,
                            r.num_f as f64,
                        );
                    }
                }
                if let Some(g) = grows
                    .iter()
                    .min_by(|a, b| a.log2_genus.partial_cmp(&b.log2_genus).unwrap())
                {
                    emit(
                        "gGHS",
                        format!("{}+{}", g.r, g.s),
                        g.t,
                        g.genus,
                        g.log2_genus,
                        g.delta,
                        (g.r + g.s) as f64 - 1.0,
                        g.num_pairs as f64,
                    );
                }
                // generic curve: m = n (type II with F = x^n - 1), public for everyone
                let (gg_, lg_) = ghs_genus(n, false);
                emit("generic", n.to_string(), n, gg_, lg_, 0, n as f64, 1.0);
            }
            n += 2;
        }
    }
    fs::write(out.join("binary_bits.csv"), bits).unwrap();
    let _ = writeln!(log, "part A done");
}

// ===========================================================================
// Finite fields F_{p^L} with log, antilog and Zech tables. Elements are kept
// in log form: `e < nn` is `g^e`, `e == nn` is zero. Points of P^1 add `INF`.

const INF: u32 = u32::MAX;

struct Gf {
    p: u32,
    #[cfg_attr(not(test), allow(dead_code))]
    l: u32,
    nn: u32,
    #[cfg_attr(not(test), allow(dead_code))]
    exp: Vec<u32>,
    log: Vec<u32>,
    zech: Vec<u32>,
    half: u32,
}

// Polynomials over F_p, ascending coefficients.
fn poly_trim(a: &mut Vec<u32>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

fn poly_mulmod(a: &[u32], b: &[u32], f: &[u32], p: u32) -> Vec<u32> {
    let l = f.len() - 1;
    let mut prod = vec![0u64; a.len() + b.len()];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            prod[i + j] = (prod[i + j] + x as u64 * y as u64) % p as u64;
        }
    }
    // reduce modulo monic f of degree l
    for k in (l..prod.len()).rev() {
        let c = prod[k];
        if c == 0 {
            continue;
        }
        prod[k] = 0;
        for i in 0..l {
            prod[k - l + i] = (prod[k - l + i] + (p as u64 - f[i] as u64) * c) % p as u64;
        }
    }
    let mut out: Vec<u32> = prod.iter().take(l).map(|&v| v as u32).collect();
    poly_trim(&mut out);
    out
}

fn poly_powmod(a: &[u32], mut e: u64, f: &[u32], p: u32) -> Vec<u32> {
    let mut r = vec![1u32];
    let mut b = a.to_vec();
    while e > 0 {
        if e & 1 == 1 {
            r = poly_mulmod(&r, &b, f, p);
        }
        b = poly_mulmod(&b, &b, f, p);
        e >>= 1;
    }
    r
}

fn inv_mod_p(a: u32, p: u32) -> u32 {
    powmod(a as u64, p as u64 - 2, p as u64) as u32
}

fn poly_rem(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let mut r = a.to_vec();
    poly_trim(&mut r);
    let db = b.len() - 1;
    let inv = inv_mod_p(b[db], p);
    while r.len() > db {
        let k = r.len() - 1;
        let c = r[k] * inv % p;
        if c != 0 {
            for i in 0..=db {
                r[k - db + i] = (r[k - db + i] + p - (c * b[i]) % p) % p;
            }
        }
        r.pop();
        poly_trim(&mut r);
    }
    r
}

fn poly_gcd(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    poly_trim(&mut a);
    poly_trim(&mut b);
    while !b.is_empty() {
        let r = poly_rem(&a, &b, p);
        a = b;
        b = r;
    }
    if let Some(&lc) = a.last() {
        let inv = inv_mod_p(lc, p);
        for c in a.iter_mut() {
            *c = *c * inv % p;
        }
    }
    a
}

fn poly_sub(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let n = a.len().max(b.len());
    let mut out = vec![0u32; n];
    for (i, o) in out.iter_mut().enumerate() {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        *o = (x + p - y) % p;
    }
    poly_trim(&mut out);
    out
}

/// Ben-Or irreducibility test for monic `f` of degree `l >= 2`.
fn poly_irreducible(f: &[u32], p: u32) -> bool {
    let l = f.len() - 1;
    let x = vec![0u32, 1];
    let mut h = x.clone();
    for _ in 1..=l / 2 {
        h = poly_powmod(&h, p as u64, f, p);
        let d = poly_sub(&h, &x, p);
        let g = poly_gcd(f, &d, p);
        if g.len() != 1 {
            return false;
        }
    }
    true
}

impl Gf {
    fn new(p: u32, l: u32) -> Gf {
        assert!(p >= 3 && is_prime(p as u64));
        let size = (p as u64).pow(l) as u32;
        let nn = size - 1;
        let factors = prime_factors(nn as u64);
        let mut exp = vec![0u32; nn as usize];
        if l == 1 {
            let mut g = 2u32;
            loop {
                if factors
                    .iter()
                    .all(|&r| powmod(g as u64, nn as u64 / r, p as u64) != 1)
                {
                    break;
                }
                g += 1;
            }
            let mut cur = 1u64;
            for e in 0..nn {
                exp[e as usize] = cur as u32;
                cur = cur * g as u64 % p as u64;
            }
        } else {
            // irreducible monic polynomial of degree l
            let mut f = Vec::new();
            'search: for c in 0..size {
                let mut coeffs = vec![0u32; l as usize + 1];
                let mut v = c;
                for coeff in coeffs.iter_mut().take(l as usize) {
                    *coeff = v % p;
                    v /= p;
                }
                coeffs[l as usize] = 1;
                if coeffs[0] == 0 {
                    continue;
                }
                if poly_irreducible(&coeffs, p) {
                    f = coeffs;
                    break 'search;
                }
            }
            assert!(!f.is_empty());
            // primitive element: smallest non-constant digit index of full order
            let to_poly = |idx: u32| -> Vec<u32> {
                let mut v = idx;
                let mut c = Vec::new();
                for _ in 0..l {
                    c.push(v % p);
                    v /= p;
                }
                poly_trim(&mut c);
                c
            };
            let mut g = Vec::new();
            for idx in p..size {
                let cand = to_poly(idx);
                let ok = factors.iter().all(|&r| {
                    let pw = poly_powmod(&cand, nn as u64 / r, &f, p);
                    pw != vec![1u32]
                });
                if ok {
                    g = cand;
                    break;
                }
            }
            assert!(!g.is_empty());
            let mut cur = vec![1u32];
            for e in 0..nn {
                let mut idx = 0u32;
                let mut mulp = 1u32;
                for i in 0..l as usize {
                    idx += cur.get(i).copied().unwrap_or(0) * mulp;
                    mulp *= p;
                }
                exp[e as usize] = idx;
                cur = poly_mulmod(&cur, &g, &f, p);
            }
        }
        let mut log = vec![nn; size as usize];
        for e in 0..nn {
            log[exp[e as usize] as usize] = e;
        }
        // Zech: log(1 + g^d)
        let mut zech = vec![nn; nn as usize];
        for d in 0..nn {
            let idx = exp[d as usize];
            // add 1 to digit 0
            let d0 = idx % p;
            let idx1 = idx - d0 + (d0 + 1) % p;
            zech[d as usize] = log[idx1 as usize];
        }
        Gf {
            p,
            l,
            nn,
            exp,
            log,
            zech,
            half: nn / 2,
        }
    }

    #[inline]
    fn zero(&self) -> u32 {
        self.nn
    }
    #[inline]
    fn is_zero(&self, a: u32) -> bool {
        a == self.nn
    }
    #[inline]
    fn mul(&self, a: u32, b: u32) -> u32 {
        if a == self.nn || b == self.nn {
            return self.nn;
        }
        let s = a + b;
        if s >= self.nn {
            s - self.nn
        } else {
            s
        }
    }
    #[inline]
    fn inv(&self, a: u32) -> u32 {
        debug_assert!(a != self.nn);
        if a == 0 {
            0
        } else {
            self.nn - a
        }
    }
    #[inline]
    fn neg(&self, a: u32) -> u32 {
        if a == self.nn {
            return a;
        }
        let s = a + self.half;
        if s >= self.nn {
            s - self.nn
        } else {
            s
        }
    }
    #[inline]
    fn add(&self, a: u32, b: u32) -> u32 {
        if a == self.nn {
            return b;
        }
        if b == self.nn {
            return a;
        }
        let d = if b >= a { b - a } else { b + self.nn - a };
        let z = self.zech[d as usize];
        if z == self.nn {
            return self.nn;
        }
        let s = a + z;
        if s >= self.nn {
            s - self.nn
        } else {
            s
        }
    }
    #[inline]
    fn sub(&self, a: u32, b: u32) -> u32 {
        self.add(a, self.neg(b))
    }
    /// Constant `c mod p` in log form.
    fn const_int(&self, c: i64) -> u32 {
        let c = c.rem_euclid(self.p as i64) as u32;
        self.log[c as usize]
    }
    /// `nn / (p^d - 1)`: the exponent step of the subfield `F_{p^d}`.
    fn step(&self, d: u32) -> u32 {
        self.nn / ((self.p as u64).pow(d) as u32 - 1)
    }
    fn in_sub(&self, a: u32, d: u32) -> bool {
        a == self.nn || a.is_multiple_of(self.step(d))
    }
    /// Quadratic character of `a` in `F_{p^d}` (`a` must lie there).
    fn chi_sub(&self, a: u32, step: u32) -> i32 {
        if a == self.nn {
            0
        } else if (a / step).is_multiple_of(2) {
            1
        } else {
            -1
        }
    }
    /// Frobenius `a -> a^{p^k}` on log-form elements; `INF` is fixed.
    fn frob(&self, a: u32, pk_mod: u64) -> u32 {
        if a == INF || a == self.nn {
            a
        } else {
            ((a as u64 * pk_mod) % self.nn as u64) as u32
        }
    }
    fn pk_mod(&self, k: u32) -> u64 {
        powmod(self.p as u64, k as u64, self.nn as u64)
    }
    /// Digit vector (ascending) of a log-form element (used by the tests).
    #[cfg_attr(not(test), allow(dead_code))]
    fn digits(&self, a: u32) -> Vec<u32> {
        let mut idx = if a == self.nn {
            0
        } else {
            self.exp[a as usize]
        };
        let mut out = Vec::new();
        for _ in 0..self.l {
            out.push(idx % self.p);
            idx /= self.p;
        }
        out
    }
}

/// Points of `P^1`: `INF` or a log-form element. `sub` with `INF` is not used.
fn cross_ratio(f: &Gf, z: [u32; 4]) -> u32 {
    let [z1, z2, z3, z4] = z;
    let d = |a: u32, b: u32| f.sub(a, b);
    let (num, den) = if z1 == INF {
        (d(z4, z2), d(z3, z2))
    } else if z2 == INF {
        (d(z3, z1), d(z4, z1))
    } else if z3 == INF {
        (d(z4, z2), d(z4, z1))
    } else if z4 == INF {
        (d(z3, z1), d(z3, z2))
    } else {
        (f.mul(d(z3, z1), d(z4, z2)), f.mul(d(z3, z2), d(z4, z1)))
    };
    f.mul(num, f.inv(den))
}

/// `j = 256 (l^2 - l + 1)^3 / (l^2 (l - 1)^2)` for the cross-ratio `l`.
fn j_from_lambda(f: &Gf, lam: u32) -> u32 {
    let one = 0u32;
    let l2 = f.mul(lam, lam);
    let num_base = f.add(f.sub(l2, lam), one);
    let num = f.mul(f.const_int(256), f.mul(num_base, f.mul(num_base, num_base)));
    let lm1 = f.sub(lam, one);
    let den = f.mul(l2, f.mul(lm1, lm1));
    f.mul(num, f.inv(den))
}

/// Trace of a curve with invariant `j` over the subfield `F_{p^n}` (step
/// `step_k`), from exact point counting of one model with that `j`.
fn trace_of_j(f: &Gf, j: u32, n: u32, step_k: u32) -> i64 {
    let p = f.p;
    let qn = (p as u64).pow(n);
    let (a2, a4, a6) = model_for_j(f, j);
    let mut sum: i64 = 0;
    let mut e = 0u32;
    // x = 0
    sum += f.chi_sub(a6, step_k) as i64;
    for _ in 1..qn {
        let x = e;
        let v = f.add(f.mul(x, f.add(f.mul(x, f.add(x, a2)), a4)), a6);
        sum += f.chi_sub(v, step_k) as i64;
        e += step_k;
        if e >= f.nn {
            e -= f.nn;
        }
    }
    -sum
}

/// A model `y^2 = x^3 + a2 x^2 + a4 x + a6` with invariant `j`.
fn model_for_j(f: &Gf, j: u32) -> (u32, u32, u32) {
    let zero = f.zero();
    let one = 0u32;
    if f.p == 3 {
        // ordinary: y^2 = x^3 + x^2 - 1/j; j = 0: y^2 = x^3 - x
        if f.is_zero(j) {
            return (zero, f.neg(one), zero);
        }
        return (one, zero, f.neg(f.inv(j)));
    }
    let c1728 = f.const_int(1728);
    if f.is_zero(j) {
        return (zero, zero, one);
    }
    if j == c1728 {
        return (zero, one, zero);
    }
    let d = f.sub(c1728, j);
    let a = f.mul(f.const_int(3), f.mul(j, d));
    let b = f.mul(f.const_int(2), f.mul(j, f.mul(d, d)));
    (zero, a, b)
}

// ===========================================================================
// Part B: odd characteristic, exact weak-curve and class counts

struct OddCfg {
    n: u32,
    sizes: &'static [u32],
    l: u32,
    r_max: u32,
    g_max: u32,
}

const ODD_CFG: [OddCfg; 6] = [
    OddCfg {
        n: 2,
        sizes: &[1, 2, 3],
        l: 6,
        r_max: 6,
        g_max: 3,
    },
    OddCfg {
        n: 3,
        sizes: &[1, 2, 3, 6],
        l: 6,
        r_max: 6,
        g_max: 5,
    },
    OddCfg {
        n: 4,
        sizes: &[1, 2, 4],
        l: 4,
        r_max: 6,
        g_max: 5,
    },
    OddCfg {
        n: 5,
        sizes: &[1, 5],
        l: 5,
        r_max: 7,
        g_max: 9,
    },
    OddCfg {
        n: 6,
        sizes: &[1, 2, 3, 6],
        l: 6,
        r_max: 6,
        g_max: 9,
    },
    OddCfg {
        n: 7,
        sizes: &[1, 7],
        l: 7,
        r_max: 7,
        g_max: 7,
    },
];
const ODD_Q: [u32; 5] = [3, 5, 7, 11, 13];
const ALLJ_BUDGET: f64 = 2.0e11;

fn rank_f2(vecs: &[u32]) -> u32 {
    let mut basis: Vec<u32> = Vec::new();
    for &v in vecs {
        let mut x = v;
        for &b in &basis {
            let hb = 1u32 << (31 - b.leading_zeros());
            if x & hb != 0 {
                x ^= b;
            }
        }
        if x != 0 {
            basis.push(x);
            basis.sort_by(|a, b| b.cmp(a));
        }
    }
    basis.len() as u32
}

/// Sigma-orbits of the allowed sizes inside `A = F_{q^L}` (as log-form points).
fn orbits_by_size(f: &Gf, sizes: &[u32]) -> BTreeMap<u32, Vec<Vec<u32>>> {
    let mut out = BTreeMap::new();
    let pm = f.pk_mod(1);
    for &d in sizes {
        let mut list: Vec<Vec<u32>> = Vec::new();
        if d == 1 {
            list.push(vec![INF]);
            list.push(vec![f.zero()]);
            for e in (0..f.nn).step_by(f.step(1) as usize) {
                list.push(vec![e]);
            }
        } else {
            let step = f.step(d);
            let proper: Vec<u32> = (1..d).filter(|&dd| d % dd == 0).collect();
            let mut e = 0u32;
            while e < f.nn {
                if proper.iter().all(|&dd| !e.is_multiple_of(f.step(dd))) {
                    let mut orb = vec![e];
                    let mut x = e;
                    let mut canonical = true;
                    for _ in 1..d {
                        x = f.frob(x, pm);
                        if x < e {
                            canonical = false;
                            break;
                        }
                        orb.push(x);
                    }
                    if canonical {
                        list.push(orb);
                    }
                }
                e += step;
            }
        }
        out.insert(d, list);
    }
    out
}

fn size_multisets(sizes: &[u32], r_min: u32, r_max: u32) -> Vec<Vec<u32>> {
    let mut sorted = sizes.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a));
    let mut out = Vec::new();
    fn rec(
        sorted: &[u32],
        start: usize,
        cur: &mut Vec<u32>,
        sum: u32,
        r_min: u32,
        r_max: u32,
        out: &mut Vec<Vec<u32>>,
    ) {
        if sum >= r_min && sum <= r_max {
            out.push(cur.clone());
        }
        for i in start..sorted.len() {
            if sum + sorted[i] <= r_max {
                cur.push(sorted[i]);
                rec(sorted, i, cur, sum + sorted[i], r_min, r_max, out);
                cur.pop();
            }
        }
    }
    rec(&sorted, 0, &mut Vec::new(), 0, r_min, r_max, &mut out);
    out
}

#[derive(Default, Clone)]
struct TypeAcc {
    instances: u64,
    weak_j: HashSet<u32>,
}

type TypeKey = (String, u32, u32, u32); // (orbit sizes, r, m, g)

/// All `k`-combinations of `0..len` as index vectors (first index limited
/// to `first` when given).
fn combinations(len: usize, k: usize, first: Option<std::ops::Range<usize>>) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    if k == 0 {
        out.push(Vec::new());
        return out;
    }
    if k > len {
        return out;
    }
    let mut c: Vec<usize> = (0..k).collect();
    loop {
        let ok = match &first {
            Some(r) => r.contains(&c[0]),
            None => true,
        };
        if ok {
            out.push(c.clone());
        }
        // advance
        let mut i = k;
        loop {
            if i == 0 {
                return out;
            }
            i -= 1;
            if c[i] < len - k + i {
                c[i] += 1;
                for j in i + 1..k {
                    c[j] = c[j - 1] + 1;
                }
                break;
            }
        }
        if let Some(r) = &first {
            if c[0] >= r.end {
                return out;
            }
        }
    }
}

/// Enumerate all `S` for one size multiset, restricted to a chunk of the
/// choices for the first (largest) size, and collect weak `B` by type.
fn enumerate_chunk(
    f: &Gf,
    n: u32,
    g_max: u32,
    sizes: &[u32],
    orbits: &BTreeMap<u32, Vec<Vec<u32>>>,
    first_range: std::ops::Range<usize>,
    acc: &mut BTreeMap<TypeKey, TypeAcc>,
) {
    let mut groups: Vec<(u32, usize)> = Vec::new();
    for &s in sizes {
        if let Some(last) = groups.last_mut() {
            if last.0 == s {
                last.1 += 1;
                continue;
            }
        }
        groups.push((s, 1));
    }
    let sizes_str = sizes
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("+");
    let r: u32 = sizes.iter().sum();
    let lists: Vec<&Vec<Vec<u32>>> = groups.iter().map(|(s, _)| &orbits[s]).collect();
    let combos: Vec<Vec<Vec<usize>>> = groups
        .iter()
        .enumerate()
        .map(|(gi, (_, cnt))| {
            combinations(
                lists[gi].len(),
                *cnt,
                if gi == 0 {
                    Some(first_range.clone())
                } else {
                    None
                },
            )
        })
        .collect();
    if combos.iter().any(|c| c.is_empty()) {
        return;
    }
    let full: u32 = (1u32 << r) - 1;
    let mut points: Vec<u32> = Vec::with_capacity(r as usize);
    let mut perm: Vec<usize> = Vec::with_capacity(r as usize);
    let mut idx: Vec<usize> = vec![0; combos.len()];
    loop {
        points.clear();
        perm.clear();
        for (gi, &ci) in idx.iter().enumerate() {
            for &oi in &combos[gi][ci] {
                let orb = &lists[gi][oi];
                let base = points.len();
                for (k, &pt) in orb.iter().enumerate() {
                    points.push(pt);
                    perm.push(base + (k + 1) % orb.len());
                }
            }
        }
        debug_assert_eq!(points.len(), r as usize);
        let apply = |mask: u32| -> u32 {
            let mut out = 0u32;
            for (i, &target) in perm.iter().enumerate().take(r as usize) {
                if mask >> i & 1 == 1 {
                    out |= 1 << target;
                }
            }
            out
        };
        let mut mask: u32 = 0b1111;
        while mask <= full {
            if mask.count_ones() == 4 {
                let mut trans = [0u32; 8];
                trans[0] = mask;
                let mut union = mask;
                let mut period = 0u32;
                for i in 1..=n as usize {
                    trans[i] = apply(trans[i - 1]);
                    if i < n as usize {
                        union |= trans[i];
                    }
                    if period == 0 && trans[i] == mask {
                        period = i as u32;
                    }
                }
                if trans[n as usize] == mask && period == n && union == full {
                    let m = rank_f2(&trans[..n as usize]);
                    let g = 1 + (1u32 << (m - 2)) * (r - 4);
                    if g <= g_max {
                        let mut b = [0u32; 4];
                        let mut k = 0;
                        for (i, &pt) in points.iter().enumerate().take(r as usize) {
                            if mask >> i & 1 == 1 {
                                b[k] = pt;
                                k += 1;
                            }
                        }
                        let lam = cross_ratio(f, b);
                        let j = j_from_lambda(f, lam);
                        let key = (sizes_str.clone(), r, m, g);
                        let e = acc.entry(key).or_default();
                        e.instances += 1;
                        e.weak_j.insert(j);
                    }
                }
            }
            mask += 1;
        }
        // next tuple of combination indices
        let mut gi = combos.len();
        loop {
            if gi == 0 {
                return;
            }
            gi -= 1;
            idx[gi] += 1;
            if idx[gi] < combos[gi].len() {
                break;
            }
            idx[gi] = 0;
        }
    }
}

struct OddResult {
    rows: String,
    class_rows: String,
    checks: String,
}

fn part_b_one(q: u32, cfg: &OddCfg, log: &mut String) -> OddResult {
    let t0 = Instant::now();
    let n = cfg.n;
    let f = Gf::new(q, cfg.l);
    let step_k = f.step(n);
    let qn = (q as u64).pow(n);
    let orbits = orbits_by_size(&f, cfg.sizes);
    let multisets = size_multisets(cfg.sizes, 4, cfg.r_max);
    let nthreads = 4usize;
    let mut acc: BTreeMap<TypeKey, TypeAcc> = BTreeMap::new();
    for ms in &multisets {
        let first_len = orbits[&ms[0]].len();
        let chunk = first_len.div_ceil(nthreads).max(1);
        let partial: Vec<BTreeMap<TypeKey, TypeAcc>> = std::thread::scope(|sc| {
            let hs: Vec<_> = (0..nthreads)
                .map(|ti| {
                    let f = &f;
                    let orbits = &orbits;
                    let ms = ms.clone();
                    sc.spawn(move || {
                        let mut a = BTreeMap::new();
                        let range = (ti * chunk)..((ti + 1) * chunk).min(first_len);
                        if range.start < range.end {
                            enumerate_chunk(f, n, cfg.g_max, &ms, orbits, range, &mut a);
                        }
                        a
                    })
                })
                .collect();
            hs.into_iter().map(|h| h.join().unwrap()).collect()
        });
        for m in partial {
            for (k, v) in m {
                let e = acc.entry(k).or_default();
                e.instances += v.instances;
                e.weak_j.extend(v.weak_j);
            }
        }
    }
    let t_enum = t0.elapsed().as_secs_f64();
    let mut checks = String::new();
    // check: every weak j in K
    let mut bad_j = 0u64;
    for v in acc.values() {
        for &j in &v.weak_j {
            if !f.in_sub(j, n) {
                bad_j += 1;
            }
        }
    }
    let _ = writeln!(checks, "q={q} n={n} weak_j_outside_K={bad_j}");
    // traces of weak j (and of all j when within budget)
    let weak_all: BTreeSet<u32> = acc
        .values()
        .flat_map(|v| v.weak_j.iter().copied())
        .collect();
    let all_j_budget = (qn as f64) * (qn as f64) <= ALLJ_BUDGET;
    let t1 = Instant::now();
    let j_list: Vec<u32> = if all_j_budget {
        let mut v = vec![f.zero()];
        v.extend((0..f.nn).step_by(step_k as usize));
        v
    } else {
        weak_all.iter().copied().collect()
    };
    let traces: Vec<i64> = {
        let chunk = j_list.len().div_ceil(nthreads).max(1);
        std::thread::scope(|sc| {
            let hs: Vec<_> = j_list
                .chunks(chunk)
                .map(|ch| {
                    let f = &f;
                    sc.spawn(move || {
                        ch.iter()
                            .map(|&j| trace_of_j(f, j, n, step_k))
                            .collect::<Vec<i64>>()
                    })
                })
                .collect();
            hs.into_iter().flat_map(|h| h.join().unwrap()).collect()
        })
    };
    let trace_of: HashMap<u32, i64> = j_list.iter().copied().zip(traces.iter().copied()).collect();
    let t_tr = t1.elapsed().as_secs_f64();
    let hasse = 2.0 * (qn as f64).sqrt();
    let mut bad_hasse = 0u64;
    for &t in trace_of.values() {
        if (t as f64).abs() > hasse + 1e-9 {
            bad_hasse += 1;
        }
    }
    let _ = writeln!(
        checks,
        "q={q} n={n} all_j_table={all_j_budget} j_listed={} hasse_violations={bad_hasse}",
        j_list.len()
    );
    if all_j_budget {
        let _ = writeln!(
            checks,
            "q={q} n={n} check6_num_j={} expected={}",
            j_list.len(),
            qn
        );
    }
    // class statistics
    let mut cnt_all: HashMap<i64, u64> = HashMap::new();
    if all_j_budget {
        for &t in &traces {
            *cnt_all.entry(t).or_insert(0) += 1;
        }
    }
    let n_all = |t: i64| -> u64 {
        cnt_all.get(&t).copied().unwrap_or(0) + cnt_all.get(&-t).copied().unwrap_or(0)
    };
    let mut rows = String::new();
    let mut class_rows = String::new();
    let c1728 = f.const_int(1728);
    for (key, v) in &acc {
        let (sizes, r, m, g) = key;
        let wj = v.weak_j.len() as f64;
        let pred = (*r as i64 - 3) as f64;
        let ratio = wj / (q as f64).powf(pred);
        // check 5 for n = 3, (m, r) = (3, 5)
        let mut c5_bad = 0u64;
        let mut c5_n = 0u64;
        if n == 3 && *m == 3 && *r == 5 {
            for &j in &v.weak_j {
                if f.is_zero(j) || j == c1728 {
                    continue;
                }
                let t = trace_of[&j];
                c5_n += 1;
                if (qn as i64 + 1 - t) % 4 != 0 || (qn as i64 + 1 + t) % 4 != 0 {
                    c5_bad += 1;
                }
            }
            let _ = writeln!(checks, "q={q} n={n} type={sizes} m={m} r={r} check5_curves={c5_n} check5_violations={c5_bad}");
        }
        let _ = writeln!(
            rows,
            "{},{},{},{},{},{},{},{},{},{:.4},{},{:.4},{}",
            q,
            n,
            cfg.l,
            sizes,
            r,
            m,
            g,
            v.instances,
            v.weak_j.len(),
            log2_f(wj.max(1.0)) / log2_f(q as f64),
            pred,
            ratio,
            all_j_budget
        );
        if all_j_budget {
            let mut wcnt: HashMap<i64, u64> = HashMap::new();
            for &j in &v.weak_j {
                *wcnt.entry(trace_of[&j]).or_insert(0) += 1;
            }
            let n_weak = |t: i64| -> u64 {
                wcnt.get(&t).copied().unwrap_or(0) + wcnt.get(&-t).copied().unwrap_or(0)
            };
            let traces_all: BTreeSet<i64> = cnt_all.keys().flat_map(|&t| [t, -t]).collect();
            let total_classes = traces_all.len();
            let mut covered = 0usize;
            let mut curves_cov = 0u64;
            let mut weak_sum = 0u64;
            let mut weak_max = 0u64;
            let mut all_in_cov = 0u64;
            let total_curves: u64 = traces_all.iter().map(|&t| n_all(t)).sum();
            for &t in &traces_all {
                let w = n_weak(t);
                if w > 0 {
                    covered += 1;
                    curves_cov += n_all(t);
                    weak_sum += w;
                    weak_max = weak_max.max(w);
                    all_in_cov += n_all(t);
                }
            }
            let _ = writeln!(
                class_rows,
                "{},{},{},{},{},{},{},{},{:.4},{:.3},{},{:.2}",
                q,
                n,
                sizes,
                r,
                m,
                g,
                total_classes,
                covered,
                curves_cov as f64 / total_curves as f64,
                if covered > 0 {
                    weak_sum as f64 / covered as f64
                } else {
                    0.0
                },
                weak_max,
                if weak_sum > 0 {
                    all_in_cov as f64 / weak_sum as f64
                } else {
                    0.0
                }
            );
        }
    }
    let _ = writeln!(
        log,
        "part B q={q} n={n} L={} types={} enum={t_enum:.1}s traces={t_tr:.1}s all_j={all_j_budget}",
        cfg.l,
        acc.len()
    );
    OddResult {
        rows,
        class_rows,
        checks,
    }
}

fn part_b(out: &Path, log: &mut String) {
    let mut rows = String::from("q,n,L,orbit_sizes,r,m,genus,instances_B,weak_j,log_q_weak_j,predicted_exponent,ratio_weak_j_over_q_pow,all_j_table\n");
    let mut class_rows = String::from("q,n,orbit_sizes,r,m,genus,total_classes,covered_classes,share_curves_in_covered,mean_weak_per_covered_class,max_weak_per_class,mean_walk_length\n");
    let mut checks = String::new();
    let qmax: u32 = std::env::var("TVFT_QMAX")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(u32::MAX);
    for cfg in &ODD_CFG {
        for &q in &ODD_Q {
            if (cfg.n == 7 && q >= 11) || q > qmax {
                continue;
            }
            let r = part_b_one(q, cfg, log);
            rows.push_str(&r.rows);
            class_rows.push_str(&r.class_rows);
            checks.push_str(&r.checks);
            fs::write(out.join("oddchar_types.csv"), &rows).unwrap();
            fs::write(out.join("oddchar_classes.csv"), &class_rows).unwrap();
            fs::write(out.join("oddchar_checks.txt"), &checks).unwrap();
            eprintln!("{}", log.lines().last().unwrap_or(""));
        }
    }
}

// ===========================================================================
// Part C: Weil-restriction-shaped genus-3 hyperelliptic Jacobians over F_q

const CENSUS_Q: [u32; 5] = [3, 5, 7, 11, 13];

/// A normalised model `y^2 = c f(x)`: `deg` 7 or 8, `c_nu` says `c = nu`,
/// `coeffs[i]` the coefficient of `x^i` (ascending, `coeffs[deg] = 1`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Model {
    deg: u8,
    c_nu: bool,
    coeffs: [u8; 9],
}

struct Census {
    q: u32,
    nu: u32,
    /// quadratic character mod p as a table over 0..p
    qr: Vec<i8>,
    f2: Gf,
    f3: Gf,
    f6: Gf,
}

fn least_non_residue(p: u32) -> u32 {
    (2..p)
        .find(|&c| powmod(c as u64, (p as u64 - 1) / 2, p as u64) != 1)
        .unwrap()
}

/// Horner evaluation of an integer-coefficient polynomial mod p.
fn eval_mod_p(f: &[u32], x: u32, p: u32) -> u32 {
    let mut v = 0u64;
    for &c in f.iter().rev() {
        v = (v * x as u64 + c as u64) % p as u64;
    }
    v as u32
}

fn poly_derivative(f: &[u32], p: u32) -> Vec<u32> {
    let mut d: Vec<u32> = f
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, &c)| (c * (i as u32 % p)) % p)
        .collect();
    poly_trim(&mut d);
    d
}

fn is_squarefree(f: &[u32], p: u32) -> bool {
    let d = poly_derivative(f, p);
    if d.is_empty() {
        return false;
    }
    poly_gcd(f, &d, p).len() == 1
}

/// `G(X, Z) = F(aX + bZ, cX + dZ)` over F_p.
fn subst_form(form: &[u32; 9], a: u32, b: u32, c: u32, d: u32, p: u32) -> [u32; 9] {
    let p64 = p as u64;
    // powers of (aX + bZ) and (cX + dZ) as coefficient vectors in X-degree
    let mut pow1: Vec<Vec<u64>> = vec![vec![1]];
    let mut pow2: Vec<Vec<u64>> = vec![vec![1]];
    for k in 1..=8 {
        let prev1 = &pow1[k - 1];
        let mut nx = vec![0u64; k + 1];
        for (i, &v) in prev1.iter().enumerate() {
            nx[i + 1] = (nx[i + 1] + v * a as u64) % p64;
            nx[i] = (nx[i] + v * b as u64) % p64;
        }
        pow1.push(nx);
        let prev2 = &pow2[k - 1];
        let mut nz = vec![0u64; k + 1];
        for (i, &v) in prev2.iter().enumerate() {
            nz[i + 1] = (nz[i + 1] + v * c as u64) % p64;
            nz[i] = (nz[i] + v * d as u64) % p64;
        }
        pow2.push(nz);
    }
    let mut out = [0u64; 9];
    for i in 0..=8 {
        if form[i] == 0 {
            continue;
        }
        let x = &pow1[i];
        let z = &pow2[8 - i];
        for (ii, &xv) in x.iter().enumerate() {
            if xv == 0 {
                continue;
            }
            for (jj, &zv) in z.iter().enumerate() {
                out[ii + jj] = (out[ii + jj] + form[i] as u64 * xv % p64 * zv) % p64;
            }
        }
    }
    let mut o = [0u32; 9];
    for i in 0..9 {
        o[i] = out[i] as u32;
    }
    o
}

impl Census {
    fn new(q: u32) -> Census {
        let nu = least_non_residue(q);
        let mut qr = vec![0i8; q as usize];
        for v in 1..q {
            qr[v as usize] = if powmod(v as u64, (q as u64 - 1) / 2, q as u64) == 1 {
                1
            } else {
                -1
            };
        }
        Census {
            q,
            nu,
            qr,
            f2: Gf::new(q, 2),
            f3: Gf::new(q, 3),
            f6: Gf::new(q, 6),
        }
    }

    /// Normalise a squarefree octic form to the model family.
    fn normalize(&self, form: &[u32; 9]) -> Model {
        let p = self.q;
        let mut g = *form;
        let deg: usize;
        if g[8] != 0 {
            deg = 8;
            // kill X^7: X -> X + bZ with b = -g7 / (8 g8)
            if g[7] != 0 {
                let b = (p - g[7] * inv_mod_p(8 % p, p) % p * inv_mod_p(g[8], p) % p) % p;
                g = subst_form(&g, 1, b, 0, 1, p);
            }
        } else {
            assert!(g[7] != 0, "double root at infinity");
            deg = 7;
            if p != 7 && g[6] != 0 {
                let b = (p - g[6] * inv_mod_p(7 % p, p) % p * inv_mod_p(g[7], p) % p) % p;
                g = subst_form(&g, 1, b, 0, 1, p);
            }
        }
        // y^2 = lead * f_monic: the twist class is the square class of lead
        let lead = g[deg];
        let c_nu = self.qr[lead as usize] == -1;
        let scale = inv_mod_p(lead, p);
        let mut coeffs = [0u8; 9];
        for i in 0..9 {
            coeffs[i] = (g[i] * scale % p) as u8;
        }
        assert_eq!(coeffs[deg], 1);
        Model {
            deg: deg as u8,
            c_nu,
            coeffs,
        }
    }

    fn model_form(&self, m: &Model) -> [u32; 9] {
        let c = if m.c_nu { self.nu } else { 1 };
        let mut form = [0u32; 9];
        for (f, &co) in form.iter_mut().zip(m.coeffs.iter()) {
            *f = co as u32 * c % self.q;
        }
        form
    }

    /// Neighbours under the moves that generate PGL_2(F_q) modulo translations.
    fn neighbours(&self, m: &Model) -> Vec<Model> {
        let p = self.q;
        let form = self.model_form(m);
        let mut out = Vec::new();
        // scaling by a generator s of F_p^*
        let s = (2..p)
            .find(|&s| mult_order(s as u64, p as u64) == (p - 1) as u64)
            .unwrap();
        out.push(self.normalize(&subst_form(&form, s, 0, 0, 1, p)));
        // translate by b then invert: X -> Z, Z -> X + bZ  (x -> 1/(x+b))
        for b in 0..p {
            out.push(self.normalize(&subst_form(&form, 0, 1, 1, b, p)));
        }
        if p == 7 && m.deg == 7 {
            out.push(self.normalize(&subst_form(&form, 1, 1, 0, 1, p)));
        }
        out
    }

    /// Point count of `y^2 = c f(x)` over `F_{q^k}` using the table field `g`.
    fn count_ext(&self, g: &Gf, coeffs: &[u32], deg: usize, c_nu: bool, k: u32) -> i64 {
        let c = if c_nu { g.const_int(self.nu as i64) } else { 0 };
        let emb: Vec<u32> = coeffs[..=deg]
            .iter()
            .map(|&v| g.const_int(v as i64))
            .collect();
        let inf: i64 = if deg == 7 {
            1
        } else if c_nu && k % 2 == 1 {
            0
        } else {
            2
        };
        let mut count = inf;
        // x = 0
        let v0 = g.mul(c, emb[0]);
        count += 1 + g.chi_sub(v0, 1) as i64;
        for x in 0..g.nn {
            let mut v = emb[deg];
            for i in (0..deg).rev() {
                v = g.add(g.mul(v, x), emb[i]);
            }
            v = g.mul(c, v);
            count += 1 + g.chi_sub(v, 1) as i64;
        }
        count
    }

    fn count_base(&self, coeffs: &[u32], deg: usize, c_nu: bool) -> i64 {
        let p = self.q;
        let c = if c_nu { self.nu } else { 1 };
        let mut count: i64 = if deg == 7 {
            1
        } else if c_nu {
            0
        } else {
            2
        };
        for x in 0..p {
            let v = eval_mod_p(&coeffs[..=deg], x, p) * c % p;
            count += 1 + self.qr[v as usize] as i64;
        }
        count
    }

    /// Res-shape test; returns `t` when `L = 1 - tT^3 + q^3T^6`.
    fn res_shaped(&self, coeffs: &[u32], deg: usize, c_nu: bool) -> Option<(i64, i64)> {
        let q = self.q as i64;
        if self.count_base(coeffs, deg, c_nu) != q + 1 {
            return None;
        }
        if self.count_ext(&self.f2, coeffs, deg, c_nu, 2) != q * q + 1 {
            return None;
        }
        let n3 = self.count_ext(&self.f3, coeffs, deg, c_nu, 3);
        let num = q * q * q + 1 - n3;
        assert!(num % 3 == 0, "L-polynomial inconsistency: N3 = {n3}");
        Some((num / 3, n3))
    }

    /// Distinct-degree factorisation pattern of `f` over F_q (degrees with
    /// multiplicity, ascending), plus `1` for the point at infinity if deg 7.
    fn cycle_type(&self, coeffs: &[u32], deg: usize) -> Vec<u32> {
        let p = self.q;
        let mut f: Vec<u32> = coeffs[..=deg].to_vec();
        poly_trim(&mut f);
        let mut pattern = Vec::new();
        if deg == 7 {
            pattern.push(1);
        }
        let x = vec![0u32, 1];
        let mut h = x.clone();
        let mut rem = f.clone();
        let mut i = 1u32;
        while rem.len() > 1 && i <= 4 {
            h = poly_powmod(&h, p as u64, &f, p);
            let g = poly_gcd(&rem, &poly_sub(&h, &x, p), p);
            let dg = (g.len() - 1) as u32;
            for _ in 0..dg / i {
                pattern.push(i);
            }
            if dg > 0 {
                // divide rem by g
                rem = poly_div_exact(&rem, &g, p);
            }
            i += 1;
        }
        if rem.len() > 1 {
            pattern.push((rem.len() - 1) as u32);
        }
        pattern.sort_unstable();
        pattern
    }

    /// Weierstrass points as points of `P^1(F_{q^6})`, if all lie there.
    fn weierstrass_points(&self, coeffs: &[u32], deg: usize, pattern: &[u32]) -> Option<Vec<u32>> {
        if pattern.iter().any(|&d| !matches!(d, 1 | 2 | 3 | 6)) {
            return None;
        }
        let g = &self.f6;
        let emb: Vec<u32> = coeffs[..=deg]
            .iter()
            .map(|&v| g.const_int(v as i64))
            .collect();
        let mut w: Vec<u32> = Vec::new();
        if deg == 7 {
            w.push(INF);
        }
        let mut seed: u64 = 0x9e37_79b9_7f4a_7c15 ^ (self.q as u64) << 32;
        for &c in coeffs[..=deg].iter() {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(c as u64 + 1442695040888963407);
        }
        gf_roots(g, &emb, &mut seed, &mut w);
        w.sort_unstable();
        w.dedup();
        assert_eq!(
            w.len(),
            8,
            "expected 8 Weierstrass points, found {}",
            w.len()
        );
        Some(w)
    }
}

// --- polynomials over a table field (log-form coefficients, ascending) ---

fn gfp_trim(g: &Gf, a: &mut Vec<u32>) {
    while a.last().is_some_and(|&c| g.is_zero(c)) {
        a.pop();
    }
}

fn gfp_rem(g: &Gf, a: &[u32], m: &[u32]) -> Vec<u32> {
    let mut r = a.to_vec();
    gfp_trim(g, &mut r);
    let dm = m.len() - 1;
    let inv = g.inv(m[dm]);
    while r.len() > dm {
        let k = r.len() - 1;
        let c = g.mul(r[k], inv);
        if !g.is_zero(c) {
            for i in 0..=dm {
                r[k - dm + i] = g.sub(r[k - dm + i], g.mul(c, m[i]));
            }
        }
        r.pop();
        gfp_trim(g, &mut r);
    }
    r
}

fn gfp_mulmod(g: &Gf, a: &[u32], b: &[u32], m: &[u32]) -> Vec<u32> {
    let mut prod = vec![g.zero(); a.len() + b.len()];
    for (i, &x) in a.iter().enumerate() {
        if g.is_zero(x) {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            prod[i + j] = g.add(prod[i + j], g.mul(x, y));
        }
    }
    gfp_rem(g, &prod, m)
}

fn gfp_powmod(g: &Gf, a: &[u32], mut e: u64, m: &[u32]) -> Vec<u32> {
    let mut r = vec![0u32]; // one
    let mut b = gfp_rem(g, a, m);
    while e > 0 {
        if e & 1 == 1 {
            r = gfp_mulmod(g, &r, &b, m);
        }
        b = gfp_mulmod(g, &b, &b, m);
        e >>= 1;
    }
    r
}

fn gfp_gcd(g: &Gf, a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut a = a.to_vec();
    let mut b = b.to_vec();
    gfp_trim(g, &mut a);
    gfp_trim(g, &mut b);
    while !b.is_empty() {
        let r = gfp_rem(g, &a, &b);
        a = b;
        b = r;
    }
    if let Some(&lc) = a.last() {
        let inv = g.inv(lc);
        for c in a.iter_mut() {
            *c = g.mul(*c, inv);
        }
    }
    a
}

fn gfp_div_exact(g: &Gf, a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut r = a.to_vec();
    gfp_trim(g, &mut r);
    let db = b.len() - 1;
    let mut q = vec![g.zero(); r.len() - db];
    let inv = g.inv(b[db]);
    for k in (db..r.len()).rev() {
        let c = g.mul(r[k], inv);
        q[k - db] = c;
        if !g.is_zero(c) {
            for i in 0..=db {
                r[k - db + i] = g.sub(r[k - db + i], g.mul(c, b[i]));
            }
        }
    }
    gfp_trim(g, &mut q);
    q
}

/// Roots in the table field of a polynomial that splits into distinct
/// linear factors there, by repeated gcd splitting with
/// `(x + a)^{(|F|-1)/2} - 1` (Cantor–Zassenhaus, odd characteristic).
fn gf_roots(g: &Gf, f: &[u32], seed: &mut u64, out: &mut Vec<u32>) {
    let mut f = f.to_vec();
    gfp_trim(g, &mut f);
    if f.len() <= 1 {
        return;
    }
    if f.len() == 2 {
        out.push(g.neg(g.mul(f[0], g.inv(f[1]))));
        return;
    }
    let half = (g.nn / 2) as u64;
    loop {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        let a = (*seed % g.nn as u64) as u32; // log form, nonzero element
        let base = vec![a, 0u32]; // x + a
        let mut h = gfp_powmod(g, &base, half, &f);
        // h - 1
        if h.is_empty() {
            h.push(g.zero());
        }
        h[0] = g.sub(h[0], 0);
        gfp_trim(g, &mut h);
        if h.is_empty() {
            continue;
        }
        let d = gfp_gcd(g, &f, &h);
        if d.len() > 1 && d.len() < f.len() {
            let q = gfp_div_exact(g, &f, &d);
            gf_roots(g, &d, seed, out);
            gf_roots(g, &q, seed, out);
            return;
        }
    }
}

fn poly_div_exact(a: &[u32], b: &[u32], p: u32) -> Vec<u32> {
    let mut r = a.to_vec();
    let db = b.len() - 1;
    let mut qout = vec![0u32; r.len() - db];
    let inv = inv_mod_p(b[db], p);
    for k in (db..r.len()).rev() {
        let c = r[k] * inv % p;
        qout[k - db] = c;
        if c != 0 {
            for i in 0..=db {
                r[k - db + i] = (r[k - db + i] + p - c * b[i] % p) % p;
            }
        }
    }
    poly_trim(&mut qout);
    qout
}

// --- Möbius transformations over F_{q^6} ---------------------------------

type Mat = [u32; 4]; // (a, b, c, d) log-form, row-major

fn mat_norm(g: &Gf, m: Mat) -> Mat {
    let lead = m.iter().copied().find(|&e| e != g.nn).expect("zero matrix");
    let inv = g.inv(lead);
    [
        g.mul(m[0], inv),
        g.mul(m[1], inv),
        g.mul(m[2], inv),
        g.mul(m[3], inv),
    ]
}

fn mat_mul(g: &Gf, x: Mat, y: Mat) -> Mat {
    [
        g.add(g.mul(x[0], y[0]), g.mul(x[1], y[2])),
        g.add(g.mul(x[0], y[1]), g.mul(x[1], y[3])),
        g.add(g.mul(x[2], y[0]), g.mul(x[3], y[2])),
        g.add(g.mul(x[2], y[1]), g.mul(x[3], y[3])),
    ]
}

fn mat_inv(g: &Gf, m: Mat) -> Mat {
    [m[3], g.neg(m[1]), g.neg(m[2]), m[0]]
}

fn mat_apply(g: &Gf, m: Mat, z: u32) -> u32 {
    let (num, den) = if z == INF {
        (m[0], m[2])
    } else {
        (g.add(g.mul(m[0], z), m[1]), g.add(g.mul(m[2], z), m[3]))
    };
    if g.is_zero(den) {
        INF
    } else {
        g.mul(num, g.inv(den))
    }
}

fn mat_frob(g: &Gf, m: Mat, pk: u64) -> Mat {
    [
        g.frob(m[0], pk),
        g.frob(m[1], pk),
        g.frob(m[2], pk),
        g.frob(m[3], pk),
    ]
}

/// The Möbius map sending `z1 -> 0`, `z2 -> 1`, `z3 -> INF`.
fn to_standard(g: &Gf, z1: u32, z2: u32, z3: u32) -> Mat {
    let one = 0u32;
    if z1 == INF {
        // (z2 - z3) / (w - z3)
        [g.zero(), g.sub(z2, z3), one, g.neg(z3)]
    } else if z2 == INF {
        // (w - z1) / (w - z3)
        [one, g.neg(z1), one, g.neg(z3)]
    } else if z3 == INF {
        // (w - z1) / (z2 - z1)
        [one, g.neg(z1), g.zero(), g.sub(z2, z1)]
    } else {
        // (w - z1)(z2 - z3) / ((w - z3)(z2 - z1))
        let a = g.sub(z2, z3);
        let c = g.sub(z2, z1);
        [a, g.neg(g.mul(z1, a)), c, g.neg(g.mul(z3, c))]
    }
}

#[derive(Clone, Debug, Default)]
struct AutInfo {
    geom_order: usize,
    k_order: usize,
    k_involutions: usize,
    klein_sigma_stable: bool,
    ghs_signature: bool,
}

/// Reduced automorphism group of the Weierstrass set `w` (8 points of
/// `P^1(F_{q^6})`), its `F_{q^3}`-rational part and the GHS signature.
fn aut_info(g: &Gf, w: &[u32]) -> AutInfo {
    let wset: BTreeSet<u32> = w.iter().copied().collect();
    let base = to_standard(g, w[0], w[1], w[2]);
    let mut group: BTreeSet<Mat> = BTreeSet::new();
    for &s1 in w {
        for &s2 in w {
            if s2 == s1 {
                continue;
            }
            for &s3 in w {
                if s3 == s1 || s3 == s2 {
                    continue;
                }
                let t = to_standard(g, s1, s2, s3);
                let m = mat_norm(g, mat_mul(g, mat_inv(g, t), base));
                let image: BTreeSet<u32> = w.iter().map(|&z| mat_apply(g, m, z)).collect();
                if image == wset {
                    group.insert(m);
                }
            }
        }
    }
    let pk3 = g.pk_mod(3);
    let pk1 = g.pk_mod(1);
    let identity = mat_norm(g, [0, g.zero(), g.zero(), 0]);
    let k_group: Vec<Mat> = group
        .iter()
        .copied()
        .filter(|&m| mat_norm(g, mat_frob(g, m, pk3)) == m)
        .collect();
    let invs: Vec<Mat> = k_group
        .iter()
        .copied()
        .filter(|&m| m != identity && mat_norm(g, mat_mul(g, m, m)) == identity)
        .collect();
    let mut info = AutInfo {
        geom_order: group.len(),
        k_order: k_group.len(),
        k_involutions: invs.len(),
        ..Default::default()
    };
    for (i, &m1) in invs.iter().enumerate() {
        for &m2 in invs.iter().skip(i + 1) {
            let m12 = mat_norm(g, mat_mul(g, m1, m2));
            if m12 != mat_norm(g, mat_mul(g, m2, m1)) {
                continue;
            }
            let v = [m1, m2, m12];
            let s1 = mat_norm(g, mat_frob(g, m1, pk1));
            let s2 = mat_norm(g, mat_frob(g, m2, pk1));
            let stable = v.contains(&s1) && v.contains(&s2);
            if !stable {
                continue;
            }
            info.klein_sigma_stable = true;
            let cyclic = s1 != m1 && s2 != m2 && mat_norm(g, mat_frob(g, m12, pk1)) != m12;
            let free = v
                .iter()
                .all(|&m| w.iter().all(|&z| mat_apply(g, m, z) != z));
            if cyclic && free {
                info.ghs_signature = true;
            }
        }
    }
    info
}

struct ResModel {
    model: Model,
    t: i64,
}

fn enumerate_models(
    cx: &Census,
    deg: usize,
    c_nu: bool,
    top: u32,
    out: &mut Vec<ResModel>,
    stats: &mut (u64, u64),
) {
    let p = cx.q;
    // free coefficient positions
    // degree 8: x^7 killed, free x^0..x^5, top x^6; degree 7: x^6 killed
    // (free x^0..x^4, top x^5) except for p = 7, where x^6 stays free.
    let (free, top_pos): (Vec<usize>, usize) = if deg == 8 || p == 7 {
        ((0..=5).collect(), 6)
    } else {
        ((0..=4).collect(), 5)
    };
    let total = (p as u64).pow(free.len() as u32);
    let mut coeffs = vec![0u32; 9];
    coeffs[deg] = 1;
    coeffs[top_pos] = top;
    for idx in 0..total {
        let mut v = idx;
        for &pos in &free {
            coeffs[pos] = (v % p as u64) as u32;
            v /= p as u64;
        }
        stats.0 += 1;
        if !is_squarefree(&coeffs[..=deg], p) {
            continue;
        }
        stats.1 += 1;
        if let Some((t, _n3)) = cx.res_shaped(&coeffs, deg, c_nu) {
            let mut c8 = [0u8; 9];
            for i in 0..9 {
                c8[i] = coeffs[i] as u8;
            }
            out.push(ResModel {
                model: Model {
                    deg: deg as u8,
                    c_nu,
                    coeffs: c8,
                },
                t,
            });
        }
    }
}

struct UnionFind {
    parent: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
        }
    }
    fn find(&mut self, x: usize) -> usize {
        let mut r = x;
        while self.parent[r] != r {
            r = self.parent[r];
        }
        let mut c = x;
        while self.parent[c] != r {
            let nx = self.parent[c];
            self.parent[c] = r;
            c = nx;
        }
        r
    }
    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra != rb {
            self.parent[ra.max(rb)] = ra.min(rb);
        }
    }
}

fn fmt_pattern(p: &[u32]) -> String {
    p.iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join("+")
}

struct CensusOut {
    rows: String,
    summary: String,
    checks: String,
}

fn part_c_one(q: u32, log: &mut String) -> CensusOut {
    let t0 = Instant::now();
    let cx = Census::new(q);
    // work items: (deg, c_nu, top)
    let mut items: Vec<(usize, bool, u32)> = Vec::new();
    for deg in [8usize, 7] {
        for c_nu in [false, true] {
            for top in 0..q {
                items.push((deg, c_nu, top));
            }
        }
    }
    let nthreads = 4usize;
    let chunk = items.len().div_ceil(nthreads);
    let results: Vec<(Vec<ResModel>, (u64, u64))> = std::thread::scope(|sc| {
        let hs: Vec<_> = items
            .chunks(chunk)
            .map(|ch| {
                let cx = &cx;
                sc.spawn(move || {
                    let mut out = Vec::new();
                    let mut stats = (0u64, 0u64);
                    for &(deg, c_nu, top) in ch {
                        enumerate_models(cx, deg, c_nu, top, &mut out, &mut stats);
                    }
                    (out, stats)
                })
            })
            .collect();
        hs.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let mut models: Vec<ResModel> = Vec::new();
    let mut n_models = 0u64;
    let mut n_sqfree = 0u64;
    for (v, (a, b)) in results {
        models.extend(v);
        n_models += a;
        n_sqfree += b;
    }
    let t_enum = t0.elapsed().as_secs_f64();
    // classes by union-find over the moves
    let index: HashMap<Model, usize> = models
        .iter()
        .enumerate()
        .map(|(i, m)| (m.model.clone(), i))
        .collect();
    assert_eq!(
        index.len(),
        models.len(),
        "duplicate models in the enumeration"
    );
    let mut uf = UnionFind::new(models.len());
    let mut missing = 0u64;
    for (i, m) in models.iter().enumerate() {
        for nb in cx.neighbours(&m.model) {
            match index.get(&nb) {
                Some(&j) => uf.union(i, j),
                None => missing += 1,
            }
        }
    }
    let mut classes: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for i in 0..models.len() {
        let r = uf.find(i);
        classes.entry(r).or_default().push(i);
    }
    let mut checks = String::new();
    let _ = writeln!(checks, "q={q} models={n_models} squarefree={n_sqfree} res_shaped_models={} classes={} neighbours_missing={missing}", models.len(), classes.len());
    assert_eq!(
        missing, 0,
        "a neighbour of a Res-shaped model is not in the enumeration"
    );
    let hasse = 2.0 * (q as f64).powf(1.5);
    let mut bad_t = 0u64;
    let mut t_inconsistent = 0u64;
    let q3 = (q as i64).pow(3);
    let mut rows = String::new();
    let mut n_sig = 0usize;
    let mut n_nosig = 0usize;
    let mut n_na = 0usize;
    let mut n_prime = 0usize;
    let mut n_prime_nosig = 0usize;
    let mut sig_bad_mod4 = 0usize;
    let mut sig_bad_cycle = 0usize;
    let mut n_subfield = 0usize;
    let mut n_subfield_sig = 0usize;
    let mut traces: BTreeSet<i64> = BTreeSet::new();
    let mut traces_sig: BTreeSet<i64> = BTreeSet::new();
    let mut by_pattern: BTreeMap<(String, &str), usize> = BTreeMap::new();
    for members in classes.values() {
        let rep = members
            .iter()
            .min_by_key(|&&i| &models[i].model)
            .copied()
            .unwrap();
        let m = &models[rep];
        // consistency of t across the class
        for &i in members {
            if models[i].t != m.t {
                t_inconsistent += 1;
            }
        }
        if (m.t as f64).abs() > hasse + 1e-9 {
            bad_t += 1;
        }
        let coeffs: Vec<u32> = m.model.coeffs.iter().map(|&v| v as u32).collect();
        let deg = m.model.deg as usize;
        let pattern = cx.cycle_type(&coeffs, deg);
        let num_e = q3 + 1 - m.t;
        let e_prime = is_prime(num_e as u64);
        let bound = (2.0 * (q as f64).sqrt()).floor() as i64;
        let t_subfield = (-bound..=bound).any(|t0| t0 * t0 * t0 - 3 * q as i64 * t0 == m.t);
        if t_subfield {
            n_subfield += 1;
        }
        let w = cx.weierstrass_points(&coeffs, deg, &pattern);
        let info = w.as_ref().map(|w| aut_info(&cx.f6, w));
        let sig_str = match &info {
            Some(i) if i.ghs_signature => "yes",
            Some(_) => "no",
            None => "no(roots outside F_q^6)",
        };
        let pat_s = fmt_pattern(&pattern);
        match sig_str {
            "yes" => n_sig += 1,
            "no" => n_nosig += 1,
            _ => n_na += 1,
        }
        *by_pattern
            .entry((pat_s.clone(), if sig_str == "yes" { "yes" } else { "no" }))
            .or_insert(0) += 1;
        if e_prime {
            n_prime += 1;
            if sig_str != "yes" {
                n_prime_nosig += 1;
            }
        }
        traces.insert(m.t);
        if sig_str == "yes" {
            if t_subfield {
                n_subfield_sig += 1;
            }
            traces_sig.insert(m.t);
            if num_e % 4 != 0 {
                sig_bad_mod4 += 1;
            }
            if !(pat_s == "1+1+3+3" || pat_s == "2+6") {
                sig_bad_cycle += 1;
            }
        }
        let coeff_s = coeffs[..=deg]
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let _ = writeln!(
            rows,
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            q,
            deg,
            if m.model.c_nu { "nu" } else { "1" },
            coeff_s,
            m.t,
            num_e,
            e_prime,
            t_subfield,
            num_e.rem_euclid(4),
            pat_s,
            info.as_ref()
                .map(|i| i.geom_order.to_string())
                .unwrap_or_else(|| "-".into()),
            info.as_ref()
                .map(|i| i.k_order.to_string())
                .unwrap_or_else(|| "-".into()),
            info.as_ref()
                .map(|i| i.k_involutions.to_string())
                .unwrap_or_else(|| "-".into()),
            info.as_ref()
                .map(|i| i.klein_sigma_stable.to_string())
                .unwrap_or_else(|| "-".into()),
            sig_str,
            members.len()
        );
    }
    let _ = writeln!(checks, "q={q} check8_hasse_violations={bad_t} t_inconsistent_in_class={t_inconsistent} check9_signature_numE_mod4_violations={sig_bad_mod4} check9_signature_cycle_violations={sig_bad_cycle}");
    let mut summary = String::new();
    let _ = writeln!(
        summary,
        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.1}",
        q,
        n_models,
        n_sqfree,
        models.len(),
        classes.len(),
        n_sig,
        n_nosig,
        n_na,
        n_prime,
        n_prime_nosig,
        n_subfield,
        n_subfield_sig,
        traces.len(),
        traces_sig.len(),
        t0.elapsed().as_secs_f64()
    );
    let mut pat_lines = String::new();
    for ((pat, sig), c) in &by_pattern {
        let _ = writeln!(pat_lines, "{q},{pat},{sig},{c}");
    }
    let _ = writeln!(log, "part C q={q} models={n_models} res_shaped={} classes={} sig={n_sig} nosig={n_nosig} na={n_na} enum={t_enum:.1}s total={:.1}s", models.len(), classes.len(), t0.elapsed().as_secs_f64());
    CensusOut {
        rows,
        summary: summary + "PATTERNS\n" + &pat_lines,
        checks,
    }
}

fn part_c(out: &Path, log: &mut String) {
    let mut rows = String::from("q,deg,c,f_coeffs_asc,t,numE,numE_prime,t_subfield_shape,numE_mod4,weierstrass_cycle_type,aut_geom_order,aut_K_order,K_involutions,klein_sigma_stable,ghs_signature,models_in_class\n");
    let mut summary = String::from("q,models_enumerated,squarefree,res_shaped_models,res_shaped_classes,classes_signature,classes_no_signature,classes_roots_outside_Fq6,classes_numE_prime,classes_numE_prime_no_signature,classes_t_subfield_shape,classes_t_subfield_shape_with_signature,distinct_traces,traces_with_signature,seconds\n");
    let mut patterns = String::from("q,cycle_type,ghs_signature,classes\n");
    let mut checks = String::new();
    let qmax: u32 = std::env::var("TVFT_QMAX")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(u32::MAX);
    for &q in &CENSUS_Q {
        if q > qmax {
            continue;
        }
        let r = part_c_one(q, log);
        rows.push_str(&r.rows);
        let mut parts = r.summary.splitn(2, "PATTERNS\n");
        summary.push_str(parts.next().unwrap());
        patterns.push_str(parts.next().unwrap_or(""));
        checks.push_str(&r.checks);
        fs::write(out.join("census.csv"), &rows).unwrap();
        fs::write(out.join("census_summary.csv"), &summary).unwrap();
        fs::write(out.join("census_patterns.csv"), &patterns).unwrap();
        fs::write(out.join("census_checks.txt"), &checks).unwrap();
        eprintln!("{}", log.lines().last().unwrap_or(""));
    }
}

// ===========================================================================
// Driver

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <out-dir> [A|B|C ...]", args[0]);
        std::process::exit(2);
    }
    let out = Path::new(&args[1]);
    fs::create_dir_all(out).unwrap();
    let parts: Vec<String> = if args.len() > 2 {
        args[2..].to_vec()
    } else {
        vec!["A".into(), "B".into(), "C".into()]
    };
    let mut log = String::new();
    let t0 = Instant::now();
    for p in &parts {
        match p.as_str() {
            "A" => part_a(out, &mut log),
            "B" => part_b(out, &mut log),
            "C" => part_c(out, &mut log),
            _ => panic!("unknown part {p}"),
        }
        let _ = writeln!(log, "elapsed {:.1}s", t0.elapsed().as_secs_f64());
        fs::write(out.join("tool_log.txt"), &log).unwrap();
    }
    eprintln!("{log}");
}

// ===========================================================================
// Tests: the protocol's checks 1–4 and the positive controls

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check1_n7() {
        let rows = binary_rows(7);
        let ti = rows.iter().find(|r| r.type_i && r.m == 4).unwrap();
        assert_eq!((ti.deg, ti.genus, ti.delta), (3, Some(7), 4));
        let tii = rows.iter().find(|r| !r.type_i && r.m == 4).unwrap();
        assert_eq!((tii.deg, tii.genus, tii.delta), (4, Some(8), 3));
        let g = gghs_rows(7);
        let h = g.iter().find(|r| r.r == 3 && r.s == 3 && r.t == 3).unwrap();
        assert_eq!((h.genus, h.delta), (Some(7), 2));
        // GHS 2002 §5 table for type II (deg 4, two factors)
        let expect = [
            (1u32, 0.404),
            (2, 0.286),
            (3, 0.269),
            (4, 0.286),
            (5, 0.323),
        ];
        for (l, v) in expect {
            let p = per_class_expectation(2.0, 4.0, 7, l);
            assert!((p - v).abs() < 0.002, "l={l} got {p}");
        }
    }

    #[test]
    fn check2_n31() {
        let rows = binary_rows(31);
        let tii = rows
            .iter()
            .filter(|r| !r.type_i)
            .min_by_key(|r| r.deg)
            .unwrap();
        assert_eq!((tii.deg, tii.delta, tii.num_f), (6, 25, 6));
        let p = per_class_expectation(6.0, 6.0, 31, 5);
        assert!((log2_f(p) + 52.0).abs() < 1.0, "basic {p}");
        let g = gghs_rows(31);
        let h = g.iter().find(|r| r.r == 5 && r.s == 5 && r.t == 5).unwrap();
        assert_eq!((h.genus, h.delta, h.num_pairs), (Some(31), 22, 6));
        let p2 = per_class_expectation(3.0, 9.0, 31, 5);
        assert!((log2_f(p2) + 38.0).abs() < 1.0, "gghs {p2}");
    }

    #[test]
    fn check3_mmt_counts() {
        // (N, n, m, genus, log2 count)
        for (nn, n, m, g, lg) in [
            (161u32, 7u32, 4u32, 7u128, 94.0f64),
            (165, 15, 5, 15, 57.0),
            (170, 5, 5, 15, 171.0),
        ] {
            let l = nn / n;
            let rows = binary_rows(n);
            let best = rows
                .iter()
                .filter(|r| r.m == m)
                .min_by(|a, b| a.log2_genus.partial_cmp(&b.log2_genus).unwrap())
                .unwrap();
            assert_eq!(best.genus, Some(g), "N={nn}");
            let widest = rows
                .iter()
                .filter(|r| r.m == m)
                .max_by_key(|r| r.deg)
                .unwrap();
            let count = 1.0 + l as f64 * widest.deg as f64 + log2_f(widest.num_f as f64);
            assert!((count - lg).abs() < 1e-9, "N={nn} count {count}");
        }
    }

    #[test]
    fn check4_composite_n() {
        let r15 = binary_rows(15);
        assert_eq!(r15.iter().map(|r| r.m).min(), Some(5));
        let r21 = binary_rows(21);
        assert_eq!(r21.iter().map(|r| r.m).min(), Some(6));
        let best = r21.iter().min_by_key(|r| r.m).unwrap();
        assert_eq!((best.deg, best.type_i), (5, true));
    }

    #[test]
    fn field_tables_consistent() {
        for (p, l) in [(3u32, 2u32), (5, 3), (7, 2), (13, 2), (3, 6)] {
            let f = Gf::new(p, l);
            // a + b via digits equals table addition
            for a in 0..f.nn.min(200) {
                for b in 0..f.nn.min(200) {
                    let da = f.digits(a);
                    let db = f.digits(b);
                    let mut idx = 0u32;
                    let mut mp = 1u32;
                    for i in 0..l as usize {
                        idx += (da[i] + db[i]) % p * mp;
                        mp *= p;
                    }
                    assert_eq!(f.add(a, b), f.log[idx as usize]);
                }
            }
            assert_eq!(f.mul(5 % f.nn, f.inv(5 % f.nn)), 0);
            assert!(f.in_sub(f.step(1), 1));
            assert_eq!(f.const_int(p as i64), f.zero());
        }
    }

    #[test]
    fn odd_char_small() {
        let mut log = String::new();
        let cfg = &ODD_CFG[1]; // n = 3
        let r = part_b_one(5, cfg, &mut log);
        assert!(r.rows.contains("5,3,6,3+1+1,5,3,3,"), "{}", r.rows);
        assert!(
            !r.checks.contains("check5_violations=1") && !r.checks.contains("weak_j_outside_K=1"),
            "{}",
            r.checks
        );
        for line in r.checks.lines() {
            if line.contains("check5_violations") {
                assert!(line.ends_with("check5_violations=0"), "{line}");
            }
        }
    }

    #[test]
    fn controls_jv_family() {
        let fixtures: [(u32, i64, usize, &str, &str); 12] = [
            (5, 14, 7, "1 3 0 4 3 4 0 1", "1+1+3+3"),
            (5, 2, 8, "3 3 0 4 1 2 4 4 4", "1+1+3+3"),
            (5, 2, 8, "1 0 2 2 3 1 1 0 4", "2+6"),
            (7, 0, 7, "0 3 0 0 0 0 0 1", "1+1+3+3"),
            (7, 16, 8, "0 4 1 0 0 6 0 6 2", "1+1+3+3"),
            (7, -24, 8, "1 0 1 1 0 6 2 0 2", "2+6"),
            (11, 12, 7, "4 5 4 10 5 10 0 1", "1+1+3+3"),
            (11, -20, 8, "10 1 9 0 2 7 10 10 3", "1+1+3+3"),
            (11, -60, 8, "7 8 5 1 2 6 2 0 3", "2+6"),
            (13, -70, 7, "0 7 0 0 12 0 0 1", "1+1+3+3"),
            (13, 10, 8, "0 6 12 0 1 5 0 12 10", "1+1+3+3"),
            (13, -2, 8, "7 0 12 7 0 5 5 0 10", "2+6"),
        ];
        let mut cx_cache: HashMap<u32, Census> = HashMap::new();
        for (p, t_exp, deg, coeffs, cyc) in fixtures {
            let cx = cx_cache.entry(p).or_insert_with(|| Census::new(p));
            let raw: Vec<u32> = coeffs.split(' ').map(|v| v.parse().unwrap()).collect();
            let mut form = [0u32; 9];
            for i in 0..=deg {
                form[i] = raw[i] % p;
            }
            let model = cx.normalize(&form);
            let c: Vec<u32> = model.coeffs.iter().map(|&v| v as u32).collect();
            let (t, _) = cx
                .res_shaped(&c, model.deg as usize, model.c_nu)
                .expect("control not Res-shaped");
            assert_eq!(t, t_exp, "p={p} {coeffs}");
            let pat = cx.cycle_type(&c, model.deg as usize);
            assert_eq!(fmt_pattern(&pat), cyc, "p={p} {coeffs}");
            let w = cx.weierstrass_points(&c, model.deg as usize, &pat).unwrap();
            let info = aut_info(&cx.f6, &w);
            assert!(info.ghs_signature, "p={p} {coeffs}: {info:?}");
            // normalisation is idempotent and neighbours normalise back into the family
            let again = cx.normalize(&cx.model_form(&model));
            assert_eq!(again, model);
        }
    }
}
