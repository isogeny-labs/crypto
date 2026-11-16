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
    assert!(
        t % n as u128 == 0,
        "n does not divide C(n+m-1,m): action not free"
    );
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
        let index: HashMap<u64, usize> =
            pats.iter().enumerate().map(|(i, p)| (pack(p), i)).collect();
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
        PatternTable {
            nb,
            m,
            pats,
            index,
            class_of,
            class_pats,
            rep,
            diagonal,
        }
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

/// Parity-aware null weight. Every target lies in hE, a subset of 2E; a point is in 2E
/// iff Tr(x) = Tr(a) (halving criterion, tested). A decomposition therefore needs an
/// EVEN number of "odd" points (Tr(x) != Tr(a)). With A_s even and B_s odd liftable x in
/// block s and multiplicity mu_s of that block in the pattern, the number of x-tuples
/// with an even number of odd members is (prod_s P_s(1) + prod_s P_s(-1)) / 2,
/// P_s(t) = sum_j C(B_s, j) C(A_s, mu_s - j) t^j. This returns twice that number, so it
/// reduces to 2 * pattern_weight when B = 0 (the factor is absorbed by the per-row
/// rescaling in stats::pool). `xodd` shorter than `xs` is read as zeros.
pub fn pattern_weight_parity(p: &[u8], xs: &[u64], xodd: &[u64]) -> f64 {
    let (mut plus, mut minus) = (1.0, 1.0);
    let mut i = 0;
    while i < p.len() {
        let mut j = i;
        while j < p.len() && p[j] == p[i] {
            j += 1;
        }
        let mu = (j - i) as u64;
        let s = p[i] as usize;
        let x = xs[s];
        let b = xodd.get(s).copied().unwrap_or(0).min(x);
        let a = x - b;
        let (mut p1, mut pm) = (0.0, 0.0);
        for t in 0..=mu.min(b) {
            let term = binom_f(b, t) * binom_f(a, mu - t);
            p1 += term;
            pm += if t % 2 == 0 { term } else { -term };
        }
        plus *= p1;
        minus *= pm;
        i = j;
    }
    plus + minus
}

/// Mod-4 null weight for a = 0 curves (review round 2, blocking defect 1). For a = 0,
/// E(F_{2^n}) has a CYCLIC 2-part Z/4 and every target lies in 4E. A liftable x (point +-Q)
/// has a class in E/4E = Z/4: type 0 (class 0), type 2 (class 2, in 2E but not 4E) or odd
/// (class +-1, not in 2E; Tr(x) != Tr(a)). Counting the 2^m sign choices whose class sum is 0
/// mod 4 (verified by test `mod4_sign_count_structure_for_a0`): 0 if the number O of odd points is
/// odd; 2^(m-1) if O > 0 is even; 2^m if O = 0 and the number T of type-2 points is even, else 0.
/// With Z_s = X_s - O_s - T_s the per-block type-0 count, the tuple-weight (sign choices / 2^m) is
/// 0, 1/2 or [T even] respectively, and this returns TWICE the sum of that weight over x-tuples:
///   (E_even - N0) + 2 * N0_Teven,
///   E_even  = #tuples with O even          = (prod P_s(1) + prod P_s(-1)) / 2,
///             P_s(t) = sum_j C(O_s,j) C(X_s-O_s, mu_s-j) t^j,
///   N0      = #tuples with O = 0           = prod C(X_s-O_s, mu_s),
///   N0_Teven= #tuples with O = 0, T even   = (N0 + prod Q_s(-1)) / 2,
///             Q_s(t) = sum_i C(T_s,i) C(Z_s, mu_s-i) t^i.
/// It reduces to 2 * pattern_weight when O = T = 0 (the factor 2 is absorbed by the per-row
/// rescaling in stats::pool). `xodd`/`xtwo` shorter than `xs` are read as zeros.
pub fn pattern_weight_mod4(p: &[u8], xs: &[u64], xodd: &[u64], xtwo: &[u64]) -> f64 {
    let (mut plus, mut minus, mut n0, mut qminus) = (1.0, 1.0, 1.0, 1.0);
    let mut i = 0;
    while i < p.len() {
        let mut j = i;
        while j < p.len() && p[j] == p[i] {
            j += 1;
        }
        let mu = (j - i) as u64;
        let s = p[i] as usize;
        let x = xs[s];
        let o = xodd.get(s).copied().unwrap_or(0).min(x);
        let t = xtwo.get(s).copied().unwrap_or(0).min(x - o);
        let (even, z) = (x - o, x - o - t);
        let (mut p1, mut pm) = (0.0, 0.0);
        for k in 0..=mu.min(o) {
            let term = binom_f(o, k) * binom_f(even, mu - k);
            p1 += term;
            pm += if k % 2 == 0 { term } else { -term };
        }
        plus *= p1;
        minus *= pm;
        n0 *= binom_f(even, mu);
        let mut qm = 0.0;
        for k in 0..=mu.min(t) {
            let term = binom_f(t, k) * binom_f(z, mu - k);
            qm += if k % 2 == 0 { term } else { -term };
        }
        qminus *= qm;
        i = j;
    }
    let e_even = (plus + minus) / 2.0;
    let n0_teven = (n0 + qminus) / 2.0;
    (e_even - n0) + 2.0 * n0_teven
}
