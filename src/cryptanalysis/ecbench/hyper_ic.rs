//! # Index calculus on a hyperelliptic Jacobian, counted for ecbench
//!
//! The hyperelliptic thread's index calculus
//! ([`crate::cryptanalysis::hyperelliptic_index_calculus`]: Adleman–DeMarrais–
//! Huang / Gaudry, factor-base walk, sparse elimination mod `r`, optional
//! single large primes) run on the word-size Cantor group
//! [`HCurve`] / [`HDiv`], so that every group operation is charged on a
//! [`GroupOps`] ledger like every other ecbench arm, and every other
//! operation is counted exactly.
//!
//! - **Factor base.**  The degree-1 places `(x, y)`, `y ≤ (p−1)/2`, in
//!   increasing `x`, truncated to `fb_size` (Gaudry's reduced base).  A
//!   place is `[P] − [∞] = (x − x₀, y₀)`; building one costs no group
//!   operation, only the evaluation of `f` and a square root, counted as
//!   `factor_base_fp_muls_uncharged`.
//! - **Walk.**  The thread's `RelationSearch::FactorBaseWalk`: start at
//!   `D₁ + D₂` (one addition, phase `walk_setup`), then each trial adds
//!   `D₁`, `D₂` (one step in `d_step_in` each) or one of `branches`
//!   shuffled factor-base places (one addition, phase `relations`).  The
//!   randomness is a `StdRng` seeded with the algorithm seed and consumed
//!   in the thread's order, so on a run that solves in its first round the
//!   trajectory, the trial count and the group operations equal the
//!   thread's `hec_index_calculus_dlp` with the same parameters (tested).
//!   One deliberate difference: when the rows do not pin `k`, the thread
//!   restarts its walk from the seed and re-pays every operation; this arm
//!   keeps walking from where it stopped.
//! - **Smoothness oracle.**  `u` (monic, `deg ≤ g`) is split over `F_p`:
//!   closed form at degree 1 and 2; above, `L = gcd(u, x^p − x)` decides
//!   it (smooth iff `u / L` has no factor outside `L`), and the roots of
//!   `L` come from Cantor–Zassenhaus with the deterministic shifts
//!   `b = 0, 1, 2, …`.  Every `F_p` multiplication it performs — inside
//!   inversions (Fermat) and square roots (Tonelli–Shanks) included — is
//!   counted as `oracle_fp_muls_uncharged`; additions are free, as in every
//!   field-operation tally of the repository.
//! - **Linear algebra.**  The thread's Markowitz sparse elimination mod `r`
//!   (`sparse_solve_for_k`), on `u64` words: the same pivot order, its
//!   multiplications mod `r` counted as `linalg_mulmods_uncharged` and its
//!   inversions as `linalg_inversions_uncharged`.
//! - **The cofactor.**  A relation `a·D₁ + b·D₂ = Σ cᵢ Fᵢ` holds in the
//!   whole Jacobian.  Projecting onto the `r`-part (an endomorphism that
//!   fixes `⟨D₁⟩`) makes it a relation among the logarithms of the
//!   projected places, which exist when the `r`-part is cyclic: the arm
//!   refuses an instance with `r² | #J`.
//!
//! The arm returns its candidate unverified; the runner verifies it.

use std::collections::BTreeMap;
use std::time::Instant;

use rand::rngs::StdRng;
use rand::{RngCore, SeedableRng};

use crate::cryptanalysis::hyperelliptic_word::{HCurve, HDiv};
use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};

/// How many times the arm collects more rows when the system leaves `k`
/// free, as the thread does (`hec_index_calculus_dlp`'s four rounds).
pub const ROUNDS: usize = 4;

/// The parameters that shape the search.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HyperIcParams {
    /// Factor-base cap: the first `fb_size` places by `x`; `usize::MAX`
    /// keeps every degree-1 place.
    pub fb_size: usize,
    /// Keep partial relations with one place outside a truncated base and
    /// combine pairs sharing it (Thériault).
    pub large_primes: bool,
    /// Rows collected beyond the `m + 1` unknowns.
    pub extra_relations: usize,
    /// Factor-base places the walk steps by.
    pub branches: usize,
    /// One step in `d_step_in` is by `D₁`, one by `D₂`, the rest by places.
    pub d_step_in: u64,
    /// Relation trials before the run counts as exhausted.
    pub max_trials: u64,
}

/// What a run did, by phase.
#[derive(Clone, Debug, Default)]
pub struct HyperIcOutcome {
    pub recovered: Option<u64>,
    pub exhausted: bool,
    /// Phases in order: `factor_base`, `walk_setup`, `relations`,
    /// `linear_algebra`; each with its ledger, its native counters and its
    /// wall time.
    pub phases: Vec<(&'static str, GroupOps, BTreeMap<String, u64>, u64)>,
    /// Shape descriptors (no suffix) and every `*_uncharged` counter.
    pub counters: BTreeMap<String, u64>,
    /// The factor base's columns, in column order.
    pub columns: Vec<HDiv>,
    /// Affine points `(x, ±y)` over the columns' abscissae.
    pub signed_points: u64,
}

// ── F_p, counted ───────────────────────────────────────────────────────

/// `F_p` for `p < 256`, every multiplication counted into the tally the
/// caller passes.
struct Fp {
    p: u64,
    /// `p − 1 = q·2^s`, `q` odd.
    q: u64,
    s: u32,
    /// A quadratic non-residue, for Tonelli–Shanks.
    z: u64,
}

impl Fp {
    fn new(p: u64, c: &mut u64) -> Fp {
        let mut q = p - 1;
        let mut s = 0;
        while q.is_multiple_of(2) {
            q /= 2;
            s += 1;
        }
        let mut f = Fp { p, q, s, z: 0 };
        let mut z = 2;
        while f.pow(z, (p - 1) / 2, c) != p - 1 {
            z += 1;
        }
        f.z = z;
        f
    }

    fn mul(&self, a: u64, b: u64, c: &mut u64) -> u64 {
        *c += 1;
        a * b % self.p
    }

    fn add(&self, a: u64, b: u64) -> u64 {
        (a + b) % self.p
    }

    fn sub(&self, a: u64, b: u64) -> u64 {
        (a + self.p - b) % self.p
    }

    fn neg(&self, a: u64) -> u64 {
        (self.p - a) % self.p
    }

    /// `a^e` by right-to-left square-and-multiply.
    fn pow(&self, a: u64, mut e: u64, c: &mut u64) -> u64 {
        let mut result = 1;
        let mut base = a % self.p;
        while e > 0 {
            if e & 1 == 1 {
                result = self.mul(result, base, c);
            }
            e >>= 1;
            if e > 0 {
                base = self.mul(base, base, c);
            }
        }
        result
    }

    /// `a⁻¹` by Fermat, `a ≠ 0`.
    fn inv(&self, a: u64, c: &mut u64) -> u64 {
        debug_assert!(!a.is_multiple_of(self.p));
        self.pow(a, self.p - 2, c)
    }

    /// A square root of `a`, or `None` for a non-residue: Euler's
    /// criterion, then Tonelli–Shanks.
    fn sqrt(&self, a: u64, c: &mut u64) -> Option<u64> {
        let a = a % self.p;
        if a == 0 {
            return Some(0);
        }
        if self.pow(a, (self.p - 1) / 2, c) != 1 {
            return None;
        }
        let mut m = self.s;
        let mut cc = self.pow(self.z, self.q, c);
        let mut t = self.pow(a, self.q, c);
        let mut r = self.pow(a, self.q.div_ceil(2), c);
        while t != 1 {
            let mut i = 0;
            let mut t2 = t;
            while t2 != 1 {
                t2 = self.mul(t2, t2, c);
                i += 1;
            }
            let mut b = cc;
            for _ in 0..m - i - 1 {
                b = self.mul(b, b, c);
            }
            m = i;
            cc = self.mul(b, b, c);
            t = self.mul(t, cc, c);
            r = self.mul(r, b, c);
        }
        Some(r)
    }

    // ── Polynomials: lowest degree first, no trailing zeros ────────────

    fn trim(a: &mut Vec<u64>) {
        while a.last() == Some(&0) {
            a.pop();
        }
    }

    fn deg(a: &[u64]) -> usize {
        a.len().saturating_sub(1)
    }

    fn pmul(&self, a: &[u64], b: &[u64], c: &mut u64) -> Vec<u64> {
        if a.is_empty() || b.is_empty() {
            return Vec::new();
        }
        let mut out = vec![0u64; a.len() + b.len() - 1];
        for (i, &x) in a.iter().enumerate() {
            for (j, &y) in b.iter().enumerate() {
                out[i + j] = self.add(out[i + j], self.mul(x, y, c));
            }
        }
        Self::trim(&mut out);
        out
    }

    /// `(a div b, a mod b)`, `b ≠ 0`.  A monic divisor needs no inversion
    /// and no multiplication for the quotient digit.
    fn pdivrem(&self, a: &[u64], b: &[u64], c: &mut u64) -> (Vec<u64>, Vec<u64>) {
        debug_assert!(!b.is_empty());
        let mut r = a.to_vec();
        Self::trim(&mut r);
        if r.len() < b.len() {
            return (Vec::new(), r);
        }
        let db = Self::deg(b);
        let lead = b[db];
        let inv_lead = if lead == 1 { 1 } else { self.inv(lead, c) };
        let mut q = vec![0u64; r.len() - db];
        while !r.is_empty() && Self::deg(&r) >= db {
            let dr = Self::deg(&r);
            let top = r[dr];
            let digit = if inv_lead == 1 {
                top
            } else {
                self.mul(top, inv_lead, c)
            };
            q[dr - db] = digit;
            for i in 0..db {
                let t = self.mul(digit, b[i], c);
                r[dr - db + i] = self.sub(r[dr - db + i], t);
            }
            r[dr] = 0;
            Self::trim(&mut r);
        }
        Self::trim(&mut q);
        (q, r)
    }

    fn prem(&self, a: &[u64], b: &[u64], c: &mut u64) -> Vec<u64> {
        self.pdivrem(a, b, c).1
    }

    fn monic(&self, mut a: Vec<u64>, c: &mut u64) -> Vec<u64> {
        if let Some(&lead) = a.last() {
            if lead != 1 {
                let inv = self.inv(lead, c);
                let n = a.len();
                for x in a.iter_mut().take(n - 1) {
                    *x = self.mul(*x, inv, c);
                }
                a[n - 1] = 1;
            }
        }
        a
    }

    /// Monic `gcd(a, b)`.
    fn pgcd(&self, a: &[u64], b: &[u64], c: &mut u64) -> Vec<u64> {
        let (mut a, mut b) = (a.to_vec(), b.to_vec());
        Self::trim(&mut a);
        Self::trim(&mut b);
        while !b.is_empty() {
            let r = self.prem(&a, &b, c);
            a = b;
            b = r;
        }
        self.monic(a, c)
    }

    /// `base^e mod m`, `m` monic of degree ≥ 1.
    fn ppowmod(&self, base: &[u64], mut e: u64, m: &[u64], c: &mut u64) -> Vec<u64> {
        let mut result = self.prem(&[1], m, c);
        let mut acc = self.prem(base, m, c);
        while e > 0 {
            if e & 1 == 1 {
                result = self.prem(&self.pmul(&result, &acc, c), m, c);
            }
            e >>= 1;
            if e > 0 {
                acc = self.prem(&self.pmul(&acc, &acc, c), m, c);
            }
        }
        result
    }

    /// Horner evaluation.
    fn eval(&self, a: &[u64], x: u64, c: &mut u64) -> u64 {
        let mut acc = 0;
        for (i, &coef) in a.iter().enumerate().rev() {
            acc = if i + 1 == a.len() {
                coef
            } else {
                self.add(self.mul(acc, x, c), coef)
            };
        }
        acc
    }

    /// Roots of a monic quadratic `x² + a₁x + a₀`, sorted, with
    /// multiplicity; `None` when it is irreducible.
    fn quadratic_roots(&self, u: &[u64], c: &mut u64) -> Option<Vec<(u64, usize)>> {
        let (a0, a1) = (u[0], u[1]);
        let four_a0 = self.add(self.add(a0, a0), self.add(a0, a0));
        let disc = self.sub(self.mul(a1, a1, c), four_a0);
        let inv2 = self.p.div_ceil(2);
        let neg_a1 = self.neg(a1);
        if disc == 0 {
            return Some(vec![(self.mul(neg_a1, inv2, c), 2)]);
        }
        let s = self.sqrt(disc, c)?;
        let r1 = self.mul(self.add(neg_a1, s), inv2, c);
        let r2 = self.mul(self.sub(neg_a1, s), inv2, c);
        Some(if r1 < r2 {
            vec![(r1, 1), (r2, 1)]
        } else {
            vec![(r2, 1), (r1, 1)]
        })
    }

    /// Roots of a monic, squarefree, completely split `l`, sorted.
    fn split_roots(&self, l: &[u64], c: &mut u64) -> Vec<u64> {
        let mut out = match Self::deg(l) {
            0 => Vec::new(),
            1 => vec![self.neg(l[0])],
            2 => self
                .quadratic_roots(l, c)
                .expect("a split quadratic has roots")
                .into_iter()
                .map(|(x, _)| x)
                .collect(),
            _ => {
                // Cantor–Zassenhaus with the shifts b = 0, 1, 2, …: the
                // gcd with (x + b)^((p−1)/2) − 1 collects the roots whose
                // shift is a non-zero square.
                let half = (self.p - 1) / 2;
                let mut found = None;
                for b in 0..self.p {
                    let mut h = self.ppowmod(&[b, 1], half, l, c);
                    if h.is_empty() {
                        h.push(0);
                    }
                    h[0] = self.sub(h[0], 1);
                    Self::trim(&mut h);
                    if h.is_empty() {
                        continue;
                    }
                    let g = self.pgcd(l, &h, c);
                    let dg = Self::deg(&g);
                    if dg > 0 && dg < Self::deg(l) {
                        found = Some(g);
                        break;
                    }
                }
                match found {
                    Some(g) => {
                        let (other, _) = self.pdivrem(l, &g, c);
                        let mut roots = self.split_roots(&g, c);
                        roots.extend(self.split_roots(&other, c));
                        roots
                    }
                    // Not reached for p ≥ 5; scan rather than fail.
                    None => (0..self.p).filter(|&x| self.eval(l, x, c) == 0).collect(),
                }
            }
        };
        out.sort_unstable();
        out
    }

    /// Split the monic `u` (`1 ≤ deg u`) into linear factors over `F_p`:
    /// `(root, multiplicity)` sorted by root, or `None` when some
    /// irreducible factor has degree ≥ 2.
    fn split(&self, u: &[u64], c: &mut u64) -> Option<Vec<(u64, usize)>> {
        match Self::deg(u) {
            0 => return Some(Vec::new()),
            1 => return Some(vec![(self.neg(u[0]), 1)]),
            2 => return self.quadratic_roots(u, c),
            _ => {}
        }
        let xp = self.ppowmod(&[0, 1], self.p, u, c);
        let mut xp_minus_x = xp;
        xp_minus_x.resize(xp_minus_x.len().max(2), 0);
        xp_minus_x[1] = self.sub(xp_minus_x[1], 1);
        Self::trim(&mut xp_minus_x);
        // L: the product of u's distinct linear factors.
        let l = self.pgcd(u, &xp_minus_x, c);
        if Self::deg(&l) == 0 {
            return None;
        }
        if Self::deg(&l) < Self::deg(u) {
            // Smooth iff every factor of u / L divides a power of L.
            let (mut q, _) = self.pdivrem(u, &l, c);
            loop {
                let h = self.pgcd(&q, &l, c);
                if Self::deg(&h) == 0 {
                    break;
                }
                q = self.pdivrem(&q, &h, c).0;
            }
            if Self::deg(&q) > 0 {
                return None;
            }
        }
        let roots = self.split_roots(&l, c);
        // Multiplicities by synthetic division.
        let mut out = Vec::with_capacity(roots.len());
        let mut rest = u.to_vec();
        for x in roots {
            let lin = [self.neg(x), 1];
            let mut mult = 0;
            loop {
                let (q, r) = self.pdivrem(&rest, &lin, c);
                if !r.is_empty() {
                    break;
                }
                rest = q;
                mult += 1;
            }
            out.push((x, mult));
        }
        debug_assert_eq!(rest, vec![1]);
        Some(out)
    }
}

// ── Relations ──────────────────────────────────────────────────────────

/// `a·D₁ + b·D₂ = Σ cⱼ Fⱼ` (the walk's steps already subtracted).
#[derive(Clone, Debug)]
struct Relation {
    a: u64,
    b: u64,
    entries: Vec<(usize, i64)>,
}

enum Decomposition {
    Smooth(Vec<(usize, i64)>),
    OneLargePrime {
        entries: Vec<(usize, i64)>,
        x: u64,
        coef: i64,
    },
    SmoothOffBase,
    NotSmooth,
}

/// The exact work of the relation phase, by counter.
#[derive(Default)]
struct Tally {
    oracle_fp_muls: u64,
    factor_base_lookups: u64,
    canonicalisations: u64,
    step_draws: u64,
    large_prime_mulmods: u64,
    large_prime_lookups: u64,
    linalg_mulmods: u64,
    linalg_inversions: u64,
}

fn mulmod(a: u64, b: u64, n: u64) -> u64 {
    (u128::from(a) * u128::from(b) % u128::from(n)) as u64
}

fn signed_mod(v: i64, n: u64) -> u64 {
    let m = v.unsigned_abs() % n;
    if v >= 0 {
        m
    } else {
        (n - m) % n
    }
}

/// `a⁻¹ mod n` by the extended Euclidean algorithm, `n` prime.
fn inv_mod(a: u64, n: u64) -> Option<u64> {
    let (mut r0, mut r1) = (i128::from(n), i128::from(a % n));
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    (r0 == 1).then(|| t0.rem_euclid(i128::from(n)) as u64)
}

/// The thread's `combine_partials`: `e₂·r₁ − e₁·r₂` eliminates the shared
/// off-base place.
fn combine(
    r1: &Relation,
    e1: i64,
    r2: &Relation,
    e2: i64,
    n: u64,
    t: &mut Tally,
) -> Option<Relation> {
    let mut scale = |v: u64, k: i64| {
        t.large_prime_mulmods += 1;
        let s = mulmod(v, k.unsigned_abs() % n, n);
        if k < 0 {
            (n - s) % n
        } else {
            s
        }
    };
    let a = (scale(r1.a, e2) + scale(r2.a, -e1)) % n;
    let b = (scale(r1.b, e2) + scale(r2.b, -e1)) % n;
    let mut acc: BTreeMap<usize, i64> = BTreeMap::new();
    for &(j, c) in &r1.entries {
        *acc.entry(j).or_insert(0) += e2 * c;
    }
    for &(j, c) in &r2.entries {
        *acc.entry(j).or_insert(0) -= e1 * c;
    }
    let entries: Vec<(usize, i64)> = acc.into_iter().filter(|&(_, c)| c != 0).collect();
    if entries.is_empty() && a == 0 && b == 0 {
        return None;
    }
    Some(Relation { a, b, entries })
}

/// The thread's Markowitz sparse elimination mod `n`, on words: eliminate
/// every factor-base column and read `k` (column `m`) off a surviving row.
fn sparse_solve_for_k(
    mut rows: Vec<Vec<(usize, u64)>>,
    mut rhs: Vec<u64>,
    m: usize,
    n: u64,
    t: &mut Tally,
) -> Option<u64> {
    let mut col_count = vec![0usize; m + 1];
    let mut holders: Vec<Vec<usize>> = vec![Vec::new(); m + 1];
    for (i, row) in rows.iter().enumerate() {
        for &(j, _) in row {
            col_count[j] += 1;
            holders[j].push(i);
        }
    }
    let mut eliminated = vec![false; rows.len()];
    let has = |row: &[(usize, u64)], col: usize| row.binary_search_by_key(&col, |&(j, _)| j);

    while let Some(col) = (0..m)
        .filter(|&j| col_count[j] > 0)
        .min_by_key(|&j| col_count[j])
    {
        let mut live = std::mem::take(&mut holders[col]);
        live.retain(|&i| !eliminated[i] && has(&rows[i], col).is_ok());
        live.sort_unstable();
        live.dedup();
        debug_assert_eq!(live.len(), col_count[col]);
        let pivot = *live.iter().min_by_key(|&&i| rows[i].len())?;

        let at = has(&rows[pivot], col).ok()?;
        t.linalg_inversions += 1;
        let inv = inv_mod(rows[pivot][at].1, n)?;
        for (_, v) in rows[pivot].iter_mut() {
            *v = mulmod(*v, inv, n);
            t.linalg_mulmods += 1;
        }
        rhs[pivot] = mulmod(rhs[pivot], inv, n);
        t.linalg_mulmods += 1;
        eliminated[pivot] = true;
        let pivot_row = std::mem::take(&mut rows[pivot]);
        let pivot_rhs = rhs[pivot];
        for &(j, _) in &pivot_row {
            col_count[j] -= 1;
        }

        for i in live {
            if i == pivot {
                continue;
            }
            let old = std::mem::take(&mut rows[i]);
            let factor = old[has(&old, col).ok()?].1;
            let mut merged: Vec<(usize, u64)> = Vec::with_capacity(old.len() + pivot_row.len());
            let (mut a, mut b) = (0, 0);
            while a < old.len() || b < pivot_row.len() {
                let ja = old.get(a).map_or(usize::MAX, |e| e.0);
                let jb = pivot_row.get(b).map_or(usize::MAX, |e| e.0);
                if ja < jb {
                    merged.push(old[a]);
                    a += 1;
                    continue;
                }
                let term = mulmod(factor, pivot_row[b].1, n);
                t.linalg_mulmods += 1;
                let v = if ja == jb {
                    let v = (old[a].1 + n - term) % n;
                    a += 1;
                    v
                } else {
                    (n - term) % n
                };
                if v != 0 {
                    if ja != jb {
                        holders[jb].push(i);
                    }
                    merged.push((jb, v));
                }
                b += 1;
            }
            for &(j, _) in &old {
                col_count[j] -= 1;
            }
            for &(j, _) in &merged {
                col_count[j] += 1;
            }
            rhs[i] = (rhs[i] + n - mulmod(factor, pivot_rhs, n)) % n;
            t.linalg_mulmods += 1;
            rows[i] = merged;
        }
    }

    for (i, row) in rows.iter().enumerate() {
        if eliminated[i] {
            continue;
        }
        if row.len() == 1 && row[0].0 == m {
            t.linalg_inversions += 1;
            let inv = inv_mod(row[0].1, n)?;
            t.linalg_mulmods += 1;
            return Some(mulmod(rhs[i], inv, n));
        }
    }
    None
}

/// Rows `Σ cᵢ yᵢ − b·k ≡ a (mod n)` over the unknowns `(y₀ … y_{m−1}, k)`.
fn solve_relations(relations: &[Relation], m: usize, n: u64, t: &mut Tally) -> Option<u64> {
    let mut rows = Vec::with_capacity(relations.len());
    let mut rhs = Vec::with_capacity(relations.len());
    for rel in relations {
        let mut row: BTreeMap<usize, u64> = BTreeMap::new();
        for &(j, c) in &rel.entries {
            let e = row.entry(j).or_insert(0);
            *e = (*e + signed_mod(c, n)) % n;
        }
        row.insert(m, (n - rel.b % n) % n);
        rows.push(row.into_iter().filter(|&(_, v)| v != 0).collect());
        rhs.push(rel.a % n);
    }
    sparse_solve_for_k(rows, rhs, m, n, t)
}

// ── The run ────────────────────────────────────────────────────────────

/// Solve `target = k·gen` in the subgroup of prime order `r`, under the
/// algorithm seed `seed`.  `group_order` is `#J`; the run refuses
/// `r² | #J` (see the module notes).
pub fn solve(
    curve: &HCurve,
    gen: HDiv,
    target: HDiv,
    r: u64,
    group_order: u64,
    seed: u64,
    params: &HyperIcParams,
) -> Result<HyperIcOutcome, String> {
    if !group_order.is_multiple_of(r) || (group_order / r).is_multiple_of(r) {
        return Err(format!(
            "ic.hyperelliptic needs r ∥ #J (a cyclic r-part); r = {r}, #J = {group_order}"
        ));
    }
    if params.branches == 0 || params.d_step_in < 2 {
        return Err("ic.hyperelliptic needs branches ≥ 1 and d_step_in ≥ 2".into());
    }
    let p = curve.p();
    let f = curve.f_coeffs();
    let half = (p - 1) / 2;

    // ── factor_base ──
    let t_fb = Instant::now();
    let mut fb_muls = 0u64;
    let fp = Fp::new(p, &mut fb_muls);
    let mut columns: Vec<HDiv> = Vec::new();
    let mut col_ys: Vec<u64> = Vec::new();
    let mut col_of_x: Vec<Option<usize>> = vec![None; p as usize];
    let mut signed_points = 0u64;
    for x in 0..p {
        if columns.len() >= params.fb_size {
            break;
        }
        let fx = fp.eval(&f, x, &mut fb_muls);
        let Some(y) = fp.sqrt(fx, &mut fb_muls) else {
            continue;
        };
        let y = if y > half { p - y } else { y };
        let d = curve
            .point(x, y)
            .ok_or("a factor-base place is not on the curve")?;
        col_of_x[x as usize] = Some(columns.len());
        columns.push(d);
        col_ys.push(y);
        signed_points += if y == 0 { 1 } else { 2 };
    }
    let fb_wall = t_fb.elapsed().as_nanos() as u64;
    let m = columns.len();
    if m == 0 {
        return Err("the factor base is empty".into());
    }

    // ── walk_setup ──
    let t_setup = Instant::now();
    let mut setup_ops = GroupOps::default();
    let mut tally = Tally::default();
    let mut setup_draws = 0u64;
    let mut rng = StdRng::seed_from_u64(seed);
    let branches = params.branches.min(m).max(1);
    let mut chosen: Vec<usize> = (0..m).collect();
    for i in (1..chosen.len()).rev() {
        setup_draws += 1;
        let j = (rng.next_u64() as usize) % (i + 1);
        chosen.swap(i, j);
    }
    let fb_steps: Vec<usize> = chosen.into_iter().take(branches).collect();
    let setup_wall_before_start = t_setup.elapsed().as_nanos() as u64;

    // ── relations (the start D₁ + D₂ is charged to walk_setup) ──
    let mut rel_ops = GroupOps::default();
    let mut rel_wall = 0u64;
    let mut setup_wall = setup_wall_before_start;
    let mut la_wall = 0u64;
    let mut relations: Vec<Relation> = Vec::new();
    let mut partials: BTreeMap<u64, (Relation, i64)> = BTreeMap::new();
    let mut taken = vec![0i64; m];
    let mut touched: Vec<usize> = Vec::new();
    let mut position: Option<(u64, u64, HDiv)> = None;
    let (mut trials, mut smooth, mut partial_count, mut combined, mut off_base) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut rounds = 0u64;
    let mut extra = params.extra_relations;
    let mut recovered = None;

    let subtract_taken = |entries: Vec<(usize, i64)>, taken: &[i64], touched: &[usize]| {
        let mut acc: BTreeMap<usize, i64> = entries.into_iter().collect();
        for &idx in touched {
            *acc.entry(idx).or_insert(0) -= taken[idx];
        }
        acc.into_iter().filter(|&(_, c)| c != 0).collect::<Vec<_>>()
    };

    for _round in 0..ROUNDS {
        let wanted = m + 1 + extra;
        let t_rel = Instant::now();
        while relations.len() < wanted && trials < params.max_trials {
            trials += 1;
            let (a, b, d) = match position.take() {
                None => {
                    let t0 = Instant::now();
                    let d = curve.add(&mut setup_ops, gen, target);
                    setup_wall += t0.elapsed().as_nanos() as u64;
                    (1 % r, 1 % r, d)
                }
                Some((a, b, d)) => {
                    tally.step_draws += 1;
                    let roll = rng.next_u64() % params.d_step_in;
                    if roll == 0 {
                        ((a + 1) % r, b, curve.add(&mut rel_ops, d, gen))
                    } else if roll == 1 {
                        (a, (b + 1) % r, curve.add(&mut rel_ops, d, target))
                    } else {
                        tally.step_draws += 1;
                        let idx = fb_steps[(rng.next_u64() as usize) % fb_steps.len()];
                        if let Err(at) = touched.binary_search(&idx) {
                            touched.insert(at, idx);
                        }
                        taken[idx] += 1;
                        (a, b, curve.add(&mut rel_ops, d, columns[idx]))
                    }
                }
            };
            position = Some((a, b, d));

            if d.is_identity() {
                smooth += 1;
                relations.push(Relation {
                    a,
                    b,
                    entries: subtract_taken(Vec::new(), &taken, &touched),
                });
                continue;
            }
            match classify(&fp, &d, &col_of_x, &col_ys, half, &mut tally) {
                Decomposition::Smooth(entries) => {
                    smooth += 1;
                    relations.push(Relation {
                        a,
                        b,
                        entries: subtract_taken(entries, &taken, &touched),
                    });
                }
                Decomposition::OneLargePrime { entries, x, coef } if params.large_primes => {
                    smooth += 1;
                    partial_count += 1;
                    let rel = Relation {
                        a,
                        b,
                        entries: subtract_taken(entries, &taken, &touched),
                    };
                    tally.large_prime_lookups += 1;
                    match partials.get(&x) {
                        None => {
                            partials.insert(x, (rel, coef));
                        }
                        Some((first, first_coef)) => {
                            if let Some(full) =
                                combine(first, *first_coef, &rel, coef, r, &mut tally)
                            {
                                combined += 1;
                                relations.push(full);
                            }
                        }
                    }
                }
                Decomposition::OneLargePrime { .. } | Decomposition::SmoothOffBase => {
                    smooth += 1;
                    off_base += 1;
                }
                Decomposition::NotSmooth => {}
            }
        }
        rel_wall += t_rel.elapsed().as_nanos() as u64;
        if relations.len() < m + 1 {
            break;
        }
        rounds += 1;
        let t_la = Instant::now();
        let k = solve_relations(&relations, m, r, &mut tally);
        la_wall += t_la.elapsed().as_nanos() as u64;
        if let Some(k) = k {
            recovered = Some(k);
            break;
        }
        if trials >= params.max_trials {
            break;
        }
        extra += m / 4 + 8;
    }
    // The start's own wall time sits inside the first round's clock.
    rel_wall = rel_wall.saturating_sub(setup_wall - setup_wall_before_start);

    let named = |pairs: &[(&str, u64)]| -> BTreeMap<String, u64> {
        pairs.iter().map(|&(k, v)| (k.to_string(), v)).collect()
    };
    let fb_native = named(&[("factor_base_fp_muls_uncharged", fb_muls)]);
    let setup_native = named(&[("step_draws_uncharged", setup_draws)]);
    let mut rel_native = named(&[
        ("oracle_fp_muls_uncharged", tally.oracle_fp_muls),
        ("factor_base_lookups_uncharged", tally.factor_base_lookups),
        ("canonicalisations_uncharged", tally.canonicalisations),
        ("step_draws_uncharged", tally.step_draws),
    ]);
    if params.large_primes {
        rel_native.insert(
            "large_prime_mulmods_uncharged".into(),
            tally.large_prime_mulmods,
        );
        rel_native.insert(
            "large_prime_lookups_uncharged".into(),
            tally.large_prime_lookups,
        );
    }
    let la_native = named(&[
        ("linalg_mulmods_uncharged", tally.linalg_mulmods),
        ("linalg_inversions_uncharged", tally.linalg_inversions),
    ]);

    let mut counters: BTreeMap<String, u64> = BTreeMap::new();
    for native in [&fb_native, &setup_native, &rel_native, &la_native] {
        for (k, v) in native {
            *counters.entry(k.clone()).or_insert(0) += v;
        }
    }
    for (k, v) in [
        ("factor_base_columns", m as u64),
        ("walk_branches", branches as u64),
        ("trials", trials),
        ("smooth_trials", smooth),
        ("relations", relations.len() as u64),
        ("partial_relations", partial_count),
        ("combined_relations", combined),
        ("discarded_off_base", off_base),
        ("solve_rounds", rounds),
    ] {
        counters.insert(k.into(), v);
    }

    Ok(HyperIcOutcome {
        recovered,
        // No candidate: the trial cap, or ROUNDS solves that left `k` free.
        exhausted: recovered.is_none(),
        phases: vec![
            ("factor_base", GroupOps::default(), fb_native, fb_wall),
            ("walk_setup", setup_ops, setup_native, setup_wall),
            ("relations", rel_ops, rel_native, rel_wall),
            ("linear_algebra", GroupOps::default(), la_native, la_wall),
        ],
        counters,
        columns,
        signed_points,
    })
}

/// The thread's `classify_counted_with` on a word-size divisor.
fn classify(
    fp: &Fp,
    d: &HDiv,
    col_of_x: &[Option<usize>],
    col_ys: &[u64],
    half: u64,
    t: &mut Tally,
) -> Decomposition {
    let u = d.u_coeffs();
    let v: Vec<u64> = d.v_coeffs();
    let Some(roots) = fp.split(&u, &mut t.oracle_fp_muls) else {
        return Decomposition::NotSmooth;
    };
    let mut acc: BTreeMap<usize, i64> = BTreeMap::new();
    let mut off: Option<(u64, i64)> = None;
    for (x, mult) in roots {
        let y = fp.eval(&v, x, &mut t.oracle_fp_muls);
        t.canonicalisations += 1;
        let sign: i64 = if y == 0 || y <= half { 1 } else { -1 };
        t.factor_base_lookups += 1;
        match col_of_x[x as usize] {
            Some(idx) => {
                debug_assert_eq!(col_ys[idx], if sign > 0 { y } else { fp.p - y });
                *acc.entry(idx).or_insert(0) += sign * mult as i64;
            }
            None => {
                if off.is_some() {
                    return Decomposition::SmoothOffBase;
                }
                off = Some((x, sign * mult as i64));
            }
        }
    }
    let entries: Vec<(usize, i64)> = acc.into_iter().filter(|&(_, c)| c != 0).collect();
    match off {
        None => Decomposition::Smooth(entries),
        Some((x, coef)) => Decomposition::OneLargePrime { entries, x, coef },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ecbench::hyper::HyperInstance;
    use crate::cryptanalysis::hyperelliptic_index_calculus::{
        hec_index_calculus_dlp, split_counted_with, HecIndexCalculusParams, LinearAlgebra,
        RelationSearch, SmoothnessTest,
    };
    use crate::prime_hyperelliptic::FpPoly;
    use num_bigint::BigUint;
    use num_traits::ToPrimitive;

    fn params(fb_size: usize, large_primes: bool) -> HyperIcParams {
        HyperIcParams {
            fb_size,
            large_primes,
            extra_relations: 8,
            branches: 64,
            d_step_in: 4,
            max_trials: 200_000,
        }
    }

    #[test]
    fn the_word_oracle_agrees_with_the_threads_on_every_small_u() {
        // Every monic u of degree ≤ 3 over F_13 and a sample of degree 4.
        let p = 13u64;
        let mut c = 0;
        let fp = Fp::new(p, &mut c);
        let bp = BigUint::from(p);
        let mut checked = 0;
        for deg in 1..=4usize {
            let total = p.pow(deg as u32);
            let step = if deg == 4 { 7 } else { 1 };
            for idx in (0..total).step_by(step) {
                let mut u: Vec<u64> = (0..deg).map(|i| idx / p.pow(i as u32) % p).collect();
                u.push(1);
                let mine = fp.split(&u, &mut c);
                let big =
                    FpPoly::from_coeffs(u.iter().map(|&x| BigUint::from(x)).collect(), bp.clone());
                let mut ops = 0usize;
                let theirs =
                    split_counted_with(&big, &bp, &mut ops, &SmoothnessTest::Scan).map(|v| {
                        v.into_iter()
                            .map(|(x, m)| (x.to_u64().unwrap(), m))
                            .collect::<Vec<_>>()
                    });
                assert_eq!(mine, theirs, "u = {u:?}");
                checked += 1;
            }
        }
        assert!(checked > 2000);
    }

    #[test]
    fn the_word_solve_matches_the_thread_trajectory_and_recovers_the_log() {
        let mut compared = 0;
        for (g, p, fb, lp) in [
            (2u32, 41u64, usize::MAX, false),
            (2, 41, 12, true),
            (3, 23, usize::MAX, false),
            (4, 17, usize::MAX, false),
        ] {
            let inst = HyperInstance::build_thread(p, g).unwrap();
            let big = inst.curve.to_big();
            let d1 = inst.curve.to_mumford(&inst.generator);
            let n = BigUint::from(inst.r);
            for (seed, k) in [(1u64, 12345u64), (2, 777), (3, inst.r - 2)] {
                let target = inst.curve.mul_uncounted(inst.generator, k % inst.r);
                let prm = params(fb, lp);
                let out = solve(
                    &inst.curve,
                    inst.generator,
                    target,
                    inst.r,
                    inst.group_order,
                    seed,
                    &prm,
                )
                .unwrap();
                assert_eq!(out.recovered, Some(k % inst.r), "g {g} p {p} seed {seed}");
                let ops: u64 = out.phases.iter().map(|ph| ph.1.adds + ph.1.doubles).sum();
                if out.counters["solve_rounds"] == 1 {
                    let tp = HecIndexCalculusParams {
                        fb_size: fb,
                        extra_relations: prm.extra_relations,
                        max_trials: prm.max_trials as usize,
                        seed,
                        search: RelationSearch::FactorBaseWalk {
                            branches: prm.branches,
                            d_step_in: prm.d_step_in,
                        },
                        linear_algebra: LinearAlgebra::Sparse,
                        smoothness: SmoothnessTest::Gcd,
                        large_primes: lp,
                    };
                    let d2 = inst.curve.to_mumford(&target);
                    let (tk, rep) = hec_index_calculus_dlp(&big, &d1, &d2, &n, &tp);
                    assert_eq!(tk, Some(BigUint::from(k % inst.r)));
                    assert_eq!(rep.trials as u64, out.counters["trials"]);
                    assert_eq!(rep.relations as u64, out.counters["relations"]);
                    assert_eq!(rep.smooth_trials as u64, out.counters["smooth_trials"]);
                    assert_eq!(rep.jacobian_ops as u64, ops, "g {g} p {p} seed {seed}");
                    assert_eq!(
                        rep.factor_base_size as u64,
                        out.counters["factor_base_columns"]
                    );
                    assert_eq!(
                        rep.partial_relations as u64,
                        out.counters["partial_relations"]
                    );
                    assert_eq!(
                        rep.combined_relations as u64,
                        out.counters["combined_relations"]
                    );
                    compared += 1;
                }
            }
        }
        assert!(compared >= 8, "only {compared} runs solved in one round");
    }

    #[test]
    fn large_primes_on_a_truncated_base_still_recover() {
        let inst = HyperInstance::build_thread(41, 2).unwrap();
        let k = 1000 % inst.r;
        let target = inst.curve.mul_uncounted(inst.generator, k);
        let out = solve(
            &inst.curve,
            inst.generator,
            target,
            inst.r,
            inst.group_order,
            5,
            &params(12, true),
        )
        .unwrap();
        assert_eq!(out.recovered, Some(k));
        assert_eq!(out.counters["factor_base_columns"], 12);
        assert!(out.counters["partial_relations"] > 0);
        assert!(out.counters["combined_relations"] > 0);
    }

    use crate::cryptanalysis::ecbench::methods::{self, MethodSpec, SolveReport};
    use crate::cryptanalysis::ecbench::workload::{CurveSpec, TargetKind, Workload};

    fn method(fb_size: &str, large_primes: &str) -> MethodSpec {
        MethodSpec {
            id: "ic.hyperelliptic".into(),
            params: [
                ("fb_size", fb_size),
                ("large_primes", large_primes),
                ("extra_relations", "8"),
                ("branches", "64"),
                ("d_step_in", "4"),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        }
    }

    /// One run through ecbench's own path: plan check, then `solve`.
    fn ecbench_run(spec: &CurveSpec, index: u64, seed: u64, m: &MethodSpec) -> (SolveReport, u128) {
        let (w, inst) = Workload::build(spec, 11, index, TargetKind::Planted).unwrap();
        let m = methods::resolve(m).unwrap();
        methods::applicable(&m, &inst, &w.curve).unwrap();
        let rep = methods::solve(&m, &inst, &w.curve, &w.target, seed).unwrap();
        (rep, w.planted.unwrap())
    }

    #[test]
    fn ecbench_recovers_planted_logs_on_thread_jacobians() {
        for (p, genus) in [(41u64, 2u32), (23, 3)] {
            let spec = CurveSpec::HyperellipticThread { p, genus };
            for (index, seed) in [(0u64, 1u64), (1, 9), (2, 4)] {
                for (fb, lp) in [("all", "0"), ("all", "1")] {
                    let (rep, planted) = ecbench_run(&spec, index, seed, &method(fb, lp));
                    assert_eq!(
                        rep.recovered,
                        Some(planted),
                        "g {genus} p {p} index {index}"
                    );
                    assert!(!rep.exhausted);
                    let names: Vec<&str> = rep.phases.iter().map(|ph| ph.name.as_str()).collect();
                    assert_eq!(
                        names,
                        ["factor_base", "walk_setup", "relations", "linear_algebra"]
                    );
                    assert_eq!(rep.phases[0].gae + rep.phases[3].gae, 0.0);
                    assert_eq!(rep.phases[1].gae, 1.0);
                    let sum: f64 = rep.phases.iter().map(|ph| ph.gae).sum();
                    assert_eq!(rep.total_gae, sum);
                    // One charged operation per trial.
                    assert_eq!(rep.total_gae as u64, rep.counters["trials"]);
                    assert!(rep.unpriced.iter().all(|n| n.ends_with("_uncharged")));
                    for must in [
                        "oracle_fp_muls_uncharged",
                        "linalg_mulmods_uncharged",
                        "factor_base_fp_muls_uncharged",
                    ] {
                        assert!(rep.unpriced.iter().any(|n| n == must), "{must}");
                    }
                    assert_eq!(rep.automorphisms_used, 2);
                    assert!(rep.factor_base.is_some());
                }
            }
        }
        // A truncated base with large primes, through the same path.
        let spec = CurveSpec::HyperellipticThread { p: 41, genus: 2 };
        let (rep, planted) = ecbench_run(&spec, 3, 2, &method("12", "1"));
        assert_eq!(rep.recovered, Some(planted));
        assert!(rep.counters["combined_relations"] > 0);
    }

    #[test]
    fn ecbench_runs_are_deterministic_in_the_seed() {
        let spec = CurveSpec::HyperellipticThread { p: 23, genus: 3 };
        let strip = |mut r: SolveReport| {
            r.solve_wall_ns = 0;
            for ph in &mut r.phases {
                ph.wall_ns = None;
            }
            r
        };
        let (a, _) = ecbench_run(&spec, 0, 5, &method("all", "0"));
        let (b, _) = ecbench_run(&spec, 0, 5, &method("all", "0"));
        let (c, _) = ecbench_run(&spec, 0, 6, &method("all", "0"));
        assert_eq!(a.counters, b.counters);
        assert_eq!(strip(a.clone()), strip(b));
        assert_ne!(a.counters, c.counters);
    }

    #[test]
    fn ic_hyperelliptic_is_refused_off_jacobians_and_other_ic_on_them() {
        let (w, inst) = Workload::build(
            &CurveSpec::PrimeSearch { bits: 16, seed: 1 },
            11,
            0,
            TargetKind::Planted,
        )
        .unwrap();
        let m = methods::resolve(&method("all", "0")).unwrap();
        assert!(methods::applicable(&m, &inst, &w.curve).is_err());
        assert!(methods::solve(&m, &inst, &w.curve, &w.target, 1).is_err());
        let spec = CurveSpec::HyperellipticThread { p: 41, genus: 2 };
        let (w, inst) = Workload::build(&spec, 11, 0, TargetKind::Planted).unwrap();
        let pipeline = methods::resolve(&MethodSpec {
            id: "ic.pipeline".into(),
            params: [
                (
                    "factor_base".to_string(),
                    "prime-abscissa:size=8".to_string(),
                ),
                ("oracle".to_string(), "subtract:m=2".to_string()),
            ]
            .into_iter()
            .collect(),
        })
        .unwrap();
        assert!(methods::applicable(&pipeline, &inst, &w.curve).is_err());
        // A search parameter has no default.
        let mut missing = method("all", "0");
        missing.params.remove("branches");
        assert!(methods::resolve(&missing).is_err());
    }

    #[test]
    fn a_square_r_part_is_refused() {
        let inst = HyperInstance::build_thread(41, 2).unwrap();
        let bogus = inst.r * inst.r;
        assert!(solve(
            &inst.curve,
            inst.generator,
            inst.generator,
            inst.r,
            bogus,
            0,
            &params(usize::MAX, false)
        )
        .is_err());
    }
}
