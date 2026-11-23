//! Linearized two-summand root finding for point decomposition on binary curves.
//!
//! Companion to [`super::semaev_decomp`], which decomposes `x_R` over the low-order subspace by looping
//! over pairs `(x_1, x_2)` and root-finding the third abscissa in `V` through the linearized subspace
//! polynomial `L_V` — `O(2^{2l} · l)` field operations for `m = 3`.  This module solves the two-summand
//! problem itself by linear algebra, so `m = 3` fixes one summand and solves for the other two in one
//! `F_2`-linear system plus one quadratic: `O(2^l · l^2)`.
//!
//! On `E: y^2 + x y = x^3 + a x^2 + b`, `P_1 + P_2 = T` means `P_1`, `P_2`, `-T` lie on a line
//! `y = λ(x + x_T) + y_T + x_T`.  Substituting into the curve,
//!
//! ```text
//!   e1 = x1 + x2 = λ^2 + λ + a + xT,     e2 = x1 x2 = xT λ^2 + ((yT + xT)^2 + b) / xT,
//! ```
//!
//! both `F_2`-affine in `λ`.  With `x1, x2 ∈ V` this forces `e1 ∈ V` and `e2 ∈ V2 = span(V·V)`.  Writing
//! `e1 = Σ t_i v_i`, `c1 = a + xT`, `λ = Q(e1 + c1) + ε` with `Q` a fixed `F_2`-linear right inverse of
//! `z ↦ z^2 + z` (so `λ^2 = λ + e1 + c1`, valid when `Tr(e1 + c1) = 0`), and `W = Q + id`, the condition
//! `e2 ∈ V2` plus the trace condition is one `F_2`-linear system in the `l + 1` unknowns `(t, ε)` with
//! `n - dim V2 + 1` equations.  Each solution gives `x^2 + e1 x + e2`, kept when its roots lie in `V`.
//!
//! The arithmetic is [`super::semaev_decomp::Gf2`] (`n ≤ 62`), the same field the pair-loop decomposer
//! uses, so the two are measured on identical instances.  `n = 131` (ECC2K-130) is above both `Gf2`
//! (u64) and `Gf2_128` (u128); it is measured in the standalone three-word crate
//! (`isogeny-labs/lfd-summation-systems`, `rust/ic_rootfind`).
//!
//! The witness is a triple of **points** summing to `R`, stronger than the abscissa triple of
//! [`super::semaev_decomp::decompose`]; the decomposable verdicts agree (tests), because `S_{m+1} = 0`
//! on abscissas in `V` is realized by points summing to `R` or `-R`, and negating gives `R`.

use super::semaev_decomp::Gf2;

/// `F_2`-linear map on `F_{2^n}` stored as the images of the `n` unit vectors; applied by iterating the
/// set bits of the argument.  (Not the byte-table form of the standalone crate; this is the readable,
/// correctness-first version — it is still `O(2^l)` per target against the pair loop's `O(2^{2l})`.)
#[derive(Clone)]
struct LinMap {
    img: Vec<u64>,
}

impl LinMap {
    fn apply(&self, mut x: u64) -> u64 {
        let mut r = 0u64;
        while x != 0 {
            r ^= self.img[x.trailing_zeros() as usize];
            x &= x - 1;
        }
        r
    }
}

/// A point `(x, y)`; `None` is the point at infinity.
pub type Pt = (u64, u64);

/// Reduced xor basis of a set of vectors, as `(pivot_bit, vector)`, highest pivot first in the vector.
fn echelon(vectors: impl IntoIterator<Item = u64>) -> Vec<(u32, u64)> {
    let mut piv: Vec<(u32, u64)> = Vec::new();
    for mut v in vectors {
        for &(pb, pv) in &piv {
            if (v >> pb) & 1 == 1 {
                v ^= pv;
            }
        }
        if v != 0 {
            let pb = 63 - v.leading_zeros();
            for q in piv.iter_mut() {
                if (q.1 >> pb) & 1 == 1 {
                    q.1 ^= v;
                }
            }
            piv.push((pb, v));
        }
    }
    piv
}

/// `rank` of a set of vectors over `F_2`.
fn rank(vectors: impl IntoIterator<Item = u64>) -> usize {
    echelon(vectors).len()
}

/// The linearized two-summand solver on one curve `y^2 + xy = x^3 + a x^2 + b` and subspace `V`.
pub struct LinearDecomposer {
    pub n: u32,
    pub a: u64,
    pub b: u64,
    pub basis: Vec<u64>,
    pub k: usize,
    /// `dim span(V·V)`.
    pub dim_v2: usize,
    /// width of the linear system, `n - dim V2 + 1`.
    pub w: usize,
    sqrtb: u64,
    trmask: u64,
    q: LinMap,
    chk: LinMap,
    pi: LinMap,
    wv: Vec<u64>,
    trv: u64,
}

impl LinearDecomposer {
    /// Build the solver.  `basis` must be `F_2`-independent; `n ≤ 62`; `n - dim V2 ≤ 62`.
    pub fn new(gf: &Gf2, a: u64, b: u64, basis: &[u64]) -> LinearDecomposer {
        let n = gf.n;
        let k = basis.len();
        assert!(n <= 62, "Gf2 solver is for n <= 62");
        assert!(
            k <= 62 && rank(basis.iter().copied()) == k,
            "basis must be independent"
        );
        assert!(b != 0, "curve singular at b = 0");

        let unit = |i: u32| 1u64 << i;
        // trace mask: Tr(x) = parity(x & trmask)
        let mut trmask = 0u64;
        for i in 0..n {
            let (mut t, mut s) = (unit(i), 0u64);
            for _ in 0..n {
                s ^= t;
                t = gf.sqr(t);
            }
            debug_assert!(s == 0 || s == 1);
            if s == 1 {
                trmask |= unit(i);
            }
        }
        let tr = |x: u64| (x & trmask).count_ones() & 1 == 1;

        // Q: a linear right inverse of z -> z^2 + z, i.e. Q(c)^2 + Q(c) = c whenever Tr(c) = 0.
        // Echelonize the images z^2 + z of the unit vectors, tracking preimages.
        let mut piv: Vec<(u32, u64, u64)> = Vec::new(); // (pivot, image, preimage)
        for i in 0..n {
            let (mut im, mut pre) = (gf.sqr(unit(i)) ^ unit(i), unit(i));
            for &(pb, pim, ppre) in &piv {
                if (im >> pb) & 1 == 1 {
                    im ^= pim;
                    pre ^= ppre;
                }
            }
            if im != 0 {
                let pb = 63 - im.leading_zeros();
                for q in piv.iter_mut() {
                    if (q.1 >> pb) & 1 == 1 {
                        q.1 ^= im;
                        q.2 ^= pre;
                    }
                }
                piv.push((pb, im, pre));
            }
        }
        piv.sort_by_key(|x| core::cmp::Reverse(x.0));
        let qimg: Vec<u64> = (0..n)
            .map(|bit| {
                let (mut v, mut pre) = (unit(bit), 0u64);
                for &(pb, pim, ppre) in &piv {
                    if (v >> pb) & 1 == 1 {
                        v ^= pim;
                        pre ^= ppre;
                    }
                }
                let _ = v;
                pre
            })
            .collect();
        let q = LinMap { img: qimg };

        // V2 = span(V . V); pi: surjection F_2^n -> F_2^(n - dim V2) with kernel V2.
        let prods: Vec<u64> = (0..k)
            .flat_map(|i| (i..k).map(move |j| (i, j)))
            .map(|(i, j)| gf.mul(basis[i], basis[j]))
            .collect();
        let v2: Vec<u64> = echelon(prods).into_iter().map(|(_, v)| v).collect();
        let dim_v2 = v2.len();
        let (pi, free) = quotient_map(&v2, n);
        assert!(free < 62, "n - dim V2 must be at most 61");
        let (chk, _) = quotient_map(basis, n);

        let sqrt_img: Vec<u64> = (0..n)
            .map(|i| {
                let mut t = unit(i);
                for _ in 0..n - 1 {
                    t = gf.sqr(t);
                }
                t
            })
            .collect();
        let sqrt = LinMap { img: sqrt_img };
        let sqrtb = sqrt.apply(b);

        let wv: Vec<u64> = basis.iter().map(|&v| q.apply(v) ^ v).collect();
        let mut trv = 0u64;
        for (i, &v) in basis.iter().enumerate() {
            if tr(v) {
                trv |= 1 << i;
            }
        }

        LinearDecomposer {
            n,
            a,
            b,
            basis: basis.to_vec(),
            k,
            dim_v2,
            w: free + 1,
            sqrtb,
            trmask,
            q,
            chk,
            pi,
            wv,
            trv,
        }
    }

    /// Low-order subspace `⟨1, z, …, z^{l−1}⟩`, the factor base of [`super::semaev_decomp::decompose`].
    pub fn low_order(gf: &Gf2, a: u64, b: u64, l: u32) -> LinearDecomposer {
        let basis: Vec<u64> = (0..l).map(|i| 1u64 << i).collect();
        LinearDecomposer::new(gf, a, b, &basis)
    }

    #[inline]
    fn tr(&self, x: u64) -> bool {
        (x & self.trmask).count_ones() & 1 == 1
    }
    #[inline]
    pub fn in_v(&self, x: u64) -> bool {
        self.chk.apply(x) == 0
    }

    /// Group-law negation.
    pub fn neg(&self, p: Pt) -> Pt {
        (p.0, p.0 ^ p.1)
    }

    /// Full binary group law (`None` = infinity), used for lifting and the completeness checks.
    pub fn add(&self, gf: &Gf2, p: Option<Pt>, q: Option<Pt>) -> Option<Pt> {
        let (p, q) = match (p, q) {
            (None, q) => return q,
            (p, None) => return p,
            (Some(p), Some(q)) => (p, q),
        };
        if p.0 == q.0 {
            if q.1 == (p.0 ^ p.1) {
                return None;
            }
            let lam = p.0 ^ gf.mul(p.1, gf.inv(p.0));
            let x3 = gf.sqr(lam) ^ lam ^ self.a;
            return Some((x3, gf.sqr(p.0) ^ gf.mul(lam ^ 1, x3)));
        }
        let lam = gf.mul(p.1 ^ q.1, gf.inv(p.0 ^ q.0));
        let x3 = gf.sqr(lam) ^ lam ^ p.0 ^ q.0 ^ self.a;
        Some((x3, gf.mul(lam, p.0 ^ x3) ^ x3 ^ p.1))
    }

    /// A point with abscissa `x`, if one exists (the lower of the two `y`).
    pub fn lift(&self, gf: &Gf2, x: u64) -> Option<Pt> {
        if x == 0 {
            return Some((0, self.sqrtb));
        }
        // y = x z, z^2 + z = x + a + b/x^2
        let c = x ^ self.a ^ gf.mul(self.b, gf.sqr(gf.inv(x)));
        if self.tr(c) {
            return None;
        }
        Some((x, gf.mul(x, self.q.apply(c))))
    }

    pub fn on_curve(&self, gf: &Gf2, p: Pt) -> bool {
        let x2 = gf.sqr(p.0);
        gf.sqr(p.1) ^ gf.mul(p.0, p.1) == gf.mul(x2, p.0) ^ gf.mul(self.a, x2) ^ self.b
    }

    /// Two-summand decompositions `P1 + P2 = T` with `x(P1), x(P2) ∈ V`, as sorted point pairs.
    /// `T` must be a finite point with `x(T) ≠ 0` (the one abscissa the line parametrization misses).
    pub fn decompose2(&self, gf: &Gf2, t: Pt) -> Vec<[Pt; 2]> {
        let mut out = Vec::new();
        self.decompose2_into(gf, t, &mut |p1, p2| {
            let mut d = [p1, p2];
            d.sort();
            out.push(d);
        });
        out.sort();
        out.dedup();
        out
    }

    /// The core two-summand solver; calls `emit(P1, P2)` for every ordered solution (duplicates possible).
    fn decompose2_into(&self, gf: &Gf2, t: Pt, emit: &mut dyn FnMut(Pt, Pt)) {
        debug_assert!(t.0 != 0);
        let k = self.k;
        let ixt = gf.inv(t.0);
        let ym = t.1 ^ t.0;
        let c1 = self.a ^ t.0;
        let c2 = gf.mul(gf.sqr(ym) ^ self.b, ixt);
        let trbit = 1u64 << (self.w - 1);
        let mut rhs = self.pi.apply(gf.mul(t.0, self.q.apply(c1) ^ c1) ^ c2);
        if self.tr(c1) {
            rhs |= trbit;
        }
        // columns of the system, in the l+1 unknowns (t_0..t_{k-1}, eps)
        let mut bvec = [0u64; 64];
        let mut bcomb = [0u64; 64];
        let mut piv = [0u32; 64];
        let mut kern = [0u64; 64];
        let (mut nb, mut nk) = (0usize, 0usize);
        for i in 0..=k {
            let mut col = if i < k {
                let mut c = self.pi.apply(gf.mul(t.0, self.wv[i]));
                if (self.trv >> i) & 1 == 1 {
                    c |= trbit;
                }
                c
            } else {
                self.pi.apply(t.0)
            };
            let mut comb = 1u64 << i;
            for j in 0..nb {
                if (col >> piv[j]) & 1 == 1 {
                    col ^= bvec[j];
                    comb ^= bcomb[j];
                }
            }
            if col != 0 {
                piv[nb] = 63 - col.leading_zeros();
                bvec[nb] = col;
                bcomb[nb] = comb;
                nb += 1;
            } else {
                kern[nk] = comb;
                nk += 1;
            }
        }
        let mut part = 0u64;
        for j in 0..nb {
            if (rhs >> piv[j]) & 1 == 1 {
                rhs ^= bvec[j];
                part ^= bcomb[j];
            }
        }
        if rhs != 0 {
            return; // inconsistent: no two-summand decomposition of this T
        }
        let mut sol = part;
        for g in 0u64..(1u64 << nk) {
            if g != 0 {
                sol ^= kern[g.trailing_zeros() as usize];
            }
            let mut e1 = 0u64;
            let mut tm = sol & ((1u64 << k) - 1);
            while tm != 0 {
                e1 ^= self.basis[tm.trailing_zeros() as usize];
                tm &= tm - 1;
            }
            let mut lam = self.q.apply(e1 ^ c1);
            if (sol >> k) & 1 == 1 {
                lam ^= 1;
            }
            let e2 = gf.mul(t.0, lam ^ e1 ^ c1) ^ c2; // lam^2 = lam + e1 + c1
            let (x1, x2);
            if e1 == 0 {
                // x^2 = e2
                let r = {
                    let mut t = e2;
                    for _ in 0..self.n - 1 {
                        t = gf.sqr(t);
                    }
                    t
                };
                x1 = r;
                x2 = r;
            } else {
                // x^2 + e1 x + e2: u = x/e1, u^2 + u = e2/e1^2
                let d = gf.mul(e2, gf.sqr(gf.inv(e1)));
                if self.tr(d) {
                    continue;
                }
                x1 = gf.mul(e1, self.q.apply(d));
                x2 = x1 ^ e1;
            }
            if !self.in_v(x1) {
                continue;
            }
            let mu = gf.mul(lam, t.0) ^ ym;
            let p1 = (x1, gf.mul(lam, x1) ^ mu);
            let p2 = (x2, gf.mul(lam, x2) ^ mu);
            emit(p1, p2);
        }
    }

    /// Three-summand decomposition `R = P1 + P2 + P3`, `x(P_i) ∈ V`, found by fixing `P3` over every
    /// abscissa in `V` (both signs) and solving the two-summand problem for `T = R - P3`.  Returns the
    /// first witness, and the number of fixed summands scanned (its cost in the oracle's native unit).
    pub fn decompose3(&self, gf: &Gf2, r: Pt) -> (Option<[Pt; 3]>, u64) {
        let span = 1u64 << self.k;
        let mut scanned = 0u64;
        let mut found: Option<[Pt; 3]> = None;
        // Gray-code walk over V
        let mut x = 0u64;
        for g in 0..span {
            if g != 0 {
                x ^= self.basis
                    [(g ^ (g >> 1) ^ ((g - 1) ^ ((g - 1) >> 1))).trailing_zeros() as usize];
            }
            let Some(p0) = self.lift(gf, x) else { continue };
            let signs = if x == 0 {
                vec![p0]
            } else {
                vec![p0, self.neg(p0)]
            };
            for p3 in signs {
                scanned += 1;
                let Some(t) = self.add(gf, Some(r), Some(self.neg(p3))) else {
                    continue;
                };
                if t.0 == 0 {
                    // the one abscissa the linear step misses: enumerate this rare case over V
                    for gg in 0..span {
                        let mut acc = 0u64;
                        for b in 0..self.k {
                            if (gg >> b) & 1 == 1 {
                                acc ^= self.basis[b];
                            }
                        }
                        if let Some(pp) = self.lift(gf, acc) {
                            for q in [pp, self.neg(pp)] {
                                if let Some(s) = self.add(gf, Some(t), Some(self.neg(q))) {
                                    if self.in_v(s.0) {
                                        found = Some(sort3([p3, q, s]));
                                        return (found, scanned);
                                    }
                                }
                            }
                        }
                    }
                    continue;
                }
                let mut hit: Option<[Pt; 2]> = None;
                self.decompose2_into(gf, t, &mut |p1, p2| {
                    if hit.is_none() {
                        hit = Some([p1, p2]);
                    }
                });
                if let Some([p1, p2]) = hit {
                    found = Some(sort3([p3, p1, p2]));
                    return (found, scanned);
                }
            }
        }
        (found, scanned)
    }

    /// Independent rational brute force: is there a point triple with abscissas in V summing to R?
    /// Triple enumeration by the group law only (no linear solve), for the correctness ground truth.
    pub fn decomposes3_brute(&self, gf: &Gf2, r: Pt) -> bool {
        let span = 1u64 << self.k;
        // signed factor-base points
        let mut fb: Vec<Pt> = Vec::new();
        for g in 0..span {
            let mut acc = 0u64;
            for b in 0..self.k {
                if (g >> b) & 1 == 1 {
                    acc ^= self.basis[b];
                }
            }
            if let Some(p) = self.lift(gf, acc) {
                fb.push(p);
                if acc != 0 {
                    fb.push(self.neg(p));
                }
            }
        }
        for i in 0..fb.len() {
            let s1 = Some(fb[i]);
            for j in i..fb.len() {
                let s2 = self.add(gf, s1, Some(fb[j]));
                for k in j..fb.len() {
                    if self.add(gf, s2, Some(fb[k])) == Some(r) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Every three-summand decomposition (for the completeness cross-check); scans the whole range.
    pub fn decompose3_all(&self, gf: &Gf2, r: Pt) -> Vec<[Pt; 3]> {
        let span = 1u64 << self.k;
        let mut out = Vec::new();
        for g in 0..span {
            let mut acc = 0u64;
            for b in 0..self.k {
                if (g >> b) & 1 == 1 {
                    acc ^= self.basis[b];
                }
            }
            let Some(p0) = self.lift(gf, acc) else {
                continue;
            };
            let signs = if acc == 0 {
                vec![p0]
            } else {
                vec![p0, self.neg(p0)]
            };
            for p3 in signs {
                let Some(t) = self.add(gf, Some(r), Some(self.neg(p3))) else {
                    continue;
                };
                if t.0 == 0 {
                    continue;
                }
                self.decompose2_into(gf, t, &mut |p1, p2| {
                    out.push(sort3([p3, p1, p2]));
                });
            }
        }
        out.sort();
        out.dedup();
        out
    }
}

fn sort3(mut d: [Pt; 3]) -> [Pt; 3] {
    d.sort();
    d
}

/// Images of the `n` unit vectors under a surjection `F_2^n -> F_2^(n - dim)` with kernel `span(basis)`,
/// and the codimension.
fn quotient_map(basis: &[u64], n: u32) -> (LinMap, usize) {
    let piv = echelon(basis.iter().copied());
    let mut pos = vec![usize::MAX; n as usize];
    let mut nfree = 0usize;
    for b in 0..n as usize {
        if !piv.iter().any(|&(pb, _)| pb as usize == b) {
            pos[b] = nfree;
            nfree += 1;
        }
    }
    assert!(nfree <= 62);
    let mut img = vec![0u64; n as usize];
    for b in 0..n as usize {
        if pos[b] != usize::MAX {
            img[b] = 1u64 << pos[b];
        } else {
            let pv = piv.iter().find(|&&(pb, _)| pb as usize == b).unwrap().1;
            let mut acc = 0u64;
            for c in 0..n as usize {
                if c != b && (pv >> c) & 1 == 1 {
                    acc ^= 1u64 << pos[c];
                }
            }
            img[b] = acc;
        }
    }
    (LinMap { img }, nfree)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_ecc::IrreduciblePoly;
    use crate::cryptanalysis::semaev_decomp::{eval_s4_general, SubspaceOracle};

    fn sparse_irr(n: u32) -> IrreduciblePoly {
        // lowest-weight trinomial x^n + x^k + 1, else a pentanomial; irreducibility by Rabin on bit vectors
        let irreducible = |low: &[u32]| -> bool {
            // represent polynomials as u128 bit vectors (n <= 62 here, products fit)
            let modp: u128 =
                (1u128 << n) | low.iter().fold(0u128, |acc, &e| acc | (1u128 << e)) | 1;
            let degree = |p: u128| -> i32 { 127 - p.leading_zeros() as i32 };
            let rem = |mut p: u128, m: u128| -> u128 {
                let dm = degree(m);
                while degree(p) >= dm && p != 0 {
                    p ^= m << (degree(p) - dm);
                }
                p
            };
            let mulmod = |a: u128, b: u128, m: u128| -> u128 {
                let mut r = 0u128;
                let mut a = a;
                let mut b = b;
                while b != 0 {
                    if b & 1 == 1 {
                        r ^= a;
                    }
                    a <<= 1;
                    b >>= 1;
                    if degree(a) >= degree(m) {
                        a ^= m << (degree(a) - degree(m)).max(0);
                        a = rem(a, m);
                    }
                }
                rem(r, m)
            };
            // x^(2^n) == x, and gcd(x^(2^(n/q)) - x, f) == 1 for primes q | n
            let mut xp = 2u128; // x
            for _ in 0..n {
                xp = mulmod(xp, xp, modp);
            }
            if xp != 2 {
                return false;
            }
            let mut m = n;
            let mut q = 2;
            let mut primes = vec![];
            while q * q <= m {
                if m.is_multiple_of(q) {
                    primes.push(q);
                    while m.is_multiple_of(q) {
                        m /= q;
                    }
                }
                q += 1;
            }
            if m > 1 {
                primes.push(m);
            }
            let gcd = |mut a: u128, mut b: u128| -> u128 {
                while b != 0 {
                    a = rem(a, b);
                    std::mem::swap(&mut a, &mut b);
                }
                a
            };
            for qp in primes {
                let mut xpp = 2u128;
                for _ in 0..n / qp {
                    xpp = mulmod(xpp, xpp, modp);
                }
                if gcd(xpp ^ 2, modp) != 1 {
                    return false;
                }
            }
            true
        };
        for k in 1..n {
            if irreducible(&[k]) {
                return IrreduciblePoly {
                    degree: n,
                    low_terms: vec![0, k],
                };
            }
        }
        for k3 in 3..n {
            for k2 in 2..k3 {
                for k1 in 1..k2 {
                    if irreducible(&[k1, k2, k3]) {
                        return IrreduciblePoly {
                            degree: n,
                            low_terms: vec![0, k1, k2, k3],
                        };
                    }
                }
            }
        }
        panic!("no sparse irreducible for n = {n}")
    }

    fn rng(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    #[test]
    fn field_maps_are_consistent() {
        for n in [7u32, 8, 11, 13, 16, 17, 23] {
            let gf = Gf2::new(&sparse_irr(n));
            let d = LinearDecomposer::low_order(&gf, 1, 1, (n / 3).max(2));
            // Q is a right inverse of z -> z^2 + z on trace-zero inputs
            let mut st = 0x1234 + n as u64;
            for _ in 0..200 {
                let c = rng(&mut st) & ((1u64 << n) - 1);
                if (c & d.trmask).count_ones() & 1 == 0 {
                    let qc = d.q.apply(c);
                    assert_eq!(gf.sqr(qc) ^ qc, c, "Q n={n}");
                }
            }
        }
    }

    #[test]
    fn lift_lands_on_curve() {
        for n in [7u32, 11, 13, 17, 23] {
            let gf = Gf2::new(&sparse_irr(n));
            for a in [0u64, 1] {
                let d = LinearDecomposer::low_order(&gf, a, 1, (n / 3).max(2));
                let mut hits = 0;
                for x in 0..(1u64 << n).min(4096) {
                    if let Some(p) = d.lift(&gf, x) {
                        assert_eq!(p.0, x);
                        assert!(d.on_curve(&gf, p), "n={n} a={a} x={x}");
                        hits += 1;
                    }
                }
                assert!(hits > 0);
            }
        }
    }

    #[test]
    fn verdict_agrees_with_pair_loop_and_witnesses_are_valid() {
        // LRF (point decomposition) vs semaev_decomp::SubspaceOracle (abscissa pair loop), same field,
        // same low-order subspace, Koblitz b = 1.  Verdicts must agree; every witness must be valid.
        let mut total = 0;
        let mut decomposable = 0;
        for n in [10u32, 13, 16, 19, 22] {
            let gf = Gf2::new(&sparse_irr(n));
            let l = (n / 3).max(2);
            let b = 1u64;
            for a in [0u64, 1] {
                let d = LinearDecomposer::low_order(&gf, a, b, l);
                let oracle = SubspaceOracle::low_order(l, b, &gf);
                let mut st = 0xC0FFEE ^ ((n as u64) << 8) ^ (a << 1);
                for _ in 0..60 {
                    // a random target point R
                    let r = loop {
                        let x = rng(&mut st) & ((1u64 << n) - 1);
                        if let Some(p) = d.lift(&gf, x) {
                            break if rng(&mut st) & 1 == 1 { d.neg(p) } else { p };
                        }
                    };
                    let (lrf, _scanned) = d.decompose3(&gf, r);
                    // LRF witness: points on the curve, abscissas in V, summing to R, S4 = 0
                    if let Some(tri) = lrf {
                        let mut acc = None;
                        for p in tri {
                            assert!(d.on_curve(&gf, p));
                            assert!(d.in_v(p.0));
                            acc = d.add(&gf, acc, Some(p));
                        }
                        assert_eq!(acc, Some(r), "LRF witness sums to R, n={n} a={a}");
                        assert_eq!(
                            eval_s4_general(tri[0].0, tri[1].0, tri[2].0, r.0, b, &gf),
                            0,
                            "LRF witness has S4 = 0"
                        );
                        decomposable += 1;
                    }
                    // verdict agreement with the abscissa pair loop
                    let (oracle_w, _pairs) = oracle.decompose(r.0, &gf);
                    assert_eq!(
                        lrf.is_some(),
                        oracle_w.is_some(),
                        "verdict disagreement n={n} a={a} xR={}",
                        r.0
                    );
                    if let Some(w) = oracle_w {
                        assert_eq!(eval_s4_general(w[0], w[1], w[2], r.0, b, &gf), 0);
                    }
                    total += 1;
                }
            }
        }
        assert!(
            total > 100 && decomposable > 10,
            "total={total} decomposable={decomposable}"
        );
    }

    #[test]
    fn lrf_matches_independent_rational_brute() {
        // LRF's rational verdict equals an independent group-law triple enumeration; and the pair-loop
        // abscissa oracle is a *superset* (it also accepts twist-only abscissa solutions), which is the
        // one legitimate disagreement.  l small so the O(2^{3l}) brute is affordable.
        let mut twist_only = 0;
        let mut checked = 0;
        for n in [9u32, 12, 15, 18] {
            let gf = Gf2::new(&sparse_irr(n));
            let l = (n / 3).max(2);
            let b = 1u64;
            for a in [0u64, 1] {
                let d = LinearDecomposer::low_order(&gf, a, b, l);
                let oracle = SubspaceOracle::low_order(l, b, &gf);
                let mut st = 0xBEEF ^ ((n as u64) << 7) ^ (a << 3);
                for _ in 0..80 {
                    let r = loop {
                        let x = rng(&mut st) & ((1u64 << n) - 1);
                        if let Some(p) = d.lift(&gf, x) {
                            break if rng(&mut st) & 1 == 1 { d.neg(p) } else { p };
                        }
                    };
                    let lrf = d.decompose3(&gf, r).0.is_some();
                    let brute = d.decomposes3_brute(&gf, r);
                    assert_eq!(
                        lrf, brute,
                        "LRF vs independent brute, n={n} a={a} xR={}",
                        r.0
                    );
                    let absc = oracle.decompose(r.0, &gf).0.is_some();
                    // abscissa oracle accepts everything rational, plus twist-only solutions
                    assert!(
                        absc || !brute,
                        "abscissa oracle must accept every rational decomposition"
                    );
                    if absc && !brute {
                        twist_only += 1;
                    }
                    checked += 1;
                }
            }
        }
        assert!(checked > 200);
        // twist-only hits exist (that is the point), but are a minority
        assert!(
            twist_only > 0,
            "expected some twist-only abscissa solutions to exercise the distinction"
        );
    }
}
