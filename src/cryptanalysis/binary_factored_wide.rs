//! # The factored `S₄` root find on two- and three-word binary fields.
//!
//! [`super::binary_radical_root::FactoredOracle`] decides an `m = 3`
//! subspace-factor-base target without the quartic: the roots of
//! `S₄(X₁, X₂, X, x_R)` in `X` are `x(R ± P₁ ± P₂)`, so
//!
//! ```text
//!     S₄(X₁, X₂, X, x_R) = S₃(u, x_R, X) · S₃(u', x_R, X),   {u, u'} = roots of S₃(X₁, X₂, ·),
//! ```
//!
//! and a pair costs one Artin–Schreier step for `{u, u'}` plus one per
//! rational `u`, with no gcd and nothing that grows with the factor-base
//! dimension `l`.  That oracle is `u64`-only and needs odd `n ≤ 63` (its
//! Artin–Schreier step is a half-trace).  This module is the same route
//! for [`Gf2_128`] (`n ≤ 127`) and [`Gf2W`] (`n ≤ 191`, the ECC2K-130
//! field), for odd **and even** `n`: the Artin–Schreier step is one
//! byte-table application of an `F₂`-linear solver, the half-trace for odd
//! `n` and a precomputed linear inverse of `z ↦ z² + z` on the trace-0
//! hyperplane for even `n` (as [`super::binary_radical_root::ArtinSchreier`]).
//!
//! `b = 1` throughout, as in `FactoredOracle` and `semaev_decomp`:
//! `S₃(p, q, X) = (p + q)²X² + pqX + (pq)² + 1`.  The Semaev polynomials do
//! not depend on `a`, so this covers both `y² + xy = x³ + x² + 1` and the
//! ECC2K-130 curve `y² + xy = x³ + 1`.
//!
//! ## Exceptional pairs
//!
//! When `S₃(X₁, X₂, ·)` has no root in `F_{2ⁿ}`, `u, u'` are conjugate and a
//! rational `X` would be a root of both factors, so the pair has no root
//! unless `x_R ∈ {X₁, X₂, 1/X₁, 1/X₂}` (the `FactoredOracle` guard).  Those
//! guarded pairs and the degenerate ones (`X₁ = X₂`, `X₁X₂ = 0`, `u = x_R`,
//! `u·x_R = 0`) are decided by evaluating the quartic at every point of
//! `V` (`2^l` Horner steps): exact, and rare except at very small `l`.
//!
//! ## Measured basis
//!
//! Workspace `docs/notes/summation-rootfind-20261124/REPORT_1C.md`
//! (counted field operations at L0 latency weights, `[pcc:unverified]`): with
//! the inversions batched per row, this route costs 0.11–0.22× the subspace
//! gcd `gcd(f, L_V mod f)` at n = 113, 127, 131 for l = 8..14, about 500 ns
//! per pair at two words and 2 µs at three words on a 2.3 GHz Xeon,
//! independent of `l`.

use super::certicom131_symlp::gf2w::{self as gw, Gf2W, W3};
use super::semaev_decomp::Gf2_128;

/// The field operations the factored route needs, for one element width.
pub trait FactorField {
    type E: Copy + PartialEq + Eq + Ord + std::fmt::Debug;
    fn degree(&self) -> u32;
    fn zero(&self) -> Self::E;
    fn one(&self) -> Self::E;
    fn add(&self, a: Self::E, b: Self::E) -> Self::E;
    fn mul(&self, a: Self::E, b: Self::E) -> Self::E;
    fn sqr(&self, a: Self::E) -> Self::E;
    fn inv(&self, a: Self::E) -> Self::E;
    /// Montgomery batch inversion; zeros stay zero.
    fn batch_inv(&self, xs: &mut [Self::E], scratch: &mut Vec<Self::E>);
    fn is_zero(&self, a: Self::E) -> bool {
        a == self.zero()
    }
    /// The element whose bit pattern is `v` (the factor-base encoding).
    fn elem(&self, v: u64) -> Self::E;
    /// `z^i`.
    fn mono(&self, i: u32) -> Self::E;
    /// Byte `j` of the bit pattern.
    fn byte(&self, a: Self::E, j: usize) -> u8;
    /// `a < 2^l` as a bit pattern, i.e. `a ∈ V`.
    fn below(&self, a: Self::E, l: u32) -> bool;
    fn bit(&self, a: Self::E, i: u32) -> bool;
    fn xor(&self, a: Self::E, b: Self::E) -> Self::E {
        self.add(a, b)
    }
}

impl FactorField for Gf2_128 {
    type E = u128;
    fn degree(&self) -> u32 {
        self.n
    }
    fn zero(&self) -> u128 {
        0
    }
    fn one(&self) -> u128 {
        1
    }
    #[inline]
    fn add(&self, a: u128, b: u128) -> u128 {
        a ^ b
    }
    #[inline]
    fn mul(&self, a: u128, b: u128) -> u128 {
        Gf2_128::mul(self, a, b)
    }
    #[inline]
    fn sqr(&self, a: u128) -> u128 {
        Gf2_128::sqr(self, a)
    }
    fn inv(&self, a: u128) -> u128 {
        Gf2_128::inv(self, a)
    }
    fn batch_inv(&self, xs: &mut [u128], scratch: &mut Vec<u128>) {
        Gf2_128::batch_inv(self, xs, scratch)
    }
    fn elem(&self, v: u64) -> u128 {
        v as u128
    }
    fn mono(&self, i: u32) -> u128 {
        1u128 << i
    }
    #[inline]
    fn byte(&self, a: u128, j: usize) -> u8 {
        (a >> (8 * j)) as u8
    }
    #[inline]
    fn below(&self, a: u128, l: u32) -> bool {
        a >> l == 0
    }
    fn bit(&self, a: u128, i: u32) -> bool {
        (a >> i) & 1 == 1
    }
}

impl FactorField for Gf2W {
    type E = W3;
    fn degree(&self) -> u32 {
        self.n
    }
    fn zero(&self) -> W3 {
        gw::ZERO
    }
    fn one(&self) -> W3 {
        gw::ONE
    }
    #[inline]
    fn add(&self, a: W3, b: W3) -> W3 {
        gw::xor(a, b)
    }
    #[inline]
    fn mul(&self, a: W3, b: W3) -> W3 {
        Gf2W::mul(self, &a, &b)
    }
    #[inline]
    fn sqr(&self, a: W3) -> W3 {
        Gf2W::sqr(self, &a)
    }
    fn inv(&self, a: W3) -> W3 {
        Gf2W::inv(self, &a)
    }
    fn batch_inv(&self, xs: &mut [W3], scratch: &mut Vec<W3>) {
        Gf2W::batch_inv(self, xs, scratch)
    }
    fn elem(&self, v: u64) -> W3 {
        gw::from_u64(v)
    }
    fn mono(&self, i: u32) -> W3 {
        self.monomial(i)
    }
    #[inline]
    fn byte(&self, a: W3, j: usize) -> u8 {
        (a[j / 8] >> (8 * (j % 8))) as u8
    }
    #[inline]
    fn below(&self, a: W3, l: u32) -> bool {
        debug_assert!(l < 64);
        a[0] >> l == 0 && a[1] == 0 && a[2] == 0
    }
    fn bit(&self, a: W3, i: u32) -> bool {
        (a[(i / 64) as usize] >> (i % 64)) & 1 == 1
    }
}

/// An `F₂`-linear map applied a byte at a time.
struct LinTab<E> {
    tab: Vec<[E; 256]>,
}

impl<E: Copy> LinTab<E> {
    fn new<F: FactorField<E = E>>(f: &F, images: &[E]) -> Self {
        let bytes = images.len().div_ceil(8);
        let mut tab = vec![[f.zero(); 256]; bytes];
        for (j, t) in tab.iter_mut().enumerate() {
            for v in 1..256usize {
                let i = 8 * j + v.trailing_zeros() as usize;
                let prev = t[v & (v - 1)];
                t[v] = if i < images.len() {
                    f.add(prev, images[i])
                } else {
                    prev
                };
            }
        }
        Self { tab }
    }

    #[inline]
    fn apply<F: FactorField<E = E>>(&self, f: &F, x: E) -> E {
        let mut r = f.zero();
        for (j, t) in self.tab.iter().enumerate() {
            let b = f.byte(x, j);
            if b != 0 {
                r = f.add(r, t[b as usize]);
            }
        }
        r
    }
}

/// Solves `z² + z = c` in `F_{2ⁿ}` for every `n`: a trace test and one
/// byte-table application.
pub struct ArtinSchreierAny<E> {
    trace_mask: E,
    solve: LinTab<E>,
}

impl<E: Copy + PartialEq> ArtinSchreierAny<E> {
    pub fn new<F: FactorField<E = E>>(f: &F) -> Self {
        let n = f.degree();
        let tr = |x: E| {
            let (mut acc, mut p) = (f.zero(), x);
            for _ in 0..n {
                acc = f.add(acc, p);
                p = f.sqr(p);
            }
            acc
        };
        let mut trace_mask = f.zero();
        for i in 0..n {
            let t = tr(f.mono(i));
            assert!(t == f.zero() || t == f.one(), "the trace lies in F₂");
            if t == f.one() {
                trace_mask = f.add(trace_mask, f.mono(i));
            }
        }
        let images: Vec<E> = if n % 2 == 1 {
            // H(c) = Σ_{k ≤ (n−1)/2} c^{4^k}: H(c)² + H(c) = c + Tr(c).
            (0..n)
                .map(|i| {
                    let (mut acc, mut p) = (f.zero(), f.mono(i));
                    for _ in 0..=(n - 1) / 2 {
                        acc = f.add(acc, p);
                        p = f.sqr(f.sqr(p));
                    }
                    acc
                })
                .collect()
        } else {
            // Gaussian elimination on (φ(zⁱ), zⁱ), φ(z) = z² + z, kept reduced at
            // the pivots; then M(eᵢ) solves eᵢ (+ t₁ for a trace-1 basis element).
            let mut rows: Vec<(u32, E, E)> = Vec::new();
            for i in 0..n {
                let zi = f.mono(i);
                let (mut v, mut pre) = (f.add(f.sqr(zi), zi), zi);
                for &(p, rv, rp) in &rows {
                    if f.bit(v, p) {
                        v = f.add(v, rv);
                        pre = f.add(pre, rp);
                    }
                }
                if let Some(p) = (0..n).rev().find(|&b| f.bit(v, b)) {
                    for r in rows.iter_mut() {
                        if f.bit(r.1, p) {
                            r.1 = f.add(r.1, v);
                            r.2 = f.add(r.2, pre);
                        }
                    }
                    rows.push((p, v, pre));
                }
            }
            assert_eq!(rows.len() as u32, n - 1, "z² + z has rank n − 1");
            let solve_any = |c: E| -> E {
                let (mut v, mut pre) = (c, f.zero());
                for &(p, rv, rp) in &rows {
                    if f.bit(v, p) {
                        v = f.add(v, rv);
                        pre = f.add(pre, rp);
                    }
                }
                assert!(v == f.zero(), "trace-0 input");
                pre
            };
            let parity = |x: E| {
                (0..n)
                    .filter(|&i| f.bit(x, i) && f.bit(trace_mask, i))
                    .count()
                    % 2
            };
            let t1 = (0..n)
                .map(|i| f.mono(i))
                .find(|&b| parity(b) == 1)
                .expect("a basis element of trace 1");
            (0..n)
                .map(|i| {
                    let e = f.mono(i);
                    solve_any(if parity(e) == 1 { f.add(e, t1) } else { e })
                })
                .collect()
        };
        Self {
            trace_mask,
            solve: LinTab::new(f, &images),
        }
    }

    /// `Tr(c)`, 0 or 1.
    #[inline]
    pub fn trace<F: FactorField<E = E>>(&self, f: &F, c: E) -> u32 {
        let mut acc = 0u32;
        for j in 0..self.solve.tab.len() {
            acc ^= (f.byte(c, j) & f.byte(self.trace_mask, j)).count_ones();
        }
        acc & 1
    }

    /// A root of `z² + z = c` when `Tr(c) = 0` (the other root is `z + 1`).
    #[inline]
    pub fn solve<F: FactorField<E = E>>(&self, f: &F, c: E) -> Option<E> {
        (self.trace(f, c) == 0).then(|| self.solve.apply(f, c))
    }
}

/// How one pair was decided.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairCase {
    /// `S₃(x₁, x₂, ·)` split and every `S₃(u, x_R, ·)` was solved in closed form.
    Factored,
    /// `S₃(x₁, x₂, ·)` has no rational root and the guard did not fire: no root.
    Rejected,
    /// Degenerate or guarded pair, decided by evaluation over `V`.
    ByEvaluation,
}

/// The factored root find for one field and one factor-base dimension `l`.
pub struct FactoredWide<E> {
    pub l: u32,
    as_: ArtinSchreierAny<E>,
}

impl<E: Copy + PartialEq + Eq + Ord + std::fmt::Debug> FactoredWide<E> {
    pub fn new<F: FactorField<E = E>>(f: &F, l: u32) -> Self {
        assert!(l < 64 && l <= f.degree(), "factor base dimension");
        Self {
            l,
            as_: ArtinSchreierAny::new(f),
        }
    }

    /// Roots of `A X² + B X + C` (`A·B ≠ 0`, `ab_inv = 1/(A·B)`) by one
    /// Artin–Schreier step: `X = (B/A)·t` with `t² + t = A·C/B²`.
    #[inline]
    fn as_roots<F: FactorField<E = E>>(
        &self,
        f: &F,
        a: E,
        b: E,
        c: E,
        ab_inv: E,
    ) -> Option<[E; 2]> {
        let (inv_a, inv_b) = (f.mul(b, ab_inv), f.mul(a, ab_inv));
        let w = f.mul(f.mul(a, c), f.sqr(inv_b));
        let t = self.as_.solve(f, w)?;
        let k = f.mul(b, inv_a);
        let x0 = f.mul(k, t);
        Some([x0, f.add(x0, k)])
    }

    /// The two roots of `S₃(p, q, X) = (p + q)²X² + pqX + (pq)² + 1`, when it
    /// splits over `F_{2ⁿ}` and is non-degenerate (`p ≠ q`, `pq ≠ 0`). One
    /// inversion inline.
    pub fn s3_roots<F: FactorField<E = E>>(&self, f: &F, p: E, q: E) -> Option<[E; 2]> {
        let a = f.sqr(f.add(p, q));
        let b = f.mul(p, q);
        if f.is_zero(a) || f.is_zero(b) {
            return None;
        }
        let c = f.add(f.sqr(b), f.one());
        self.as_roots(f, a, b, c, f.inv(f.mul(a, b)))
    }

    /// `S₄(x₁, x₂, X, x_R)` as coefficients `c₀ … c₄` of `X` (b = 1).
    pub fn quartic<F: FactorField<E = E>>(f: &F, x1: E, x2: E, xr: E) -> [E; 5] {
        let s = f.add(x1, x2);
        let p = f.mul(x1, x2);
        let (s2, p2) = (f.sqr(s), f.sqr(p));
        let (s4, p4) = (f.sqr(s2), f.sqr(p2));
        let p3 = f.mul(p2, p);
        let p_s2 = f.mul(p, s2);
        let xr2 = f.sqr(xr);
        let xr3 = f.mul(xr2, xr);
        let xr4 = f.sqr(xr2);
        let p2_xr2 = f.mul(p2, xr2);
        let one = f.one();
        let c0 = f.add(f.add(f.add(xr4, s4), f.mul(p4, xr4)), p2_xr2);
        let c1 = f.add(f.add(f.mul(p3, xr3), f.mul(p_s2, xr)), f.mul(p, xr3));
        let c2 = f.add(
            f.add(f.add(f.mul(f.mul(p2, s2), xr2), f.mul(p2, xr4)), p2),
            f.mul(s2, xr2),
        );
        let c3 = f.add(f.add(f.mul(p3, xr), f.mul(p_s2, xr3)), f.mul(p, xr));
        let c4 = f.add(f.add(f.add(one, p4), f.mul(s4, xr4)), p2_xr2);
        [c0, c1, c2, c3, c4]
    }

    /// The roots in `V` of `S₄(x₁, x₂, ·, x_R)` by evaluation at every point of `V`.
    pub fn roots_by_evaluation<F: FactorField<E = E>>(
        &self,
        f: &F,
        x1: E,
        x2: E,
        xr: E,
        out: &mut Vec<E>,
    ) {
        let q = Self::quartic(f, x1, x2, xr);
        for v in 0..(1u64 << self.l) {
            let x = f.elem(v);
            let mut acc = q[4];
            for k in (0..4).rev() {
                acc = f.add(f.mul(acc, x), q[k]);
            }
            if f.is_zero(acc) {
                out.push(x);
            }
        }
    }

    /// Every root in `V` of `S₄(x₁, x₂, ·, x_R)`, appended to `out` in no
    /// particular order and without repeats. `inv_xr` is `1/x_R` (computed
    /// once per target). Inversions inline; see [`Self::row`] for the
    /// batched form.
    pub fn pair_roots<F: FactorField<E = E>>(
        &self,
        f: &F,
        x1: E,
        x2: E,
        xr: E,
        inv_xr: E,
        out: &mut Vec<E>,
    ) -> PairCase {
        let a1 = f.sqr(f.add(x1, x2));
        let b1 = f.mul(x1, x2);
        if f.is_zero(a1) || f.is_zero(b1) {
            self.roots_by_evaluation(f, x1, x2, xr, out);
            return PairCase::ByEvaluation;
        }
        let c1 = f.add(f.sqr(b1), f.one());
        let ab_inv = f.inv(f.mul(a1, b1));
        let Some(us) = self.as_roots(f, a1, b1, c1, ab_inv) else {
            if xr == x1 || xr == x2 || inv_xr == x1 || inv_xr == x2 {
                self.roots_by_evaluation(f, x1, x2, xr, out);
                return PairCase::ByEvaluation;
            }
            return PairCase::Rejected;
        };
        let start = out.len();
        for u in us {
            let a2 = f.sqr(f.add(u, xr));
            let b2 = f.mul(u, xr);
            if f.is_zero(a2) || f.is_zero(b2) {
                out.truncate(start);
                self.roots_by_evaluation(f, x1, x2, xr, out);
                return PairCase::ByEvaluation;
            }
            let c2 = f.add(f.sqr(b2), f.one());
            let ab2 = f.inv(f.mul(a2, b2));
            if let Some(xs) = self.as_roots(f, a2, b2, c2, ab2) {
                for x in xs {
                    if f.below(x, self.l) && !out[start..].contains(&x) {
                        out.push(x);
                    }
                }
            }
        }
        PairCase::Factored
    }

    /// One row of a decomposition sweep: every pair `(x₁, x₂)` with
    /// `x₂ ∈ x2s`, for the target `x_R`. Calls `found(x₂, x₃)` for every
    /// root `x₃ ∈ V` of `S₄(x₁, x₂, ·, x_R)`. The two inversion stages are
    /// each one batched inversion for the whole row.
    #[allow(clippy::too_many_arguments)]
    pub fn row<F: FactorField<E = E>>(
        &self,
        f: &F,
        x1: E,
        x2s: &[E],
        xr: E,
        inv_xr: E,
        sc: &mut RowScratch<E>,
        mut found: impl FnMut(E, E),
    ) {
        // Stage A: u, u' for every pair, one batched inversion of a₁·b₁.
        sc.inv.clear();
        for &x2 in x2s {
            sc.inv.push(f.mul(f.sqr(f.add(x1, x2)), f.mul(x1, x2)));
        }
        f.batch_inv(&mut sc.inv, &mut sc.tmp);
        sc.us.clear();
        sc.special.clear();
        for (j, &x2) in x2s.iter().enumerate() {
            let a1 = f.sqr(f.add(x1, x2));
            let b1 = f.mul(x1, x2);
            if f.is_zero(a1) || f.is_zero(b1) {
                sc.special.push(x2);
                continue;
            }
            let c1 = f.add(f.sqr(b1), f.one());
            match self.as_roots(f, a1, b1, c1, sc.inv[j]) {
                Some([u0, u1]) => {
                    sc.us.push((x2, u0));
                    sc.us.push((x2, u1));
                }
                None if xr == x1 || xr == x2 || inv_xr == x1 || inv_xr == x2 => sc.special.push(x2),
                None => {}
            }
        }
        // Stage B: X for every rational u, one batched inversion of a₂·b₂.
        sc.inv.clear();
        for &(_, u) in &sc.us {
            sc.inv.push(f.mul(f.sqr(f.add(u, xr)), f.mul(u, xr)));
        }
        f.batch_inv(&mut sc.inv, &mut sc.tmp);
        let mut j = 0;
        while j < sc.us.len() {
            let x2 = sc.us[j].0;
            let pair = &sc.us[j..j + 2];
            let degenerate = pair
                .iter()
                .any(|&(_, u)| f.is_zero(f.add(u, xr)) || f.is_zero(f.mul(u, xr)));
            if degenerate {
                sc.special.push(x2);
            } else {
                sc.roots.clear();
                for (k, &(_, u)) in pair.iter().enumerate() {
                    let a2 = f.sqr(f.add(u, xr));
                    let b2 = f.mul(u, xr);
                    let c2 = f.add(f.sqr(b2), f.one());
                    if let Some(xs) = self.as_roots(f, a2, b2, c2, sc.inv[j + k]) {
                        for x in xs {
                            if f.below(x, self.l) && !sc.roots.contains(&x) {
                                sc.roots.push(x);
                            }
                        }
                    }
                }
                for &x3 in &sc.roots {
                    found(x2, x3);
                }
            }
            j += 2;
        }
        // Degenerate and guarded pairs by evaluation over V.
        for &x2 in &sc.special {
            sc.roots.clear();
            self.roots_by_evaluation(f, x1, x2, xr, &mut sc.roots);
            for &x3 in &sc.roots {
                found(x2, x3);
            }
        }
    }
}

/// Per-row scratch reused across rows.
pub struct RowScratch<E> {
    inv: Vec<E>,
    tmp: Vec<E>,
    us: Vec<(E, E)>,
    special: Vec<E>,
    roots: Vec<E>,
}

impl<E> Default for RowScratch<E> {
    fn default() -> Self {
        Self {
            inv: Vec::new(),
            tmp: Vec::new(),
            us: Vec::new(),
            special: Vec::new(),
            roots: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::f2m::IrreduciblePoly;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    /// Brute force over V of the quartic, sorted.
    fn truth<F: FactorField>(f: &F, l: u32, x1: F::E, x2: F::E, xr: F::E) -> Vec<F::E>
    where
        F::E: Ord + Eq + Copy + std::fmt::Debug,
    {
        let q = FactoredWide::<F::E>::quartic(f, x1, x2, xr);
        let mut r: Vec<F::E> = (0..(1u64 << l))
            .map(|v| f.elem(v))
            .filter(|&x| {
                let mut acc = q[4];
                for k in (0..4).rev() {
                    acc = f.add(f.mul(acc, x), q[k]);
                }
                f.is_zero(acc)
            })
            .collect();
        r.sort();
        r
    }

    /// A target with a planted decomposition through x₃ ∈ V: u = root of
    /// S₃(x₁, x₂, ·), x_R = root of S₃(u, x₃, ·), when both split.
    fn plant<F: FactorField>(
        f: &F,
        fw: &FactoredWide<F::E>,
        x1: F::E,
        x2: F::E,
        x3: F::E,
    ) -> Option<F::E>
    where
        F::E: Ord + Eq + Copy + std::fmt::Debug,
    {
        let [u, _] = fw.s3_roots(f, x1, x2)?;
        let [xr, _] = fw.s3_roots(f, u, x3)?;
        Some(xr)
    }

    fn check_field<F: FactorField>(f: &F, l: u32, seed: u64, trials: usize)
    where
        F::E: Ord + Eq + Copy + std::fmt::Debug,
    {
        let fw = FactoredWide::new(f, l);
        let mut rng = StdRng::seed_from_u64(seed);
        let span = 1u64 << l;
        let n = f.degree();
        let mut planted_hits = 0;
        let mut cases = [0usize; 3];
        for t in 0..trials {
            let x1 = f.elem(rng.gen_range(0..span));
            let x2 = f.elem(rng.gen_range(0..span));
            let xr = if t % 2 == 0 {
                let x3 = f.elem(rng.gen_range(0..span));
                match plant(f, &fw, x1, x2, x3) {
                    Some(xr) => {
                        let q = FactoredWide::<F::E>::quartic(f, x1, x2, xr);
                        let mut acc = q[4];
                        for k in (0..4).rev() {
                            acc = f.add(f.mul(acc, x3), q[k]);
                        }
                        assert!(
                            f.is_zero(acc),
                            "planted x₃ is a root of the quartic (n={n})"
                        );
                        planted_hits += 1;
                        xr
                    }
                    None => f.elem(rng.gen()),
                }
            } else {
                // a random element of the field
                let mut x = f.zero();
                for i in 0..n {
                    if rng.gen::<bool>() {
                        x = f.add(x, f.mono(i));
                    }
                }
                x
            };
            let inv_xr = f.inv(xr);
            let mut got = Vec::new();
            let case = fw.pair_roots(f, x1, x2, xr, inv_xr, &mut got);
            cases[case as usize] += 1;
            got.sort();
            assert_eq!(
                got,
                truth(f, l, x1, x2, xr),
                "pair_roots n={n} l={l} trial={t} case={case:?}"
            );
        }
        assert!(
            planted_hits > trials / 20,
            "planting works (n={n}): {planted_hits}"
        );
        assert!(
            cases[PairCase::Factored as usize] > 0 && cases[PairCase::Rejected as usize] > 0,
            "{cases:?}"
        );
    }

    fn check_rows<F: FactorField>(f: &F, l: u32, seed: u64, targets: usize)
    where
        F::E: Ord + Eq + Copy + std::fmt::Debug,
    {
        let fw = FactoredWide::new(f, l);
        let mut rng = StdRng::seed_from_u64(seed);
        let span = 1u64 << l;
        let v: Vec<F::E> = (0..span).map(|i| f.elem(i)).collect();
        let mut sc = RowScratch::default();
        for _ in 0..targets {
            let (a, b, c) = (
                rng.gen_range(0..span),
                rng.gen_range(0..span),
                rng.gen_range(0..span),
            );
            let xr = plant(f, &fw, v[a as usize], v[b as usize], v[c as usize])
                .unwrap_or_else(|| f.elem(rng.gen()));
            let inv_xr = f.inv(xr);
            for x1 in 0..span {
                let x1e = v[x1 as usize];
                let mut got: Vec<(F::E, F::E)> = Vec::new();
                fw.row(f, x1e, &v[x1 as usize..], xr, inv_xr, &mut sc, |x2, x3| {
                    got.push((x2, x3))
                });
                got.sort();
                let mut want: Vec<(F::E, F::E)> = Vec::new();
                for &x2 in &v[x1 as usize..] {
                    for x3 in truth(f, l, x1e, x2, xr) {
                        want.push((x2, x3));
                    }
                }
                want.sort();
                assert_eq!(got, want, "row n={} x1={x1}", f.degree());
            }
        }
    }

    #[test]
    fn two_word_odd_fields_match_brute_force() {
        for irr in [IrreduciblePoly::deg_113(), IrreduciblePoly::deg_127()] {
            let f = Gf2_128::new(&irr);
            for l in [4u32, 6, 8] {
                check_field(&f, l, 0x113 ^ irr.degree as u64 ^ (l as u64) << 8, 400);
            }
        }
    }

    #[test]
    fn three_word_fields_odd_and_even_match_brute_force() {
        for n in [131u32, 130, 163] {
            let f = Gf2W::new(&gw::low_weight_irreducible(n));
            for l in [4u32, 6, 8] {
                check_field(&f, l, 0x131 ^ n as u64 ^ (l as u64) << 8, 400);
            }
        }
    }

    #[test]
    fn two_word_even_field_matches_brute_force() {
        let f = Gf2_128::new(&gw::low_weight_irreducible(126));
        for l in [4u32, 6] {
            check_field(&f, l, 0x126 ^ (l as u64) << 8, 400);
        }
    }

    #[test]
    fn batched_rows_match_brute_force() {
        check_rows(&Gf2_128::new(&IrreduciblePoly::deg_127()), 4, 1, 6);
        check_rows(&Gf2W::new(&gw::low_weight_irreducible(131)), 4, 2, 6);
        check_rows(&Gf2W::new(&gw::low_weight_irreducible(130)), 4, 3, 6);
    }

    #[test]
    fn artin_schreier_solver_is_exact_for_odd_and_even_n() {
        for n in [113u32, 126, 127] {
            let f = Gf2_128::new(&gw::low_weight_irreducible(n));
            let a = ArtinSchreierAny::new(&f);
            let mut rng = StdRng::seed_from_u64(n as u64);
            let (mut solved, mut refused) = (0, 0);
            for _ in 0..500 {
                let c: u128 = rng.gen::<u128>() & f.mask;
                match a.solve(&f, c) {
                    Some(z) => {
                        assert_eq!(f.sqr(z) ^ z, c, "n={n}");
                        solved += 1;
                    }
                    None => refused += 1,
                }
            }
            assert!(solved > 100 && refused > 100, "n={n}: {solved}/{refused}");
        }
    }
}
