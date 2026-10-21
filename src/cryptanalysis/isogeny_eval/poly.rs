//! Dense polynomials over [`PrimeField`]: Karatsuba or three-prime NTT
//! multiplication, power series inversion (Newton), remainder by a monic divisor with a cached
//! reversed inverse, balanced products, and product / remainder trees for
//! multipoint evaluation.
//!
//! A polynomial is a little-endian coefficient vector (`a[i]` is the
//! coefficient of `Z^i`) of canonical field elements.  Inputs may carry
//! trailing zeros; outputs of [`mul`] have length `len(a) + len(b) − 1` and
//! are not trimmed, outputs of [`MonicDivisor::rem`] are trimmed.  The empty
//! vector is the zero polynomial.
//!
//! Every coefficient product is counted as one `M` (schoolbook products are
//! counted in bulk), matching the round-4 unit.  [`mul`] follows the field's
//! [`PolyMul`]: Karatsuba (`O(n^{1.585})`, the round-4 arithmetic), or the
//! three-prime NTT of [`super::ntt`] (`O(n log n)` word multiplications,
//! counted as described there) from [`NTT_THRESHOLD`] coefficients on.
//! Everything else here (Newton reciprocal, remainders, product and
//! remainder trees) goes through [`mul`].

use super::counted_fp::{OpCounter, PolyMul, PrimeField};
use super::ntt::{mul_ntt, ntt_mul_cost, NTT_WORD_COST_IN_M};

/// Below this length of the shorter operand, [`mul`] uses the schoolbook
/// product (the round-4 threshold).
pub const KARATSUBA_THRESHOLD: usize = 24;

/// Below this degree of divisor or quotient, [`MonicDivisor::rem`] uses
/// classical long division instead of a Newton reciprocal.
pub const NEWTON_DIVISION_THRESHOLD: usize = 32;

/// With [`PolyMul::Auto`], [`mul`] considers the NTT only when the shorter
/// operand has at least this many coefficients, and then takes whichever of
/// Karatsuba and NTT has the smaller counted cost for that exact shape
/// ([`karatsuba_cost`] against [`ntt_mul_cost`], `W` at one `M`).  Measured
/// (`research/isogeny_fibers_20261010/round5/r5_3_ntt_sqrt_velu`,
/// `threshold.json`): for balanced products the NTT count first wins at
/// length 512 (ratio 0.92; 1.27 at 256), for `8n × n` at `n = 768`, and
/// for the `2n × n` shapes of Newton iteration only from `n ≈ 2048`
/// (power-of-two padding), so a single length threshold would be wrong for
/// some shape; the per-shape comparison is exact for the counted unit.  In
/// contended wall time this implementation's NTT is slower per counted unit
/// than the lazily reduced schoolbook products (wall crossover near 4096,
/// same file).
pub const NTT_THRESHOLD: usize = 256;

/// The exact number of `M` [`mul_karatsuba`] counts for operand lengths
/// `la`, `lb` (it mirrors the recursion; memoised on the operand shapes,
/// of which a balanced recursion has `O(log n)`).
pub fn karatsuba_cost(la: usize, lb: usize) -> u64 {
    fn go(la: usize, lb: usize, memo: &mut std::collections::HashMap<(usize, usize), u64>) -> u64 {
        if la == 0 || lb == 0 {
            return 0;
        }
        let (la, lb) = if la >= lb { (la, lb) } else { (lb, la) };
        if lb <= KARATSUBA_THRESHOLD {
            return (la * lb) as u64;
        }
        if let Some(&c) = memo.get(&(la, lb)) {
            return c;
        }
        let c = if la >= 2 * lb {
            let full = la / lb;
            let rest = la % lb;
            full as u64 * go(lb, lb, memo) + go(rest, lb, memo)
        } else {
            let m = la / 2;
            if lb <= m {
                go(m, lb, memo) + go(la - m, lb, memo)
            } else {
                // z0 = a0·b0, z2 = a1·b1, z1 = (a0 + a1)(b0 + b1).
                go(m, m, memo) + go(la - m, lb - m, memo) + go(la - m, m.max(lb - m), memo)
            }
        };
        memo.insert((la, lb), c);
        c
    }
    go(la, lb, &mut std::collections::HashMap::new())
}

/// Drop trailing zero coefficients.
pub fn trim(a: &mut Vec<u64>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

/// The degree of `a`, `None` for the zero polynomial.
pub fn degree(a: &[u64]) -> Option<usize> {
    a.iter().rposition(|&c| c != 0)
}

/// `a(x)` by Horner's rule (`deg a` multiplications).
pub fn eval<C: OpCounter>(f: &PrimeField<C>, a: &[u64], x: u64) -> u64 {
    match degree(a) {
        None => 0,
        Some(d) => a[..d]
            .iter()
            .rev()
            .fold(a[d], |acc, &c| f.add(f.mul(acc, x), c)),
    }
}

/// `a + b`.
pub fn add<C: OpCounter>(f: &PrimeField<C>, a: &[u64], b: &[u64]) -> Vec<u64> {
    let n = a.len().max(b.len());
    (0..n)
        .map(|i| f.add(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)))
        .collect()
}

/// `a − b`.
pub fn sub<C: OpCounter>(f: &PrimeField<C>, a: &[u64], b: &[u64]) -> Vec<u64> {
    let n = a.len().max(b.len());
    (0..n)
        .map(|i| f.sub(*a.get(i).unwrap_or(&0), *b.get(i).unwrap_or(&0)))
        .collect()
}

fn add_into<C: OpCounter>(f: &PrimeField<C>, acc: &mut [u64], src: &[u64], off: usize) {
    for (slot, &v) in acc[off..].iter_mut().zip(src) {
        *slot = f.add(*slot, v);
    }
}

/// Schoolbook product with lazy `u128` accumulation; counts
/// `len(a)·len(b)` multiplications.
pub fn mul_schoolbook<C: OpCounter>(f: &PrimeField<C>, a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    f.count_muls((a.len() * b.len()) as u64);
    let p = f.p() as u128;
    let fold = f.fold();
    let mut acc = vec![0u128; a.len() + b.len() - 1];
    for (i, &ai) in a.iter().enumerate() {
        for (j, &bj) in b.iter().enumerate() {
            acc[i + j] += ai as u128 * bj as u128;
        }
        if i % fold == fold - 1 {
            for v in acc.iter_mut() {
                *v %= p;
            }
        }
    }
    acc.into_iter().map(|v| (v % p) as u64).collect()
}

/// `a·b` by the field's [`PolyMul`] choice: Karatsuba
/// ([`mul_karatsuba`]) or the NTT ([`mul_ntt`]; under [`PolyMul::Auto`]
/// only when the shorter operand has at least [`NTT_THRESHOLD`]
/// coefficients and the NTT's counted cost is the smaller one for this
/// shape).  The result has length
/// `len(a) + len(b) − 1` (empty if either is empty) and does not depend on
/// the choice; the counts do.
pub fn mul<C: OpCounter>(f: &PrimeField<C>, a: &[u64], b: &[u64]) -> Vec<u64> {
    let use_ntt = match f.poly_mul() {
        PolyMul::Karatsuba => false,
        PolyMul::Ntt => true,
        PolyMul::Auto => {
            a.len().min(b.len()) >= NTT_THRESHOLD && {
                let (w, m) = ntt_mul_cost(a.len(), b.len(), std::ptr::eq(a, b));
                w * NTT_WORD_COST_IN_M + m <= karatsuba_cost(a.len(), b.len())
            }
        }
    };
    if use_ntt {
        mul_ntt(f, a, b)
    } else {
        mul_karatsuba(f, a, b)
    }
}

/// `a·b` by Karatsuba (schoolbook below [`KARATSUBA_THRESHOLD`]),
/// whatever the field's [`PolyMul`].  The result has length
/// `len(a) + len(b) − 1` (empty if either is empty).
pub fn mul_karatsuba<C: OpCounter>(f: &PrimeField<C>, a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let (a, b) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    if b.len() <= KARATSUBA_THRESHOLD {
        return mul_schoolbook(f, a, b);
    }
    let mut out = vec![0u64; a.len() + b.len() - 1];
    if a.len() >= 2 * b.len() {
        // Unbalanced: chunks of `a` against `b`.
        for (k, chunk) in a.chunks(b.len()).enumerate() {
            let part = mul_karatsuba(f, chunk, b);
            add_into(f, &mut out, &part, k * b.len());
        }
        return out;
    }
    let m = a.len() / 2;
    let (a0, a1) = a.split_at(m);
    if b.len() <= m {
        let z0 = mul_karatsuba(f, a0, b);
        let z1 = mul_karatsuba(f, a1, b);
        add_into(f, &mut out, &z0, 0);
        add_into(f, &mut out, &z1, m);
        return out;
    }
    let (b0, b1) = b.split_at(m);
    let z0 = mul_karatsuba(f, a0, b0);
    let z2 = mul_karatsuba(f, a1, b1);
    let z1 = mul_karatsuba(f, &add(f, a0, a1), &add(f, b0, b1));
    let mut mid = sub(f, &sub(f, &z1, &z0), &z2);
    trim(&mut mid);
    add_into(f, &mut out, &z0, 0);
    add_into(f, &mut out, &z2, 2 * m);
    add_into(f, &mut out, &mid, m);
    out
}

/// `1/a mod Z^n` by Newton iteration, or `None` if `a(0) = 0` or `a` is
/// empty.  Returns exactly `n` coefficients.
pub fn inv_series<C: OpCounter>(f: &PrimeField<C>, a: &[u64], n: usize) -> Option<Vec<u64>> {
    let mut g = vec![f.inv(*a.first()?)?];
    let mut m = 1;
    while m < n {
        let m2 = (2 * m).min(n);
        let at = &a[..a.len().min(m2)];
        let mut e = mul(f, at, &g);
        e.resize(m2, 0);
        // e = 2 − a·g
        for v in e.iter_mut() {
            *v = f.neg(*v);
        }
        e[0] = f.add(e[0], 2 % f.p());
        let mut ng = mul(f, &g, &e);
        ng.truncate(m2);
        g = ng;
        m = m2;
    }
    g.resize(n, 0);
    Some(g)
}

/// A monic divisor `b` with a lazily computed, cached `1/rev(b)` so repeated
/// remainders by `b` share the Newton reciprocal.
#[derive(Clone, Debug)]
pub struct MonicDivisor {
    b: Vec<u64>,
    rev_inv: Vec<u64>,
}

impl MonicDivisor {
    /// Wrap `b`.  `None` unless `b` (after trimming) is monic of degree
    /// at least 1.
    pub fn new(mut b: Vec<u64>) -> Option<Self> {
        trim(&mut b);
        if b.len() < 2 || b.last() != Some(&1) {
            return None;
        }
        Some(MonicDivisor {
            b,
            rev_inv: Vec::new(),
        })
    }

    /// The divisor.
    pub fn poly(&self) -> &[u64] {
        &self.b
    }

    /// `a mod b` (trimmed).
    pub fn rem<C: OpCounter>(&mut self, f: &PrimeField<C>, a: &[u64]) -> Vec<u64> {
        let mut a = a.to_vec();
        trim(&mut a);
        let b = &self.b;
        let db = b.len() - 1;
        if a.len() <= db {
            return a;
        }
        let dq = a.len() - 1 - db;
        if db < NEWTON_DIVISION_THRESHOLD || dq < NEWTON_DIVISION_THRESHOLD {
            let mut r = a;
            for i in (db..r.len()).rev() {
                let q = r[i];
                if q == 0 {
                    continue;
                }
                for j in 0..db {
                    let t = f.mul(q, b[j]);
                    r[i - db + j] = f.sub(r[i - db + j], t);
                }
                r[i] = 0;
            }
            r.truncate(db);
            trim(&mut r);
            return r;
        }
        let need = dq + 1;
        if self.rev_inv.len() < need {
            let rb: Vec<u64> = b.iter().rev().copied().collect();
            // rev(b)(0) = 1 since b is monic, so the reciprocal exists.
            self.rev_inv = inv_series(f, &rb, need).unwrap_or_default();
        }
        let ra: Vec<u64> = a.iter().rev().take(need).copied().collect();
        let mut rq = mul(f, &ra, &self.rev_inv[..need]);
        rq.resize(need, 0);
        let q: Vec<u64> = rq.into_iter().rev().collect();
        let qb = mul(f, &q, b);
        let mut r = sub(f, &a, &qb);
        r.truncate(db);
        trim(&mut r);
        r
    }
}

/// The product of `factors` by a balanced binary tree ([`mul`] at each
/// node).  The empty product is `[1]`.
pub fn product<C: OpCounter>(f: &PrimeField<C>, factors: Vec<Vec<u64>>) -> Vec<u64> {
    let mut cur = factors;
    if cur.is_empty() {
        return vec![1];
    }
    while cur.len() > 1 {
        let mut next = Vec::with_capacity(cur.len().div_ceil(2));
        let mut it = cur.into_iter();
        while let Some(x) = it.next() {
            match it.next() {
                Some(y) => next.push(mul(f, &x, &y)),
                None => next.push(x),
            }
        }
        cur = next;
    }
    cur.pop().unwrap_or_else(|| vec![1])
}

/// A product tree over the leaves `Z − x_i`, used for multipoint
/// evaluation by a remainder tree.
#[derive(Clone, Debug)]
pub struct ProductTree {
    /// `levels[0]` are the leaves; the last level has one node, the root.
    levels: Vec<Vec<MonicDivisor>>,
}

impl ProductTree {
    /// Build the tree for the points `xs` (`O(M(n) log n)`).
    pub fn new<C: OpCounter>(f: &PrimeField<C>, xs: &[u64]) -> Self {
        let leaf = |x: u64| MonicDivisor {
            b: vec![f.neg(x), 1],
            rev_inv: Vec::new(),
        };
        let mut levels: Vec<Vec<MonicDivisor>> = vec![xs.iter().map(|&x| leaf(x)).collect()];
        while levels.last().is_some_and(|l| l.len() > 1) {
            let prev = &levels[levels.len() - 1];
            let next = prev
                .chunks(2)
                .map(|pair| match pair {
                    [l, r] => MonicDivisor {
                        b: mul(f, &l.b, &r.b),
                        rev_inv: Vec::new(),
                    },
                    [one] => one.clone(),
                    _ => unreachable!("chunks(2) yields one or two nodes"),
                })
                .collect();
            levels.push(next);
        }
        ProductTree { levels }
    }

    /// The number of leaves.
    pub fn len(&self) -> usize {
        self.levels[0].len()
    }

    /// Whether the tree has no leaves.
    pub fn is_empty(&self) -> bool {
        self.levels[0].is_empty()
    }

    /// `Π (Z − x_i)` (`[1]` for no leaves).
    pub fn root(&self) -> Vec<u64> {
        match self.levels.last().and_then(|l| l.first()) {
            Some(r) => r.b.clone(),
            None => vec![1],
        }
    }

    /// `g(x_i)` for every leaf, in leaf order, by a remainder tree.
    /// Reciprocals computed at each node are cached for later calls.
    pub fn evaluate<C: OpCounter>(&mut self, f: &PrimeField<C>, g: &[u64]) -> Vec<u64> {
        if self.is_empty() {
            return Vec::new();
        }
        let top = self.levels.len() - 1;
        let mut cur = vec![self.levels[top][0].rem(f, g)];
        for lv in (0..top).rev() {
            let level = &mut self.levels[lv];
            let n = level.len();
            let mut next = Vec::with_capacity(n);
            for (j, parent) in cur.iter().enumerate() {
                for c in [2 * j, 2 * j + 1] {
                    if c < n {
                        next.push(level[c].rem(f, parent));
                    }
                }
            }
            cur = next;
        }
        cur.into_iter()
            .map(|r| r.first().copied().unwrap_or(0))
            .collect()
    }
}
