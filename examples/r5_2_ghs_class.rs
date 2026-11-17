//! Round-5 lead R5.2: how much of an isogeny class over a composite-degree
//! binary field is within reach of GHS Weil descent?
//!
//! Curves: `E_{a,b}: y² + xy = x³ + a x² + b` over `K = F_{2^N}`, `b ≠ 0`,
//! `a ∈ {0, ω}` with `Tr_{K/F_2}(ω) = 1`.  Every ordinary curve over `K` is
//! isomorphic to exactly one of these (`j = 1/b`, the twist is `Tr(a)`).
//!
//! Subcommands (all output is JSON on the path given by `--out`):
//!
//! * `selftest` — hand-checkable magic-number cases, point counts against a
//!   naive count and against `ic_boundary::binary_point_count`, the `√b`
//!   orbit rank against `ec_trapdoor::magic_number`, the division polynomial
//!   against `binary_velu::division_polynomial`, and the ℓ = 3 neighbours
//!   against `binary_isogeny::l_isogenous_neighbours` (Φ₃).
//! * `census N...` — exhaustive over every `b ∈ K*` and both twists: point
//!   count (one Walsh–Hadamard transform gives every Kloosterman sum), the
//!   isogeny class (`#E`), the GHS magic number and genus for every
//!   factorisation `N = l·n` with `n ≥ 2`, and the cost-model verdicts.
//! * `walk N` — the odd-ℓ isogeny graph (ℓ ∈ {3, 5, 7}, Elkies kernels found
//!   by factoring the division polynomial over `K`, codomain by binary Vélu)
//!   on every class that contains both weak and strong curves; distances to
//!   the nearest weak curve, components, and a counted attacker search.
//!
//! Magic number (stated exactly; derivation in RESULTS.md §2):
//!
//! ```text
//!   σ = (x ↦ x^{2^l}),  c_i = σ^i(√b),  U = Span_{F_2}{ (c_i, 1) : 0 ≤ i < n } ⊂ K × F_2
//!   m = dim U,  m' = dim Span{c_i},  f = Ord_σ(√b) (deg f = m')
//!   m = m' + [f(1) ≠ 0]                       (the (0,1) ∈ U case)
//!   g = 2^{m−1} − 1  if (0,1) ∈ U  (⇔ (x+1) ∤ f),   else g = 2^{m−1}
//! ```

use std::cell::Cell;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

use serde_json::{json, Value};

use crypto_lib::binary_ecc::{F2mElement, IrreduciblePoly};
use crypto_lib::cryptanalysis::binary_isogeny::{j_invariant, l_isogenous_neighbours};
use crypto_lib::cryptanalysis::binary_velu::{division_polynomial, elt, to_u64};
use crypto_lib::cryptanalysis::ec_trapdoor::{audit_curve, magic_number, FieldTower};
use crypto_lib::cryptanalysis::ghs_descent::ECurve;
use crypto_lib::cryptanalysis::ic_boundary::{binary_point_count, ArtinSchreier};
use crypto_lib::cryptanalysis::semaev_decomp::Gf2;

// ── F_{2^N}, N ≤ 31, polynomial basis in a u64 ──────────────────────────

struct Fld {
    n: u32,
    modp: u64,
    trmask: u64,
    muls: Cell<u64>,
}

fn clmul(a: u64, b: u64) -> u64 {
    let mut r = 0u64;
    let mut aa = a;
    let mut bb = b;
    while bb != 0 {
        if bb & 1 == 1 {
            r ^= aa;
        }
        aa <<= 1;
        bb >>= 1;
    }
    r
}

fn reduce_by(r: u64, modp: u64, n: u32) -> u64 {
    let mut r = r;
    while r >> n != 0 {
        let hb = 63 - r.leading_zeros();
        r ^= modp << (hb - n);
    }
    r
}

fn poly_gcd_f2(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let db = 63 - b.leading_zeros();
        while a != 0 && 63 - a.leading_zeros() >= db {
            let da = 63 - a.leading_zeros();
            a ^= b << (da - db);
        }
        std::mem::swap(&mut a, &mut b);
    }
    a
}

fn prime_factors(mut x: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut p = 2u64;
    while p * p <= x {
        if x.is_multiple_of(p) {
            out.push(p);
            while x.is_multiple_of(p) {
                x /= p;
            }
        }
        p += 1;
    }
    if x > 1 {
        out.push(x);
    }
    out
}

/// Rabin: `f` of degree `n` is irreducible iff `X^{2^n} ≡ X` and
/// `gcd(X^{2^{n/p}} − X, f) = 1` for each prime `p | n`.
fn is_irreducible_f2(f: u64, n: u32) -> bool {
    let sq = |a: u64| reduce_by(clmul(a, a), f, n);
    let mut pw = vec![2u64];
    let mut cur = 2u64;
    for _ in 0..n {
        cur = sq(cur);
        pw.push(cur);
    }
    if pw[n as usize] != 2 {
        return false;
    }
    for p in prime_factors(n as u64) {
        let k = (n as u64 / p) as usize;
        if poly_gcd_f2(pw[k] ^ 2, f) != 1 {
            return false;
        }
    }
    true
}

impl Fld {
    fn new(n: u32) -> Self {
        assert!((2..=31).contains(&n));
        let top = 1u64 << n;
        let mut modp = 0;
        'search: for k in 1..n {
            let f = top | (1 << k) | 1;
            if is_irreducible_f2(f, n) {
                modp = f;
                break 'search;
            }
        }
        if modp == 0 {
            'p5: for a in 3..n {
                for b in 2..a {
                    for c in 1..b {
                        let f = top | (1 << a) | (1 << b) | (1 << c) | 1;
                        if is_irreducible_f2(f, n) {
                            modp = f;
                            break 'p5;
                        }
                    }
                }
            }
        }
        assert!(modp != 0);
        let mut fld = Fld {
            n,
            modp,
            trmask: 0,
            muls: Cell::new(0),
        };
        let mut mask = 0u64;
        for i in 0..n {
            if fld.tr_slow(1u64 << i) == 1 {
                mask |= 1 << i;
            }
        }
        fld.trmask = mask;
        fld
    }
    fn irr(&self) -> IrreduciblePoly {
        IrreduciblePoly {
            degree: self.n,
            low_terms: (0..self.n).filter(|i| (self.modp >> i) & 1 == 1).collect(),
        }
    }
    #[inline]
    fn mul(&self, a: u64, b: u64) -> u64 {
        self.muls.set(self.muls.get() + 1);
        reduce_by(clmul(a, b), self.modp, self.n)
    }
    #[inline]
    fn sqr(&self, a: u64) -> u64 {
        self.mul(a, a)
    }
    fn pow2k(&self, a: u64, k: u32) -> u64 {
        let mut x = a;
        for _ in 0..k {
            x = self.sqr(x);
        }
        x
    }
    fn tr_slow(&self, a: u64) -> u64 {
        let mut s = 0u64;
        let mut x = a;
        for _ in 0..self.n {
            s ^= x;
            x = self.sqr(x);
        }
        assert!(s <= 1);
        s
    }
    #[inline]
    fn tr(&self, a: u64) -> u64 {
        ((a & self.trmask).count_ones() & 1) as u64
    }
    fn sqrt(&self, a: u64) -> u64 {
        self.pow2k(a, self.n - 1)
    }
    fn inv(&self, a: u64) -> u64 {
        // a^(2^n - 2) = (a^(2^(n-1) - 1))^2
        assert!(a != 0);
        let mut r = a;
        for _ in 1..self.n - 1 {
            r = self.mul(self.sqr(r), a);
        }
        self.sqr(r)
    }
    /// An element of trace 1 (the twist coefficient ω).
    fn omega(&self) -> u64 {
        (1..).find(|&x| self.tr(x) == 1).unwrap()
    }
}

// ── point counts via one Walsh–Hadamard transform ───────────────────────
//
// #E_{a,b} = 2 + Σ_{x≠0} (1 + (−1)^{Tr(x + a + b/x²)})
//          = 2^N + 1 + (−1)^{Tr a} (W(√b) − 1),
// W(c) = Σ_u (−1)^{Tr(u^{−1}) + Tr(c u)}  (0^{−1} := 0).
// So t(0,b) = 1 − W(√b) and t(ω,b) = −t(0,b).

/// `t0[b]` = trace of Frobenius of `E_{0,b}` for every `b` (index 0 unused).
fn all_traces(f: &Fld) -> Vec<i32> {
    let n = f.n;
    let size = 1usize << n;
    let mut g = vec![1i32; size];
    for (u, gu) in g.iter_mut().enumerate().skip(1) {
        let iu = f.inv(u as u64);
        if f.tr(iu) == 1 {
            *gu = -1;
        }
    }
    // in-place WHT
    let mut h = 1;
    while h < size {
        for i in (0..size).step_by(2 * h) {
            for j in i..i + h {
                let x = g[j];
                let y = g[j + h];
                g[j] = x + y;
                g[j + h] = x - y;
            }
        }
        h *= 2;
    }
    // w(c)_j = Tr(c z^j); rows[i] = w(z^i)
    let mut rows = vec![0u64; n as usize];
    for (i, row) in rows.iter_mut().enumerate() {
        let mut w = 0u64;
        for j in 0..n {
            if f.tr(f.mul(1u64 << i, 1u64 << j)) == 1 {
                w |= 1 << j;
            }
        }
        *row = w;
    }
    let wmap = |c: u64| -> usize {
        let mut w = 0u64;
        for (i, row) in rows.iter().enumerate() {
            if (c >> i) & 1 == 1 {
                w ^= row;
            }
        }
        w as usize
    };
    let mut t0 = vec![0i32; size];
    for (b, tb) in t0.iter_mut().enumerate().skip(1) {
        let c = f.sqrt(b as u64);
        *tb = 1 - g[wmap(c)];
    }
    t0
}

fn naive_count(f: &Fld, a: u64, b: u64) -> u64 {
    let mut c = 2u64;
    for x in 1..(1u64 << f.n) {
        let ix = f.inv(x);
        let v = x ^ a ^ f.mul(b, f.sqr(ix));
        if f.tr(v) == 0 {
            c += 2;
        }
    }
    c
}

// ── magic number ────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, Default)]
struct Magic {
    /// dim Span{(c_i, 1)}
    m: u32,
    /// dim Span{c_i} = deg Ord_σ(√b)
    m_prime: u32,
    /// f(1) for f = Ord_σ(√b) (1 ⇔ (x+1) ∤ f ⇔ (0,1) ∈ U)
    f_at_1: u32,
    /// σ-orbit length of √b (b lies in F_{q^orbit})
    orbit: u32,
    genus: u64,
}

/// Rank of a list of bit-vectors (≤ 64 bits) over F_2.
fn rank_f2(vs: &[u64]) -> u32 {
    let mut basis: Vec<u64> = Vec::new();
    for &v0 in vs {
        let mut v = v0;
        for &bv in &basis {
            v = v.min(v ^ bv);
        }
        if v != 0 {
            basis.push(v);
            basis.sort_unstable_by(|a, b| b.cmp(a));
        }
    }
    basis.len() as u32
}

fn magic(f: &Fld, b: u64, l: u32, n: u32) -> Magic {
    let sb = f.sqrt(b);
    let mut cs = Vec::with_capacity(n as usize);
    let mut cur = sb;
    let mut orbit = 0;
    for i in 0..n {
        cs.push(cur);
        cur = f.pow2k(cur, l);
        if orbit == 0 && cur == sb {
            orbit = i + 1;
        }
    }
    let ext: Vec<u64> = cs.iter().map(|&c| c | (1u64 << f.n)).collect();
    let m = rank_f2(&ext);
    // first linear dependency among c_0, c_1, ... gives f = Ord_σ(√b)
    let mut basis: Vec<(u64, u64)> = Vec::new(); // (vector, combination mask)
    let mut m_prime = n;
    let mut f_at_1 = 0;
    for (k, &c) in cs.iter().enumerate() {
        let mut v = c;
        let mut comb = 1u64 << k;
        // reduce by pivots (highest bit)
        loop {
            if v == 0 {
                break;
            }
            let hb = 63 - v.leading_zeros();
            if let Some(&(bv, bc)) = basis.iter().find(|(bv, _)| 63 - bv.leading_zeros() == hb) {
                v ^= bv;
                comb ^= bc;
            } else {
                break;
            }
        }
        if v == 0 {
            m_prime = k as u32;
            f_at_1 = comb.count_ones() & 1;
            break;
        }
        basis.push((v, comb));
    }
    if m_prime == n {
        // no dependency among n vectors: f = x^n + 1 (σ^n = 1) has f(1) = 0
        f_at_1 = 0;
    }
    let genus = if m == 0 {
        0
    } else if m == m_prime + 1 {
        (1u64 << (m - 1)) - 1
    } else {
        1u64 << (m - 1)
    };
    Magic {
        m,
        m_prime,
        f_at_1,
        orbit,
        genus,
    }
}

fn factorisations(nn: u32) -> Vec<(u32, u32)> {
    (1..nn)
        .filter(|l| nn.is_multiple_of(*l) && nn / l >= 2)
        .map(|l| (l, nn / l))
        .collect()
}

fn largest_prime_factor(mut x: u64) -> u64 {
    let mut best = 1;
    let mut p = 2u64;
    while p * p <= x {
        while x.is_multiple_of(p) {
            best = p;
            x /= p;
        }
        p += 1;
    }
    if x > 1 {
        best = best.max(x);
    }
    best
}

fn log2_factorial(g: u64) -> f64 {
    if g < 2 {
        return 0.0;
    }
    if g < 200 {
        return (2..=g).map(|i| (i as f64).log2()).sum();
    }
    let x = g as f64;
    // Stirling
    (x * x.ln() - x + 0.5 * (2.0 * std::f64::consts::PI * x).ln()) / std::f64::consts::LN_2
}

/// Cost-model verdicts for one curve and one factorisation.
#[derive(Clone, Copy, Default)]
struct Verdict {
    applicable: bool,
    subfield: bool,
    transfer: bool,
    weak_m1: bool,
    weak_m2: bool,
    weak_a: bool,
    weak_p: bool,
    weak_lg: bool,
    /// log2(IC cost under M1) − log2(rho cost); +inf if not applicable
    margin_m1: f64,
    /// log2(IC cost under P) − log2(rho cost); +inf if not applicable
    margin_p: f64,
}

/// log2 L_x[1/2, √2] = √2·√(ln x · ln ln x) / ln 2, with x = q^g (o(1) dropped).
fn log2_l_half(log2_x: f64) -> f64 {
    let lnx = log2_x * std::f64::consts::LN_2;
    if lnx <= std::f64::consts::E {
        return 0.0;
    }
    std::f64::consts::SQRT_2 * (lnx * lnx.ln()).sqrt() / std::f64::consts::LN_2
}

fn verdict(mg: &Magic, l: u32, n: u32, twist: u32, log2_r: f64) -> Verdict {
    let g = mg.genus;
    let subfield = mg.orbit < n;
    // Menezes–Qu condition (recalled): n odd, or m = n, or Tr_{K/F2}(a) = 0.
    let applicable = (twist == 0 || n % 2 == 1 || mg.m == n) && !subfield;
    let q_log = l as f64;
    let sqrtq1 = ((2f64).powf(q_log / 2.0) + 1.0).log2();
    let transfer = log2_r <= 2.0 * g as f64 * sqrtq1;
    let rho = 0.5 * (std::f64::consts::PI / 4.0).log2() + 0.5 * log2_r;
    let mut v = Verdict {
        applicable,
        subfield,
        transfer,
        margin_m1: f64::INFINITY,
        margin_p: f64::INFINITY,
        ..Default::default()
    };
    if applicable && transfer && g >= 2 {
        let e = 2.0 - 2.0 / g as f64;
        let ic1 = e * q_log;
        let ic2 = ic1 + log2_factorial(g);
        v.margin_m1 = ic1 - rho;
        v.weak_m1 = ic1 < rho;
        v.weak_m2 = ic2 < rho;
        // P: the cheaper of Gaudry/GTTD with its g! splitting factor and
        // Enge–Gaudry L_{q^g}[1/2, √2] (both recalled, used as models only)
        let icp = ic2.min(log2_l_half(q_log * g as f64));
        v.margin_p = icp - rho;
        v.weak_p = icp < rho;
        // LG: structural low-genus reach, g ≤ 2n with n ≥ 3
        v.weak_lg = n >= 3 && g <= 2 * n as u64;
    }
    // asymptotic-exponent class: fixed (n, g), q → ∞, r ≈ q^n
    if applicable && g >= n as u64 && g >= 2 {
        v.weak_a = 2.0 - 2.0 / (g as f64) < n as f64 / 2.0;
    }
    v
}

// ── census ──────────────────────────────────────────────────────────────

fn census(nn: u32) -> Value {
    let f = Fld::new(nn);
    let facts = factorisations(nn);
    let t0 = all_traces(&f);
    let q = 1i64 << nn;
    // class key: (twist, t)
    let mut lpf: HashMap<i64, u64> = HashMap::new();
    struct ClassAcc {
        size: u64,
        log2_r: f64,
        weak: Vec<[u64; 5]>, // per fact: M1, M2, P, LG, A
        any: [u64; 5],
        mmin: Vec<u32>,
        mhist: Vec<BTreeMap<u32, u64>>,
        best_margin: f64,
    }
    let nf = facts.len();
    let mut classes: BTreeMap<(u32, i64), ClassAcc> = BTreeMap::new();
    let mut fact_stats: Vec<BTreeMap<&str, u64>> = vec![BTreeMap::new(); nf];
    let mut m_hist: Vec<BTreeMap<u32, u64>> = vec![BTreeMap::new(); nf];
    let mut g_hist: Vec<BTreeMap<u64, u64>> = vec![BTreeMap::new(); nf];
    let mut formula_mismatch = 0u64;
    let mut any_stats: BTreeMap<&str, u64> = BTreeMap::new();
    let mut curve_margin: Vec<((u32, i64), f64)> = Vec::with_capacity(2 << nn);
    for b in 1..(1u64 << nn) {
        let mags: Vec<Magic> = facts.iter().map(|&(l, n)| magic(&f, b, l, n)).collect();
        for mg in &mags {
            if mg.m != mg.m_prime + mg.f_at_1 {
                formula_mismatch += 1;
            }
        }
        for twist in 0..2u32 {
            let t = if twist == 0 {
                t0[b as usize] as i64
            } else {
                -(t0[b as usize] as i64)
            };
            let order = (q + 1 - t) as u64;
            let r = *lpf
                .entry(order as i64)
                .or_insert_with(|| largest_prime_factor(order));
            let log2_r = (r as f64).log2();
            let acc = classes.entry((twist, t)).or_insert_with(|| ClassAcc {
                size: 0,
                log2_r,
                weak: vec![[0; 5]; nf],
                any: [0; 5],
                mmin: vec![u32::MAX; nf],
                mhist: vec![BTreeMap::new(); nf],
                best_margin: f64::INFINITY,
            });
            acc.size += 1;
            let mut any = [false; 5];
            let mut curve_best = f64::INFINITY;
            for (k, &(l, n)) in facts.iter().enumerate() {
                let mg = &mags[k];
                let v = verdict(mg, l, n, twist, log2_r);
                *m_hist[k].entry(mg.m).or_default() += 1;
                *g_hist[k].entry(mg.genus).or_default() += 1;
                let st = &mut fact_stats[k];
                *st.entry("curves").or_default() += 1;
                *st.entry("applicable").or_default() += v.applicable as u64;
                *st.entry("subfield").or_default() += v.subfield as u64;
                *st.entry("transfer_and_applicable").or_default() +=
                    (v.transfer && v.applicable) as u64;
                *st.entry("weak_M1").or_default() += v.weak_m1 as u64;
                *st.entry("weak_M2").or_default() += v.weak_m2 as u64;
                *st.entry("weak_A").or_default() += v.weak_a as u64;
                *st.entry("weak_P").or_default() += v.weak_p as u64;
                *st.entry("weak_LG").or_default() += v.weak_lg as u64;
                if twist == 1 && n % 2 == 0 && mg.m != n {
                    *st.entry("twist_excluded_n_even").or_default() += 1;
                }
                acc.weak[k][0] += v.weak_m1 as u64;
                acc.weak[k][1] += v.weak_m2 as u64;
                acc.weak[k][2] += v.weak_p as u64;
                acc.weak[k][3] += v.weak_lg as u64;
                acc.weak[k][4] += v.weak_a as u64;
                any[0] |= v.weak_m1;
                any[1] |= v.weak_m2;
                any[2] |= v.weak_p;
                any[3] |= v.weak_lg;
                any[4] |= v.weak_a;
                curve_best = curve_best.min(v.margin_p);
                acc.mmin[k] = acc.mmin[k].min(mg.m);
                *acc.mhist[k].entry(mg.m).or_default() += 1;
                acc.best_margin = acc.best_margin.min(v.margin_p);
            }
            for i in 0..5 {
                acc.any[i] += any[i] as u64;
            }
            curve_margin.push(((twist, t), curve_best));
            *any_stats.entry("curves").or_default() += 1;
            *any_stats.entry("weak_M1").or_default() += any[0] as u64;
            *any_stats.entry("weak_M2").or_default() += any[1] as u64;
            *any_stats.entry("weak_P").or_default() += any[2] as u64;
            *any_stats.entry("weak_LG").or_default() += any[3] as u64;
            *any_stats.entry("weak_A").or_default() += any[4] as u64;
        }
    }
    // class-level summaries
    let model_names = ["M1", "M2", "P", "LG", "A"];
    let mut per_fact = Vec::new();
    for (k, &(l, n)) in facts.iter().enumerate() {
        let mut with_weak = [0u64; 5];
        let mut mixed = [0u64; 5];
        let mut fracs: [Vec<f64>; 5] = Default::default();
        let mut weak_in_mixed = [0u64; 5];
        let mut curves_in_mixed = [0u64; 5];
        let mut mixed_min_m = 0u64;
        for acc in classes.values() {
            for i in 0..5 {
                let w = acc.weak[k][i];
                if w > 0 {
                    with_weak[i] += 1;
                }
                if w > 0 && w < acc.size {
                    mixed[i] += 1;
                    fracs[i].push(w as f64 / acc.size as f64);
                    weak_in_mixed[i] += w;
                    curves_in_mixed[i] += acc.size;
                }
            }
            if acc.mhist[k].len() > 1 {
                mixed_min_m += 1;
            }
        }
        let mut models = serde_json::Map::new();
        for i in 0..5 {
            let mut fr = fracs[i].clone();
            fr.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let qt = |p: f64| -> Value {
                if fr.is_empty() {
                    Value::Null
                } else {
                    json!(fr[((fr.len() - 1) as f64 * p).round() as usize])
                }
            };
            models.insert(
                model_names[i].to_string(),
                json!({
                    "classes_with_weak": with_weak[i],
                    "mixed_classes": mixed[i],
                    "weak_fraction_in_mixed_classes": {
                        "min": qt(0.0), "median": qt(0.5), "max": qt(1.0),
                        "pooled": if curves_in_mixed[i] > 0 {
                            json!(weak_in_mixed[i] as f64 / curves_in_mixed[i] as f64)
                        } else { Value::Null },
                    },
                }),
            );
        }
        per_fact.push(json!({
            "l": l, "n": n,
            "counts": fact_stats[k],
            "m_hist": m_hist[k],
            "genus_hist": g_hist[k].iter().map(|(g, c)| (g.to_string(), *c)).collect::<BTreeMap<_, _>>(),
            "classes_with_more_than_one_m": mixed_min_m,
            "models": models,
        }));
    }
    // per-class spread of the best P margin over its curves
    let mut spread: HashMap<(u32, i64), (f64, f64, u64, u64)> = HashMap::new();
    for &(key, mgn) in &curve_margin {
        let e = spread
            .entry(key)
            .or_insert((f64::INFINITY, f64::NEG_INFINITY, 0, 0));
        if mgn.is_finite() {
            e.0 = e.0.min(mgn);
            e.1 = e.1.max(mgn);
        } else {
            e.3 += 1;
        }
    }
    for &(key, mgn) in &curve_margin {
        let e = spread.get_mut(&key).unwrap();
        if mgn.is_finite() && mgn <= e.0 + 0.5 {
            e.2 += 1;
        }
    }
    let mut any_classes = [0u64; 5];
    let mut any_mixed = [0u64; 5];
    let mut class_rows = Vec::new();
    for (&(tw, t), acc) in &classes {
        for i in 0..5 {
            if acc.any[i] > 0 {
                any_classes[i] += 1;
                if acc.any[i] < acc.size {
                    any_mixed[i] += 1;
                }
            }
        }
        let e = spread[&(tw, t)];
        let worst = if e.1.is_finite() {
            json!((e.1 * 1000.0).round() / 1000.0)
        } else {
            Value::Null
        };
        let spread_v = json!([worst, e.2, e.3]);
        class_rows.push(json!([
            tw,
            t,
            acc.size,
            (acc.log2_r * 1000.0).round() / 1000.0,
            acc.any,
            acc.weak,
            acc.mmin,
            if acc.best_margin.is_finite() {
                json!((acc.best_margin * 1000.0).round() / 1000.0)
            } else {
                Value::Null
            },
            spread_v,
        ]));
    }
    json!({
        "N": nn,
        "modulus_hex": format!("{:#x}", f.modp),
        "factorisations": facts,
        "curves": 2 * ((1u64 << nn) - 1),
        "classes": classes.len(),
        "formula_mismatch_m_vs_mprime_plus_f1": formula_mismatch,
        "per_factorisation": per_fact,
        "any_factorisation": {
            "counts": any_stats,
            "classes_with_weak": {"M1": any_classes[0], "M2": any_classes[1], "P": any_classes[2], "LG": any_classes[3], "A": any_classes[4]},
            "mixed_classes": {"M1": any_mixed[0], "M2": any_mixed[1], "P": any_mixed[2], "LG": any_mixed[3], "A": any_mixed[4]},
        },
        "class_rows_schema": ["twist", "t", "size", "log2_r", "weak_any[M1,M2,P,LG,A]",
            "weak_per_fact[[M1,M2,P,LG,A]...]", "min_m_per_fact", "best_margin_P_log2",
            "[worst_finite_best_margin_P, curves_within_0.5bit_of_class_best, curves_no_applicable_descent]"],
        "class_rows": class_rows,
    })
}

// ── polynomials over K ──────────────────────────────────────────────────

type Poly = Vec<u64>;

fn trim(p: &mut Poly) {
    while p.last() == Some(&0) {
        p.pop();
    }
}
fn deg(p: &Poly) -> isize {
    p.len() as isize - 1
}
fn padd(a: &Poly, b: &Poly) -> Poly {
    let mut r = vec![0u64; a.len().max(b.len())];
    for (i, &x) in a.iter().enumerate() {
        r[i] ^= x;
    }
    for (i, &x) in b.iter().enumerate() {
        r[i] ^= x;
    }
    trim(&mut r);
    r
}
fn pmul(f: &Fld, a: &Poly, b: &Poly) -> Poly {
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let mut r = vec![0u64; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            if y != 0 {
                r[i + j] ^= f.mul(x, y);
            }
        }
    }
    trim(&mut r);
    r
}
fn pdivrem(f: &Fld, a: &Poly, m: &Poly) -> (Poly, Poly) {
    let mut r = a.clone();
    trim(&mut r);
    let dm = deg(m);
    assert!(dm >= 0);
    let lc_inv = f.inv(*m.last().unwrap());
    if deg(&r) < dm {
        return (vec![], r);
    }
    let mut qt = vec![0u64; (deg(&r) - dm + 1) as usize];
    while deg(&r) >= dm {
        let dr = deg(&r);
        let c = f.mul(*r.last().unwrap(), lc_inv);
        let s = (dr - dm) as usize;
        qt[s] = c;
        for (i, &mi) in m.iter().enumerate() {
            if mi != 0 {
                r[s + i] ^= f.mul(c, mi);
            }
        }
        trim(&mut r);
    }
    trim(&mut qt);
    (qt, r)
}
fn prem(f: &Fld, a: &Poly, m: &Poly) -> Poly {
    pdivrem(f, a, m).1
}
fn pmonic(f: &Fld, a: &Poly) -> Poly {
    if a.is_empty() {
        return vec![];
    }
    let li = f.inv(*a.last().unwrap());
    a.iter().map(|&x| f.mul(x, li)).collect()
}
fn pgcd(f: &Fld, a: &Poly, b: &Poly) -> Poly {
    let mut x = a.clone();
    let mut y = b.clone();
    trim(&mut x);
    trim(&mut y);
    while !y.is_empty() {
        let r = prem(f, &x, &y);
        x = y;
        y = r;
    }
    pmonic(f, &x)
}
fn psqr_mod(f: &Fld, a: &Poly, m: &Poly) -> Poly {
    let mut r = vec![0u64; if a.is_empty() { 0 } else { 2 * a.len() - 1 }];
    for (i, &x) in a.iter().enumerate() {
        r[2 * i] = f.sqr(x);
    }
    trim(&mut r);
    prem(f, &r, m)
}
fn pmulmod(f: &Fld, a: &Poly, b: &Poly, m: &Poly) -> Poly {
    prem(f, &pmul(f, a, b), m)
}
/// X^{2^k} mod m
fn x_pow2k(f: &Fld, k: u32, m: &Poly) -> Poly {
    let mut x = prem(f, &vec![0, 1], m);
    for _ in 0..k {
        x = psqr_mod(f, &x, m);
    }
    x
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

/// Split a product of distinct monic irreducibles of degree `e` over K.
fn split_equal_degree(f: &Fld, g: &Poly, e: u32, rng: &mut Rng, out: &mut Vec<Poly>) {
    let dg = deg(g);
    if dg <= 0 {
        return;
    }
    if dg as u32 == e {
        out.push(pmonic(f, g));
        return;
    }
    let mask = (1u64 << f.n) - 1;
    loop {
        // T = Σ_{i < N e} r^{2^i} mod g for a random r of degree < deg g.
        // (r = βX alone cannot separate two degree-e factors whose roots
        // have the same sum, since Tr(βθ) = Tr_K(β·Tr_{L/K}θ).)
        let mut r: Poly = (0..dg as usize).map(|_| rng.next() & mask).collect();
        trim(&mut r);
        if deg(&r) < 1 {
            continue;
        }
        let mut s = r;
        let mut acc = s.clone();
        for _ in 1..(f.n * e) {
            s = psqr_mod(f, &s, g);
            acc = padd(&acc, &s);
        }
        let d = pgcd(f, g, &acc);
        if deg(&d) > 0 && deg(&d) < dg {
            let (q2, _) = pdivrem(f, g, &d);
            split_equal_degree(f, &d, e, rng, out);
            split_equal_degree(f, &q2, e, rng, out);
            return;
        }
    }
}

/// Division polynomial ψ_ℓ of E_{a,b} (independent of a in char 2).
fn division_poly(f: &Fld, b: u64, ell: u32) -> Poly {
    let psi1: Poly = vec![1];
    let psi2: Poly = vec![0, 1];
    let psi3: Poly = vec![b, 0, 0, 1, 1];
    let psi4: Poly = vec![0, 0, b, 0, 0, 0, 1];
    let cube = |p: &Poly| pmul(f, &pmul(f, p, p), p);
    let psi5 = padd(&pmul(f, &psi4, &cube(&psi2)), &pmul(f, &psi1, &cube(&psi3)));
    match ell {
        3 => psi3,
        5 => psi5,
        7 => padd(&pmul(f, &psi5, &cube(&psi3)), &pmul(f, &psi2, &cube(&psi4))),
        _ => panic!("ell must be 3, 5 or 7"),
    }
}

/// x(2P) = x² + b/x² on E_{a,b}.
fn dbl_x(f: &Fld, x: u64, b: u64) -> u64 {
    let x2 = f.sqr(x);
    x2 ^ f.mul(b, f.inv(x2))
}

/// Sums `t` of the rational order-ℓ kernels' abscissae (one per ± pair).
/// Returns (kernel t values, errors).
fn rational_kernels(f: &Fld, b: u64, ell: u32, rng: &mut Rng) -> (Vec<u64>, u64) {
    let d = (ell - 1) / 2;
    let psi = pmonic(f, &division_poly(f, b, ell));
    let mut errors = 0;
    let xq = x_pow2k(f, f.n, &psi);
    let g1 = pgcd(f, &psi, &padd(&xq, &vec![0, 1]));
    let mut lin = Vec::new();
    split_equal_degree(f, &g1, 1, rng, &mut lin);
    let roots: Vec<u64> = lin.iter().map(|p| p[0]).collect();
    let rootset: HashSet<u64> = roots.iter().copied().collect();
    let mut used: HashSet<u64> = HashSet::new();
    let mut ts = Vec::new();
    for &x0 in &roots {
        if used.contains(&x0) {
            continue;
        }
        let mut orbit = vec![x0];
        let mut cur = dbl_x(f, x0, b);
        while cur != x0 && orbit.len() <= d as usize {
            orbit.push(cur);
            cur = dbl_x(f, cur, b);
        }
        if orbit.len() != d as usize || orbit.iter().any(|x| !rootset.contains(x)) {
            errors += 1;
            continue;
        }
        let mut t = 0;
        for &x in &orbit {
            used.insert(x);
            t ^= x;
        }
        ts.push(t);
    }
    if d > 1 {
        let xqd = x_pow2k(f, f.n * d, &psi);
        let gd = pgcd(f, &psi, &padd(&xqd, &vec![0, 1]));
        let (gd_only, rem) = pdivrem(f, &gd, &g1);
        assert!(rem.is_empty());
        if deg(&gd_only) > 0 {
            let mut facs = Vec::new();
            split_equal_degree(f, &gd_only, d, rng, &mut facs);
            for fac in facs {
                // θ = X mod fac; θ^{-1} = (θ^{d-1} + … + f_1)/f_0
                let f0 = fac[0];
                let tail: Poly = fac[1..].to_vec();
                let inv_f0 = f.inv(f0);
                let mut th_inv: Poly = tail.iter().map(|&c| f.mul(c, inv_f0)).collect();
                trim(&mut th_inv);
                let th2: Poly = prem(f, &vec![0, 0, 1], &fac);
                let thinv2 = pmulmod(f, &th_inv, &th_inv, &fac);
                let dth = padd(&th2, &thinv2.iter().map(|&c| f.mul(c, b)).collect());
                // evaluate fac at dth (Horner) mod fac
                let mut acc: Poly = vec![];
                for &c in fac.iter().rev() {
                    acc = pmulmod(f, &acc, &dth, &fac);
                    acc = padd(&acc, &vec![c]);
                }
                if acc.is_empty() {
                    ts.push(fac[d as usize - 1]);
                }
            }
        }
    }
    (ts, errors)
}

fn orbit_rep(f: &Fld, b: u64) -> u64 {
    let mut best = b;
    let mut cur = b;
    for _ in 1..f.n {
        cur = f.sqr(cur);
        best = best.min(cur);
    }
    best
}

// ── walk ────────────────────────────────────────────────────────────────

fn walk(nn: u32, ells: &[u32], samples: usize) -> Value {
    let f = Fld::new(nn);
    let facts = factorisations(nn);
    let t0 = all_traces(&f);
    let q = 1i64 << nn;
    let mut rng = Rng(0x9e3779b97f4a7c15 ^ nn as u64);
    // weak flags per curve (best over factorisations) for M1 and A, and per factorisation M1
    let models = ["M1", "LG5", "BEST"];
    // per (twist, b) → target bits: bit0 M1-any, bit1 LG-any, bit2 within 0.5 bit
    // of the class's best P margin (filled after the first pass)
    let size = 1usize << nn;
    let mut weak = vec![[0u8; 2]; size];
    let mut pm = vec![[f64::INFINITY; 2]; size];
    let mut cmin: HashMap<(u32, i64), f64> = HashMap::new();
    let class_of = |tw: u32, b: usize| -> i64 {
        if tw == 0 {
            t0[b] as i64
        } else {
            -(t0[b] as i64)
        }
    };
    let mut lpf: HashMap<i64, f64> = HashMap::new();
    for b in 1..size {
        let mags: Vec<Magic> = facts
            .iter()
            .map(|&(l, n)| magic(&f, b as u64, l, n))
            .collect();
        for tw in 0..2u32 {
            let t = class_of(tw, b);
            let order = (q + 1 - t) as u64;
            let lr = *lpf
                .entry(order as i64)
                .or_insert_with(|| (largest_prime_factor(order) as f64).log2());
            let mut w = 0u8;
            for (k, &(l, n)) in facts.iter().enumerate() {
                let v = verdict(&mags[k], l, n, tw, lr);
                if v.weak_m1 {
                    w |= 1;
                }
                // walk target LG5: low genus at a factorisation with n ≥ 5,
                // the only range where LG separates curves (n = 3, 4 give g ≤ 2n always)
                if v.weak_lg && n >= 5 {
                    w |= 2;
                }
                pm[b][tw as usize] = pm[b][tw as usize].min(v.margin_p);
            }
            weak[b][tw as usize] = w;
            let e = cmin.entry((tw, t)).or_insert(f64::INFINITY);
            *e = e.min(pm[b][tw as usize]);
        }
    }
    for b in 1..size {
        for tw in 0..2u32 {
            let c = cmin[&(tw, class_of(tw, b))];
            let x = pm[b][tw as usize];
            if c.is_finite() && x <= c + 0.5 {
                weak[b][tw as usize] |= 4;
            }
        }
    }
    let mut out_models = serde_json::Map::new();
    // neighbour cache on orbit reps: rep -> Vec<(ell, neighbour rep)>
    let mut nbr_cache: HashMap<u64, Vec<(u32, u64)>> = HashMap::new();
    let mut kernel_hist: BTreeMap<String, u64> = BTreeMap::new();
    let mut trace_fail = 0u64;
    let mut orbit_errors = 0u64;
    let mut asym_edges = 0u64;
    let mut kernel_searches = 0u64;
    let muls_before = f.muls.get();
    for (mi, mname) in models.iter().enumerate() {
        // classes: key (tw, t) -> (weak count, size)
        let mut cls: HashMap<(u32, i64), (u64, u64)> = HashMap::new();
        for (b, wb) in weak.iter().enumerate().skip(1) {
            for tw in 0..2u32 {
                let t = if tw == 0 {
                    t0[b] as i64
                } else {
                    -(t0[b] as i64)
                };
                let e = cls.entry((tw, t)).or_default();
                e.1 += 1;
                e.0 += ((wb[tw as usize] >> mi) & 1) as u64;
            }
        }
        let mut mixed: Vec<(u32, i64)> = cls
            .iter()
            .filter(|(_, &(w, s))| w > 0 && w < s)
            .map(|(k, _)| *k)
            .collect();
        mixed.sort();
        let mixed_set: HashSet<(u32, i64)> = mixed.iter().copied().collect();
        // orbit reps in mixed classes, by class
        let mut reps_by_class: HashMap<(u32, i64), Vec<u64>> = HashMap::new();
        let mut orbit_size: HashMap<u64, u64> = HashMap::new();
        for b in 1..size as u64 {
            let rep = orbit_rep(&f, b);
            *orbit_size.entry(rep).or_default() += 1;
            if rep != b {
                continue;
            }
            for tw in 0..2u32 {
                let t = if tw == 0 {
                    t0[b as usize] as i64
                } else {
                    -(t0[b as usize] as i64)
                };
                if mixed_set.contains(&(tw, t)) {
                    reps_by_class.entry((tw, t)).or_default().push(b);
                }
            }
        }
        // neighbours (twist-independent: ψ_ℓ and Vélu do not see a)
        let mut all_reps: Vec<u64> = reps_by_class.values().flatten().copied().collect();
        all_reps.sort_unstable();
        all_reps.dedup();
        for &b in &all_reps {
            if nbr_cache.contains_key(&b) {
                continue;
            }
            let mut nb = Vec::new();
            if std::env::var("R52_DEBUG").is_ok() {
                eprintln!("rep {b:#x} cache {}", nbr_cache.len());
            }
            for &ell in ells {
                kernel_searches += 1;
                let t_start = std::time::Instant::now();
                let (ts, errs) = rational_kernels(&f, b, ell, &mut rng);
                if std::env::var("R52_DEBUG").is_ok() {
                    eprintln!("  ell {ell} kernels {} {:?}", ts.len(), t_start.elapsed());
                }
                orbit_errors += errs;
                *kernel_hist
                    .entry(format!("ell{}_k{}", ell, ts.len()))
                    .or_default() += 1;
                for t in ts {
                    let b2 = b ^ t ^ f.sqr(t);
                    if b2 == 0 || t0[b2 as usize] != t0[b as usize] {
                        trace_fail += 1;
                        continue;
                    }
                    nb.push((ell, orbit_rep(&f, b2)));
                }
            }
            nbr_cache.insert(b, nb);
        }
        // per class BFS
        let mut dist_hist: BTreeMap<String, u64> = BTreeMap::new(); // curve-weighted
        let mut unreachable_curves = 0u64;
        let mut strong_curves = 0u64;
        let mut comps_total = 0u64;
        let mut comps_without_weak = 0u64;
        let mut class_rows = Vec::new();
        let mut search_rows = Vec::new();
        let mut srch_expanded: Vec<u64> = Vec::new();
        let mut srch_len: Vec<u64> = Vec::new();
        let mut srch_fail = 0u64;
        for key in &mixed {
            let reps = &reps_by_class[key];
            let tw = key.0 as usize;
            let repset: HashSet<u64> = reps.iter().copied().collect();
            let is_weak = |b: u64| (weak[b as usize][tw] >> mi) & 1 == 1;
            // adjacency restricted to class (neighbours of a curve are in its class)
            let adj = |b: u64| -> Vec<u64> {
                nbr_cache[&b]
                    .iter()
                    .map(|&(_, r)| r)
                    .filter(|r| repset.contains(r))
                    .collect()
            };
            for &b in reps {
                for nbv in adj(b) {
                    if !adj(nbv).contains(&b) {
                        asym_edges += 1;
                    }
                }
            }
            // multi-source BFS from weak reps
            let mut dist: HashMap<u64, u32> = HashMap::new();
            let mut dq = VecDeque::new();
            for &b in reps {
                if is_weak(b) {
                    dist.insert(b, 0);
                    dq.push_back(b);
                }
            }
            while let Some(u) = dq.pop_front() {
                let du = dist[&u];
                for v in adj(u) {
                    if let std::collections::hash_map::Entry::Vacant(e) = dist.entry(v) {
                        e.insert(du + 1);
                        dq.push_back(v);
                    }
                }
            }
            let mut maxd = 0;
            let mut class_strong = 0u64;
            let mut class_unreach = 0u64;
            for &b in reps {
                let os = orbit_size[&b];
                if is_weak(b) {
                    continue;
                }
                class_strong += os;
                match dist.get(&b) {
                    Some(&d) => {
                        maxd = maxd.max(d);
                        *dist_hist.entry(d.to_string()).or_default() += os;
                    }
                    None => {
                        class_unreach += os;
                    }
                }
            }
            strong_curves += class_strong;
            unreachable_curves += class_unreach;
            // components
            let mut seen: HashSet<u64> = HashSet::new();
            let mut ncomp = 0;
            let mut nweakless = 0;
            for &b in reps {
                if seen.contains(&b) {
                    continue;
                }
                ncomp += 1;
                let mut has_weak = false;
                let mut st = vec![b];
                seen.insert(b);
                while let Some(u) = st.pop() {
                    has_weak |= is_weak(u);
                    for v in adj(u) {
                        if seen.insert(v) {
                            st.push(v);
                        }
                    }
                }
                if !has_weak {
                    nweakless += 1;
                }
            }
            comps_total += ncomp;
            comps_without_weak += nweakless;
            let class_size: u64 = reps.iter().map(|b| orbit_size[b]).sum();
            let weak_curves: u64 = reps
                .iter()
                .filter(|&&b| is_weak(b))
                .map(|b| orbit_size[b])
                .sum();
            class_rows.push(json!([
                key.0,
                key.1,
                class_size,
                weak_curves,
                reps.len(),
                ncomp,
                nweakless,
                maxd,
                class_unreach
            ]));
            // attacker search: BFS from sampled strong reps until a weak rep
            let strong_reps: Vec<u64> = reps.iter().copied().filter(|&b| !is_weak(b)).collect();
            let take = samples.min(strong_reps.len());
            for s in 0..take {
                let start = strong_reps[(rng.next() as usize) % strong_reps.len()];
                let _ = s;
                let mut seen2: HashMap<u64, u32> = HashMap::new();
                let mut dq2 = VecDeque::new();
                seen2.insert(start, 0);
                dq2.push_back(start);
                let mut expanded = 0u64;
                let mut found = None;
                'bfs: while let Some(u) = dq2.pop_front() {
                    expanded += 1; // one neighbour computation (all ℓ) at u
                    let du = seen2[&u];
                    for v in adj(u) {
                        if let std::collections::hash_map::Entry::Vacant(e) = seen2.entry(v) {
                            let dv = du + 1;
                            e.insert(dv);
                            if is_weak(v) {
                                found = Some(dv);
                                break 'bfs;
                            }
                            dq2.push_back(v);
                        }
                    }
                }
                match found {
                    Some(d) => {
                        srch_expanded.push(expanded);
                        srch_len.push(d as u64);
                    }
                    None => srch_fail += 1,
                }
            }
            if take > 0 {
                search_rows.push(json!([key.0, key.1, take]));
            }
        }
        let mean = |v: &Vec<u64>| -> Value {
            if v.is_empty() {
                Value::Null
            } else {
                json!(v.iter().sum::<u64>() as f64 / v.len() as f64)
            }
        };
        let maxv = |v: &Vec<u64>| -> Value { v.iter().max().map_or(Value::Null, |x| json!(x)) };
        out_models.insert(
            mname.to_string(),
            json!({
                "mixed_classes": mixed.len(),
                "strong_curves_in_mixed": strong_curves,
                "unreachable_strong_curves": unreachable_curves,
                "distance_hist_curves": dist_hist,
                "components_total": comps_total,
                "components_without_weak": comps_without_weak,
                "class_rows_schema": ["twist","t","size","weak_curves","orbit_reps","components","components_without_weak","max_distance","unreachable_strong_curves"],
                "class_rows": class_rows,
                "attacker_search": {
                    "starts": srch_expanded.len() as u64 + srch_fail,
                    "found": srch_expanded.len(),
                    "failed_unreachable": srch_fail,
                    "mean_expanded_curves": mean(&srch_expanded),
                    "max_expanded_curves": maxv(&srch_expanded),
                    "mean_path_isogenies": mean(&srch_len),
                    "max_path_isogenies": maxv(&srch_len),
                    "path_len_hist": srch_len.iter().fold(BTreeMap::new(), |mut m: BTreeMap<String,u64>, &x| { *m.entry(x.to_string()).or_default() += 1; m }),
                    "expanded_hist": srch_expanded.iter().fold(BTreeMap::new(), |mut m: BTreeMap<String,u64>, &x| { *m.entry(x.to_string()).or_default() += 1; m }),
                },
            }),
        );
    }
    let kmuls = f.muls.get() - muls_before;
    json!({
        "N": nn,
        "modulus_hex": format!("{:#x}", f.modp),
        "ells": ells,
        "samples_per_class": samples,
        "graph_validation": {
            "codomain_trace_mismatch": trace_fail,
            "kernel_orbit_errors": orbit_errors,
            "asymmetric_edges": asym_edges,
        },
        "kernel_count_hist": kernel_hist,
        "kernel_searches": kernel_searches,
        "field_muls_in_graph_build_and_census": kmuls,
        "field_muls_per_kernel_search": if kernel_searches > 0 { json!(kmuls as f64 / kernel_searches as f64) } else { Value::Null },
        "models": out_models,
    })
}

// ── selftest ────────────────────────────────────────────────────────────

fn selftest() -> Value {
    let mut checks = serde_json::Map::new();
    // 1. point counts: WHT vs naive for N = 6..=10, all b, both twists
    let mut pc_ok = 0u64;
    let mut pc_bad = 0u64;
    for nn in 6..=10u32 {
        let f = Fld::new(nn);
        let t0 = all_traces(&f);
        let w = f.omega();
        for b in 1..(1u64 << nn) {
            let q = 1i64 << nn;
            let e0 = naive_count(&f, 0, b) as i64;
            let e1 = naive_count(&f, w, b) as i64;
            let ok = e0 == q + 1 - t0[b as usize] as i64 && e1 == q + 1 + t0[b as usize] as i64;
            if ok {
                pc_ok += 1;
            } else {
                pc_bad += 1;
            }
        }
    }
    checks.insert(
        "wht_vs_naive_N6_10".into(),
        json!({"ok": pc_ok, "bad": pc_bad}),
    );
    // 2. against the library point count, N = 12, 16 (sampled b)
    let mut lib_ok = 0;
    let mut lib_bad = 0;
    for nn in [12u32, 16] {
        let f = Fld::new(nn);
        let t0 = all_traces(&f);
        let gf = Gf2::new(&f.irr());
        let ash = ArtinSchreier::new(&gf);
        let mut rng = Rng(12345 + nn as u64);
        for _ in 0..40 {
            let b = rng.next() % ((1u64 << nn) - 1) + 1;
            let c = binary_point_count(&gf, &ash, 0, b) as i64;
            if c == (1i64 << nn) + 1 - t0[b as usize] as i64 {
                lib_ok += 1;
            } else {
                lib_bad += 1;
            }
        }
    }
    checks.insert(
        "wht_vs_library_count_N12_16".into(),
        json!({"ok": lib_ok, "bad": lib_bad}),
    );
    // 3. hand-checkable magic numbers at N = 12
    let f = Fld::new(12);
    let mut hand = Vec::new();
    for (l, n) in factorisations(12) {
        let mg = magic(&f, 1, l, n);
        hand.push(json!({"case": "b=1", "l": l, "n": n, "m": mg.m, "g": mg.genus}));
        assert_eq!((mg.m, mg.genus), (1, 1), "b = 1 must give m = 1, g = 1");
    }
    // b in k = F_{2^l}\F_2 for l = 4: m = 1, g = 1
    let mut rng = Rng(77);
    let in_k4 = loop {
        let x = rng.next() & 0xfff;
        if x > 1 && f.pow2k(x, 4) == x {
            break x;
        }
    };
    let mg = magic(&f, in_k4, 4, 3);
    assert_eq!((mg.m, mg.genus), (1, 1));
    hand.push(json!({"case": "b in F_16, (l,n)=(4,3)", "m": mg.m, "g": mg.genus}));
    // √b = c + σ(c) with c generic, (l,n) = (4,3): Ord = x²+x+1, m' = 2, m = 3, g = 3
    let mut found = false;
    for _ in 0..200 {
        let c = rng.next() & 0xfff;
        let s = c ^ f.pow2k(c, 4);
        if s == 0 || f.pow2k(s, 4) == s {
            continue;
        }
        let b = f.sqr(s);
        let mg = magic(&f, b, 4, 3);
        assert_eq!((mg.m_prime, mg.f_at_1, mg.m, mg.genus), (2, 1, 3, 3));
        hand.push(json!({"case": "sqrt(b)=c+σ(c), (l,n)=(4,3)", "m_prime": mg.m_prime, "m": mg.m, "g": mg.genus}));
        found = true;
        break;
    }
    assert!(found);
    // generic b at (l,n)=(4,3): m = 3, g = 4; at (6,2): m = 2, g = 2; (3,4): m = 4, g = 8
    let mut generic = BTreeMap::new();
    for _ in 0..200 {
        let b = rng.next() & 0xfff;
        if b == 0 {
            continue;
        }
        for (l, n) in [(4u32, 3u32), (6, 2), (3, 4)] {
            let mg = magic(&f, b, l, n);
            *generic
                .entry(format!("l{}n{}_m{}_g{}", l, n, mg.m, mg.genus))
                .or_insert(0u64) += 1;
        }
    }
    checks.insert("hand_cases_N12".into(), json!(hand));
    checks.insert("random_b_N12_m_g_hist".into(), json!(generic));
    // 4. m' (√b-orbit rank) against ec_trapdoor::magic_number; record the library genus
    let irr = f.irr();
    let mut lib_mprime_ok = 0;
    let mut lib_mprime_bad = 0;
    let mut genus_cmp: BTreeMap<String, u64> = BTreeMap::new();
    for _ in 0..300 {
        let b = (rng.next() & 0xfff).max(1);
        let a = F2mElement::zero(12);
        let be = elt(b, 12);
        for (l, n) in factorisations(12) {
            let tower = FieldTower::new(12, n, l, irr.clone());
            let lm = magic_number(&tower, &a, &be);
            let mg = magic(&f, b, l, n);
            if lm == mg.m_prime {
                lib_mprime_ok += 1;
            } else {
                lib_mprime_bad += 1;
            }
        }
        for row in audit_curve(12, &irr, &a, &be) {
            let mg = magic(&f, b, row.l, row.n);
            let lg: u64 = row.genus.to_u64_digits().first().copied().unwrap_or(0);
            *genus_cmp
                .entry(format!(
                    "lib(m={},g={}) ours(m={},g={})",
                    row.magic_m, lg, mg.m, mg.genus
                ))
                .or_default() += 1;
        }
    }
    checks.insert(
        "mprime_vs_library_magic_number".into(),
        json!({"ok": lib_mprime_ok, "bad": lib_mprime_bad}),
    );
    checks.insert("library_audit_curve_vs_ours_N12".into(), json!(genus_cmp));
    // 5. division polynomials against the library (ℓ = 3, 5, 7)
    let mut dp_ok = 0;
    let mut dp_bad = 0;
    for _ in 0..10 {
        let b = (rng.next() & 0xfff).max(1);
        for ell in [3u32, 5, 7] {
            let ours = division_poly(&f, b, ell);
            let lib = division_polynomial(&elt(b, 12), ell, 12, &irr);
            let ld = lib.degree().map_or(-1, |d| d as isize);
            let same =
                ld == deg(&ours) && (0..ours.len()).all(|i| to_u64(&lib.coeff(i)) == ours[i]);
            if same {
                dp_ok += 1;
            } else {
                dp_bad += 1;
            }
        }
    }
    checks.insert(
        "division_poly_vs_library".into(),
        json!({"ok": dp_ok, "bad": dp_bad}),
    );
    // 6. ℓ = 3 neighbours against Φ₃ roots (library), j' = 1/b'
    let mut phi_ok = 0;
    let mut phi_bad = 0;
    let mut r2 = Rng(4242);
    for _ in 0..12 {
        let b = (r2.next() & 0xfff).max(1);
        let (ts, _) = rational_kernels(&f, b, 3, &mut r2);
        let mut ours: Vec<u64> = ts
            .iter()
            .map(|&t| f.inv(b ^ t ^ f.sqr(t)))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        ours.sort_unstable();
        let curve = ECurve::new(12, irr.clone(), F2mElement::zero(12), elt(b, 12));
        let mut lib: Vec<u64> = l_isogenous_neighbours(&curve, 3)
            .iter()
            .map(|e| to_u64(&j_invariant(e)))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        lib.sort_unstable();
        if ours == lib {
            phi_ok += 1;
        } else {
            phi_bad += 1;
            eprintln!("phi3 mismatch b={b:#x}: ours {ours:x?} lib {lib:x?}");
        }
    }
    checks.insert(
        "ell3_neighbours_vs_phi3".into(),
        json!({"ok": phi_ok, "bad": phi_bad}),
    );
    // 7. the 2-isogeny (Kronecker: Φ₂ ≡ (X² − Y)(X − Y²) mod 2) is Galois
    //    conjugation; squaring b preserves m for every factorisation.
    let mut sq_same = 0;
    let mut sq_diff = 0;
    for _ in 0..200 {
        let b = (rng.next() & 0xfff).max(1);
        for (l, n) in factorisations(12) {
            if magic(&f, b, l, n).m == magic(&f, f.sqr(b), l, n).m {
                sq_same += 1;
            } else {
                sq_diff += 1;
            }
        }
    }
    checks.insert(
        "m_invariant_under_b_to_b2".into(),
        json!({"same": sq_same, "diff": sq_diff}),
    );
    Value::Object(checks)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut out_path: Option<String> = None;
    let mut rest = Vec::new();
    let mut i = 1;
    let mut samples = 20usize;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_path = Some(args[i + 1].clone());
                i += 2;
            }
            "--samples" => {
                samples = args[i + 1].parse().unwrap();
                i += 2;
            }
            _ => {
                rest.push(args[i].clone());
                i += 1;
            }
        }
    }
    let cmd = rest.first().cloned().unwrap_or_else(|| "selftest".into());
    let nums: Vec<u32> = rest.iter().skip(1).map(|s| s.parse().unwrap()).collect();
    let out = match cmd.as_str() {
        "selftest" => selftest(),
        "census" => Value::Array(
            nums.iter()
                .map(|&nn| {
                    eprintln!("census N={nn}");
                    census(nn)
                })
                .collect(),
        ),
        "walk" => Value::Array(
            nums.iter()
                .map(|&nn| {
                    eprintln!("walk N={nn}");
                    walk(nn, &[3, 5, 7], samples)
                })
                .collect(),
        ),
        _ => panic!("usage: selftest | census N... | walk N... [--samples S] [--out path]"),
    };
    let s = serde_json::to_string_pretty(&out).unwrap();
    match out_path {
        Some(p) => fs::write(p, s).unwrap(),
        None => println!("{s}"),
    }
}
