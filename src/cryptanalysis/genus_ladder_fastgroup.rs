//! A cheaper group law for the genus-ladder rho reference (smoke instrument,
//! 2026-11-16; research directory `research/hecdlp_fastgroup_20261116/`).
//!
//! [`FastJ`] adds and doubles reduced Mumford divisors on `y² = f(x)`,
//! `deg f = 2g + 1`, `h = 0`, at genus 3 and 4, with the same
//! [`JDiv`] representation as [`JCurve::cantor`] and exactly the same
//! results.  Every multiplication and inversion of the fast path is tallied
//! as it is executed (`mm`, `inverse`), so the counts are exact.
//!
//! # Algorithm (generic case: `deg u₁ = deg u₂ = g`, coprime `u`'s)
//!
//! The composition is Cantor's with Lange/Nagao's "one reduction step folded
//! into the composition" and with all divisions done *projectively*, so the
//! whole addition needs inversions only at two points that depend on nothing
//! but earlier field products:
//!
//! 1. `s ≡ (v₂ − v₁)/u₁ (mod u₂)` (doubling: `s ≡ k/(2v) (mod u)` with
//!    `k = (f − v²)/u`).  `s` solves a `g × g` linear system whose columns are
//!    `x^j u₁ mod u₂`; it is solved by division-free Gaussian elimination and a
//!    projective back substitution, giving `s̃ = r·s` for a scalar `r` (the
//!    product of the pivots) with no inversion.
//! 2. `u' = ((s l + 2 s v₁ − k) / u₂)` normalised, where `l = s u₁` and
//!    `k = (f − v₁²)/u₁` (`deg k = g + 1`; only its two top coefficients
//!    enter, because only the coefficients of `s(l + 2v₁) − k` of degree
//!    `≥ g` are needed for the exact quotient by the monic `u₂`).  In
//!    projective form `ũ = r²(…)` has leading coefficient `σ̃²`, `σ̃ = s̃_{g-1}`.
//!    **Inversion 1**: `1/(r σ̃)`, from which `1/r` and `1/σ̃²` follow by two
//!    products.
//! 3. `v' = −(l + v₁) mod u'`, then the second reduction step
//!    `u'' = (f − v'²)/u'` (again only the top coefficients of `v'²` are
//!    needed), `v'' = −v' mod u''`.  At genus 4 the leading coefficient of
//!    `u''` is `−v'_{5}²`: **inversion 2** (at genus 3 it is the leading
//!    coefficient of `f`, a constant: no inversion).
//!
//! Both inversions are *stages* of a small state machine
//! ([`FastJ::add_batch`]): `W` independent additions run in lock step, and the
//! inversions of one stage are done together by Montgomery's simultaneous
//! inversion (`1` inversion and `3(W − 1)` multiplications per stage).
//!
//! Degenerate inputs (an identity operand, `deg u < g`, a common root of the
//! `u`'s, `a = −b`, a doubling with `gcd(u, v) ≠ 1`, a vanishing pivot,
//! `σ̃ = 0`, `v'_5 = 0`) are detected on the way and delegated to
//! [`JCurve::cantor`]; their field work is tallied separately
//! (`fallback_*`).  They have probability `O(g/p)` per operation.
//!
//! Sources relied on (algorithmic ideas only; operation counts below are
//! this module's own, none of the published counts was reproduced or
//! checked): Cantor 1987; Harley (genus-2 explicit formulas); Lange 2005
//! (Formulae for arithmetic on genus 2 hyperelliptic curves); Nagao 2000
//! (genus > 2 divisor arithmetic with polynomial remainder sequences);
//! Pelzl, Wollinger, Guajardo, Paar 2003 (genus 3 and 4 explicit formulae);
//! Montgomery 1987 (simultaneous inversion).  The details of those papers
//! were not available offline in this session.

use std::cell::Cell;

use super::genus_ladder::{JCurve, JDiv};

/// Polynomial scratch capacity (largest intermediate: degree `3g − 2 = 10`).
const MX: usize = 12;

/// Exact operation tallies of a [`FastJ`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FastTally {
    /// Additions or doublings completed on the fast path.
    pub fast_ops: u64,
    /// `F_p` multiplications (squarings included) of the fast path,
    /// batch-inversion products included.
    pub fast_mul: u64,
    /// `F_p` inversions of the fast path (a batch inversion counts once).
    pub fast_inv: u64,
    /// Operations delegated to `JCurve::cantor` (degenerate inputs).
    pub fallback_ops: u64,
    pub fallback_mul: u64,
    pub fallback_inv: u64,
    /// Operations with an identity operand (no field work).
    pub trivial_ops: u64,
}

/// The fast group law of one curve.
pub struct FastJ {
    curve: JCurve,
    p: u64,
    g: usize,
    f: [u64; MX],
    monic: bool,
    /// `1 / f_{2g+1}` (setup, not tallied).
    f_lead_inv: u64,
    mul: Cell<u64>,
    inv: Cell<u64>,
    fast_ops: Cell<u64>,
    fb_ops: Cell<u64>,
    fb_mul: Cell<u64>,
    fb_inv: Cell<u64>,
    triv: Cell<u64>,
}

#[derive(Clone, Copy)]
struct Mid1 {
    a: JDiv,
    b: JDiv,
    r: u64,
    sig: u64,
    q: [u64; MX],
    wt: [u64; MX],
    den: u64,
}

#[derive(Clone, Copy)]
struct Mid2 {
    vp: [u64; MX],
    q2: [u64; MX],
    lead: u64,
}

// Mid1 stays inline so a batched group operation performs no per-point heap allocation.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Copy)]
// A transient Copy value on the Cantor-addition hot path: boxing `Mid` would
// allocate once per addition, so the size difference is accepted.
#[allow(clippy::large_enum_variant)]
enum Stage1 {
    Done(JDiv),
    Mid(Mid1),
}

#[derive(Clone, Copy)]
enum Stage2 {
    Done(JDiv),
    Mid(Mid2),
}

impl FastJ {
    /// Genus 3 or 4, `deg f = 2g + 1`, `f` squarefree (checked by
    /// [`JCurve::new`]).
    pub fn new(p: u64, genus: u32, f: &[u64]) -> Result<FastJ, String> {
        if genus != 3 && genus != 4 {
            return Err(format!("FastJ supports genus 3 and 4, not {genus}"));
        }
        let curve = JCurve::new(p, genus, f)?;
        let g = genus as usize;
        let mut fa = [0u64; MX];
        fa[..f.len()].copy_from_slice(f);
        let lead = f[2 * g + 1];
        // lead^(p-2)
        let (mut e, mut b, mut acc) = (p - 2, lead, 1u64);
        while e > 0 {
            if e & 1 == 1 {
                acc = acc * b % p;
            }
            b = b * b % p;
            e >>= 1;
        }
        Ok(FastJ {
            curve,
            p,
            g,
            f: fa,
            monic: lead == 1,
            f_lead_inv: acc,
            mul: Cell::new(0),
            inv: Cell::new(0),
            fast_ops: Cell::new(0),
            fb_ops: Cell::new(0),
            fb_mul: Cell::new(0),
            fb_inv: Cell::new(0),
            triv: Cell::new(0),
        })
    }

    /// The reference curve (Cantor), for comparison and for the fallback.
    pub fn curve(&self) -> &JCurve {
        &self.curve
    }

    pub fn tally(&self) -> FastTally {
        FastTally {
            fast_ops: self.fast_ops.get(),
            fast_mul: self.mul.get(),
            fast_inv: self.inv.get(),
            fallback_ops: self.fb_ops.get(),
            fallback_mul: self.fb_mul.get(),
            fallback_inv: self.fb_inv.get(),
            trivial_ops: self.triv.get(),
        }
    }

    pub fn reset_tally(&self) {
        for c in [
            &self.mul,
            &self.inv,
            &self.fast_ops,
            &self.fb_ops,
            &self.fb_mul,
            &self.fb_inv,
            &self.triv,
        ] {
            c.set(0);
        }
    }

    // ── counted field arithmetic ────────────────────────────────────────

    #[inline(always)]
    fn mm(&self, a: u64, b: u64) -> u64 {
        self.mul.set(self.mul.get() + 1);
        a * b % self.p
    }
    #[inline(always)]
    fn ad(&self, a: u64, b: u64) -> u64 {
        let s = a + b;
        if s >= self.p {
            s - self.p
        } else {
            s
        }
    }
    #[inline(always)]
    fn sb(&self, a: u64, b: u64) -> u64 {
        if a >= b {
            a - b
        } else {
            a + self.p - b
        }
    }
    #[inline(always)]
    fn ng(&self, a: u64) -> u64 {
        if a == 0 {
            0
        } else {
            self.p - a
        }
    }
    /// One counted inversion (extended Euclid, as `JCurve` does), `a ≠ 0`.
    fn inverse(&self, a: u64) -> u64 {
        debug_assert!(a != 0 && a < self.p);
        self.inv.set(self.inv.get() + 1);
        let (mut r0, mut r1) = (self.p as i64, a as i64);
        let (mut t0, mut t1) = (0i64, 1i64);
        while r1 != 0 {
            let q = r0 / r1;
            (r0, r1) = (r1, r0 - q * r1);
            (t0, t1) = (t1, t0 - q * t1);
        }
        t0.rem_euclid(self.p as i64) as u64
    }

    /// Karatsuba product of two length-`n` coefficient slices (`n ≤ 4`),
    /// `out` of length `2n − 1`: 1, 3, 6, 9 multiplications for `n = 1..4`.
    fn kmul(&self, a: &[u64], b: &[u64], out: &mut [u64]) {
        let n = a.len();
        match n {
            1 => out[0] = self.mm(a[0], b[0]),
            2 => {
                let c0 = self.mm(a[0], b[0]);
                let c1 = self.mm(a[1], b[1]);
                let m = self.mm(self.ad(a[0], a[1]), self.ad(b[0], b[1]));
                out[0] = c0;
                out[1] = self.sb(self.sb(m, c0), c1);
                out[2] = c1;
            }
            3 => {
                let c0 = self.mm(a[0], b[0]);
                let c1 = self.mm(a[1], b[1]);
                let c2 = self.mm(a[2], b[2]);
                let m01 = self.mm(self.ad(a[0], a[1]), self.ad(b[0], b[1]));
                let m02 = self.mm(self.ad(a[0], a[2]), self.ad(b[0], b[2]));
                let m12 = self.mm(self.ad(a[1], a[2]), self.ad(b[1], b[2]));
                out[0] = c0;
                out[1] = self.sb(self.sb(m01, c0), c1);
                out[2] = self.ad(self.sb(self.sb(m02, c0), c2), c1);
                out[3] = self.sb(self.sb(m12, c1), c2);
                out[4] = c2;
            }
            4 => {
                let mut lo = [0u64; 3];
                let mut hi = [0u64; 3];
                let mut mid = [0u64; 3];
                self.kmul(&a[0..2], &b[0..2], &mut lo);
                self.kmul(&a[2..4], &b[2..4], &mut hi);
                let sa = [self.ad(a[0], a[2]), self.ad(a[1], a[3])];
                let sbb = [self.ad(b[0], b[2]), self.ad(b[1], b[3])];
                self.kmul(&sa, &sbb, &mut mid);
                for o in out.iter_mut().take(7) {
                    *o = 0;
                }
                for i in 0..3 {
                    out[i] = lo[i];
                    out[4 + i] = hi[i];
                }
                for i in 0..3 {
                    let m = self.sb(self.sb(mid[i], lo[i]), hi[i]);
                    out[2 + i] = self.ad(out[2 + i], m);
                }
            }
            _ => unreachable!("kmul supports lengths 1 to 4"),
        }
    }

    // ── fallback ────────────────────────────────────────────────────────

    fn fallback(&self, a: JDiv, b: JDiv) -> JDiv {
        if a.is_identity() || b.is_identity() {
            self.triv.set(self.triv.get() + 1);
            return self.curve.cantor(a, b);
        }
        let (m0, i0) = (self.curve.fmul(), self.curve.finv());
        let d = self.curve.cantor(a, b);
        self.fb_ops.set(self.fb_ops.get() + 1);
        self.fb_mul.set(self.fb_mul.get() + self.curve.fmul() - m0);
        self.fb_inv.set(self.fb_inv.get() + self.curve.finv() - i0);
        d
    }

    // ── stage 1: everything up to the first inversion ───────────────────

    fn stage1(&self, a: JDiv, b: JDiv) -> Stage1 {
        let g = self.g;
        if a.is_identity() || b.is_identity() || a.deg as usize != g || b.deg as usize != g {
            return Stage1::Done(self.fallback(a, b));
        }
        let dbl = a == b;
        let (mut u1, mut v1, mut u2, mut v2) = ([0u64; MX], [0u64; MX], [0u64; MX], [0u64; MX]);
        for i in 0..g {
            u1[i] = u64::from(a.u[i]);
            v1[i] = u64::from(a.v[i]);
            u2[i] = u64::from(b.u[i]);
            v2[i] = u64::from(b.v[i]);
        }
        let f = &self.f;
        let phi = f[2 * g + 1];
        // matrix M[i][j] (j < g: columns x^j * c mod div), M[i][g]: right side
        let mut m = [[0u64; 6]; 5];
        let mut col = [0u64; MX];
        let div: [u64; MX];
        let (kg, kg1): (u64, u64);
        if !dbl {
            div = u2;
            let mut nz = false;
            for i in 0..g {
                m[i][g] = self.sb(v2[i], v1[i]);
                nz |= m[i][g] != 0;
            }
            if !nz {
                return Stage1::Done(self.fallback(a, b));
            }
            let mut zc = true;
            for i in 0..g {
                col[i] = self.sb(u1[i], u2[i]);
                zc &= col[i] == 0;
            }
            if zc {
                return Stage1::Done(self.fallback(a, b));
            }
            // top two coefficients of k = (f - v1^2)/u1
            kg1 = phi;
            kg = if self.monic {
                self.sb(f[2 * g], u1[g - 1])
            } else {
                self.sb(f[2 * g], self.mm(u1[g - 1], phi))
            };
        } else {
            div = u1;
            // k = (f - v^2) / u, all g+2 coefficients; v^2 needed at
            // degrees g ..= 2g-2.
            let mut nn = [0u64; MX]; // N_t for t = g ..= 2g+1 stored at t - g
            for t in g..=2 * g + 1 {
                let mut sq = 0u64;
                if t <= 2 * g - 2 {
                    let lo = t.saturating_sub(g - 1);
                    for i in lo..=t / 2 {
                        let j = t - i;
                        let pr = self.mm(v1[i], v1[j]);
                        sq = self.ad(sq, pr);
                        if i < j {
                            sq = self.ad(sq, pr);
                        }
                    }
                }
                nn[t - g] = self.sb(f[t], sq);
            }
            let mut k = [0u64; MX];
            for i in (0..=g + 1).rev() {
                let mut acc = nn[i];
                for j in 0..g {
                    let idx = g + i - j;
                    if idx <= g + 1 {
                        acc = self.sb(acc, self.mm(u1[j], k[idx]));
                    }
                }
                k[i] = acc;
            }
            kg1 = k[g + 1];
            kg = k[g];
            // k mod u
            let c1 = k[g + 1];
            for i in 0..g {
                k[i + 1] = self.sb(k[i + 1], self.mm(c1, u1[i]));
            }
            let c0 = k[g];
            for i in 0..g {
                k[i] = self.sb(k[i], self.mm(c0, u1[i]));
            }
            let mut nz = false;
            for i in 0..g {
                m[i][g] = k[i];
                nz |= k[i] != 0;
                col[i] = self.ad(v1[i], v1[i]);
            }
            if !nz {
                return Stage1::Done(self.fallback(a, b));
            }
        }
        // columns
        for j in 0..g {
            for i in 0..g {
                m[i][j] = col[i];
            }
            if j + 1 < g {
                let top = col[g - 1];
                let mut nc = [0u64; MX];
                nc[0] = self.ng(self.mm(top, div[0]));
                for i in 1..g {
                    nc[i] = self.sb(col[i - 1], self.mm(top, div[i]));
                }
                col = nc;
            }
        }
        // division-free elimination with row pivoting
        for k in 0..g {
            let Some(pr) = (k..g).find(|&i| m[i][k] != 0) else {
                return Stage1::Done(self.fallback(a, b));
            };
            m.swap(k, pr);
            let dk = m[k][k];
            for i in k + 1..g {
                let aik = m[i][k];
                for j in k + 1..=g {
                    let t1 = self.mm(dk, m[i][j]);
                    let t2 = self.mm(aik, m[k][j]);
                    m[i][j] = self.sb(t1, t2);
                }
                m[i][k] = 0;
            }
        }
        // projective back substitution: s~ = r s, r = product of pivots
        let mut n = [0u64; MX];
        n[g - 1] = m[g - 1][g];
        let mut dprod = m[g - 1][g - 1];
        for k in (0..g - 1).rev() {
            let mut acc = self.mm(m[k][g], dprod);
            for j in k + 1..g {
                acc = self.sb(acc, self.mm(m[k][j], n[j]));
            }
            for nj in n.iter_mut().take(g).skip(k + 1) {
                *nj = self.mm(*nj, m[k][k]);
            }
            n[k] = acc;
            dprod = self.mm(dprod, m[k][k]);
        }
        let r = dprod;
        let sig = n[g - 1];
        if sig == 0 {
            return Stage1::Done(self.fallback(a, b));
        }
        // l~ = s~ u1 (degree 2g-1)
        let mut kp = [0u64; MX];
        self.kmul(&n[..g], &u1[..g], &mut kp[..2 * g - 1]);
        let mut l = [0u64; MX];
        l[..2 * g - 1].copy_from_slice(&kp[..2 * g - 1]);
        for i in 0..g {
            l[g + i] = self.ad(l[g + i], n[i]);
        }
        let mut wt = l;
        let mut mt = l;
        for i in 0..g {
            let rv = self.mm(r, v1[i]);
            wt[i] = self.ad(wt[i], rv);
            mt[i] = self.ad(self.ad(mt[i], rv), rv);
        }
        // P = s~ m~, degrees 0 ..= 3g-2
        let mut pl = [0u64; MX];
        let mut ph = [0u64; MX];
        self.kmul(&n[..g], &mt[..g], &mut pl[..2 * g - 1]);
        self.kmul(&n[..g], &mt[g..2 * g], &mut ph[..2 * g - 1]);
        let mut pp = [0u64; 2 * MX];
        pp[..2 * g - 1].copy_from_slice(&pl[..2 * g - 1]);
        for i in 0..2 * g - 1 {
            pp[g + i] = self.ad(pp[g + i], ph[i]);
        }
        // N~_t for t = g ..= 3g-2 (stored at t - g)
        let r2 = self.mm(r, r);
        let mut nd = [0u64; MX];
        for i in 0..=2 * g - 2 {
            nd[i] = pp[g + i];
        }
        nd[0] = self.sb(nd[0], self.mm(r2, kg));
        nd[1] = self.sb(nd[1], if kg1 == 1 { r2 } else { self.mm(r2, kg1) });
        // exact quotient by the monic divisor
        let mut q = [0u64; MX];
        for i in (0..=2 * g - 2).rev() {
            let mut acc = nd[i];
            for j in 0..g {
                let idx = g + i - j;
                if idx <= 2 * g - 2 {
                    acc = self.sb(acc, self.mm(div[j], q[idx]));
                }
            }
            q[i] = acc;
        }
        let den = self.mm(r, sig);
        Stage1::Mid(Mid1 {
            a,
            b,
            r,
            sig,
            q,
            wt,
            den,
        })
    }

    // ── stage 2: up to the second inversion ─────────────────────────────

    fn stage2(&self, s: Mid1, iden: u64) -> Stage2 {
        let g = self.g;
        let du = 2 * g - 2;
        let ir = self.mm(s.sig, iden);
        let is = self.mm(s.r, iden);
        let is2 = self.mm(is, is);
        let mut up = [0u64; MX];
        for i in 0..du {
            up[i] = self.mm(s.q[i], is2);
        }
        // w~ mod u'
        let mut w = s.wt;
        let c = w[2 * g - 1];
        for i in 0..du {
            w[i + 1] = self.sb(w[i + 1], self.mm(c, up[i]));
        }
        w[2 * g - 1] = 0;
        let c = w[du];
        for i in 0..du {
            w[i] = self.sb(w[i], self.mm(c, up[i]));
        }
        let nir = self.ng(ir);
        let mut vp = [0u64; MX];
        for i in 0..du {
            vp[i] = self.mm(w[i], nir);
        }
        // N'' = f - v'^2 at degrees du ..= dmax
        let dm = du - 1;
        let dmax = (2 * g + 1).max(2 * dm);
        let dq = dmax - du;
        let mut nn = [0u64; MX];
        for t in du..=dmax {
            let mut sq = 0u64;
            if t <= 2 * dm {
                let lo = t.saturating_sub(dm);
                for i in lo..=t / 2 {
                    let j = t - i;
                    let pr = self.mm(vp[i], vp[j]);
                    sq = self.ad(sq, pr);
                    if i < j {
                        sq = self.ad(sq, pr);
                    }
                }
            }
            let ft = if t <= 2 * g + 1 { self.f[t] } else { 0 };
            nn[t - du] = self.sb(ft, sq);
        }
        let mut q2 = [0u64; MX];
        for i in (0..=dq).rev() {
            let mut acc = nn[i];
            for j in 0..du {
                let idx = du + i - j;
                if idx <= dq {
                    acc = self.sb(acc, self.mm(up[j], q2[idx]));
                }
            }
            q2[i] = acc;
        }
        let lead = q2[dq];
        if lead == 0 || dq != g {
            return Stage2::Done(self.fallback(s.a, s.b));
        }
        Stage2::Mid(Mid2 { vp, q2, lead })
    }

    /// Whether stage 3 needs an inversion of `lead` (genus 4).
    #[inline]
    fn needs_inv2(&self) -> bool {
        self.g == 4
    }

    // ── stage 3 ─────────────────────────────────────────────────────────

    fn stage3(&self, s: Mid2, ilead: u64) -> JDiv {
        let g = self.g;
        let mut ul = [0u64; MX];
        if self.needs_inv2() {
            for i in 0..g {
                ul[i] = self.mm(s.q2[i], ilead);
            }
        } else if self.monic {
            ul[..g].copy_from_slice(&s.q2[..g]);
        } else {
            for i in 0..g {
                ul[i] = self.mm(s.q2[i], self.f_lead_inv);
            }
        }
        let mut v = s.vp;
        for t in (g..=2 * g - 3).rev() {
            let c = v[t];
            for j in 0..g {
                v[t - g + j] = self.sb(v[t - g + j], self.mm(c, ul[j]));
            }
            v[t] = 0;
        }
        let mut d = JDiv {
            deg: g as u8,
            ..JDiv::IDENTITY
        };
        for i in 0..g {
            d.u[i] = ul[i] as u32;
            d.v[i] = self.ng(v[i]) as u32;
        }
        self.fast_ops.set(self.fast_ops.get() + 1);
        d
    }

    // ── public API ──────────────────────────────────────────────────────

    /// `a + b` with at most two inversions of its own.
    pub fn add(&self, a: JDiv, b: JDiv) -> JDiv {
        match self.stage1(a, b) {
            Stage1::Done(d) => d,
            Stage1::Mid(m1) => {
                let iden = self.inverse(m1.den);
                match self.stage2(m1, iden) {
                    Stage2::Done(d) => d,
                    Stage2::Mid(m2) => {
                        let il = if self.needs_inv2() {
                            self.inverse(m2.lead)
                        } else {
                            0
                        };
                        self.stage3(m2, il)
                    }
                }
            }
        }
    }

    pub fn double(&self, a: JDiv) -> JDiv {
        self.add(a, a)
    }

    /// Montgomery's simultaneous inversion: one inversion and `3(n − 1)`
    /// multiplications for `n` nonzero values.
    fn batch_inv(&self, vals: &[u64]) -> Vec<u64> {
        let n = vals.len();
        if n == 0 {
            return Vec::new();
        }
        let mut pre = Vec::with_capacity(n);
        let mut acc = vals[0];
        pre.push(acc);
        for &x in &vals[1..] {
            acc = self.mm(acc, x);
            pre.push(acc);
        }
        let mut it = self.inverse(acc);
        let mut out = vec![0u64; n];
        for i in (1..n).rev() {
            out[i] = self.mm(pre[i - 1], it);
            it = self.mm(it, vals[i]);
        }
        out[0] = it;
        out
    }

    /// `pairs[i].0 + pairs[i].1` for all `i`, the inversions of each stage
    /// batched across the whole slice.
    pub fn add_batch(&self, pairs: &[(JDiv, JDiv)]) -> Vec<JDiv> {
        let mut out: Vec<Option<JDiv>> = vec![None; pairs.len()];
        let mut mids: Vec<(usize, Mid1)> = Vec::new();
        for (i, &(a, b)) in pairs.iter().enumerate() {
            match self.stage1(a, b) {
                Stage1::Done(d) => out[i] = Some(d),
                Stage1::Mid(m) => mids.push((i, m)),
            }
        }
        let dens: Vec<u64> = mids.iter().map(|(_, m)| m.den).collect();
        let inv1 = self.batch_inv(&dens);
        let mut mids2: Vec<(usize, Mid2)> = Vec::new();
        for ((i, m1), iv) in mids.into_iter().zip(inv1) {
            match self.stage2(m1, iv) {
                Stage2::Done(d) => out[i] = Some(d),
                Stage2::Mid(m) => mids2.push((i, m)),
            }
        }
        let inv2 = if self.needs_inv2() {
            let leads: Vec<u64> = mids2.iter().map(|(_, m)| m.lead).collect();
            self.batch_inv(&leads)
        } else {
            vec![0; mids2.len()]
        };
        for ((i, m2), iv) in mids2.into_iter().zip(inv2) {
            out[i] = Some(self.stage3(m2, iv));
        }
        out.into_iter()
            .map(|d| d.expect("every slot filled"))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::super::genus_ladder::{random_f, JCurve, JDiv};
    use super::*;

    fn mk(p: u64, g: u32, seed0: u64) -> FastJ {
        for s in seed0.. {
            if let Ok(c) = FastJ::new(p, g, &random_f(p, g, s)) {
                return c;
            }
        }
        unreachable!()
    }

    /// Divisors of every shape: identity, points, partial sums, random
    /// reduced divisors, negatives.
    fn pool(c: &JCurve, want: usize, st: &mut u64) -> Vec<JDiv> {
        let p = c.p();
        let mut v = vec![JDiv::IDENTITY];
        for x in 0..p.min(40) {
            for y in 0..p {
                if let Some(d) = c.point(x, y) {
                    v.push(d);
                    if v.len() > want / 3 {
                        break;
                    }
                }
            }
        }
        let pts = v.clone();
        for i in 0..pts.len().min(12) {
            for j in 0..pts.len().min(12) {
                v.push(c.cantor(pts[i], pts[j]));
            }
        }
        while v.len() < want {
            v.push(c.random_divisor(st));
        }
        let neg: Vec<JDiv> = v.iter().map(|&d| c.negate(d)).collect();
        v.extend(neg);
        v.sort_by_key(|d| (d.deg, d.u, d.v));
        v.dedup();
        v
    }

    fn check_all_pairs(p: u64, g: u32, seed: u64) {
        let fj = mk(p, g, seed);
        let c = fj.curve();
        let mut st = seed ^ 0xABCD;
        let pl = pool(c, 60, &mut st);
        let mut pairs = Vec::new();
        for &a in &pl {
            for &b in &pl {
                let want = c.cantor(a, b);
                assert_eq!(fj.add(a, b), want, "p={p} g={g} a={a:?} b={b:?}");
                pairs.push((a, b));
            }
        }
        let batch = fj.add_batch(&pairs);
        for ((a, b), got) in pairs.iter().zip(batch) {
            assert_eq!(got, c.cantor(*a, *b), "batch p={p} g={g}");
        }
        // group-law identities on the fast law
        for w in pl.windows(3).take(200) {
            let (a, b, d) = (w[0], w[1], w[2]);
            assert_eq!(fj.add(fj.add(a, b), d), fj.add(a, fj.add(b, d)));
            assert!(fj.add(a, c.negate(a)).is_identity());
        }
    }

    #[test]
    fn kmul_matches_schoolbook() {
        let fj = mk(8209, 3, 1);
        let p = 8209u64;
        let mut s = 12345u64;
        for n in 1..=4usize {
            for _ in 0..200 {
                let mut a = vec![0u64; n];
                let mut b = vec![0u64; n];
                for i in 0..n {
                    s = s
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    a[i] = (s >> 33) % p;
                    s = s
                        .wrapping_mul(6364136223846793005)
                        .wrapping_add(1442695040888963407);
                    b[i] = (s >> 33) % p;
                }
                let mut out = vec![0u64; 2 * n - 1];
                let m0 = fj.mul.get();
                fj.kmul(&a, &b, &mut out);
                assert_eq!(fj.mul.get() - m0, [1, 3, 6, 9][n - 1]);
                for k in 0..2 * n - 1 {
                    let mut e = 0u64;
                    for i in 0..n {
                        if k >= i && k - i < n {
                            e = (e + a[i] * b[k - i]) % p;
                        }
                    }
                    assert_eq!(out[k], e, "n={n} k={k}");
                }
            }
        }
    }

    #[test]
    fn small_primes_all_cases_g3_g4() {
        // Tiny fields make every degenerate case frequent (deg < g, shared
        // roots, a = -b, doubling of 2-torsion, singular systems).
        for (p, g, s) in [(7u64, 3u32, 1u64), (11, 3, 5), (13, 4, 2), (17, 4, 9)] {
            check_all_pairs(p, g, s);
        }
    }

    #[test]
    fn medium_primes_g3_g4() {
        check_all_pairs(101, 3, 3);
        check_all_pairs(103, 4, 3);
    }

    #[test]
    fn large_prime_random_and_exact_counts() {
        for (p, g) in [(8209u64, 3u32), (8209, 4), (32749, 3), (32749, 4)] {
            let fj = mk(p, g, 100);
            let c = fj.curve();
            let mut st = 77u64;
            let mut seen_add: Option<(u64, u64)> = None;
            let mut seen_dbl: Option<(u64, u64)> = None;
            let (mut fast, mut total) = (0u64, 0u64);
            for _ in 0..1500 {
                let a = c.random_divisor(&mut st);
                let b = c.random_divisor(&mut st);
                for (x, y) in [(a, b), (a, a)] {
                    let t0 = fj.tally();
                    let got = fj.add(x, y);
                    let t1 = fj.tally();
                    assert_eq!(got, c.cantor(x, y));
                    total += 1;
                    if t1.fast_ops > t0.fast_ops {
                        fast += 1;
                        let cost = (t1.fast_mul - t0.fast_mul, t1.fast_inv - t0.fast_inv);
                        let slot = if x == y { &mut seen_dbl } else { &mut seen_add };
                        match slot {
                            None => *slot = Some(cost),
                            Some(c0) => assert_eq!(*c0, cost, "fast-path cost is constant"),
                        }
                    }
                }
            }
            assert!(fast * 100 >= total * 99, "fast path {fast}/{total}");
            eprintln!("p={p} g={g}: add {seen_add:?} dbl {seen_dbl:?} fast {fast}/{total}");
            // batched == single
            let pairs: Vec<(JDiv, JDiv)> = (0..64)
                .map(|_| (c.random_divisor(&mut st), c.random_divisor(&mut st)))
                .collect();
            let got = fj.add_batch(&pairs);
            for ((a, b), d) in pairs.iter().zip(got) {
                assert_eq!(d, c.cantor(*a, *b));
            }
        }
    }

    #[test]
    fn non_monic_f_is_handled() {
        // f with leading coefficient 3: the instrument uses monic f, the
        // module must still be right.
        for g in [3u32, 4] {
            let p = 8209u64;
            let mut f = random_f(p, g, 5);
            *f.last_mut().unwrap() = 3;
            let Ok(fj) = FastJ::new(p, g, &f) else {
                continue;
            };
            let c = fj.curve();
            let mut st = 9u64;
            for _ in 0..300 {
                let a = c.random_divisor(&mut st);
                let b = c.random_divisor(&mut st);
                assert_eq!(fj.add(a, b), c.cantor(a, b));
                assert_eq!(fj.double(a), c.cantor(a, a));
            }
        }
    }

    // ── smoke benchmark (ignored; run explicitly) ───────────────────────

    fn best5<F: FnMut() -> u64>(mut f: F) -> f64 {
        let mut best = f64::MAX;
        for _ in 0..5 {
            let t = std::time::Instant::now();
            let n = f();
            best = best.min(t.elapsed().as_nanos() as f64 / n as f64);
        }
        best
    }

    /// SMOKE, not a verdict.  Per-addition costs and times on random
    /// additions and lock-step walks.  Writes JSON to
    /// `$FASTGROUP_SMOKE_OUT` (default: stderr only).
    #[test]
    #[ignore]
    fn fastgroup_smoke_bench() {
        let mut rows = Vec::new();
        for (p, g, cseed) in [(8209u64, 4u32, 363u64), (8209, 3, 11)] {
            let f = random_f(p, g, cseed);
            let fj = FastJ::new(p, g, &f).unwrap();
            let c = fj.curve();
            let mut st = 4242u64;
            // table of random divisors, lanes = independent walkers
            let table: Vec<JDiv> = (0..32).map(|_| c.random_divisor(&mut st)).collect();
            let lanes0: Vec<JDiv> = (0..64).map(|_| c.random_divisor(&mut st)).collect();
            let hash = |d: &JDiv| (d.u[0] as usize ^ (d.v[0] as usize).wrapping_mul(31)) & 31;
            // counted: single, generic additions
            fj.reset_tally();
            let c0 = (c.fmul(), c.finv());
            let n_single = 3000u64;
            let mut a = lanes0[0];
            for _ in 0..n_single {
                a = fj.add(a, table[hash(&a)]);
            }
            let t = fj.tally();
            let single = (
                t.fast_mul as f64 / t.fast_ops as f64,
                t.fast_inv as f64 / t.fast_ops as f64,
                t.fast_ops,
                t.fallback_ops,
            );
            let mut a = lanes0[0];
            let r0 = (c.fmul(), c.finv());
            for _ in 0..n_single {
                a = c.cantor(a, table[hash(&a)]);
            }
            let cantor_cost = (
                (c.fmul() - r0.0) as f64 / n_single as f64,
                (c.finv() - r0.1) as f64 / n_single as f64,
            );
            let _ = c0;
            // timings, single lane
            let t_fast = best5(|| {
                let mut a = lanes0[0];
                for _ in 0..n_single {
                    a = fj.add(a, table[hash(&a)]);
                }
                std::hint::black_box(a);
                n_single
            });
            let t_cantor = best5(|| {
                let mut a = lanes0[0];
                for _ in 0..n_single {
                    a = c.cantor(a, table[hash(&a)]);
                }
                std::hint::black_box(a);
                n_single
            });
            // batched lock-step walks
            let mut batched = Vec::new();
            for w in [16usize, 64] {
                fj.reset_tally();
                let steps = 400usize;
                let mut lanes: Vec<JDiv> = lanes0[..w].to_vec();
                for _ in 0..steps {
                    let pairs: Vec<(JDiv, JDiv)> =
                        lanes.iter().map(|d| (*d, table[hash(d)])).collect();
                    lanes = fj.add_batch(&pairs);
                }
                let t = fj.tally();
                let ops = (t.fast_ops + t.fallback_ops + t.trivial_ops) as f64;
                let tb = best5(|| {
                    let mut lanes: Vec<JDiv> = lanes0[..w].to_vec();
                    for _ in 0..steps {
                        let pairs: Vec<(JDiv, JDiv)> =
                            lanes.iter().map(|d| (*d, table[hash(d)])).collect();
                        lanes = fj.add_batch(&pairs);
                    }
                    std::hint::black_box(&lanes);
                    (steps * w) as u64
                });
                batched.push((
                    w,
                    t.fast_mul as f64 / t.fast_ops as f64,
                    t.fast_inv as f64 / t.fast_ops as f64,
                    t.fallback_ops,
                    ops,
                    tb,
                ));
            }
            // inversion vs multiplication time (same Euclid as the law)
            let tm = best5(|| {
                let mut x = 12345u64;
                for i in 0..200_000u64 {
                    x = (x * (i + 7919)) % p;
                }
                std::hint::black_box(x);
                200_000
            });
            let ti = best5(|| {
                let mut x = 1u64;
                for i in 1..50_000u64 {
                    x = fj.inverse((x + i) % (p - 1) + 1);
                }
                std::hint::black_box(x);
                50_000
            });
            rows.push(format!(
                "{{\"p\":{p},\"g\":{g},\"fast_single\":{{\"mul\":{:.3},\"inv\":{:.3},\"ops\":{},\"fallback_ops\":{},\"ns\":{:.1}}},\"cantor\":{{\"mul\":{:.3},\"inv\":{:.3},\"ns\":{:.1}}},\"batched\":[{}],\"ns_mulmod\":{:.3},\"ns_inv_euclid\":{:.3}}}",
                single.0, single.1, single.2, single.3, t_fast,
                cantor_cost.0, cantor_cost.1, t_cantor,
                batched
                    .iter()
                    .map(|(w, m, i, fb, ops, tb)| format!(
                        "{{\"W\":{w},\"mul\":{m:.3},\"inv\":{i:.5},\"fallback_ops\":{fb},\"ops\":{ops},\"ns\":{tb:.1}}}"
                    ))
                    .collect::<Vec<_>>()
                    .join(","),
                tm, ti
            ));
        }
        let json = format!("[{}]", rows.join(",\n"));
        eprintln!("{json}");
        if let Ok(path) = std::env::var("FASTGROUP_SMOKE_OUT") {
            std::fs::write(path, json).unwrap();
        }
    }
}
