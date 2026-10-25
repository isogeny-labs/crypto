//! Wide quartic arithmetic/decomposition core for the C2 width work.
//!
//! Coefficients are u32; packed fields, orders and scalars are u128.
//! This module has no legacy CountedGroup/stage adapter. Full wide IC and
//! ecbench integration remain unverified/open; the original backend and
//! frozen method identities are separate. See WIDE_CORE_PROTOCOL.md.
//!
//! **Gaudry's `k = 4` decomposition in characteristic 2, with FGHR's
//! 2-torsion symmetry: `E/F_{q⁴}`, `q = 2^m`, the factor base on a line
//! in a coordinate where the 2-torsion translation is `t ↦ t + 1`.**
//!
//! The characteristic-2 counterpart of [`super::gaudry_quartic_d4`].  An
//! ordinary curve `E: y² + xy = x³ + ax² + b` over `F_{q⁴}` always has the
//! rational 2-torsion point `T = (0, √b)`, and `x(P + T) = √b / x(P)`.  With
//! `β = b^{1/4}` and
//!
//! ```text
//!     t = β / (x + β),        x = β (t + 1) / t,
//! ```
//!
//! the translation is `t(P + T) = t(P) + 1` for every `a, b` (no subfield
//! condition), so the base `{P : t(P) ∈ F_q}` is `T`-stable.  Since
//! `2T = 0`, translating an even number of the four summands keeps their
//! sum, and `H(t₁, …, t₄; t_R) = S₅(x(t₁), …, x(t_R)) · Π t_i⁸ · t_R⁸` is
//! invariant under permutations of the `t_i` and even numbers of
//! translations `t_i ↦ t_i + 1`.  The translations' invariants are
//! `w_i = t_i² + t_i` and `π = t₁ + t₂ + t₃ + t₄` with `π² + π = Σ w_i`;
//! with the symmetric group, the invariant ring is polynomial in
//! `s₂, s₃, s₄ = e_k(w)` and `π`, where `s₁ = π² + π`.  Each `s_k` has
//! degree 2 in every `t_i` and `π` degree 1: the weights `(2, 2, 2, 1)` of
//! the odd-characteristic system, the same 105 invariant monomials, the same
//! Hilbert series and the same weighted Macaulay degree 26.
//!
//! Everything else is over `F_q = F_{2^m}` (table arithmetic, every
//! multiplication counted): the interpolation of `H` on the 105 monomials
//! (checked on fresh points), its Weil restriction to four equations over
//! `F_q`, their weighted Macaulay solve with the eigen step along the chains
//! of `π` ([`solve_bin`], the binary form of `gaudry_quartic`'s
//! `solve_along_chains`), the split of each solution back to four points,
//! and an independent meet-in-the-middle enumeration over the same base.

use std::cell::Cell;
use std::collections::{BTreeSet, HashMap};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use serde::Serialize;

// ── F_{2^m} ───────────────────────────────────────────────────────────────

/// Primitive polynomials of degree `m` (the `x^m` bit included).
const PRIMITIVE: [u32; 19] = [
    0, 0, 0x7, 0xB, 0x13, 0x25, 0x43, 0x89, 0x11D, 0x211, 0x409, 0x805, 0x1053, 0x201B, 0x4443,
    0x8003, 0x1100B, 0x20009, 0x40081,
];

/// The logarithm tables of `F_{2^m}`: shareable across threads, no
/// counter (the solver's loops count for themselves).
pub struct GfTab {
    pub q: u32,
    exp: Vec<u32>,
    log: Vec<u32>,
}

impl GfTab {
    #[inline(always)]
    pub fn mul(&self, a: u32, b: u32) -> u32 {
        if a == 0 || b == 0 {
            0
        } else {
            self.exp[(self.log[a as usize] + self.log[b as usize]) as usize]
        }
    }
    #[inline(always)]
    pub fn inv(&self, a: u32) -> u32 {
        assert!(a != 0, "inverse of 0");
        self.exp[((self.q - 1 - self.log[a as usize]) % (self.q - 1)) as usize]
    }
    /// `row ← row + f · piv` elementwise (`f ≠ 0`).
    #[inline(always)]
    pub fn axpy(&self, row: &mut [u32], piv: &[u32], f: u32) {
        let lf = self.log[f as usize];
        for (r, &v) in row.iter_mut().zip(piv) {
            if v != 0 {
                *r ^= self.exp[(lf + self.log[v as usize]) as usize];
            }
        }
    }
}

/// `F_{2^m}`, `2 ≤ m ≤ 18`, by logarithm tables.  `mul` and `inv` count
/// one multiplication each on [`Gf2q::muls`]; the `_nc` forms are for
/// loops that count for themselves.
pub struct Gf2q {
    pub m: u32,
    pub q: u32,
    pub modulus: u32,
    pub tab: GfTab,
    /// `t` with `t² + t = w`, or `u32::MAX` when `Tr(w) ≠ 0`.
    artin: Vec<u32>,
    muls: Cell<u64>,
}

impl Gf2q {
    pub fn new(m: u32) -> Gf2q {
        assert!((2..=18).contains(&m), "2 ≤ m ≤ 18");
        let q = 1u32 << m;
        let modulus = PRIMITIVE[m as usize];
        let mut exp = vec![0u32; 2 * q as usize];
        let mut log = vec![0u32; q as usize];
        let mut v = 1u32;
        for i in 0..(q - 1) {
            exp[i as usize] = v as u32;
            assert!(i == 0 || v != 1, "x is primitive modulo {modulus:#x}");
            log[v as usize] = i;
            v <<= 1;
            if v & q != 0 {
                v ^= modulus;
            }
        }
        assert_eq!(v, 1, "x has order q − 1");
        for i in (q - 1)..(2 * q) {
            exp[i as usize] = exp[(i - (q - 1)) as usize];
        }
        let mut f = Gf2q {
            m,
            q,
            modulus,
            tab: GfTab { q, exp, log },
            artin: vec![u32::MAX; q as usize],
            muls: Cell::new(0),
        };
        for t in 0..q as u32 {
            let w = f.mul_nc(t, t) ^ t;
            f.artin[w as usize] = t.min(t ^ 1);
        }
        f
    }
    pub fn muls(&self) -> u64 {
        self.muls.get()
    }
    pub fn count(&self, k: u64) {
        self.muls.set(self.muls.get() + k);
    }
    #[inline(always)]
    pub fn mul_nc(&self, a: u32, b: u32) -> u32 {
        self.tab.mul(a, b)
    }
    #[inline(always)]
    pub fn mul(&self, a: u32, b: u32) -> u32 {
        self.count(1);
        self.tab.mul(a, b)
    }
    pub fn inv_nc(&self, a: u32) -> u32 {
        self.tab.inv(a)
    }
    pub fn inv(&self, a: u32) -> u32 {
        self.count(1);
        self.tab.inv(a)
    }
    /// The smaller `t` with `t² + t = w`, if any (a table: no arithmetic).
    pub fn artin_schreier(&self, w: u32) -> Option<u32> {
        let t = self.artin[w as usize];
        (t != u32::MAX).then_some(t)
    }
    /// `√a = a^{2^{m−1}}`: `m − 1` squarings, counted.
    pub fn sqrt(&self, a: u32) -> u32 {
        let mut r = a;
        for _ in 1..self.m {
            r = self.mul(r, r);
        }
        r
    }
}

// ── F_{q⁴} = F_q[z] / (z⁴ − c₃z³ − c₂z² − c₁z − c₀) ───────────────────────

/// An element of `F_{q⁴}`: coefficients of `1, z, z², z³` over `F_q`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Eb(pub [u32; 4]);

impl Eb {
    pub const ZERO: Eb = Eb([0; 4]);
    pub const ONE: Eb = Eb([1, 0, 0, 0]);
    pub fn is_zero(&self) -> bool {
        self.0 == [0; 4]
    }
    pub fn in_fq(&self) -> bool {
        self.0[1] == 0 && self.0[2] == 0 && self.0[3] == 0
    }
    pub fn add(&self, o: &Eb) -> Eb {
        Eb(core::array::from_fn(|i| self.0[i] ^ o.0[i]))
    }
}

pub struct Fq4 {
    pub f: Gf2q,
    /// `z⁴ = c₀ + c₁z + c₂z² + c₃z³`.
    pub c: [u32; 4],
    /// `(zⁱ)^q`, for the Frobenius.
    zq: [Eb; 4],
}

impl Fq4 {
    /// The first `z⁴ + c₃z³ + c₁z + c₀`, `c₃ ∈ {0, 1}` (by `(c₃, c₁, c₀)` in
    /// order) irreducible over `F_q`: no factor of degree 1 or 2, i.e.
    /// `z^{q²} − z` a unit modulo it.
    pub fn new(m: u32) -> Fq4 {
        assert!((4..=18).contains(&m), "4 ≤ m ≤ 18");
        let mut fq = Fq4 {
            f: Gf2q::new(m),
            c: [0; 4],
            zq: [Eb::ONE; 4],
        };
        // z⁴ + c₁z + c₀ is an affine linearised polynomial in characteristic
        // 2, often reducible for every (c₁, c₀); the z³ term avoids that.
        for (c3, c1) in (0..2u32).flat_map(|c3| (0..fq.f.q as u32).map(move |c1| (c3, c1))) {
            for c0 in 1..fq.f.q as u32 {
                fq.c = [c0, c1, 0, c3];
                let z = Eb([0, 1, 0, 0]);
                let d = fq.pow2k(&z, 2 * m).add(&z);
                if d.is_zero() || fq.mat_rank(&d) != 4 {
                    continue;
                }
                let zq = fq.pow2k(&z, m);
                let mut acc = Eb::ONE;
                for i in 0..4 {
                    fq.zq[i] = acc;
                    acc = fq.mul_nc(&acc, &zq);
                }
                fq.f.muls.set(0);
                return fq;
            }
        }
        unreachable!("an irreducible quartic exists")
    }
    /// Rank over `F_q` of multiplication by `d` (4 means a unit).
    fn mat_rank(&self, d: &Eb) -> usize {
        let f = &self.f;
        let mut m: Vec<[u32; 4]> = (0..4)
            .map(|i| {
                let mut e = Eb::ZERO;
                e.0[i] = 1;
                self.mul_nc(d, &e).0
            })
            .collect();
        let mut rank = 0;
        for c in 0..4 {
            let Some(r) = (rank..4).find(|&r| m[r][c] != 0) else {
                continue;
            };
            m.swap(r, rank);
            let inv = f.inv_nc(m[rank][c]);
            let piv = m[rank];
            for rr in 0..4 {
                if rr != rank && m[rr][c] != 0 {
                    let k = f.mul_nc(m[rr][c], inv);
                    for j in 0..4 {
                        m[rr][j] ^= f.mul_nc(k, piv[j]);
                    }
                }
            }
            rank += 1;
        }
        rank
    }
    pub fn from_fq(&self, v: u32) -> Eb {
        Eb([v, 0, 0, 0])
    }
    fn reduce(&self, p: &mut [u32; 7], counted: bool) -> Eb {
        let f = &self.f;
        for k in (4..7).rev() {
            let v = p[k];
            if v == 0 {
                continue;
            }
            for i in 0..4 {
                if self.c[i] != 0 {
                    p[k - 4 + i] ^= f.mul_nc(v, self.c[i]);
                    if counted {
                        f.count(1);
                    }
                }
            }
        }
        Eb([p[0], p[1], p[2], p[3]])
    }
    fn mul_impl(&self, a: &Eb, b: &Eb, counted: bool) -> Eb {
        let f = &self.f;
        let mut p = [0u32; 7];
        for i in 0..4 {
            if a.0[i] == 0 {
                continue;
            }
            for j in 0..4 {
                if b.0[j] != 0 {
                    p[i + j] ^= f.mul_nc(a.0[i], b.0[j]);
                    if counted {
                        f.count(1);
                    }
                }
            }
        }
        self.reduce(&mut p, counted)
    }
    pub fn mul(&self, a: &Eb, b: &Eb) -> Eb {
        self.mul_impl(a, b, true)
    }
    fn mul_nc(&self, a: &Eb, b: &Eb) -> Eb {
        self.mul_impl(a, b, false)
    }
    /// `a²`: the coefficients squared into even positions, then reduced.
    pub fn sq(&self, a: &Eb) -> Eb {
        let f = &self.f;
        let mut p = [0u32; 7];
        for i in 0..4 {
            if a.0[i] != 0 {
                p[2 * i] = f.mul(a.0[i], a.0[i]);
            }
        }
        self.reduce(&mut p, true)
    }
    fn sq_nc(&self, a: &Eb) -> Eb {
        let f = &self.f;
        let mut p = [0u32; 7];
        for i in 0..4 {
            p[2 * i] = f.mul_nc(a.0[i], a.0[i]);
        }
        self.reduce(&mut p, false)
    }
    /// `a^{2^k}` (uncounted: setup only).
    fn pow2k(&self, a: &Eb, k: u32) -> Eb {
        let mut r = *a;
        for _ in 0..k {
            r = self.sq_nc(&r);
        }
        r
    }
    pub fn scale(&self, a: &Eb, k: u32) -> Eb {
        Eb(core::array::from_fn(|i| self.f.mul(a.0[i], k)))
    }
    /// `a^q`: `F_q`-linear, `Σ aᵢ (zⁱ)^q`.
    pub fn frob(&self, a: &Eb) -> Eb {
        let mut r = Eb([a.0[0], 0, 0, 0]);
        for i in 1..4 {
            if a.0[i] != 0 {
                r = r.add(&self.scale(&self.zq[i], a.0[i]));
            }
        }
        r
    }
    /// `a⁻¹ = a^{q + q² + q³} / N(a)`, `N(a) = a · a^{q + q² + q³} ∈ F_q`.
    pub fn inv(&self, a: &Eb) -> Eb {
        assert!(!a.is_zero(), "inverse of 0");
        let a1 = self.frob(a);
        let a2 = self.frob(&a1);
        let a3 = self.frob(&a2);
        let c = self.mul(&self.mul(&a1, &a2), &a3);
        let n = self.mul(a, &c);
        debug_assert!(n.in_fq(), "the norm lies in F_q");
        self.scale(&c, self.f.inv(n.0[0]))
    }
    /// Whether `a ∈ F_{q²}` (`a^{q²} = a`; uncounted, a diagnostic).
    pub fn in_fq2(&self, a: &Eb) -> bool {
        self.pow2k(a, 2 * self.f.m) == *a
    }
    /// `a^{2^k}`, counted.
    pub fn pow2k_counted(&self, a: &Eb, k: u32) -> Eb {
        let mut r = *a;
        for _ in 0..k {
            r = self.sq(&r);
        }
        r
    }
    /// `√a = a^{2^{4m − 1}}`.
    pub fn sqrt(&self, a: &Eb) -> Eb {
        self.pow2k_counted(a, 4 * self.f.m - 1)
    }
    pub fn random(&self, rng: &mut StdRng) -> Eb {
        Eb(core::array::from_fn(|_| rng.gen_range(0..self.f.q) as u32))
    }
    /// Pack into one word: coefficient `i` in bits `[im, (i + 1)m)`.
    pub fn pack(&self, a: &Eb) -> u128 {
        assert!(
            a.0.iter().all(|&v| v < self.f.q),
            "coefficient outside base field"
        );
        (0..4).fold(0u128, |acc, i| {
            acc | (a.0[i] as u128) << (i as u32 * self.f.m)
        })
    }
    pub fn unpack(&self, v: u128) -> Eb {
        assert!(
            v < (1u128 << (4 * self.f.m)),
            "packed element exceeds field width"
        );
        let mask = (1u128 << self.f.m) - 1;
        Eb(core::array::from_fn(|i| {
            ((v >> (i as u32 * self.f.m)) & mask) as u32
        }))
    }
}

// ── The curve y² + xy = x³ + ax² + b over F_{q⁴} ─────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PtB {
    pub x: Eb,
    pub y: Eb,
    pub inf: bool,
}

impl PtB {
    pub const INF: PtB = PtB {
        x: Eb::ZERO,
        y: Eb::ZERO,
        inf: true,
    };
}

pub struct CurveB {
    pub f: Fq4,
    pub a: Eb,
    pub b: Eb,
    /// `β = b^{1/4}`.
    pub beta: Eb,
    /// `u ↦ u² + u` over `F_2` on the packed bits, row-reduced with its
    /// transformation, for the lift.
    as_solver: Vec<(u128, u128)>,
    ops: Cell<u64>,
}

impl CurveB {
    pub fn new(f: Fq4, a: Eb, b: Eb) -> CurveB {
        assert!(!b.is_zero(), "b ≠ 0: the curve is ordinary and smooth");
        let beta = f.pow2k(&b, 4 * f.f.m - 2);
        assert_eq!(f.sq_nc(&f.sq_nc(&beta)), b, "β⁴ = b");
        let n = 4 * f.f.m;
        // Rows: image of each basis bit, with the bit itself as the
        // transformation; reduce to echelon form by leading bit.
        let mut rows: Vec<(u128, u128)> = (0..n)
            .map(|i| {
                let e = f.unpack(1u128 << i);
                let img = f.sq_nc(&e).add(&e);
                (f.pack(&img), 1u128 << i)
            })
            .collect();
        let mut basis: Vec<(u128, u128)> = Vec::new();
        for (mut v, mut tr) in rows.drain(..) {
            // Basis kept by leading bit, descending; each lead unique.
            for &(bv, btr) in &basis {
                let lead = 127 - bv.leading_zeros();
                if v >> lead & 1 == 1 {
                    v ^= bv;
                    tr ^= btr;
                }
            }
            if v != 0 {
                basis.push((v, tr));
                basis.sort_by_key(|x| std::cmp::Reverse(127 - x.0.leading_zeros()));
            }
        }
        f.f.muls.set(0);
        CurveB {
            f,
            a,
            b,
            beta,
            as_solver: basis,
            ops: Cell::new(0),
        }
    }
    pub fn ops(&self) -> u64 {
        self.ops.get()
    }
    /// A `u` with `u² + u = c`, if `Tr(c) = 0` (linear algebra over `F_2`,
    /// no field multiplication).
    fn artin_schreier(&self, c: &Eb) -> Option<Eb> {
        let mut v = self.f.pack(c);
        let mut u = 0u128;
        for &(bv, btr) in &self.as_solver {
            let lead = 127 - bv.leading_zeros();
            if v >> lead & 1 == 1 {
                v ^= bv;
                u ^= btr;
            }
        }
        (v == 0).then(|| self.f.unpack(u))
    }
    /// A point with abscissa `x`, if one exists; the ordinate's choice of
    /// the two is the sampler's.
    pub fn lift_x(&self, x: &Eb, rng: &mut StdRng) -> Option<PtB> {
        let f = &self.f;
        if x.is_zero() {
            return Some(PtB {
                x: *x,
                y: f.sqrt(&self.b),
                inf: false,
            });
        }
        let x2 = f.sq(x);
        let c = x.add(&self.a).add(&f.mul(&self.b, &f.inv(&x2)));
        let u = self.artin_schreier(&c)?;
        let u = if rng.gen_bool(0.5) {
            u
        } else {
            u.add(&Eb::ONE)
        };
        Some(PtB {
            x: *x,
            y: f.mul(x, &u),
            inf: false,
        })
    }
    pub fn on_curve(&self, p: &PtB) -> bool {
        if p.inf {
            return true;
        }
        let f = &self.f;
        let lhs = f.sq(&p.y).add(&f.mul(&p.x, &p.y));
        let x2 = f.sq(&p.x);
        let rhs = f.mul(&x2, &p.x).add(&f.mul(&self.a, &x2)).add(&self.b);
        lhs == rhs
    }
    pub fn neg(&self, p: &PtB) -> PtB {
        if p.inf {
            return *p;
        }
        PtB {
            x: p.x,
            y: p.x.add(&p.y),
            inf: false,
        }
    }
    pub fn add(&self, p: &PtB, q: &PtB) -> PtB {
        self.ops.set(self.ops.get() + 1);
        if p.inf {
            return *q;
        }
        if q.inf {
            return *p;
        }
        let f = &self.f;
        if p.x == q.x {
            if p.y != q.y || p.x.is_zero() {
                return PtB::INF;
            }
            // Doubling: λ = x + y/x, x₃ = λ² + λ + a, y₃ = x² + (λ + 1)x₃.
            let l = p.x.add(&f.mul(&p.y, &f.inv(&p.x)));
            let x3 = f.sq(&l).add(&l).add(&self.a);
            let y3 = f.sq(&p.x).add(&f.mul(&l.add(&Eb::ONE), &x3));
            return PtB {
                x: x3,
                y: y3,
                inf: false,
            };
        }
        let l = f.mul(&p.y.add(&q.y), &f.inv(&p.x.add(&q.x)));
        let x3 = f.sq(&l).add(&l).add(&p.x).add(&q.x).add(&self.a);
        let y3 = f.mul(&l, &p.x.add(&x3)).add(&x3).add(&p.y);
        PtB {
            x: x3,
            y: y3,
            inf: false,
        }
    }
    pub fn sub(&self, p: &PtB, q: &PtB) -> PtB {
        self.add(p, &self.neg(q))
    }
    pub fn mul(&self, p: &PtB, mut k: u128) -> PtB {
        let mut acc = PtB::INF;
        let mut base = *p;
        while k > 0 {
            if k & 1 == 1 {
                acc = self.add(&acc, &base);
            }
            base = self.add(&base, &base);
            k >>= 1;
        }
        acc
    }
    /// `T = (0, √b)`.
    pub fn torsion(&self) -> PtB {
        PtB {
            x: Eb::ZERO,
            y: self.f.sqrt(&self.b),
            inf: false,
        }
    }
    /// `t = β / (x + β)` (`x = β` is the order-4 point, `t = ∞`).
    pub fn t_of_x(&self, x: &Eb) -> Eb {
        let f = &self.f;
        f.mul(&self.beta, &f.inv(&x.add(&self.beta)))
    }
    /// `x = β (t + 1) / t`.
    pub fn x_of_t(&self, t: &Eb) -> Eb {
        let f = &self.f;
        f.mul(&f.mul(&self.beta, &t.add(&Eb::ONE)), &f.inv(t))
    }

    /// `S₃` as a quadratic in its last argument `U`: `(x₁² + x₂²)U² +
    /// x₁x₂U + x₁²x₂² + b`.
    fn s3_quadratic(&self, x1: &Eb, x2: &Eb) -> [Eb; 3] {
        let f = &self.f;
        let s1 = f.sq(x1);
        let s2 = f.sq(x2);
        [f.mul(&s1, &s2).add(&self.b), f.mul(x1, x2), s1.add(&s2)]
    }
    /// `S₃(x₁, x₂, x₃) = x₁²x₂² + x₁²x₃² + x₂²x₃² + x₁x₂x₃ + b`.
    pub fn s3(&self, x1: &Eb, x2: &Eb, x3: &Eb) -> Eb {
        let f = &self.f;
        let [c, bq, a] = self.s3_quadratic(x1, x2);
        f.mul(&a, &f.sq(x3)).add(&f.mul(&bq, x3)).add(&c)
    }
    /// `S₄ = Res_U(S₃(x₁, x₂, U), S₃(x₃, x₄, U))`: for quadratics
    /// `(A, B, C)`, `(A₁C₂ + A₂C₁)² + (A₁B₂ + A₂B₁)(B₁C₂ + B₂C₁)`.
    pub fn s4(&self, x1: &Eb, x2: &Eb, x3: &Eb, x4: &Eb) -> Eb {
        let f = &self.f;
        let [c1, b1, a1] = self.s3_quadratic(x1, x2);
        let [c2, b2, a2] = self.s3_quadratic(x3, x4);
        let ac = f.mul(&a1, &c2).add(&f.mul(&a2, &c1));
        let ab = f.mul(&a1, &b2).add(&f.mul(&a2, &b1));
        let bc = f.mul(&b1, &c2).add(&f.mul(&b2, &c1));
        f.sq(&ac).add(&f.mul(&ab, &bc))
    }
    /// `S₅(x₁, …, x₅) = Res_U(S₃(x₁, x₂, U), S₄(x₃, x₄, x₅, U))`, `S₄` of
    /// formal degree 4 in `U` interpolated on five nodes, the resultant as
    /// the 6 × 6 Sylvester determinant.
    pub fn s5(&self, x: &[Eb; 5]) -> Eb {
        let f = &self.f;
        let nodes: [Eb; 5] = core::array::from_fn(|i| Eb([i as u32, 1, 0, 0]));
        let vals: [Eb; 5] = core::array::from_fn(|i| self.s4(&x[2], &x[3], &x[4], &nodes[i]));
        let g = interpolate(f, &nodes, &vals);
        let [c, bq, a] = self.s3_quadratic(&x[0], &x[1]);
        // Sylvester: 4 rows of (A, B, C) shifted, 2 rows of g (degree 4).
        let mut m = vec![vec![Eb::ZERO; 6]; 6];
        for r in 0..4 {
            m[r][r] = a;
            m[r][r + 1] = bq;
            m[r][r + 2] = c;
        }
        for r in 0..2 {
            for k in 0..5 {
                m[4 + r][r + k] = g[4 - k];
            }
        }
        det_e(f, m)
    }
}

/// The polynomial of degree `< n` through `(nodes[i], vals[i])`, ascending.
fn interpolate(f: &Fq4, nodes: &[Eb], vals: &[Eb]) -> Vec<Eb> {
    let n = nodes.len();
    let mut out = vec![Eb::ZERO; n];
    for i in 0..n {
        // basis polynomial Π_{j≠i} (U − u_j) / (u_i − u_j)
        let mut poly = vec![Eb::ONE];
        let mut den = Eb::ONE;
        for j in 0..n {
            if j == i {
                continue;
            }
            let mut next = vec![Eb::ZERO; poly.len() + 1];
            for (k, c) in poly.iter().enumerate() {
                next[k + 1] = next[k + 1].add(c);
                next[k] = next[k].add(&f.mul(c, &nodes[j]));
            }
            poly = next;
            den = f.mul(&den, &nodes[i].add(&nodes[j]));
        }
        let s = f.mul(&vals[i], &f.inv(&den));
        for (k, c) in poly.iter().enumerate() {
            out[k] = out[k].add(&f.mul(c, &s));
        }
    }
    out
}

fn det_e(f: &Fq4, mut m: Vec<Vec<Eb>>) -> Eb {
    let n = m.len();
    let mut det = Eb::ONE;
    for c in 0..n {
        let Some(r) = (c..n).find(|&r| !m[r][c].is_zero()) else {
            return Eb::ZERO;
        };
        m.swap(r, c);
        det = f.mul(&det, &m[c][c]);
        let inv = f.inv(&m[c][c]);
        for r in (c + 1)..n {
            if m[r][c].is_zero() {
                continue;
            }
            let k = f.mul(&m[r][c], &inv);
            for j in c..n {
                let v = f.mul(&k, &m[c][j]);
                m[r][j] = m[r][j].add(&v);
            }
        }
    }
    det
}

// ── The D₄-symmetrised S₅ and its Weil restriction ───────────────────────

/// `s₂^a s₃^b s₄^c π^d` with `2(a + b + c) + d ≤ 8`: 105 monomials, as
/// `[a, b, c, d]`.
pub fn invariant_monomials() -> Vec<[u8; 4]> {
    let mut v = Vec::new();
    for d in 0..=8u8 {
        let k = (8 - d) / 2;
        for a in 0..=k {
            for b in 0..=(k - a) {
                for c in 0..=(k - a - b) {
                    v.push([a, b, c, d]);
                }
            }
        }
    }
    v
}

/// `(s₂, s₃, s₄, π)` of `(t₁, …, t₄)`.
fn invariants(f: &Fq4, t: &[Eb; 4]) -> [Eb; 4] {
    let w: Vec<Eb> = t.iter().map(|v| f.sq(v).add(v)).collect();
    let mut e = [Eb::ONE, Eb::ZERO, Eb::ZERO, Eb::ZERO, Eb::ZERO];
    for wi in &w {
        for k in (1..5).rev() {
            e[k] = e[k].add(&f.mul(&e[k - 1], wi));
        }
    }
    let pi = t[0].add(&t[1]).add(&t[2]).add(&t[3]);
    [e[2], e[3], e[4], pi]
}

fn mono_values(f: &Fq4, inv: &[Eb; 4], monos: &[[u8; 4]]) -> Vec<Eb> {
    let mut pows = [[Eb::ONE; 9]; 4];
    for i in 0..4 {
        for k in 1..9 {
            pows[i][k] = f.mul(&pows[i][k - 1], &inv[i]);
        }
    }
    monos
        .iter()
        .map(|m| {
            f.mul(
                &f.mul(&pows[0][m[0] as usize], &pows[1][m[1] as usize]),
                &f.mul(&pows[2][m[2] as usize], &pows[3][m[3] as usize]),
            )
        })
        .collect()
}

/// `H = Σ_m coef[m](t_R) · m(s₂, s₃, s₄, π)`, nine `F_{q⁴}` coefficients in
/// `t_R` per monomial.
pub struct SymmetrisedBin {
    pub monos: Vec<[u8; 4]>,
    pub coef: Vec<[Eb; 9]>,
    pub checked_points: usize,
    pub precompute_muls: u64,
}

impl SymmetrisedBin {
    fn h(curve: &CurveB, t: &[Eb; 4], tr: &Eb) -> Eb {
        let f = &curve.f;
        let x: [Eb; 5] = [
            curve.x_of_t(&t[0]),
            curve.x_of_t(&t[1]),
            curve.x_of_t(&t[2]),
            curve.x_of_t(&t[3]),
            curve.x_of_t(tr),
        ];
        let mut v = curve.s5(&x);
        for ti in t.iter().chain(std::iter::once(tr)) {
            v = f.mul(&v, &f.pow2k_counted(ti, 3));
        }
        v
    }

    /// Nine distinct nonzero nodes for `t_R`, outside `F_q`.
    fn nodes() -> [Eb; 9] {
        core::array::from_fn(|i| Eb([i as u32, 1, 0, 0]))
    }

    fn coefficients_in_last(curve: &CurveB, t: &[Eb; 4], vinv: &[[Eb; 9]; 9]) -> [Eb; 9] {
        let f = &curve.f;
        let vals: Vec<Eb> = Self::nodes().iter().map(|n| Self::h(curve, t, n)).collect();
        core::array::from_fn(|j| {
            vals.iter()
                .enumerate()
                .fold(Eb::ZERO, |acc, (k, v)| acc.add(&f.mul(v, &vinv[j][k])))
        })
    }

    fn random_t(f: &Fq4, rng: &mut StdRng) -> [Eb; 4] {
        core::array::from_fn(|_| loop {
            let v = f.random(rng);
            if !v.is_zero() && v != Eb::ONE {
                break v;
            }
        })
    }

    /// Interpolate on 105 random points and check on `checks` fresh ones;
    /// `Err` if `H` is not in the span of the invariant monomials.
    pub fn precompute(curve: &CurveB, rng: &mut StdRng, checks: usize) -> Result<Self, String> {
        let f = &curve.f;
        let before = f.f.muls();
        let nodes = Self::nodes();
        // Inverse Vandermonde: coefficient j of the polynomial through the
        // nodes is Σ_k vinv[j][k] · value_k.
        let vinv: [[Eb; 9]; 9] = {
            let mut out = [[Eb::ZERO; 9]; 9];
            for k in 0..9 {
                let vals: Vec<Eb> = (0..9)
                    .map(|i| if i == k { Eb::ONE } else { Eb::ZERO })
                    .collect();
                let poly = interpolate(f, &nodes, &vals);
                for j in 0..9 {
                    out[j][k] = poly[j];
                }
            }
            out
        };
        let monos = invariant_monomials();
        let n = monos.len();
        for _attempt in 0..8 {
            let mut sys: Vec<Vec<Eb>> = Vec::with_capacity(n);
            for _ in 0..n {
                let t = Self::random_t(f, rng);
                let mut row = mono_values(f, &invariants(f, &t), &monos);
                row.extend(Self::coefficients_in_last(curve, &t, &vinv));
                sys.push(row);
            }
            let mut ok = true;
            for c in 0..n {
                let Some(r) = (c..n).find(|&r| !sys[r][c].is_zero()) else {
                    ok = false;
                    break;
                };
                sys.swap(r, c);
                let inv = f.inv(&sys[c][c]);
                let pivot: Vec<Eb> = sys[c].iter().map(|v| f.mul(v, &inv)).collect();
                sys[c] = pivot.clone();
                for r in 0..n {
                    if r == c || sys[r][c].is_zero() {
                        continue;
                    }
                    let k = sys[r][c];
                    for j in c..pivot.len() {
                        if !pivot[j].is_zero() {
                            let v = f.mul(&k, &pivot[j]);
                            sys[r][j] = sys[r][j].add(&v);
                        }
                    }
                }
            }
            if !ok {
                continue;
            }
            let coef: Vec<[Eb; 9]> = (0..n)
                .map(|r| core::array::from_fn(|j| sys[r][n + j]))
                .collect();
            for t_ in 0..checks {
                let t = Self::random_t(f, rng);
                let want = Self::coefficients_in_last(curve, &t, &vinv);
                let mv = mono_values(f, &invariants(f, &t), &monos);
                for j in 0..9 {
                    let got = mv
                        .iter()
                        .zip(&coef)
                        .fold(Eb::ZERO, |acc, (m, c)| acc.add(&f.mul(m, &c[j])));
                    if got != want[j] {
                        return Err(format!(
                            "H is not in the span of the invariants (check point {t_}, coefficient t_R^{j})"
                        ));
                    }
                }
            }
            return Ok(SymmetrisedBin {
                monos,
                coef,
                checked_points: checks,
                precompute_muls: f.f.muls() - before,
            });
        }
        Err("the interpolation system was singular eight times".into())
    }

    /// `H(·; t_R)` at every monomial, in `F_{q⁴}`.
    fn at(&self, f: &Fq4, t_r: &Eb) -> Vec<Eb> {
        let mut pows = [Eb::ONE; 9];
        for k in 1..9 {
            pows[k] = f.mul(&pows[k - 1], t_r);
        }
        self.coef
            .iter()
            .map(|c| (0..9).fold(Eb::ZERO, |acc, j| acc.add(&f.mul(&c[j], &pows[j]))))
            .collect()
    }

    /// The four Weil components in `(s₂, s₃, s₄, π)`: weighted form,
    /// weights [`WEIGHTS`].
    pub fn system_weighted(&self, f: &Fq4, t_r: &Eb) -> Vec<HashMap<[u8; 4], u32>> {
        let vals = self.at(f, t_r);
        let mut out: Vec<HashMap<[u8; 4], u32>> = vec![HashMap::new(); 4];
        for (m, v) in self.monos.iter().zip(&vals) {
            for (i, comp) in out.iter_mut().enumerate() {
                if v.0[i] != 0 {
                    *comp.entry(*m).or_insert(0) ^= v.0[i];
                }
            }
        }
        for c in &mut out {
            c.retain(|_, v| *v != 0);
        }
        out
    }

    /// The unweighted form in `(s₁, s₂, s₃, s₄, π)`: `π^d` reduced modulo
    /// `π² + π + s₁` to `A_d(s₁) + B_d(s₁) π` (so every term has total
    /// degree ≤ 4), and the fifth equation `π² + π + s₁`.
    pub fn system_plain(&self, f: &Fq4, t_r: &Eb) -> Vec<HashMap<[u8; 5], u32>> {
        // π^d = Σ_k A[d][k] s₁^k + Σ_k B[d][k] s₁^k π, coefficients in F_2.
        let mut ab: Vec<(Vec<u8>, Vec<u8>)> = vec![(vec![1], vec![0])];
        for d in 1..=8usize {
            let (a, b) = &ab[d - 1];
            // π^{d} = A π + B π² = A π + B (π + s₁) = B s₁ + (A + B) π
            let mut na = vec![0u8; b.len() + 1];
            for (k, &c) in b.iter().enumerate() {
                na[k + 1] ^= c;
            }
            let len = a.len().max(b.len());
            let nb: Vec<u8> = (0..len)
                .map(|k| a.get(k).copied().unwrap_or(0) ^ b.get(k).copied().unwrap_or(0))
                .collect();
            ab.push((na, nb));
        }
        let vals = self.at(f, t_r);
        let mut out: Vec<HashMap<[u8; 5], u32>> = vec![HashMap::new(); 4];
        for (m, v) in self.monos.iter().zip(&vals) {
            let (a, b) = &ab[m[3] as usize];
            for (part, pe) in [(a, 0u8), (b, 1u8)] {
                for (k, &c) in part.iter().enumerate() {
                    if c == 0 {
                        continue;
                    }
                    let key = [k as u8, m[0], m[1], m[2], pe];
                    for (i, comp) in out.iter_mut().enumerate() {
                        if v.0[i] != 0 {
                            *comp.entry(key).or_insert(0) ^= v.0[i];
                        }
                    }
                }
            }
        }
        for c in &mut out {
            c.retain(|_, v| *v != 0);
        }
        let mut rel = HashMap::new();
        rel.insert([0, 0, 0, 0, 2], 1);
        rel.insert([0, 0, 0, 0, 1], 1);
        rel.insert([1, 0, 0, 0, 0], 1);
        out.push(rel);
        out
    }
}

/// The weights of `(s₂, s₃, s₄, π)`.
pub const WEIGHTS: [u8; 4] = [2, 2, 2, 1];

/// The weights of `(S₂, S₃, s₄, P) = (s₂², s₃², s₄, π²)` in the
/// Frobenius-reduced form.
pub const FROBENIUS_WEIGHTS: [u8; 4] = [2, 2, 1, 1];

impl SymmetrisedBin {
    /// Whether every monomial carrying `H` has even exponents in `s₂`, `s₃`
    /// and `π` (measured on every instance from the interpolated form: true
    /// on every curve tried, not proved).  Then, since squaring is a
    /// bijection of `F_q`, `H` is a polynomial in `S₂ = s₂²`, `S₃ = s₃²`,
    /// `s₄` and `P = π²` with the same `F_q`-rational solutions: four
    /// equations of weighted degree 4 in the weights `(2, 2, 1, 1)`, Hilbert
    /// series `(1 + t²)²(1 + t + t² + t³)²`, 64 solutions over the closure
    /// against 512, weighted Macaulay degree 11.
    pub fn frobenius_reducible(&self) -> bool {
        self.monos
            .iter()
            .zip(&self.coef)
            .filter(|(_, c)| c.iter().any(|v| !v.is_zero()))
            .all(|(m, _)| m[0] % 2 == 0 && m[1] % 2 == 0 && m[3] % 2 == 0)
    }

    /// The four Weil components in `(S₂, S₃, s₄, P)` (requires
    /// [`Self::frobenius_reducible`]).
    pub fn system_frobenius(&self, f: &Fq4, t_r: &Eb) -> Vec<HashMap<[u8; 4], u32>> {
        assert!(
            self.frobenius_reducible(),
            "H has an odd power of s₂, s₃ or π"
        );
        let vals = self.at(f, t_r);
        let mut out: Vec<HashMap<[u8; 4], u32>> = vec![HashMap::new(); 4];
        for (m, v) in self.monos.iter().zip(&vals) {
            if v.is_zero() {
                continue;
            }
            let key = [m[0] / 2, m[1] / 2, m[2], m[3] / 2];
            for (i, comp) in out.iter_mut().enumerate() {
                if v.0[i] != 0 {
                    *comp.entry(key).or_insert(0) ^= v.0[i];
                }
            }
        }
        for c in &mut out {
            c.retain(|_, v| *v != 0);
        }
        out
    }
}

// ── Instances, the base, and the independent enumeration ─────────────────

/// A random ordinary curve over `F_{q⁴}` with `j = 1/b` outside `F_{q²}`
/// (no subfield structure).  Deterministic in `(m, seed)`.
pub fn generate_instance(m: u32, seed: u64) -> CurveB {
    let mut rng = StdRng::seed_from_u64(seed ^ 0xb1d4);
    let f = Fq4::new(m);
    loop {
        let b = f.random(&mut rng);
        // b ∉ F_{q²}: b^{q²} ≠ b.
        if b.is_zero() || f.pow2k(&b, 2 * m) == b {
            continue;
        }
        let a = f.random(&mut rng);
        return CurveB::new(f, a, b);
    }
}

/// The base: one point per `t ∈ F_q \ {0, 1}` that lifts, with its `t`.
pub fn t_base(curve: &CurveB, rng: &mut StdRng) -> Vec<(u32, PtB)> {
    let f = &curve.f;
    (2..f.f.q as u32)
        .filter_map(|t| {
            let x = curve.x_of_t(&f.from_fq(t));
            curve.lift_x(&x, rng).map(|p| (t, p))
        })
        .collect()
}

/// Every set of four distinct base points with `±P_i ± P_j ± P_k ± P_l =
/// R`, as sorted `t`-quadruples, by meet in the middle over pairs.
pub fn mitm_t_decompositions(curve: &CurveB, base: &[(u32, PtB)], r: &PtB) -> BTreeSet<[u32; 4]> {
    let n = base.len();
    let mut table: HashMap<Eb, Vec<(usize, usize)>> = HashMap::new();
    for i in 0..n {
        for j in (i + 1)..n {
            for q in [
                curve.add(&base[i].1, &base[j].1),
                curve.sub(&base[i].1, &base[j].1),
            ] {
                if !q.inf {
                    table.entry(q.x).or_default().push((i, j));
                }
            }
        }
    }
    let mut out = BTreeSet::new();
    for k in 0..n {
        for l in (k + 1)..n {
            for (sk, sl) in [(false, false), (false, true), (true, false), (true, true)] {
                let pk = if sk { curve.neg(&base[k].1) } else { base[k].1 };
                let pl = if sl { curve.neg(&base[l].1) } else { base[l].1 };
                let v = curve.sub(&curve.sub(r, &pk), &pl);
                if v.inf {
                    continue;
                }
                if let Some(pairs) = table.get(&v.x) {
                    for &(i, j) in pairs {
                        if i == k || i == l || j == k || j == l {
                            continue;
                        }
                        let mut ts = [base[i].0, base[j].0, base[k].0, base[l].0];
                        ts.sort_unstable();
                        out.insert(ts);
                    }
                }
            }
        }
    }
    out
}

/// The signs that make the four points sum to `r`, applied, if any do.
fn signed_sum(curve: &CurveB, pts: &[PtB; 4], r: &PtB) -> Option<[PtB; 4]> {
    for s in 0..16u32 {
        let signed: [PtB; 4] = core::array::from_fn(|i| {
            if s >> i & 1 == 1 {
                curve.neg(&pts[i])
            } else {
                pts[i]
            }
        });
        let mut acc = PtB::INF;
        for q in &signed {
            acc = curve.add(&acc, q);
        }
        if !acc.inf && acc.x == r.x && acc.y == r.y {
            return Some(signed);
        }
    }
    None
}

// ── Weighted monomials ───────────────────────────────────────────────────

fn wdeg<const N: usize>(m: &[u8; N], w: &[u8; N]) -> u32 {
    m.iter().zip(w).map(|(&x, &k)| x as u32 * k as u32).sum()
}

/// Weighted degree, then reverse lexicographic (grevlex for weights 1).
fn wcmp<const N: usize>(a: &[u8; N], b: &[u8; N], w: &[u8; N]) -> std::cmp::Ordering {
    let (da, db) = (wdeg(a, w), wdeg(b, w));
    if da != db {
        return da.cmp(&db);
    }
    for i in (0..N).rev() {
        if a[i] != b[i] {
            return b[i].cmp(&a[i]);
        }
    }
    std::cmp::Ordering::Equal
}

/// Monomials of weighted degree `≤ d`, descending.
fn monomials_desc<const N: usize>(d: u8, w: &[u8; N]) -> Vec<[u8; N]> {
    fn rec<const N: usize>(
        i: usize,
        left: u8,
        w: &[u8; N],
        cur: &mut [u8; N],
        out: &mut Vec<[u8; N]>,
    ) {
        if i == N {
            out.push(*cur);
            return;
        }
        for k in 0..=left / w[i] {
            cur[i] = k;
            rec(i + 1, left - k * w[i], w, cur, out);
        }
        cur[i] = 0;
    }
    let mut out = Vec::new();
    rec(0, d, w, &mut [0u8; N], &mut out);
    out.sort_by(|x, y| wcmp(y, x, w));
    out
}

fn lead<const N: usize>(f: &HashMap<[u8; N], u32>, w: &[u8; N]) -> Option<[u8; N]> {
    f.iter()
        .filter(|(_, &c)| c != 0)
        .map(|(m, _)| *m)
        .max_by(|a, b| wcmp(a, b, w))
}

/// Make the leading monomials distinct by eliminating equal leads.
fn triangularise<const N: usize>(
    g: &GfTab,
    comps: &[HashMap<[u8; N], u32>],
    w: &[u8; N],
    muls: &mut u64,
) -> (Vec<HashMap<[u8; N], u32>>, Vec<Option<[u8; N]>>) {
    let mut fs = comps.to_vec();
    for _ in 0..16 {
        fs.sort_by(|a, b| match (lead(a, w), lead(b, w)) {
            (Some(x), Some(y)) => wcmp(&y, &x, w),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        });
        let mut changed = false;
        for i in 1..fs.len() {
            let (Some(li), Some(lj)) = (lead(&fs[i], w), lead(&fs[i - 1], w)) else {
                continue;
            };
            if li == lj {
                let k = g.mul(fs[i][&li], g.inv(fs[i - 1][&lj]));
                *muls += 1;
                let prev = fs[i - 1].clone();
                for (&m, &c) in &prev {
                    *fs[i].entry(m).or_insert(0) ^= g.mul(k, c);
                    *muls += 1;
                }
                fs[i].retain(|_, c| *c != 0);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let leads = fs.iter().map(|f| lead(f, w)).collect();
    (fs, leads)
}

// ── Elimination ──────────────────────────────────────────────────────────

struct Echelon {
    rows: Vec<Vec<u32>>,
    pivot_row: Vec<u32>,
    zero_rows: usize,
    zero_ids: Vec<u32>,
}

/// Forward elimination by leading column (as `gaudry_quartic`'s
/// `echelon_by_lead_with`): one multiplication per nonzero entry of a pivot
/// row per row it is subtracted from, and per entry normalised.  With
/// `sparsest`, each column's pivot is the sparsest candidate row.
fn echelon(g: &GfTab, mut rows: Vec<Vec<u32>>, muls: &mut u64, sparsest: bool) -> Echelon {
    let ncols = rows.first().map_or(0, |r| r.len());
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); ncols];
    let mut zero_rows = 0usize;
    let mut zero_ids = Vec::new();
    let mut nnz: Vec<u32> = rows
        .par_iter()
        .map(|r| r.iter().filter(|&&x| x != 0).count() as u32)
        .collect();
    for (r, row) in rows.iter().enumerate() {
        match row.iter().position(|&x| x != 0) {
            Some(c) => buckets[c].push(r as u32),
            None => {
                zero_rows += 1;
                zero_ids.push(r as u32);
            }
        }
    }
    let mut pivot_row = vec![u32::MAX; ncols];
    for c in 0..ncols {
        let mut b = std::mem::take(&mut buckets[c]);
        if b.is_empty() {
            continue;
        }
        b.sort_unstable();
        if sparsest && b.len() > 1 {
            let best = (0..b.len())
                .min_by_key(|&i| (nnz[b[i] as usize], i))
                .unwrap_or(0);
            b.swap(0, best);
        }
        let pr = b[0] as usize;
        let mut piv = std::mem::take(&mut rows[pr]);
        let inv = g.inv(piv[c]);
        let mut nz: Vec<u32> = Vec::new();
        for j in c..ncols {
            if piv[j] != 0 {
                piv[j] = g.mul(piv[j], inv);
                nz.push(j as u32);
            }
        }
        *muls += nz.len() as u64;
        let dense = nz.len() * 8 > ncols - c;
        let mut others: Vec<(u32, Vec<u32>, Option<usize>, u32)> = b[1..]
            .iter()
            .map(|&r| (r, std::mem::take(&mut rows[r as usize]), None, 0))
            .collect();
        *muls += (others.len() * nz.len()) as u64;
        others.par_iter_mut().for_each(|(_, row, ld, cnt)| {
            let f = row[c];
            if dense {
                g.axpy(&mut row[c..], &piv[c..], f);
            } else {
                for &j in &nz {
                    let j = j as usize;
                    row[j] ^= g.mul(f, piv[j]);
                }
            }
            *ld = row[c + 1..].iter().position(|&x| x != 0).map(|k| k + c + 1);
            if let Some(l) = *ld {
                *cnt = row[l..].iter().filter(|&&x| x != 0).count() as u32;
            }
        });
        for (r, row, ld, cnt) in others {
            nnz[r as usize] = cnt;
            rows[r as usize] = row;
            match ld {
                Some(l) => buckets[l].push(r),
                None => {
                    zero_rows += 1;
                    zero_ids.push(r);
                }
            }
        }
        rows[pr] = piv;
        pivot_row[c] = pr as u32;
    }
    zero_ids.sort_unstable();
    Echelon {
        rows,
        pivot_row,
        zero_rows,
        zero_ids,
    }
}

/// Reduce held rows into the echelon until `need` new pivots are found.
fn top_up(
    g: &GfTab,
    ech: &mut Echelon,
    held: Vec<Vec<u32>>,
    mut need: usize,
    muls: &mut u64,
) -> u64 {
    let ncols = ech.pivot_row.len();
    let mut used = 0;
    for mut v in held {
        if need == 0 {
            break;
        }
        used += 1;
        let mut ld = None;
        for c in 0..ncols {
            if v[c] == 0 {
                continue;
            }
            let pr = ech.pivot_row[c];
            if pr == u32::MAX {
                ld = Some(c);
                break;
            }
            let f = v[c];
            let piv = &ech.rows[pr as usize];
            *muls += piv[c..].iter().filter(|&&x| x != 0).count() as u64;
            g.axpy(&mut v[c..], &piv[c..], f);
        }
        let Some(c) = ld else {
            ech.zero_rows += 1;
            continue;
        };
        let inv = g.inv(v[c]);
        for x in &mut v[c..] {
            if *x != 0 {
                *x = g.mul(*x, inv);
                *muls += 1;
            }
        }
        ech.pivot_row[c] = ech.rows.len() as u32;
        ech.rows.push(v);
        need -= 1;
    }
    used
}

// ── Small linear algebra and polynomials over F_q (counted) ──────────────

/// Reduced row echelon form; returns pivot columns.
fn rref(g: &GfTab, m: &mut [Vec<u32>], muls: &mut u64) -> Vec<usize> {
    let rows = m.len();
    let cols = if rows > 0 { m[0].len() } else { 0 };
    let mut pivots = Vec::new();
    let mut r = 0;
    for c in 0..cols {
        if r >= rows {
            break;
        }
        let Some(pr) = (r..rows).find(|&i| m[i][c] != 0) else {
            continue;
        };
        m.swap(r, pr);
        let inv = g.inv(m[r][c]);
        for v in &mut m[r][c..] {
            *v = g.mul(*v, inv);
        }
        *muls += (cols - c) as u64;
        let piv = m[r].clone();
        for i in 0..rows {
            if i != r && m[i][c] != 0 {
                let f = m[i][c];
                g.axpy(&mut m[i][c..], &piv[c..], f);
                *muls += (cols - c) as u64;
            }
        }
        pivots.push(c);
        r += 1;
    }
    pivots
}

/// A basis of the kernel of `m − λI` (`m` square).
fn kernel_shifted(g: &GfTab, m: &[Vec<u32>], lambda: u32, muls: &mut u64) -> Vec<Vec<u32>> {
    let n = m.len();
    let mut a = m.to_vec();
    for (i, row) in a.iter_mut().enumerate() {
        row[i] ^= lambda;
    }
    let pivots = rref(g, &mut a, muls);
    let mut is_piv = vec![false; n];
    for &c in &pivots {
        is_piv[c] = true;
    }
    (0..n)
        .filter(|&c| !is_piv[c])
        .map(|fc| {
            let mut v = vec![0u32; n];
            v[fc] = 1;
            for (r, &pc) in pivots.iter().enumerate() {
                v[pc] = a[r][fc];
            }
            v
        })
        .collect()
}

fn det(g: &GfTab, mut a: Vec<Vec<u32>>, muls: &mut u64) -> u32 {
    let k = a.len();
    let mut d = 1u32;
    for c in 0..k {
        let Some(r) = (c..k).find(|&r| a[r][c] != 0) else {
            return 0;
        };
        a.swap(r, c);
        d = g.mul(d, a[c][c]);
        let inv = g.inv(a[c][c]);
        *muls += 1;
        for r in (c + 1)..k {
            if a[r][c] == 0 {
                continue;
            }
            let f = g.mul(a[r][c], inv);
            let piv = a[c].clone();
            g.axpy(&mut a[r][c + 1..], &piv[c + 1..], f);
            *muls += (k - c) as u64;
        }
    }
    d
}

fn ptrim(a: &mut Vec<u32>) {
    while a.last() == Some(&0) {
        a.pop();
    }
}

/// `a mod h` (`h` monic).
fn pmod(g: &GfTab, a: &[u32], h: &[u32], muls: &mut u64) -> Vec<u32> {
    let mut r = a.to_vec();
    ptrim(&mut r);
    let dh = h.len() - 1;
    while r.len() > dh {
        let c = *r.last().unwrap();
        let shift = r.len() - 1 - dh;
        for (i, &hi) in h.iter().enumerate() {
            if hi != 0 {
                r[shift + i] ^= g.mul(c, hi);
                *muls += 1;
            }
        }
        ptrim(&mut r);
    }
    r
}

fn pmul(g: &GfTab, a: &[u32], b: &[u32], muls: &mut u64) -> Vec<u32> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let mut out = vec![0u32; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        for (j, &y) in b.iter().enumerate() {
            if y != 0 {
                out[i + j] ^= g.mul(x, y);
                *muls += 1;
            }
        }
    }
    out
}

fn pmonic(g: &GfTab, mut a: Vec<u32>, muls: &mut u64) -> Vec<u32> {
    ptrim(&mut a);
    if let Some(&l) = a.last() {
        if l != 1 {
            let li = g.inv(l);
            for c in a.iter_mut() {
                *c = g.mul(*c, li);
            }
            *muls += a.len() as u64;
        }
    }
    a
}

fn pgcd(g: &GfTab, a: &[u32], b: &[u32], muls: &mut u64) -> Vec<u32> {
    let mut a = pmonic(g, a.to_vec(), muls);
    let mut b = pmonic(g, b.to_vec(), muls);
    while !b.is_empty() {
        let r = pmod(g, &a, &b, muls);
        a = b;
        b = pmonic(g, r, muls);
    }
    a
}

fn pdiv(g: &GfTab, a: &[u32], h: &[u32], muls: &mut u64) -> Vec<u32> {
    let mut r = a.to_vec();
    ptrim(&mut r);
    let dh = h.len() - 1;
    let mut qt = vec![0u32; r.len().saturating_sub(dh)];
    while r.len() > dh {
        let c = *r.last().unwrap();
        let shift = r.len() - 1 - dh;
        qt[shift] = c;
        for (i, &hi) in h.iter().enumerate() {
            if hi != 0 {
                r[shift + i] ^= g.mul(c, hi);
                *muls += 1;
            }
        }
        ptrim(&mut r);
    }
    qt
}

/// Distinct roots in `F_q` of `poly` (ascending coefficients): `gcd` with
/// `X^q − X` (by `m` squarings modulo the polynomial), then splitting by
/// `Tr(δX)`; every multiplication counted.
pub fn roots(f: &Gf2q, poly: &[u32], muls: &mut u64) -> Vec<u32> {
    let g = &f.tab;
    let p = pmonic(g, poly.to_vec(), muls);
    if p.len() <= 1 {
        return Vec::new();
    }
    let mut r = pmod(g, &[0, 1], &p, muls);
    for _ in 0..f.m {
        let sq = pmul(g, &r, &r, muls);
        r = pmod(g, &sq, &p, muls);
    }
    if r.len() < 2 {
        r.resize(2, 0);
    }
    r[1] ^= 1;
    ptrim(&mut r);
    let h = if r.is_empty() {
        p.clone()
    } else {
        pgcd(g, &p, &r, muls)
    };
    let mut out = Vec::new();
    split_roots(f, h, &mut out, 1, muls);
    out.sort_unstable();
    out
}

fn split_roots(f: &Gf2q, h: Vec<u32>, out: &mut Vec<u32>, mut seed: u32, muls: &mut u64) {
    let g = &f.tab;
    match h.len() {
        0 | 1 => {}
        2 => out.push(h[0]),
        _ => loop {
            let delta = g.exp[(seed % (f.q - 1)) as usize];
            seed += 1;
            let mut x = pmod(g, &[0, delta], &h, muls);
            let mut acc = x.clone();
            for _ in 1..f.m {
                let sq = pmul(g, &x, &x, muls);
                x = pmod(g, &sq, &h, muls);
                let len = acc.len().max(x.len());
                acc.resize(len, 0);
                for (i, &c) in x.iter().enumerate() {
                    acc[i] ^= c;
                }
                ptrim(&mut acc);
            }
            if acc.is_empty() {
                continue;
            }
            let d = pgcd(g, &h, &acc, muls);
            if d.len() > 1 && d.len() < h.len() {
                let e = pdiv(g, &h, &d, muls);
                split_roots(f, d, out, seed, muls);
                split_roots(f, e, out, seed + 7, muls);
                return;
            }
        },
    }
}

// ── The solve ────────────────────────────────────────────────────────────

/// What one solve cost, by phase, in `F_q` multiplications.
#[derive(Clone, Debug, Default, Serialize)]
pub struct BinSolve {
    pub macaulay_degree: u8,
    pub rows: usize,
    pub cols: usize,
    pub pivots: usize,
    pub zero_rows: usize,
    pub dim: usize,
    pub chains: usize,
    pub weil_muls: u64,
    pub triangularise_muls: u64,
    pub echelon_muls: u64,
    pub nf_muls: u64,
    pub charpoly_muls: u64,
    pub roots_muls: u64,
    pub eigenvector_muls: u64,
    pub verify_muls: u64,
    pub split_muls: u64,
    pub total_muls: u64,
    pub rational_eigenvalues: usize,
    pub degenerate_eigenspaces: usize,
    pub e_solutions: usize,
    pub topped_up: u64,
    pub replay: String,
    pub outcome: String,
    pub total_ms: f64,
}

impl BinSolve {
    fn close(&mut self) {
        self.total_muls = self.weil_muls
            + self.triangularise_muls
            + self.echelon_muls
            + self.nf_muls
            + self.charpoly_muls
            + self.roots_muls
            + self.eigenvector_muls
            + self.verify_muls
            + self.split_muls;
    }
}

/// The recorded shape of a solved system (F4Remake's idea).
#[derive(Clone, Debug, Default)]
pub struct ShapeB {
    leads: Vec<Option<Vec<u8>>>,
    rows_built: usize,
    zero_ids: Vec<u32>,
    pivots: usize,
}

/// A shape carried from solve to solve, with its outcomes.
#[derive(Clone, Debug, Default)]
pub struct BinTrace {
    pub shape: Option<ShapeB>,
    pub replays: u64,
    pub fallbacks: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct BinOptions {
    /// `(s₂, s₃, s₄, π)`, weights `(2, 2, 2, 1)`, degree 26; otherwise the
    /// unweighted `(s₁, s₂, s₃, s₄, π)` with `π² + π + s₁`, degree 14.
    pub weighted: bool,
    /// The Frobenius-reduced form `(S₂, S₃, s₄, P)`, weights
    /// `(2, 2, 1, 1)`, degree 11 (implies the weighted invariants).
    #[serde(default)]
    pub frobenius: bool,
    pub sparsest: bool,
    pub replay: bool,
}

struct Chains {
    pos: Vec<(u32, u32)>,
    len: Vec<u32>,
    nf: Vec<Vec<(u32, u32, u32)>>,
}

impl Chains {
    fn powers(g: &GfTab, lam: u32, top: usize, muls: &mut u64) -> Vec<u32> {
        let mut pw = vec![1u32; top + 1];
        for t in 1..=top {
            pw[t] = g.mul(pw[t - 1], lam);
        }
        *muls += top as u64;
        pw
    }
    fn top(&self) -> usize {
        *self.len.iter().max().unwrap_or(&0) as usize
    }
    fn matrix(&self, g: &GfTab, lam: u32, muls: &mut u64) -> Vec<Vec<u32>> {
        let pw = Self::powers(g, lam, self.top(), muls);
        let k = self.len.len();
        let mut a = vec![vec![0u32; k]; k];
        for (j, row) in self.nf.iter().enumerate() {
            for &(i, t, c) in row {
                a[j][i as usize] ^= g.mul(c, pw[t as usize]);
            }
            *muls += row.len() as u64;
            a[j][j] ^= pw[self.len[j] as usize];
        }
        a
    }
    fn eigenvectors(&self, g: &GfTab, lam: u32, muls: &mut u64) -> Vec<Vec<u32>> {
        let a = self.matrix(g, lam, muls);
        let ker = kernel_shifted(g, &a, 0, muls);
        let pw = Self::powers(g, lam, self.top(), muls);
        ker.iter()
            .map(|u| {
                *muls += self.pos.len() as u64;
                self.pos
                    .iter()
                    .map(|&(j, t)| g.mul(u[j as usize], pw[t as usize]))
                    .collect()
            })
            .collect()
    }
}

/// The characteristic polynomial of a small square matrix, by its
/// determinant at `k + 1` points interpolated (needs `q > k`).
fn small_charpoly(f: &Gf2q, a: &[Vec<u32>], muls: &mut u64) -> Vec<u32> {
    let g = &f.tab;
    let k = a.len();
    let xs: Vec<u32> = (0..=k as u32).collect();
    let vals: Vec<u32> = xs
        .iter()
        .map(|&x| {
            let mut m = a.to_vec();
            for (i, row) in m.iter_mut().enumerate() {
                row[i] ^= x;
            }
            det(g, m, muls)
        })
        .collect();
    newton_interpolate(g, &xs, &vals, muls)
}

/// The polynomial through `(xs[i], vals[i])`, ascending, by Newton's
/// divided differences (one multiplication per division).
fn newton_interpolate(g: &GfTab, xs: &[u32], vals: &[u32], muls: &mut u64) -> Vec<u32> {
    let n = xs.len();
    let mut dd = vals.to_vec();
    for lvl in 1..n {
        for i in (lvl..n).rev() {
            let num = dd[i] ^ dd[i - 1];
            dd[i] = g.mul(num, g.inv(xs[i] ^ xs[i - lvl]));
        }
        *muls += (n - lvl) as u64;
    }
    let mut poly = vec![0u32; n];
    for i in (0..n).rev() {
        let mut nxt = vec![0u32; n];
        for d in 0..n - 1 {
            if poly[d] != 0 {
                nxt[d + 1] ^= poly[d];
                nxt[d] ^= g.mul(poly[d], xs[i]);
            }
        }
        *muls += (n - 1 - i) as u64;
        nxt[0] ^= dd[i];
        poly = nxt;
    }
    poly
}

/// Split an eigenspace by another unknown `v`, known only on the standard
/// monomials `b` with `v·b` standard (`known`: `(b, v·b)` indices).
fn split_partial(
    f: &Gf2q,
    w: &[Vec<u32>],
    known: &[(usize, usize)],
    muls: &mut u64,
) -> Vec<Vec<u32>> {
    let g = &f.tab;
    let k = w.len();
    let mut a = vec![vec![0u32; k]; k];
    for i in 0..k {
        let mut sys: Vec<Vec<u32>> = known
            .iter()
            .map(|&(l, t)| {
                let mut row: Vec<u32> = (0..k).map(|j| w[j][l]).collect();
                row.push(w[i][t]);
                row
            })
            .collect();
        let pivots = rref(g, &mut sys, muls);
        if pivots.len() != k || pivots.iter().any(|&c| c >= k) {
            return w.to_vec();
        }
        for (r, &pc) in pivots.iter().enumerate() {
            a[i][pc] = sys[r][k];
        }
    }
    let at: Vec<Vec<u32>> = (0..k).map(|j| (0..k).map(|i| a[i][j]).collect()).collect();
    let cp = small_charpoly(f, &a, muls);
    let mus = roots(f, &cp, muls);
    let dim = w[0].len();
    let mut out = Vec::new();
    for mu in mus {
        for v in kernel_shifted(g, &at, mu, muls) {
            let mut u = vec![0u32; dim];
            for j in 0..k {
                if v[j] == 0 {
                    continue;
                }
                for r in 0..dim {
                    if w[j][r] != 0 {
                        u[r] ^= g.mul(v[j], w[j][r]);
                        *muls += 1;
                    }
                }
            }
            out.push(u);
        }
    }
    out
}

fn eval<const N: usize>(
    g: &GfTab,
    comp: &HashMap<[u8; N], u32>,
    e: &[u32; N],
    muls: &mut u64,
) -> u32 {
    let mut acc = 0u32;
    for (m, &c) in comp {
        let mut t = c;
        for k in 0..N {
            for _ in 0..m[k] {
                t = g.mul(t, e[k]);
                *muls += 1;
            }
        }
        acc ^= t;
    }
    acc
}

/// All `F_q`-rational solutions of `comps` (weights `w`), through a
/// weighted Macaulay matrix at `macaulay`, the eigen step along the
/// chains of unknown `mvar` (weight 1).  `Err(())` only from a replay
/// whose leads or row count differ.
#[allow(clippy::too_many_arguments)]
fn solve_once<const N: usize>(
    f: &Gf2q,
    comps: &[HashMap<[u8; N], u32>],
    w: &[u8; N],
    mvar: usize,
    macaulay: u8,
    sparsest: bool,
    st: &mut BinSolve,
    replay: Option<&ShapeB>,
    record: Option<&mut ShapeB>,
) -> Result<Option<Vec<[u32; N]>>, ()> {
    let g = &f.tab;
    let q = f.q as u64;
    let t0 = std::time::Instant::now();
    st.macaulay_degree = macaulay;
    let cols = monomials_desc(macaulay, w);
    let col_index: HashMap<[u8; N], usize> =
        cols.iter().enumerate().map(|(i, m)| (*m, i)).collect();
    let mut tri = 0;
    let (comps, leads) = triangularise(g, comps, w, &mut tri);
    st.triangularise_muls += tri;
    let leads_v: Vec<Option<Vec<u8>>> = leads.iter().map(|l| l.map(|m| m.to_vec())).collect();
    if let Some(t) = replay {
        if t.leads != leads_v {
            return Err(());
        }
    }
    let divides = |lm: &[u8; N], m: &[u8; N]| (0..N).all(|k| lm[k] <= m[k]);
    let mut rows: Vec<Vec<u32>> = Vec::new();
    let mut held: Vec<Vec<u32>> = Vec::new();
    let (mut built, mut skip_at) = (0usize, 0usize);
    for (i, comp) in comps.iter().enumerate() {
        let dg = comp.keys().map(|m| wdeg(m, w)).max().unwrap_or(0) as u8;
        if dg > macaulay {
            st.outcome = "an equation above the Macaulay degree".into();
            return Ok(None);
        }
        for sh in monomials_desc::<N>(macaulay - dg, w) {
            if leads
                .iter()
                .take(i)
                .any(|lm| lm.is_some_and(|lm| divides(&lm, &sh)))
            {
                continue;
            }
            let index = built;
            built += 1;
            let mut row = vec![0u32; cols.len()];
            for (&e, &c) in comp {
                let m: [u8; N] = core::array::from_fn(|k| e[k] + sh[k]);
                row[col_index[&m]] ^= c;
            }
            if let Some(t) = replay {
                if skip_at < t.zero_ids.len() && t.zero_ids[skip_at] as usize == index {
                    skip_at += 1;
                    held.push(row);
                    continue;
                }
            }
            rows.push(row);
        }
    }
    if let Some(t) = replay {
        if t.rows_built != built {
            return Err(());
        }
    }
    st.rows = rows.len();
    st.cols = cols.len();
    let mut em = 0;
    let mut ech = echelon(g, rows, &mut em, sparsest);
    if let Some(t) = replay {
        let have = ech.pivot_row.iter().filter(|&&r| r != u32::MAX).count();
        if have < t.pivots {
            st.topped_up += top_up(g, &mut ech, held, t.pivots - have, &mut em);
        }
    }
    st.echelon_muls += em;
    st.zero_rows = ech.zero_rows;
    st.pivots = ech.pivot_row.iter().filter(|&&r| r != u32::MAX).count();
    let is_std: Vec<bool> = (0..cols.len())
        .map(|c| ech.pivot_row[c] == u32::MAX && wdeg(&cols[c], w) < macaulay as u32)
        .collect();
    let standard: Vec<usize> = (0..cols.len()).filter(|&c| is_std[c]).collect();
    let dim = standard.len();
    st.dim = dim;
    if let Some(out) = record {
        *out = ShapeB {
            leads: leads_v,
            rows_built: built,
            zero_ids: ech.zero_ids.clone(),
            pivots: st.pivots,
        };
    }
    let mut std_index = vec![u32::MAX; cols.len()];
    for (i, &c) in standard.iter().enumerate() {
        std_index[c] = i as u32;
    }
    // Columns of 1 and of each unknown.
    let mut idx = Vec::with_capacity(N + 1);
    for v in 0..=N {
        let m: [u8; N] = core::array::from_fn(|k| u8::from(v > 0 && k == v - 1));
        match col_index.get(&m) {
            Some(&c) if is_std[c] => idx.push(std_index[c] as usize),
            _ => {
                st.outcome = "1 or a variable is not standard".into();
                return Ok(None);
            }
        }
    }
    // Chains of the multiplication unknown.
    let mut next: Vec<Result<usize, usize>> = Vec::with_capacity(dim);
    for &bc in &standard {
        let mut m = cols[bc];
        m[mvar] += 1;
        let Some(&t) = col_index.get(&m) else {
            st.outcome = "border unreachable at this degree".into();
            return Ok(None);
        };
        next.push(if is_std[t] {
            Ok(std_index[t] as usize)
        } else {
            Err(t)
        });
    }
    let mut pos = vec![(u32::MAX, u32::MAX); dim];
    let (mut len, mut ends) = (Vec::new(), Vec::new());
    for s in 0..dim {
        if cols[standard[s]][mvar] != 0 {
            continue;
        }
        let j = len.len() as u32;
        let (mut cur, mut t) = (s, 0u32);
        loop {
            if pos[cur].0 != u32::MAX {
                st.outcome = "chain structure: a monomial on two chains".into();
                return Ok(None);
            }
            pos[cur] = (j, t);
            t += 1;
            match next[cur] {
                Ok(n) => cur = n,
                Err(b) => {
                    ends.push(b);
                    break;
                }
            }
        }
        len.push(t);
    }
    if pos.iter().any(|&(j, _)| j == u32::MAX) {
        st.outcome = "chain structure: standard monomials are not an order ideal".into();
        return Ok(None);
    }
    st.chains = len.len();
    let ncols = cols.len();
    let nnz: Vec<u64> = (0..ncols)
        .map(|c| match ech.pivot_row[c] {
            u32::MAX => 0,
            r => ech.rows[r as usize][c..]
                .iter()
                .filter(|&&v| v != 0)
                .count() as u64,
        })
        .collect();
    let reduced: Vec<Result<(Vec<u32>, u64), ()>> = ends
        .par_iter()
        .map(|&b| {
            let mut v = vec![0u32; ncols];
            v[b] = 1;
            let mut m = 0u64;
            for c in b..ncols {
                if v[c] == 0 || is_std[c] {
                    continue;
                }
                let pr = ech.pivot_row[c];
                if pr == u32::MAX {
                    return Err(());
                }
                let fct = v[c];
                g.axpy(&mut v[c..], &ech.rows[pr as usize][c..], fct);
                m += nnz[c];
            }
            Ok((v, m))
        })
        .collect();
    let mut nf = Vec::with_capacity(ends.len());
    for r in reduced {
        let Ok((v, m)) = r else {
            st.outcome = "border unreachable at this degree".into();
            return Ok(None);
        };
        st.nf_muls += m;
        nf.push(
            standard
                .iter()
                .enumerate()
                .filter(|&(_, &c)| v[c] != 0)
                .map(|(s, &c)| (pos[s].0, pos[s].1, v[c]))
                .collect(),
        );
    }
    let ch = Chains { pos, len, nf };
    // Rational eigenvalues: zeros of det A on F_q, or its roots.
    let npts = if q <= dim as u64 + 1 {
        q
    } else {
        dim as u64 + 1
    };
    let vals: Vec<(u32, u64)> = (0..npts as u32)
        .into_par_iter()
        .map(|x| {
            let mut m = 0;
            (det(g, ch.matrix(g, x, &mut m), &mut m), m)
        })
        .collect();
    st.charpoly_muls += vals.iter().map(|v| v.1).sum::<u64>();
    let lambdas: Vec<u32> = if npts == q {
        (0..q as u32).filter(|&x| vals[x as usize].0 == 0).collect()
    } else {
        let xs: Vec<u32> = (0..npts as u32).collect();
        let ys: Vec<u32> = vals.iter().map(|v| v.0).collect();
        let mut cm = 0;
        let poly = newton_interpolate(g, &xs, &ys, &mut cm);
        st.charpoly_muls += cm;
        if poly.len() != dim + 1 || poly[dim] != 1 {
            st.outcome = "det A is not monic of degree dim".into();
            return Ok(None);
        }
        let mut rm = 0;
        let r = roots(f, &poly, &mut rm);
        st.roots_muls += rm;
        r
    };
    st.rational_eigenvalues = lambdas.len();
    let known: Vec<Vec<(usize, usize)>> = (0..N)
        .filter(|&v| v != mvar)
        .map(|v| {
            (0..dim)
                .filter_map(|s| {
                    let mut m = cols[standard[s]];
                    m[v] += 1;
                    let &t = col_index.get(&m)?;
                    is_std[t].then(|| (s, std_index[t] as usize))
                })
                .collect()
        })
        .collect();
    let mut out = Vec::new();
    for lam in lambdas {
        let mut ev = 0;
        let mut ker = ch.eigenvectors(g, lam, &mut ev);
        if ker.len() > 1 {
            st.degenerate_eigenspaces += 1;
            for kn in &known {
                if ker.len() <= 1 {
                    break;
                }
                ker = split_partial(f, &ker, kn, &mut ev);
            }
        }
        st.eigenvector_muls += ev;
        for v in &ker {
            if v[idx[0]] == 0 {
                continue;
            }
            let inv = g.inv(v[idx[0]]);
            let e: [u32; N] = core::array::from_fn(|k| g.mul(v[idx[k + 1]], inv));
            if e[mvar] != lam {
                continue;
            }
            let mut vm = 0;
            let ok = comps.iter().all(|c| eval(g, c, &e, &mut vm) == 0);
            st.verify_muls += vm;
            if ok {
                out.push(e);
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    st.e_solutions = out.len();
    st.outcome = "solved".into();
    st.total_ms += t0.elapsed().as_secs_f64() * 1e3;
    Ok(Some(out))
}

/// [`solve_once`] with a shape trace: replay when one is recorded (a
/// mismatch of leads or rows runs the full solve too, both charged),
/// record on the first solved system.
#[allow(clippy::too_many_arguments)]
pub fn solve_bin<const N: usize>(
    f: &Gf2q,
    comps: &[HashMap<[u8; N], u32>],
    w: &[u8; N],
    mvar: usize,
    macaulay: u8,
    sparsest: bool,
    st: &mut BinSolve,
    trace: Option<&mut BinTrace>,
) -> Option<Vec<[u32; N]>> {
    let Some(tr) = trace else {
        st.replay = "none".into();
        return solve_once(f, comps, w, mvar, macaulay, sparsest, st, None, None)
            .ok()
            .flatten();
    };
    if let Some(shape) = &tr.shape {
        if let Ok(s) = solve_once(f, comps, w, mvar, macaulay, sparsest, st, Some(shape), None) {
            tr.replays += 1;
            st.replay = "replayed".into();
            return s;
        }
        tr.fallbacks += 1;
        st.replay = "fell back".into();
        return solve_once(f, comps, w, mvar, macaulay, sparsest, st, None, None)
            .ok()
            .flatten();
    }
    let mut shape = ShapeB::default();
    let s = solve_once(
        f,
        comps,
        w,
        mvar,
        macaulay,
        sparsest,
        st,
        None,
        Some(&mut shape),
    )
    .ok()
    .flatten();
    st.replay = "recorded".into();
    if s.is_some() {
        tr.shape = Some(shape);
    }
    s
}

// ── One decomposition ────────────────────────────────────────────────────

#[derive(Clone, Debug, Default, Serialize)]
pub struct BinD4Solve {
    pub solve: BinSolve,
    pub rational_solutions: usize,
    /// The decompositions as sorted `t`-quadruples.
    pub decompositions: Vec<[u32; 4]>,
    #[serde(skip)]
    pub witness: Option<[PtB; 4]>,
}

/// Decompose `r` as `±P₁ ± P₂ ± P₃ ± P₄` with `t(P_i) ∈ F_q`: the Weil
/// restriction of `H(·; t(r))`, its solve, and for each rational solution
/// the split `w_i` (roots of `X⁴ + s₁X³ + s₂X² + s₃X + s₄`), `t_i² + t_i =
/// w_i` with `Σ t_i = π`, and the check that the points sum to `r`.
/// `Err` carries what an unsolved attempt cost.
#[allow(clippy::result_large_err)]
pub fn solve_d4_bin(
    curve: &CurveB,
    pre: &SymmetrisedBin,
    r: &PtB,
    opts: BinOptions,
    macaulay: u8,
    rng: &mut StdRng,
    trace: Option<&mut BinTrace>,
) -> Result<BinD4Solve, BinSolve> {
    let f = &curve.f;
    let g = &f.f.tab;
    let mut out = BinD4Solve::default();
    let before = f.f.muls();
    let t_r = curve.t_of_x(&r.x);
    let sols: Option<Vec<[u32; 5]>> = if opts.frobenius {
        let comps = pre.system_frobenius(f, &t_r);
        out.solve.weil_muls = f.f.muls() - before;
        let fq = &f.f;
        solve_bin::<4>(
            fq,
            &comps,
            &FROBENIUS_WEIGHTS,
            3,
            macaulay,
            opts.sparsest,
            &mut out.solve,
            trace,
        )
        .map(|v| {
            v.into_iter()
                .map(|s| {
                    // s₂ = √S₂, s₃ = √S₃, π = √P, s₁ = P + π.
                    let before = fq.muls();
                    let (s2, s3, pi) = (fq.sqrt(s[0]), fq.sqrt(s[1]), fq.sqrt(s[3]));
                    out.solve.split_muls += fq.muls() - before;
                    [s[3] ^ pi, s2, s3, s[2], pi]
                })
                .collect()
        })
    } else if opts.weighted {
        let comps = pre.system_weighted(f, &t_r);
        out.solve.weil_muls = f.f.muls() - before;
        solve_bin::<4>(
            &f.f,
            &comps,
            &WEIGHTS,
            3,
            macaulay,
            opts.sparsest,
            &mut out.solve,
            trace,
        )
        .map(|v| {
            v.into_iter()
                .map(|s| {
                    out.solve.split_muls += 1;
                    [g.mul(s[3], s[3]) ^ s[3], s[0], s[1], s[2], s[3]]
                })
                .collect()
        })
    } else {
        let comps = pre.system_plain(f, &t_r);
        out.solve.weil_muls = f.f.muls() - before;
        solve_bin::<5>(
            &f.f,
            &comps,
            &[1; 5],
            4,
            macaulay,
            opts.sparsest,
            &mut out.solve,
            trace,
        )
    };
    let Some(sols) = sols else {
        out.solve.close();
        return Err(out.solve);
    };
    out.rational_solutions = sols.len();
    let mut found = BTreeSet::new();
    for s in sols {
        // s = (s₁, s₂, s₃, s₄, π).
        let mut sm = 0;
        let quartic = [s[3], s[2], s[1], s[0], 1];
        let rts = roots(&f.f, &quartic, &mut sm);
        out.solve.split_muls += sm;
        // The multisets of roots with these symmetric functions.
        let mut multisets = Vec::new();
        for (i1, &a) in rts.iter().enumerate() {
            for (i2, &b) in rts.iter().enumerate().skip(i1) {
                for (i3, &c) in rts.iter().enumerate().skip(i2) {
                    for &d in rts.iter().skip(i3) {
                        let x = [a, b, c, d];
                        let mut e = [1u32, 0, 0, 0, 0];
                        for &v in &x {
                            for k in (1..5).rev() {
                                e[k] ^= g.mul(e[k - 1], v);
                            }
                        }
                        out.solve.split_muls += 16;
                        if e[1] == s[0] && e[2] == s[1] && e[3] == s[2] && e[4] == s[3] {
                            multisets.push(x);
                        }
                    }
                }
            }
        }
        for ws in multisets {
            let base_t: Option<Vec<u32>> = ws.iter().map(|&w| f.f.artin_schreier(w)).collect();
            let Some(base_t) = base_t else {
                continue;
            };
            for flips in 0..16u32 {
                let ts: [u32; 4] = core::array::from_fn(|i| base_t[i] ^ ((flips >> i & 1) as u32));
                if ts.iter().fold(0u32, |a, &v| a ^ v) != s[4] {
                    continue;
                }
                if ts.iter().any(|&v| v < 2) {
                    continue;
                }
                let mut sorted = ts;
                sorted.sort_unstable();
                if sorted.windows(2).any(|w| w[0] == w[1]) {
                    continue;
                }
                // The lifts' field work is the split's (the sign checks
                // below are charged as additions).
                let lift_before = f.f.muls();
                let pts: Option<Vec<PtB>> = ts
                    .iter()
                    .map(|&v| curve.lift_x(&curve.x_of_t(&f.from_fq(v)), rng))
                    .collect();
                out.solve.split_muls += f.f.muls() - lift_before;
                let Some(pts) = pts else {
                    continue;
                };
                let pts = [pts[0], pts[1], pts[2], pts[3]];
                if let Some(signed) = signed_sum(curve, &pts, r) {
                    found.insert(sorted);
                    if out.witness.is_none() {
                        out.witness = Some(signed);
                    }
                }
            }
        }
    }
    out.decompositions = found.into_iter().collect();
    out.solve.close();
    Ok(out)
}

// ── Curves with a prime-order subgroup, for ecbench ──────────────────────

/// `#E(F_{q⁴})`: the one multiple in the Hasse interval
/// `[q⁴ + 1 − 2q², q⁴ + 1 + 2q²]` that kills a few random points, each
/// point's killing multiples found by baby steps and giant steps
/// (`≈ 2q` of each over the interval's `4q² + 1` values).
pub fn group_order_bin(c: &CurveB, rng: &mut StdRng) -> Option<u128> {
    let q = c.f.f.q as u128;
    let q4 = q.pow(4);
    let (lo, hi) = (q4 + 1 - 2 * q * q, q4 + 1 + 2 * q * q);
    let width = hi - lo + 1;
    let b = 2 * q + 1;
    assert!((b - 1) * (b - 1) < width && width <= b * b);
    let key = |p: &PtB| -> u128 {
        if p.inf {
            u128::MAX
        } else {
            c.f.pack(&p.x) * 2 + u128::from(c.f.pack(&p.y) > c.f.pack(&p.x.add(&p.y)))
        }
    };
    let mut cands: Option<Vec<u128>> = None;
    for _ in 0..24 {
        let pt = loop {
            let x = c.f.random(rng);
            if let Some(p) = c.lift_x(&x, rng) {
                break p;
            }
        };
        // Baby steps: j·P for 0 ≤ j < b.
        let mut baby: HashMap<u128, Vec<u128>> = HashMap::new();
        let mut acc = PtB::INF;
        for j in 0..b {
            baby.entry(key(&acc)).or_default().push(j);
            acc = c.add(&acc, &pt);
        }
        // Giant steps: (lo + i·b)·P; mP = O for m = lo + i·b + j iff
        // (lo + i·b)·P = −j·P.
        let step = c.mul(&pt, b);
        let mut g = c.mul(&pt, lo);
        let mut here = Vec::new();
        let mut i = 0u128;
        while lo + i * b <= hi {
            let target = key(&c.neg(&g));
            if let Some(js) = baby.get(&target) {
                for &j in js {
                    let m = lo + i * b + j;
                    if m <= hi && c.mul(&pt, m).inf {
                        here.push(m);
                    }
                }
            }
            g = c.add(&g, &step);
            i += 1;
        }
        here.sort_unstable();
        here.dedup();
        cands = Some(match cands {
            None => here,
            Some(v) => v.into_iter().filter(|m| here.contains(m)).collect(),
        });
        if cands.as_ref().is_some_and(|v| v.len() == 1) {
            return cands.map(|v| v[0]);
        }
    }
    None
}

fn is_prime(n: u128) -> bool {
    // First twelve prime bases are deterministic below
    // 318665857834031151167461; every supported order is below 2^73.
    assert!(n < (1u128 << 73), "prime check outside certified range");
    const BASES: [u128; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    if n < 2 {
        return false;
    }
    for p in BASES {
        if n == p {
            return true;
        }
        if n.is_multiple_of(p) {
            return false;
        }
    }
    use super::koblitz_strong_rho::RhoScalar;
    let mul = |a, b| <u128 as RhoScalar>::mul_mod(a, b, n);
    let pow = |mut a, mut e| {
        let mut r = 1;
        while e > 0 {
            if e & 1 == 1 {
                r = mul(r, a);
            }
            a = mul(a, a);
            e >>= 1;
        }
        r
    };
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'base: for a in BASES {
        let mut x = pow(a, d);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mul(x, x);
            if x == n - 1 {
                continue 'base;
            }
        }
        return false;
    }
    true
}

/// A curve of [`generate_instance`]'s family with `#E = h·r`, `r` prime.
pub struct BinCurve {
    pub curve: CurveB,
    pub instance_seed: u64,
    pub group_order: u128,
    pub r: u128,
    pub cofactor: u128,
    pub generator: PtB,
}

/// The first instance seed from `seed` with `#E = h·r`, `r` prime,
/// `h ≤ max_cofactor`.  Deterministic in `(m, seed, max_cofactor)`.
pub fn find_bin_curve(m: u32, seed: u64, max_cofactor: u64) -> Result<BinCurve, String> {
    for s in seed..seed + 2_000 {
        let curve = generate_instance(m, s);
        let mut rng = StdRng::seed_from_u64(s ^ 0x0c0f);
        let Some(order) = group_order_bin(&curve, &mut rng) else {
            continue;
        };
        let Some(h) = (2..=max_cofactor)
            .find(|&h| order % u128::from(h) == 0 && is_prime(order / u128::from(h)))
        else {
            continue;
        };
        let r = order / u128::from(h);
        let generator = loop {
            let x = curve.f.random(&mut rng);
            let Some(p) = curve.lift_x(&x, &mut rng) else {
                continue;
            };
            let g = curve.mul(&p, u128::from(h));
            if !g.inf {
                break g;
            }
        };
        if !curve.mul(&generator, r).inf {
            continue;
        }
        curve.f.f.muls.set(0);
        return Ok(BinCurve {
            curve,
            instance_seed: s,
            group_order: order,
            r,
            cofactor: u128::from(h),
            generator,
        });
    }
    Err(format!(
        "no curve with cofactor ≤ {max_cofactor} over F_(2^{m})^4 from seed {seed}"
    ))
}

/// `F_q` multiplications in one affine addition of two distinct points.
pub fn fq_muls_per_add(c: &CurveB) -> u64 {
    let mut rng = StdRng::seed_from_u64(0xadd);
    let p = loop {
        let x = c.f.random(&mut rng);
        if let Some(p) = c.lift_x(&x, &mut rng) {
            break p;
        }
    };
    let q = c.add(&p, &p);
    let before = c.f.f.muls();
    let _ = c.add(&p, &q);
    c.f.f.muls() - before
}

/// The tower as `GF(2)[X]/(g)`: `θ`, the first of `z, z + 1, z + x, …`
/// (`x` the generator of `F_q`) whose minimal polynomial over `F_2` has
/// degree `4m`, gives `g` (bit `i` for `X^i`, the `X^{4m}` bit included)
/// and the coordinates of an element in `1, θ, …, θ^{4m−1}`.  The curve's
/// identity is `curve_id::binary` of `(g, a, b)` in that basis.
pub struct TowerBasis {
    pub modulus: u128,
    /// Rows of the inverse of the matrix whose column `i` is `θ^i` packed.
    inv: Vec<u128>,
}

impl TowerBasis {
    pub fn new(f: &Fq4) -> TowerBasis {
        let n = 4 * f.f.m as usize;
        for c0 in 0..f.f.q as u32 {
            for c1 in 1..f.f.q as u32 {
                let theta = Eb([c0, c1, 0, 0]);
                // Powers θ⁰ … θⁿ packed; reduce θⁿ by the first n.
                let mut pw = Vec::with_capacity(n + 1);
                let mut acc = Eb::ONE;
                for _ in 0..=n {
                    pw.push(f.pack(&acc));
                    acc = f.mul_nc(&acc, &theta);
                }
                // Gaussian elimination over F_2 on columns: express each
                // packed bit vector in the θ-power basis.
                let mut rows: Vec<(u128, u128)> = Vec::new(); // (vector, combination)
                let mut ok = true;
                for (i, &v) in pw.iter().take(n).enumerate() {
                    let (mut v, mut comb) = (v, 1u128 << i);
                    for &(bv, bc) in &rows {
                        let lead = 127 - bv.leading_zeros();
                        if v >> lead & 1 == 1 {
                            v ^= bv;
                            comb ^= bc;
                        }
                    }
                    if v == 0 {
                        ok = false;
                        break;
                    }
                    rows.push((v, comb));
                    rows.sort_by_key(|x| std::cmp::Reverse(127 - x.0.leading_zeros()));
                }
                if !ok {
                    continue;
                }
                let solve = |mut v: u128| -> u128 {
                    let mut comb = 0u128;
                    for &(bv, bc) in &rows {
                        let lead = 127 - bv.leading_zeros();
                        if v >> lead & 1 == 1 {
                            v ^= bv;
                            comb ^= bc;
                        }
                    }
                    debug_assert_eq!(v, 0);
                    comb
                };
                let modulus = (1u128 << n) | solve(pw[n]);
                // The inverse map: packed bit j ↦ its θ-coordinates.
                let inv: Vec<u128> = (0..n).map(|j| solve(1u128 << j)).collect();
                return TowerBasis { modulus, inv };
            }
        }
        unreachable!("the tower has a generator over F_2")
    }
    /// `e` in the basis `1, θ, …, θ^{4m−1}`.
    pub fn coords(&self, f: &Fq4, e: &Eb) -> u128 {
        let v = f.pack(e);
        (0..self.inv.len())
            .filter(|&j| v >> j & 1 == 1)
            .fold(0u128, |acc, j| acc ^ self.inv[j])
    }
}
