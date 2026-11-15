//! # Genus ladder with intermediate-divisor harvesting.
//!
//! Protocol: `research/genus_ladder_harvest_20261115/PROTOCOL.md`.  Every
//! figure this module produces is `[pcc:unverified]`.
//!
//! [`run_ic_harvest`] is `genus_ladder::run_ic` (structure, random streams
//! and counters copied, so that with `harvest = false` it reproduces the
//! frozen instrument count for count; the test `harvest_off_is_run_ic`
//! checks that) with one addition.
//!
//! ## The harvested relation
//!
//! At genus `g ≥ 3` the composition of two reduced divisors in general
//! position has `deg u = 2g`.  Cantor's first reduction step forms
//! `u′ = monic((f − v²)/u)`, of degree `2g − 2` when `deg v = 2g − 1`, and
//! `v′ = −v mod u′`.  Since `div(y − v) = D(u, v) + D(u′, v mod u′) − n·∞`,
//! the divisor `D(u′, v′) = D(u′, −v mod u′)` is linearly equivalent to
//! `D(u, v)`, i.e. to the reduced sum itself: the step does not change the
//! class.  (`D(u′, +v mod u′)` is the negative.)  So when `u′` splits into
//! linear factors, the places `(x, v′(x))` over its roots sum to the walk's
//! current element `cur = a·G + b·T`, and the row has exactly the right-hand
//! side of a main relation (`rhs = h·a`, `tcoef = −h·b`).  `verify_relations`
//! checks every harvested relation by group arithmetic (`Σ places = cur`).
//!
//! The intermediate is a by-product of the group operation that produced
//! `cur`: no extra group operation and no extra `F_p` work in the group law.
//! Its smoothness work (parity prefilter, `x^p` test, roots) is charged to
//! `rel` like the main candidate's.  At genus 2 the first step already
//! lands at degree `g` (no intermediate); at genus 4 the intermediate has
//! degree 6 and at most [`REL_FB`] of its places may lie in the base.
//!
//! Only the trial's own addition (`cur + jump`, or the restart's
//! `aG + bT`) is harvested, not the doublings inside scalar multiplications.
//!
//! The structured-elimination option of `IcParams` is not supported here
//! (`prm.sge` must be `None`).

use std::cell::Cell;
use std::collections::HashMap;

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use super::chained_transfer::ic_control::{
    addm, build_rows, mulmod, peel, subm, wiedemann, Dsu, Nat, Rel, REL_FB,
};
use super::genus_ladder::{
    disc_character, Arm, IcParams, IcResult, Instance, JCurve, JDiv, MAX_GENUS,
};
use super::ic_boundary::{CountedGroup, GroupOps};
use super::residual_walk::mix64;

/// Polynomial capacity (as in `genus_ladder`).
const CAP: usize = 24;

// ── polynomials over F_p on the stack (copied from `genus_ladder`) ─────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Poly {
    c: [u64; CAP],
    n: usize,
}

impl Poly {
    const ZERO: Poly = Poly { c: [0; CAP], n: 0 };

    fn constant(a: u64) -> Poly {
        let mut q = Poly::ZERO;
        if a != 0 {
            q.c[0] = a;
            q.n = 1;
        }
        q
    }
    fn from_slice(s: &[u64]) -> Poly {
        assert!(s.len() <= CAP, "polynomial past capacity");
        let mut q = Poly::ZERO;
        q.c[..s.len()].copy_from_slice(s);
        q.n = s.len();
        q.trim();
        q
    }
    fn trim(&mut self) {
        while self.n > 0 && self.c[self.n - 1] == 0 {
            self.n -= 1;
        }
    }
    fn is_zero(&self) -> bool {
        self.n == 0
    }
    fn deg(&self) -> usize {
        self.n.saturating_sub(1)
    }
    fn lead(&self) -> u64 {
        if self.n == 0 {
            0
        } else {
            self.c[self.n - 1]
        }
    }
}

/// `F_p` with the same tally of multiplications and inversions as
/// `genus_ladder` (the operation sequence is identical).
#[derive(Clone, Debug)]
struct Fk {
    p: u64,
    mul: Cell<u64>,
    inv: Cell<u64>,
}

impl Fk {
    #[inline]
    fn tick(&self, n: usize) {
        self.mul.set(self.mul.get() + n as u64);
    }
    fn inv(&self, a: u64) -> u64 {
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
    fn add(&self, a: &Poly, b: &Poly) -> Poly {
        let p = self.p;
        let mut q = Poly::ZERO;
        let n = a.n.max(b.n);
        for i in 0..n {
            let s = a.c[i] + b.c[i];
            q.c[i] = if s >= p { s - p } else { s };
        }
        q.n = n;
        q.trim();
        q
    }
    fn sub(&self, a: &Poly, b: &Poly) -> Poly {
        let p = self.p;
        let mut q = Poly::ZERO;
        let n = a.n.max(b.n);
        for i in 0..n {
            q.c[i] = if a.c[i] >= b.c[i] {
                a.c[i] - b.c[i]
            } else {
                a.c[i] + p - b.c[i]
            };
        }
        q.n = n;
        q.trim();
        q
    }
    fn neg(&self, a: &Poly) -> Poly {
        let mut q = *a;
        for i in 0..a.n {
            q.c[i] = (self.p - a.c[i]) % self.p;
        }
        q
    }
    fn scale(&self, a: &Poly, s: u64) -> Poly {
        let mut q = Poly::ZERO;
        for i in 0..a.n {
            q.c[i] = a.c[i] * s % self.p;
        }
        self.tick(a.n);
        q.n = a.n;
        q.trim();
        q
    }
    fn mul(&self, a: &Poly, b: &Poly) -> Poly {
        if a.is_zero() || b.is_zero() {
            return Poly::ZERO;
        }
        let n = a.n + b.n - 1;
        assert!(n <= CAP, "polynomial product past capacity");
        let mut acc = [0u128; CAP];
        for i in 0..a.n {
            let ai = a.c[i] as u128;
            for j in 0..b.n {
                acc[i + j] += ai * b.c[j] as u128;
            }
        }
        self.tick(a.n * b.n);
        let mut q = Poly::ZERO;
        let p = self.p as u128;
        for i in 0..n {
            q.c[i] = (acc[i] % p) as u64;
        }
        q.n = n;
        q.trim();
        q
    }
    fn divrem(&self, a: &Poly, b: &Poly) -> (Poly, Poly) {
        assert!(!b.is_zero(), "division by the zero polynomial");
        let p = self.p;
        let mut r = *a;
        let mut q = Poly::ZERO;
        if a.n < b.n {
            return (q, r);
        }
        let monic = b.lead() == 1;
        let li = if monic { 1 } else { self.inv(b.lead()) };
        let db = b.deg();
        while r.n >= b.n && !r.is_zero() {
            let shift = r.deg() - db;
            let coef = if monic {
                r.lead()
            } else {
                self.tick(1);
                r.lead() * li % p
            };
            q.c[shift] = coef;
            for j in 0..db {
                let t = coef * b.c[j] % p;
                let x = r.c[shift + j];
                r.c[shift + j] = if x >= t { x - t } else { x + p - t };
            }
            self.tick(db);
            r.c[shift + db] = 0;
            r.trim();
        }
        q.n = a.n - b.n + 1;
        q.trim();
        (q, r)
    }
    fn rem(&self, a: &Poly, b: &Poly) -> Poly {
        self.divrem(a, b).1
    }
    fn monic(&self, a: &Poly) -> Poly {
        if a.is_zero() || a.lead() == 1 {
            return *a;
        }
        self.scale(a, self.inv(a.lead()))
    }
    fn xgcd(&self, a: &Poly, b: &Poly) -> (Poly, Poly, Poly) {
        let (mut r0, mut r1) = (*a, *b);
        let (mut s0, mut s1) = (Poly::constant(1), Poly::ZERO);
        let (mut t0, mut t1) = (Poly::ZERO, Poly::constant(1));
        while !r1.is_zero() {
            let (q, r) = self.divrem(&r0, &r1);
            r0 = r1;
            r1 = r;
            let s = self.sub(&s0, &self.mul(&q, &s1));
            s0 = s1;
            s1 = s;
            let t = self.sub(&t0, &self.mul(&q, &t1));
            t0 = t1;
            t1 = t;
        }
        if r0.is_zero() || r0.lead() == 1 {
            return (r0, s0, t0);
        }
        let li = self.inv(r0.lead());
        (
            self.scale(&r0, li),
            self.scale(&s0, li),
            self.scale(&t0, li),
        )
    }
}

// ── the curve, with the first reduction step exposed ────────────────────

/// A semi-reduced divisor `(u, v)`: `u` monic (lowest degree first, with
/// its leading 1), `deg v < deg u`, `u | f − v²`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemiDiv {
    pub u: Vec<u64>,
    pub v: Vec<u64>,
}

impl SemiDiv {
    pub fn deg(&self) -> usize {
        self.u.len() - 1
    }
}

/// `y² = f(x)` with Cantor's arithmetic copied from `genus_ladder::JCurve`
/// (same operation sequence, same field tally), plus
/// [`HCurve::cantor_with_intermediate`].
#[derive(Clone, Debug)]
pub struct HCurve {
    k: Fk,
    genus: u8,
    f: Poly,
}

impl HCurve {
    /// The same curve as `c`, with a fresh field tally.
    pub fn from_jcurve(c: &JCurve) -> HCurve {
        HCurve {
            k: Fk {
                p: c.p(),
                mul: Cell::new(0),
                inv: Cell::new(0),
            },
            genus: c.genus() as u8,
            f: Poly::from_slice(&c.f_coeffs()),
        }
    }
    pub fn p(&self) -> u64 {
        self.k.p
    }
    pub fn genus(&self) -> u32 {
        u32::from(self.genus)
    }
    pub fn fmul(&self) -> u64 {
        self.k.mul.get()
    }
    pub fn finv(&self) -> u64 {
        self.k.inv.get()
    }

    fn polys(&self, d: &JDiv) -> (Poly, Poly) {
        let n = d.deg as usize;
        let mut u = Poly::ZERO;
        let mut v = Poly::ZERO;
        for i in 0..n {
            u.c[i] = u64::from(d.u[i]);
            v.c[i] = u64::from(d.v[i]);
        }
        u.c[n] = 1;
        u.n = n + 1;
        v.n = n;
        v.trim();
        (u, v)
    }

    fn pack(&self, u: &Poly, v: &Poly) -> JDiv {
        debug_assert!(u.lead() == 1 && u.deg() <= self.genus as usize);
        debug_assert!(v.is_zero() || v.deg() < u.deg());
        let mut d = JDiv {
            deg: u.deg() as u8,
            ..JDiv::IDENTITY
        };
        for i in 0..u.deg() {
            d.u[i] = u.c[i] as u32;
        }
        for i in 0..v.n {
            d.v[i] = v.c[i] as u32;
        }
        d
    }

    pub fn u_coeffs(&self, d: &JDiv) -> Vec<u64> {
        let mut out: Vec<u64> = d.u[..d.deg as usize]
            .iter()
            .map(|&c| u64::from(c))
            .collect();
        out.push(1);
        out
    }
    pub fn v_coeffs(&self, d: &JDiv) -> Vec<u64> {
        d.v[..d.deg as usize]
            .iter()
            .map(|&c| u64::from(c))
            .collect()
    }

    /// `[P] − [∞]` for an affine point (uncounted check, as in `JCurve`).
    pub fn point(&self, x: u64, y: u64) -> Option<JDiv> {
        let p = self.k.p;
        let mut fx = 0u64;
        for i in (0..self.f.n).rev() {
            fx = (fx * x + self.f.c[i]) % p;
        }
        if x >= p || y >= p || y * y % p != fx {
            return None;
        }
        let mut d = JDiv {
            deg: 1,
            ..JDiv::IDENTITY
        };
        d.u[0] = ((p - x) % p) as u32;
        d.v[0] = y as u32;
        Some(d)
    }

    pub fn key_of(&self, d: &JDiv) -> u64 {
        let mut h = 0x243F_6A88_85A3_08D3u64 ^ u64::from(d.deg);
        for i in 0..d.deg as usize {
            h = mix64(h ^ (u64::from(d.u[i]) << 32 | u64::from(d.v[i])));
        }
        h
    }

    pub fn negate(&self, d: JDiv) -> JDiv {
        let p = self.k.p;
        let mut n = d;
        for i in 0..d.deg as usize {
            n.v[i] = ((p - u64::from(d.v[i])) % p) as u32;
        }
        n
    }

    /// Cantor's composition and reduction (identical to `JCurve::cantor`).
    pub fn cantor(&self, a: JDiv, b: JDiv) -> JDiv {
        self.cantor_inner(a, b, false).0
    }

    /// Cantor's composition and reduction, and the semi-reduced divisor
    /// after the first reduction step when the composition had degree `2g`
    /// and that step gave degree `2g − 2 > g` (genus `≥ 3`).  The returned
    /// intermediate is linearly equivalent to the reduced sum (module
    /// notes).  The field work is that of [`HCurve::cantor`], tally for tally.
    pub fn cantor_with_intermediate(&self, a: JDiv, b: JDiv) -> (JDiv, Option<SemiDiv>) {
        let (d, semi) = self.cantor_inner(a, b, true);
        let semi = semi.map(|(u, v)| SemiDiv {
            u: u.c[..u.n].to_vec(),
            v: v.c[..u.deg()].to_vec(),
        });
        (d, semi)
    }

    fn cantor_inner(&self, a: JDiv, b: JDiv, capture: bool) -> (JDiv, Option<(Poly, Poly)>) {
        if a.is_identity() {
            return (b, None);
        }
        if b.is_identity() {
            return (a, None);
        }
        let k = &self.k;
        let (u1, v1) = self.polys(&a);
        let (u2, v2) = self.polys(&b);
        let (d1, e1, e2) = k.xgcd(&u1, &u2);
        let (u, v) = if d1.n == 1 {
            let u = k.mul(&u1, &u2);
            let t = k.mul(&k.mul(&e1, &u1), &k.sub(&v2, &v1));
            (u, k.rem(&k.add(&v1, &t), &u))
        } else {
            let (d, c1, c2) = k.xgcd(&d1, &k.add(&v1, &v2));
            let s1 = k.mul(&c1, &e1);
            let s2 = k.mul(&c1, &e2);
            let (u, ur) = k.divrem(&k.mul(&u1, &u2), &k.mul(&d, &d));
            debug_assert!(ur.is_zero(), "u₁u₂/d² is exact");
            let t1 = k.mul(&k.mul(&s1, &u1), &v2);
            let t2 = k.mul(&k.mul(&s2, &u2), &v1);
            let t3 = k.mul(&c2, &k.add(&k.mul(&v1, &v2), &self.f));
            let (vq, vr) = k.divrem(&k.add(&k.add(&t1, &t2), &t3), &d);
            debug_assert!(vr.is_zero(), "the v numerator divides by d");
            (u, k.rem(&vq, &u))
        };
        self.reduce(u, v, capture)
    }

    fn reduce(&self, mut u: Poly, mut v: Poly, capture: bool) -> (JDiv, Option<(Poly, Poly)>) {
        let k = &self.k;
        let g = self.genus as usize;
        let d0 = u.deg();
        let mut first = true;
        let mut semi = None;
        while u.deg() > g {
            let t = k.sub(&self.f, &k.mul(&v, &v));
            let (q, r) = k.divrem(&t, &u);
            debug_assert!(r.is_zero(), "(f − v²)/u is exact");
            let un = k.monic(&q);
            v = k.rem(&k.neg(&v), &un);
            u = un;
            if capture && first && d0 == 2 * g && u.deg() == 2 * g - 2 && u.deg() > g {
                semi = Some((u, v));
            }
            first = false;
        }
        let u = k.monic(&u);
        (self.pack(&u, &v), semi)
    }

    /// `[k]D`, right-to-left (the certificate path).
    pub fn mul_rtl(&self, d: JDiv, mut k: u64) -> JDiv {
        let mut acc = JDiv::IDENTITY;
        let mut base = d;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.cantor(acc, base);
            }
            k >>= 1;
            if k > 0 {
                base = self.cantor(base, base);
            }
        }
        acc
    }

    /// One counted group operation that also returns the intermediate.
    pub fn add_with_intermediate(
        &self,
        ops: &mut GroupOps,
        a: JDiv,
        b: JDiv,
    ) -> (JDiv, Option<SemiDiv>) {
        if a == b {
            ops.doubles += 1;
        } else {
            ops.adds += 1;
        }
        self.cantor_with_intermediate(a, b)
    }
}

impl CountedGroup for HCurve {
    type Elt = JDiv;
    fn identity(&self) -> JDiv {
        JDiv::IDENTITY
    }
    fn is_identity(&self, d: &JDiv) -> bool {
        d.is_identity()
    }
    fn add(&self, ops: &mut GroupOps, a: JDiv, b: JDiv) -> JDiv {
        if a == b {
            ops.doubles += 1;
        } else {
            ops.adds += 1;
        }
        self.cantor(a, b)
    }
    fn double(&self, ops: &mut GroupOps, a: JDiv) -> JDiv {
        ops.doubles += 1;
        self.cantor(a, a)
    }
    fn neg(&self, a: JDiv) -> JDiv {
        self.negate(a)
    }
    fn key(&self, d: &JDiv) -> u64 {
        self.key_of(d)
    }
}

// ── smoothness tests up to degree 2g − 2 (copied from `genus_ladder`) ──
//
// `genus_ladder::splits_completely` and `roots_of` hold residues in arrays of
// `MAX_GENUS` entries; an intermediate at genus 4 has degree 6.  These copies
// are the same arithmetic with room for degree [`RCAP`], counted identically.

/// Largest degree of a polynomial tested for splitting (`2·MAX_GENUS − 2`).
const RCAP: usize = 2 * MAX_GENUS - 2;

type Res = [u64; RCAP];

struct ModU {
    p: u64,
    d: usize,
    u: Res,
}

impl ModU {
    fn new(p: u64, u: &[u64]) -> ModU {
        let d = u.len() - 1;
        assert!((1..=RCAP).contains(&d), "degree {d} past capacity");
        let mut uu = [0u64; RCAP];
        uu[..d].copy_from_slice(&u[..d]);
        ModU { p, d, u: uu }
    }
    fn mul(&self, a: &Res, b: &Res, n: &mut Nat) -> Res {
        let (p, d) = (self.p, self.d);
        let mut t = [0u64; 2 * RCAP];
        for i in 0..d {
            for j in 0..d {
                t[i + j] = (t[i + j] + a[i] * b[j]) % p;
            }
        }
        n.mul += (d * d) as u64;
        n.add += (d * d) as u64;
        for k in (d..(2 * d).saturating_sub(1)).rev() {
            let c = t[k];
            for j in 0..d {
                t[k - d + j] = (t[k - d + j] + (p - self.u[j]) * c) % p;
            }
            n.mul += d as u64;
            n.add += d as u64;
        }
        let mut out = [0u64; RCAP];
        out[..d].copy_from_slice(&t[..d]);
        out
    }
    fn mul_xpa(&self, a: &Res, aa: u64, n: &mut Nat) -> Res {
        let (p, d) = (self.p, self.d);
        let top = a[d - 1];
        let mut out = [0u64; RCAP];
        out[1..d].copy_from_slice(&a[..d - 1]);
        for j in 0..d {
            out[j] = (out[j] + (p - self.u[j]) * top + aa * a[j]) % p;
        }
        let work = if aa == 0 { d } else { 2 * d } as u64;
        n.mul += work;
        n.add += work;
        out
    }
    fn base(&self, aa: u64) -> Res {
        let mut b = [0u64; RCAP];
        if self.d == 1 {
            b[0] = (aa + self.p - self.u[0]) % self.p;
        } else {
            b[0] = aa;
            b[1] = 1;
        }
        b
    }
    fn pow_xpa(&self, aa: u64, e: u64, n: &mut Nat) -> Res {
        let mut res = self.base(aa);
        let bits = 64 - e.leading_zeros();
        for i in (0..bits - 1).rev() {
            res = self.mul(&res, &res, n);
            if (e >> i) & 1 == 1 {
                res = self.mul_xpa(&res, aa, n);
            }
        }
        res
    }
}

/// `u | x^p − x` for monic `u` of degree `1..=RCAP`.
pub fn splits_completely(p: u64, u: &[u64], n: &mut Nat) -> bool {
    let ctx = ModU::new(p, u);
    let xp = ctx.pow_xpa(0, p, n);
    xp[..ctx.d] == ctx.base(0)[..ctx.d]
}

fn vtrim(a: &mut Vec<u64>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

fn pow_mod(mut a: u64, mut e: u64, p: u64) -> u64 {
    let mut r = 1u64;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % p;
        }
        a = a * a % p;
        e >>= 1;
    }
    r
}

fn cinv(p: u64, a: u64, n: &mut Nat) -> u64 {
    let e = p - 2;
    n.mul += u64::from(64 - e.leading_zeros()) + u64::from(e.count_ones()) - 2;
    pow_mod(a, e, p)
}

fn vrem(p: u64, a: &[u64], m: &[u64], n: &mut Nat) -> Vec<u64> {
    let mut r = a.to_vec();
    vtrim(&mut r);
    let dm = m.len() - 1;
    let lead = m[dm];
    let li = if lead == 1 { 1 } else { cinv(p, lead, n) };
    while r.len() > dm {
        let top = r.len() - 1;
        let mut c = r[top];
        if li != 1 {
            c = c * li % p;
            n.mul += 1;
        }
        for j in 0..dm {
            let t = c * m[j] % p;
            let x = r[top - dm + j];
            r[top - dm + j] = if x >= t { x - t } else { x + p - t };
        }
        n.mul += dm as u64;
        n.add += dm as u64;
        r.pop();
        vtrim(&mut r);
    }
    r
}

fn vgcd_monic(p: u64, a: &[u64], b: &[u64], n: &mut Nat) -> Vec<u64> {
    let (mut a, mut b) = (a.to_vec(), b.to_vec());
    vtrim(&mut a);
    vtrim(&mut b);
    while !b.is_empty() {
        let r = vrem(p, &a, &b, n);
        a = b;
        b = r;
    }
    if let Some(&lead) = a.last() {
        if lead != 1 {
            let il = cinv(p, lead, n);
            for c in a.iter_mut() {
                *c = *c * il % p;
            }
            n.mul += a.len() as u64;
        }
    }
    a
}

fn vdiv_monic(p: u64, a: &[u64], m: &[u64], n: &mut Nat) -> Vec<u64> {
    let dm = m.len() - 1;
    let mut r = a.to_vec();
    let mut q = vec![0u64; a.len() - dm];
    for top in (dm..a.len()).rev() {
        let c = r[top];
        q[top - dm] = c;
        for j in 0..dm {
            let t = c * m[j] % p;
            let x = r[top - dm + j];
            r[top - dm + j] = if x >= t { x - t } else { x + p - t };
        }
        n.mul += dm as u64;
        n.add += dm as u64;
    }
    q
}

/// The roots of a monic `u` known to split completely (Cantor–Zassenhaus).
pub fn roots_of(p: u64, u: &[u64], rng: &mut StdRng, n: &mut Nat) -> Option<Vec<u64>> {
    let d = u.len() - 1;
    if d == 0 {
        return Some(Vec::new());
    }
    if d == 1 {
        return Some(vec![(p - u[0]) % p]);
    }
    for _ in 0..64 {
        let aa = rng.gen_range(0..p);
        let ctx = ModU::new(p, u);
        let h = ctx.pow_xpa(aa, (p - 1) / 2, n);
        let mut hm1: Vec<u64> = h[..d].to_vec();
        hm1[0] = (hm1[0] + p - 1) % p;
        vtrim(&mut hm1);
        if hm1.is_empty() {
            continue;
        }
        let g = vgcd_monic(p, u, &hm1, n);
        let dg = g.len() - 1;
        if dg == 0 || dg == d {
            continue;
        }
        let q = vdiv_monic(p, u, &g, n);
        let mut out = roots_of(p, &g, rng, n)?;
        out.extend(roots_of(p, &q, rng, n)?);
        return Some(out);
    }
    None
}

// ── the run ─────────────────────────────────────────────────────────────

/// Counters of one relation source (main candidates or intermediates).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SourceCounts {
    /// Polynomials of the source's full degree that reached the decision.
    pub candidates: u64,
    pub prefilter_rejects: u64,
    pub splits: u64,
    pub relations: u64,
    pub rels_by_lp: [u64; 3],
    pub bad_relations: u64,
    /// Split candidates dropped because more than `REL_FB` places lay in
    /// the base (genus 4 intermediates only).
    pub overflow: u64,
}

#[derive(Clone, Debug, Default)]
pub struct HarvestResult {
    /// The `genus_ladder` result layout.  `relations`, `rels_by_lp` and
    /// `bad_relations` are totals over both sources; `candidates`,
    /// `prefilter_rejects` and `splits` are the main source's.
    pub ic: IcResult,
    pub harvest: bool,
    pub main: SourceCounts,
    pub harvested: SourceCounts,
    /// Trials whose group operation produced an intermediate.
    pub intermediates: u64,
}

/// Shared read-only state of the smoothness decision.
struct Ctx<'a> {
    curve: &'a HCurve,
    p: u64,
    r: u64,
    h: u64,
    chi: &'a [i8],
    col: &'a [u32],
    max_lp: usize,
    prefilter: bool,
    verify: bool,
}

/// The relation pipeline of `run_ic` on one candidate `(u, v)` whose class
/// is `ca·G + cb·T` (`u` monic of degree `d ≥ 1`, lowest first).  Charges
/// `nat`, consumes `rng` exactly as `run_ic` does.
#[allow(clippy::too_many_arguments)]
fn try_relation(
    cx: &Ctx,
    u: &[u64],
    vc: &[u64],
    cur: &JDiv,
    ca: u64,
    cb: u64,
    rng: &mut StdRng,
    nat: &mut Nat,
    sc: &mut SourceCounts,
    out: &mut RelSink,
) {
    let p = cx.p;
    let d = u.len() - 1;
    sc.candidates += 1;
    let mut decided = false;
    if cx.prefilter && d >= 2 {
        let ch = disc_character(p, u, cx.chi, nat);
        if ch != 1 {
            sc.prefilter_rejects += 1;
            return;
        }
        decided = d == 2;
    }
    if !decided && !splits_completely(p, u, nat) {
        return;
    }
    sc.splits += 1;
    let Some(roots) = roots_of(p, u, rng, nat) else {
        return;
    };
    let mut rel = Rel {
        fb: [(0, 0); REL_FB],
        nfb: 0,
        lp: [(0, 0); 2],
        nlp: 0,
        tcoef: 0,
        rhs: 0,
    };
    let mut lps: Vec<(u32, i8)> = Vec::new();
    let mut places: Vec<(u64, u64)> = Vec::new();
    let mut overflow = false;
    for &x in &roots {
        let mut y = 0u64;
        for &cf in vc.iter().rev() {
            y = (y * x + cf) % p;
        }
        nat.mul += vc.len() as u64;
        nat.add += vc.len() as u64;
        places.push((x, y));
        if y == 0 {
            continue; // a 2-torsion place: its log is 0 in the r-part
        }
        let eps: i8 = if y > (p - 1) / 2 { -1 } else { 1 };
        if cx.col[x as usize] != u32::MAX {
            if (rel.nfb as usize) < REL_FB {
                rel.fb[rel.nfb as usize] = (cx.col[x as usize], eps);
                rel.nfb += 1;
            } else {
                overflow = true;
            }
        } else {
            lps.push((x as u32, eps));
        }
    }
    if lps.len() > cx.max_lp {
        return;
    }
    if rel.nfb == 0 && lps.is_empty() {
        return;
    }
    if overflow {
        sc.overflow += 1;
        return;
    }
    nat.mul += 2;
    let r = cx.r;
    rel.rhs = mulmod(cx.h % r, ca, r);
    rel.tcoef = subm(0, mulmod(cx.h % r, cb, r), r);
    if cx.verify {
        let mut sum = JDiv::IDENTITY;
        for &(x, y) in &places {
            let pt = cx.curve.point(x, y).expect("a root of u lies on the curve");
            sum = cx.curve.cantor(sum, pt);
        }
        if sum != *cur {
            sc.bad_relations += 1;
        }
    }
    for (x, e) in lps {
        let id = *out.vid.entry(x).or_insert_with(|| out.dsu.add_vertex());
        rel.lp[rel.nlp as usize] = (id, e);
        rel.nlp += 1;
    }
    let (a, _, b, _) = rel.endpoints();
    out.dsu.edge(a, b);
    sc.rels_by_lp[rel.nlp as usize] += 1;
    sc.relations += 1;
    out.rels.push(rel);
}

struct RelSink {
    rels: Vec<Rel>,
    vid: HashMap<u32, u32>,
    dsu: Dsu,
}

/// `genus_ladder::run_ic` with optional harvesting of the first-reduction
/// intermediate (module notes).  With `harvest = false` every count equals
/// `run_ic`'s.
pub fn run_ic_harvest(inst: &Instance, prm: IcParams, harvest: bool) -> HarvestResult {
    assert!(
        prm.sge.is_none(),
        "run_ic_harvest: structured elimination is not supported"
    );
    let start = std::time::Instant::now();
    let curve = HCurve::from_jcurve(&inst.curve);
    let curve = &curve;
    let (p, r, h) = (inst.p, inst.r, inst.h);
    let g = inst.genus as usize;
    assert!(g <= REL_FB && g <= MAX_GENUS);
    let (fm0, fi0) = (curve.fmul(), curve.finv());
    let mut rng = StdRng::seed_from_u64(prm.seed);
    let mut res = IcResult {
        arm: prm.arm.name(),
        prefilter: prm.prefilter,
        p,
        genus: inst.genus,
        ..Default::default()
    };
    let mut main = SourceCounts::default();
    let mut harv = SourceCounts::default();
    let mut intermediates = 0u64;
    let mut ops = GroupOps::default();
    let f: Vec<u64> = curve.f.c[..curve.f.n].to_vec();

    let mut chi = vec![-1i8; p as usize];
    chi[0] = 0;
    for y in 1..=(p - 1) / 2 {
        chi[(y * y % p) as usize] = 1;
    }
    res.rel.mul += (p - 1) / 2;
    let want = if prm.arm == Arm::Full {
        usize::MAX
    } else {
        prm.nb
    };
    let mut col = vec![u32::MAX; p as usize];
    let mut ncols = 0usize;
    for x in 0..p {
        if ncols >= want {
            break;
        }
        let mut fx = 0u64;
        for &cf in f.iter().rev() {
            fx = (fx * x + cf) % p;
        }
        res.rel.mul += f.len() as u64;
        res.rel.add += f.len() as u64;
        if chi[fx as usize] == 1 {
            col[x as usize] = ncols as u32;
            ncols += 1;
        }
    }
    res.fb_cols = ncols;
    let tcol = ncols as u32;
    let nunk = ncols + 1;

    let short = r.min(1 << 12);
    let mut jumps: Vec<(JDiv, u64, u64)> = Vec::new();
    for _ in 0..16 {
        let a = rng.gen_range(1..short);
        let b = rng.gen_range(1..short);
        let ag = curve.mul(&mut ops, inst.g, a);
        let bt = curve.mul(&mut ops, inst.target, b);
        jumps.push((curve.add(&mut ops, ag, bt), a, b));
    }
    res.setup_ops = ops.adds + ops.doubles;

    let cx = Ctx {
        curve,
        p,
        r,
        h,
        chi: &chi,
        col: &col,
        max_lp: prm.arm.max_lp(),
        prefilter: prm.prefilter,
        verify: prm.verify_relations,
    };
    let mut sink = RelSink {
        rels: Vec::new(),
        vid: HashMap::new(),
        dsu: Dsu::new(),
    };
    let margin = |n: usize| 8 + n / 100;
    let mut cur = JDiv::IDENTITY;
    let (mut ca, mut cb) = (0u64, 0u64);
    let mut since_restart = u64::MAX;
    let mut next_check = nunk as u64;
    let mut solved = false;

    'outer: while !solved {
        while (sink.rels.len() as u64) < next_check {
            if res.trials >= prm.max_trials {
                res.exhausted = true;
                break 'outer;
            }
            let semi;
            if since_restart >= prm.restart.max(1) {
                let a = rng.gen_range(1..r);
                let b = rng.gen_range(1..r);
                let ag = curve.mul(&mut ops, inst.g, a);
                let bt = curve.mul(&mut ops, inst.target, b);
                if harvest {
                    (cur, semi) = curve.add_with_intermediate(&mut ops, ag, bt);
                } else {
                    cur = curve.add(&mut ops, ag, bt);
                    semi = None;
                }
                ca = a;
                cb = b;
                since_restart = 0;
            } else {
                let j = (mix64(curve.key_of(&cur)) >> 24) as usize % 16;
                let (m, aj, bj) = jumps[j];
                if harvest {
                    (cur, semi) = curve.add_with_intermediate(&mut ops, cur, m);
                } else {
                    cur = curve.add(&mut ops, cur, m);
                    semi = None;
                }
                ca = addm(ca, aj, r);
                cb = addm(cb, bj, r);
                since_restart += 1;
            }
            res.trials += 1;
            if cur.deg as usize == g {
                let u = curve.u_coeffs(&cur);
                let vc = curve.v_coeffs(&cur);
                try_relation(
                    &cx,
                    &u,
                    &vc,
                    &cur,
                    ca,
                    cb,
                    &mut rng,
                    &mut res.rel,
                    &mut main,
                    &mut sink,
                );
            }
            if let Some(sd) = semi {
                intermediates += 1;
                try_relation(
                    &cx,
                    &sd.u,
                    &sd.v,
                    &cur,
                    ca,
                    cb,
                    &mut rng,
                    &mut res.rel,
                    &mut harv,
                    &mut sink,
                );
            }
        }

        res.cycles = sink.dsu.cycles0();
        let nver = sink.dsu.parent.len();
        if prm.arm != Arm::Full {
            let need = (nunk + margin(nunk)) as i64;
            if res.cycles < need {
                next_check = sink.rels.len() as u64 + (need - res.cycles).max(1) as u64;
                continue;
            }
        } else {
            let mut scratch = Nat::default();
            let rows = build_rows(&sink.rels, nver, tcol, r, &mut scratch);
            let (core, cc, _) = peel(&rows, nunk, tcol, &mut scratch);
            if core.len() < cc + margin(cc) {
                next_check = sink.rels.len() as u64 + (nunk as u64 / 32).max(8);
                continue;
            }
        }

        res.attempts += 1;
        let mut g_nat = Nat::default();
        let rows = build_rows(&sink.rels, nver, tcol, r, &mut g_nat);
        let mut p_nat = Nat::default();
        let (core, cc, ct) = peel(&rows, nunk, tcol, &mut p_nat);
        res.la_graph.merge(g_nat);
        res.la_peel.merge(p_nat);
        res.la.merge(g_nat);
        res.la.merge(p_nat);
        res.core_rows = core.len();
        res.core_cols = cc;
        if core.len() < cc + margin(cc) {
            next_check = sink.rels.len() as u64 + (nunk as u64 / 20).max(8);
            continue;
        }
        let mut w_nat = Nat::default();
        let dense = res.attempts > 1;
        let sol = wiedemann(&core, cc, r, &mut rng, dense, &mut w_nat).map(|x| x[ct as usize]);
        res.la_wied.merge(w_nat);
        res.la.merge(w_nat);
        if let Some(t) = sol {
            let v0 = ops.adds + ops.doubles;
            let chk = curve.mul(&mut ops, inst.g, t);
            res.verify_ops = ops.adds + ops.doubles - v0;
            let (cm, ci) = (curve.fmul(), curve.finv());
            let ok = chk == inst.target && curve.mul_rtl(inst.g, t) == inst.target;
            curve.k.mul.set(cm);
            curve.k.inv.set(ci);
            if ok {
                res.recovered = Some(t);
                res.certified = true;
                solved = true;
                continue;
            }
        }
        next_check = sink.rels.len() as u64 + (nunk as u64 / 20).max(16);
    }
    res.la.add += sink.dsu.ops;
    res.group = ops;
    res.walk_ops = (ops.adds + ops.doubles) - res.setup_ops - res.verify_ops;
    res.cantor_mul = curve.fmul() - fm0;
    res.cantor_inv = curve.finv() - fi0;
    if prm.verify_relations {
        res.cantor_mul = 0;
        res.cantor_inv = 0;
    }
    res.candidates = main.candidates;
    res.prefilter_rejects = main.prefilter_rejects;
    res.splits = main.splits;
    res.relations = main.relations + harv.relations;
    for i in 0..3 {
        res.rels_by_lp[i] = main.rels_by_lp[i] + harv.rels_by_lp[i];
    }
    res.bad_relations = main.bad_relations + harv.bad_relations;
    res.wall_ns = start.elapsed().as_nanos() as u64;
    HarvestResult {
        ic: res,
        harvest,
        main,
        harvested: harv,
        intermediates,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::genus_ladder::{self as gl, make_instance};

    fn first_instance(p: u64, g: u32) -> Instance {
        for cs in 1..400u64 {
            if let Ok(i) = make_instance(p, g, cs, 7, 64) {
                return i;
            }
        }
        panic!("no instance at p = {p}, g = {g}");
    }

    fn prm(arm: Arm, p: u64, g: u32, seed: u64, pre: bool, verify: bool) -> IcParams {
        IcParams {
            arm,
            nb: gl::fb_size_2lp(p, g, 2.0),
            seed,
            max_trials: 5_000_000,
            restart: gl::RESTART,
            prefilter: pre,
            verify_relations: verify,
            sge: None,
        }
    }

    /// Every count of `IcResult` that `run_ic` fixes, as one comparable string.
    fn counts(o: &IcResult) -> String {
        format!(
            "{:?}|{}|{}|{}|{}|{}|{:?}|{:?}|{:?}|{:?}|{:?}|{}|{}|{}|{}|{}|{}|{:?}|{}|{}|{}|{}|{}|{:?}|{}|{}",
            o.group,
            o.setup_ops,
            o.walk_ops,
            o.verify_ops,
            o.cantor_mul,
            o.cantor_inv,
            o.rel,
            o.la,
            o.la_graph,
            o.la_peel,
            o.la_wied,
            o.fb_cols,
            o.trials,
            o.candidates,
            o.prefilter_rejects,
            o.splits,
            o.relations,
            o.rels_by_lp,
            o.bad_relations,
            o.cycles,
            o.core_rows,
            o.core_cols,
            o.attempts,
            o.recovered,
            o.certified,
            o.exhausted,
        )
    }

    /// H1: with harvesting off the run is `genus_ladder::run_ic`, count for count.
    #[test]
    fn harvest_off_is_run_ic() {
        for (p, g) in [(131u64, 2u32), (131, 3), (131, 4), (257, 3)] {
            let inst = first_instance(p, g);
            for arm in [Arm::Full, Arm::DoubleLp] {
                for pre in [false, true] {
                    for seed in [1u64, 2] {
                        let pr = prm(arm, p, g, seed, pre, false);
                        let a = gl::run_ic(&inst, pr);
                        let b = run_ic_harvest(&inst, pr, false);
                        assert!(a.certified, "p={p} g={g} {arm:?}");
                        assert_eq!(counts(&a), counts(&b.ic), "p={p} g={g} {arm:?} pre={pre}");
                        assert_eq!(b.harvested, SourceCounts::default());
                        assert_eq!(b.intermediates, 0);
                    }
                }
            }
        }
    }

    /// The reduced output of `cantor_with_intermediate` is `JCurve::cantor`'s,
    /// the field tally is the same, and the intermediate (when present) has
    /// degree `2g − 2`, satisfies `u | f − v²`, and its places sum to the
    /// reduced divisor whenever `u` splits.
    #[test]
    fn intermediate_is_equivalent_to_the_sum() {
        for (p, g) in [(131u64, 2u32), (131, 3), (131, 4), (1031, 3)] {
            let inst = first_instance(p, g);
            let hc = HCurve::from_jcurve(&inst.curve);
            let jc = &inst.curve;
            let mut st = 99u64;
            let mut a = jc.random_divisor(&mut st);
            let b = jc.random_divisor(&mut st);
            let (mut seen, mut split) = (0, 0);
            for _ in 0..3000 {
                let (s, semi) = hc.cantor_with_intermediate(a, b);
                let s2 = jc.cantor(a, b);
                assert_eq!(s, s2);
                if let Some(sd) = semi {
                    assert!(g >= 3);
                    seen += 1;
                    assert_eq!(sd.deg(), 2 * g as usize - 2);
                    assert_eq!(*sd.u.last().unwrap(), 1);
                    // u | f − v²
                    let mut nat = Nat::default();
                    let mut t = jc.f_coeffs();
                    let vv = &sd.v;
                    t.resize(t.len().max(2 * vv.len()), 0);
                    for i in 0..vv.len() {
                        for j in 0..vv.len() {
                            let k = i + j;
                            t[k] = (t[k] + p - vv[i] * vv[j] % p) % p;
                        }
                    }
                    let mut rr = t.clone();
                    let du = sd.deg();
                    while rr.len() > du {
                        let c = *rr.last().unwrap();
                        let top = rr.len() - 1;
                        for jj in 0..du {
                            rr[top - du + jj] = (rr[top - du + jj] + p - c * sd.u[jj] % p) % p;
                        }
                        rr.pop();
                    }
                    assert!(rr.iter().all(|&c| c == 0), "u' divides f − v'^2");
                    if splits_completely(p, &sd.u, &mut nat) {
                        split += 1;
                        let mut rng = StdRng::seed_from_u64(5);
                        let roots = roots_of(p, &sd.u, &mut rng, &mut nat).unwrap();
                        let mut sum = JDiv::IDENTITY;
                        for x in roots {
                            let mut y = 0u64;
                            for &cf in sd.v.iter().rev() {
                                y = (y * x + cf) % p;
                            }
                            sum = hc.cantor(sum, hc.point(x, y).unwrap());
                        }
                        assert_eq!(sum, s, "the intermediate's places sum to +D");
                        assert_ne!(sum, hc.negate(s));
                    }
                }
                a = s;
            }
            if g >= 3 {
                assert!(seen > 2500, "intermediates at p={p} g={g}: {seen}");
            }
            if g == 3 {
                assert!(split > 50, "split intermediates at p={p} g={g}: {split}");
            }
            if g == 2 {
                assert_eq!(seen, 0);
            }
        }
    }

    /// The field tally of `HCurve::cantor` equals `JCurve::cantor`'s.
    #[test]
    fn field_tally_matches() {
        let inst = first_instance(131, 3);
        let hc = HCurve::from_jcurve(&inst.curve);
        let jc = &inst.curve;
        let mut st = 3u64;
        let mut a = jc.random_divisor(&mut st);
        let b = jc.random_divisor(&mut st);
        let (jm, ji) = (jc.fmul(), jc.finv());
        let (hm, hi) = (hc.fmul(), hc.finv());
        for _ in 0..500 {
            let (x, _) = hc.cantor_with_intermediate(a, b);
            let y = jc.cantor(a, b);
            assert_eq!(x, y);
            a = x;
        }
        assert_eq!(jc.fmul() - jm, hc.fmul() - hm);
        assert_eq!(jc.finv() - ji, hc.finv() - hi);
    }

    /// H2: harvesting certifies and every harvested relation checks out by
    /// group arithmetic (p = 131, genus 3), both arms.
    #[test]
    fn harvest_verify_mode_no_bad_relations() {
        let inst = first_instance(131, 3);
        for arm in [Arm::Full, Arm::DoubleLp] {
            for seed in 1..=3u64 {
                let o = run_ic_harvest(&inst, prm(arm, 131, 3, seed, true, true), true);
                assert!(o.ic.certified, "{arm:?} seed {seed}");
                assert_eq!(o.ic.recovered, Some(inst.t_secret));
                assert_eq!(o.ic.bad_relations, 0);
                assert_eq!(o.harvested.bad_relations, 0);
                assert!(o.harvested.relations > 0, "{arm:?}: {:?}", o.harvested);
                assert!(o.intermediates > 0);
            }
        }
        // Genus 2: nothing to harvest, and the run equals the off path.
        let i2 = first_instance(131, 2);
        let pr = prm(Arm::DoubleLp, 131, 2, 1, true, false);
        let on = run_ic_harvest(&i2, pr, true);
        let off = run_ic_harvest(&i2, pr, false);
        assert_eq!(on.intermediates, 0);
        assert_eq!(counts(&on.ic), counts(&off.ic));
        // Genus 4: verify mode too.
        let i4 = first_instance(131, 4);
        let o = run_ic_harvest(&i4, prm(Arm::DoubleLp, 131, 4, 1, true, true), true);
        assert!(o.ic.certified);
        assert_eq!(o.ic.bad_relations, 0);
    }
}
