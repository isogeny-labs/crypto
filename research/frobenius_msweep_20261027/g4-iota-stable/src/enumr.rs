//! Exact enumeration of m-term decompositions, orbit quotient, rank, ANF degree, cost model.
//!
//! A decomposition of a target R in E(F_q) over a base F is an unordered set of m points of
//! P_F = {Q : x(Q) in F, [h]Q != O} lying in m pairwise distinct classes, with sum R. The class
//! of Q is the 4-set {+-Q, +-Q + T}. Two decompositions are in the same orbit iff they have the
//! same multiset of (class, sign); sign is taken modulo T (log mod ne/2). On an iota-stable F
//! the even-weight T-translations act freely on decompositions, so every orbit has exactly
//! 2^(m-1) elements (identity checked exactly per base).

use crate::curve::{Group, NONE};

#[derive(Clone, Copy, Debug)]
pub struct Item {
    pub ell: u32,
    pub cid: u32,
    pub sc: u32,
    pub x: u32,
}

pub struct Base {
    pub ne: u32,
    pub mm: u32,
    pub items: Vec<Item>,
    pub ncls: usize,
    /// number of x in the set that carry points
    pub nx: usize,
    pub next: Vec<u32>,
    /// fraction of items whose T-translate is also an item (1.0 on an iota-stable set)
    pub closure: f64,
}

pub fn build_base(g: &Group, xs: &[u32]) -> Base {
    let (ne, mm) = (g.ne, g.m);
    let mut raw: Vec<(u32, u32, u32, u32)> = vec![]; // (lc, sign, ell, x)
    let mut nx = 0;
    for &x in xs {
        let l = g.logx[x as usize];
        if l == NONE {
            continue;
        }
        if (g.h as u64 * l as u64) % ne as u64 == 0 {
            continue; // order divides the cofactor: trivial log in the odd subgroup
        }
        nx += 1;
        for ell in [l, ne - l] {
            let lam = ell % mm;
            let neg = mm - lam;
            assert!(lam != 0 && lam != neg, "degenerate class x={x}");
            let lc = lam.min(neg);
            raw.push((lc, (lam != lc) as u32, ell, x));
        }
    }
    let mut lcs: Vec<u32> = raw.iter().map(|r| r.0).collect();
    lcs.sort_unstable();
    lcs.dedup();
    let mut items: Vec<Item> = raw
        .iter()
        .map(|&(lc, sg, ell, x)| {
            let cid = lcs.binary_search(&lc).unwrap() as u32;
            Item {
                ell,
                cid,
                sc: 2 * cid + sg,
                x,
            }
        })
        .collect();
    items.sort_unstable_by_key(|i| (i.sc, i.ell));
    assert!(2 * lcs.len() < 4096, "too many classes for 12-bit packing");
    let n = items.len();
    let mut next = vec![n as u32; n];
    for i in (0..n).rev() {
        next[i] = if i + 1 == n {
            n as u32
        } else if items[i + 1].cid > items[i].cid {
            (i + 1) as u32
        } else {
            next[i + 1]
        };
    }
    let mut ells: Vec<u32> = items.iter().map(|i| i.ell).collect();
    ells.sort_unstable();
    let closed = items
        .iter()
        .filter(|i| ells.binary_search(&((i.ell + mm) % ne)).is_ok())
        .count();
    let closure = if n == 0 {
        1.0
    } else {
        closed as f64 / n as f64
    };
    Base {
        ne,
        mm,
        items,
        ncls: lcs.len(),
        nx,
        next,
        closure,
    }
}

#[inline]
fn add_mod(a: u32, b: u32, m: u32) -> u32 {
    let s = a as u64 + b as u64;
    (if s >= m as u64 { s - m as u64 } else { s }) as u32
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

fn rec(
    b: &Base,
    m: usize,
    depth: usize,
    start: usize,
    sum: u32,
    key: u128,
    out: &mut Vec<(u32, u128)>,
) {
    let n = b.items.len();
    for i in start..n {
        let it = b.items[i];
        let s2 = add_mod(sum, it.ell, b.ne);
        let k2 = key | ((it.sc as u128) << (12 * depth));
        if depth + 1 == m {
            out.push((s2, k2));
        } else {
            rec(b, m, depth + 1, b.next[i] as usize, s2, k2, out);
        }
    }
}

/// All decompositions as (sum, orbit key). Exact, exhaustive over every target at once.
pub fn enumerate(b: &Base, m: usize, cap: f64) -> Result<Vec<(u32, u128)>, String> {
    assert!((1..=8).contains(&m));
    let ub = binom(b.items.len(), m);
    if ub > cap {
        return Err(format!(
            "C({},{})={ub:.3e} exceeds cap {cap:.3e}",
            b.items.len(),
            m
        ));
    }
    let mut out = Vec::with_capacity((ub as usize / 2).min(1 << 22) + 16);
    rec(b, m, 0, 0, 0, 0, &mut out);
    Ok(out)
}

pub struct Tally {
    pub ne: u32,
    pub d: Vec<u32>,
    pub o: Vec<u32>,
    /// off[s]..off[s+1] indexes `keys`: the distinct orbit keys of target s
    pub off: Vec<u32>,
    pub keys: Vec<u128>,
    pub total: u64,
}

pub fn tally(ne: u32, mut raw: Vec<(u32, u128)>) -> Tally {
    raw.sort_unstable();
    let total = raw.len() as u64;
    let mut d = vec![0u32; ne as usize];
    let mut o = vec![0u32; ne as usize];
    let mut keys = Vec::new();
    let mut prev: Option<(u32, u128)> = None;
    for &(s, k) in &raw {
        d[s as usize] += 1;
        if prev != Some((s, k)) {
            o[s as usize] += 1;
            keys.push(k);
        }
        prev = Some((s, k));
    }
    let mut off = vec![0u32; ne as usize + 1];
    for s in 0..ne as usize {
        off[s + 1] = off[s] + o[s];
    }
    Tally {
        ne,
        d,
        o,
        off,
        keys,
        total,
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Mom {
    pub n: u64,
    pub mean: f64,
    pub var: f64,
    pub fano: f64,
    pub hit: f64,
}

/// Exact population moments of a count array over the targets s = 0, step, 2 step, ...
pub fn moments(cnt: &[u32], step: u32) -> Mom {
    let (mut n, mut s1, mut s2, mut hit) = (0u64, 0f64, 0f64, 0u64);
    let mut s = 0usize;
    while s < cnt.len() {
        let c = cnt[s] as f64;
        n += 1;
        s1 += c;
        s2 += c * c;
        hit += (cnt[s] > 0) as u64;
        s += step as usize;
    }
    let mean = s1 / n as f64;
    let var = s2 / n as f64 - mean * mean;
    Mom {
        n,
        mean,
        var,
        fano: if mean > 0.0 { var / mean } else { f64::NAN },
        hit: hit as f64 / n as f64,
    }
}

/// True iff D[s] = 2^(m-1) O[s] for every target s.
pub fn identity_exact(t: &Tally, m: usize) -> bool {
    let f = 1u64 << (m - 1);
    (0..t.ne as usize).all(|s| t.d[s] as u64 == f * t.o[s] as u64)
}

/// Quotient search in E/<T>: signed-class selections (distinct classes) per sum mod ne/2.
/// On an iota-stable base every selection lifts to 2^m points tuples, 2^(m-1) with sum s0 and 2^(m-1)
/// with sum s0 + T, so O[s] = quot[s mod ne/2] exactly for both s and s + ne/2.
pub fn quot_counts(b: &Base, m: usize) -> Vec<u32> {
    let mm = b.mm;
    let mut sc: Vec<(u32, u32)> = vec![]; // (cid, value mod mm), one per distinct signed class
    let mut last = u32::MAX;
    for it in &b.items {
        if it.sc != last {
            sc.push((it.cid, it.ell % mm));
            last = it.sc;
        }
    }
    let n = sc.len();
    let mut next = vec![n; n];
    for i in (0..n).rev() {
        next[i] = if i + 1 == n {
            n
        } else if sc[i + 1].0 > sc[i].0 {
            i + 1
        } else {
            next[i + 1]
        };
    }
    fn go(
        sc: &[(u32, u32)],
        next: &[usize],
        mm: u32,
        m: usize,
        depth: usize,
        start: usize,
        sum: u32,
        cnt: &mut [u32],
    ) {
        for i in start..sc.len() {
            let s2 = add_mod(sum, sc[i].1, mm);
            if depth + 1 == m {
                cnt[s2 as usize] += 1;
            } else {
                go(sc, next, mm, m, depth + 1, next[i], s2, cnt);
            }
        }
    }
    let mut cnt = vec![0u32; mm as usize];
    go(&sc, &next, mm, m, 0, 0, 0, &mut cnt);
    cnt
}

pub fn n_signed_classes(b: &Base) -> usize {
    let mut v: Vec<u32> = b.items.iter().map(|i| i.sc).collect();
    v.dedup();
    v.len()
}

/// Meet-in-the-middle cost model (group-law-free probe counts). Two orientations exist: a left table of
/// all floor(m/2)-subsets probed with all ceil(m/2)-subsets once per target, or the reverse. With t targets
/// amortising one table, the cost is table + t * probe; the cheaper orientation is chosen per arm and per t.
/// `flipped` is true when the table holds the ceil(m/2)-subsets. (`tests/identities.rs` runs an honest
/// counted MITM on tiny bases and checks that it finds exactly the decompositions D[s].)
pub struct Cost {
    pub table: f64,
    pub probe: f64,
    pub flipped: bool,
}
pub fn mitm_total(n_items: usize, m: usize, t: f64) -> Cost {
    let a = m / 2;
    let (lo, hi) = (binom(n_items, a), binom(n_items, m - a));
    // orientation 0: table lo, probe hi; orientation 1: table hi, probe lo
    if lo + t * hi <= hi + t * lo {
        Cost {
            table: lo,
            probe: hi,
            flipped: false,
        }
    } else {
        Cost {
            table: hi,
            probe: lo,
            flipped: true,
        }
    }
}

// ------------------------------------------------------------------ ANF ----

/// Algebraic degree and term count of the indicator of `set` as a Boolean function of the n
/// coordinate bits of x (degree is independent of the basis; the term count is not).
pub fn anf(n: u32, set: &[u32]) -> (u32, u64) {
    let sz = 1usize << n;
    let mut f = vec![0u8; sz];
    for &x in set {
        f[x as usize] = 1;
    }
    for i in 0..n {
        let bit = 1usize << i;
        for mask in 0..sz {
            if mask & bit != 0 {
                f[mask] ^= f[mask ^ bit];
            }
        }
    }
    let (mut deg, mut terms) = (0u32, 0u64);
    for mask in 0..sz {
        if f[mask] == 1 {
            terms += 1;
            deg = deg.max((mask as u32).count_ones());
        }
    }
    (deg, terms)
}

/// Algebraic degree (max over the n output bits) of a parametrisation map from l free bits
/// (index bits of `table`) to x in F_2^n. Linear maps (subspace bases) have degree 1.
pub fn param_degree(n: u32, l: u32, table: &[u32]) -> u32 {
    assert_eq!(table.len(), 1usize << l);
    let mut best = 0;
    for j in 0..n {
        let mut f: Vec<u8> = table.iter().map(|&x| ((x >> j) & 1) as u8).collect();
        for i in 0..l {
            let bit = 1usize << i;
            for mask in 0..f.len() {
                if mask & bit != 0 {
                    f[mask] ^= f[mask ^ bit];
                }
            }
        }
        for mask in 0..f.len() {
            if f[mask] == 1 {
                best = best.max((mask as u32).count_ones());
            }
        }
    }
    best
}

// ----------------------------------------------------------------- rank ----

fn mulmod(a: u64, b: u64, p: u64) -> u64 {
    // p < 2^32 (asserted in Elim::new), so the product fits in u64
    (a * b) % p
}
fn powmod(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, p);
        }
        a = mulmod(a, a, p);
        e >>= 1;
    }
    r
}

/// Incremental row-echelon rank over F_p.
pub struct Elim {
    p: u64,
    piv: Vec<Option<Vec<u64>>>,
    pub rank: usize,
}
impl Elim {
    pub fn new(p: u64, ncols: usize) -> Elim {
        assert!(p < (1u64 << 32), "modulus must be below 2^32");
        Elim {
            p,
            piv: vec![None; ncols],
            rank: 0,
        }
    }
    /// Adds a sparse row (col, coefficient mod p). True iff the rank grew.
    pub fn add(&mut self, row: &[(usize, u64)]) -> bool {
        let p = self.p;
        let mut v = vec![0u64; self.piv.len()];
        for &(c, a) in row {
            v[c] = (v[c] + a) % p;
        }
        for c in 0..v.len() {
            if v[c] == 0 {
                continue;
            }
            match &self.piv[c] {
                Some(r) => {
                    let f = v[c];
                    for j in c..v.len() {
                        if r[j] != 0 {
                            v[j] = (v[j] + p - mulmod(f, r[j], p)) % p;
                        }
                    }
                }
                None => {
                    let inv = powmod(v[c], p - 2, p);
                    for j in c..v.len() {
                        v[j] = mulmod(v[j], inv, p);
                    }
                    self.piv[c] = Some(v);
                    self.rank += 1;
                    return true;
                }
            }
        }
        false
    }
}

pub fn row_of_key(key: u128, m: usize, p: u64) -> Vec<(usize, u64)> {
    (0..m)
        .map(|i| {
            let sc = ((key >> (12 * i)) & 0xFFF) as usize;
            (sc >> 1, if sc & 1 == 0 { 1 } else { p - 1 })
        })
        .collect()
}

#[derive(Clone, Debug, Default)]
pub struct RankOut {
    pub active: usize,
    pub rank_final: usize,
    pub targets_used: u64,
    pub rows_total: u64,
    pub t50: Option<u64>,
    /// whole targets needed to reach 90 percent rank (a probe is paid in full)
    pub t90: Option<u64>,
    /// fractional targets: completed targets plus the share of the last target's rows that was needed
    pub t90f: Option<f64>,
    pub t100: Option<u64>,
    pub rows90: Option<u64>,
    pub stalled: bool,
}

/// Streams targets (in the given order), adds the orbit rows of each (rows of one target in a seeded
/// random order), and records how many targets/rows were needed to reach 50/90/100 percent of the active
/// columns (columns that occur in at least one orbit row of any listed target). The stream stops at 90
/// percent rank when there are more than 120 active columns (dense elimination is quadratic; t100 is then
/// NA by design), otherwise at full rank, a long stall (3 active + 300 consecutive redundant rows) or the
/// end of the order.
pub fn rank_stream(t: &Tally, order: &[u32], m: usize, ncls: usize, p: u64, seed: u64) -> RankOut {
    let mut act = vec![false; ncls];
    for &s in order {
        for k in t.off[s as usize]..t.off[s as usize + 1] {
            for (c, _) in row_of_key(t.keys[k as usize], m, p) {
                act[c] = true;
            }
        }
    }
    let active = act.iter().filter(|&&a| a).count();
    let mut out = RankOut {
        active,
        ..Default::default()
    };
    if active == 0 {
        return out;
    }
    let thr = |f: f64| ((f * active as f64).ceil() as usize).max(1);
    let (th50, th90, th100) = (thr(0.5), thr(0.9), active);
    let full_cap = active <= 120;
    let mut el = Elim::new(p, ncls);
    let mut rng = crate::bases::Rng(seed);
    let (mut rows, mut since) = (0u64, 0u64);
    let stall_cap = 3 * active as u64 + 300;
    'outer: for (ti, &s) in order.iter().enumerate() {
        let (lo, hi) = (t.off[s as usize] as usize, t.off[s as usize + 1] as usize);
        let mut idx: Vec<usize> = (lo..hi).collect();
        for i in (1..idx.len()).rev() {
            idx.swap(i, rng.below(i as u64 + 1) as usize);
        }
        let nr = idx.len();
        let tn = ti as u64 + 1;
        for (j, &k) in idx.iter().enumerate() {
            rows += 1;
            if el.add(&row_of_key(t.keys[k], m, p)) {
                since = 0;
            } else {
                since += 1;
            }
            if out.t50.is_none() && el.rank >= th50 {
                out.t50 = Some(tn);
            }
            if out.t90.is_none() && el.rank >= th90 {
                out.t90 = Some(tn);
                out.t90f = Some(ti as f64 + (j + 1) as f64 / nr as f64);
                out.rows90 = Some(rows);
                if !full_cap {
                    out.targets_used = tn;
                    break 'outer;
                }
            }
            if out.t100.is_none() && el.rank >= th100 {
                out.t100 = Some(tn);
            }
        }
        out.targets_used = tn;
        if el.rank >= th100 {
            break;
        }
        if since > stall_cap {
            out.stalled = true;
            break;
        }
    }
    out.rows_total = rows;
    out.rank_final = el.rank;
    out
}
