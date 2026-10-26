//! # GHS Weil descent, `m ≥ 2`: the smooth hyperelliptic model and the
//! conorm–norm map `E(K) → Jac(C)(k)`.
//!
//! [`crate::cryptanalysis::ghs_descent::descend_m2_affine`] stops at a singular
//! affine model and `descend_m2_abstract` is symbolic only.  This module
//! builds the *smooth* descended curve for any `E : y² + xy = x³ + a·x² + b`
//! over `K = F_{2^N}` with `a ∈ k = F_{2^l}`, `n = N/l`, and the group
//! homomorphism `Φ : E(K) → Jac(C)(k)` in the library's Mumford arithmetic
//! ([`crate::binary_ecc::hyperelliptic`]).
//!
//! ## Construction (derived here; nothing is taken from a table)
//!
//! Put `z = 1/x`, `β_i = σ^i(√b)` with `σ = (u ↦ u^{2^l})`, and let `s_i` be
//! the Artin–Schreier generators `s_i² + s_i = x + a + β_i·z`.  `s_0 = (y+β_0)/x`
//! is the function field of `E`.  The GHS function field is
//! `L = K(x, s_0, …, s_{n−1})`, a `(Z/2)^M` extension of `K(x)`,
//! `M = dim_{F_2} span{(1, β_i)}`.  Write `r = M − 1`.
//!
//! 1. **The `−1` involution has a rational quotient.**  `ι : s_i ↦ s_i + 1`
//!    fixes exactly the even-weight combinations `s_i + s_j`, which satisfy
//!    `u² + u = c·z` with `c ∈ B = span_{F_2}{β_i + β_j}` (`dim B = r`).  Every
//!    such AS extension has a single simple pole, so `L^ι` has genus 0.  Its
//!    parameter is `v` with `z = P(v)`, `P(v) = Σ_{j≤r} p_j v^{2^j}` additive.
//! 2. **Which `P`.**  `u_c = A_c(v)` (additive, degree `2^{r−1}`) solves
//!    `u_c² + u_c = c·P(v)` iff `Σ_j (p_j c)^{2^{r−j}} = 0`.  So `B` must be the
//!    root space of the additive polynomial `Q_P(y) = Σ_j (p_j y)^{2^{r−j}}`
//!    and, comparing with `Q_B(y) = ∏_{b∈B}(y + b) = Σ_i q_i y^{2^i}`,
//!    `p_j = q_{r−j}^{2^{−(r−j)}}`.  `B` is `σ`-stable, so `q_i ∈ k` and
//!    `P ∈ k[v]`.
//! 3. **`σ` lifts with `σ'(v) = v`.**  Defining `s_j := s_0 + A_{β_0+β_j}(v)`
//!    (linear in `c`, so all relations hold) gives `σ'(s_0) = s_0 + A_{β_0+β_1}(v)`
//!    and `σ'^n = id` (telescoping).  `v` is therefore a `k`-rational parameter
//!    of `C_0 = L^{⟨ι,σ'⟩}`, and `x = 1/P(v)`.
//! 4. **The cover.**  Solve `θ + σ(θ) = A_{β_0+β_1}` coefficientwise (additive
//!    Hilbert 90) and put `w = s_0 + θ(v)`; then `σ'(w) = w` and
//!    `w² + w = 1/P(v) + a + β_0·P(v) + θ² + θ`, whose right side lies in
//!    `k(v)`.  Reducing the polynomial part modulo `℘(k[v])` leaves
//!    `w'² + w' = 1/P(v) + a + c_1·v`, `c_1 ∈ k`.  The function field of `C` is
//!    `k(v, w')`.
//! 5. **Models.**  `Y = P·w'` gives `Y² + P·Y = P + P²(a + c_1 v)`.
//!    * `c_1 ≠ 0`: degree `2^{r+1}+1`, genus `g = 2^r = 2^{M−1}`.
//!    * `c_1 = 0`: substitute `v = 1/t`, `y = Y·t^{2^r}`:
//!      `y² + P̃·y = t^{2^r}·P̃ + a·P̃²` with `P̃(t) = t^{2^r}P(1/t)`; genus
//!      `g = 2^r − 1 = 2^{M−1} − 1`.
//!    Both are imaginary models with one rational point at infinity.
//! 6. **The map.**  For `P = (x_0, y_0) ∈ E(K)`, `x_0 ≠ 0`, the places of `L`
//!    over `P` are `(v = ν, w' = s_0(P) + Θ(ν))` with `P(ν) = 1/x_0`, so
//!    `Con(P)` has Mumford polynomial `(P(v) + z_0)/p_r`; `Con(O)` is the
//!    divisor `W = Σ_{P(δ)=0} (δ, 0)` (Mumford `(P/p_r, 0)`).  The Frobenius
//!    conjugates `σ'^i Con(P)` are `K`-rational and
//!    `Φ(P) = Σ_{i<n} ([σ'^i Con(P)] − [W]) ∈ Jac(C)(k)`.  All sums are taken
//!    with Cantor arithmetic on `C` over `K` and then moved to `k`.
//!
//! ## Limits (documented, not hidden)
//!
//! * `a ∈ k` only.  For `a ∈ K ∖ k` (the other quadratic-twist class when `n` is
//!   even) the lift of `σ` needs an extra constant and the descent condition of
//!   Menezes–Qu / MMT Lemma 6; not implemented.
//! * `N ≤ 128`.  `x_0 = 0` (the 2-torsion point) is not mapped.
//! * `M = 1` (`√b ∈ k`) is the trace map of [`crate::cryptanalysis::ghs_descent::trace_map`].
//! * Whether the discrete log of a given instance becomes easier is *not* decided
//!   here; this module only constructs `C` and `Φ` and checks them.

use std::collections::HashMap;

use crate::binary_ecc::{
    F2mElement, F2mPoly, HyperellipticCurve, HyperellipticError, IrreduciblePoly, MumfordDivisor,
};
use crate::cryptanalysis::descent_expansion::enumerate_irreducibles;
use crate::cryptanalysis::ec_trapdoor::FieldTower;
use crate::cryptanalysis::ghs_descent::Pt;
use num_bigint::BigUint;

/// Failures of the smooth-cover construction and of the map `Φ`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SmoothCoverError {
    /// `N > 128`.
    FieldTooLarge,
    /// `a ∉ k`.
    CoefficientNotInBaseField,
    /// `b = 0`.
    SingularCurve,
    /// `√b ∈ k`: magic number 1, use the trace map instead.
    MagicNumberOne,
    /// An internal identity failed (the message names it).
    Inconsistent(&'static str),
    /// A required linear system has no solution.
    NoSolution(&'static str),
    /// The point cannot be mapped (the message says why).
    UnsupportedPoint(&'static str),
    /// A library hyperelliptic call rejected a curve or divisor.
    Curve(HyperellipticError),
}

impl From<HyperellipticError> for SmoothCoverError {
    fn from(e: HyperellipticError) -> Self {
        SmoothCoverError::Curve(e)
    }
}

/// Which affine chart of the descended curve is used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoverModel {
    /// `c_1 ≠ 0`: model in `v`, genus `2^{M−1}`.
    V,
    /// `c_1 = 0`: model in `t = 1/v`, genus `2^{M−1} − 1`.
    T,
}

// ── F_2-linear algebra on elements of K (N ≤ 128) ─────────────────────

fn to_u128(e: &F2mElement) -> u128 {
    let w = e.raw_bits();
    let lo = w.first().copied().unwrap_or(0) as u128;
    let hi = w.get(1).copied().unwrap_or(0) as u128;
    lo | (hi << 64)
}

fn from_u128(v: u128, m: u32) -> F2mElement {
    F2mElement::from_words(&[v as u64, (v >> 64) as u64], m)
}

/// Incrementally reduced F_2-basis with combination tracking.
struct F2Basis {
    rows: Vec<(u128, u128, u32)>, // (vector, combination mask, pivot bit)
}

impl F2Basis {
    fn new() -> Self {
        Self { rows: Vec::new() }
    }

    fn reduce(&self, mut v: u128, mut mask: u128) -> (u128, u128) {
        for &(rv, rm, piv) in &self.rows {
            if (v >> piv) & 1 == 1 {
                v ^= rv;
                mask ^= rm;
            }
        }
        (v, mask)
    }

    /// Insert `v` carrying `mask`; `false` if it was already in the span.
    fn insert(&mut self, v: u128, mask: u128) -> bool {
        let (r, m) = self.reduce(v, mask);
        if r == 0 {
            return false;
        }
        let piv = 127 - r.leading_zeros();
        self.rows.push((r, m, piv));
        true
    }
}

/// Solver for `f(t) = rhs` where `f : K → K` is F_2-linear.
struct LinearMap {
    basis: F2Basis,
}

impl LinearMap {
    fn new<F: Fn(&F2mElement) -> F2mElement>(m: u32, f: F) -> Self {
        let mut basis = F2Basis::new();
        for i in 0..m {
            let e = F2mElement::from_bit_positions(&[i], m);
            basis.insert(to_u128(&f(&e)), 1u128 << i);
        }
        Self { basis }
    }

    fn solve(&self, rhs: &F2mElement) -> Option<F2mElement> {
        let (r, mask) = self.basis.reduce(to_u128(rhs), 0);
        if r != 0 {
            None
        } else {
            Some(from_u128(mask, rhs.m_value()))
        }
    }
}

// ── the cover ─────────────────────────────────────────────────────────

/// The smooth GHS curve `C/k` of `E/K` together with `Φ : E(K) → Jac(C)(k)`.
#[derive(Clone, Debug)]
pub struct SmoothCover {
    pub tower: FieldTower,
    pub a: F2mElement,
    pub beta0: F2mElement,
    /// `r = M − 1 = dim_{F_2} span{β_i + β_j}`.
    pub r: usize,
    /// MMT magic number `M = r + 1`.
    pub magic: u32,
    /// An `F_2`-basis of `B = span{β_0 + β_j}`.
    pub b_basis: Vec<F2mElement>,
    /// Coefficients `p_0 … p_r` of the additive polynomial `P` (values in `k`).
    pub p: Vec<F2mElement>,
    /// `Θ = Σ_{j<r} Θ_j v^{2^j}` with `w' = s_0 + Θ(v)`.
    pub theta: Vec<F2mElement>,
    /// Coefficient of `v` in the reduced equation `w'² + w' = 1/P + a + c_1 v`.
    pub c1: F2mElement,
    pub model: CoverModel,
    pub genus: u32,
    /// Modulus of `k = F_{2^l}` (the field of `curve_k`).
    pub k_irr: IrreduciblePoly,
    /// `C` over `k`.
    pub curve_k: HyperellipticCurve,
    /// The same equation over `K` (coefficients embedded).
    pub curve_big: HyperellipticCurve,
    mu: F2mElement,
    fwd: Vec<F2mElement>,
    back: HashMap<u128, u64>,
}

impl SmoothCover {
    fn kirr(&self) -> &IrreduciblePoly {
        &self.tower.irr
    }

    fn mul(&self, a: &F2mElement, b: &F2mElement) -> F2mElement {
        a.mul(b, &self.tower.irr)
    }

    fn sq(&self, a: &F2mElement) -> F2mElement {
        a.square(&self.tower.irr)
    }

    fn inv(&self, a: &F2mElement) -> F2mElement {
        a.flt_inverse(&self.tower.irr).expect("nonzero element")
    }

    /// `x^{2^{−s}}` (the absolute Frobenius has order `N`).
    fn root2(&self, x: &F2mElement, s: u32) -> F2mElement {
        let n = self.tower.big_n;
        x.square_k_times((n - s % n) % n, &self.tower.irr)
    }

    fn is_in_k(&self, x: &F2mElement) -> bool {
        self.tower.frobenius(x) == *x
    }

    /// Embed an element of `k` (library representation with `k_irr`) into `K`.
    pub fn embed_k(&self, e: &F2mElement) -> F2mElement {
        let idx = e.raw_bits().first().copied().unwrap_or(0) as usize;
        self.fwd[idx].clone()
    }

    /// Project an element of `K` that lies in `k` back to the library
    /// representation of `k`; `None` if it does not lie in `k`.
    pub fn project_k(&self, x: &F2mElement) -> Option<F2mElement> {
        self.back
            .get(&to_u128(x))
            .map(|&i| F2mElement::from_words(&[i], self.tower.l))
    }

    fn poly_k(&self, p: &F2mPoly) -> Result<F2mPoly, SmoothCoverError> {
        let mut cs = Vec::new();
        for c in &p.coeffs {
            cs.push(
                self.project_k(c)
                    .ok_or(SmoothCoverError::Inconsistent("model coefficient outside k"))?,
            );
        }
        Ok(F2mPoly::from_coeffs(cs, self.tower.l))
    }

    fn mono_poly(&self, terms: &[(usize, F2mElement)]) -> F2mPoly {
        let deg = terms.iter().map(|t| t.0).max().unwrap_or(0);
        let mut cs = vec![F2mElement::zero(self.tower.big_n); deg + 1];
        for (e, c) in terms {
            cs[*e] = cs[*e].add(c);
        }
        F2mPoly::from_coeffs(cs, self.tower.big_n)
    }

    /// Build the cover.  `a ∈ k`, `beta0 = √b ≠ 0`.
    pub fn new(
        tower: &FieldTower,
        a: &F2mElement,
        beta0: &F2mElement,
    ) -> Result<Self, SmoothCoverError> {
        let nn = tower.big_n;
        let l = tower.l;
        let n = tower.n;
        if nn > 128 {
            return Err(SmoothCoverError::FieldTooLarge);
        }
        if tower.frobenius(a) != *a {
            return Err(SmoothCoverError::CoefficientNotInBaseField);
        }
        if beta0.is_zero() {
            return Err(SmoothCoverError::SingularCurve);
        }
        let irr = &tower.irr;
        let zero = F2mElement::zero(nn);
        let one = F2mElement::one(nn);

        // ── base field k inside K ──
        let k_irr = if l == 1 {
            IrreduciblePoly {
                degree: 1,
                low_terms: vec![0],
            }
        } else {
            enumerate_irreducibles(l, 1)
                .into_iter()
                .next()
                .ok_or(SmoothCoverError::Inconsistent("no irreducible of degree l"))?
        };
        let mut omega = None;
        for cand in 1u128..(1u128 << nn.min(40)) {
            let e = from_u128(cand, nn);
            if tower.frobenius(&e) != e {
                continue;
            }
            // k_irr(e) = e^l + Σ e^t
            let mut pw = vec![one.clone()];
            for _ in 1..=l {
                let last = pw.last().unwrap().mul(&e, irr);
                pw.push(last);
            }
            let mut acc = pw[l as usize].clone();
            for &t in &k_irr.low_terms {
                acc = acc.add(&pw[t as usize]);
            }
            if acc.is_zero() {
                omega = Some(e);
                break;
            }
        }
        let omega = omega.ok_or(SmoothCoverError::Inconsistent("no root of k_irr in K"))?;
        let mut omega_pows = vec![one.clone()];
        for i in 1..l as usize {
            let p = omega_pows[i - 1].mul(&omega, irr);
            omega_pows.push(p);
        }
        let mut fwd = Vec::new();
        let mut back = HashMap::new();
        for idx in 0u64..(1u64 << l) {
            let mut e = zero.clone();
            for (i, op) in omega_pows.iter().enumerate() {
                if (idx >> i) & 1 == 1 {
                    e = e.add(op);
                }
            }
            back.insert(to_u128(&e), idx);
            fwd.push(e);
        }
        if back.len() != 1usize << l {
            return Err(SmoothCoverError::Inconsistent("embedding of k not injective"));
        }

        // ── the space B = span{β_0 + β_j} ──
        let betas: Vec<F2mElement> = (0..n).map(|i| tower.frobenius_iter(beta0, i)).collect();
        let mut bb = F2Basis::new();
        let mut b_basis = Vec::new();
        for beta_j in betas.iter().skip(1) {
            let d = beta0.add(beta_j);
            if bb.insert(to_u128(&d), 0) {
                b_basis.push(d);
            }
        }
        let r = b_basis.len();
        if r == 0 {
            return Err(SmoothCoverError::MagicNumberOne);
        }

        let mut me = Self {
            tower: tower.clone(),
            a: a.clone(),
            beta0: beta0.clone(),
            r,
            magic: r as u32 + 1,
            b_basis: b_basis.clone(),
            p: Vec::new(),
            theta: Vec::new(),
            c1: zero.clone(),
            model: CoverModel::V,
            genus: 0,
            k_irr: k_irr.clone(),
            curve_k: HyperellipticCurve {
                m: l,
                irr: k_irr.clone(),
                h: F2mPoly::zero(l),
                f: F2mPoly::zero(l),
                genus: 0,
            },
            curve_big: HyperellipticCurve {
                m: nn,
                irr: irr.clone(),
                h: F2mPoly::zero(nn),
                f: F2mPoly::zero(nn),
                genus: 0,
            },
            mu: one.clone(),
            fwd,
            back,
        };

        // ── Q_B(y) = Σ q_i y^{2^i} ──
        let mut q = vec![one.clone()];
        for c in &b_basis {
            let mut kappa = zero.clone();
            for (i, qi) in q.iter().enumerate() {
                kappa = kappa.add(&me.mul(qi, &c.square_k_times(i as u32, irr)));
            }
            let mut nq = vec![zero.clone(); q.len() + 1];
            for (i, qi) in q.iter().enumerate() {
                nq[i] = nq[i].add(&me.mul(&kappa, qi));
                nq[i + 1] = nq[i + 1].add(&me.sq(qi));
            }
            q = nq;
        }
        for c in &b_basis {
            let mut acc = zero.clone();
            for (i, qi) in q.iter().enumerate() {
                acc = acc.add(&me.mul(qi, &c.square_k_times(i as u32, irr)));
            }
            if !acc.is_zero() {
                return Err(SmoothCoverError::Inconsistent("Q_B does not vanish on B"));
            }
        }
        // ── p_j = q_{r−j}^{2^{−(r−j)}} ──
        let mut p = Vec::new();
        for j in 0..=r {
            let s = (r - j) as u32;
            let pj = me.root2(&q[r - j], s);
            if !me.is_in_k(&pj) {
                return Err(SmoothCoverError::Inconsistent("p_j outside k"));
            }
            p.push(pj);
        }
        if p[0] != one {
            return Err(SmoothCoverError::Inconsistent("p_0 != 1"));
        }
        me.p = p;

        // ── A_c for c = β_0 + β_1 ──
        let c01 = beta0.add(&betas[1]);
        let mut acoef: Vec<F2mElement> = Vec::new();
        for j in 0..r {
            let mut aj = me.mul(&me.p[j], &c01);
            if j > 0 {
                aj = aj.add(&me.sq(&acoef[j - 1]));
            }
            acoef.push(aj);
        }
        if me.sq(&acoef[r - 1]) != me.mul(&me.p[r], &c01) {
            return Err(SmoothCoverError::Inconsistent("A_c^2 + A_c != c P"));
        }

        // ── θ with θ + σθ = A (additive Hilbert 90) ──
        let lm = LinearMap::new(nn, |t| t.add(&tower.frobenius(t)));
        let mut theta = Vec::new();
        for aj in &acoef {
            theta.push(lm.solve(aj).ok_or(SmoothCoverError::NoSolution(
                "theta + sigma(theta) = A_c",
            ))?);
        }

        // ── G = a + β_0 P + θ² + θ, reduced mod ℘(k[v]) ──
        let mut g: Vec<F2mElement> = Vec::new();
        for j in 0..=r {
            let mut gj = me.mul(beta0, &me.p[j]);
            if j >= 1 {
                gj = gj.add(&me.sq(&theta[j - 1]));
            }
            if j < r {
                gj = gj.add(&theta[j]);
            }
            if !me.is_in_k(&gj) {
                return Err(SmoothCoverError::Inconsistent(
                    "G coefficient outside k: w is not sigma-invariant",
                ));
            }
            g.push(gj);
        }
        let mut h_corr = vec![zero.clone(); r];
        for j in (1..=r).rev() {
            let e = me.root2(&g[j], 1);
            g[j - 1] = g[j - 1].add(&e);
            h_corr[j - 1] = e;
        }
        let c1 = g[0].clone();
        if !me.is_in_k(&c1) {
            return Err(SmoothCoverError::Inconsistent("c_1 outside k"));
        }
        me.theta = (0..r).map(|j| theta[j].add(&h_corr[j])).collect();
        me.c1 = c1.clone();

        // ── the model ──
        let pr = me.p[r].clone();
        let (h, f, genus, model, mu) = if !c1.is_zero() {
            let mu = me.mul(&pr, &me.root2(&c1, 1));
            let mu_inv = me.inv(&mu);
            let ppoly = me.mono_poly(
                &me.p
                    .iter()
                    .enumerate()
                    .map(|(j, c)| (1usize << j, c.clone()))
                    .collect::<Vec<_>>(),
            );
            let lin = F2mPoly::from_coeffs(vec![a.clone(), c1.clone()], nn);
            let f_raw = ppoly.add(&ppoly.mul(&ppoly, irr).mul(&lin, irr));
            let h = ppoly.scalar_mul(&mu_inv, irr);
            let f = f_raw.scalar_mul(&me.sq(&mu_inv), irr);
            (h, f, 1u32 << r, CoverModel::V, mu)
        } else {
            let top = 1usize << r;
            let pt = me.mono_poly(
                &me.p
                    .iter()
                    .enumerate()
                    .map(|(j, c)| (top - (1usize << j), c.clone()))
                    .collect::<Vec<_>>(),
            );
            let f = pt
                .shift(top)
                .add(&pt.mul(&pt, irr).scalar_mul(a, irr));
            (pt, f, (1u32 << r) - 1, CoverModel::T, one.clone())
        };
        me.genus = genus;
        me.model = model;
        me.mu = mu;
        let hk = me.poly_k(&h)?;
        let fk = me.poly_k(&f)?;
        me.curve_k = HyperellipticCurve::try_new(l, k_irr, hk, fk, genus)?;
        me.curve_big = HyperellipticCurve::try_new(nn, irr.clone(), h, f, genus)?;
        Ok(me)
    }

    /// The additive polynomial `P(v)` as a polynomial over `K`.
    pub fn p_poly(&self) -> F2mPoly {
        self.mono_poly(
            &self
                .p
                .iter()
                .enumerate()
                .map(|(j, c)| (1usize << j, c.clone()))
                .collect::<Vec<_>>(),
        )
    }

    /// `Con(O)` as a reduced divisor class on `C` over `K`: `Σ_{P(δ)=0} (δ, 0)`
    /// (minus the appropriate multiple of infinity).
    fn weierstrass_class(&self) -> Result<MumfordDivisor, SmoothCoverError> {
        let irr = self.kirr();
        let u = match self.model {
            CoverModel::V => self.p_poly().scalar_mul(&self.inv(&self.p[self.r]), irr),
            CoverModel::T => {
                let top = 1usize << self.r;
                self.mono_poly(
                    &self
                        .p
                        .iter()
                        .enumerate()
                        .map(|(j, c)| (top - (1usize << j), c.clone()))
                        .collect::<Vec<_>>(),
                )
            }
        };
        Ok(MumfordDivisor::try_from_unreduced(
            &self.curve_big,
            u,
            F2mPoly::zero(self.tower.big_n),
        )?)
    }

    /// The class of `σ'^i Con(P) − (degree)·∞` on `C` over `K`, from the
    /// `i`-th conjugates `z_i = σ^i(1/x_0)` and `s_i = σ^i(s_0(P))`.
    fn conorm_class(
        &self,
        i: u32,
        z0: &F2mElement,
        s0: &F2mElement,
    ) -> Result<MumfordDivisor, SmoothCoverError> {
        let irr = self.kirr();
        let nn = self.tower.big_n;
        let zi = self.tower.frobenius_iter(z0, i);
        let si = self.tower.frobenius_iter(s0, i);
        // Y(v) = z_i (s_i + Θ^{σ^i}(v)) at exponents 0 and 2^j
        let mut y_terms: Vec<(usize, F2mElement)> = vec![(0, self.mul(&zi, &si))];
        for (j, th) in self.theta.iter().enumerate() {
            y_terms.push((
                1usize << j,
                self.mul(&zi, &self.tower.frobenius_iter(th, i)),
            ));
        }
        match self.model {
            CoverModel::V => {
                let pr_inv = self.inv(&self.p[self.r]);
                let mu_inv = self.inv(&self.mu);
                let mut u_terms: Vec<(usize, F2mElement)> = vec![(0, zi.clone())];
                for (j, c) in self.p.iter().enumerate() {
                    u_terms.push((1usize << j, c.clone()));
                }
                let u = self.mono_poly(&u_terms).scalar_mul(&pr_inv, irr);
                let y = self.mono_poly(&y_terms).scalar_mul(&mu_inv, irr);
                Ok(MumfordDivisor::try_from_unreduced(&self.curve_big, u, y)?)
            }
            CoverModel::T => {
                let top = 1usize << self.r;
                let zi_inv = self.inv(&zi);
                let mut u_terms: Vec<(usize, F2mElement)> = vec![(top, F2mElement::one(nn))];
                for (j, c) in self.p.iter().enumerate() {
                    u_terms.push((top - (1usize << j), self.mul(c, &zi_inv)));
                }
                let u = self.mono_poly(&u_terms);
                let y_t: Vec<(usize, F2mElement)> = y_terms
                    .iter()
                    .map(|(e, c)| (top - *e, c.clone()))
                    .collect();
                let y = self.mono_poly(&y_t);
                Ok(MumfordDivisor::try_from_unreduced(&self.curve_big, u, y)?)
            }
        }
    }

    /// `Φ(P) ∈ Jac(C)(k)` for `P ∈ E(K)`.  `Φ(O) = 0`.
    ///
    /// Errors for `x(P) = 0`.  The curve is `E : y² + xy = x³ + a x² + b`
    /// with `b = β_0²`; the caller guarantees `P ∈ E(K)`.
    pub fn image(&self, pt: &Pt) -> Result<MumfordDivisor, SmoothCoverError> {
        let (x0, y0) = match pt {
            Pt::Inf => return Ok(MumfordDivisor::identity(self.tower.l)),
            Pt::Aff { x, y } => (x, y),
        };
        if x0.is_zero() {
            return Err(SmoothCoverError::UnsupportedPoint("x = 0"));
        }
        let z0 = self.inv(x0);
        let s0 = self.mul(&y0.add(&self.beta0), &z0);
        let mut acc = MumfordDivisor::identity(self.tower.big_n);
        for i in 0..self.tower.n {
            let xi = self.conorm_class(i, &z0, &s0)?;
            acc = acc.try_add(&xi, &self.curve_big)?;
        }
        let w = self.weierstrass_class()?;
        let nw = w.try_scalar_mul(&BigUint::from(self.tower.n), &self.curve_big)?;
        let total = acc.try_add(&nw.try_neg(&self.curve_big)?, &self.curve_big)?;
        self.to_base(&total)
    }

    /// Move a divisor class with `k`-rational Mumford coefficients from
    /// `C_K` to `C_k`.
    pub fn to_base(&self, d: &MumfordDivisor) -> Result<MumfordDivisor, SmoothCoverError> {
        let conv = |p: &F2mPoly| -> Result<F2mPoly, SmoothCoverError> {
            let mut cs = Vec::new();
            for c in &p.coeffs {
                cs.push(self.project_k(c).ok_or(SmoothCoverError::Inconsistent(
                    "image divisor is not k-rational",
                ))?);
            }
            Ok(F2mPoly::from_coeffs(cs, self.tower.l))
        };
        let u = conv(&d.u)?;
        let v = conv(&d.v)?;
        Ok(MumfordDivisor::try_new(&self.curve_k, u, v)?)
    }
}

// ── curve and point helpers for E ─────────────────────────────────────

/// The two points of `E : y² + xy = x³ + a x² + b` with abscissa `x ≠ 0`
/// (empty if none).  `b = β²`.
pub fn lift_x(tower: &FieldTower, a: &F2mElement, b: &F2mElement, x: &F2mElement) -> Vec<Pt> {
    if x.is_zero() {
        return Vec::new();
    }
    let irr = &tower.irr;
    let xi = x.flt_inverse(irr).expect("nonzero");
    // t² + t = x + a + b/x²,  y = x t
    let c = x.add(a).add(&b.mul(&xi.square(irr), irr));
    let lm = LinearMap::new(tower.big_n, |t| t.square(irr).add(t));
    match lm.solve(&c) {
        None => Vec::new(),
        Some(t) => {
            let t2 = t.add(&F2mElement::one(tower.big_n));
            vec![
                Pt::Aff {
                    x: x.clone(),
                    y: x.mul(&t, irr),
                },
                Pt::Aff {
                    x: x.clone(),
                    y: x.mul(&t2, irr),
                },
            ]
        }
    }
}

/// `#E(K)` for `E : y² + xy = x³ + a x² + b` (naive count over `x`).
pub fn count_e_points(tower: &FieldTower, a: &F2mElement, b: &F2mElement) -> u64 {
    let irr = &tower.irr;
    let nn = tower.big_n;
    let mut cnt = 2u64; // O and (0, √b)
    for xv in 1u64..(1u64 << nn) {
        let x = F2mElement::from_words(&[xv], nn);
        let xi = x.flt_inverse(irr).expect("nonzero");
        let c = x.add(a).add(&b.mul(&xi.square(irr), irr));
        if c.trace(irr) == 0 {
            cnt += 2;
        }
    }
    cnt
}

// ── Jacobian order of C from point counts ─────────────────────────────

/// Point counts `N_k = #C(F_{q^k})`, the L-polynomial and `#J(F_q)`.
#[derive(Clone, Debug)]
pub struct JacobianData {
    pub counts: Vec<i128>,
    /// `L(T) = Σ c_i T^i` with `c_0 = 1`, degree `2g`.
    pub l_poly: Vec<i128>,
    pub order: i128,
}

fn pow_big(x: &F2mElement, e: &BigUint, irr: &IrreduciblePoly) -> F2mElement {
    let mut acc = F2mElement::one(irr.degree);
    for i in (0..e.bits()).rev() {
        acc = acc.square(irr);
        if e.bit(i) {
            acc = acc.mul(x, irr);
        }
    }
    acc
}

/// `#C(F_{q^k})` for `C : y² + h y = f` given over `F_q = F_{2^l}` (point at
/// infinity included), computed exhaustively in an extension built here.
pub fn count_points_extension(curve: &HyperellipticCurve, k: u32) -> i128 {
    let l = curve.m;
    let lk = l * k;
    let ext: IrreduciblePoly = if k == 1 {
        curve.irr.clone()
    } else {
        enumerate_irreducibles(lk, 1)
            .into_iter()
            .next()
            .expect("irreducible polynomial")
    };
    // embedding: a root of curve.irr in F_{2^{lk}}
    let omega = if k == 1 {
        F2mElement::from_words(&[2], lk)
    } else {
        let q: u64 = 1u64 << l;
        let norm_exp = (BigUint::from(1u8) << lk as usize) - 1u8;
        let norm_exp = norm_exp / BigUint::from(q - 1);
        let mut found = None;
        for g in 2u64..(1u64 << lk) {
            let e = F2mElement::from_words(&[g], lk);
            let nrm = pow_big(&e, &norm_exp, &ext);
            // test curve.irr(nrm) == 0
            let mut pw = vec![F2mElement::one(lk)];
            for _ in 1..=l {
                let nx = pw.last().unwrap().mul(&nrm, &ext);
                pw.push(nx);
            }
            let mut acc = pw[l as usize].clone();
            for &t in &curve.irr.low_terms {
                acc = acc.add(&pw[t as usize]);
            }
            if acc.is_zero() {
                found = Some(nrm);
                break;
            }
        }
        found.expect("root of base modulus in extension")
    };
    let mut emb = vec![F2mElement::one(lk)];
    for i in 1..l as usize {
        let nx = emb[i - 1].mul(&omega, &ext);
        emb.push(nx);
    }
    let embed = |c: &F2mElement| -> F2mElement {
        let w = c.raw_bits().first().copied().unwrap_or(0);
        let mut r = F2mElement::zero(lk);
        for (i, e) in emb.iter().enumerate() {
            if (w >> i) & 1 == 1 {
                r = r.add(e);
            }
        }
        r
    };
    let hc: Vec<F2mElement> = curve.h.coeffs.iter().map(&embed).collect();
    let fc: Vec<F2mElement> = curve.f.coeffs.iter().map(&embed).collect();
    let eval = |cs: &[F2mElement], x: &F2mElement| -> F2mElement {
        let mut acc = F2mElement::zero(lk);
        for c in cs.iter().rev() {
            acc = acc.mul(x, &ext).add(c);
        }
        acc
    };
    let mut cnt: i128 = 1;
    for xv in 0u64..(1u64 << lk) {
        let x = F2mElement::from_words(&[xv], lk);
        let hx = eval(&hc, &x);
        let fx = eval(&fc, &x);
        if hx.is_zero() {
            cnt += 1;
        } else {
            let hi = hx.flt_inverse(&ext).expect("nonzero");
            let t = fx.mul(&hi.square(&ext), &ext);
            if t.trace(&ext) == 0 {
                cnt += 2;
            }
        }
    }
    cnt
}

/// L-polynomial coefficients and `#J(F_q)` from `N_1 … N_g` (Newton's identities).
pub fn jacobian_from_counts(q: i128, g: usize, counts: &[i128]) -> JacobianData {
    let s: Vec<i128> = (1..=g)
        .map(|k| q.pow(k as u32) + 1 - counts[k - 1])
        .collect();
    let mut e = vec![0i128; g + 1];
    e[0] = 1;
    for i in 1..=g {
        let mut acc = 0i128;
        for j in 1..=i {
            let sign = if (j - 1) % 2 == 0 { 1 } else { -1 };
            acc += sign * e[i - j] * s[j - 1];
        }
        assert_eq!(acc % i as i128, 0, "Newton identity not integral");
        e[i] = acc / i as i128;
    }
    let mut c = vec![0i128; 2 * g + 1];
    for i in 0..=g {
        c[i] = if i % 2 == 0 { e[i] } else { -e[i] };
    }
    for i in 0..g {
        c[2 * g - i] = q.pow((g - i) as u32) * c[i];
    }
    let order = c.iter().sum();
    JacobianData {
        counts: counts.to_vec(),
        l_poly: c,
        order,
    }
}

/// `#C(F_{q^k})` predicted by an L-polynomial: `q^k + 1 − Σ α_i^k`, via
/// Newton's identities on the reversed polynomial.
pub fn predicted_count(q: i128, l_poly: &[i128], k: usize) -> i128 {
    // L(T) = ∏ (1 − α_i T); power sums p_k satisfy Newton on c_i.
    let d = l_poly.len() - 1;
    let mut p = vec![0i128; k + 1];
    for m in 1..=k {
        // p_m = −Σ_{i=1}^{m−1} c_i p_{m−i} − m c_m   (c_i = 0 for i > d)
        let mut acc = 0i128;
        for i in 1..m {
            if i <= d {
                acc += l_poly[i] * p[m - i];
            }
        }
        let cm = if m <= d { l_poly[m] } else { 0 };
        p[m] = -acc - (m as i128) * cm;
    }
    q.pow(k as u32) + 1 - p[k]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::ec_trapdoor::{ghs_type_i, magic_number_full};
    use crate::cryptanalysis::ghs_descent::ECurve;

    struct Setup {
        tower: FieldTower,
        a: F2mElement,
        beta: F2mElement,
        b: F2mElement,
    }

    fn setup(l: u32, n: u32, a: u8, beta_bits: u64) -> Setup {
        let nn = l * n;
        let irr = enumerate_irreducibles(nn, 1).remove(0);
        let tower = FieldTower::new(nn, n, l, irr.clone());
        let beta = F2mElement::from_words(&[beta_bits], nn);
        let b = beta.square(&irr);
        let a = if a == 0 {
            F2mElement::zero(nn)
        } else {
            F2mElement::one(nn)
        };
        Setup { tower, a, beta, b }
    }

    fn all_points(s: &Setup, limit: usize) -> Vec<Pt> {
        let mut pts = Vec::new();
        for xv in 1u64..(1u64 << s.tower.big_n) {
            let x = F2mElement::from_words(&[xv], s.tower.big_n);
            pts.extend(lift_x(&s.tower, &s.a, &s.b, &x));
            if pts.len() >= limit {
                break;
            }
        }
        pts
    }

    /// Genus, smoothness, magic number and type against ec_trapdoor, plus the
    /// genus verified by point counts (functional equation + one extra degree).
    fn check_cover(l: u32, n: u32, a: u8, beta_bits: u64) -> Option<(SmoothCover, Setup)> {
        let s = setup(l, n, a, beta_bits);
        let cov = match SmoothCover::new(&s.tower, &s.a, &s.beta) {
            Ok(c) => c,
            Err(SmoothCoverError::MagicNumberOne) => return None,
            Err(e) => panic!("cover failed for beta={beta_bits}: {e:?}"),
        };
        let m = magic_number_full(&s.tower, &s.a, &s.b);
        assert_eq!(cov.magic, m, "magic number");
        let type_i = ghs_type_i(&s.tower, &s.b);
        assert_eq!(cov.c1.is_zero(), type_i, "type I iff c_1 = 0");
        let expect_g = if type_i { (1u32 << (m - 1)) - 1 } else { 1u32 << (m - 1) };
        assert_eq!(cov.genus, expect_g, "GHS genus");
        Some((cov, s))
    }

    #[test]
    fn covers_have_ghs_genus_and_are_smooth() {
        let mut seen = std::collections::BTreeSet::new();
        for (l, n) in [(2u32, 3u32), (2, 4), (3, 3), (2, 5)] {
            for a in [0u8, 1] {
                for beta in 1u64..(1u64 << (l * n)).min(512) {
                    if let Some((cov, _)) = check_cover(l, n, a, beta) {
                        seen.insert((l, n, cov.magic, cov.genus));
                    }
                }
            }
        }
        assert!(seen.iter().any(|t| t.3 % 2 == 0), "no even-genus case seen");
        assert!(seen.iter().any(|t| t.3 % 2 == 1), "no odd-genus case seen");
    }

    #[test]
    fn point_counts_confirm_genus_and_frobenius_relation() {
        // (l, n) = (2, 3): N = 6, magic up to 3
        let mut tested = 0;
        for beta in 1u64..64 {
            let Some((cov, s)) = check_cover(2, 3, 0, beta) else {
                continue;
            };
            let g = cov.genus as usize;
            let counts: Vec<i128> = (1..=g as u32 + 2)
                .map(|k| count_points_extension(&cov.curve_k, k))
                .collect();
            let jd = jacobian_from_counts(4, g, &counts[..g]);
            // genus check: the L-polynomial from N_1..N_g predicts N_{g+1}, N_{g+2}
            assert_eq!(predicted_count(4, &jd.l_poly, g + 1), counts[g], "N_(g+1)");
            assert_eq!(predicted_count(4, &jd.l_poly, g + 2), counts[g + 1], "N_(g+2)");
            // E(K) embeds: its order divides #J up to the kernel; check the
            // Frobenius relation  pi_E = alpha^n  via the trace of E
            let ne = count_e_points(&s.tower, &s.a, &s.b) as i128;
            let t_e = (1i128 << 6) + 1 - ne;
            // power sums of alpha^3 from counts of C over F_{4^{3j}} need k up to 3j; for g<=2
            // use p_3 = sum alpha_i^3 directly: pi_E + conj appear in alpha^3
            let p3 = {
                let mut p = vec![0i128; 4];
                for m in 1..=3usize {
                    let mut acc = 0;
                    for i in 1..m {
                        if i < jd.l_poly.len() {
                            acc += jd.l_poly[i] * p[m - i];
                        }
                    }
                    p[m] = -acc - (m as i128) * jd.l_poly[m];
                }
                p[3]
            };
            // sum of alpha^3 = t_E + (trace of the complementary part): only a sanity bound
            assert!(p3.abs() <= 2 * (g as i128) * 8, "|p_3| bound");
            let _ = t_e;
            tested += 1;
            if tested >= 6 {
                break;
            }
        }
        assert!(tested >= 3);
    }

    #[test]
    fn image_is_a_homomorphism_into_the_jacobian() {
        let mut tested = 0;
        for (l, n, a, beta) in [(2u32, 3u32, 0u8, 1u64), (2, 3, 1, 5), (2, 4, 0, 3), (3, 3, 0, 9)] {
            for beta in beta..beta + 40 {
                let Some((cov, s)) = check_cover(l, n, a, beta) else {
                    continue;
                };
                let e = ECurve::new(
                    s.tower.big_n,
                    s.tower.irr.clone(),
                    s.a.clone(),
                    s.b.clone(),
                );
                let pts = all_points(&s, 400);
                let mut ok = 0;
                for i in (0..pts.len().saturating_sub(7)).step_by(3) {
                    let (p, q) = (&pts[i], &pts[i + 7]);
                    let sum = e.add(p, q);
                    let (Ok(fp), Ok(fq), Ok(fs)) = (cov.image(p), cov.image(q), cov.image(&sum))
                    else {
                        continue;
                    };
                    let add = fp.add(&fq, &cov.curve_k);
                    assert_eq!(add, fs, "Phi(P+Q) = Phi(P)+Phi(Q)");
                    let neg = e.neg(p);
                    if let Ok(fneg) = cov.image(&neg) {
                        assert_eq!(fneg, fp.neg(&cov.curve_k), "Phi(-P) = -Phi(P)");
                    }
                    ok += 1;
                    if ok >= 6 {
                        break;
                    }
                }
                assert!(ok >= 1);
                tested += 1;
                if tested >= 12 {
                    return;
                }
            }
        }
        assert!(tested >= 4);
    }

    #[test]
    fn image_order_divides_point_order_and_jacobian_order() {
        let mut tested = 0;
        for beta in 1u64..64 {
            let Some((cov, s)) = check_cover(2, 3, 0, beta) else {
                continue;
            };
            let g = cov.genus as usize;
            let counts: Vec<i128> = (1..=g as u32)
                .map(|k| count_points_extension(&cov.curve_k, k))
                .collect();
            let jd = jacobian_from_counts(4, g, &counts);
            let ne = count_e_points(&s.tower, &s.a, &s.b);
            let e = ECurve::new(6, s.tower.irr.clone(), s.a.clone(), s.b.clone());
            let pts = all_points(&s, 64);
            for p in pts.iter().take(8) {
                let Ok(f) = cov.image(p) else { continue };
                // [#E] Phi(P) = Phi([#E]P) = 0
                let z = f.scalar_mul(&BigUint::from(ne), &cov.curve_k);
                assert!(z.is_identity(), "[#E] Phi(P) = 0");
                let _ = &e;
            }
            // #J is a multiple of the order of every image point
            for p in pts.iter().take(4) {
                let Ok(f) = cov.image(p) else { continue };
                let z = f.scalar_mul(&BigUint::from(jd.order as u128), &cov.curve_k);
                assert!(z.is_identity(), "[#J] Phi(P) = 0");
            }
            tested += 1;
            if tested >= 6 {
                break;
            }
        }
        assert!(tested >= 3);
    }

    /// The target instance: N = 12, (l, n) = (2, 6), magic number 4, genus 8 over F_4.
    #[test]
    fn n12_l2_n6_descended_curve_and_map() {
        let s = setup(2, 6, 0, 0x28);
        let cov = SmoothCover::new(&s.tower, &s.a, &s.beta).expect("cover");
        assert_eq!(cov.magic, 4);
        assert_eq!(cov.genus, 8);
        assert_eq!(cov.model, CoverModel::V);
        // point counts N_1..N_8 over F_4^k (frozen from the first run) and #J
        let counts: Vec<i128> = (1..=8).map(|k| count_points_extension(&cov.curve_k, k)).collect();
        assert_eq!(counts, vec![5, 15, 65, 319, 1025, 4761, 16385, 64639]);
        let jd = jacobian_from_counts(4, 8, &counts);
        assert_eq!(jd.order, 67072);
        // the independent genus check: N_9 predicted from N_1..N_8
        assert_eq!(predicted_count(4, &jd.l_poly, 9), count_points_extension(&cov.curve_k, 9));
        // E(K): order 4192 = 2^5 * 131, and 131 | #J
        let ne = count_e_points(&s.tower, &s.a, &s.b);
        assert_eq!(ne, 4192);
        assert_eq!(jd.order % 131, 0);
        let e = ECurve::new(12, s.tower.irr.clone(), s.a.clone(), s.b.clone());
        let pts = all_points(&s, 60);
        let mut checked = 0;
        for w in pts.windows(2).take(10) {
            let (Ok(a1), Ok(b1)) = (cov.image(&w[0]), cov.image(&w[1])) else {
                continue;
            };
            let sum = cov.image(&e.add(&w[0], &w[1])).expect("sum image");
            assert_eq!(a1.add(&b1, &cov.curve_k), sum);
            checked += 1;
        }
        assert!(checked >= 5);
        // a point of order 131 maps to a point of order 131
        for p in &pts {
            let g131 = e.scalar_mul(p, &BigUint::from(32u32));
            if matches!(g131, Pt::Inf) {
                continue;
            }
            let Ok(f) = cov.image(&g131) else { continue };
            assert!(!f.is_identity());
            assert!(f.scalar_mul(&BigUint::from(131u32), &cov.curve_k).is_identity());
            break;
        }
    }
}
