//! # Closed-form roots of the binary Semaev `S₄` quartic.
//!
//! [`super::semaev_decomp`] decides an `m = 3` target by looping over pairs
//! `(X₁, X₂)` of the factor base `V = {x : x < 2^l}` and finding the roots
//! in `V` of a quartic in `X₃`.  It does that as `gcd(f, L_V mod f)`, with
//! `L_V` the subspace polynomial: about `l` Frobenius squarings modulo `f`
//! and a straight-line gcd.  This module asks whether the roots can be had
//! in closed form more cheaply, as the radical root finder of
//! [`super::prime_radical_root`] did over `F_p`.
//!
//! ## The closed form in characteristic 2
//!
//! A quartic over `F_{2^n}` is solvable by radicals, despite
//! `petit_quisquater.rs` saying otherwise ("no closed-form root via
//! Artin–Schreier"), and without leaving `F_{2^n}`:
//!
//! 1. **Affine form.**  For monic `f = x⁴ + f₃x³ + f₂x² + f₁x + f₀` with
//!    `f₃ ≠ 0`, put `x = y + s`, `s = √(f₁/f₃)`, which kills the linear
//!    term: `y⁴ + f₃y³ + b₂y² + b₀`, `b₂ = f₂ + f₃s`, `b₀ = f(s)`.  Then
//!    `y = 1/z` gives `A(z) = z⁴ + αz² + βz + γ` with `α = b₂/b₀`,
//!    `β = f₃/b₀`, `γ = 1/b₀`: an **affine** polynomial, `L(z) + γ` with
//!    `L(z) = z⁴ + αz² + βz` linear over `F₂`.
//! 2. **Kernel.**  `L(z) = z(z³ + αz + β)`, so `ker L` is `{0}` with the
//!    roots of the cubic: a subspace of dimension 0, 1 or 2, so the cubic
//!    has 0, 1 or 3 roots and `A` has 1, {0, 2} or {0, 4}.
//! 3. **The cubic is a Dickson equation.**  `z = √α·t` turns it into
//!    `t³ + t = a`, `a = β/α^{3/2}`, which is `D₃(t) = a` for the Dickson
//!    polynomial `D_k(w + 1/w) = w^k + w^{−k}`.  For odd `n` and
//!    `Tr(1/a) = 0`, the `w` lies in `F_{2^n}^*`, where cubing is a
//!    bijection, so `t = D_e(a)` with `3e ≡ 1 (mod 2ⁿ − 1)`; for
//!    `Tr(1/a) = 1` it lies in the order-`2ⁿ + 1` subgroup of
//!    `F_{2^{2n}}`, and `t = D_e(a)` with `3e ≡ 1 (mod (2ⁿ + 1)/3)` when a
//!    cube root exists there (checked by `t³ + t = a`).  `D_e` is a
//!    Lucas ladder in `F_{2^n}`: one product and one square a bit of `e`.
//! 4. **Two Artin–Schreier steps.**  With a kernel element `d`,
//!    `L = (z² + cz)∘(z² + dz)` for `c = β/d` (`cd = β`, `d² + c = α`), so
//!    `A(z) = 0` iff `v = z² + dz` solves `v² + cv = γ`: two quadratics,
//!    each a half-trace (`n` odd) — an `F₂`-linear map, applied by table.
//! 5. `x = s + 1/z`, and `x ∈ V` iff `x < 2^l`.
//!
//! When the cubic has no root (`A` is then bijective, a third of all
//! quartics), `L` does not split over `F_{2^n}`; those quartics, and the
//! degenerate ones (`f₃ = 0`, `b₀ = 0`, `α = 0`, `9 | 2ⁿ + 1`), go to the
//! existing gcd.  The fallback share bounds the possible gain.
//!
//! [`RadicalQuartic::row`] runs the closed form over a row of quartics
//! with each of its four inversion stages batched (Montgomery's trick), as
//! `decompose` batches its leading coefficients; [`RadicalQuartic::solve`]
//! is the scalar reference.

use super::semaev_decomp::{Gf2, Poly, QuarticSubspaceRoots};

/// An `F₂`-linear map on single-word field elements, applied a byte at a
/// time through precomputed tables.
pub struct LinMap {
    tab: Vec<[u64; 256]>,
}

impl LinMap {
    /// `images[i]` is the image of the basis element `zⁱ`.
    pub fn from_images(images: &[u64]) -> Self {
        let bytes = images.len().div_ceil(8);
        let mut tab = vec![[0u64; 256]; bytes];
        for (j, t) in tab.iter_mut().enumerate() {
            for v in 1..256usize {
                let mut acc = 0u64;
                for b in 0..8 {
                    let i = 8 * j + b;
                    if v >> b & 1 == 1 && i < images.len() {
                        acc ^= images[i];
                    }
                }
                t[v] = acc;
            }
        }
        Self { tab }
    }

    #[inline(always)]
    pub fn apply(&self, x: u64) -> u64 {
        let mut r = 0u64;
        for (j, t) in self.tab.iter().enumerate() {
            r ^= t[((x >> (8 * j)) & 0xff) as usize];
        }
        r
    }
}

/// `x⁻¹ mod m` for `gcd(x, m) = 1`.
fn inv_mod(x: u128, m: u128) -> Option<u128> {
    let (mut r0, mut r1) = (m as i128, (x % m) as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1 != 0 {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    (r0 == 1).then(|| t0.rem_euclid(m as i128) as u128)
}

/// Why a quartic went to the gcd instead of the closed form.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Fallback {
    /// `f₃ = 0` (every pair with `X₁X₂ = 0`): no shift kills the linear term.
    F3Zero,
    /// `f(s) = 0`: the shift point is itself a root.
    ShiftRoot,
    /// `α = 0`: the cubic degenerates to `z³ = β`.
    AlphaZero,
    /// The resolvent cubic has no root in `F_{2^n}`; `A` is bijective.
    Bijective,
    /// `9 | 2ⁿ + 1`, where the order-`(2ⁿ+1)/3` cube root is not an
    /// exponent.
    NoExponent,
}

/// Outcome of the closed form on one quartic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Solved {
    /// Every root in `V`, ascending.
    Roots(Vec<u64>),
    Fallback(Fallback),
}

/// Precomputation for one field and one subspace dimension `l`.
pub struct RadicalQuartic {
    pub n: u32,
    pub l: u32,
    sqrt: LinMap,
    half_trace: LinMap,
    trace_mask: u64,
    /// `3⁻¹ mod (2ⁿ − 1)`.
    e_minus: u64,
    /// `3⁻¹ mod (2ⁿ + 1)/3`, when it exists.
    e_plus: Option<u64>,
}

/// Per-row scratch, reused across rows so the pipeline does not allocate.
#[derive(Default)]
pub struct RowScratch {
    inv: Vec<u64>,
    tmp: Vec<u64>,
    idx: Vec<usize>,
    st: Vec<[u64; 6]>,
    zs: Vec<u64>,
    zowner: Vec<usize>,
}

/// Tallies of what [`RadicalQuartic::row`] did, by case.
#[derive(Clone, Copy, Debug, Default)]
pub struct RowCounts {
    pub quartics: u64,
    pub closed_form: u64,
    pub f3_zero: u64,
    pub shift_root: u64,
    pub alpha_zero: u64,
    pub bijective: u64,
    pub no_exponent: u64,
    pub roots_in_v: u64,
}

impl RadicalQuartic {
    /// Needs odd `n` (the half-trace solves Artin–Schreier there).
    pub fn new(l: u32, gf: &Gf2) -> Option<Self> {
        let n = gf.n;
        if n % 2 == 0 || n > 63 || l > n {
            return None;
        }
        let basis: Vec<u64> = (0..n).map(|i| 1u64 << i).collect();
        let sqrt: Vec<u64> = basis.iter().map(|&b| gf.sqr_k(b, n - 1)).collect();
        let tr = |x: u64| {
            let (mut acc, mut p) = (0u64, x);
            for _ in 0..n {
                acc ^= p;
                p = gf.sqr(p);
            }
            acc
        };
        let mut trace_mask = 0u64;
        for (i, &b) in basis.iter().enumerate() {
            let t = tr(b);
            debug_assert!(t <= 1);
            trace_mask |= (t & 1) << i;
        }
        // HT(c) = Σ_{i=0}^{(n−1)/2} c^{4^i}.
        let ht: Vec<u64> = basis
            .iter()
            .map(|&b| {
                let (mut acc, mut p) = (0u64, b);
                for _ in 0..=(n - 1) / 2 {
                    acc ^= p;
                    p = gf.sqr_k(p, 2);
                }
                acc
            })
            .collect();
        let qm = (1u128 << n) - 1;
        let e_minus = inv_mod(3, qm)? as u64;
        let np = ((1u128 << n) + 1) / 3;
        let e_plus = inv_mod(3, np).map(|e| e as u64);
        Some(Self {
            n,
            l,
            sqrt: LinMap::from_images(&sqrt),
            half_trace: LinMap::from_images(&ht),
            trace_mask,
            e_minus,
            e_plus,
        })
    }

    #[inline(always)]
    pub fn trace(&self, x: u64) -> u64 {
        ((x & self.trace_mask).count_ones() & 1) as u64
    }

    #[inline(always)]
    pub fn sqrt(&self, x: u64) -> u64 {
        self.sqrt.apply(x)
    }

    #[inline(always)]
    pub fn half_trace(&self, x: u64) -> u64 {
        self.half_trace.apply(x)
    }

    /// `D_e(a)`, the Dickson polynomial of index `e` (`D₀ = 0`, `D₁ = a`).
    #[inline(always)]
    pub fn dickson(&self, a: u64, e: u64, gf: &Gf2) -> u64 {
        let (mut lo, mut hi) = (0u64, a);
        let top = 63 - e.leading_zeros() as i32;
        let mut i = top;
        while i >= 0 {
            let mid = gf.mul(lo, hi) ^ a;
            if e >> i & 1 == 1 {
                lo = mid;
                hi = gf.sqr(hi);
            } else {
                hi = mid;
                lo = gf.sqr(lo);
            }
            i -= 1;
        }
        lo
    }

    /// A root of `t³ + t = a` (`a ≠ 0`), or why there is none to use.
    #[inline(always)]
    fn cubic_root(&self, a: u64, inv_a: u64, gf: &Gf2) -> Result<u64, Fallback> {
        let e = if self.trace(inv_a) == 0 {
            self.e_minus
        } else {
            self.e_plus.ok_or(Fallback::NoExponent)?
        };
        let t = self.dickson(a, e, gf);
        if gf.mul(gf.sqr(t), t) ^ t == a {
            Ok(t)
        } else {
            Err(Fallback::Bijective)
        }
    }

    /// The closed form on one monic quartic, inversions inline.  The
    /// reference for [`Self::row`]; slow, since each inversion is a full
    /// Itoh–Tsujii chain.
    pub fn solve(&self, f: [u64; 4], gf: &Gf2) -> Solved {
        let [f0, f1, f2, f3] = f;
        if f3 == 0 {
            return Solved::Fallback(Fallback::F3Zero);
        }
        let s = self.sqrt(gf.mul(f1, gf.inv(f3)));
        let (s2, b2) = (gf.sqr(s), gf.mul(f3, s) ^ f2);
        let b0 = gf.sqr(s2) ^ gf.mul(f3, gf.mul(s2, s)) ^ gf.mul(f2, s2) ^ gf.mul(f1, s) ^ f0;
        if b0 == 0 {
            return Solved::Fallback(Fallback::ShiftRoot);
        }
        let g = gf.inv(b0);
        let (alpha, beta, gamma) = (gf.mul(b2, g), gf.mul(f3, g), g);
        if alpha == 0 {
            return Solved::Fallback(Fallback::AlphaZero);
        }
        let ra = self.sqrt(alpha);
        let a32 = gf.mul(alpha, ra);
        let p = gf.inv(gf.mul(beta, a32));
        let (inv_a32, inv_beta) = (gf.mul(beta, p), gf.mul(a32, p));
        let (a, inv_a) = (gf.mul(beta, inv_a32), gf.mul(a32, inv_beta));
        let t = match self.cubic_root(a, inv_a, gf) {
            Ok(t) => t,
            Err(why) => return Solved::Fallback(why),
        };
        let d = gf.mul(ra, t);
        let id = gf.inv(d);
        let mut roots = Vec::new();
        for z in self
            .affine_roots(d, id, beta, inv_beta, gamma, gf)
            .into_iter()
            .flatten()
        {
            let x = s ^ gf.inv(z);
            if x >> self.l == 0 {
                roots.push(x);
            }
        }
        roots.sort_unstable();
        Solved::Roots(roots)
    }

    /// The roots of `z⁴ + αz² + βz + γ` from a kernel element `d`.
    #[inline(always)]
    fn affine_roots(
        &self,
        d: u64,
        id: u64,
        beta: u64,
        inv_beta: u64,
        gamma: u64,
        gf: &Gf2,
    ) -> [Option<u64>; 4] {
        let mut out = [None; 4];
        let c = gf.mul(beta, id);
        // v² + cv = γ  ⇔  (v/c)² + v/c = γ/c², and γ/c² = γ·(d/β)².
        let w = gf.mul(gamma, gf.sqr(gf.mul(d, inv_beta)));
        if self.trace(w) == 1 {
            return out;
        }
        let v0 = gf.mul(c, self.half_trace(w));
        let id2 = gf.sqr(id);
        for (k, v) in [v0, v0 ^ c].into_iter().enumerate() {
            // z² + dz = v  ⇔  (z/d)² + z/d = v/d².
            let q = gf.mul(v, id2);
            if self.trace(q) == 0 {
                let z = gf.mul(d, self.half_trace(q));
                out[2 * k] = Some(z);
                out[2 * k + 1] = Some(z ^ d);
            }
        }
        out
    }

    /// Subspace roots of every quartic in `fs` (monic, four low
    /// coefficients), written to `out[i]` as a gcd-shaped [`Poly`]-free
    /// root list; quartics the closed form cannot take are solved by
    /// `fallback` and their gcd's roots found by evaluation over `V`
    /// only when the gcd has degree ≥ 2 (as `decompose` does).
    ///
    /// Each of the four inversion stages is one batched inversion for the
    /// whole row.  Dispatched to a `pclmulqdq` build as `decompose` is.
    pub fn row(
        &self,
        fs: &[[u64; 4]],
        fallback: &QuarticSubspaceRoots,
        gf: &Gf2,
        sc: &mut RowScratch,
        out: &mut Vec<RowRoots>,
        counts: &mut RowCounts,
    ) {
        #[cfg(target_arch = "x86_64")]
        if gf.folds() {
            // SAFETY: a field folds only when `pclmulqdq` was detected.
            return unsafe { self.row_clmul(fs, fallback, gf, sc, out, counts) };
        }
        self.row_body(fs, fallback, gf, sc, out, counts)
    }

    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "pclmulqdq")]
    unsafe fn row_clmul(
        &self,
        fs: &[[u64; 4]],
        fallback: &QuarticSubspaceRoots,
        gf: &Gf2,
        sc: &mut RowScratch,
        out: &mut Vec<RowRoots>,
        counts: &mut RowCounts,
    ) {
        self.row_body(fs, fallback, gf, sc, out, counts)
    }

    #[inline(always)]
    fn row_body(
        &self,
        fs: &[[u64; 4]],
        fallback: &QuarticSubspaceRoots,
        gf: &Gf2,
        sc: &mut RowScratch,
        out: &mut Vec<RowRoots>,
        counts: &mut RowCounts,
    ) {
        out.clear();
        out.resize(fs.len(), RowRoots::default());
        counts.quartics += fs.len() as u64;
        let fall = |i: usize, why: Fallback, out: &mut Vec<RowRoots>, counts: &mut RowCounts| {
            match why {
                Fallback::F3Zero => counts.f3_zero += 1,
                Fallback::ShiftRoot => counts.shift_root += 1,
                Fallback::AlphaZero => counts.alpha_zero += 1,
                Fallback::Bijective => counts.bijective += 1,
                Fallback::NoExponent => counts.no_exponent += 1,
            }
            out[i] = RowRoots::from_gcd(&fallback.gcd(fs[i], gf));
        };

        // Stage 1: 1/f₃.
        sc.idx.clear();
        sc.inv.clear();
        for (i, f) in fs.iter().enumerate() {
            if f[3] == 0 {
                fall(i, Fallback::F3Zero, out, counts);
            } else {
                sc.idx.push(i);
                sc.inv.push(f[3]);
            }
        }
        gf.batch_inv(&mut sc.inv, &mut sc.tmp);
        // st[k] = [s, b₂, b₀, ., ., .] for the k-th survivor.
        sc.st.clear();
        let mut keep = 0;
        for k in 0..sc.idx.len() {
            let i = sc.idx[k];
            let [f0, f1, f2, f3] = fs[i];
            let s = self.sqrt(gf.mul(f1, sc.inv[k]));
            let s2 = gf.sqr(s);
            let b2 = gf.mul(f3, s) ^ f2;
            let b0 = gf.sqr(s2) ^ gf.mul(f3, gf.mul(s2, s)) ^ gf.mul(f2, s2) ^ gf.mul(f1, s) ^ f0;
            if b0 == 0 {
                fall(i, Fallback::ShiftRoot, out, counts);
                continue;
            }
            sc.idx[keep] = i;
            keep += 1;
            sc.st.push([s, b2, b0, 0, 0, 0]);
        }
        sc.idx.truncate(keep);

        // Stage 2: 1/b₀ = γ.
        sc.inv.clear();
        sc.inv.extend(sc.st.iter().map(|st| st[2]));
        gf.batch_inv(&mut sc.inv, &mut sc.tmp);
        let mut keep = 0;
        for k in 0..sc.idx.len() {
            let i = sc.idx[k];
            let [s, b2, _, _, _, _] = sc.st[k];
            let g = sc.inv[k];
            let alpha = gf.mul(b2, g);
            if alpha == 0 {
                fall(i, Fallback::AlphaZero, out, counts);
                continue;
            }
            let beta = gf.mul(fs[i][3], g);
            let ra = self.sqrt(alpha);
            let a32 = gf.mul(alpha, ra);
            // [s, γ, β, √α, α^{3/2}, .]
            sc.st[keep] = [s, g, beta, ra, a32, 0];
            sc.idx[keep] = i;
            keep += 1;
        }
        sc.idx.truncate(keep);
        sc.st.truncate(keep);

        // Stage 3: 1/(β·α^{3/2}), giving 1/β and a = β/α^{3/2} together;
        // then the cubic's root and the kernel element d.
        sc.inv.clear();
        sc.inv.extend(sc.st.iter().map(|st| gf.mul(st[2], st[4])));
        gf.batch_inv(&mut sc.inv, &mut sc.tmp);
        let mut keep = 0;
        for k in 0..sc.idx.len() {
            let i = sc.idx[k];
            let [s, g, beta, ra, a32, _] = sc.st[k];
            let p = sc.inv[k];
            let (inv_a32, inv_beta) = (gf.mul(beta, p), gf.mul(a32, p));
            let (a, inv_a) = (gf.mul(beta, inv_a32), gf.mul(a32, inv_beta));
            let t = match self.cubic_root(a, inv_a, gf) {
                Ok(t) => t,
                Err(why) => {
                    fall(i, why, out, counts);
                    continue;
                }
            };
            let d = gf.mul(ra, t);
            // [s, γ, β, 1/β, d, .]
            sc.st[keep] = [s, g, beta, inv_beta, d, 0];
            sc.idx[keep] = i;
            keep += 1;
        }
        sc.idx.truncate(keep);
        sc.st.truncate(keep);
        counts.closed_form += keep as u64;

        // Stage 4: 1/d, then the affine roots z.
        sc.inv.clear();
        sc.inv.extend(sc.st.iter().map(|st| st[4]));
        gf.batch_inv(&mut sc.inv, &mut sc.tmp);
        sc.zs.clear();
        sc.zowner.clear();
        for k in 0..sc.idx.len() {
            let [_, g, beta, inv_beta, d, _] = sc.st[k];
            for z in self
                .affine_roots(d, sc.inv[k], beta, inv_beta, g, gf)
                .into_iter()
                .flatten()
            {
                sc.zs.push(z);
                sc.zowner.push(k);
            }
        }

        // Stage 5: x = s + 1/z for every candidate, kept when in V.
        gf.batch_inv(&mut sc.zs, &mut sc.tmp);
        for (j, &iz) in sc.zs.iter().enumerate() {
            let k = sc.zowner[j];
            let x = sc.st[k][0] ^ iz;
            if x >> self.l == 0 {
                out[sc.idx[k]].push(x);
                counts.roots_in_v += 1;
            }
        }
    }
}

/// Up to four roots in `V` for one quartic, or the fallback gcd when it
/// has degree ≥ 2 (rare; its roots are then not listed).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RowRoots {
    pub len: u8,
    pub roots: [u64; 4],
    /// The fallback gcd had degree ≥ 2; `roots` is empty, as in
    /// `decompose`, which then searches `V` by evaluation.
    pub multi_gcd: bool,
}

impl RowRoots {
    #[inline(always)]
    fn push(&mut self, x: u64) {
        self.roots[self.len as usize] = x;
        self.len += 1;
    }

    /// From a gcd of degree ≤ 1 (the root itself is found later, by one
    /// inversion, only on success — as `decompose` does), or flag ≥ 2.
    fn from_gcd(g: &Poly) -> Self {
        let mut r = Self::default();
        match g.deg() {
            None | Some(0) => {}
            Some(1) => {
                // Mark one root, value unresolved: callers that need it
                // pay one inversion, exactly as `decompose` does.
                r.len = 1;
                r.roots[0] = u64::MAX;
            }
            Some(_) => r.multi_gcd = true,
        }
        r
    }

    pub fn count(&self) -> usize {
        self.len as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::koblitz_index_calculus::find_irreducible;
    use crate::cryptanalysis::semaev_decomp::quartic_in_x3;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    fn field(n: u32) -> Gf2 {
        Gf2::new(&find_irreducible(n).expect("irreducible"))
    }

    fn eval(f: [u64; 4], x: u64, gf: &Gf2) -> u64 {
        let mut v = 1u64;
        for c in f.iter().rev() {
            v = gf.mul(v, x) ^ c;
        }
        v
    }

    fn monic(q: &Poly, gf: &Gf2) -> Option<[u64; 4]> {
        (q.deg() == Some(4)).then(|| {
            let li = gf.inv(q.c[4]);
            [0, 1, 2, 3].map(|i| gf.mul(q.c[i], li))
        })
    }

    #[test]
    fn dickson_inverts_the_cubic_on_its_own_branch() {
        for n in [11u32, 13, 17, 23] {
            let gf = field(n);
            let r = RadicalQuartic::new(4, &gf).unwrap();
            let mut rng = StdRng::seed_from_u64(n as u64);
            let (mut solved, mut total) = (0, 0);
            for _ in 0..2000 {
                let a = rng.gen::<u64>() & ((1 << n) - 1);
                if a == 0 {
                    continue;
                }
                total += 1;
                // Brute-force root count of t³ + t + a.
                let roots = (0..1u64 << n.min(13))
                    .filter(|&t| gf.mul(gf.sqr(t), t) ^ t == a)
                    .count();
                match r.cubic_root(a, gf.inv(a), &gf) {
                    Ok(t) => {
                        assert_eq!(gf.mul(gf.sqr(t), t) ^ t, a);
                        solved += 1;
                    }
                    Err(Fallback::Bijective) if n <= 13 => assert_eq!(roots, 0, "n={n} a={a:#x}"),
                    Err(_) => {}
                }
            }
            // A cubic t³+t+a has a root for 2/3 of a (1/2 one, 1/6 three).
            let share = solved as f64 / total as f64;
            assert!((share - 2.0 / 3.0).abs() < 0.05, "n={n} share={share}");
        }
    }

    #[test]
    fn closed_form_finds_exactly_the_subspace_roots() {
        for (n, l) in [(11u32, 5u32), (13, 6), (17, 7), (23, 8)] {
            let gf = field(n);
            let r = RadicalQuartic::new(l, &gf).unwrap();
            let base = QuarticSubspaceRoots::new(l, &gf);
            let mut rng = StdRng::seed_from_u64(1000 + n as u64);
            let mut checked = 0;
            for _ in 0..600 {
                let xr = rng.gen::<u64>() & ((1 << n) - 1);
                let x1 = rng.gen_range(0..1u64 << l);
                let x2 = rng.gen_range(0..1u64 << l);
                let Some(f) = monic(&quartic_in_x3(x1, x2, xr, &gf), &gf) else {
                    continue;
                };
                let truth: Vec<u64> = (0..1u64 << l).filter(|&x| eval(f, x, &gf) == 0).collect();
                let g = base.gcd(f, &gf);
                assert_eq!(g.deg().unwrap_or(0), truth.len(), "baseline disagrees");
                match r.solve(f, &gf) {
                    Solved::Roots(got) => {
                        assert_eq!(got, truth, "n={n} l={l} f={f:x?}");
                        checked += 1;
                    }
                    Solved::Fallback(_) => {}
                }
            }
            assert!(checked > 300, "closed form taken too rarely: {checked}");
        }
    }

    #[test]
    fn random_quartics_with_planted_subspace_roots() {
        // Plant 0, 1, 2 or 4 roots in V so every root-count branch runs.
        let (n, l) = (23u32, 9u32);
        let gf = field(n);
        let r = RadicalQuartic::new(l, &gf).unwrap();
        let base = QuarticSubspaceRoots::new(l, &gf);
        let mut rng = StdRng::seed_from_u64(7);
        let mut by_count = [0usize; 5];
        let mut sc = RowScratch::default();
        let mut out = Vec::new();
        let mut counts = RowCounts::default();
        let mut fs = Vec::new();
        for _ in 0..4000 {
            let k = [0usize, 1, 2, 4][rng.gen_range(0..4)];
            let mut f = [0u64; 5];
            f[0] = 1;
            let mut deg = 0;
            for _ in 0..k {
                let root = rng.gen_range(0..1u64 << l);
                // f ← f·(x + root)
                let mut g = [0u64; 5];
                for i in 0..=deg {
                    g[i + 1] ^= f[i];
                    g[i] ^= gf.mul(f[i], root);
                }
                f = g;
                deg += 1;
            }
            while deg < 4 {
                let c = rng.gen::<u64>() & ((1 << n) - 1);
                let mut g = [0u64; 5];
                for i in 0..=deg {
                    g[i + 1] ^= f[i];
                    g[i] ^= gf.mul(f[i], c);
                }
                f = g;
                deg += 1;
            }
            let f4 = [f[0], f[1], f[2], f[3]];
            fs.push(f4);
            let truth: Vec<u64> = (0..1u64 << l).filter(|&x| eval(f4, x, &gf) == 0).collect();
            by_count[truth.len().min(4)] += 1;
            if let Solved::Roots(got) = r.solve(f4, &gf) {
                assert_eq!(got, truth);
            }
            assert_eq!(base.gcd(f4, &gf).deg().unwrap_or(0), truth.len());
        }
        // The batched row pipeline agrees with the scalar reference.
        r.row(&fs, &base, &gf, &mut sc, &mut out, &mut counts);
        for (f, got) in fs.iter().zip(&out) {
            let truth: Vec<u64> = (0..1u64 << l).filter(|&x| eval(*f, x, &gf) == 0).collect();
            if got.multi_gcd {
                assert!(truth.len() >= 2);
            } else {
                assert_eq!(got.count(), truth.len());
                if got.roots[..got.count()].iter().all(|&x| x != u64::MAX) {
                    let mut g = got.roots[..got.count()].to_vec();
                    g.sort_unstable();
                    assert_eq!(g, truth);
                }
            }
        }
        assert!(
            by_count.iter().filter(|&&c| c > 0).count() >= 4,
            "{by_count:?}"
        );
    }
}
