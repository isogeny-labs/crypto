//! # F4 over a mixed-radix tower quotient ring
//!
//! [`f4_fp_tower`](super::f4_fp_tower) handles degree-2 tower steps only: its
//! monomials are square-free bitmasks. This module is the same algorithm over
//! the quotient
//!
//! ```text
//!     R = F_p[y_0, …, y_{n−1}, u_0, …] / (y_j^{r_j} − rule_j),   r_j ∈ {2, 3, 4}
//! ```
//!
//! so that a tower step of radix 3 (`y_j³ = y_{j+1}`, the *cube rule*), a
//! mixed tower (`2, 2, 3`) and the radix-4 control (`y_j⁴ = y_{j+1}`) can be
//! measured by the same engine. The design, the invariants and the tests are
//! in `docs/perf/20261114-f4-tower-cuberule.md`. This is an engine extension,
//! not a performance change: the radix-2 engine is untouched (its items are
//! only made `pub(crate)` so that its linear algebra is shared), and a pure
//! radix-2 ring run here gives the radix-2 engine's trace step for step
//! (tested).
//!
//! ## What changes against the radix-2 engine
//!
//! - **Monomials.** A tower variable carries an exponent in `0..r_j`, two
//!   bits wide. A key is one `u128`, grevlex with variable 0 largest, like
//!   the radix-2 key but with a different field layout:
//!   `[degree: 16 bits][free: 4 × 8 bits][tower: 40 × 2 bits]`. The tower
//!   field and each free byte hold the *complement* of the exponent, so that
//!   a smaller exponent of a later variable is a larger key. Degree is the
//!   exponent sum.
//! - **Rules.** A [`PowRule`] rewrites `y_j^{r} = Σ_{k<r} (c_k + b_k·y_next)·y_j^k`.
//!   With `r = 2` it is a `SquareRule` (`c = [d, c]`, `b = [b, a]`); with
//!   `r = 3` it is the cube rule. Its leading monomial in grevlex is
//!   `y_j^r`, so the rules are a Gröbner basis (pairwise coprime leads).
//! - **Products** carry on overflow. `y_j · m` with `e_j(m) = r_j − 1`
//!   replaces `y_j^{r_j}` by the rule's right-hand side, which can push
//!   `y_next` over its own limit and cascade up the tower.
//! - **Tower pairs.** The S-polynomial of `g` against `y_v^r − …`, for every
//!   tower variable `v` with `e_v(LM g) ≥ 1`, is `NF(y_v^{r_v − e_v} · g)` at
//!   degree `deg LM(g) + r_v − e_v`. For `r = 2` this is `NF(y_v · g)` at
//!   `deg + 1`, as before.
//!
//! Everything else (Gebauer–Möller on the critical pairs, symbolic
//! preprocessing, Faugère–Lachartre elimination, full-rank exit, the dense
//! `B'` block, the lazy accumulator and its SIMD kernels) is the radix-2
//! engine's code, shared through its `pub(crate)` items.
//!
//! ## Limits
//!
//! At most 40 tower variables, 4 free variables with exponents below 256,
//! `p < 2³²`, total degrees below `2¹⁶`. Not supported here (use the radix-2
//! engine): weighted gradings, F4 trace learning and replay, the signature
//! engine, the generic-field engine.

use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::sync::Arc;
use std::time::Instant;

use rayon::prelude::*;

use super::f4_fp_tower::{
    echelon, eliminate, number_of, order_columns, Echelon, Elem, ElimStats, Fp, MonoHasher,
    MonoMap, ProductCache, RPoly, Row, RowDesc, SquareRule, StepTrace, Stop, TowerF4Options,
    TowerRing, NONE, PRODUCT_CACHE_TERMS,
};

/// A monomial key; see the module notes for the layout.
pub type XMono = u128;

/// Tower variables a key can carry.
pub const MAX_XTOWER: usize = 40;
/// Free variables a key can carry.
pub const MAX_XFREE: usize = 4;
/// The largest radix of a tower step.
pub const MAX_RADIX: u8 = 4;

const TW_BITS: u32 = 2 * MAX_XTOWER as u32; // 80
const TW_MASK: u128 = (1u128 << TW_BITS) - 1;
/// The low bit of every two-bit lane.
const LO: u128 = 0x5555_5555_5555_5555_5555;
const FREE_SHIFT: u32 = TW_BITS; // 80
const DEG_SHIFT: u32 = 112;
const MAX_DEGREE: u32 = (1 << 16) - 1;

/// The monomial `1`.
pub const X_ONE: XMono = (0xffff_ffffu128 << FREE_SHIFT) | TW_MASK;

// ── Keys ─────────────────────────────────────────────────────────────

/// The exponents of the tower variables of `m`: lane `j`, two bits at bit
/// `2j`.
#[inline]
pub fn x_tw(m: XMono) -> u128 {
    !m & TW_MASK
}

/// The free exponents of `m`.
#[inline]
pub fn x_free(m: XMono) -> [u8; MAX_XFREE] {
    let mut f = [0u8; MAX_XFREE];
    for (k, e) in f.iter_mut().enumerate() {
        *e = 255 - ((m >> (FREE_SHIFT + 8 * k as u32)) & 0xff) as u8;
    }
    f
}

/// Total degree of `m`: the exponent sum.
#[inline]
pub fn x_degree(m: XMono) -> u32 {
    (m >> DEG_SHIFT) as u32
}

/// The sum of the lanes of `tw`.
#[inline]
fn lane_sum(tw: u128) -> u32 {
    (tw & LO).count_ones() + 2 * ((tw >> 1) & LO).count_ones()
}

/// The key of tower exponents `tw` and free exponents `free`.
#[inline]
pub fn x_mono(tw: u128, free: [u8; MAX_XFREE]) -> XMono {
    let deg = lane_sum(tw) + free.iter().map(|&e| u32::from(e)).sum::<u32>();
    assert!(
        deg <= MAX_DEGREE,
        "f4_fp_tower_radix: degree above 2^16 − 1"
    );
    x_mono_with_degree(deg, tw, free)
}

#[inline]
fn x_mono_with_degree(deg: u32, tw: u128, free: [u8; MAX_XFREE]) -> XMono {
    let mut key = (u128::from(deg) << DEG_SHIFT) | (!tw & TW_MASK);
    for (k, &e) in free.iter().enumerate() {
        key |= u128::from(255 - e) << (FREE_SHIFT + 8 * k as u32);
    }
    key
}

/// The lanes where `a > b`, as bit `2j`.
#[inline]
fn lanes_gt(a: u128, b: u128) -> u128 {
    let (a1, a0, b1, b0) = ((a >> 1) & LO, a & LO, (b >> 1) & LO, b & LO);
    ((a1 & !b1) | (!(a1 ^ b1) & a0 & !b0)) & LO
}

/// The lanes of `tw` that are non-zero, as bit `2j`.
#[inline]
fn lanes_nz(tw: u128) -> u128 {
    (tw | (tw >> 1)) & LO
}

/// The set bits of a lane mask, as lane indices, ascending.
#[inline]
fn each_lane(mut nz: u128, mut f: impl FnMut(usize)) {
    while nz != 0 {
        f((nz.trailing_zeros() / 2) as usize);
        nz &= nz - 1;
    }
}

#[inline]
fn lane(tw: u128, j: usize) -> u32 {
    ((tw >> (2 * j)) & 3) as u32
}

#[inline]
fn add_free(a: [u8; MAX_XFREE], b: [u8; MAX_XFREE]) -> [u8; MAX_XFREE] {
    let mut f = [0u8; MAX_XFREE];
    for k in 0..MAX_XFREE {
        f[k] = a[k]
            .checked_add(b[k])
            .expect("f4_fp_tower_radix: a free exponent passed 255");
    }
    f
}

/// `a | b` in the polynomial ring.
#[inline]
pub(crate) fn x_divides(a: XMono, b: XMono) -> bool {
    if x_degree(a) > x_degree(b) || lanes_gt(x_tw(a), x_tw(b)) != 0 {
        return false;
    }
    let (fa, fb) = (x_free(a), x_free(b));
    fa.iter().zip(&fb).all(|(x, y)| x <= y)
}

#[inline]
pub(crate) fn x_lcm(a: XMono, b: XMono) -> XMono {
    let (ta, tb) = (x_tw(a), x_tw(b));
    let g = lanes_gt(ta, tb);
    let m = g | (g << 1);
    let (fa, fb) = (x_free(a), x_free(b));
    let mut f = [0u8; MAX_XFREE];
    for k in 0..MAX_XFREE {
        f[k] = fa[k].max(fb[k]);
    }
    x_mono((ta & m) | (tb & !m & TW_MASK), f)
}

/// `a / b`, for `b | a`.
#[inline]
pub(crate) fn x_quotient(a: XMono, b: XMono) -> XMono {
    let (fa, fb) = (x_free(a), x_free(b));
    let mut f = [0u8; MAX_XFREE];
    for k in 0..MAX_XFREE {
        f[k] = fa[k] - fb[k];
    }
    // Each lane of `a` is at least that of `b`: the subtraction borrows
    // nothing.
    x_mono_with_degree(x_degree(a) - x_degree(b), x_tw(a) - x_tw(b), f)
}

#[inline]
pub(crate) fn x_coprime(a: XMono, b: XMono) -> bool {
    if lanes_nz(x_tw(a)) & lanes_nz(x_tw(b)) != 0 {
        return false;
    }
    let (fa, fb) = (x_free(a), x_free(b));
    fa.iter().zip(&fb).all(|(x, y)| *x == 0 || *y == 0)
}

/// Decodes a key into one exponent per tower variable of `n_tower`, then
/// the free exponents.
pub fn x_exponents(m: XMono, n_tower: usize, n_free: usize) -> Vec<u32> {
    let tw = x_tw(m);
    let mut e: Vec<u32> = (0..n_tower).map(|j| lane(tw, j)).collect();
    e.extend(x_free(m).iter().take(n_free).map(|&x| u32::from(x)));
    e
}

/// The key of an exponent vector: tower exponents `< 4` first, then free.
pub fn x_from_exponents(e: &[u32], n_tower: usize) -> XMono {
    let mut tw = 0u128;
    for (j, &x) in e.iter().take(n_tower).enumerate() {
        assert!(x < 4, "f4_fp_tower_radix: a tower exponent is 0..=3");
        tw |= u128::from(x) << (2 * j);
    }
    let mut f = [0u8; MAX_XFREE];
    for (k, &x) in e.iter().skip(n_tower).enumerate() {
        f[k] = u8::try_from(x).expect("f4_fp_tower_radix: free exponent above 255");
    }
    x_mono(tw, f)
}

// ── Polynomials ──────────────────────────────────────────────────────

/// An element of `R` in normal form: keys strictly descending, coefficients
/// non-zero and below `p`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct XPoly {
    pub monos: Vec<XMono>,
    pub coefs: Vec<u32>,
}

impl XPoly {
    /// Sort, merge repeats and drop zeros.
    pub(crate) fn from_terms(fp: Fp, mut terms: Vec<(XMono, u64)>) -> XPoly {
        terms.sort_unstable_by_key(|t| std::cmp::Reverse(t.0));
        let mut monos = Vec::with_capacity(terms.len());
        let mut coefs: Vec<u32> = Vec::with_capacity(terms.len());
        let mut i = 0;
        while i < terms.len() {
            let m = terms[i].0;
            let mut c = 0u64;
            while i < terms.len() && terms[i].0 == m {
                c = fp.add(c, fp.reduce(terms[i].1));
                i += 1;
            }
            if c != 0 {
                monos.push(m);
                coefs.push(c as u32);
            }
        }
        XPoly { monos, coefs }
    }

    pub fn len(&self) -> usize {
        self.monos.len()
    }

    pub fn is_empty(&self) -> bool {
        self.monos.is_empty()
    }

    /// Leading monomial.
    pub fn lm(&self) -> Option<XMono> {
        self.monos.first().copied()
    }

    pub fn degree(&self) -> u32 {
        self.monos.iter().map(|&m| x_degree(m)).max().unwrap_or(0)
    }

    /// Whether this is a non-zero constant.
    pub fn is_unit(&self) -> bool {
        self.monos.len() == 1 && self.monos[0] == X_ONE
    }

    /// The terms as a container the shared basis element type holds.
    fn into_rpoly(self) -> RPoly {
        RPoly {
            monos: self.monos,
            coefs: self.coefs,
        }
    }
}

fn xpoly_of(e: &Elem) -> XPoly {
    let (monos, coefs) = e.terms().unzip();
    XPoly { monos, coefs }
}

// ── The ring ─────────────────────────────────────────────────────────

/// How one tower variable's power rewrites:
///
/// ```text
///     y_j^radix = Σ_{k < radix} (c[k] + b[k]·y_next) · y_j^k
/// ```
///
/// with coefficients in `[0, p)`. On a tower's top level `next` is `None`
/// and every `b[k]` is 0. Radix 2 is the radix-2 engine's `SquareRule`
/// (`c = [d, c]`, `b = [b, a]`); radix 3 is the cube rule, built by
/// [`PowRule::cube`]; the Kummer step `y^radix = y_next` by
/// [`PowRule::kummer`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PowRule {
    pub radix: u8,
    pub next: Option<u8>,
    pub c: [u64; 4],
    pub b: [u64; 4],
}

/// The cube rule `y_j³ = y_next·(b₀ + b₁·y_j + b₂·y_j²) + c₀ + c₁·y_j + c₂·y_j²`
/// is a [`PowRule`] of radix 3; this names it.
pub type CubeRule = PowRule;

impl PowRule {
    /// `y^radix = y_next` (`next = Some`), or `y^radix = 1` on a top level.
    pub fn kummer(radix: u8, next: Option<u8>) -> Self {
        let mut r = PowRule {
            radix,
            next,
            c: [0; 4],
            b: [0; 4],
        };
        match next {
            Some(_) => r.b[0] = 1,
            None => r.c[0] = 1,
        }
        r
    }

    /// The cube rule `y_j³ = c₀ + c₁·y_j + c₂·y_j² + y_next·(b₀ + b₁·y_j + b₂·y_j²)`.
    pub fn cube(next: Option<u8>, c: [u64; 3], b: [u64; 3]) -> Self {
        PowRule {
            radix: 3,
            next,
            c: [c[0], c[1], c[2], 0],
            b: [b[0], b[1], b[2], 0],
        }
    }

    /// The radix-2 rule of a `SquareRule`.
    pub fn from_square(r: &SquareRule) -> Self {
        PowRule {
            radix: 2,
            next: r.next,
            c: [r.d, r.c, 0, 0],
            b: [r.b, r.a, 0, 0],
        }
    }
}

/// `F_p[y_0, …, y_{n−1}, u_0, …] / (y_j^{r_j} − rule_j)`. The tower
/// variables come first; the free variables follow, at global indices
/// `n_tower .. n_tower + n_free`.
#[derive(Clone, Debug)]
pub struct RadixRing {
    pub p: u64,
    /// One rule per tower variable.
    pub rules: Vec<PowRule>,
    pub n_free: usize,
}

/// The two working lists of [`RadixRing::exp_product_into`].
#[derive(Default)]
struct Scratch {
    cur: Vec<(u128, u64)>,
    next: Vec<(u128, u64)>,
}

/// Combine repeated exponent vectors and drop zero coefficients, in place.
fn merge_exps(fp: Fp, terms: &mut Vec<(u128, u64)>) {
    if terms.len() < 2 {
        return;
    }
    terms.sort_unstable_by_key(|t| t.0);
    let mut out = 0usize;
    let mut i = 0usize;
    while i < terms.len() {
        let m = terms[i].0;
        let mut c = 0u64;
        while i < terms.len() && terms[i].0 == m {
            c = fp.add(c, terms[i].1);
            i += 1;
        }
        if c != 0 {
            terms[out] = (m, c);
            out += 1;
        }
    }
    terms.truncate(out);
}

impl RadixRing {
    /// The radix-2 ring of a [`TowerRing`], rule for rule.
    pub fn from_square_ring(r: &TowerRing) -> Self {
        RadixRing {
            p: r.p,
            rules: r.rules.iter().map(PowRule::from_square).collect(),
            n_free: r.n_free,
        }
    }

    pub fn n_tower(&self) -> usize {
        self.rules.len()
    }

    pub fn n_vars(&self) -> usize {
        self.rules.len() + self.n_free
    }

    /// The number of standard monomials of the tower alone: `Π r_j`.
    pub fn tower_size(&self) -> u128 {
        self.rules.iter().map(|r| u128::from(r.radix)).product()
    }

    /// Panics unless the rules describe a tower this engine handles: radix
    /// 2 to 4, every `next` a later tower variable (so each rewrite lowers
    /// the monomial in grevlex, and `y_j^r` is the leading monomial), and a
    /// top rule with no `y_next` term.
    pub fn check(&self) {
        assert!(
            self.rules.len() <= MAX_XTOWER,
            "f4_fp_tower_radix: at most {MAX_XTOWER} tower variables"
        );
        assert!(
            self.n_free <= MAX_XFREE,
            "f4_fp_tower_radix: at most {MAX_XFREE} free variables"
        );
        for (j, r) in self.rules.iter().enumerate() {
            assert!(
                (2..=MAX_RADIX).contains(&r.radix),
                "f4_fp_tower_radix: rule {j} has radix {} (2..=4 supported)",
                r.radix
            );
            for k in 0..4 {
                for &x in &[r.c[k], r.b[k]] {
                    assert!(
                        x < self.p,
                        "f4_fp_tower_radix: rule {j} has a coefficient ≥ p"
                    );
                    assert!(
                        k < r.radix as usize || x == 0,
                        "f4_fp_tower_radix: rule {j} has a term above y^(radix−1)"
                    );
                }
            }
            match r.next {
                Some(nx) => assert!(
                    (nx as usize) > j && (nx as usize) < self.rules.len(),
                    "f4_fp_tower_radix: rule {j} must point at a later tower variable"
                ),
                None => assert!(
                    r.b.iter().all(|&x| x == 0),
                    "f4_fp_tower_radix: top rule {j} cannot use a next level"
                ),
            }
        }
    }

    /// The lanes' limits: lane `j` holds `r_j − 1`.
    fn cap(&self) -> u128 {
        self.rules.iter().enumerate().fold(0u128, |acc, (j, r)| {
            acc | (u128::from(r.radix - 1) << (2 * j))
        })
    }

    /// `y_j · (e, coef)` in normal form, pushed onto `out` as tower
    /// exponent vectors with coefficients (repeats allowed).
    fn mul_var_into(&self, fp: Fp, e: u128, j: usize, coef: u64, out: &mut Vec<(u128, u64)>) {
        let sh = 2 * j;
        let r = &self.rules[j];
        let ej = (e >> sh) & 3;
        if ej + 1 < u128::from(r.radix) {
            out.push((e + (1u128 << sh), coef));
            return;
        }
        // y_j^radix: lane j empties and the rule's right-hand side comes in.
        let e0 = e & !(3u128 << sh);
        for k in 0..r.radix as usize {
            if r.c[k] != 0 {
                out.push((e0 | ((k as u128) << sh), fp.mul(coef, r.c[k])));
            }
        }
        if let Some(nx) = r.next {
            for k in 0..r.radix as usize {
                if r.b[k] != 0 {
                    self.mul_var_into(
                        fp,
                        e0 | ((k as u128) << sh),
                        nx as usize,
                        fp.mul(coef, r.b[k]),
                        out,
                    );
                }
            }
        }
    }

    /// `coef · y^ea · y^eb` in normal form (tower parts only), pushed onto
    /// `out` with repeats allowed.
    fn exp_product_into(
        &self,
        fp: Fp,
        cap: u128,
        ea: u128,
        eb: u128,
        coef: u64,
        out: &mut Vec<(u128, u64)>,
        scratch: &mut Scratch,
    ) {
        let over = lanes_gt(eb, cap - ea);
        if over == 0 {
            out.push((ea + eb, coef));
            return;
        }
        // The lanes that stay within their limit are added outright; the
        // others are multiplied in one variable at a time, merging after
        // each so that the list does not grow exponentially.
        let keep = over | (over << 1);
        let Scratch { cur, next } = scratch;
        cur.clear();
        cur.push((ea + (eb & !keep & TW_MASK), coef));
        each_lane(over, |j| {
            for _ in 0..lane(eb, j) {
                next.clear();
                for &(m, c) in cur.iter() {
                    self.mul_var_into(fp, m, j, c, next);
                }
                merge_exps(fp, next);
                std::mem::swap(cur, next);
            }
        });
        out.extend_from_slice(cur);
    }

    /// `u · g` in normal form.
    pub fn mul_mono(&self, g: &XPoly, u: XMono) -> XPoly {
        let fp = Fp::new(self.p);
        let terms = g.monos.iter().copied().zip(g.coefs.iter().copied());
        self.mul_terms(fp, terms, g.len(), u)
    }

    fn mul_elem(&self, fp: Fp, g: &Elem, u: XMono) -> XPoly {
        self.mul_terms(fp, g.terms(), g.len(), u)
    }

    /// `u · g` in normal form, for `g` given by its terms, descending and
    /// with non-zero coefficients (`len` of them).
    pub(crate) fn mul_terms(
        &self,
        fp: Fp,
        g: impl Iterator<Item = (XMono, u32)>,
        len: usize,
        u: XMono,
    ) -> XPoly {
        let (tu, fu) = (x_tw(u), x_free(u));
        let has_free = fu != [0; MAX_XFREE];
        let cap = self.cap();
        // A term whose tower lanes absorb `u`'s without overflow is a plain
        // product: its key is the term's key plus a constant (degree up,
        // the complemented lanes and free bytes down by `u`'s, nothing
        // borrows), so those terms stay distinct and descending and go
        // straight into the result. Only the terms that rewrite (`carried`)
        // come out in no order, and only they are sorted.
        let mut shift = (u128::from(x_degree(u)) << DEG_SHIFT) - tu;
        for (k, &e) in fu.iter().enumerate() {
            shift -= u128::from(e) << (FREE_SHIFT + 8 * k as u32);
        }
        let mut monos: Vec<XMono> = Vec::with_capacity(len);
        let mut coefs: Vec<u32> = Vec::with_capacity(len);
        let mut carried: Vec<(XMono, u64)> = Vec::new();
        let mut tmp: Vec<(u128, u64)> = Vec::new();
        let mut scratch = Scratch::default();
        for (m, c) in g {
            let tm = x_tw(m);
            if has_free {
                // Panics on a free exponent past 255, as the radix-2 engine.
                add_free(x_free(m), fu);
            }
            if lanes_gt(tu, cap - tm) == 0 {
                monos.push(m + shift);
                coefs.push(c);
            } else {
                let f = add_free(x_free(m), fu);
                tmp.clear();
                self.exp_product_into(fp, cap, tm, tu, u64::from(c), &mut tmp, &mut scratch);
                carried.extend(tmp.iter().map(|&(x, cc)| (x_mono(x, f), cc)));
            }
        }
        if carried.is_empty() {
            return XPoly { monos, coefs };
        }
        carried.sort_unstable_by_key(|t| std::cmp::Reverse(t.0));
        // Merge the two descending lists, adding the coefficients of a
        // repeated monomial and dropping a zero sum.
        let mut out = XPoly {
            monos: Vec::with_capacity(monos.len() + carried.len()),
            coefs: Vec::with_capacity(monos.len() + carried.len()),
        };
        let (mut i, mut j) = (0, 0);
        loop {
            let m = match (monos.get(i), carried.get(j)) {
                (Some(&a), Some(&(b, _))) => a.max(b),
                (Some(&a), None) => a,
                (None, Some(&(b, _))) => b,
                (None, None) => break,
            };
            let mut c = 0u64;
            if i < monos.len() && monos[i] == m {
                c = u64::from(coefs[i]);
                i += 1;
            }
            while j < carried.len() && carried[j].0 == m {
                c = fp.add(c, carried[j].1);
                j += 1;
            }
            if c != 0 {
                out.monos.push(m);
                out.coefs.push(c as u32);
            }
        }
        out
    }

    /// The normal form of a polynomial given by arbitrary exponent vectors
    /// (length [`n_vars`](Self::n_vars)) and coefficients.
    pub fn from_raw(&self, terms: &[(Vec<u32>, u64)]) -> XPoly {
        let fp = Fp::new(self.p);
        let n_t = self.n_tower();
        let mut out: Vec<(XMono, u64)> = Vec::new();
        let mut cur: Vec<(u128, u64)> = Vec::new();
        let mut next: Vec<(u128, u64)> = Vec::new();
        for (e, c) in terms {
            assert_eq!(e.len(), self.n_vars(), "f4_fp_tower_radix: exponent length");
            let c = c % self.p;
            if c == 0 {
                continue;
            }
            let mut f = [0u8; MAX_XFREE];
            for k in 0..self.n_free {
                f[k] =
                    u8::try_from(e[n_t + k]).expect("f4_fp_tower_radix: free exponent above 255");
            }
            cur.clear();
            cur.push((0, c));
            for (j, &ej) in e.iter().take(n_t).enumerate() {
                for _ in 0..ej {
                    next.clear();
                    for &(m, cc) in &cur {
                        self.mul_var_into(fp, m, j, cc, &mut next);
                    }
                    merge_exps(fp, &mut next);
                    std::mem::swap(&mut cur, &mut next);
                }
            }
            for &(m, cc) in &cur {
                out.push((x_mono(m, f), cc));
            }
        }
        XPoly::from_terms(fp, out)
    }

    /// `g` at a point: one value per variable, tower variables first.
    pub fn eval(&self, g: &XPoly, x: &[u64]) -> u64 {
        let fp = Fp::new(self.p);
        let n_t = self.n_tower();
        let mut acc = 0u64;
        for (&m, &c) in g.monos.iter().zip(&g.coefs) {
            let mut t = u64::from(c);
            let tw = x_tw(m);
            for (j, xj) in x.iter().enumerate().take(n_t) {
                let e = lane(tw, j);
                if e > 0 {
                    t = fp.mul(t, fp.pow(*xj, u64::from(e)));
                }
            }
            for (k, &e) in x_free(m).iter().enumerate().take(self.n_free) {
                if e > 0 {
                    t = fp.mul(t, fp.pow(x[n_t + k], u64::from(e)));
                }
            }
            acc = fp.add(acc, t);
        }
        acc
    }

    /// A radix-2 normal form re-keyed in this ring's layout. The ring must
    /// be radix 2 on every level the polynomial uses (every term is
    /// square-free there).
    pub fn xpoly_from_rpoly(&self, g: &RPoly) -> XPoly {
        use super::f4_fp_tower::{free_of, mask_of};
        let n_t = self.n_tower();
        let terms: Vec<(XMono, u64)> = g
            .monos
            .iter()
            .zip(&g.coefs)
            .map(|(&m, &c)| {
                let mut tw = 0u128;
                let mask = mask_of(m);
                for j in 0..n_t.min(64) {
                    tw |= u128::from((mask >> j) & 1) << (2 * j);
                }
                (x_mono(tw, free_of(m)), u64::from(c))
            })
            .collect();
        XPoly::from_terms(Fp::new(self.p), terms)
    }
}

// ── Report ───────────────────────────────────────────────────────────

/// The result of [`f4_radix`]: the fields of the radix-2 engine's
/// `TowerF4Report` that a run without trace learning or replay fills, over
/// [`XPoly`] elements.
#[derive(Clone, Debug, Default)]
pub struct RadixF4Report {
    /// The active basis when the run ended (`[1]` when inconsistent). It
    /// is a Gröbner basis only if the run ended with no pairs left.
    pub basis: Vec<XPoly>,
    pub inconsistent: bool,
    pub degree_reached: u32,
    /// Highest step degree at which the basis gained an element, or at
    /// which `1` appeared: the solving degree.
    pub solving_degree_max: u32,
    pub last_productive_degree: u32,
    pub max_cols_to_solution: usize,
    pub steps_to_solution: usize,
    pub steps: usize,
    pub max_rows: usize,
    pub max_cols: usize,
    pub max_nnz: u64,
    pub max_residual_rows: usize,
    pub rows_skipped: u64,
    pub max_dense_entries: u64,
    /// The unit: `F_p` multiply-adds in the eliminations.
    pub muladds: u64,
    pub critical_pairs_reduced: u64,
    pub tower_pairs_reduced: u64,
    pub pairs_product_skipped: u64,
    pub pairs_chain_skipped: u64,
    pub reducer_rows: u64,
    pub pairs_above_bound: usize,
    pub staircase_at_stop: Option<usize>,
    pub timed_out: bool,
    pub oversize: bool,
    pub ms: f64,
    pub trace: Vec<StepTrace>,
    pub stopped_step: Option<StepTrace>,
}

// ── Pairs and the basis ──────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
enum PairKind {
    /// The S-polynomial of basis elements `i < j`.
    Critical(u32, u32),
    /// `y_v^k · g_i`, with `k = r_v − e_v(LM g_i)`.
    Tower(u32, u8, u8),
}

#[derive(Clone, Copy, Debug)]
struct Pair {
    kind: PairKind,
    /// The lcm of the leading monomials (critical pairs).
    lcm: XMono,
    deg: u32,
}

struct State<'r> {
    ring: &'r RadixRing,
    polys: Vec<Elem>,
    lm: Vec<XMono>,
    active: Vec<bool>,
    /// Active leading monomial → element.
    lm_index: MonoMap<u32>,
    pairs: Vec<Pair>,
}

/// The next smaller exponent vector `s ≤ e` lane-wise, in the order a
/// mixed-radix counter with lane 0 least significant counts down (for
/// 0/1 lanes: `(s − 1) & e`). `false` when `s` was zero.
#[inline]
fn lanes_down(s: &mut u128, e: u128) -> bool {
    let nz = lanes_nz(*s);
    if nz == 0 {
        return false;
    }
    let j = (nz.trailing_zeros() / 2) as usize;
    let low = (1u128 << (2 * j)) - 1;
    let sj = (*s >> (2 * j)) & 3;
    *s = (*s & !((1u128 << (2 * j + 2)) - 1)) | ((sj - 1) << (2 * j)) | (e & low);
    true
}

/// Every monomial `L'` with `lo | L' | L`, `L'` ≠ `L`, is passed to `f`
/// until it returns `true`; returns whether one did.
fn any_between(lo: XMono, l: XMono, mut f: impl FnMut(XMono) -> bool) -> bool {
    let (lo_t, l_t) = (x_tw(lo), x_tw(l));
    let (lo_f, l_f) = (x_free(lo), x_free(l));
    let mut t = lo_t;
    loop {
        let mut fv = lo_f;
        loop {
            let cand = x_mono(t, fv);
            if cand != l && f(cand) {
                return true;
            }
            // next free vector (odometer)
            let mut k = 0;
            loop {
                if k == MAX_XFREE {
                    break;
                }
                if fv[k] < l_f[k] {
                    fv[k] += 1;
                    break;
                }
                fv[k] = lo_f[k];
                k += 1;
            }
            if k == MAX_XFREE {
                break;
            }
        }
        // next tower vector: the lowest lane below its upper bound steps up
        // and the lanes under it return to their lower bound
        let room = lanes_gt(l_t, t);
        if room == 0 {
            break;
        }
        let j = (room.trailing_zeros() / 2) as usize;
        let low = (1u128 << (2 * j)) - 1;
        t = (t & !((1u128 << (2 * j + 2)) - 1))
            | ((u128::from(lane(t, j)) + 1) << (2 * j))
            | (lo_t & low);
    }
    false
}

impl State<'_> {
    /// Becker–Weispfenning `UPDATE` for the critical pairs of a new
    /// element, plus its tower pairs.
    fn insert(&mut self, h_poly: Elem, rep: &mut RadixF4Report) {
        let h = self.polys.len();
        let lh = h_poly.lm();
        self.polys.push(h_poly);
        self.lm.push(lh);
        self.active.push(false);

        let dh = x_degree(lh);
        let th = x_tw(lh);
        each_lane(lanes_nz(th), |v| {
            let k = u32::from(self.ring.rules[v].radix) - lane(th, v);
            self.pairs.push(Pair {
                kind: PairKind::Tower(h as u32, v as u8, k as u8),
                lcm: lh,
                deg: dh + k,
            });
        });

        // New pairs (g, h): see the radix-2 engine, which this follows line
        // for line.
        let mut groups: MonoMap<(u32, bool)> = MonoMap::default();
        for g in 0..h {
            if !self.active[g] {
                continue;
            }
            let l = x_lcm(lh, self.lm[g]);
            let e = groups.entry(l).or_insert((g as u32, false));
            if x_coprime(lh, self.lm[g]) {
                e.1 = true;
            }
        }
        let mut kept: Vec<(u32, XMono)> = Vec::new();
        for (&l, &(g, has_coprime)) in &groups {
            if any_between(lh, l, |c| groups.contains_key(&c)) {
                rep.pairs_chain_skipped += 1;
                continue;
            }
            if has_coprime {
                rep.pairs_product_skipped += 1;
                continue;
            }
            kept.push((g, l));
        }
        let lm = &self.lm;
        let mut dropped = 0u64;
        self.pairs.retain(|p| match p.kind {
            PairKind::Tower(..) => true,
            PairKind::Critical(i, j) => {
                let keep = !x_divides(lh, p.lcm)
                    || x_lcm(lm[i as usize], lh) == p.lcm
                    || x_lcm(lm[j as usize], lh) == p.lcm;
                if !keep {
                    dropped += 1;
                }
                keep
            }
        });
        rep.pairs_chain_skipped += dropped;
        kept.sort_unstable_by_key(|&(g, _)| g);
        for (g, l) in kept {
            self.pairs.push(Pair {
                kind: PairKind::Critical(g, h as u32),
                lcm: l,
                deg: x_degree(l),
            });
        }
        for g in 0..h {
            if self.active[g] && x_divides(lh, self.lm[g]) {
                self.active[g] = false;
                if self.lm_index.get(&self.lm[g]) == Some(&(g as u32)) {
                    self.lm_index.remove(&self.lm[g]);
                }
            }
        }
        self.active[h] = true;
        self.lm_index.insert(lh, h as u32);
    }

    /// An active element whose leading monomial divides `m`, the shortest;
    /// the first found among equals, in the radix-2 engine's order.
    fn reducer_for(&self, m: XMono) -> Option<u32> {
        let e = x_tw(m);
        let f = x_free(m);
        let mut best: Option<u32> = None;
        let mut s = e;
        loop {
            let mut fv = [0u8; MAX_XFREE];
            loop {
                if let Some(&g) = self.lm_index.get(&x_mono(s, fv)) {
                    if best
                        .is_none_or(|b| self.polys[g as usize].len() < self.polys[b as usize].len())
                    {
                        best = Some(g);
                    }
                }
                let mut k = 0;
                loop {
                    if k == MAX_XFREE {
                        break;
                    }
                    if fv[k] < f[k] {
                        fv[k] += 1;
                        break;
                    }
                    fv[k] = 0;
                    k += 1;
                }
                if k == MAX_XFREE {
                    break;
                }
            }
            if !lanes_down(&mut s, e) {
                break;
            }
        }
        best
    }

    fn active_lms(&self) -> Vec<XMono> {
        (0..self.polys.len())
            .filter(|&g| self.active[g])
            .map(|g| self.lm[g])
            .collect()
    }
}

/// The number of standard monomials of `R / (lms)`, or `None` if it
/// exceeds `bound` or is infinite (a free variable without a pure power
/// among the leading monomials). The tower rules' own leads `y_j^{r_j}` are
/// not in `lms`; the walk stops at exponent `r_j − 1`.
pub fn x_staircase_at_most(lms: &[XMono], ring: &RadixRing, bound: usize) -> Option<usize> {
    let (n_tower, n_free) = (ring.n_tower(), ring.n_free);
    let mut cap = [0u8; MAX_XFREE];
    for (k, c) in cap.iter_mut().enumerate().take(n_free) {
        *c = lms
            .iter()
            .filter(|&&m| {
                x_tw(m) == 0
                    && x_free(m)
                        .iter()
                        .enumerate()
                        .all(|(i, &e)| (i == k) == (e > 0))
            })
            .map(|&m| x_free(m)[k])
            .min()?;
    }
    struct Ctx<'a> {
        lms: &'a [XMono],
        ring: &'a RadixRing,
        n_tower: usize,
        n_free: usize,
        cap: [u8; MAX_XFREE],
    }
    fn walk(
        k: usize,
        tw: u128,
        free: &mut [u8; MAX_XFREE],
        ctx: &Ctx,
        count: &mut usize,
        bound: usize,
    ) -> bool {
        let cur = x_mono(tw, *free);
        if ctx.lms.iter().any(|&l| x_divides(l, cur)) {
            return true;
        }
        if k == ctx.n_tower + ctx.n_free {
            *count += 1;
            return *count <= bound;
        }
        if k < ctx.n_tower {
            for e in 0..u128::from(ctx.ring.rules[k].radix) {
                if !walk(k + 1, tw | (e << (2 * k)), free, ctx, count, bound) {
                    return false;
                }
            }
            true
        } else {
            let f = k - ctx.n_tower;
            for e in 0..ctx.cap[f] {
                free[f] = e;
                if !walk(k + 1, tw, free, ctx, count, bound) {
                    free[f] = 0;
                    return false;
                }
            }
            free[f] = 0;
            true
        }
    }
    let ctx = Ctx {
        lms,
        ring,
        n_tower,
        n_free,
        cap,
    };
    let mut count = 0usize;
    let mut free = [0u8; MAX_XFREE];
    walk(0, 0, &mut free, &ctx, &mut count, bound).then_some(count)
}

/// Forms the products `mult · polys[g]` of `todo` a batch at a time and
/// records each non-empty one in `out`; see the radix-2 engine's
/// `scan_products`, which this repeats over [`XMono`] keys.
#[allow(clippy::too_many_arguments)]
fn scan_products(
    ring: &RadixRing,
    fp: Fp,
    polys: &[Elem],
    todo: &[(XMono, u32)],
    skip: usize,
    out: &mut Vec<RowDesc>,
    examined: &mut MonoMap<u32>,
    frontier: &mut Vec<XMono>,
    cache: &mut ProductCache,
) -> u64 {
    let mut terms = 0u64;
    let mut ids: Vec<u32> = Vec::new();
    for batch in todo.chunks(1024) {
        let prods: Vec<XPoly> = batch
            .par_iter()
            .map(|&(mult, g)| ring.mul_elem(fp, &polys[g as usize], mult))
            .collect();
        for (&(mult, g), p) in batch.iter().zip(&prods) {
            let Some(lead) = p.lm() else { continue };
            terms += p.len() as u64;
            let keep = cache.has_room(p.len());
            ids.clear();
            if keep {
                ids.extend(p.monos.iter().take(skip).map(|m| examined[m]));
            }
            for &m in p.monos.iter().skip(skip) {
                let id = number_of(examined, frontier, m);
                if keep {
                    ids.push(id);
                }
            }
            out.push(RowDesc {
                mult,
                g,
                lead,
                len: p.len() as u32,
                at: if keep {
                    cache.keep(&ids, &p.coefs)
                } else {
                    NONE
                },
            });
        }
    }
    terms
}

// ── The algorithm ────────────────────────────────────────────────────

/// **F4 on `R`** for the ideal generated by `input` (normal forms), with
/// the normal selection strategy. See the module notes.
///
/// `opts` is the radix-2 engine's option set. Weights, `record_trace` and
/// `replay` are not supported and must be unset.
pub fn f4_radix(input: &[XPoly], ring: &RadixRing, opts: &TowerF4Options) -> RadixF4Report {
    f4_radix_cached(input, ring, opts, PRODUCT_CACHE_TERMS)
}

fn f4_radix_cached(
    input: &[XPoly],
    ring: &RadixRing,
    opts: &TowerF4Options,
    cache_terms: usize,
) -> RadixF4Report {
    ring.check();
    assert!(
        opts.weights.is_none() && opts.replay.is_none() && !opts.record_trace,
        "f4_fp_tower_radix: weights, trace learning and replay are radix-2 engine features"
    );
    assert!(
        opts.max_degree <= MAX_DEGREE,
        "f4_fp_tower_radix: max_degree above 2^16 − 1"
    );
    let started = Instant::now();
    let fp = Fp::new(ring.p);
    let mut rep = RadixF4Report::default();
    let finish = |mut rep: RadixF4Report| {
        rep.ms = started.elapsed().as_secs_f64() * 1e3;
        rep
    };
    let unit = || {
        vec![XPoly {
            monos: vec![X_ONE],
            coefs: vec![1],
        }]
    };

    let inputs: Vec<XPoly> = input.iter().filter(|p| !p.is_empty()).cloned().collect();
    if inputs.iter().any(|p| p.is_unit()) {
        rep.inconsistent = true;
        rep.basis = unit();
        return finish(rep);
    }

    // The initial echelon: distinct leading monomials to start from.
    let start = {
        let mut all: Vec<XMono> = inputs
            .iter()
            .flat_map(|p| p.monos.iter().copied())
            .collect();
        all.sort_unstable_by(|a, b| b.cmp(a));
        all.dedup();
        let col_of: MonoMap<u32> = all
            .iter()
            .enumerate()
            .map(|(i, &m)| (m, i as u32))
            .collect();
        let rows: Vec<Row> = inputs
            .iter()
            .map(|p| Row {
                cols: p.monos.iter().map(|m| col_of[m]).collect(),
                vals: p.coefs.clone(),
            })
            .collect();
        match echelon(fp, rows, all.len(), &mut rep.muladds, opts.deadline) {
            Some(piv) => piv
                .into_iter()
                .map(|r| XPoly {
                    monos: r.cols.iter().map(|&c| all[c as usize]).collect(),
                    coefs: r.vals,
                })
                .collect::<Vec<_>>(),
            None => {
                rep.timed_out = true;
                rep.basis = inputs;
                return finish(rep);
            }
        }
    };
    if start.iter().any(|p| p.is_unit()) {
        rep.inconsistent = true;
        rep.basis = unit();
        return finish(rep);
    }
    let mut start = start;
    start.sort_by_key(|p| std::cmp::Reverse(p.lm().unwrap()));

    let mut s = State {
        ring,
        polys: Vec::new(),
        lm: Vec::new(),
        active: Vec::new(),
        lm_index: MonoMap::default(),
        pairs: Vec::new(),
    };
    for p in start {
        s.insert(Elem::from_rpoly(p.into_rpoly()), &mut rep);
    }
    // (solving_degree_max, max_cols_to_solution, steps_to_solution)
    let mut learned = (0u32, 0usize, 0usize);
    let mut max_cols_so_far = 0usize;

    let active_basis = |s: &State| -> Vec<XPoly> {
        (0..s.polys.len())
            .filter(|&g| s.active[g])
            .map(|g| xpoly_of(&s.polys[g]))
            .collect()
    };
    let kept = |s: &State, tr: &mut StepTrace| {
        tr.basis_active = s.active.iter().filter(|&&a| a).count();
        tr.basis_kept = s.polys.len();
        tr.basis_entries = s.polys.iter().map(|p| p.vals.len() as u64).sum();
        tr.pairs_left = s.pairs.len();
    };
    let record = |rep: &mut RadixF4Report, learned: (u32, usize, usize)| {
        rep.solving_degree_max = learned.0;
        rep.max_cols_to_solution = learned.1;
        rep.steps_to_solution = learned.2;
    };

    while !s.pairs.is_empty() {
        if opts.deadline.is_some_and(|d| Instant::now() >= d) {
            rep.timed_out = true;
            break;
        }
        let d = s.pairs.iter().map(|p| p.deg).min().unwrap();
        if d > opts.max_degree {
            rep.pairs_above_bound = s.pairs.len();
            break;
        }
        let step_started = Instant::now();
        let (selected, rest): (Vec<Pair>, Vec<Pair>) = s.pairs.drain(..).partition(|p| p.deg == d);
        s.pairs = rest;
        rep.steps += 1;
        rep.degree_reached = rep.degree_reached.max(d);
        let mut tr = StepTrace {
            degree: d,
            ..Default::default()
        };

        // The rows the step reduces: both halves of every critical pair and
        // every tower product, each (multiplier, element) once.
        let mut seen: HashMap<(XMono, u32), (), BuildHasherDefault<MonoHasher>> =
            HashMap::default();
        let mut jobs: Vec<(XMono, u32)> = Vec::new();
        let mut lcm_cols: MonoMap<()> = MonoMap::default();
        for p in &selected {
            match p.kind {
                PairKind::Critical(i, j) => {
                    tr.critical_pairs += 1;
                    lcm_cols.insert(p.lcm, ());
                    for g in [i, j] {
                        let mult = x_quotient(p.lcm, s.lm[g as usize]);
                        if seen.insert((mult, g), ()).is_none() {
                            jobs.push((mult, g));
                        }
                    }
                }
                PairKind::Tower(g, v, k) => {
                    tr.tower_pairs += 1;
                    // A tower multiplier `y_v^k` completes LM(g)'s `y_v` to
                    // the rule's `y_v^r`.
                    let mult = x_mono(u128::from(k) << (2 * v as usize), [0; MAX_XFREE]);
                    if seen.insert((mult, g), ()).is_none() {
                        jobs.push((mult, g));
                    }
                }
            }
        }
        rep.critical_pairs_reduced += tr.critical_pairs as u64;
        rep.tower_pairs_reduced += tr.tower_pairs as u64;

        // Symbolic preprocessing, one frontier at a time (see the radix-2
        // engine).
        let mut examined: MonoMap<u32> = lcm_cols
            .keys()
            .enumerate()
            .map(|(i, &m)| (m, i as u32))
            .collect();
        let mut no_divisor: MonoMap<()> = MonoMap::default();
        let mut frontier: Vec<XMono> = Vec::new();
        let mut s_desc: Vec<RowDesc> = Vec::with_capacity(jobs.len());
        let mut cache = ProductCache::new(cache_terms);
        let mut nnz = scan_products(
            ring,
            fp,
            &s.polys,
            &jobs,
            0,
            &mut s_desc,
            &mut examined,
            &mut frontier,
            &mut cache,
        );
        let mut r_desc: Vec<RowDesc> = Vec::new();
        let mut aborted = false;
        while !frontier.is_empty() {
            if opts.deadline.is_some_and(|dl| Instant::now() >= dl) {
                aborted = true;
                break;
            }
            let found: Vec<(XMono, Option<u32>)> = frontier
                .par_iter()
                .map(|&m| (m, s.reducer_for(m)))
                .collect();
            frontier.clear();
            let mut todo: Vec<(XMono, u32)> = Vec::new();
            for &(m, g) in &found {
                match g {
                    Some(g) => todo.push((x_quotient(m, s.lm[g as usize]), g)),
                    None => {
                        no_divisor.insert(m, ());
                    }
                }
            }
            let before = r_desc.len();
            nnz += scan_products(
                ring,
                fp,
                &s.polys,
                &todo,
                1,
                &mut r_desc,
                &mut examined,
                &mut frontier,
                &mut cache,
            );
            debug_assert!(
                r_desc[before..]
                    .iter()
                    .zip(found.iter().filter(|f| f.1.is_some()))
                    .all(|(d, f)| d.lead == f.0),
                "a reducer must lead with its monomial"
            );
            if opts.max_nnz.is_some_and(|cap| nnz > cap) {
                rep.oversize = true;
                aborted = true;
                break;
            }
        }
        if aborted {
            if !rep.oversize {
                rep.timed_out = true;
            }
            s.pairs.extend(selected);
            tr.nnz = nnz;
            kept(&s, &mut tr);
            tr.ms = step_started.elapsed().as_secs_f64() * 1e3;
            if let Some(f) = opts.on_stop {
                f(rep.steps, rep.oversize, &tr);
            }
            rep.stopped_step = Some(tr);
            break;
        }
        rep.reducer_rows += r_desc.len() as u64;

        // Columns, descending.
        let (cols, col_of_id) = order_columns(&examined);
        let mut col_of = examined;
        for c in col_of.values_mut() {
            *c = col_of_id[*c as usize];
        }
        let n_cols = cols.len();
        let mut pivot_of = vec![NONE; n_cols];
        let mut piv: Vec<RowDesc> = r_desc;
        for (k, d) in piv.iter().enumerate() {
            pivot_of[col_of[&d.lead] as usize] = k as u32;
        }
        let n_reducers = piv.len();
        // The shortest S-row at each lcm column joins the reducers, made
        // monic (the A|B and C|D split of the radix-2 engine).
        s_desc.sort_by_key(|d| (std::cmp::Reverse(d.lead), d.len));
        let mut s_list: Vec<RowDesc> = Vec::with_capacity(s_desc.len());
        for d in s_desc {
            let c = col_of[&d.lead] as usize;
            if pivot_of[c] == NONE && lcm_cols.contains_key(&d.lead) {
                pivot_of[c] = piv.len() as u32;
                piv.push(d);
            } else {
                s_list.push(d);
            }
        }
        tr.promoted_rows = piv.len() - n_reducers;
        tr.s_rows = s_list.len();
        tr.reducer_rows = n_reducers;
        tr.cols = n_cols;
        tr.nnz = nnz;
        rep.max_rows = rep.max_rows.max(s_list.len() + piv.len());
        rep.max_cols = rep.max_cols.max(n_cols);
        rep.max_nnz = rep.max_nnz.max(nnz);
        max_cols_so_far = max_cols_so_far.max(n_cols);

        tr.ms_rows = step_started.elapsed().as_secs_f64() * 1e3;

        debug_assert!(
            (0..n_cols).all(|c| (pivot_of[c] == NONE) == no_divisor.contains_key(&cols[c])),
            "the columns without a reducer must be those without a divisor"
        );
        let polys = &s.polys;
        let form = |d: &RowDesc| -> Row {
            if d.at != NONE {
                let at = d.at as usize..d.at as usize + d.len as usize;
                return Row {
                    cols: cache.ids[at.clone()]
                        .iter()
                        .map(|&id| col_of_id[id as usize])
                        .collect(),
                    vals: cache.coefs[at].to_vec(),
                };
            }
            let p = ring.mul_elem(fp, &polys[d.g as usize], d.mult);
            debug_assert_eq!(
                p.len(),
                d.len as usize,
                "a row must be formed as it was scanned"
            );
            Row {
                cols: p.monos.iter().map(|m| col_of[m]).collect(),
                vals: p.coefs,
            }
        };
        let piv_row = |k: usize| -> Row {
            let mut r = form(&piv[k]);
            if r.vals[0] != 1 {
                // Only a promoted tower row can lead with another
                // coefficient: products of monic elements stay monic.
                debug_assert!(k >= n_reducers);
                let inv = fp.inv(u64::from(r.vals[0]));
                for v in &mut r.vals {
                    *v = fp.mul(u64::from(*v), inv) as u32;
                }
            }
            r
        };
        let s_row = |k: usize| form(&s_list[k]);
        let piv_leads: Vec<u32> = piv.iter().map(|d| col_of[&d.lead]).collect();
        let mut st = ElimStats::default();
        let result = eliminate(
            fp,
            n_cols,
            &piv_leads,
            &piv_row,
            &pivot_of,
            s_list.len(),
            &s_row,
            opts.deadline,
            opts.max_dense.or(opts.max_nnz),
            opts.full_rank_exit,
            &mut st,
        );
        drop(cache);
        drop(col_of_id);
        tr.muladds = st.muladds;
        rep.muladds += st.muladds;
        tr.residual_rows = st.residual_rows;
        tr.full_rank_after = st.full_rank_after;
        tr.skipped_rows = st.skipped_rows;
        rep.rows_skipped += st.skipped_rows as u64;
        tr.residual_cols = st.q_cols;
        tr.dense_entries = st.dense_entries;
        tr.ms_reduce = st.ms_reducers;
        tr.ms_echelon = st.ms_residues;
        rep.max_residual_rows = rep.max_residual_rows.max(st.residual_rows);
        rep.max_dense_entries = rep.max_dense_entries.max(st.dense_entries);
        let ech = match result {
            Ok(ech) => ech,
            Err(stop) => {
                match stop {
                    Stop::Deadline => rep.timed_out = true,
                    Stop::Oversize => rep.oversize = true,
                }
                s.pairs.extend(selected);
                kept(&s, &mut tr);
                tr.ms = step_started.elapsed().as_secs_f64() * 1e3;
                if let Some(f) = opts.on_stop {
                    f(rep.steps, rep.oversize, &tr);
                }
                rep.stopped_step = Some(tr);
                break;
            }
        };
        let update_started = Instant::now();

        // The new elements are the echelon's rows, over the step's columns
        // without a divisor.
        let Echelon {
            q_col,
            leads,
            rows: mut ech_rows,
        } = ech;
        let q_monos: Arc<[XMono]> = q_col.iter().map(|&c| cols[c as usize]).collect();
        let mut fresh: Vec<Elem> = leads
            .iter()
            .map(|&l| {
                Elem::new(
                    q_monos.clone(),
                    l,
                    std::mem::take(&mut ech_rows[l as usize]),
                )
            })
            .collect();
        drop(ech_rows);
        tr.fresh = fresh.len();
        tr.fresh_min_degree = fresh.iter().map(|p| x_degree(p.lm())).min().unwrap_or(0);
        if !fresh.is_empty() {
            learned = (learned.0.max(d), max_cols_so_far, rep.steps);
            rep.last_productive_degree = d;
        }
        let refuted = fresh.iter().any(|p| p.lm() == X_ONE);
        if refuted {
            rep.inconsistent = true;
            rep.basis = unit();
            record(&mut rep, learned);
            kept(&s, &mut tr);
            tr.ms = step_started.elapsed().as_secs_f64() * 1e3;
            if let Some(f) = opts.on_step {
                f(rep.steps, &tr);
            }
            rep.trace.push(tr);
            return finish(rep);
        }
        // Largest leading monomial first, so that a later, smaller one
        // retires the earlier ones it divides (`UPDATE`'s G_new).
        fresh.sort_by_key(|p| std::cmp::Reverse(p.lm()));
        for p in fresh {
            debug_assert_eq!(p.vals[0], 1, "an echelon row is monic");
            s.insert(p, &mut rep);
        }
        kept(&s, &mut tr);
        tr.ms_update = update_started.elapsed().as_secs_f64() * 1e3;
        tr.ms = step_started.elapsed().as_secs_f64() * 1e3;
        if let Some(f) = opts.on_step {
            f(rep.steps, &tr);
        }
        rep.trace.push(tr);

        if let Some(bound) = opts.stop_staircase {
            if !s.pairs.is_empty() {
                if let Some(count) = x_staircase_at_most(&s.active_lms(), ring, bound) {
                    rep.staircase_at_stop = Some(count);
                    break;
                }
            }
        }
    }
    record(&mut rep, learned);
    rep.basis = active_basis(&s);
    finish(rep)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::f4_fp::{self, F4Options, Ordering};
    use crate::cryptanalysis::f4_fp_tower::{self as narrow, f4_tower};
    use rand::{Rng, SeedableRng};
    use std::collections::{BTreeMap, BTreeSet};

    fn fp(p: u64) -> Fp {
        Fp::new(p)
    }

    fn sub(f: Fp, a: u64, b: u64) -> u64 {
        f.add(a, f.neg(b))
    }

    // ── Reference arithmetic on dense exponent vectors ──

    /// Grevlex on exponent vectors, variable 0 largest: degree first, then
    /// the smaller exponent of the last differing variable wins.
    fn grevlex_gt(a: &[u32], b: &[u32]) -> bool {
        let (da, db): (u32, u32) = (a.iter().sum(), b.iter().sum());
        if da != db {
            return da > db;
        }
        for i in (0..a.len()).rev() {
            if a[i] != b[i] {
                return a[i] < b[i];
            }
        }
        false
    }

    /// Generic reduction by `y_j^{r_j} − rule_j`, an independent reference
    /// for the rewriting.
    fn reference_nf(ring: &RadixRing, terms: &[(Vec<u32>, u64)]) -> BTreeMap<Vec<u32>, u64> {
        let p = ring.p;
        let f = fp(p);
        let mut work: BTreeMap<Vec<u32>, u64> = BTreeMap::new();
        for (e, c) in terms {
            let v = work.entry(e.clone()).or_insert(0);
            *v = f.add(*v, c % p);
        }
        // Variable by variable, lowest first: reducing `y_j^r` only raises
        // later variables, so once lane `j` is below its radix it stays.
        for j in 0..ring.n_tower() {
            let r = ring.rules[j];
            loop {
                let batch: Vec<(Vec<u32>, u64)> = work
                    .iter()
                    .filter(|(e, c)| **c != 0 && e[j] >= u32::from(r.radix))
                    .map(|(e, c)| (e.clone(), *c))
                    .collect();
                if batch.is_empty() {
                    break;
                }
                for (e, c) in batch {
                    work.remove(&e);
                    let mut base = e.clone();
                    base[j] -= u32::from(r.radix);
                    for k in 0..r.radix as usize {
                        for (coef, nx) in [(r.c[k], None), (r.b[k], r.next)] {
                            if coef == 0 {
                                continue;
                            }
                            let mut t = base.clone();
                            t[j] += k as u32;
                            if let Some(nx) = nx {
                                t[nx as usize] += 1;
                            }
                            let v = work.entry(t).or_insert(0);
                            *v = f.add(*v, f.mul(c, coef));
                        }
                    }
                }
            }
        }
        work.retain(|_, c| *c != 0);
        work
    }

    fn as_map(ring: &RadixRing, g: &XPoly) -> BTreeMap<Vec<u32>, u64> {
        g.monos
            .iter()
            .zip(&g.coefs)
            .map(|(&m, &c)| (x_exponents(m, ring.n_tower(), ring.n_free), u64::from(c)))
            .collect()
    }

    fn random_ring(
        rng: &mut impl rand::Rng,
        p: u64,
        radices: &[u8],
        blocks: usize,
        n_free: usize,
    ) -> RadixRing {
        let t = radices.len();
        let mut rules = Vec::new();
        for b in 0..blocks {
            for (j, &radix) in radices.iter().enumerate() {
                let top = j + 1 == t;
                let mut r = PowRule {
                    radix,
                    next: (!top).then_some((b * t + j + 1) as u8),
                    c: [0; 4],
                    b: [0; 4],
                };
                for k in 0..radix as usize {
                    r.c[k] = rng.gen_range(0..p);
                    if !top {
                        r.b[k] = rng.gen_range(0..p);
                    }
                }
                rules.push(r);
            }
        }
        RadixRing { p, rules, n_free }
    }

    #[test]
    fn keys_order_like_grevlex_and_the_monomial_algebra_is_exact() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(11);
        let n_t = 12usize;
        let n_f = 2usize;
        let rnd = |rng: &mut rand::rngs::StdRng| -> Vec<u32> {
            let mut e: Vec<u32> = (0..n_t)
                .map(|_| {
                    if rng.gen_bool(0.5) {
                        0
                    } else {
                        rng.gen_range(0..4)
                    }
                })
                .collect();
            e.extend((0..n_f).map(|_| rng.gen_range(0..5)));
            e
        };
        for _ in 0..4000 {
            let (a, b) = (rnd(&mut rng), rnd(&mut rng));
            let (ka, kb) = (x_from_exponents(&a, n_t), x_from_exponents(&b, n_t));
            assert_eq!(x_exponents(ka, n_t, n_f)[..], a[..n_t + n_f]);
            assert_eq!(x_degree(ka), a.iter().sum::<u32>());
            // The key order is grevlex.
            assert_eq!(ka > kb, grevlex_gt(&a, &b), "{a:?} vs {b:?}");
            assert_eq!(ka == kb, a == b);
            // Divisibility, lcm, coprimality.
            let divides = a.iter().zip(&b).all(|(x, y)| x <= y);
            assert_eq!(x_divides(ka, kb), divides);
            let l: Vec<u32> = a.iter().zip(&b).map(|(x, y)| *x.max(y)).collect();
            assert_eq!(x_lcm(ka, kb), x_from_exponents(&l, n_t));
            let cop = a.iter().zip(&b).all(|(x, y)| *x == 0 || *y == 0);
            assert_eq!(x_coprime(ka, kb), cop);
            if divides {
                let q: Vec<u32> = b.iter().zip(&a).map(|(x, y)| x - y).collect();
                assert_eq!(x_quotient(kb, ka), x_from_exponents(&q, n_t));
            }
        }
        assert_eq!(x_from_exponents(&vec![0; n_t + n_f], n_t), X_ONE);
    }

    #[test]
    fn the_divisor_walks_visit_every_divisor_once() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(12);
        let n_t = 6usize;
        for _ in 0..200 {
            let e: Vec<u32> = (0..n_t)
                .map(|_| {
                    if rng.gen_bool(0.4) {
                        0
                    } else {
                        rng.gen_range(0..4)
                    }
                })
                .collect();
            let tw = x_tw(x_from_exponents(&e, n_t));
            // `lanes_down` walks all vectors ≤ e, descending as a
            // mixed-radix number.
            let mut seen = vec![tw];
            let mut s = tw;
            while lanes_down(&mut s, tw) {
                seen.push(s);
            }
            let want: usize = e.iter().map(|&x| x as usize + 1).product();
            assert_eq!(seen.len(), want);
            let uniq: BTreeSet<u128> = seen.iter().copied().collect();
            assert_eq!(uniq.len(), want);
            assert!(seen.windows(2).all(|w| w[0] > w[1]));
            // `any_between(lo, l)` sees every monomial in [lo, l] but `l`.
            let lo_e: Vec<u32> = e.iter().map(|&x| if x > 0 { x - 1 } else { 0 }).collect();
            let (lo, l) = (x_from_exponents(&lo_e, n_t), x_from_exponents(&e, n_t));
            let mut visited = BTreeSet::new();
            assert!(!any_between(lo, l, |c| {
                assert!(visited.insert(c), "a monomial was visited twice");
                false
            }));
            let want: usize = e
                .iter()
                .zip(&lo_e)
                .map(|(&x, &y)| (x - y) as usize + 1)
                .product::<usize>()
                - 1;
            assert_eq!(visited.len(), want);
            assert!(!visited.contains(&l));
            assert!(visited.iter().all(|&c| x_divides(lo, c) && x_divides(c, l)));
        }
    }

    #[test]
    fn products_match_a_generic_reduction() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(2027);
        let p = 10007u64;
        let towers: [&[u8]; 6] = [
            &[3],
            &[3, 3],
            &[2, 3, 2],
            &[4, 3, 2, 2],
            &[3, 4, 3],
            &[4, 4],
        ];
        for radices in towers {
            for n_free in [0usize, 1] {
                let blocks = if radices.len() <= 2 { 2 } else { 1 };
                let ring = random_ring(&mut rng, p, radices, blocks, n_free);
                ring.check();
                let n = ring.n_vars();
                for _ in 0..15 {
                    let rand_poly = |rng: &mut rand::rngs::StdRng| -> Vec<(Vec<u32>, u64)> {
                        (0..rng.gen_range(1..5))
                            .map(|_| {
                                let e: Vec<u32> = (0..n)
                                    .map(|i| {
                                        if rng.gen_bool(0.6) {
                                            0
                                        } else if i < ring.n_tower() {
                                            rng.gen_range(0..=u32::from(ring.rules[i].radix) + 1)
                                        } else {
                                            rng.gen_range(0..3)
                                        }
                                    })
                                    .collect();
                                (e, rng.gen_range(1..p))
                            })
                            .collect()
                    };
                    let (ra, rb) = (rand_poly(&mut rng), rand_poly(&mut rng));
                    // `from_raw` against the generic reduction.
                    let ga = ring.from_raw(&ra);
                    assert_eq!(as_map(&ring, &ga), reference_nf(&ring, &ra), "{radices:?}");
                    assert!(ga.monos.windows(2).all(|w| w[0] > w[1]));
                    assert!(ga.coefs.iter().all(|&c| c != 0));
                    // A product by a monomial of the ring against the same
                    // reduction of the raw product.
                    let gb = ring.from_raw(&rb);
                    for &m in gb.monos.iter().take(4) {
                        let got = ring.mul_mono(&ga, m);
                        let mut want = Vec::new();
                        for (ea, ca) in &ra {
                            let eb = x_exponents(m, ring.n_tower(), ring.n_free);
                            let e: Vec<u32> = ea.iter().zip(&eb).map(|(x, y)| x + y).collect();
                            want.push((e, *ca));
                        }
                        assert_eq!(as_map(&ring, &got), reference_nf(&ring, &want));
                        assert!(got.monos.windows(2).all(|w| w[0] > w[1]));
                        // A basis element stored as an element multiplies
                        // the same.
                        let el = Elem::from_rpoly(ga.clone().into_rpoly());
                        assert_eq!(ring.mul_elem(fp(p), &el, m), got);
                    }
                }
            }
        }
    }

    #[test]
    fn a_free_exponent_past_255_is_refused() {
        let ring = RadixRing {
            p: 7,
            rules: vec![PowRule::kummer(3, None)],
            n_free: 1,
        };
        let g = ring.from_raw(&[(vec![0, 200], 1)]);
        let u = ring.from_raw(&[(vec![0, 100], 1)]);
        let r = std::panic::catch_unwind(|| ring.mul_mono(&g, u.lm().unwrap()));
        assert!(r.is_err());
    }

    // ── A radix-2 ring gives the radix-2 engine's run ──

    fn narrow_exps(ring: &TowerRing, m: narrow::Mono) -> Vec<u32> {
        let mask = narrow::mask_of(m);
        let mut e: Vec<u32> = (0..ring.n_tower())
            .map(|j| ((mask >> j) & 1) as u32)
            .collect();
        e.extend(
            narrow::free_of(m)
                .iter()
                .take(ring.n_free)
                .map(|&x| u32::from(x)),
        );
        e
    }

    fn trace_fields(t: &StepTrace) -> Vec<u64> {
        vec![
            u64::from(t.degree),
            t.critical_pairs as u64,
            t.tower_pairs as u64,
            t.s_rows as u64,
            t.reducer_rows as u64,
            t.promoted_rows as u64,
            t.cols as u64,
            t.nnz,
            t.residual_rows as u64,
            t.residual_cols as u64,
            t.full_rank_after.map_or(u64::MAX, |x| x as u64),
            t.skipped_rows as u64,
            t.dense_entries,
            t.fresh as u64,
            u64::from(t.fresh_min_degree),
            t.muladds,
            t.basis_active as u64,
            t.basis_kept as u64,
            t.basis_entries,
            t.pairs_left as u64,
        ]
    }

    /// Runs both engines on the same system over the same radix-2 ring and
    /// asserts the same run: every step's counts, the counters, the basis.
    fn assert_same_run(
        nring: &narrow::TowerRing,
        xring: &RadixRing,
        ninput: &[narrow::RPoly],
        xinput: &[XPoly],
        opts: &narrow::TowerF4Options,
        tag: &str,
    ) {
        let nrep = f4_tower(ninput, nring, opts);
        let xrep = f4_radix(xinput, xring, opts);
        println!(
            "identity {tag}: steps {}, tower pairs {}, muladds {}, inconsistent {}, D {}",
            xrep.steps,
            xrep.tower_pairs_reduced,
            xrep.muladds,
            xrep.inconsistent,
            xrep.solving_degree_max
        );
        assert_eq!(nrep.inconsistent, xrep.inconsistent, "{tag}");
        assert_eq!(nrep.steps, xrep.steps, "{tag}");
        assert_eq!(nrep.solving_degree_max, xrep.solving_degree_max, "{tag}");
        assert_eq!(nrep.degree_reached, xrep.degree_reached, "{tag}");
        assert_eq!(nrep.pairs_above_bound, xrep.pairs_above_bound, "{tag}");
        assert_eq!(nrep.muladds, xrep.muladds, "{tag}");
        assert_eq!(
            nrep.critical_pairs_reduced, xrep.critical_pairs_reduced,
            "{tag}"
        );
        assert_eq!(nrep.tower_pairs_reduced, xrep.tower_pairs_reduced, "{tag}");
        assert_eq!(nrep.pairs_chain_skipped, xrep.pairs_chain_skipped, "{tag}");
        assert_eq!(
            nrep.pairs_product_skipped, xrep.pairs_product_skipped,
            "{tag}"
        );
        assert_eq!(nrep.reducer_rows, xrep.reducer_rows, "{tag}");
        assert_eq!(nrep.max_cols, xrep.max_cols, "{tag}");
        assert_eq!(nrep.max_rows, xrep.max_rows, "{tag}");
        assert_eq!(nrep.max_nnz, xrep.max_nnz, "{tag}");
        assert_eq!(nrep.trace.len(), xrep.trace.len(), "{tag}");
        for (a, b) in nrep.trace.iter().zip(&xrep.trace) {
            assert_eq!(trace_fields(a), trace_fields(b), "{tag}");
        }
        // The basis is the same, element for element.
        assert_eq!(nrep.basis.len(), xrep.basis.len(), "{tag}");
        for (a, b) in nrep.basis.iter().zip(&xrep.basis) {
            let am: Vec<(Vec<u32>, u32)> = a
                .monos
                .iter()
                .zip(&a.coefs)
                .map(|(&m, &c)| (narrow_exps(nring, m), c))
                .collect();
            let bm: Vec<(Vec<u32>, u32)> = b
                .monos
                .iter()
                .zip(&b.coefs)
                .map(|(&m, &c)| (x_exponents(m, xring.n_tower(), xring.n_free), c))
                .collect();
            assert_eq!(am, bm, "{tag}");
        }
    }

    #[test]
    fn a_radix_2_ring_gives_the_radix_2_engines_run_step_for_step() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(77);
        for (p, blocks, t, n_free, general, n_eq) in [
            (97u64, 2usize, 3usize, 0usize, false, 1usize),
            (97, 2, 3, 0, false, 2),
            (193, 2, 4, 0, true, 2),
            (193, 3, 2, 1, true, 3),
            (65521, 2, 3, 1, true, 3),
            (193, 3, 3, 0, false, 3),
        ] {
            // The radix-2 ring (a, b, c, d in general position when asked).
            let mut rules = Vec::new();
            for b in 0..blocks {
                for j in 0..t {
                    let top = j + 1 == t;
                    rules.push(if general {
                        narrow::SquareRule {
                            next: (!top).then_some((b * t + j + 1) as u8),
                            a: if top { 0 } else { rng.gen_range(0..p) },
                            b: if top { 0 } else { rng.gen_range(0..p) },
                            c: rng.gen_range(0..p),
                            d: rng.gen_range(0..p),
                        }
                    } else {
                        narrow::SquareRule {
                            next: (!top).then_some((b * t + j + 1) as u8),
                            a: 0,
                            b: u64::from(!top),
                            c: 0,
                            d: if top { rng.gen_range(1..p) } else { 0 },
                        }
                    });
                }
            }
            let nring = narrow::TowerRing { p, rules, n_free };
            let xring = RadixRing::from_square_ring(&nring);
            xring.check();
            for trial in 0..3 {
                let n = nring.n_vars();
                let raws: Vec<Vec<(Vec<u32>, u64)>> = (0..n_eq)
                    .map(|_| {
                        (0..rng.gen_range(3..9))
                            .map(|_| {
                                let e: Vec<u32> = (0..n)
                                    .map(|i| {
                                        if i < nring.n_tower() {
                                            u32::from(rng.gen_bool(0.25))
                                        } else {
                                            rng.gen_range(0..3)
                                        }
                                    })
                                    .collect();
                                (e, rng.gen_range(1..p))
                            })
                            .collect()
                    })
                    .collect();
                let ninput: Vec<narrow::RPoly> = raws.iter().map(|r| nring.from_raw(r)).collect();
                let xinput: Vec<XPoly> = raws.iter().map(|r| xring.from_raw(r)).collect();
                // The two normal forms agree.
                for (a, b) in ninput.iter().zip(&xinput) {
                    assert_eq!(&xring.xpoly_from_rpoly(a), b);
                }
                let tag = format!("p {p} blocks {blocks} t {t} free {n_free} trial {trial}");
                assert_same_run(
                    &nring,
                    &xring,
                    &ninput,
                    &xinput,
                    &narrow::TowerF4Options::new(7),
                    &tag,
                );
            }
        }
    }

    // ── Towers with points ──

    fn prime_factors(mut n: u64) -> Vec<u64> {
        let mut f = Vec::new();
        let mut q = 2;
        while q * q <= n {
            if n.is_multiple_of(q) {
                f.push(q);
                while n.is_multiple_of(q) {
                    n /= q;
                }
            }
            q += 1;
        }
        if n > 1 {
            f.push(n);
        }
        f
    }

    /// An element of order exactly `n` in `F_p^*` (`n | p − 1`).
    fn root_of_unity(p: u64, n: u64) -> u64 {
        assert_eq!((p - 1) % n, 0);
        let f = fp(p);
        (2..p)
            .map(|h| f.pow(h, (p - 1) / n))
            .find(|&z| prime_factors(n).iter().all(|&q| f.pow(z, n / q) != 1))
            .unwrap()
    }

    /// One Kummer block `y_0 = ζ^k`, `y_{j+1} = y_j^{r_j}`, `y_{t-1}^{r_{t-1}} = 1`
    /// on the variables `base .. base + t`: the rules, and every point
    /// `(y_0 … y_{t−1})`, indexed by `k`.
    fn kummer_block(p: u64, radices: &[u8], base: usize) -> (Vec<PowRule>, Vec<Vec<u64>>) {
        let f = fp(p);
        let t = radices.len();
        let n: u64 = radices.iter().map(|&r| u64::from(r)).product();
        let zeta = root_of_unity(p, n);
        let rules = (0..t)
            .map(|j| PowRule::kummer(radices[j], (j + 1 < t).then_some((base + j + 1) as u8)))
            .collect();
        let points = (0..n)
            .map(|k| {
                let mut y = vec![f.pow(zeta, k)];
                for j in 1..t {
                    y.push(f.pow(y[j - 1], u64::from(radices[j - 1])));
                }
                y
            })
            .collect();
        (rules, points)
    }

    /// A random polynomial of degree at most `d` in each of `m` variables.
    fn random_raw(
        rng: &mut impl rand::Rng,
        m: usize,
        d: u32,
        terms: usize,
        p: u64,
    ) -> Vec<(Vec<u32>, u64)> {
        (0..terms)
            .map(|_| {
                let e: Vec<u32> = (0..m).map(|_| rng.gen_range(0..=d)).collect();
                (e, rng.gen_range(1..p))
            })
            .collect()
    }

    fn eval_raw(f: Fp, raw: &[(Vec<u32>, u64)], x: &[u64]) -> u64 {
        let mut acc = 0u64;
        for (e, c) in raw {
            let mut t = *c;
            for (xi, &ei) in x.iter().zip(e) {
                t = f.mul(t, f.pow(*xi, u64::from(ei)));
            }
            acc = f.add(acc, t);
        }
        acc
    }

    /// The generic F4's leading monomials of the polynomial-ring ideal
    /// `(raw system in y, y_j^{r_j} − rule_j)`, minimal generators.
    fn generic_minimal_lms(
        ring: &RadixRing,
        eqs: &[Vec<(Vec<u32>, u64)>],
    ) -> Option<BTreeSet<Vec<u32>>> {
        let (p, n) = (ring.p, ring.n_vars());
        let mut input: Vec<f4_fp::Poly> = Vec::new();
        for e in eqs {
            input.push(f4_fp::normalise(e, p, Ordering::Grevlex));
        }
        for (j, r) in ring.rules.iter().enumerate() {
            let mut terms: Vec<(Vec<u32>, u64)> = Vec::new();
            let mut lead = vec![0u32; n];
            lead[j] = u32::from(r.radix);
            terms.push((lead, 1));
            for k in 0..r.radix as usize {
                for (coef, nx) in [(r.c[k], None), (r.b[k], r.next)] {
                    if coef != 0 {
                        let mut e = vec![0u32; n];
                        e[j] += k as u32;
                        if let Some(nx) = nx {
                            e[nx as usize] += 1;
                        }
                        terms.push((e, p - coef));
                    }
                }
            }
            input.push(f4_fp::normalise(&terms, p, Ordering::Grevlex));
        }
        let rep = f4_fp::f4(&input, n, p, &F4Options::new(Ordering::Grevlex, 200));
        if rep.inconsistent {
            return None;
        }
        assert!(!rep.timed_out && rep.pairs_above_bound == 0);
        let lms: Vec<Vec<u32>> = rep
            .basis
            .iter()
            .map(|g| {
                g.iter()
                    .map(|(e, _)| e.clone())
                    .reduce(|a, b| if grevlex_gt(&a, &b) { a } else { b })
                    .unwrap()
            })
            .collect();
        Some(minimal(&lms))
    }

    fn minimal(lms: &[Vec<u32>]) -> BTreeSet<Vec<u32>> {
        lms.iter()
            .filter(|a| {
                !lms.iter()
                    .any(|b| *b != **a && b.iter().zip(a.iter()).all(|(x, y)| x <= y))
            })
            .cloned()
            .collect()
    }

    /// One planted-solution decomposition system over a Kummer tower:
    /// `m` blocks `x_i = g·y_{i,0}`, `n_eq` random equations in `x`, one
    /// root planted at the block points `plant`.
    struct Decomp {
        ring: RadixRing,
        t: usize,
        m: usize,
        g: u64,
        pts: Vec<Vec<u64>>,
        raw_x: Vec<Vec<(Vec<u32>, u64)>>,
    }

    impl Decomp {
        fn new(
            rng: &mut impl rand::Rng,
            p: u64,
            radices: &[u8],
            m: usize,
            n_eq: usize,
            d: u32,
            terms: usize,
            plant: bool,
        ) -> Self {
            let t = radices.len();
            let mut rules = Vec::new();
            let mut pts = Vec::new();
            for i in 0..m {
                let (r, q) = kummer_block(p, radices, i * t);
                rules.extend(r);
                pts = q;
            }
            let ring = RadixRing {
                p,
                rules,
                n_free: 0,
            };
            let g = rng.gen_range(1..p);
            let f = fp(p);
            let pl: Vec<usize> = (0..m).map(|_| rng.gen_range(0..pts.len())).collect();
            let raw_x = (0..n_eq)
                .map(|_| {
                    let mut raw = random_raw(rng, m, d, terms, p);
                    if plant {
                        let x: Vec<u64> = pl.iter().map(|&k| f.mul(g, pts[k][0])).collect();
                        let v = eval_raw(f, &raw, &x);
                        raw.push((vec![0; m], f.neg(v)));
                    }
                    raw
                })
                .collect();
            Decomp {
                ring,
                t,
                m,
                g,
                pts,
                raw_x,
            }
        }

        /// The equations in the ring's variables (`x_i = g·y_{i,0}`), raw.
        fn raw_y(&self) -> Vec<Vec<(Vec<u32>, u64)>> {
            let f = fp(self.ring.p);
            self.raw_x
                .iter()
                .map(|raw| {
                    raw.iter()
                        .map(|(e, c)| {
                            let mut ey = vec![0u32; self.ring.n_vars()];
                            let mut coef = *c;
                            for (i, &ei) in e.iter().enumerate() {
                                ey[i * self.t] = ei;
                                coef = f.mul(coef, f.pow(self.g, u64::from(ei)));
                            }
                            (ey, coef)
                        })
                        .collect()
                })
                .collect()
        }

        /// Every `(k_1 … k_m)` whose point satisfies the raw equations in `x`.
        fn exhaustive(&self) -> Vec<Vec<usize>> {
            let f = fp(self.ring.p);
            let n = self.pts.len();
            let mut out = Vec::new();
            let mut k = vec![0usize; self.m];
            'all: loop {
                let x: Vec<u64> = k.iter().map(|&ki| f.mul(self.g, self.pts[ki][0])).collect();
                if self.raw_x.iter().all(|r| eval_raw(f, r, &x) == 0) {
                    out.push(k.clone());
                }
                let mut i = 0;
                loop {
                    if i == self.m {
                        break 'all;
                    }
                    k[i] += 1;
                    if k[i] < n {
                        break;
                    }
                    k[i] = 0;
                    i += 1;
                }
            }
            out
        }

        fn full_point(&self, k: &[usize]) -> Vec<u64> {
            k.iter().flat_map(|&ki| self.pts[ki].clone()).collect()
        }
    }

    /// Runs the system and checks it against the exhaustive search, the
    /// generic F4 on the same ideal without the tower rewriting, and the
    /// staircase. Returns `(solutions, solving degree)`.
    fn check_decomposition(d: &Decomp, generic: bool, tag: &str) -> (usize, u32) {
        let ring = &d.ring;
        let input: Vec<XPoly> = d.raw_y().iter().map(|r| ring.from_raw(r)).collect();
        let rep = f4_radix(&input, ring, &TowerF4Options::new(200));
        assert!(!rep.timed_out && rep.pairs_above_bound == 0, "{tag}");
        let sols = d.exhaustive();
        assert_eq!(rep.inconsistent, sols.is_empty(), "{tag}");
        let generic_lms = generic.then(|| generic_minimal_lms(ring, &d.raw_y()));
        if rep.inconsistent {
            if let Some(g) = generic_lms {
                assert!(g.is_none(), "{tag}: the generic F4 found a solution");
            }
            return (0, rep.solving_degree_max);
        }
        // No solution lost: every basis element vanishes at every solution.
        for s in &sols {
            let pt = d.full_point(s);
            for b in &rep.basis {
                assert_eq!(ring.eval(b, &pt), 0, "{tag}: a solution was lost");
            }
        }
        // No solution invented: the standard monomials of the basis count
        // the points of the zero-dimensional radical ideal, and the points
        // of the tower's full variety are the block points.
        let lms: Vec<XMono> = rep.basis.iter().map(|b| b.lm().unwrap()).collect();
        assert_eq!(
            x_staircase_at_most(&lms, ring, 1 << 14),
            Some(sols.len()),
            "{tag}"
        );
        // The same monomial ideal as the generic F4's.
        if let Some(g) = generic_lms {
            let g = g.expect("the generic F4 refuted a consistent system");
            let mut mine: Vec<Vec<u32>> = lms
                .iter()
                .map(|&m| x_exponents(m, ring.n_tower(), 0))
                .collect();
            for (j, r) in ring.rules.iter().enumerate() {
                let mut e = vec![0u32; ring.n_tower()];
                e[j] = u32::from(r.radix);
                mine.push(e);
            }
            assert_eq!(minimal(&mine), g, "{tag}: leading monomial ideals differ");
        }
        (sols.len(), rep.solving_degree_max)
    }

    /// The same comparison on consistent systems with planted solutions
    /// over Kummer towers (complete runs, then a degree-cut run), where the
    /// F4 reaches a non-trivial Groebner basis instead of a refutation.
    #[test]
    fn a_radix_2_ring_gives_the_radix_2_engines_run_on_planted_systems() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(78);
        for (p, radices, m, n_eq) in [
            (97u64, vec![2u8, 2, 2], 2usize, 1usize),
            (97, vec![2, 2, 2], 3, 2),
            (193, vec![2, 2, 2, 2], 2, 1),
            (257, vec![2, 2, 2, 2, 2], 2, 1),
            (193, vec![2, 2, 2], 3, 3),
        ] {
            for trial in 0..3 {
                let d = Decomp::new(&mut rng, p, &radices, m, n_eq, 2, 5, true);
                let nring = narrow::TowerRing {
                    p,
                    rules: d
                        .ring
                        .rules
                        .iter()
                        .map(|r| narrow::SquareRule {
                            next: r.next,
                            a: r.b[1],
                            b: r.b[0],
                            c: r.c[1],
                            d: r.c[0],
                        })
                        .collect(),
                    n_free: 0,
                };
                let raws = d.raw_y();
                let ninput: Vec<narrow::RPoly> = raws.iter().map(|r| nring.from_raw(r)).collect();
                let xinput: Vec<XPoly> = raws.iter().map(|r| d.ring.from_raw(r)).collect();
                for max_degree in [200u32, 4] {
                    let tag = format!(
                        "planted p {p} t {} m {m} eq {n_eq} trial {trial} cut {max_degree}",
                        radices.len()
                    );
                    assert_same_run(
                        &nring,
                        &d.ring,
                        &ninput,
                        &xinput,
                        &narrow::TowerF4Options::new(max_degree),
                        &tag,
                    );
                }
            }
        }
    }

    #[test]
    fn a_degree_one_tower_pair_is_the_cube_rules_degree_two_row() {
        // p = 19, y_0^9 = 1 through y_0^3 = y_1, y_1^3 = 1.
        let p = 19u64;
        let (rules, pts) = kummer_block(p, &[3, 3], 0);
        let ring = RadixRing {
            p,
            rules,
            n_free: 0,
        };
        // y_0 − 1: LM y_0 (exponent 1 of 3): the tower pair is y_0² · g at
        // degree 1 + 2 = 3, and the system has the single point y_0 = 1.
        let g = ring.from_raw(&[(vec![1, 0], 1), (vec![0, 0], p - 1)]);
        let rep = f4_radix(&[g], &ring, &TowerF4Options::new(10));
        assert!(!rep.inconsistent);
        assert_eq!(rep.trace[0].degree, 3);
        assert_eq!(rep.trace[0].tower_pairs, 1);
        assert_eq!(rep.solving_degree_max, 3);
        let lms: Vec<XMono> = rep.basis.iter().map(|b| b.lm().unwrap()).collect();
        assert_eq!(x_staircase_at_most(&lms, &ring, 100), Some(1));
        assert!(pts.iter().filter(|y| y[0] == 1).count() == 1);
        // y_0 + 1 has no root in μ_9: the engine refutes it.
        let g = ring.from_raw(&[(vec![1, 0], 1), (vec![0, 0], 1)]);
        assert!(f4_radix(&[g], &ring, &TowerF4Options::new(10)).inconsistent);
    }

    #[test]
    fn radix_3_towers_refute_exactly_the_empty_systems_and_count_the_rest() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(3);
        for (p, t) in [(19u64, 1usize), (109, 2), (109, 3), (163, 4)] {
            let radices = vec![3u8; t];
            for trial in 0..8 {
                let plant = trial % 2 == 0;
                let d = Decomp::new(&mut rng, p, &radices, 2, 1, 2, 5, plant);
                check_decomposition(&d, false, &format!("p {p} t {t} trial {trial}"));
            }
        }
    }

    #[test]
    fn mixed_radix_decomposition_systems_agree_with_exhaustive_search_and_the_generic_f4() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(5);
        let cases: [(u64, &[u8]); 9] = [
            (13, &[3, 2]),
            (19, &[3, 3]),
            (37, &[2, 3, 2]),
            (37, &[3, 2, 2]),
            (61, &[2, 2, 3]),
            (73, &[3, 3, 2]),
            (73, &[2, 3, 3]),
            (109, &[3, 3, 3]),
            (109, &[3, 3]),
        ];
        let mut sd = BTreeMap::new();
        for (p, radices) in cases {
            for (m, n_eq, trial) in [(2usize, 1usize, 0usize), (2, 1, 1), (3, 2, 2), (3, 3, 3)] {
                let tag = format!("p {p} radices {radices:?} m {m} eq {n_eq} trial {trial}");
                let d = Decomp::new(&mut rng, p, radices, m, n_eq, 2, 4, trial != 1);
                // The generic F4 (no tower rewriting) is the slow reference: up to 7 variables.
                let (nsol, deg) = check_decomposition(&d, m * radices.len() <= 7, &tag);
                sd.insert(tag, (nsol, deg));
            }
        }
        // An engineering smoke output (not a result): printed with `--nocapture`.
        for (tag, (nsol, deg)) in &sd {
            println!("smoke {tag}: solutions {nsol}, D {deg}");
        }
    }

    /// The radix-4 control: `y^4 = y_next` is the radix-2 tower with every
    /// other level eliminated. Same ideal, same solution set.
    #[test]
    fn the_radix_4_control_has_the_radix_2_solution_set() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(9);
        let cases: [(u64, usize, usize); 4] = [(17, 2, 2), (97, 2, 2), (257, 2, 2), (257, 3, 2)];
        for (p, t4, m) in cases {
            let n_bits_ok = (p - 1) % (1u64 << (2 * t4)) == 0;
            assert!(n_bits_ok);
            let r4 = vec![4u8; t4];
            let r2 = vec![2u8; 2 * t4];
            for trial in 0..6 {
                let seed = rng.gen::<u64>();
                let plant = trial % 3 != 1;
                // The same random equations in x (same seed) on both presentations.
                let d4 = Decomp::new(
                    &mut rand::rngs::StdRng::seed_from_u64(seed),
                    p,
                    &r4,
                    m,
                    1,
                    3,
                    5,
                    plant,
                );
                let d2 = Decomp::new(
                    &mut rand::rngs::StdRng::seed_from_u64(seed),
                    p,
                    &r2,
                    m,
                    1,
                    3,
                    5,
                    plant,
                );
                assert_eq!(d4.g, d2.g);
                assert_eq!(d4.raw_x, d2.raw_x);
                let tag = format!("p {p} t4 {t4} m {m} trial {trial}");
                let (n4, deg4) = check_decomposition(&d4, false, &tag);
                let (n2, deg2) = check_decomposition(&d2, false, &tag);
                // Same solution set: the exhaustive searches agree on the
                // block exponents (ζ₄-power for ζ₂-power: the same x), and
                // both engines' staircases count it.
                let x_of = |d: &Decomp, k: &[usize]| -> Vec<u64> {
                    k.iter().map(|&ki| fp(p).mul(d.g, d.pts[ki][0])).collect()
                };
                let s4: BTreeSet<Vec<u64>> = d4.exhaustive().iter().map(|k| x_of(&d4, k)).collect();
                let s2: BTreeSet<Vec<u64>> = d2.exhaustive().iter().map(|k| x_of(&d2, k)).collect();
                assert_eq!(s4, s2, "{tag}");
                assert_eq!(n4, n2, "{tag}");
                // The radix-2 solution lifts to the radix-4 ring's by keeping
                // every other level.
                for k in d2.exhaustive() {
                    let full2 = d2.full_point(&k);
                    let kk4: Vec<usize> = (0..m)
                        .map(|i| {
                            let x = d2.pts[k[i]][0];
                            d4.pts.iter().position(|y| y[0] == x).unwrap()
                        })
                        .collect();
                    let full4 = d4.full_point(&kk4);
                    let picked: Vec<u64> = (0..m)
                        .flat_map(|i| (0..t4).map(move |l| (i, l)))
                        .map(|(i, l)| full2[i * 2 * t4 + 2 * l])
                        .collect();
                    assert_eq!(picked, full4, "{tag}");
                }
                println!("smoke {tag}: solutions {n2}, D radix-2 {deg2}, D radix-4 {deg4}");
            }
        }
    }

    /// A Dickson (Chebyshev) radix-3 tower `y_{j+1} = y_j³ − 3y_j`, which
    /// exercises the rule's `y_j` and `y_next` terms: `V = {ζ^k u + ζ^{-k} u⁻¹}`.
    #[test]
    fn a_radix_3_dickson_tower_agrees_with_exhaustive_search() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(21);
        for (p, t) in [(109u64, 2usize), (163, 3)] {
            let f = fp(p);
            let n = 3u64.pow(t as u32);
            let zeta = root_of_unity(p, n);
            // A u with 3^t distinct points.
            let (u, pts) = (2..p)
                .find_map(|u| {
                    let pts: BTreeSet<u64> = (0..n)
                        .map(|k| {
                            f.add(
                                f.mul(f.pow(zeta, k), u),
                                f.mul(f.pow(zeta, n - k), f.inv(u)),
                            )
                        })
                        .collect();
                    (pts.len() as u64 == n).then_some((u, pts))
                })
                .unwrap();
            let c_top = f.add(f.pow(u, n), f.inv(f.pow(u, n)));
            // y_j³ = 3·y_j + y_{j+1} on every level, y³ = 3y + c_top on the top.
            let blocks = 2usize;
            let mut rules = Vec::new();
            for b in 0..blocks {
                for j in 0..t {
                    let top = j + 1 == t;
                    let mut r = PowRule::cube(
                        (!top).then_some((b * t + j + 1) as u8),
                        [if top { c_top } else { 0 }, 3, 0],
                        [u64::from(!top), 0, 0],
                    );
                    r.c[2] = 0;
                    rules.push(r);
                }
            }
            let ring = RadixRing {
                p,
                rules,
                n_free: 0,
            };
            ring.check();
            // The tower's points: y_0 ∈ V, y_{j+1} = y_j³ − 3y_j.
            let chain = |y0: u64| -> Vec<u64> {
                let mut y = vec![y0];
                for j in 1..t {
                    let a = y[j - 1];
                    y.push(sub(f, f.pow(a, 3), f.mul(3, a)));
                }
                y
            };
            let vs: Vec<u64> = pts.into_iter().collect();
            for y0 in &vs {
                let y = chain(*y0);
                let top = sub(f, f.pow(y[t - 1], 3), f.mul(3, y[t - 1]));
                assert_eq!(top, c_top, "the tower's top relation");
            }
            for trial in 0..6 {
                let raw: Vec<(Vec<u32>, u64)> = (0..5)
                    .map(|_| {
                        let mut e = vec![0u32; ring.n_vars()];
                        e[0] = rng.gen_range(0..=3);
                        e[t] = rng.gen_range(0..=3);
                        (e, rng.gen_range(1..p))
                    })
                    .collect();
                let mut raw = raw;
                if trial % 2 == 0 {
                    // plant a root
                    let (a, b) = (
                        vs[rng.gen_range(0..vs.len())],
                        vs[rng.gen_range(0..vs.len())],
                    );
                    let pt = [chain(a), chain(b)].concat();
                    let g = ring.from_raw(&raw);
                    let v = ring.eval(&g, &pt);
                    raw.push((vec![0; ring.n_vars()], f.neg(v)));
                }
                let g = ring.from_raw(&raw);
                let rep = f4_radix(std::slice::from_ref(&g), &ring, &TowerF4Options::new(100));
                let mut zeros = Vec::new();
                for a in &vs {
                    for b in &vs {
                        let pt = [chain(*a), chain(*b)].concat();
                        if ring.eval(&g, &pt) == 0 {
                            zeros.push(pt);
                        }
                    }
                }
                assert_eq!(rep.inconsistent, zeros.is_empty(), "p {p} trial {trial}");
                if !rep.inconsistent {
                    for z in &zeros {
                        for b in &rep.basis {
                            assert_eq!(ring.eval(b, z), 0);
                        }
                    }
                    let lms: Vec<XMono> = rep.basis.iter().map(|b| b.lm().unwrap()).collect();
                    assert_eq!(x_staircase_at_most(&lms, &ring, 1 << 14), Some(zeros.len()));
                }
                // The generic F4 on the polynomial-ring ideal agrees.
                let raw_y: Vec<Vec<(Vec<u32>, u64)>> = vec![raw];
                let gl = generic_minimal_lms(&ring, &raw_y);
                assert_eq!(gl.is_none(), rep.inconsistent);
                if let Some(gl) = gl {
                    let lms: Vec<XMono> = rep.basis.iter().map(|b| b.lm().unwrap()).collect();
                    let mut mine: Vec<Vec<u32>> = lms
                        .iter()
                        .map(|&m| x_exponents(m, ring.n_tower(), 0))
                        .collect();
                    for (j, r) in ring.rules.iter().enumerate() {
                        let mut e = vec![0u32; ring.n_tower()];
                        e[j] = u32::from(r.radix);
                        mine.push(e);
                    }
                    assert_eq!(minimal(&mine), gl);
                }
            }
        }
    }

    #[test]
    fn a_free_variable_is_eliminated_like_a_chain_link_on_a_radix_3_tower() {
        // u = x_1·x_2 on μ_9 × μ_9 (p = 19), then u² − 5 against x_1 + x_2:
        // brute force over (x_1, x_2, u).
        let p = 19u64;
        let f = fp(p);
        let (r0, pts) = kummer_block(p, &[3, 3], 0);
        let (r1, _) = kummer_block(p, &[3, 3], 2);
        let ring = RadixRing {
            p,
            rules: [r0, r1].concat(),
            n_free: 1,
        };
        let n = ring.n_vars(); // 5: y00 y01 y10 y11 u
        let sys = [
            // u − x1 x2
            vec![(vec![0, 0, 0, 0, 1], 1), (vec![1, 0, 1, 0, 0], p - 1)],
            // u² − 5
            vec![(vec![0, 0, 0, 0, 2], 1), (vec![0; n], p - 5)],
        ];
        let input: Vec<XPoly> = sys.iter().map(|r| ring.from_raw(r)).collect();
        let rep = f4_radix(&input, &ring, &TowerF4Options::new(100));
        assert!(!rep.timed_out && rep.pairs_above_bound == 0);
        let mut zeros = Vec::new();
        for a in &pts {
            for b in &pts {
                for u in 0..p {
                    let pt = [a.clone(), b.clone(), vec![u]].concat();
                    if sys.iter().all(|r| {
                        let g = ring.from_raw(r);
                        ring.eval(&g, &pt) == 0
                    }) {
                        zeros.push(pt);
                    }
                }
            }
        }
        assert_eq!(rep.inconsistent, zeros.is_empty());
        let _ = f;
        if !rep.inconsistent {
            for z in &zeros {
                for b in &rep.basis {
                    assert_eq!(ring.eval(b, z), 0);
                }
            }
            let lms: Vec<XMono> = rep.basis.iter().map(|b| b.lm().unwrap()).collect();
            assert_eq!(x_staircase_at_most(&lms, &ring, 1 << 12), Some(zeros.len()));
        }
        println!(
            "smoke free-variable radix-3: solutions {}, D {}",
            zeros.len(),
            rep.solving_degree_max
        );
    }

    #[test]
    fn the_run_does_not_depend_on_the_product_cache() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(31);
        let d = Decomp::new(&mut rng, 73, &[3, 3, 2], 2, 1, 2, 4, true);
        let input: Vec<XPoly> = d.raw_y().iter().map(|r| d.ring.from_raw(r)).collect();
        let opts = TowerF4Options::new(100);
        let a = f4_radix_cached(&input, &d.ring, &opts, PRODUCT_CACHE_TERMS);
        let b = f4_radix_cached(&input, &d.ring, &opts, 0);
        let c = f4_radix_cached(&input, &d.ring, &opts, 37);
        for r in [&b, &c] {
            assert_eq!(a.basis, r.basis);
            assert_eq!(a.muladds, r.muladds);
            assert_eq!(a.steps, r.steps);
            assert_eq!(a.solving_degree_max, r.solving_degree_max);
        }
    }

    #[test]
    fn the_stop_and_the_degree_bound_behave_like_the_radix_2_engines() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(41);
        let d = Decomp::new(&mut rng, 73, &[3, 2, 3], 2, 1, 2, 5, true);
        let input: Vec<XPoly> = d.raw_y().iter().map(|r| d.ring.from_raw(r)).collect();
        let full = f4_radix(&input, &d.ring, &TowerF4Options::new(100));
        assert!(!full.inconsistent);
        // A degree bound below the solving degree leaves pairs.
        let cut = f4_radix(&input, &d.ring, &TowerF4Options::new(1));
        assert!(cut.pairs_above_bound > 0 || cut.steps == 0);
        // The staircase stop keeps every solution.
        let sols = d.exhaustive();
        let mut o = TowerF4Options::new(100);
        o.stop_staircase = Some(1 << 12);
        let stopped = f4_radix(&input, &d.ring, &o);
        for s in &sols {
            let pt = d.full_point(s);
            for b in &stopped.basis {
                assert_eq!(d.ring.eval(b, &pt), 0);
            }
        }
        // Weights and traces are the radix-2 engine's.
        let mut w = TowerF4Options::new(10);
        w.record_trace = true;
        assert!(std::panic::catch_unwind(|| f4_radix(&[], &d.ring, &w)).is_err());
    }

    #[test]
    fn the_ring_check_refuses_what_the_engine_does_not_handle() {
        let bad = |r: PowRule, n: usize| {
            let ring = RadixRing {
                p: 31,
                rules: vec![r; n],
                n_free: 0,
            };
            std::panic::catch_unwind(|| ring.check()).is_err()
        };
        assert!(bad(PowRule::kummer(5, None), 1));
        assert!(bad(PowRule::kummer(1, None), 1));
        // A next pointing backwards or at itself.
        assert!(bad(PowRule::kummer(3, Some(0)), 2));
        // A top rule with a next term.
        let mut r = PowRule::kummer(3, None);
        r.b[0] = 1;
        assert!(bad(r, 1));
        // A term above y^(radix − 1).
        let mut r = PowRule::kummer(3, None);
        r.c[3] = 1;
        assert!(bad(r, 1));
        assert!(!bad(PowRule::kummer(3, None), 1));
    }

    /// Reviewer's adversarial test: mixed 2,3,2 and 3,2,3,2 towers over
    /// primes where 12 (resp. 36) divides p - 1, planted and unplanted,
    /// against exhaustive search, the staircase and the generic F4.
    #[test]
    fn review_mixed_2_3_2_towers_over_primes_with_12_dividing_p_minus_1() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(20261115);
        let cases: [(u64, &[u8]); 7] = [
            (61, &[2, 3, 2]),
            (97, &[2, 3, 2]),
            (109, &[2, 3, 2]),
            (157, &[2, 3, 2]),
            (37, &[3, 2, 3, 2]),
            (73, &[3, 2, 3, 2]),
            (109, &[2, 3, 2, 3]),
        ];
        for (p, radices) in cases {
            assert_eq!(
                (p - 1) % radices.iter().map(|&r| u64::from(r)).product::<u64>(),
                0
            );
            for (m, n_eq, d, trial) in [
                (2usize, 1usize, 3u32, 0usize),
                (2, 2, 2, 1),
                (3, 2, 2, 2),
                (3, 3, 2, 3),
            ] {
                let tag = format!("review p {p} {radices:?} m {m} eq {n_eq} trial {trial}");
                let dd = Decomp::new(&mut rng, p, radices, m, n_eq, d, 4, trial != 1);
                check_decomposition(&dd, m * radices.len() <= 7, &tag);
            }
        }
    }

    /// Reviewer's adversarial test: multiplication in a random mixed ring
    /// with every b coefficient non-zero is commutative, associative and
    /// equals the independent dense reduction (sum of term-by-term products
    /// against the dense product reduced by the generic rewriting).
    #[test]
    fn review_products_are_associative_and_match_dense_reduction() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(424242);
        let ring = random_ring(&mut rng, 101, &[2, 3, 3, 2, 3], 1, 0);
        let n = ring.n_vars();
        let rnd_raw = |rng: &mut rand::rngs::StdRng| -> Vec<(Vec<u32>, u64)> {
            (0..4)
                .map(|_| {
                    (
                        (0..n).map(|_| rng.gen_range(0..2u32)).collect(),
                        rng.gen_range(1..101u64),
                    )
                })
                .collect()
        };
        for _ in 0..200 {
            let (a, b) = (rnd_raw(&mut rng), rnd_raw(&mut rng));
            let pa = ring.from_raw(&a);
            // a * c as a sum over the terms of b of (term * a) then times c monomials
            let mut ab: BTreeMap<Vec<u32>, u64> = BTreeMap::new();
            let f = fp(101);
            for (e, cf) in &b {
                let m = ring.from_raw(&[(e.clone(), 1)]);
                let t = ring.mul_mono(&pa, m.monos[0]);
                for (&mm, &cc) in t.monos.iter().zip(&t.coefs) {
                    let k = x_exponents(mm, ring.n_tower(), 0);
                    let v = ab.entry(k).or_insert(0);
                    *v = f.add(*v, f.mul(*cf, u64::from(cc)));
                }
            }
            ab.retain(|_, v| *v != 0);
            // dense reference: multiply the raw polynomials, reduce
            let mut dense = Vec::new();
            for (ea, ca) in &a {
                for (eb, cb) in &b {
                    let e: Vec<u32> = ea.iter().zip(eb).map(|(x, y)| x + y).collect();
                    dense.push((e, f.mul(*ca, *cb)));
                }
            }
            assert_eq!(ab, reference_nf(&ring, &dense));
        }
    }
}
