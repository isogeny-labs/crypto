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
        Mult { tabs, mus: mus.to_vec() }
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
        for (mi, mu) in mults.iter().enumerate() {
            for (t, tab) in mu.tabs.iter().enumerate() {
                let mut hits = 0u64;
                for &a in ks.iter() {
                    let b = tab[a as usize];
                    if stamp[b as usize] == sid {
                        hits += 1;
                        if mi == 0 && samples.len() < sample_cap && (nsub % 7 == 1 || samples.len() < 4) {
                            samples.push(Sample { idx: idx.to_vec(), a, b });
                        }
                    }
                }
                u[mi][t] += hits;
                if let (Some(sx), true) = (stratum, mi < 2) {
                    strat_u[mi][sx] += hits;
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
    CensusOut { nx: ds.len(), nsub, nsol, sum_cc1, cnt, u, samples, sol_samples, strat_nsub, strat_nsol, strat_u }
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
    IndStats { p: ones as f64 / half as f64, joint, concord, count_equal, ncls: half as u64 }
}

/// Pearson correlation of the indicators at one lag.
pub fn corr(p: f64, joint: u64, ncls: u64) -> f64 {
    if p <= 0.0 || p >= 1.0 {
        return f64::NAN;
    }
    (joint as f64 / ncls as f64 - p * p) / (p * (1.0 - p))
}

/// Lemma A2-prime in signed form for one coincidence: every pair of sign patterns (p, q) with signed sums
/// +-a and +-b differs on a nonempty proper set I, and 2 * sum_{i in I} s_i d_i = sum_p - sum_q (mod r),
/// i.e. the I-part of the first decomposition is the unique half of (A - B) in the odd-order group.
/// Returns (pairs_checked, all_ok).
pub fn halving_check(ds: &[u32], r: u32, idx: &[usize], a: u32, b: u32) -> (u32, bool) {
    let m = idx.len();
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
    for &(p, sp) in &pa {
        for &(q, sq) in &pb {
            let i_set = p ^ q;
            checked += 1;
            if i_set == 0 || i_set == full {
                ok = false;
                continue;
            }
            // sum over I of the signs of p
            let mut t: u64 = 0;
            for (i, &k) in idx.iter().enumerate() {
                if (i_set >> i) & 1 == 1 {
                    let d = ds[k] as u64;
                    t += if (p >> i) & 1 == 0 { d } else { r as u64 - d };
                }
            }
            let lhs = (2 * (t % r as u64)) % r as u64;
            let rhs = (sp as u64 + r as u64 - sq as u64) % r as u64;
            if lhs != rhs {
                ok = false;
            }
        }
    }
    (checked, ok)
}

/// Proposition Sigma, setwise form: for how many (target class a with Sol(a) nonempty, lag j in 1..n-1) is the
/// whole solution set closed under the coordinatewise shift S -> sigma^j S (every image is a solution of a)?
/// Returns (targets with solutions, pairs checked, closed pairs). Exact; memory ~ number of solutions.
pub fn sigma_closure(set: &crate::bases::BaseSet, f: &crate::field::Fq, r: u32, m: usize) -> (u64, u64, u64) {
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
