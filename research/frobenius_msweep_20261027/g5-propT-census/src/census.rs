//! Exact same-instance overlap census (Lemma A2-prime) over the prime-order subgroup G.
//!
//! Legs are classes of G (x-coordinates of points of G); a class has an unsigned logarithm d in 1..=(r-1)/2.
//! A set S of m classes solves target class a iff some signed sum  sum_i s_i d_i  equals +-a (mod r): the
//! chained-S3 system over the legs of G has a solution (tested against `chain::member` in the tests and
//! on samples inside every run). Sol(a) = {S : a in K(S)} where K(S) is the set of classes of the 2^m signed sums.
//!
//! Overlap of conjugate instances: S in Sol(a) and S in Sol(class(mu a)) for mu a unit != +-1. For mu = lam^j
//! these are the same-instance shared tuples of R and sigma^j R. U counts (S, class a, j in 1..=(n-1)/2) triples,
//! i.e. unordered pairs of conjugate target classes {a, lam^j a} (each pair once) times shared tuples.

use crate::curve::Group;

/// Multiplier tables: tabs[t][a] = class(mu_t * a) for a in 0..=half.
pub struct Mult {
    pub tabs: Vec<Vec<u32>>,
    pub mus: Vec<u32>,
}

impl Mult {
    pub fn from_units(r: u32, mus: &[u32]) -> Mult {
        let half = ((r - 1) / 2) as usize;
        let mut tabs = Vec::with_capacity(mus.len());
        for &mu in mus {
            let mut t = vec![0u32; half + 1];
            for a in 1..=half {
                let v = ((a as u64 * mu as u64) % r as u64) as u32;
                t[a] = v.min(r - v);
            }
            tabs.push(t);
        }
        Mult {
            tabs,
            mus: mus.to_vec(),
        }
    }
    /// mu_j = lam^j for j = 1..=(n-1)/2 (the other half of the lags is the same set of unordered pairs).
    pub fn frobenius(g: &Group) -> Mult {
        let jmax = (g.n - 1) / 2;
        let mus: Vec<u32> = (1..=jmax).map(|j| g.lam_pow(j)).collect();
        Mult::from_units(g.r, &mus)
    }
    /// (n-1)/2 random units mu with mu != +-1 and pairwise distinct classes of multiplier (null multipliers).
    pub fn random_null(r: u32, count: usize, rng: &mut crate::rng::Rng) -> Mult {
        let mut mus: Vec<u32> = vec![];
        while mus.len() < count {
            let mu = 2 + rng.below((r - 3) as u64) as u32; // 2..=r-2
            let c = mu.min(r - mu);
            if mus.iter().all(|&m| m.min(r - m) != c) {
                mus.push(mu);
            }
        }
        Mult::from_units(r, &mus)
    }
    pub fn len(&self) -> usize {
        self.tabs.len()
    }
    /// Positive-control multipliers: for each lag a random m-subset S0 of the base is drawn, two distinct classes
    /// a != b of K(S0) are picked, and mu = b / a (mod r, random sign). Then class(mu a) = b is in K(S0), so every lag
    /// has at least one guaranteed hit (an exact floor of `count` hits per base). Returns None if the base is too poor.
    pub fn planted(
        r: u32,
        ds: &[u32],
        m: usize,
        count: usize,
        rng: &mut crate::rng::Rng,
    ) -> Option<Mult> {
        let n = ds.len();
        if n < m {
            return None;
        }
        let mut mus: Vec<u32> = vec![];
        let mut tries = 0;
        while mus.len() < count {
            tries += 1;
            if tries > 20000 {
                return None;
            }
            let mut idx: Vec<usize> = vec![];
            while idx.len() < m {
                let i = rng.below(n as u64) as usize;
                if !idx.contains(&i) {
                    idx.push(i);
                }
            }
            let mut ks: Vec<u32> = vec![];
            for mask in 0u32..(1u32 << m) {
                let mut sum: u64 = 0;
                for (i, &k) in idx.iter().enumerate() {
                    let d = ds[k] as u64;
                    sum += if (mask >> i) & 1 == 0 {
                        d
                    } else {
                        r as u64 - d
                    };
                }
                let v = (sum % r as u64) as u32;
                let c = v.min(r - v);
                if c != 0 && !ks.contains(&c) {
                    ks.push(c);
                }
            }
            if ks.len() < 2 {
                continue;
            }
            let ia = rng.below(ks.len() as u64) as usize;
            let mut ib = rng.below(ks.len() as u64) as usize;
            if ib == ia {
                ib = (ib + 1) % ks.len();
            }
            let (a, b) = (ks[ia], ks[ib]);
            let mut mu = ((b as u64 * crate::curve::pow_mod(a as u64, r as u64 - 2, r as u64))
                % r as u64) as u32;
            if rng.below(2) == 1 {
                mu = r - mu;
            }
            let c = mu.min(r - mu);
            if c <= 1 || mus.iter().any(|&x| x.min(r - x) == c) {
                continue;
            }
            mus.push(mu);
        }
        Some(Mult::from_units(r, &mus))
    }
}

pub fn binom(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let mut r = 1.0f64;
    for i in 0..k {
        r = r * (n - i) as f64 / (i + 1) as f64;
    }
    r
}

/// Rough count of inner-loop operations of one census (used for caps and runtime estimates).
pub fn est_ops(nx: usize, m: usize, jn: usize) -> f64 {
    let c = (1u64 << (m - 1)) as f64;
    binom(nx, m) * (2.0 * c + c * (jn as f64) * 2.0 * 2.0 + 8.0)
}

/// Depth-first enumeration of all m-subsets (indices into `ds`, ascending). At the leaf the visitor gets the
/// indices and the 2^(m-1) signed sums mod r (first leg sign +; the global negation gives the same classes).
pub fn for_each_subset<F: FnMut(&[usize], &[u32])>(ds: &[u32], r: u32, m: usize, visit: &mut F) {
    assert!(m >= 2);
    let n = ds.len();
    if n < m {
        return;
    }
    let mut idx = vec![0usize; m];
    let mut sums: Vec<Vec<u32>> = vec![Vec::new(); m];
    let mut leaf: Vec<u32> = Vec::with_capacity(1 << (m - 1));
    fn rec<F: FnMut(&[usize], &[u32])>(
        level: usize,
        start: usize,
        ds: &[u32],
        r: u32,
        m: usize,
        idx: &mut Vec<usize>,
        sums: &mut Vec<Vec<u32>>,
        leaf: &mut Vec<u32>,
        visit: &mut F,
    ) {
        // `level` legs are fixed (idx[..level]); choose leg number level (0-based).
        let n = ds.len();
        let remaining = m - level; // legs still to choose, including this one
        if level == 0 {
            for i in start..=(n - remaining) {
                idx[0] = i;
                sums[1].clear();
                sums[1].push(ds[i]);
                rec(1, i + 1, ds, r, m, idx, sums, leaf, visit);
            }
            return;
        }
        if level == m - 1 {
            for i in start..n {
                idx[level] = i;
                let d = ds[i];
                leaf.clear();
                for &v in sums[level].iter() {
                    let p = v + d;
                    leaf.push(if p >= r { p - r } else { p });
                    leaf.push(if v >= d { v - d } else { v + r - d });
                }
                visit(&idx[..], &leaf[..]);
            }
            return;
        }
        for i in start..=(n - remaining) {
            idx[level] = i;
            let d = ds[i];
            let (lo, hi) = sums.split_at_mut(level + 1);
            let prev = &lo[level];
            let cur = &mut hi[0];
            cur.clear();
            for &v in prev.iter() {
                let p = v + d;
                cur.push(if p >= r { p - r } else { p });
                cur.push(if v >= d { v - d } else { v + r - d });
            }
            rec(level + 1, i + 1, ds, r, m, idx, sums, leaf, visit);
        }
    }
    rec(0, 0, ds, r, m, &mut idx, &mut sums, &mut leaf, visit);
}

#[derive(Clone, Debug)]
pub struct Sample {
    pub idx: Vec<usize>,
    pub a: u32,
    pub b: u32,
    /// index of the multiplier in the Frobenius set (mu = mults[0].mus[t])
    pub t: usize,
}

#[derive(Debug)]
pub struct CensusOut {
    pub nx: usize,
    pub nsub: u64,
    /// number of (S, class) solution pairs = sum_S |K(S)|
    pub nsol: u64,
    /// sum over S of c(c-1), c = |K(S)| (for the independence prediction)
    pub sum_cc1: u64,
    /// cnt[a] = number of S with a in K(S)
    pub cnt: Vec<u32>,
    /// per multiplier set, per lag: shared (S, class, lag) triples
    pub u: Vec<Vec<u64>>,
    /// recorded coincidences of multiplier set 0 (up to cap), spread over the run
    pub samples: Vec<Sample>,
    /// recorded plain solutions (idx, class) with a class NOT in K(S) as a negative: (idx, in_class, out_class)
    pub sol_samples: Vec<(Vec<usize>, u32, u32)>,
    /// strata by number of distinct sigma-orbits among the legs (1, 2, >= 3): subsets, solution pairs, hits for
    /// multiplier sets 0 and 1 (all zero when no orbit labels were supplied)
    pub strat_nsub: [u64; 3],
    pub strat_nsol: [u64; 3],
    pub strat_u: [[u64; 3]; 2],
    /// Low-height single-orbit stratum "1L" (needs orbit exponents): (tuple, lag index) pairs where the tuple has all
    /// its legs in one sigma-orbit with circular spread s and the lag j = t + 1 satisfies m^2 2^s (2^(j/2) + 1)^2 < r.
    /// On such pairs a Frobenius coincidence is impossible (valuation argument, PROTOCOL 3.4).
    /// s1l_nsub = single-orbit tuples with at least one such lag; s1l_pairs = number of (tuple, lag) pairs;
    /// s1l_cc1 = sum of c (c - 1) over those pairs (independence-model weight); s1l_u[mi] = hits of set mi.
    pub s1l_nsub: u64,
    pub s1l_pairs: u64,
    pub s1l_cc1: u64,
    pub s1l_u: [u64; 2],
}

/// Orbit labels of the base positions: orbit id, exponent k within the orbit (x = sigma^k(rep)), and the orbit length n.
pub struct OrbInfo<'a> {
    pub orbit: &'a [u32],
    pub k: Option<&'a [u32]>,
    pub n: u32,
}

/// Full census of one base. `mults[0]` is conventionally the Frobenius set, `mults[1]` the null set.
pub fn census(
    ds: &[u32],
    r: u32,
    m: usize,
    mults: &[&Mult],
    sample_cap: usize,
    sol_sample_cap: usize,
    orb: Option<&[u32]>,
) -> CensusOut {
    let oi = orb.map(|o| OrbInfo {
        orbit: o,
        k: None,
        n: 0,
    });
    census_ex(ds, r, m, mults, sample_cap, sol_sample_cap, oi.as_ref())
}

/// Largest circular gap removed: the length s of the shortest arc of Z/n containing all exponents `ks`.
pub fn circular_spread(ks: &mut Vec<u32>, n: u32) -> u32 {
    ks.sort_unstable();
    let mut max_gap = 0;
    for i in 0..ks.len() {
        let next = if i + 1 < ks.len() {
            ks[i + 1]
        } else {
            ks[0] + n
        };
        max_gap = max_gap.max(next - ks[i]);
    }
    n - max_gap
}

/// Is the (tuple of m legs in one orbit with spread s, lag j) pair provably free of Frobenius coincidences?
/// Condition m^2 2^s (2^(j/2) + 1)^2 < r (norm of a nonzero element of the prime above r is at least r).
pub fn low_height(m: usize, s: u32, j: u32, r: u32) -> bool {
    let hmax = (m as f64) * 2f64.powf(s as f64 / 2.0) * (2f64.powf(j as f64 / 2.0) + 1.0);
    hmax * hmax < r as f64
}

pub fn census_ex(
    ds: &[u32],
    r: u32,
    m: usize,
    mults: &[&Mult],
    sample_cap: usize,
    sol_sample_cap: usize,
    oinfo: Option<&OrbInfo>,
) -> CensusOut {
    let orb: Option<&[u32]> = oinfo.map(|o| o.orbit);
    // lowtab[s][t]: lag j = t + 1 of the first multiplier set
    let lowtab: Option<Vec<Vec<bool>>> = match oinfo {
        Some(OrbInfo { k: Some(_), n, .. }) if !mults.is_empty() => {
            let nl = mults[0].len();
            Some(
                (0..*n)
                    .map(|sp| {
                        (0..nl)
                            .map(|t| low_height(m, sp, t as u32 + 1, r))
                            .collect()
                    })
                    .collect(),
            )
        }
        _ => None,
    };
    let mut s1l = (0u64, 0u64, 0u64, [0u64; 2]);
    let half = ((r - 1) / 2) as usize;
    let mut stamp = vec![0u32; half + 1];
    let mut sid: u32 = 0;
    let mut cnt = vec![0u32; half + 1];
    let mut u: Vec<Vec<u64>> = mults.iter().map(|mu| vec![0u64; mu.len()]).collect();
    let mut nsub = 0u64;
    let mut nsol = 0u64;
    let mut sum_cc1 = 0u64;
    let mut samples: Vec<Sample> = vec![];
    let mut sol_samples: Vec<(Vec<usize>, u32, u32)> = vec![];
    let mut ks: Vec<u32> = Vec::with_capacity(1 << (m - 1));
    let mut strat_nsub = [0u64; 3];
    let mut strat_nsol = [0u64; 3];
    let mut strat_u = [[0u64; 3]; 2];
    let rr = r;
    let mut visit = |idx: &[usize], leaf: &[u32]| {
        sid += 1;
        nsub += 1;
        ks.clear();
        for &v in leaf {
            let c = v.min(rr - v);
            if c != 0 && stamp[c as usize] != sid {
                stamp[c as usize] = sid;
                ks.push(c);
            }
        }
        let c = ks.len() as u64;
        nsol += c;
        sum_cc1 += c * c.saturating_sub(1);
        for &a in ks.iter() {
            cnt[a as usize] += 1;
        }
        let stratum = match orb {
            Some(o) => {
                let mut ids: Vec<u32> = idx.iter().map(|&i| o[i]).collect();
                ids.sort_unstable();
                ids.dedup();
                Some(ids.len().min(3) - 1)
            }
            None => None,
        };
        if let Some(sx) = stratum {
            strat_nsub[sx] += 1;
            strat_nsol[sx] += c;
        }
        let mut lowrow: Option<&Vec<bool>> = None;
        if let (Some(0), Some(tab), Some(oi)) = (stratum, lowtab.as_ref(), oinfo) {
            if let Some(kk) = oi.k {
                let mut pos: Vec<u32> = idx.iter().map(|&i| kk[i]).collect();
                let sp = circular_spread(&mut pos, oi.n);
                let row = &tab[sp as usize];
                let npairs = row.iter().filter(|&&b| b).count() as u64;
                if npairs > 0 {
                    s1l.0 += 1;
                    s1l.1 += npairs;
                    s1l.2 += npairs * c * c.saturating_sub(1);
                    lowrow = Some(row);
                }
            }
        }
        for (mi, mu) in mults.iter().enumerate() {
            for (t, tab) in mu.tabs.iter().enumerate() {
                let mut hits = 0u64;
                for &a in ks.iter() {
                    let b = tab[a as usize];
                    if stamp[b as usize] == sid {
                        hits += 1;
                        if mi == 0
                            && samples.len() < sample_cap
                            && (nsub % 7 == 1 || samples.len() < 4)
                        {
                            samples.push(Sample {
                                idx: idx.to_vec(),
                                a,
                                b,
                                t,
                            });
                        }
                    }
                }
                u[mi][t] += hits;
                if let (Some(sx), true) = (stratum, mi < 2) {
                    strat_u[mi][sx] += hits;
                }
                if let (Some(row), true) = (lowrow, mi < 2) {
                    if row[t] {
                        s1l.3[mi] += hits;
                    }
                }
            }
        }
        if sol_samples.len() < sol_sample_cap && nsub % 5 == 2 && !ks.is_empty() {
            // a class outside K(S) (smallest) as the negative
            let mut out = 1u32;
            while (out as usize) <= half && stamp[out as usize] == sid {
                out += 1;
            }
            if (out as usize) <= half {
                sol_samples.push((idx.to_vec(), ks[0], out));
            }
        }
    };
    for_each_subset(ds, r, m, &mut visit);
    CensusOut {
        nx: ds.len(),
        nsub,
        nsol,
        sum_cc1,
        cnt,
        u,
        samples,
        sol_samples,
        strat_nsub,
        strat_nsol,
        strat_u,
        s1l_nsub: s1l.0,
        s1l_pairs: s1l.1,
        s1l_cc1: s1l.2,
        s1l_u: s1l.3,
    }
}

/// Independence prediction of sum_t U[t] for a multiplier set with `jn` lags:
/// each of the c classes of K(S) maps under mu to a class that is uniform on the other (r-3)/2 classes.
pub fn u_pred(out: &CensusOut, r: u32, jn: usize) -> f64 {
    out.sum_cc1 as f64 / ((r as f64 - 3.0) / 2.0) * jn as f64
}

/// Only the solution marks (cnt) of a base: used for the shared-leg lower bound.
pub fn solution_counts(ds: &[u32], r: u32, m: usize) -> Vec<u32> {
    let half = ((r - 1) / 2) as usize;
    let mut stamp = vec![0u32; half + 1];
    let mut cnt = vec![0u32; half + 1];
    let mut sid = 0u32;
    let mut visit = |_idx: &[usize], leaf: &[u32]| {
        sid += 1;
        for &v in leaf {
            let c = v.min(r - v) as usize;
            if c != 0 && stamp[c] != sid {
                stamp[c] = sid;
                cnt[c] += 1;
            }
        }
    };
    for_each_subset(ds, r, m, &mut visit);
    cnt
}

/// Indicator statistics from a solution-count array and a multiplier table set.
pub struct IndStats {
    /// P(chi = 1) over the (r-1)/2 target classes
    pub p: f64,
    /// per lag: number of classes a with chi(a) = chi(mu a) = 1
    pub joint: Vec<u64>,
    /// per lag: number of classes with chi(a) == chi(mu a)
    pub concord: Vec<u64>,
    /// per lag: number of classes with cnt(a) == cnt(mu a)
    pub count_equal: Vec<u64>,
    pub ncls: u64,
}

pub fn ind_stats(cnt: &[u32], mu: &Mult) -> IndStats {
    let half = cnt.len() - 1;
    let mut ones = 0u64;
    for a in 1..=half {
        if cnt[a] > 0 {
            ones += 1;
        }
    }
    let mut joint = vec![0u64; mu.len()];
    let mut concord = vec![0u64; mu.len()];
    let mut count_equal = vec![0u64; mu.len()];
    for (t, tab) in mu.tabs.iter().enumerate() {
        for a in 1..=half {
            let b = tab[a] as usize;
            let (ca, cb) = (cnt[a], cnt[b]);
            if ca > 0 && cb > 0 {
                joint[t] += 1;
            }
            if (ca > 0) == (cb > 0) {
                concord[t] += 1;
            }
            if ca == cb {
                count_equal[t] += 1;
            }
        }
    }
    IndStats {
        p: ones as f64 / half as f64,
        joint,
        concord,
        count_equal,
        ncls: half as u64,
    }
}

/// Pearson correlation of the indicators at one lag.
pub fn corr(p: f64, joint: u64, ncls: u64) -> f64 {
    if p <= 0.0 || p >= 1.0 {
        return f64::NAN;
    }
    (joint as f64 / ncls as f64 - p * p) / (p * (1.0 - p))
}

/// Lemma A2-prime in signed form, checked against the multiplier that produced the coincidence. For a recorded
/// coincidence (a in K(S), b = class(mu a) in K(S)) and every pair of sign patterns (p, q) with signed sums
/// sa = +-a and sb = +-b: (i) sb = s mu sa for a sign s (this ties the multiplier table to the unit mu), (ii) the set I
/// where p and q differ is nonempty and proper, (iii) the I-part of p sums to h sa with h = (1 - s mu)/2 computed with
/// the inverse of 2 mod r (the odd-order halving). Condition (iii) follows algebraically from (i) and the definition of
/// the signed sums, so this is a consistency check of the sample bookkeeping (tables, signs, lag index) against the
/// lemma, not an independent test of the lemma. Returns (pairs_checked, all_ok).
pub fn halving_check(ds: &[u32], r: u32, idx: &[usize], a: u32, b: u32, mu: u32) -> (u32, bool) {
    let m = idx.len();
    let inv2 = ((r as u64 + 1) / 2) as u64;
    let mut pa: Vec<(u32, u32)> = vec![]; // (mask, signed sum)
    let mut pb: Vec<(u32, u32)> = vec![];
    for mask in 0u32..(1u32 << m) {
        let mut s: u64 = 0;
        for (i, &k) in idx.iter().enumerate() {
            let d = ds[k] as u64;
            if (mask >> i) & 1 == 0 {
                s += d;
            } else {
                s += r as u64 - d;
            }
        }
        let s = (s % r as u64) as u32;
        let c = s.min(r - s);
        if c == a {
            pa.push((mask, s));
        }
        if c == b {
            pb.push((mask, s));
        }
    }
    let full = (1u32 << m) - 1;
    let mut ok = !pa.is_empty() && !pb.is_empty();
    let mut checked = 0u32;
    for &(p, sa) in &pa {
        for &(q, sb) in &pb {
            checked += 1;
            let mu_sa = ((mu as u64 * sa as u64) % r as u64) as u32;
            let sgn: i32 = if sb == mu_sa {
                1
            } else if sb == (r - mu_sa) % r {
                -1
            } else {
                ok = false;
                continue;
            };
            let i_set = p ^ q;
            if i_set == 0 || i_set == full {
                ok = false;
                continue;
            }
            let mut t: u64 = 0;
            for (i, &k) in idx.iter().enumerate() {
                if (i_set >> i) & 1 == 1 {
                    let d = ds[k] as u64;
                    t += if (p >> i) & 1 == 0 { d } else { r as u64 - d };
                }
            }
            let t = t % r as u64;
            let num = if sgn == 1 {
                (1 + r as u64 - mu as u64) % r as u64
            } else {
                (1 + mu as u64) % r as u64
            };
            let h = (num * inv2) % r as u64;
            if t != (h * sa as u64) % r as u64 {
                ok = false;
            }
        }
    }
    (checked, ok)
}

/// Proposition Sigma, setwise form: for how many (target class a with Sol(a) nonempty, lag j in 1..n-1) is the
/// whole solution set closed under the coordinatewise shift S -> sigma^j S (every image is a solution of a)?
/// Returns (targets with solutions, pairs checked, closed pairs). Exact; memory ~ number of solutions.
pub fn sigma_closure(
    set: &crate::bases::BaseSet,
    f: &crate::field::Fq,
    r: u32,
    m: usize,
) -> (u64, u64, u64) {
    use std::collections::{HashMap, HashSet};
    let half = ((r - 1) / 2) as usize;
    let index_of: HashMap<u32, usize> = set.xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
    let mut sols: HashMap<u32, HashSet<Vec<usize>>> = HashMap::new();
    let mut stamp = vec![0u32; half + 1];
    let mut sid = 0u32;
    let mut visit = |idx: &[usize], leaf: &[u32]| {
        sid += 1;
        for &v in leaf {
            let c = v.min(r - v);
            if c != 0 && stamp[c as usize] != sid {
                stamp[c as usize] = sid;
                sols.entry(c).or_default().insert(idx.to_vec());
            }
        }
    };
    for_each_subset(&set.ds, r, m, &mut visit);
    let (mut targets, mut pairs, mut closed) = (0u64, 0u64, 0u64);
    for (_a, s) in sols.iter() {
        targets += 1;
        for j in 1..f.n {
            pairs += 1;
            let mut all = true;
            for t in s.iter() {
                let mut img: Vec<usize> = Vec::with_capacity(m);
                let mut inside = true;
                for &i in t {
                    match index_of.get(&f.frob(set.xs[i], j)) {
                        Some(&k) => img.push(k),
                        None => {
                            inside = false;
                            break;
                        }
                    }
                }
                if !inside {
                    all = false;
                    break;
                }
                img.sort_unstable();
                if !s.contains(&img) {
                    all = false;
                    break;
                }
            }
            if all {
                closed += 1;
            }
        }
    }
    (targets, pairs, closed)
}
