//! **Lead R6.5: the finite-size A-pair correction applied back to the §15.2
//! j = 0 census.**
//!
//! Round-4 lead 16.6 derived, for an A-closed factor base when the
//! automorphism group `A` acts trivially on the cofactor classes, a
//! correction `1 + (h−1)(|A|−2)/(2|F|)` to the m = 2 class factor, and noted
//! it applies to the j = 0 census of `RESEARCH_ISOGENY_FIBERS_ECDLP.md` §15.2
//! (`round3/census_j0_*.json`, produced by `automorphism_vs_translation.py`).
//! This example
//!
//! * `reanalyse`: reads the recorded census JSON files (no rerun), extracts
//!   the plain and ζ-closed m = 2, 3 counts, computes the correction with
//!   `|A| = 3` and `|A| = 6`, the residuals before and after, and — from the
//!   recorded curve parameters alone — rebuilds the two small-x bases to read
//!   off their cofactor-class composition `n₀`, which fixes the exact
//!   collision-free pair count `H₂ = g₂(n₀) − |F|/2`;
//! * `census`: the §15.2 census (exact `D_m` over all targets in `G`, m = 2, 3,
//!   signed bases, columns = orbits of `[h]P` under `⟨−1, ζ⟩`) on one recorded
//!   curve, for `|F| ∈ {36, 72, 144}`, on the deterministic small-x bases and
//!   on an ensemble of random bases, so that the excess of the ζ-closed base
//!   over the plain one can be read as a function of `1/|F|` with intervals.
//!
//! Derivation used throughout (h = 3, `A = ⟨−1, ζ⟩`, ζ trivial on `E[3](F_p)`):
//! for a signed base the number of m-multisets whose cofactor-class sum is 0
//! is a function `g_m(n₀)` of the number `n₀` of base points lying in `G`
//! alone, because negation pairs the nonzero classes (`n₁ = n₂`). A plain base
//! has `n₀ = 2·Bin(|F|/2, 1/3)`, a ζ-closed base `n₀ = 6·Bin(|F|/6, 1/3)`:
//! same mean, three times the variance. `g₂` is convex, so the ζ-closed base
//! gains in expectation `g₂''/2 · ΔVar = (3/4)·(8|F|/9) = 2|F|/3` pair hits on
//! a baseline of `|F|²/6`: ratio `1 + 4/|F|`, which is 16.6's formula with
//! `|A| = 6`. The same count arrives pair by pair: the hits that are
//! deterministic are `{ζᵃP, −ζᵇP}`, `a ≠ b`, whose class is
//! `(χ(ζᵃ) − χ(ζᵇ))·cls P = 0` for every `P`; they need both signs, which is
//! why negation counts in `|A|`.
//!
//! ```text
//! cargo run --release --example r6_5_census_correction -- reanalyse \
//!     --json research/isogeny_fibers_20261010/round3/census_j0_p18.json,... --out out.json
//! cargo run --release --example r6_5_census_correction -- census --p 195709 --b 3 \
//!     --sizes 36,72,144 --reps 200 --seed 1 --out census.json
//! ```

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;
use serde_json::Value;

// ── F_p and the curve y² = x³ + b ─────────────────────────────────────────

fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

fn powmod(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, p);
        }
        a = mulmod(a, a, p);
        e >>= 1;
    }
    r
}

/// Deterministic Miller–Rabin for u64.
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for q in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n.is_multiple_of(q) {
            return n == q;
        }
    }
    let mut d = n - 1;
    let mut s = 0;
    while d.is_multiple_of(2) {
        d /= 2;
        s += 1;
    }
    'outer: for a in [2u64, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

/// A point; `None` is the point at infinity.
type Pt = Option<(u64, u64)>;

#[derive(Clone)]
struct J0 {
    p: u64,
    b: u64,
    /// A primitive cube root of unity.
    omega: u64,
}

impl J0 {
    fn new(p: u64, b: u64, omega: Option<u64>) -> J0 {
        assert!(is_prime(p) && p % 3 == 1, "p must be a prime ≡ 1 (mod 3)");
        let omega = omega.unwrap_or_else(|| {
            // as in automorphism_vs_translation.py: smallest g with g^((p−1)/3) ≠ 1
            let mut g = 2;
            while powmod(g, (p - 1) / 3, p) == 1 {
                g += 1;
            }
            powmod(g, (p - 1) / 3, p)
        });
        assert!(
            omega != 1 && powmod(omega, 3, p) == 1,
            "omega is not a primitive cube root"
        );
        J0 { p, b, omega }
    }
    fn key(&self, pt: &Pt) -> u64 {
        match pt {
            None => u64::MAX,
            Some((x, y)) => x * self.p + y,
        }
    }
    fn on(&self, pt: &Pt) -> bool {
        match pt {
            None => true,
            Some((x, y)) => {
                let p = self.p;
                mulmod(*y, *y, p) == (mulmod(mulmod(*x, *x, p), *x, p) + self.b) % p
            }
        }
    }
    fn neg(&self, pt: &Pt) -> Pt {
        pt.map(|(x, y)| (x, (self.p - y) % self.p))
    }
    fn zeta(&self, pt: &Pt) -> Pt {
        pt.map(|(x, y)| (mulmod(self.omega, x, self.p), y))
    }
    fn add(&self, a: &Pt, b: &Pt) -> Pt {
        let p = self.p;
        let (x1, y1) = match a {
            None => return *b,
            Some(v) => *v,
        };
        let (x2, y2) = match b {
            None => return *a,
            Some(v) => *v,
        };
        let lam = if x1 == x2 {
            if (y1 + y2) % p == 0 {
                return None;
            }
            let num = mulmod(3, mulmod(x1, x1, p), p);
            mulmod(num, powmod((2 * y1) % p, p - 2, p), p)
        } else {
            let num = (y2 + p - y1) % p;
            mulmod(num, powmod((x2 + p - x1) % p, p - 2, p), p)
        };
        let x3 = (mulmod(lam, lam, p) + 2 * p - x1 - x2) % p;
        let y3 = (mulmod(lam, (x1 + p - x3) % p, p) + p - y1) % p;
        Some((x3, y3))
    }
    fn mul(&self, mut k: u64, pt: &Pt) -> Pt {
        let mut r: Pt = None;
        let mut q = *pt;
        while k > 0 {
            if k & 1 == 1 {
                r = self.add(&r, &q);
            }
            q = self.add(&q, &q);
            k >>= 1;
        }
        r
    }
    /// Legendre symbol of a: 1, −1 or 0.
    fn legendre(&self, a: u64) -> i64 {
        let a = a % self.p;
        if a == 0 {
            return 0;
        }
        if powmod(a, (self.p - 1) / 2, self.p) == 1 {
            1
        } else {
            -1
        }
    }
    fn rhs(&self, x: u64) -> u64 {
        let p = self.p;
        (mulmod(mulmod(x, x, p), x, p) + self.b) % p
    }
    /// Tonelli–Shanks square root.
    fn sqrt(&self, a: u64) -> Option<u64> {
        let p = self.p;
        let a = a % p;
        if a == 0 {
            return Some(0);
        }
        if self.legendre(a) != 1 {
            return None;
        }
        if p % 4 == 3 {
            return Some(powmod(a, (p + 1) / 4, p));
        }
        let mut q = p - 1;
        let mut s = 0;
        while q.is_multiple_of(2) {
            q /= 2;
            s += 1;
        }
        let mut z = 2;
        while self.legendre(z) != -1 {
            z += 1;
        }
        let mut m = s;
        let mut c = powmod(z, q, p);
        let mut t = powmod(a, q, p);
        let mut r = powmod(a, q.div_ceil(2), p);
        while t != 1 {
            let mut i = 0;
            let mut tt = t;
            while tt != 1 {
                tt = mulmod(tt, tt, p);
                i += 1;
            }
            let mut bb = c;
            for _ in 0..(m - i - 1) {
                bb = mulmod(bb, bb, p);
            }
            m = i;
            c = mulmod(bb, bb, p);
            t = mulmod(t, c, p);
            r = mulmod(r, bb, p);
        }
        Some(r)
    }
    /// Both points with abscissa x (one if y = 0, none if x³ + b is a non-residue).
    fn lift(&self, x: u64) -> Vec<(u64, u64)> {
        match self.sqrt(self.rhs(x)) {
            None => vec![],
            Some(0) => vec![(x, 0)],
            Some(s) => vec![(x, s), (x, self.p - s)],
        }
    }
    /// #E(F_p) by the Legendre sum.
    fn order(&self) -> u64 {
        let mut n: i64 = self.p as i64 + 1;
        for x in 0..self.p {
            n += self.legendre(self.rhs(x));
        }
        n as u64
    }
    /// The small-x base of `automorphism_vs_translation.py`: points with
    /// x = 1, 2, … until `k` points are collected (k even).
    fn small_x(&self, k: usize) -> Vec<(u64, u64)> {
        let mut pts = Vec::new();
        let mut x = 1;
        while pts.len() < k {
            pts.extend(self.lift(x));
            x += 1;
        }
        pts.truncate(k);
        assert!(pts.len().is_multiple_of(2), "odd small-x base");
        pts
    }
    fn zeta_close(&self, s: &[(u64, u64)]) -> Vec<(u64, u64)> {
        let mut out: Vec<(u64, u64)> = s.to_vec();
        out.extend(s.iter().map(|q| self.zeta(&Some(*q)).unwrap()));
        out.extend(s.iter().map(|q| self.zeta(&self.zeta(&Some(*q))).unwrap()));
        out
    }
}

// ── composition counts (derived) ──────────────────────────────────────────

/// Pairs (with repetition) of a signed base of size f with n₀ points in G
/// whose class sum is 0, when h = 3 (classes ±1 are paired by negation).
fn g2(n0: u64, f: u64) -> u64 {
    let n1 = (f - n0) / 2;
    n0 * (n0 + 1) / 2 + n1 * n1
}

fn c3(n: u64) -> u64 {
    // C(n + 2, 3): multisets of size 3 from n elements
    (n + 2) * (n + 1) * n / 6
}

/// Triples (with repetition) with class sum 0, h = 3.
fn g3(n0: u64, f: u64) -> u64 {
    let n1 = (f - n0) / 2;
    c3(n0) + n0 * n1 * n1 + 2 * c3(n1)
}

/// Binomial(n, 1/3) probabilities.
fn binom_third(n: u64) -> Vec<f64> {
    let mut v = vec![0f64; n as usize + 1];
    for (k, slot) in v.iter_mut().enumerate() {
        let k = k as u64;
        let mut lc = 0f64;
        for i in 0..k {
            lc += ((n - i) as f64).ln() - ((i + 1) as f64).ln();
        }
        *slot = (lc + k as f64 * (1f64 / 3f64).ln() + (n - k) as f64 * (2f64 / 3f64).ln()).exp();
    }
    v
}

/// Derived expectations under the composition model for a base of size f
/// whose `A`-orbits have size `a` (2 for plain, 6 for ζ-closed):
/// `(E[H₂], E[g₃], Var n₀)` with `H₂ = g₂(n₀) − f/2` (the `f/2` pairs `{P, −P}`).
fn expected_counts(f: u64, a: u64) -> (f64, f64, f64) {
    let pr = binom_third(f / a);
    let mut eh2 = 0f64;
    let mut eg3 = 0f64;
    let mut var = 0f64;
    let mean = f as f64 / 3.0;
    for (k, &q) in pr.iter().enumerate() {
        let n0 = a * k as u64;
        eh2 += q * (g2(n0, f) as f64 - f as f64 / 2.0);
        eg3 += q * g3(n0, f) as f64;
        var += q * (n0 as f64 - mean).powi(2);
    }
    (eh2, eg3, var)
}

/// Derived: mean and standard deviation of the single-base ratio
/// `H₂(ζ-closed) / H₂(plain)` under independent compositions, and the
/// probability that the ratio is below 1 (collisions ignored).
fn ratio_distribution(f: u64) -> (f64, f64, f64) {
    let pz = binom_third(f / 6);
    let pp = binom_third(f / 2);
    let (mut m1, mut m2, mut below) = (0f64, 0f64, 0f64);
    for (kz, &qz) in pz.iter().enumerate() {
        let hz = g2(6 * kz as u64, f) as f64 - f as f64 / 2.0;
        for (kp, &qp) in pp.iter().enumerate() {
            let hp = g2(2 * kp as u64, f) as f64 - f as f64 / 2.0;
            let r = hz / hp;
            m1 += qz * qp * r;
            m2 += qz * qp * r * r;
            if r < 1.0 {
                below += qz * qp;
            }
        }
    }
    (m1, (m2 - m1 * m1).max(0.0).sqrt(), below)
}

/// 16.6's correction `1 + (h−1)(|A|−2)/(2|F|)`.
fn correction(h: u64, a: u64, f: u64) -> f64 {
    1.0 + (h - 1) as f64 * (a - 2) as f64 / (2.0 * f as f64)
}

// ── the census ────────────────────────────────────────────────────────────

#[derive(Serialize, Clone)]
struct MStat {
    /// Multisets of m base points whose sum is a nonzero point of G
    /// (for m = 3, excluding the degenerate {P, Q, −Q}).
    representations: u64,
    /// Multisets summing to O.
    zero_sums: u64,
    /// Distinct nonzero points of G hit.
    distinct: u64,
    hit_rate: f64,
    /// `D_m · m! / |F|^m`.
    c: f64,
    /// `U · (r / D_m) · L_m`, L₂ = 1, L₃ = |F|.
    cost_model: f64,
}

#[derive(Serialize, Clone)]
struct Census {
    arm: String,
    rep: String,
    points: usize,
    columns: usize,
    classes: usize,
    /// Base points lying in G (class 0).
    n0: u64,
    /// `g₂(n₀)`, `g₃(n₀)`: class-sum-0 multisets predicted by the composition.
    g2: u64,
    g3: u64,
    /// `representations + zero_sums == g_m(n₀)` for m = 2 and 3.
    identity_m2: bool,
    identity_m3: bool,
    /// m = 3 multisets {P, Q, −Q} (a pair summing to O), skipped as in the
    /// legacy census (`if s is None: continue`).
    degenerate_m3: u64,
    m2: MStat,
    m3: MStat,
}

struct Ctx<'a> {
    e: &'a J0,
    r: u64,
    h: u64,
}

fn census(cx: &Ctx, arm: &str, rep: &str, base: &[(u64, u64)]) -> Census {
    let e = cx.e;
    let n = base.len();
    let f = n as u64;
    let pts: Vec<Pt> = base.iter().map(|q| Some(*q)).collect();
    let keys: HashSet<u64> = pts.iter().map(|q| e.key(q)).collect();
    assert_eq!(keys.len(), n, "{arm}: duplicate base points");
    for q in &pts {
        assert!(e.on(q), "{arm}: point off the curve");
        assert!(keys.contains(&e.key(&e.neg(q))), "{arm}: base not signed");
    }
    let cls: Vec<Pt> = pts.iter().map(|q| e.mul(cx.r, q)).collect();
    let classes: HashSet<u64> = cls.iter().map(|q| e.key(q)).collect();
    let n0 = cls.iter().filter(|c| c.is_none()).count() as u64;
    // columns: orbits of [h]P under ⟨−1, ζ⟩
    let mut seen: HashSet<u64> = HashSet::new();
    let mut cols = 0usize;
    for q in &pts {
        let hq = e.mul(cx.h, q);
        if hq.is_none() || seen.contains(&e.key(&hq)) {
            continue;
        }
        cols += 1;
        let mut z = hq;
        for _ in 0..3 {
            seen.insert(e.key(&z));
            seen.insert(e.key(&e.neg(&z)));
            z = e.zeta(&z);
        }
    }
    // pairs keyed by class sum
    let mut pairs: HashMap<u64, Vec<(usize, usize, Pt)>> = HashMap::new();
    let zero_key = e.key(&None);
    let mut hits2: HashSet<u64> = HashSet::new();
    let (mut reps2, mut zero2) = (0u64, 0u64);
    for i in 0..n {
        for j in i..n {
            let s = e.add(&pts[i], &pts[j]);
            let ck = e.key(&e.add(&cls[i], &cls[j]));
            if ck == zero_key {
                if s.is_none() {
                    zero2 += 1;
                } else {
                    hits2.insert(e.key(&s));
                    reps2 += 1;
                }
            }
            pairs.entry(ck).or_default().push((i, j, s));
        }
    }
    let mut hits3: HashSet<u64> = HashSet::new();
    let (mut reps3, mut zero3, mut degen3) = (0u64, 0u64, 0u64);
    for i in 0..n {
        let need = e.key(&e.neg(&cls[i]));
        if let Some(v) = pairs.get(&need) {
            for (j, _k, s) in v {
                // Counts use each multiset once (i ≤ j ≤ k); the distinct-hit
                // set, like the legacy census, takes every split {P_i} ∪ pair,
                // so {P, Q, −Q} still hits P through the split Q + (P − Q).
                let counted = *j >= i;
                if s.is_none() {
                    // the legacy census skips pair sums equal to O
                    if counted {
                        degen3 += 1;
                    }
                    continue;
                }
                let t = e.add(s, &pts[i]);
                if t.is_none() {
                    if counted {
                        zero3 += 1;
                    }
                } else {
                    hits3.insert(e.key(&t));
                    if counted {
                        reps3 += 1;
                    }
                }
            }
        }
    }
    let mk = |reps: u64, zero: u64, d: usize, fact: f64, m: i32, l: f64| MStat {
        representations: reps,
        zero_sums: zero,
        distinct: d as u64,
        hit_rate: d as f64 / cx.r as f64,
        c: d as f64 * fact / (n as f64).powi(m),
        cost_model: if d > 0 {
            cols as f64 * (cx.r as f64 / d as f64) * l
        } else {
            f64::NAN
        },
    };
    let (gg2, gg3) = (g2(n0, f), g3(n0, f));
    Census {
        arm: arm.to_string(),
        rep: rep.to_string(),
        points: n,
        columns: cols,
        classes: classes.len(),
        n0,
        g2: gg2,
        g3: gg3,
        identity_m2: reps2 + zero2 == gg2,
        identity_m3: reps3 + zero3 + degen3 == gg3,
        degenerate_m3: degen3,
        m2: mk(reps2, zero2, hits2.len(), 2.0, 2, 1.0),
        m3: mk(reps3, zero3, hits3.len(), 6.0, 3, n as f64),
    }
}

// ── random bases ──────────────────────────────────────────────────────────

/// `f/2` random abscissae with points (x ≠ 0, so the 3-torsion points
/// (0, ±√b) are excluded as in the small-x base), both signs.
fn random_plain(e: &J0, rng: &mut StdRng, f: usize) -> Vec<(u64, u64)> {
    let mut xs: HashSet<u64> = HashSet::new();
    let mut out = Vec::new();
    while out.len() < f {
        let x = rng.gen_range(1..e.p);
        if xs.contains(&x) {
            continue;
        }
        let l = e.lift(x);
        if l.len() == 2 {
            xs.insert(x);
            out.extend(l);
        }
    }
    out
}

/// `f/6` random ζ-orbits of abscissae, closed under ζ and negation.
fn random_zeta(e: &J0, rng: &mut StdRng, f: usize) -> Vec<(u64, u64)> {
    assert!(f.is_multiple_of(6));
    let mut xs: HashSet<u64> = HashSet::new();
    let mut s = Vec::new();
    while s.len() < f / 3 {
        let x = rng.gen_range(1..e.p);
        if xs.contains(&x) {
            continue;
        }
        let l = e.lift(x);
        if l.len() == 2 {
            xs.insert(x);
            xs.insert(mulmod(e.omega, x, e.p));
            xs.insert(mulmod(mulmod(e.omega, e.omega, e.p), x, e.p));
            s.extend(l);
        }
    }
    e.zeta_close(&s)
}

// ── statistics ────────────────────────────────────────────────────────────

#[derive(Serialize, Clone)]
struct Stat {
    /// Ratio of means over the random bases (ζ-closed / plain).
    value: f64,
    /// 95% percentile bootstrap over bases, 2000 resamples, arms resampled independently.
    ci95: (f64, f64),
    /// The same ratio on the deterministic small-x bases of §15.2.
    smallx: f64,
    /// Mean and standard deviation over reps of the per-base ratio
    /// `ζ(i) / plain(i)` (the resolution of one census, as in round 3).
    single_mean: f64,
    single_sd: f64,
}

fn mean(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

fn ratio_stat(z: &[f64], pl: &[f64], z_small: f64, p_small: f64, rng: &mut StdRng) -> Stat {
    let value = mean(z) / mean(pl);
    let mut bs: Vec<f64> = (0..2000)
        .map(|_| {
            let mz: f64 = (0..z.len())
                .map(|_| z[rng.gen_range(0..z.len())])
                .sum::<f64>()
                / z.len() as f64;
            let mp: f64 = (0..pl.len())
                .map(|_| pl[rng.gen_range(0..pl.len())])
                .sum::<f64>()
                / pl.len() as f64;
            mz / mp
        })
        .collect();
    bs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let single: Vec<f64> = z.iter().zip(pl).map(|(a, b)| a / b).collect();
    let sm = mean(&single);
    let sd =
        (single.iter().map(|x| (x - sm).powi(2)).sum::<f64>() / (single.len() as f64 - 1.0)).sqrt();
    Stat {
        value,
        ci95: (bs[49], bs[1949]),
        smallx: z_small / p_small,
        single_mean: sm,
        single_sd: sd,
    }
}

#[derive(Serialize)]
struct SizeSummary {
    size: u64,
    n_reps: usize,
    inv_size: f64,
    /// Derived: 16.6's correction with |A| = 6 and with |A| = 3.
    predicted_a6: f64,
    predicted_a3: f64,
    /// Derived from the composition model: E[H₂ ratio] (= 1 + 4/|F| exactly),
    /// the m = 3 raw-multiset ratio `(E[g₃^ζ] − |F|/3) / E[g₃^plain]`
    /// (the `|F|/3` are the structured O-triples `{ζᵃP, ζᵇP, ζᶜP}`), and the
    /// single-census mean / sd / P(ratio < 1) of the H₂ ratio.
    derived_h2_ratio: f64,
    derived_h3_ratio: f64,
    derived_single_mean: f64,
    derived_single_sd: f64,
    derived_prob_below_1: f64,
    /// Measured ratios ζ-closed / plain.
    d2: Stat,
    h2: Stat,
    d3: Stat,
    h3: Stat,
    columns: Stat,
    cost2: Stat,
    cost3: Stat,
    /// Means over random bases (plain, ζ-closed).
    mean_hit_rate3: (f64, f64),
    mean_n0: (f64, f64),
    identities_ok: bool,
    smallx_n0: (u64, u64),
}

#[derive(Serialize)]
struct CensusOut {
    p: u64,
    b: u64,
    omega: u64,
    order: u64,
    r: u64,
    h: u64,
    r_is_prime: bool,
    /// The rational 3-torsion point (0, √b) and whether ζ fixes it.
    t3: (u64, u64),
    zeta_fixes_t3: bool,
    three_t3_is_zero: bool,
    seed: u64,
    reps: usize,
    summary: Vec<SizeSummary>,
    census: Vec<Census>,
}

fn run_census(p: u64, b: u64, sizes: &[u64], reps: usize, seed: u64) -> CensusOut {
    let e = J0::new(p, b, None);
    let order = e.order();
    assert!(
        order.is_multiple_of(3) && !order.is_multiple_of(9),
        "N must be 3·r"
    );
    let r = order / 3;
    let h = 3;
    let r_is_prime = is_prime(r);
    assert!(r_is_prime, "r = N/3 must be prime");
    let t3 = {
        let l = e.lift(0);
        assert_eq!(
            l.len(),
            2,
            "x = 0 must lift (b a square) for a rational 3-torsion point"
        );
        l[0]
    };
    let t3p: Pt = Some(t3);
    let three_t3_is_zero = e.mul(3, &t3p).is_none();
    let zeta_fixes_t3 = e.zeta(&t3p) == t3p;
    let cx = Ctx { e: &e, r, h };
    let mut rng = StdRng::seed_from_u64(seed);
    let mut all = Vec::new();
    let mut summary = Vec::new();
    for &f in sizes {
        assert!(f.is_multiple_of(6));
        let fu = f as usize;
        let sp = census(&cx, "plain", "smallx", &e.small_x(fu));
        let sz = census(&cx, "zeta", "smallx", &e.zeta_close(&e.small_x(fu / 3)));
        let mut cp = Vec::new();
        let mut cz = Vec::new();
        for i in 0..reps {
            cp.push(census(
                &cx,
                "plain",
                &format!("rand{i}"),
                &random_plain(&e, &mut rng, fu),
            ));
            cz.push(census(
                &cx,
                "zeta",
                &format!("rand{i}"),
                &random_zeta(&e, &mut rng, fu),
            ));
        }
        let identities_ok = [&sp, &sz]
            .into_iter()
            .chain(cp.iter())
            .chain(cz.iter())
            .all(|c| c.identity_m2 && c.identity_m3);
        let col =
            |v: &[Census], g: &dyn Fn(&Census) -> f64| -> Vec<f64> { v.iter().map(g).collect() };
        let mut st = |g: &dyn Fn(&Census) -> f64| {
            ratio_stat(&col(&cz, g), &col(&cp, g), g(&sz), g(&sp), &mut rng)
        };
        let d2 = st(&|c| c.m2.distinct as f64);
        let h2 = st(&|c| c.m2.representations as f64);
        let d3 = st(&|c| c.m3.distinct as f64);
        let h3 = st(&|c| c.m3.representations as f64);
        let columns = st(&|c| c.columns as f64);
        let cost2 = st(&|c| c.m2.cost_model);
        let cost3 = st(&|c| c.m3.cost_model);
        let (eh2p, eg3p, _) = expected_counts(f, 2);
        let (eh2z, eg3z, _) = expected_counts(f, 6);
        let (sm, sd, below) = ratio_distribution(f);
        summary.push(SizeSummary {
            size: f,
            n_reps: reps,
            inv_size: 1.0 / f as f64,
            predicted_a6: correction(h, 6, f),
            predicted_a3: correction(h, 3, f),
            derived_h2_ratio: eh2z / eh2p,
            derived_h3_ratio: (eg3z - f as f64 / 3.0) / eg3p,
            derived_single_mean: sm,
            derived_single_sd: sd,
            derived_prob_below_1: below,
            d2,
            h2,
            d3,
            h3,
            columns,
            cost2,
            cost3,
            mean_hit_rate3: (
                mean(&col(&cp, &|c| c.m3.hit_rate)),
                mean(&col(&cz, &|c| c.m3.hit_rate)),
            ),
            mean_n0: (
                mean(&col(&cp, &|c| c.n0 as f64)),
                mean(&col(&cz, &|c| c.n0 as f64)),
            ),
            identities_ok,
            smallx_n0: (sp.n0, sz.n0),
        });
        all.push(sp);
        all.push(sz);
        all.extend(cp);
        all.extend(cz);
        eprintln!("size {f}: done ({} censuses)", 2 * reps + 2);
    }
    CensusOut {
        p,
        b,
        omega: e.omega,
        order,
        r,
        h,
        r_is_prime,
        t3,
        zeta_fixes_t3,
        three_t3_is_zero,
        seed,
        reps,
        summary,
        census: all,
    }
}

// ── re-analysis of the recorded census files ──────────────────────────────

#[derive(Serialize)]
struct RecordedArm {
    points: u64,
    columns: u64,
    d2: u64,
    c2: f64,
    d3: u64,
    c3: f64,
    /// Measured here: the census rerun in Rust on the rebuilt base, as a check
    /// that this implementation reproduces the recorded legacy census exactly.
    d2_recomputed: u64,
    d3_recomputed: u64,
    columns_recomputed: usize,
    reproduced: bool,
    /// Reconstructed from (p, b, ω): the base's class composition.
    n0_reconstructed: u64,
    class0_orbits_reconstructed: u64,
    columns_reconstructed: usize,
    /// `g₂(n₀) − |F|/2`: the collision-free pair-hit count the composition fixes.
    h2_from_composition: u64,
    /// `D₂ ≤ H₂`; the gap is the number of coincident pair sums.
    collisions_m2_implied: i64,
    /// Expected coincidences if the H₂ sums were uniform in G: `H₂²/(2r)`.
    collisions_m2_uniform_model: f64,
    g3_from_composition: u64,
}

#[derive(Serialize)]
struct Recorded {
    file: String,
    p: u64,
    b: u64,
    n: u64,
    r: u64,
    h: u64,
    omega: u64,
    t: (u64, u64),
    /// Checks on the recorded parameters (measured here).
    order_recomputed: u64,
    order_matches: bool,
    r_is_prime: bool,
    omega_is_primitive_cube_root: bool,
    zeta_fixes_t: bool,
    three_t_is_zero: bool,
    plain: RecordedArm,
    zeta: RecordedArm,
    /// Measured ratios ζ-closed / plain (c and D give the same ratio at equal |F|).
    ratio_c2: f64,
    ratio_c3: f64,
    /// Derived corrections and residuals.
    predicted_a3: f64,
    predicted_a6: f64,
    residual_c2_before: f64,
    residual_c2_after_a3: f64,
    residual_c2_after_a6: f64,
    /// Ratio predicted from the two reconstructed compositions, collision-free.
    ratio_h2_from_compositions: f64,
    residual_c2_given_composition: f64,
    /// m = 3: ratio of raw class-sum-0 multisets from the compositions
    /// (after removing the ζ-closed base's `|F|/3` structured O-triples).
    ratio_h3_from_compositions: f64,
    derived_h3_ratio_expectation: f64,
    /// Composition model at this |F|: single-census mean, sd, P(ratio < 1).
    derived_single_mean: f64,
    derived_single_sd: f64,
    derived_prob_below_1: f64,
    z_before: f64,
    z_after_a6: f64,
}

fn arm(v: &Value, name: &str) -> (u64, u64, u64, f64, u64, f64) {
    let a = &v["bases"][name];
    let g = |k: &str| {
        a[k].as_u64()
            .unwrap_or_else(|| panic!("missing {name}.{k}"))
    };
    (
        g("points"),
        g("columns"),
        a["m2"]["D"].as_u64().unwrap(),
        a["m2"]["c"].as_f64().unwrap(),
        a["m3"]["D"].as_u64().unwrap(),
        a["m3"]["c"].as_f64().unwrap(),
    )
}

fn reanalyse(path: &str) -> Recorded {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let v: Value = serde_json::from_str(&text).unwrap();
    let cu = &v["curve"];
    let u = |k: &str| cu[k].as_u64().unwrap();
    let (p, b, n, r, h, omega) = (u("p"), u("b"), u("N"), u("r"), u("h"), u("omega"));
    let t = (cu["T"][0].as_u64().unwrap(), cu["T"][1].as_u64().unwrap());
    let e = J0::new(p, b, Some(omega));
    let order_recomputed = e.order();
    let tp: Pt = Some(t);
    let pl = arm(&v, "plain (small x)");
    let ze = arm(&v, "zeta-closed (automorphism orbits, a = 3)");
    assert_eq!(pl.0, ze.0, "bases of unequal size");
    let f = pl.0;
    let cx = Ctx { e: &e, r, h };
    let rebuild =
        |rec: (u64, u64, u64, f64, u64, f64), base: Vec<(u64, u64)>, orbit: u64| -> RecordedArm {
            assert_eq!(base.len() as u64, f);
            let pts: Vec<Pt> = base.iter().map(|q| Some(*q)).collect();
            let n0 = pts.iter().filter(|q| e.mul(r, q).is_none()).count() as u64;
            // columns exactly as the census counts them
            let mut seen: HashSet<u64> = HashSet::new();
            let mut cols = 0usize;
            for q in &pts {
                let hq = e.mul(cx.h, q);
                if hq.is_none() || seen.contains(&e.key(&hq)) {
                    continue;
                }
                cols += 1;
                let mut z = hq;
                for _ in 0..3 {
                    seen.insert(e.key(&z));
                    seen.insert(e.key(&e.neg(&z)));
                    z = e.zeta(&z);
                }
            }
            let h2 = g2(n0, f) - f / 2;
            let rc = census(&cx, "recomputed", "smallx", &base);
            RecordedArm {
                d2_recomputed: rc.m2.distinct,
                d3_recomputed: rc.m3.distinct,
                columns_recomputed: rc.columns,
                reproduced: rc.m2.distinct == rec.2
                    && rc.m3.distinct == rec.4
                    && rc.columns as u64 == rec.1,
                points: rec.0,
                columns: rec.1,
                d2: rec.2,
                c2: rec.3,
                d3: rec.4,
                c3: rec.5,
                n0_reconstructed: n0,
                class0_orbits_reconstructed: n0 / orbit,
                columns_reconstructed: cols,
                h2_from_composition: h2,
                collisions_m2_implied: h2 as i64 - rec.2 as i64,
                collisions_m2_uniform_model: (h2 as f64).powi(2) / (2.0 * r as f64),
                g3_from_composition: g3(n0, f),
            }
        };
    let plain = rebuild(pl, e.small_x(f as usize), 2);
    let zeta = rebuild(ze, e.zeta_close(&e.small_x(f as usize / 3)), 6);
    let ratio_c2 = ze.3 / pl.3;
    let ratio_c3 = ze.5 / pl.5;
    let (pa3, pa6) = (correction(h, 3, f), correction(h, 6, f));
    let (sm, sd, below) = ratio_distribution(f);
    let (_, eg3p, _) = expected_counts(f, 2);
    let (_, eg3z, _) = expected_counts(f, 6);
    let rh2 = zeta.h2_from_composition as f64 / plain.h2_from_composition as f64;
    let rh3 = (g3(zeta.n0_reconstructed, f) as f64 - f as f64 / 3.0)
        / g3(plain.n0_reconstructed, f) as f64;
    Recorded {
        file: path.to_string(),
        p,
        b,
        n,
        r,
        h,
        omega,
        t,
        order_recomputed,
        order_matches: order_recomputed == n,
        r_is_prime: is_prime(r) && n == 3 * r,
        omega_is_primitive_cube_root: omega != 1 && powmod(omega, 3, p) == 1,
        zeta_fixes_t: e.zeta(&tp) == tp,
        three_t_is_zero: e.mul(3, &tp).is_none(),
        plain,
        zeta,
        ratio_c2,
        ratio_c3,
        predicted_a3: pa3,
        predicted_a6: pa6,
        residual_c2_before: ratio_c2 - 1.0,
        residual_c2_after_a3: ratio_c2 - pa3,
        residual_c2_after_a6: ratio_c2 - pa6,
        ratio_h2_from_compositions: rh2,
        residual_c2_given_composition: ratio_c2 - rh2,
        ratio_h3_from_compositions: rh3,
        derived_h3_ratio_expectation: (eg3z - f as f64 / 3.0) / eg3p,
        derived_single_mean: sm,
        derived_single_sd: sd,
        derived_prob_below_1: below,
        z_before: (ratio_c2 - 1.0) / sd,
        z_after_a6: (ratio_c2 - pa6) / sd,
    }
}

#[derive(Serialize)]
struct ReOut {
    records: Vec<Recorded>,
    /// Over the recorded censuses: mean measured ratio, residual before / after
    /// (|A| = 6) and the standard error of the mean under the composition model.
    mean_ratio_c2: f64,
    mean_residual_before: f64,
    mean_residual_after_a6: f64,
    sem_composition_model: f64,
    falsifier_residual_grew: bool,
}

// ── main ──────────────────────────────────────────────────────────────────

fn arg(args: &[String], name: &str, default: &str) -> String {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1).cloned())
        .unwrap_or_else(|| default.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("");
    let out = arg(&args, "--out", "");
    match cmd {
        "reanalyse" => {
            let files = arg(&args, "--json", "");
            let records: Vec<Recorded> = files
                .split(',')
                .filter(|s| !s.is_empty())
                .map(reanalyse)
                .collect();
            assert!(!records.is_empty(), "--json <file,file,...> required");
            let k = records.len() as f64;
            let mean_ratio_c2 = records.iter().map(|r| r.ratio_c2).sum::<f64>() / k;
            let a6 = records[0].predicted_a6;
            let sem = records[0].derived_single_sd / k.sqrt();
            let re = ReOut {
                mean_residual_before: mean_ratio_c2 - 1.0,
                mean_residual_after_a6: mean_ratio_c2 - a6,
                sem_composition_model: sem,
                falsifier_residual_grew: (mean_ratio_c2 - a6).abs() > (mean_ratio_c2 - 1.0).abs(),
                mean_ratio_c2,
                records,
            };
            let s = serde_json::to_string_pretty(&re).unwrap();
            if out.is_empty() {
                println!("{s}");
            } else {
                fs::write(&out, s).unwrap();
                eprintln!("wrote {out}");
            }
        }
        "census" => {
            let p: u64 = arg(&args, "--p", "195709").parse().unwrap();
            let b: u64 = arg(&args, "--b", "3").parse().unwrap();
            let sizes: Vec<u64> = arg(&args, "--sizes", "36,72,144")
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            let reps: usize = arg(&args, "--reps", "200").parse().unwrap();
            let seed: u64 = arg(&args, "--seed", "1").parse().unwrap();
            let res = run_census(p, b, &sizes, reps, seed);
            let s = serde_json::to_string_pretty(&res).unwrap();
            if out.is_empty() {
                println!("{s}");
            } else {
                fs::write(&out, s).unwrap();
                eprintln!("wrote {out}");
            }
        }
        _ => {
            eprintln!("usage: r6_5_census_correction reanalyse --json f1,f2 [--out o.json]");
            eprintln!("       r6_5_census_correction census --p P --b B --sizes 36,72,144 --reps N --seed S [--out o.json]");
            std::process::exit(2);
        }
    }
}
