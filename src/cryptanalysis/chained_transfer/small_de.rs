//! Does a small endomorphism discriminant `D_E` reduce the *measured* cost of
//! prime-field index calculus once coordinates and the factor base are adapted
//! to the endomorphism?  Protocol and amendments:
//! `research/small_de_endo_fb_20261112/PROTOCOL.md`.
//!
//! The pieces, each testable on its own:
//! - [`construct`]: a prime-field curve with `#E = h·r`, `r` prime, `h <= 8`,
//!   with a chosen CM discriminant (`D = -3, -4, -7, -8, -11`) or a generic
//!   curve of large `|d_K|`; `d_K` and crater status come from `endo_census`.
//! - [`Endo`]: the endomorphism `psi` as a rational map (the order-3 and order-4
//!   automorphisms, or a Velu isogeny followed by an isomorphism for the
//!   degree-2 and -3 endomorphisms), its scalar action `lambda` with
//!   `psi(G) = [lambda]G` checked against an independent scalar multiplication,
//!   or, for the control arm, plain multiplication by a random scalar.
//! - [`Fb`]: the factor bases (x-interval, automorphism-orbit closed, and the
//!   layered `psi`-image base) with, for every point, the column it folds to and
//!   the coefficient `L(P) = coef * U_col (mod r)`.
//! - [`relation_phase`], [`solve_target`]: the decomposition search (the `S_3`
//!   quadratic for `m = 2`, the `S_4` quartic solved as two nested quadratics
//!   for `m = 3`), a sparse modular elimination ([`solve_sparse`]) and the
//!   per-target descent whose every log is certified by `[k]G == T`.
//!
//! Costs are two ledgers, kept apart: *priced* group operations
//! (`GroupOps`, one group-addition equivalent each) and *native* work that the
//! group-operation unit does not charge ([`Native`]: field multiplications,
//! inversions, membership tests, elimination multiply-adds).  Totals are
//! reported at 0, 0.1 and 1 GAE per native operation (AGENTS.md section 5a).

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use num_bigint::BigInt;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;

use super::ec::{Curve, VeluIsogeny};
use super::endo_census::{
    analyse, count_trace, curve_with_j, fundamental_discriminants, kronecker, on_crater,
};
use super::field::{is_prime, Fp, F2};
use crate::cryptanalysis::ic_boundary::{
    rho_walk_with, CountedGroup, CubicClasses, GroupOps, NegationClasses, PointClasses, PrimeCurve,
    PrimePoint, QuarticClasses, RhoWalk,
};

// ---------------------------------------------------------------- modular

#[inline]
fn mm(a: u64, b: u64, m: u64) -> u64 {
    a * b % m
}
fn pw(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1 % m;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mm(r, a, m);
        }
        a = mm(a, a, m);
        e >>= 1;
    }
    r
}
fn inv_mod(a: u64, m: u64) -> u64 {
    pw(a, m - 2, m)
}
fn sub_mod(a: u64, b: u64, m: u64) -> u64 {
    if a >= b {
        a - b
    } else {
        a + m - b
    }
}
fn add_mod(a: u64, b: u64, m: u64) -> u64 {
    let s = a + b;
    if s >= m {
        s - m
    } else {
        s
    }
}

/// Native (unpriced) operation counts.
#[derive(Clone, Copy, Debug, Default, Serialize, PartialEq, Eq)]
pub struct Native {
    /// Field multiplications and squarings.
    pub muls: u64,
    /// Field inversions.
    pub invs: u64,
    /// Factor-base membership tests (one per candidate root).
    pub tests: u64,
    /// Multiply-adds of the modular elimination.
    pub la_ops: u64,
}

impl Native {
    pub fn total(&self) -> u64 {
        self.muls + self.invs + self.tests + self.la_ops
    }
    pub fn merge(&mut self, o: &Native) {
        self.muls += o.muls;
        self.invs += o.invs;
        self.tests += o.tests;
        self.la_ops += o.la_ops;
    }
}

/// Priced plus native cost at a native weight `w` (GAE per native operation).
pub fn priced(gae: f64, native: u64, w: f64) -> f64 {
    gae + w * native as f64
}

// ---------------------------------------------------------------- instances

/// A prime-field curve `y^2 = x^3 + ax + b` with `#E = h r`, `r` prime, and a
/// generator `g` of the order-`r` subgroup.
#[derive(Clone, Debug)]
pub struct Inst {
    /// Requested CM discriminant (0 for the generic curve).
    pub d: i64,
    pub curve: PrimeCurve,
    pub n: u64,
    pub h: u64,
    pub r: u64,
    pub t: i64,
    pub g: PrimePoint,
    /// A point of `E(F_p)` with `[h]P0 = G`: relation trials are multiples of
    /// `P0`, so no cofactor component has to cancel.
    pub p0: PrimePoint,
    /// Fundamental discriminant found by `endo_census::analyse` (None if above its bound).
    pub d_k: Option<i64>,
    /// Conductor `f` with `t^2 - 4p = d_K f^2`.
    pub f: Option<u64>,
    /// `Some(true)` when `j(E)` is the class-number-one `j` of `d_K` (End = O_K).
    pub crater: Option<bool>,
}

fn splitmix(x: &mut u64) -> u64 {
    *x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = *x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn cofactor_split(n: u64, p: u64, hmax: u64) -> Option<(u64, u64)> {
    (1..=hmax).find_map(|h| {
        (n.is_multiple_of(h) && n / h > p / 5 && is_prime(n / h)).then_some((h, n / h))
    })
}

fn make_inst(
    d: i64,
    p: u64,
    a: u64,
    b: u64,
    t: i64,
    h: u64,
    r: u64,
    rng: &mut StdRng,
) -> Option<Inst> {
    let curve = PrimeCurve { p, a, b };
    let n = (p as i64 + 1 - t) as u64;
    let mut ops = GroupOps::default();
    let (g, p0) = loop {
        let x = rng.gen_range(0..p);
        let Some(y) = curve.sqrt(curve.rhs(x)) else {
            continue;
        };
        let pt = PrimePoint::affine(x, y);
        let g = curve.mul(&mut ops, pt, h);
        if g.infinity {
            continue;
        }
        if !curve.mul(&mut ops, g, r).infinity {
            return None;
        }
        break (g, pt);
    };
    let bp = BigInt::from(p);
    let discs = fundamental_discriminants(4400);
    let an = analyse(&bp, &BigInt::from(t), &discs);
    let f = an.f.as_ref().and_then(|f| u64::try_from(f.clone()).ok());
    let crater = an
        .d_k
        .and_then(|dk| on_crater(&bp, &BigInt::from(a), &BigInt::from(b), dk));
    Some(Inst {
        d,
        curve,
        n,
        h,
        r,
        t,
        g,
        p0,
        d_k: an.d_k,
        f,
        crater,
    })
}

fn next_prime_from(mut x: u64) -> u64 {
    if x.is_multiple_of(2) {
        x += 1;
    }
    while !is_prime(x) {
        x += 2;
    }
    x
}

/// Construct an instance with `p` of exactly `bits` bits.  `d != 0`: a CM
/// curve with `d_K = d` on the crater (`End = O_K`).  `d == 0`: a generic
/// curve whose `d_K` is above 100 in absolute value (or beyond the census
/// bound).  This is the one-time construction cost, never in a per-instance
/// total.
pub fn construct(d: i64, bits: u32, seed: u64) -> Option<Inst> {
    let mut rng =
        StdRng::seed_from_u64(seed ^ ((bits as u64) << 8) ^ (d as u64).wrapping_mul(0x9E37));
    let mut s = seed ^ 0xD1B5_4A32;
    let lo = 1u64 << (bits - 1);
    let mut p = next_prime_from(lo + splitmix(&mut s) % (lo / 2));
    for _ in 0..40_000 {
        p = next_prime_from(p + 2);
        if p >= (1u64 << bits) {
            p = next_prime_from(lo + splitmix(&mut s) % (lo / 2));
        }
        if p < 11 || Fp::new(p).is_none() {
            continue;
        }
        let cands: Vec<(u64, u64)> = if d == 0 {
            (0..24)
                .map(|_| (rng.gen_range(1..p), rng.gen_range(1..p)))
                .collect()
        } else {
            if kronecker(d, p) != 1 {
                continue;
            }
            match d {
                -3 => (1..=48).map(|b| (0, b)).collect(),
                -4 => (1..=48).map(|a| (a, 0)).collect(),
                _ => {
                    let j = match d {
                        -7 => -3375,
                        -8 => 8000,
                        -11 => -32768,
                        _ => return None,
                    };
                    let (a, b) = curve_with_j(p, j)?;
                    let mut c = 2u64;
                    while pw(c, (p - 1) / 2, p) == 1 {
                        c += 1;
                    }
                    let c2 = mm(c, c, p);
                    let c3 = mm(c2, c, p);
                    vec![(a, b), (mm(a, c2, p), mm(b, c3, p))]
                }
            }
        };
        let mut seen = vec![];
        for (a, b) in cands {
            let disc = (4 * mm(mm(a, a, p), a, p) + 27 * mm(b, b, p)) % p;
            if disc == 0 {
                continue;
            }
            let Some(t) = count_trace(p, a, b) else {
                continue;
            };
            let n = (p as i64 + 1 - t) as u64;
            if seen.contains(&n) {
                continue;
            }
            seen.push(n);
            let Some((h, r)) = cofactor_split(n, p, 4) else {
                continue;
            };
            if r < 11 {
                continue;
            }
            let Some(inst) = make_inst(d, p, a, b, t, h, r, &mut rng) else {
                continue;
            };
            let ok = if d == 0 {
                inst.d_k.is_none_or(|k| k.abs() > 100)
            } else {
                inst.d_k == Some(d) && inst.crater == Some(true)
            };
            if ok {
                return Some(inst);
            }
        }
    }
    None
}

// ---------------------------------------------------------------- endomorphisms

/// How `psi` is evaluated.
pub enum EndoKind {
    /// `D = -3`: `(x, y) -> (omega x, y)`.
    Zeta { omega: u64 },
    /// `D = -4`: `(x, y) -> (-x, i y)`.
    Iota { i: u64 },
    /// `D = -7, -8, -11`: Velu isogeny of degree `ell` to `E'` then `E' ~ E`.
    Velu {
        phi: Box<VeluIsogeny>,
        s: u64,
        u3: u64,
        ell: u32,
    },
    /// Control: multiplication by a scalar (no rational map).
    Scalar,
}

/// An endomorphism and its action `psi(G) = [lambda]G` on the order-`r` subgroup.
pub struct Endo {
    pub kind: EndoKind,
    pub lambda: u64,
    pub name: &'static str,
}

fn to_pt(p: PrimePoint) -> super::ec::Pt {
    if p.infinity {
        None
    } else {
        Some((F2::base(p.x), F2::base(p.y)))
    }
}

impl Endo {
    /// `psi(P)`; field work into `nat`, group operations into `ops`.
    pub fn apply(
        &self,
        c: &PrimeCurve,
        ops: &mut GroupOps,
        nat: &mut Native,
        pt: PrimePoint,
    ) -> PrimePoint {
        if pt.infinity {
            return pt;
        }
        match &self.kind {
            EndoKind::Zeta { omega } => {
                nat.muls += 1;
                PrimePoint::affine(mm(*omega, pt.x, c.p), pt.y)
            }
            EndoKind::Iota { i } => {
                nat.muls += 1;
                PrimePoint::affine(sub_mod(0, pt.x, c.p), mm(*i, pt.y, c.p))
            }
            EndoKind::Velu { phi, s, u3, ell } => {
                // kernel sum: ell - 1 group additions, then the isomorphism
                ops.adds += (*ell as u64) - 1;
                nat.muls += 2;
                match phi.eval(&to_pt(pt)) {
                    None => PrimePoint::INFINITY,
                    Some((x, y)) => {
                        debug_assert!(x.b == 0 && y.b == 0);
                        PrimePoint::affine(mm(*s, x.a, c.p), mm(*u3, y.a, c.p))
                    }
                }
            }
            EndoKind::Scalar => c.mul(ops, pt, self.lambda),
        }
    }
}

fn roots_mod_r(tr: u64, nm: u64, r: u64) -> Vec<u64> {
    // roots of x^2 - tr x + nm mod r
    let disc = sub_mod(mm(tr, tr, r), mm(4, nm, r), r);
    let Some(s) = (PrimeCurve { p: r, a: 0, b: 0 }).sqrt(disc) else {
        return vec![];
    };
    let i2 = inv_mod(2, r);
    let r1 = mm(add_mod(tr % r, s, r), i2, r);
    let r2 = mm(sub_mod(tr % r, s, r), i2, r);
    if r1 == r2 {
        vec![r1]
    } else {
        vec![r1, r2]
    }
}

/// Build the endomorphism of the curve's CM order.  `None` if `inst.d` has none.
pub fn endo_for(inst: &Inst) -> Option<Endo> {
    let c = &inst.curve;
    let (p, r) = (c.p, inst.r);
    let mut ops = GroupOps::default();
    match inst.d {
        -3 => {
            let s = c.sqrt(sub_mod(0, 3, p))?;
            let omega = mm(add_mod(sub_mod(0, 1, p), s, p), inv_mod(2, p), p);
            let zg = PrimePoint::affine(mm(omega, inst.g.x, p), inst.g.y);
            for lam in roots_mod_r(sub_mod(0, 1, r), 1, r) {
                if c.mul(&mut ops, inst.g, lam) == zg {
                    return Some(Endo {
                        kind: EndoKind::Zeta { omega },
                        lambda: lam,
                        name: "zeta3",
                    });
                }
            }
            None
        }
        -4 => {
            let i = c.sqrt(sub_mod(0, 1, p))?;
            let ig = PrimePoint::affine(sub_mod(0, inst.g.x, p), mm(i, inst.g.y, p));
            for lam in roots_mod_r(0, 1, r) {
                if c.mul(&mut ops, inst.g, lam) == ig {
                    return Some(Endo {
                        kind: EndoKind::Iota { i },
                        lambda: lam,
                        name: "iota4",
                    });
                }
            }
            None
        }
        -7 | -8 | -11 => {
            let (tr, nm, name): (u64, u64, &'static str) = match inst.d {
                -7 => (1, 2, "+-(1+sqrt-7)/2"),
                -8 => (0, 2, "sqrt-2"),
                _ => (1, 3, "+-(1+sqrt-11)/2"),
            };
            let k = Fp::new(p)?;
            let e = Curve::new(k, 0, c.a, c.b)?;
            // x-coordinates of the rational points of the kernel candidates
            let xs: Vec<u64> = (0..p)
                .filter(|&x| {
                    if nm == 2 {
                        c.rhs(x) == 0
                    } else {
                        // 3-division polynomial 3x^4 + 6a x^2 + 12 b x - a^2
                        let x2 = mm(x, x, p);
                        let v = add_mod(
                            add_mod(mm(3, mm(x2, x2, p), p), mm(6, mm(c.a, x2, p), p), p),
                            sub_mod(mm(12, mm(c.b, x, p), p), mm(c.a, c.a, p), p),
                            p,
                        );
                        v == 0
                    }
                })
                .collect();
            for x0 in xs {
                let rhs = F2::base(c.rhs(x0));
                let Some(y0) = k.sqrt2(rhs) else {
                    continue;
                };
                let g0 = Some((F2::base(x0), y0));
                let Some(phi) = VeluIsogeny::from_generator(&e, &g0) else {
                    continue;
                };
                if phi.degree() != nm as usize {
                    continue;
                }
                let (a2, b2) = (phi.codomain.a4, phi.codomain.a6);
                if a2 == 0 || b2 == 0 || c.a == 0 || c.b == 0 {
                    continue;
                }
                // E' -> E: x = s x', y = u^3 y', s = u^2, a = s^2 a', b = s^3 b'
                let s = mm(mm(c.b, a2, p), inv_mod(mm(b2, c.a, p), p), p);
                if mm(mm(s, s, p), a2, p) != c.a || mm(mm(mm(s, s, p), s, p), b2, p) != c.b {
                    continue;
                }
                let Some(u) = c.sqrt(s) else {
                    continue;
                };
                let u3 = mm(mm(u, u, p), u, p);
                let mut endo = Endo {
                    kind: EndoKind::Velu {
                        phi: Box::new(phi),
                        s,
                        u3,
                        ell: nm as u32,
                    },
                    lambda: 0,
                    name,
                };
                let mut nat = Native::default();
                let img = endo.apply(c, &mut ops, &mut nat, inst.g);
                // the isomorphism is fixed up to sign, so the map is +-psi
                for lam in roots_mod_r(tr, nm, r) {
                    for l in [lam, sub_mod(0, lam, r)] {
                        if c.mul(&mut ops, inst.g, l) == img {
                            endo.lambda = l;
                            return Some(endo);
                        }
                    }
                }
            }
            None
        }
        _ => None,
    }
}

/// The control endomorphism: multiplication by a random scalar.
pub fn scalar_endo(inst: &Inst, seed: u64) -> Endo {
    let mut rng = StdRng::seed_from_u64(seed);
    Endo {
        kind: EndoKind::Scalar,
        lambda: rng.gen_range(2..inst.r - 1),
        name: "scalar",
    }
}

// ---------------------------------------------------------------- hash table

/// Open-addressing map from an abscissa to a factor-base index.
pub struct XTab {
    keys: Vec<u32>,
    vals: Vec<u32>,
    shift: u32,
}

impl XTab {
    pub fn new(n: usize) -> XTab {
        let bits = (usize::BITS - (2 * n.max(4)).leading_zeros()).max(4);
        XTab {
            keys: vec![0; 1 << bits],
            vals: vec![0; 1 << bits],
            shift: 64 - bits,
        }
    }
    #[inline]
    fn slot(&self, x: u64) -> usize {
        (x.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> self.shift) as usize
    }
    pub fn insert(&mut self, x: u64, v: u32) {
        let m = self.keys.len() - 1;
        let mut i = self.slot(x);
        while self.keys[i] != 0 {
            i = (i + 1) & m;
        }
        self.keys[i] = x as u32 + 1;
        self.vals[i] = v;
    }
    #[inline]
    pub fn get(&self, x: u64) -> Option<u32> {
        let m = self.keys.len() - 1;
        let mut i = self.slot(x);
        let k = x as u32 + 1;
        loop {
            let kk = self.keys[i];
            if kk == k {
                return Some(self.vals[i]);
            }
            if kk == 0 {
                return None;
            }
            i = (i + 1) & m;
        }
    }
}

// ---------------------------------------------------------------- factor bases

/// A factor base: points of `E(F_p)` (one per `+-` class), each folded to a
/// column with `L(P) = coef * U_col (mod r)`, where `L(P) = log_G [h]P`.
pub struct Fb {
    pub label: String,
    pub x: Vec<u64>,
    pub y: Vec<u64>,
    pub col: Vec<u32>,
    pub coef: Vec<u64>,
    pub ncols: usize,
    pub tab: XTab,
    /// Seed points scanned (candidate abscissae tested).
    pub scanned: u64,
    /// Layered points dropped because their abscissa was already present.
    pub dropped: u64,
    pub build_ops: GroupOps,
    pub build_native: Native,
}

impl Fb {
    pub fn len(&self) -> usize {
        self.x.len()
    }
    pub fn is_empty(&self) -> bool {
        self.x.is_empty()
    }
    fn push(&mut self, pt: PrimePoint, col: u32, coef: u64) -> bool {
        if pt.infinity || pt.y == 0 || self.tab.get(pt.x).is_some() {
            self.dropped += 1;
            return false;
        }
        self.tab.insert(pt.x, self.x.len() as u32);
        self.x.push(pt.x);
        self.y.push(pt.y);
        self.col.push(col);
        self.coef.push(coef);
        true
    }
    fn empty(label: &str, n: usize) -> Fb {
        Fb {
            label: label.to_string(),
            x: vec![],
            y: vec![],
            col: vec![],
            coef: vec![],
            ncols: 0,
            tab: XTab::new(n + 8),
            scanned: 0,
            dropped: 0,
            build_ops: GroupOps::default(),
            build_native: Native::default(),
        }
    }
    pub fn point(&self, i: usize) -> PrimePoint {
        PrimePoint::affine(self.x[i], self.y[i])
    }
}

fn next_seed_point(c: &PrimeCurve, x: &mut u64, nat: &mut Native, scanned: &mut u64) -> PrimePoint {
    loop {
        let xx = *x;
        *x += 1;
        *scanned += 1;
        let mut muls = 3; // x^3 + ax + b
        let y = c.sqrt_counted(c.rhs(xx), &mut muls);
        nat.muls += muls;
        if let Some(y) = y {
            if y != 0 {
                return PrimePoint::affine(xx, y);
            }
        }
    }
}

/// `x`-interval base: the first `n` abscissae (from 1) with `x^3 + ax + b` a
/// nonzero square, one column each.
pub fn fb_interval(inst: &Inst, n: usize) -> Fb {
    let mut fb = Fb::empty("x-interval", n);
    let mut x = 1u64;
    let mut sc = 0;
    while fb.len() < n {
        let pt = next_seed_point(&inst.curve, &mut x, &mut fb.build_native, &mut sc);
        let col = fb.len() as u32;
        fb.push(pt, col, 1);
    }
    fb.scanned = sc;
    fb.ncols = fb.len();
    fb
}

/// Automorphism-orbit closed base for `D = -3` (`x -> omega x`, orbits of
/// three classes) or `D = -4` (`x -> -x`, orbits of two): seeds are scanned
/// in order, each contributes its whole orbit, until at least `n` classes.
/// A column is one orbit; `coef = lambda^j` for the `j`-th image.
pub fn fb_aut(inst: &Inst, endo: &Endo, n: usize) -> Fb {
    let c = &inst.curve;
    let orbit = match endo.kind {
        EndoKind::Zeta { .. } => 3,
        EndoKind::Iota { .. } => 2,
        _ => panic!("fb_aut needs an automorphism"),
    };
    let mut fb = Fb::empty("aut-orbit-closed", n + orbit);
    let mut x = 1u64;
    let mut sc = 0;
    while fb.len() < n {
        let seed = next_seed_point(c, &mut x, &mut fb.build_native, &mut sc);
        if fb.tab.get(seed.x).is_some() {
            continue; // already inside an earlier orbit
        }
        let col = fb.ncols as u32;
        let mut cur = seed;
        let mut coef = 1u64;
        let mut added = 0;
        for _ in 0..orbit {
            let (mut o, mut nn) = (GroupOps::default(), Native::default());
            if fb.push(cur, col, coef) {
                added += 1;
            }
            cur = endo.apply(c, &mut o, &mut nn, cur);
            fb.build_native.merge(&nn);
            coef = mm(coef, endo.lambda, inst.r);
        }
        if added > 0 {
            fb.ncols += 1;
        }
    }
    fb.scanned = sc;
    fb
}

/// Layered `psi`-image base: `ceil(n / layers)` x-interval seeds, each with
/// its images `psi^i(P)`, `i < layers`, one column per seed and
/// `coef = lambda^i`.  The base is closed under `psi` except at its top layer.
pub fn fb_layered(inst: &Inst, endo: &Endo, n: usize, layers: usize) -> Fb {
    let c = &inst.curve;
    let nseed = n.div_ceil(layers);
    let mut fb = Fb::empty("psi-layered", n + layers);
    let mut x = 1u64;
    let mut sc = 0;
    for _ in 0..nseed {
        let seed = next_seed_point(c, &mut x, &mut fb.build_native, &mut sc);
        let col = fb.ncols as u32;
        let mut cur = seed;
        let mut coef = 1u64;
        let mut added = 0;
        for _ in 0..layers {
            if fb.push(cur, col, coef) {
                added += 1;
            }
            let (mut o, mut nn) = (GroupOps::default(), Native::default());
            cur = endo.apply(c, &mut o, &mut nn, cur);
            fb.build_ops.merge(o);
            fb.build_native.merge(&nn);
            coef = mm(coef, endo.lambda, inst.r);
        }
        if added > 0 {
            fb.ncols += 1;
        }
    }
    fb.scanned = sc;
    fb
}

// ---------------------------------------------------------------- decomposition

/// One relation `sum eps_k L(P_{idx_k}) = rhs (mod r)`.
#[derive(Clone, Debug)]
pub struct Relation {
    pub terms: Vec<(u32, i8)>,
    pub rhs: u64,
}

struct Sweeper<'a> {
    fb: &'a Fb,
    c: &'a PrimeCurve,
    /// Active indices (ascending); all points, or the known ones.
    act: Vec<u32>,
    pref: Vec<u64>,
    d: Vec<u64>,
}

impl<'a> Sweeper<'a> {
    fn new(fb: &'a Fb, c: &'a PrimeCurve, avail: Option<&[bool]>) -> Self {
        let act: Vec<u32> = (0..fb.len() as u32)
            .filter(|&i| avail.is_none_or(|a| a[fb.col[i as usize] as usize]))
            .collect();
        let n = act.len();
        Sweeper {
            fb,
            c,
            act,
            pref: vec![0; n + 1],
            d: vec![0; n],
        }
    }

    /// All `(i, c, j, s)` with `U = -c P_i + s P_j`, `i >= i_min`, `j >= i`:
    /// the roots `x(U + c P_i)` of the `S_3` quadratic in the second variable
    /// (evaluated from the known `y_i`, `y_U`, no square root), tested for
    /// membership, hits confirmed with a priced group addition.
    fn pairs(
        &mut self,
        u: PrimePoint,
        i_min: u32,
        ops: &mut GroupOps,
        nat: &mut Native,
        out: &mut Vec<(u32, i8, u32, i8)>,
    ) {
        let (fb, cc) = (self.fb, self.c);
        let Sweeper {
            act: act_all,
            pref,
            d: dvec,
            ..
        } = self;
        let p = cc.p;
        let start = act_all.partition_point(|&i| i < i_min);
        let act = &act_all[start..];
        let n = act.len();
        if n == 0 {
            return;
        }
        // d_i = x_i - x_U, batch inversion (Montgomery's trick)
        pref[0] = 1;
        for (k, &i) in act.iter().enumerate() {
            let mut d = fb.x[i as usize] + p - u.x;
            if d >= p {
                d -= p;
            }
            dvec[k] = d;
            let dd = if d == 0 { 1 } else { d };
            pref[k + 1] = mm(pref[k], dd, p);
        }
        let mut inv_all = inv_mod(pref[n], p);
        nat.invs += 1;
        nat.muls += 3 * n as u64 + 4 * n as u64;
        nat.tests += 2 * n as u64;
        for k in (0..n).rev() {
            let i = act[k] as usize;
            let d = dvec[k];
            if d == 0 {
                continue;
            }
            let inv = mm(inv_all, pref[k], p);
            inv_all = mm(inv_all, d, p);
            let (xi, yi) = (fb.x[i], fb.y[i]);
            let sx = xi + u.x;
            for (cs, dy) in [
                (1i8, sub_mod(yi, u.y, p)),
                (-1i8, sub_mod(sub_mod(0, yi, p), u.y, p)),
            ] {
                let lam = mm(dy, inv, p);
                let mut xr = mm(lam, lam, p) + 2 * p - sx % p;
                xr %= p;
                if let Some(j) = fb.tab.get(xr) {
                    if (j as usize) < i {
                        continue;
                    }
                    if act_all.binary_search(&j).is_err() {
                        continue;
                    }
                    // confirm and fix the sign of P_j
                    let pi = fb.point(i);
                    let q = if cs == 1 { pi } else { cc.neg(pi) };
                    let w = cc.add(ops, u, q);
                    if w.infinity {
                        continue;
                    }
                    let yj = fb.y[j as usize];
                    let s = if w.y == yj {
                        1
                    } else if w.y == sub_mod(0, yj, p) {
                        -1
                    } else {
                        continue;
                    };
                    if w.x != fb.x[j as usize] {
                        continue;
                    }
                    out.push((i as u32, cs, j, s));
                }
            }
        }
    }

    /// Every decomposition of `R` into `m` factor-base points (up to sign),
    /// as term lists with `R = sum eps_k P_{idx_k}`.
    fn decompose(
        &mut self,
        r_pt: PrimePoint,
        m: usize,
        ops: &mut GroupOps,
        nat: &mut Native,
    ) -> Vec<Vec<(u32, i8)>> {
        let mut res = vec![];
        let mut hits = vec![];
        if m == 2 {
            self.pairs(r_pt, 0, ops, nat, &mut hits);
            for (i, c, j, s) in hits {
                res.push(vec![(i, -c), (j, s)]);
            }
        } else {
            let acts = self.act.clone();
            for &i3 in &acts {
                let p3 = self.fb.point(i3 as usize);
                for c3 in [1i8, -1] {
                    let q = if c3 == 1 { p3 } else { self.c.neg(p3) };
                    let u = self.c.add(ops, r_pt, q);
                    if u.infinity {
                        continue;
                    }
                    hits.clear();
                    self.pairs(u, i3, ops, nat, &mut hits);
                    for &(i2, c2, j, s) in &hits {
                        res.push(vec![(i3, -c3), (i2, -c2), (j, s)]);
                    }
                }
            }
        }
        res
    }
}

/// Verify a relation's points really sum to `R`.
pub fn check_decomposition(c: &PrimeCurve, fb: &Fb, r_pt: PrimePoint, terms: &[(u32, i8)]) -> bool {
    let mut ops = GroupOps::default();
    let mut acc = PrimePoint::INFINITY;
    for &(i, e) in terms {
        let pt = fb.point(i as usize);
        let pt = if e < 0 { c.neg(pt) } else { pt };
        acc = c.add(&mut ops, acc, pt);
    }
    acc == r_pt
}

// ---------------------------------------------------------------- linear algebra

/// Result of [`solve_sparse`].
pub struct LaOut {
    pub value: Vec<Option<u64>>,
    pub mul_adds: u64,
    pub invs: u64,
    pub rows: usize,
    pub cols: usize,
    pub pivots: usize,
    pub max_row: usize,
    pub determined: usize,
    pub consistent: bool,
}

/// Sparse Gaussian elimination mod the prime `r`, columns taken in order of
/// current weight (lightest first), pivot the sparsest row; back-substitution
/// determines exactly the columns forced by the relations.
pub fn solve_sparse(rows: &[(Vec<(u32, u64)>, u64)], ncols: usize, r: u64) -> LaOut {
    let nrows = rows.len();
    let mut ent: Vec<Vec<(u32, u64)>> = rows.iter().map(|x| x.0.clone()).collect();
    let mut rhs: Vec<u64> = rows.iter().map(|x| x.1).collect();
    let mut alive = vec![true; nrows];
    let mut col_rows: Vec<Vec<u32>> = vec![vec![]; ncols];
    let mut weight = vec![0u32; ncols];
    for (ri, e) in ent.iter().enumerate() {
        for &(c, _) in e {
            col_rows[c as usize].push(ri as u32);
            weight[c as usize] += 1;
        }
    }
    let mut heap: BinaryHeap<Reverse<(u32, u32)>> = (0..ncols)
        .filter(|&c| weight[c] > 0)
        .map(|c| Reverse((weight[c], c as u32)))
        .collect();
    let mut mul_adds = 0u64;
    let mut invs = 0u64;
    let mut max_row = 0usize;
    let mut piv: Vec<(u32, Vec<(u32, u64)>, u64, u64)> = vec![];
    while let Some(Reverse((w, c))) = heap.pop() {
        if weight[c as usize] != w || w == 0 {
            continue;
        }
        let mut cand: Vec<u32> = col_rows[c as usize]
            .iter()
            .copied()
            .filter(|&ri| {
                alive[ri as usize] && ent[ri as usize].binary_search_by_key(&c, |e| e.0).is_ok()
            })
            .collect();
        cand.sort_unstable();
        cand.dedup();
        let Some(&pr) = cand.iter().min_by_key(|&&ri| ent[ri as usize].len()) else {
            weight[c as usize] = 0;
            continue;
        };
        let prow = ent[pr as usize].clone();
        let prhs = rhs[pr as usize];
        max_row = max_row.max(prow.len());
        let apc = prow[prow.binary_search_by_key(&c, |e| e.0).unwrap()].1;
        let inv = inv_mod(apc, r);
        invs += 1;
        let mut touched: Vec<u32> = vec![];
        for &rj in &cand {
            if rj == pr {
                continue;
            }
            let row = std::mem::take(&mut ent[rj as usize]);
            let ajc = row[row.binary_search_by_key(&c, |e| e.0).unwrap()].1;
            let f = mm(ajc, inv, r);
            mul_adds += prow.len() as u64 + 1;
            rhs[rj as usize] = sub_mod(rhs[rj as usize], mm(f, prhs, r), r);
            let mut merged: Vec<(u32, u64)> = Vec::with_capacity(row.len() + prow.len());
            let (mut a, mut b) = (0, 0);
            while a < row.len() || b < prow.len() {
                if b == prow.len() || (a < row.len() && row[a].0 < prow[b].0) {
                    merged.push(row[a]);
                    a += 1;
                } else if a == row.len() || prow[b].0 < row[a].0 {
                    let col = prow[b].0;
                    let v = sub_mod(0, mm(f, prow[b].1, r), r);
                    merged.push((col, v));
                    weight[col as usize] += 1;
                    col_rows[col as usize].push(rj);
                    touched.push(col);
                    b += 1;
                } else {
                    let col = row[a].0;
                    let v = sub_mod(row[a].1, mm(f, prow[b].1, r), r);
                    if v != 0 {
                        merged.push((col, v));
                    } else {
                        weight[col as usize] -= 1;
                        touched.push(col);
                    }
                    a += 1;
                    b += 1;
                }
            }
            max_row = max_row.max(merged.len());
            ent[rj as usize] = merged;
        }
        for &(col, _) in &prow {
            weight[col as usize] -= 1;
            touched.push(col);
        }
        alive[pr as usize] = false;
        ent[pr as usize] = vec![];
        touched.sort_unstable();
        touched.dedup();
        for col in touched {
            if weight[col as usize] > 0 {
                heap.push(Reverse((weight[col as usize], col)));
            }
        }
        piv.push((c, prow, prhs, inv));
    }
    let consistent = (0..nrows).all(|i| !alive[i] || !ent[i].is_empty() || rhs[i] == 0);
    let mut value: Vec<Option<u64>> = vec![None; ncols];
    let mut determined = 0;
    for (c, row, prhs, inv) in piv.iter().rev() {
        let mut sum = *prhs;
        let mut ok = true;
        for &(k, a) in row {
            if k == *c {
                continue;
            }
            match value[k as usize] {
                Some(v) => {
                    sum = sub_mod(sum, mm(a, v, r), r);
                    mul_adds += 1;
                }
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            value[*c as usize] = Some(mm(sum, *inv, r));
            mul_adds += 1;
            determined += 1;
        }
    }
    LaOut {
        value,
        mul_adds,
        invs,
        rows: nrows,
        cols: ncols,
        pivots: piv.len(),
        max_row,
        determined,
        consistent,
    }
}

// ---------------------------------------------------------------- relation phase

/// What the relation phase measured.
#[derive(Clone, Debug, Default, Serialize)]
pub struct PhaseStats {
    pub fb_points: usize,
    pub columns: usize,
    pub dropped: u64,
    pub trials: u64,
    pub trials_with_hit: u64,
    pub relations: u64,
    pub decomp_prob: f64,
    pub hits_per_trial: f64,
    pub trials_per_relation: f64,
    /// Field operations (muls + invs) of the decomposition sweeps per trial.
    pub sweep_field_ops_per_trial: f64,
    pub la_attempts: u32,
    pub la_rows: usize,
    pub la_cols: usize,
    pub la_pivots: usize,
    pub la_max_row: usize,
    pub la_final_ops: u64,
    pub la_total_ops: u64,
    pub determined_cols: usize,
    pub determined_frac: f64,
    /// True when the 90% rule was met; false = stopped at the relation cap.
    pub rule_met: bool,
    pub gae: f64,
    pub native: Native,
    pub build_gae: f64,
    pub build_native: Native,
    pub wall_ms: u64,
}

pub struct PhaseOut {
    pub stats: PhaseStats,
    /// `U_col` for determined columns.
    pub known: Vec<Option<u64>>,
}

/// Jump table `J_s = [alpha_s]G` for the trial walk.
struct Jumps {
    pts: Vec<PrimePoint>,
    al: Vec<u64>,
}

/// A uniformly random element `K = [r]Z` of the cofactor part (`[h]K = O`, so
/// `L(K) = 0`); `O` when `h = 1`.
fn random_cofactor_part(inst: &Inst, rng: &mut StdRng, ops: &mut GroupOps) -> PrimePoint {
    if inst.h == 1 {
        return PrimePoint::INFINITY;
    }
    let c = &inst.curve;
    loop {
        let x = rng.gen_range(0..c.p);
        if let Some(y) = c.sqrt(c.rhs(x)) {
            return c.mul(ops, PrimePoint::affine(x, y), inst.r);
        }
    }
}

/// `R + K` for a fresh random cofactor-part `K` (no work when `h = 1`).
fn add_cofactor_part(
    inst: &Inst,
    rng: &mut StdRng,
    ops: &mut GroupOps,
    pt: PrimePoint,
) -> PrimePoint {
    let k = random_cofactor_part(inst, rng, ops);
    if k.infinity {
        pt
    } else {
        inst.curve.add(ops, pt, k)
    }
}

/// `J_s = [alpha_s] P0 + K_s`, `alpha_s` modulo the group order `n`, `K_s` a
/// random cofactor-part element: the trial point's class in `E / <G>` then
/// diffuses over the whole cofactor group, so no relation type is missing
/// (with `R = [a]P0` alone, `P0` in `<G>` or a non-cyclic cofactor group
/// leaves the class fixed and the system shift-invariant).
fn make_jumps(inst: &Inst, rng: &mut StdRng, ops: &mut GroupOps) -> Jumps {
    let mut pts = vec![];
    let mut al = vec![];
    for _ in 0..16 {
        let a = rng.gen_range(1..inst.n);
        let j = inst.curve.mul(ops, inst.p0, a);
        pts.push(add_cofactor_part(inst, rng, ops, j));
        al.push(a);
    }
    Jumps { pts, al }
}

/// Relation phase: trials `R = [a]G` (a walk with 16 jumps, one priced
/// addition per trial), all decompositions of each `R`, elimination at
/// checkpoints `1, 1.5, 2.25, ...` times the column count, stop when at least
/// 90% of the columns are determined or after `cap` times the columns.
pub fn relation_phase(inst: &Inst, fb: &Fb, m: usize, seed: u64) -> PhaseOut {
    let t0 = std::time::Instant::now();
    let c = &inst.curve;
    let r = inst.r;
    let mut rng = StdRng::seed_from_u64(seed);
    let mut ops = GroupOps::default();
    let mut nat = Native::default();
    let jumps = make_jumps(inst, &mut rng, &mut ops);
    let nn = inst.n;
    let mut a = rng.gen_range(1..nn);
    let mut rp = c.mul(&mut ops, inst.p0, a);
    rp = add_cofactor_part(inst, &mut rng, &mut ops, rp);
    let mut sw = Sweeper::new(fb, c, None);
    let mut rels: Vec<Relation> = vec![];
    let (mut trials, mut trials_hit) = (0u64, 0u64);
    let mut st = PhaseStats {
        fb_points: fb.len(),
        columns: fb.ncols,
        dropped: fb.dropped,
        ..Default::default()
    };
    let mut factor = 1.0f64;
    let cap_factor = 12.0f64;
    let mut known: Vec<Option<u64>> = vec![None; fb.ncols];
    let mut la_total = 0u64;
    let mut la_native_inv = 0u64;
    loop {
        let target = (factor * fb.ncols as f64).ceil() as usize;
        while rels.len() < target {
            trials += 1;
            if rp.infinity {
                a = rng.gen_range(1..nn);
                rp = c.mul(&mut ops, inst.p0, a);
                rp = add_cofactor_part(inst, &mut rng, &mut ops, rp);
            }
            let decs = sw.decompose(rp, m, &mut ops, &mut nat);
            if !decs.is_empty() {
                trials_hit += 1;
            }
            for terms in decs {
                rels.push(Relation { terms, rhs: a % r });
            }
            let s = rng.gen_range(0..16);
            rp = c.add(&mut ops, rp, jumps.pts[s]);
            a = add_mod(a, jumps.al[s], nn);
        }
        // eliminate
        let rows = rows_of(fb, &rels, r);
        let la = solve_sparse(&rows, fb.ncols, r);
        st.la_attempts += 1;
        la_total += la.mul_adds;
        la_native_inv += la.invs;
        st.la_rows = la.rows;
        st.la_cols = la.cols;
        st.la_pivots = la.pivots;
        st.la_max_row = st.la_max_row.max(la.max_row);
        st.la_final_ops = la.mul_adds;
        st.determined_cols = la.determined;
        st.determined_frac = la.determined as f64 / fb.ncols as f64;
        known = la.value;
        if st.determined_frac >= 0.9 {
            st.rule_met = true;
            break;
        }
        if factor >= cap_factor {
            break;
        }
        factor *= 1.5;
    }
    nat.la_ops += la_total;
    nat.invs += la_native_inv;
    st.la_total_ops = la_total;
    st.trials = trials;
    st.trials_with_hit = trials_hit;
    st.relations = rels.len() as u64;
    st.decomp_prob = trials_hit as f64 / trials as f64;
    st.hits_per_trial = rels.len() as f64 / trials as f64;
    st.trials_per_relation = trials as f64 / rels.len().max(1) as f64;
    st.sweep_field_ops_per_trial = (nat.muls + nat.invs - la_native_inv) as f64 / trials as f64;
    st.gae = ops.gae();
    st.native = nat;
    st.build_gae = fb.build_ops.gae();
    st.build_native = fb.build_native;
    st.wall_ms = t0.elapsed().as_millis() as u64;
    PhaseOut { stats: st, known }
}

fn rows_of(fb: &Fb, rels: &[Relation], r: u64) -> Vec<(Vec<(u32, u64)>, u64)> {
    rels.iter()
        .filter_map(|rel| {
            let mut e: Vec<(u32, u64)> = vec![];
            for &(i, eps) in &rel.terms {
                let col = fb.col[i as usize];
                let mut cf = fb.coef[i as usize];
                if eps < 0 {
                    cf = sub_mod(0, cf, r);
                }
                e.push((col, cf));
            }
            e.sort_unstable_by_key(|x| x.0);
            let mut merged: Vec<(u32, u64)> = vec![];
            for (col, cf) in e {
                match merged.last_mut() {
                    Some(l) if l.0 == col => l.1 = add_mod(l.1, cf, r),
                    _ => merged.push((col, cf)),
                }
            }
            merged.retain(|x| x.1 != 0);
            if merged.is_empty() {
                None
            } else {
                Some((merged, rel.rhs))
            }
        })
        .collect()
}

// ---------------------------------------------------------------- targets

/// One target's descent.
#[derive(Clone, Debug, Default, Serialize)]
pub struct TargetStats {
    pub solved: bool,
    pub certified: bool,
    pub recovered: Option<u64>,
    pub trials: u64,
    pub gae: f64,
    pub native: Native,
}

/// Recover `log_G T`: trials `R = T + [a]G`, a decomposition into factor-base
/// points whose columns are all known gives
/// `h (t + a) = sum eps L(P)`; the log is certified by `[k]G == T`.
pub fn solve_target(
    inst: &Inst,
    fb: &Fb,
    known: &[Option<u64>],
    m: usize,
    target: PrimePoint,
    seed: u64,
    max_trials: u64,
) -> TargetStats {
    let c = &inst.curve;
    let r = inst.r;
    let mut rng = StdRng::seed_from_u64(seed);
    let mut ops = GroupOps::default();
    let mut nat = Native::default();
    let avail: Vec<bool> = known.iter().map(|k| k.is_some()).collect();
    let mut sw = Sweeper::new(fb, c, Some(&avail));
    let jumps = make_jumps(inst, &mut rng, &mut ops);
    let nn = inst.n;
    // T'' in <G> with [h]T'' = T; R = [a]P0 + T'' projects to [a + t]G
    let tpp = c.mul(&mut ops, target, inv_mod(inst.h % r, r));
    let mut a = rng.gen_range(1..nn);
    let g_a = c.mul(&mut ops, inst.p0, a);
    let mut rp = c.add(&mut ops, tpp, g_a);
    rp = add_cofactor_part(inst, &mut rng, &mut ops, rp);
    let mut out = TargetStats::default();
    while out.trials < max_trials {
        out.trials += 1;
        if !rp.infinity {
            let decs = sw.decompose(rp, m, &mut ops, &mut nat);
            if let Some(terms) = decs.into_iter().next() {
                let mut sum = 0u64;
                for &(i, eps) in &terms {
                    let l = mm(
                        fb.coef[i as usize],
                        known[fb.col[i as usize] as usize].unwrap(),
                        r,
                    );
                    sum = if eps > 0 {
                        add_mod(sum, l, r)
                    } else {
                        sub_mod(sum, l, r)
                    };
                }
                let t = sub_mod(sum, a % r, r);
                out.recovered = Some(t);
                out.solved = true;
                break;
            }
        }
        let s = rng.gen_range(0..16);
        rp = c.add(&mut ops, rp, jumps.pts[s]);
        a = add_mod(a, jumps.al[s], nn);
    }
    out.gae = ops.gae();
    out.native = nat;
    if let Some(k) = out.recovered {
        // independent certificate, not charged
        let mut o2 = GroupOps::default();
        out.certified = c.mul(&mut o2, inst.g, k) == target;
    }
    out
}

// ---------------------------------------------------------------- rho references

/// A matched rho run's cost.
#[derive(Clone, Debug, Serialize)]
pub struct RhoCost {
    pub method: String,
    pub automorphisms: u32,
    pub gae: f64,
    /// Canonicalisations the unit does not charge.
    pub native: u64,
    pub field_muls: u64,
    pub field_invs: u64,
    pub verified: bool,
    pub certified: bool,
}

/// Which symmetry a rho arm uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RhoKind {
    Plain,
    Negation,
    /// The full automorphism group of `D = -3` (A = 6) or `D = -4` (A = 4).
    Aut,
}

pub fn rho_cost(inst: &Inst, kind: RhoKind, target: PrimePoint, seed: u64) -> Option<RhoCost> {
    let c = &inst.curve;
    let (g, r) = (inst.g, inst.r);
    let max = 400 * r;
    let mut setup = GroupOps::default();
    let res = match kind {
        RhoKind::Plain => {
            rho_walk_with(c, &PointClasses, g, target, r, seed, max, RhoWalk::plain())
        }
        RhoKind::Negation => rho_walk_with(
            c,
            &NegationClasses { r },
            g,
            target,
            r,
            seed,
            max,
            RhoWalk::negation(),
        ),
        RhoKind::Aut => match inst.d {
            -3 => {
                let cl = CubicClasses::new(c, g, r, &mut setup)?;
                rho_walk_with(c, &cl, g, target, r, seed, max, RhoWalk::negation())
            }
            -4 => {
                let cl = QuarticClasses::new(c, g, r, &mut setup)?;
                rho_walk_with(c, &cl, g, target, r, seed, max, RhoWalk::negation())
            }
            _ => return None,
        },
    };
    let k = res.recovered?;
    let mut o2 = GroupOps::default();
    let certified = c.mul(&mut o2, g, k) == target;
    let fo = res.group_ops.field_ops;
    let sfo = setup.field_ops;
    Some(RhoCost {
        method: res.method.clone(),
        automorphisms: res.automorphisms,
        gae: res.gae + setup.gae(),
        native: res
            .counters
            .get("canonicalisations_uncharged")
            .copied()
            .unwrap_or(0),
        field_muls: fo.muls + fo.sqrs + sfo.muls + sfo.sqrs,
        field_invs: fo.invs + sfo.invs,
        verified: res.verified,
        certified,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dlog(inst: &Inst, pt: PrimePoint) -> u64 {
        let mut ops = GroupOps::default();
        let mut acc = PrimePoint::INFINITY;
        for k in 0..inst.r {
            if acc == pt {
                return k;
            }
            acc = inst.curve.add(&mut ops, acc, inst.g);
        }
        panic!("not in subgroup");
    }

    fn inst_for(d: i64, bits: u32) -> Inst {
        construct(d, bits, 7).unwrap_or_else(|| panic!("no instance for d={d}"))
    }

    #[test]
    fn lambda_action_each_d() {
        for d in [-3, -4, -7, -8, -11] {
            let inst = inst_for(d, 14);
            assert_eq!(inst.d_k, Some(d));
            assert_eq!(inst.crater, Some(true));
            let e = endo_for(&inst).unwrap_or_else(|| panic!("no endo for {d}"));
            let c = &inst.curve;
            let mut ops = GroupOps::default();
            let mut nat = Native::default();
            // psi(G) = [lambda]G, and on further subgroup points
            for k in [1u64, 2, 17, 1234] {
                let pt = c.mul(&mut ops, inst.g, k);
                let img = e.apply(c, &mut ops, &mut nat, pt);
                assert!(c.is_on_curve(img), "d={d} k={k} img={img:?} pt={pt:?}");
                assert_eq!(
                    img,
                    c.mul(&mut ops, inst.g, mm(k, e.lambda, inst.r)),
                    "d={d}"
                );
            }
            // psi is additive on random full-group points
            let p1 = c.mul(&mut ops, inst.g, 5);
            let p2 = c.mul(&mut ops, inst.g, 9);
            let s = c.add(&mut ops, p1, p2);
            let lhs = e.apply(c, &mut ops, &mut nat, s);
            let a = e.apply(c, &mut ops, &mut nat, p1);
            let b = e.apply(c, &mut ops, &mut nat, p2);
            assert_eq!(lhs, c.add(&mut ops, a, b));
        }
    }

    #[test]
    fn generic_curve_has_large_dk() {
        let inst = inst_for(0, 14);
        assert!(inst.d_k.is_none_or(|k| k.abs() > 100));
        assert_eq!(inst.n, inst.h * inst.r);
    }

    #[test]
    fn aut_base_is_closed_and_logs_transport() {
        for d in [-3, -4] {
            let inst = inst_for(d, 12);
            let e = endo_for(&inst).unwrap();
            let fb = fb_aut(&inst, &e, 40);
            let c = &inst.curve;
            let mut ops = GroupOps::default();
            let mut nat = Native::default();
            // closed: psi of every point is in the table, ncols < len
            for i in 0..fb.len() {
                let im = e.apply(c, &mut ops, &mut nat, fb.point(i));
                assert!(fb.tab.get(im.x).is_some(), "d={d} not closed");
            }
            assert!(fb.ncols * if d == -3 { 3 } else { 2 } >= fb.len());
            // L(P_i) = coef_i * U_col, with U_col = L(seed)
            let h = inst.h;
            let l = |pt: PrimePoint| {
                let mut o = GroupOps::default();
                dlog(&inst, c.mul(&mut o, pt, h))
            };
            for i in 0..fb.len() {
                let first = (0..fb.len()).find(|&k| fb.col[k] == fb.col[i]).unwrap();
                assert_eq!(
                    l(fb.point(i)),
                    mm(fb.coef[i], l(fb.point(first)), inst.r),
                    "d={d} i={i}"
                );
            }
        }
    }

    #[test]
    fn layered_base_transports_logs_for_psi_and_scalar() {
        for d in [-7, -8, -11] {
            let inst = inst_for(d, 12);
            let e = endo_for(&inst).unwrap();
            let fb = fb_layered(&inst, &e, 24, 4);
            let c = &inst.curve;
            assert!(fb.ncols * 4 >= fb.len());
            let h = inst.h;
            let l = |pt: PrimePoint| {
                let mut o = GroupOps::default();
                dlog(&inst, c.mul(&mut o, pt, h))
            };
            for i in 0..fb.len() {
                let first = (0..fb.len()).find(|&k| fb.col[k] == fb.col[i]).unwrap();
                assert_eq!(
                    l(fb.point(i)),
                    mm(fb.coef[i], l(fb.point(first)), inst.r),
                    "d={d}"
                );
            }
        }
        let inst = inst_for(0, 12);
        let e = scalar_endo(&inst, 3);
        let fb = fb_layered(&inst, &e, 24, 4);
        let h = inst.h;
        let l = |pt: PrimePoint| {
            let mut o = GroupOps::default();
            dlog(&inst, inst.curve.mul(&mut o, pt, h))
        };
        for i in 0..fb.len() {
            let first = (0..fb.len()).find(|&k| fb.col[k] == fb.col[i]).unwrap();
            assert_eq!(l(fb.point(i)), mm(fb.coef[i], l(fb.point(first)), inst.r));
        }
    }

    #[test]
    fn decompositions_verify_and_logs_certify() {
        for (d, m) in [(0, 2), (-3, 2), (-7, 2), (0, 3), (-4, 3)] {
            let inst = inst_for(d, 12);
            let n = (inst.r as f64).powf(1.0 / m as f64).ceil() as usize;
            let (fb_a, fb_b) = if d == 0 || d == -7 {
                let e = if d == 0 {
                    scalar_endo(&inst, 5)
                } else {
                    endo_for(&inst).unwrap()
                };
                (fb_layered(&inst, &e, n.max(8), 4), None::<Fb>)
            } else {
                let e = endo_for(&inst).unwrap();
                (fb_aut(&inst, &e, n.max(8)), None)
            };
            let _ = fb_b;
            let fb = fb_a;
            let out = relation_phase(&inst, &fb, m, 99);
            assert!(out.stats.relations > 0, "d={d} m={m}");
            // decomposition verifies on fresh R
            let mut sw = Sweeper::new(&fb, &inst.curve, None);
            let mut ops = GroupOps::default();
            let mut nat = Native::default();
            let mut found = 0;
            for k in 1..200u64 {
                let rp = inst.curve.mul(&mut ops, inst.g, 31 * k + 7);
                for terms in sw.decompose(rp, m, &mut ops, &mut nat) {
                    assert!(check_decomposition(&inst.curve, &fb, rp, &terms));
                    found += 1;
                }
            }
            assert!(found > 0, "d={d} m={m}: no decomposition found");
            // every determined column equals the true log
            for (col, v) in out.known.iter().enumerate() {
                if let Some(v) = v {
                    let i = (0..fb.len()).find(|&i| fb.col[i] as usize == col).unwrap();
                    let mut o = GroupOps::default();
                    let truth = dlog(&inst, inst.curve.mul(&mut o, fb.point(i), inst.h));
                    assert_eq!(mm(fb.coef[i], *v, inst.r), truth, "d={d} m={m} col={col}");
                }
            }
            // targets certified
            let mut rng = StdRng::seed_from_u64(4);
            let mut ok = 0;
            for k in 0..6 {
                let t = rng.gen_range(1..inst.r);
                let tp = inst.curve.mul(&mut ops, inst.g, t);
                let ts = solve_target(&inst, &fb, &out.known, m, tp, 1000 + k, 5000);
                if ts.solved {
                    assert!(ts.certified);
                    assert_eq!(ts.recovered, Some(t));
                    ok += 1;
                }
            }
            assert!(ok >= 3, "d={d} m={m}: only {ok}/6 targets solved");
        }
    }

    #[test]
    fn s3_quadratic_roots_are_the_sum_and_difference() {
        let inst = inst_for(0, 12);
        let c = &inst.curve;
        let p = c.p;
        let s3 = |x1: u64, x2: u64, x3: u64| {
            let (a, b) = (c.a, c.b);
            let d = sub_mod(x1, x2, p);
            let t1 = mm(mm(d, d, p), mm(x3, x3, p), p);
            let inner = add_mod(
                mm(add_mod(x1, x2, p), add_mod(mm(x1, x2, p), a, p), p),
                mm(2, b, p),
                p,
            );
            let t2 = mm(mm(2, x3, p), inner, p);
            let q = sub_mod(mm(x1, x2, p), a, p);
            let t3 = sub_mod(mm(q, q, p), mm(4, mm(b, add_mod(x1, x2, p), p), p), p);
            add_mod(sub_mod(t1, t2, p), t3, p)
        };
        let mut ops = GroupOps::default();
        let p1 = c.mul(&mut ops, inst.g, 11);
        let p2 = c.mul(&mut ops, inst.g, 29);
        let sum = c.add(&mut ops, p1, p2);
        let dif = c.add(&mut ops, p1, c.neg(p2));
        assert_eq!(s3(p1.x, p2.x, sum.x), 0);
        assert_eq!(s3(p1.x, p2.x, dif.x), 0);
        assert_ne!(s3(p1.x, p2.x, (sum.x + 1) % p), 0);
    }

    #[test]
    fn sparse_solver_matches_truth() {
        let r = 10007u64;
        let n = 30;
        let truth: Vec<u64> = (0..n).map(|i| (i * 977 + 13) % r).collect();
        let mut rng = StdRng::seed_from_u64(1);
        let mut rows = vec![];
        for _ in 0..90 {
            let a = rng.gen_range(0..n);
            let b = rng.gen_range(0..n);
            if a == b {
                continue;
            }
            let (ca, cb) = (rng.gen_range(1..r), rng.gen_range(1..r));
            let rhs = add_mod(
                mm(ca, truth[a as usize], r),
                mm(cb, truth[b as usize], r),
                r,
            );
            let mut e = vec![(a as u32, ca), (b as u32, cb)];
            e.sort_unstable_by_key(|x| x.0);
            rows.push((e, rhs));
        }
        let out = solve_sparse(&rows, n as usize, r);
        assert!(out.consistent);
        assert!(out.determined >= 20);
        for (i, v) in out.value.iter().enumerate() {
            if let Some(v) = v {
                assert_eq!(*v, truth[i]);
            }
        }
    }
}
