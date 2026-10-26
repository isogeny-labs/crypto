//! Exact counting: multisets of shifts, diagonal classes, D(n,m).

use std::collections::HashMap;

pub fn binom(n: u64, k: u64) -> u128 {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut r: u128 = 1;
    for i in 0..k {
        r = r * (n - i) as u128 / (i + 1) as u128; // exact at each step
    }
    r
}

/// N_m = C(n+m-1, m): multisets of m shifts in Z/n.
pub fn n_patterns(n: u64, m: u64) -> u128 {
    binom(n + m - 1, m)
}

/// D(n,m) = C(n+m-1,m)/n. Valid (an integer, free diagonal action) for n prime, m < n.
pub fn d_nm(n: u64, m: u64) -> u128 {
    let t = n_patterns(n, m);
    assert!(t % n as u128 == 0, "n does not divide C(n+m-1,m): action not free");
    t / n as u128
}

/// n^(m-1) / D(n,m): the over-count of the n^{m-1} baseline.
pub fn baseline_ratio(n: u64, m: u64) -> f64 {
    (n as f64).powi(m as i32 - 1) / d_nm(n, m) as f64
}

/// Sorted tuples (multisets) of size m over 0..nb.
pub fn all_patterns(nb: usize, m: usize) -> Vec<Vec<u8>> {
    fn rec(nb: usize, m: usize, start: usize, cur: &mut Vec<u8>, out: &mut Vec<Vec<u8>>) {
        if cur.len() == m {
            out.push(cur.clone());
            return;
        }
        for k in start..nb {
            cur.push(k as u8);
            rec(nb, m, k, cur, out);
            cur.pop();
        }
    }
    let mut out = vec![];
    rec(nb, m, 0, &mut vec![], &mut out);
    out
}

pub fn pack(p: &[u8]) -> u64 {
    let mut v = 0u64;
    for &k in p {
        v = (v << 5) | k as u64;
    }
    v
}

pub fn shift_pattern(p: &[u8], j: usize, n: usize) -> Vec<u8> {
    let mut q: Vec<u8> = p.iter().map(|&k| ((k as usize + j) % n) as u8).collect();
    q.sort();
    q
}

/// Canonical member of the diagonal class: lexicographically least shift.
pub fn canon(p: &[u8], n: usize) -> Vec<u8> {
    (0..n).map(|j| shift_pattern(p, j, n)).min().unwrap()
}

/// Sorted multiplicity vector (descending) of a pattern.
pub fn shape(p: &[u8]) -> Vec<u8> {
    let mut v: Vec<u8> = vec![];
    let mut i = 0;
    while i < p.len() {
        let mut j = i;
        while j < p.len() && p[j] == p[i] {
            j += 1;
        }
        v.push((j - i) as u8);
        i = j;
    }
    v.sort_by(|a, b| b.cmp(a));
    v
}

pub struct PatternTable {
    pub nb: usize,
    pub m: usize,
    pub pats: Vec<Vec<u8>>,
    pub index: HashMap<u64, usize>,
    /// class id of each pattern; classes are diagonal orbits when nb is the
    /// cyclic group size n (`diagonal = true`), otherwise one class per pattern.
    pub class_of: Vec<usize>,
    pub class_pats: Vec<Vec<usize>>,
    /// pattern index of the canonical representative of each class
    pub rep: Vec<usize>,
    pub diagonal: bool,
}

impl PatternTable {
    pub fn new(nb: usize, m: usize, diagonal: bool) -> PatternTable {
        let pats = all_patterns(nb, m);
        let index: HashMap<u64, usize> = pats.iter().enumerate().map(|(i, p)| (pack(p), i)).collect();
        let mut class_of = vec![usize::MAX; pats.len()];
        let mut class_pats: Vec<Vec<usize>> = vec![];
        let mut rep = vec![];
        if diagonal {
            let mut cid: HashMap<u64, usize> = HashMap::new();
            for (i, p) in pats.iter().enumerate() {
                let c = pack(&canon(p, nb));
                let id = *cid.entry(c).or_insert_with(|| {
                    class_pats.push(vec![]);
                    rep.push(index[&c]);
                    class_pats.len() - 1
                });
                class_of[i] = id;
                class_pats[id].push(i);
            }
        } else {
            for i in 0..pats.len() {
                class_of[i] = i;
                class_pats.push(vec![i]);
                rep.push(i);
            }
        }
        PatternTable { nb, m, pats, index, class_of, class_pats, rep, diagonal }
    }
    pub fn n_classes(&self) -> usize {
        self.class_pats.len()
    }
}

/// C(x, k) as f64 for small k (x real count of available items).
pub fn binom_f(x: u64, k: u64) -> f64 {
    binom(x, k) as f64
}

/// Null weight of a pattern given liftable-x counts per block:
/// prod over distinct shifts s with multiplicity mu_s of C(X_s, mu_s).
/// Expected decompositions of one target in this pattern: weight * 2^m / #E.
pub fn pattern_weight(p: &[u8], xs: &[u64]) -> f64 {
    let mut w = 1.0;
    let mut i = 0;
    while i < p.len() {
        let mut j = i;
        while j < p.len() && p[j] == p[i] {
            j += 1;
        }
        w *= binom_f(xs[p[i] as usize], (j - i) as u64);
        i = j;
    }
    w
}
