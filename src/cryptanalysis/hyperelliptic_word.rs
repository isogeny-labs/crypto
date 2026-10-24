//! # Word-size hyperelliptic Jacobians: genus ≤ 4 over `F_p`, `p < 256`
//!
//! A `Copy` reduced Mumford divisor ([`HDiv`]) and Cantor's algorithm on
//! fixed-size stack polynomials ([`HCurve`]), for the imaginary model
//! `y² = f(x)`, `deg f = 2g + 1`, `f` squarefree.  It is the counted group
//! ecbench's generic arms (`rho.plain`, `rho.negation`, BSGS, the
//! kangaroo) run on for the hyperelliptic thread's instances
//! ([`crate::cryptanalysis::hyperelliptic_ic_bench::thread_instance`]); the
//! `BigUint` substrate ([`crate::prime_hyperelliptic`]) stays the reference
//! the tests compare against.
//!
//! - **Representation.**  `D = (u, v)`, `u` monic of degree `deg ≤ g`
//!   (its non-leading coefficients stored), `deg v < deg u`; the identity is
//!   `deg = 0`.  Reduced representatives are unique, so `==` is equality of
//!   classes.
//! - **Arithmetic.**  Cantor: composition by two extended gcds (one when
//!   `gcd(u₁, u₂) = 1`, by CRT), then the reduction loop
//!   `u' = (f − v²)/u` made monic, `v' = −v mod u'`.  Doubling is
//!   composition with itself.  `−(u, v) = (u, −v)`.
//! - **Charge.**  [`CountedGroup`] counts one addition or one doubling per
//!   Cantor call, whatever the field work behind it (`counts_field_ops`
//!   stays `false`: the field tally is unknown, not zero).
//! - **Key.**  Injective: `Σ_{e<deg} p^{2e}` plus the index of `(u, v)`
//!   among degree-`deg` pairs in base `p`.  Below `p^{2g} + p^{2g−2}·2`,
//!   so under `2^64` for `g = 4`, `p ≤ 251`.
//! - **Group order.**  [`HCurve::jacobian_order`]: `L(T)`'s first `g − 1`
//!   coefficients from point counts over `F_{p^k}`, `k < g`, leave `#J =
//!   L(1)` one unknown (`a_g`, `|a_g| ≤ C(2g, g)·p^{g/2}`); the multiples of
//!   the prime `r` in that window and the Hasse–Weil interval that kill 32
//!   random divisors are the candidates, and exactly one must survive.

use num_bigint::BigUint;
use num_traits::ToPrimitive;

use crate::cryptanalysis::ic_boundary::{CountedGroup, GroupOps};
use crate::cryptanalysis::residual_walk::{is_prime_u64, mix64};
use crate::prime_hyperelliptic::{FpPoly, HyperellipticCurveP, MumfordDivisorP};

/// The largest genus the fixed representation holds.
pub const MAX_GENUS: usize = 4;
/// Polynomial capacity (coefficients).  Cantor's largest intermediate at
/// `g = 4` has degree 13; 32 leaves room.
const CAP: usize = 32;

/// A reduced Mumford divisor `(u, v)` on a curve of genus ≤ 4 over `F_p`,
/// `p < 256`.  `u = x^deg + u[deg−1] x^{deg−1} + … + u[0]`, `v = Σ v[i] x^i`
/// (`i < deg`); unused slots are zero.  It does not record its curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct HDiv {
    pub deg: u8,
    pub u: [u8; MAX_GENUS],
    pub v: [u8; MAX_GENUS],
}

impl HDiv {
    /// The identity class `(1, 0)`.
    pub const IDENTITY: HDiv = HDiv {
        deg: 0,
        u: [0; MAX_GENUS],
        v: [0; MAX_GENUS],
    };

    pub fn is_identity(&self) -> bool {
        self.deg == 0
    }

    /// `u`, lowest degree first, with its leading 1 (length `deg + 1`).
    pub fn u_coeffs(&self) -> Vec<u64> {
        let d = self.deg as usize;
        let mut out: Vec<u64> = self.u[..d].iter().map(|&c| u64::from(c)).collect();
        out.push(1);
        out
    }

    /// `v`, lowest degree first, without trailing zeros.
    pub fn v_coeffs(&self) -> Vec<u64> {
        let mut out: Vec<u64> = self.v[..self.deg as usize]
            .iter()
            .map(|&c| u64::from(c))
            .collect();
        while out.last() == Some(&0) {
            out.pop();
        }
        out
    }
}

/// A polynomial over `F_p` on the stack: `c[..n]`, `c[n − 1] ≠ 0`, the
/// rest zero.  `n = 0` is the zero polynomial.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Poly {
    c: [u32; CAP],
    n: usize,
}

impl Poly {
    const ZERO: Poly = Poly { c: [0; CAP], n: 0 };

    fn constant(a: u32) -> Poly {
        let mut q = Poly::ZERO;
        if a != 0 {
            q.c[0] = a;
            q.n = 1;
        }
        q
    }

    fn from_slice(s: &[u32]) -> Poly {
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

    /// `deg`, with the zero polynomial at 0 (callers test `is_zero`).
    fn deg(&self) -> usize {
        self.n.saturating_sub(1)
    }

    fn lead(&self) -> u32 {
        if self.n == 0 {
            0
        } else {
            self.c[self.n - 1]
        }
    }
}

/// `F_p` for `p < 256`, with its inverse and square-root tables.
#[derive(Clone)]
struct Fp {
    p: u32,
    inv: [u16; 256],
    /// A square root of `a`, or `u16::MAX` for a non-residue.
    sqrt: [u16; 256],
}

impl Fp {
    fn new(p: u32) -> Fp {
        let mut inv = [0u16; 256];
        let mut sqrt = [u16::MAX; 256];
        for a in 1..p {
            for b in 1..p {
                if a * b % p == 1 {
                    inv[a as usize] = b as u16;
                    break;
                }
            }
        }
        for y in 0..p {
            let s = (y * y % p) as usize;
            if sqrt[s] == u16::MAX {
                sqrt[s] = y as u16;
            }
        }
        Fp { p, inv, sqrt }
    }

    #[inline]
    fn inv(&self, a: u32) -> u32 {
        debug_assert!(a != 0 && a < self.p);
        u32::from(self.inv[a as usize])
    }

    fn add(&self, a: &Poly, b: &Poly) -> Poly {
        let mut q = Poly::ZERO;
        let n = a.n.max(b.n);
        for i in 0..n {
            q.c[i] = (a.c[i] + b.c[i]) % self.p;
        }
        q.n = n;
        q.trim();
        q
    }

    fn sub(&self, a: &Poly, b: &Poly) -> Poly {
        let mut q = Poly::ZERO;
        let n = a.n.max(b.n);
        for i in 0..n {
            q.c[i] = (a.c[i] + self.p - b.c[i]) % self.p;
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

    fn scale(&self, a: &Poly, s: u32) -> Poly {
        let mut q = Poly::ZERO;
        for i in 0..a.n {
            q.c[i] = a.c[i] * s % self.p;
        }
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
        // p < 256: each product is < 2^16, a sum of ≤ 32 of them < 2^21.
        let mut q = Poly::ZERO;
        for i in 0..a.n {
            if a.c[i] == 0 {
                continue;
            }
            for j in 0..b.n {
                q.c[i + j] += a.c[i] * b.c[j];
            }
        }
        for c in q.c[..n].iter_mut() {
            *c %= self.p;
        }
        q.n = n;
        q.trim();
        q
    }

    /// `(a div b, a mod b)`, `b ≠ 0`.
    fn divrem(&self, a: &Poly, b: &Poly) -> (Poly, Poly) {
        assert!(!b.is_zero(), "division by the zero polynomial");
        let p = self.p;
        let mut r = *a;
        let mut q = Poly::ZERO;
        if a.n < b.n {
            return (q, r);
        }
        let li = self.inv(b.lead());
        let db = b.deg();
        while r.n >= b.n && !r.is_zero() {
            let shift = r.deg() - db;
            let coef = r.lead() * li % p;
            q.c[shift] = coef;
            for j in 0..b.n {
                let t = coef * b.c[j] % p;
                r.c[shift + j] = (r.c[shift + j] + p - t) % p;
            }
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

    /// `(g, s, t)` with `s·a + t·b = g`, `g` monic (zero only for `a = b = 0`).
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
        if r0.is_zero() {
            return (r0, s0, t0);
        }
        let li = self.inv(r0.lead());
        (
            self.scale(&r0, li),
            self.scale(&s0, li),
            self.scale(&t0, li),
        )
    }

    fn eval(&self, a: &Poly, x: u32) -> u32 {
        let mut acc = 0;
        for i in (0..a.n).rev() {
            acc = (acc * x + a.c[i]) % self.p;
        }
        acc
    }
}

/// A curve `y² = f(x)` of genus `g ≤ 4` over `F_p`, `p < 256`, `deg f =
/// 2g + 1`, `f` squarefree, with Cantor's arithmetic on its Jacobian.
#[derive(Clone)]
pub struct HCurve {
    k: Fp,
    genus: u8,
    f: Poly,
    /// `Σ_{e<d} p^{2e}`: where the degree-`d` keys start.
    offsets: [u64; MAX_GENUS + 1],
}

impl std::fmt::Debug for HCurve {
    fn fmt(&self, fm: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fm.debug_struct("HCurve")
            .field("p", &self.k.p)
            .field("genus", &self.genus)
            .field("f", &self.f_coeffs())
            .finish()
    }
}

impl HCurve {
    /// The curve `y² = f[0] + f[1] x + … + f[2g+1] x^{2g+1}` over `F_p`.
    /// Requires an odd prime `p < 256`, `1 ≤ g ≤ 4`, coefficients below
    /// `p`, `deg f = 2g + 1` exactly (trailing zeros allowed) and `f`
    /// squarefree.
    pub fn new(p: u64, genus: u32, f: &[u64]) -> Result<HCurve, String> {
        if !(3..256).contains(&p) || !is_prime_u64(p) {
            return Err(format!("p must be an odd prime below 256, not {p}"));
        }
        if !(1..=MAX_GENUS as u32).contains(&genus) {
            return Err(format!("genus must be 1 to {MAX_GENUS}, not {genus}"));
        }
        if let Some(&c) = f.iter().find(|&&c| c >= p) {
            return Err(format!("coefficient {c} is not reduced mod {p}"));
        }
        if f.len() > CAP {
            return Err("too many coefficients".into());
        }
        let k = Fp::new(p as u32);
        let fc: Vec<u32> = f.iter().map(|&c| c as u32).collect();
        let fp = Poly::from_slice(&fc);
        if fp.is_zero() || fp.deg() != 2 * genus as usize + 1 {
            return Err(format!(
                "deg f must be 2g + 1 = {} for the imaginary model",
                2 * genus + 1
            ));
        }
        // Squarefree: gcd(f, f') = 1.
        let mut d = Poly::ZERO;
        for i in 1..fp.n {
            d.c[i - 1] = fp.c[i] * (i as u32 % k.p) % k.p;
        }
        d.n = fp.n - 1;
        d.trim();
        if d.is_zero() || k.xgcd(&fp, &d).0.n != 1 {
            return Err("f is not squarefree".into());
        }
        let mut offsets = [0u64; MAX_GENUS + 1];
        for e in 1..=MAX_GENUS {
            let step = (p as u128).pow(2 * (e as u32 - 1));
            let next = u128::from(offsets[e - 1]) + step;
            offsets[e] = u64::try_from(next).unwrap_or(u64::MAX);
        }
        // Every key fits a word: the largest is offsets[g] + p^{2g} − 1.
        let top = u128::from(offsets[genus as usize]) + (p as u128).pow(2 * genus);
        if top > u128::from(u64::MAX) {
            return Err(format!("keys of genus {genus} over F_{p} pass 2^64"));
        }
        Ok(HCurve {
            k,
            genus: genus as u8,
            f: fp,
            offsets,
        })
    }

    /// The word-size copy of a `BigUint` curve.
    pub fn from_big(c: &HyperellipticCurveP) -> Result<HCurve, String> {
        let p = c.p.to_u64().ok_or("p does not fit a word")?;
        let f: Vec<u64> =
            c.f.coeffs
                .iter()
                .map(|x| x.to_u64().ok_or("coefficient past a word"))
                .collect::<Result<_, _>>()?;
        HCurve::new(p, c.genus, &f)
    }

    /// The `BigUint` copy of this curve.
    pub fn to_big(&self) -> HyperellipticCurveP {
        let p = BigUint::from(self.k.p);
        let f = FpPoly::from_coeffs(
            self.f_coeffs().into_iter().map(BigUint::from).collect(),
            p.clone(),
        );
        HyperellipticCurveP::new(p, f, u32::from(self.genus))
    }

    pub fn p(&self) -> u64 {
        u64::from(self.k.p)
    }

    pub fn genus(&self) -> u32 {
        u32::from(self.genus)
    }

    /// `f`, lowest degree first, length `2g + 2`.
    pub fn f_coeffs(&self) -> Vec<u64> {
        self.f.c[..self.f.n].iter().map(|&c| u64::from(c)).collect()
    }

    fn polys(&self, d: &HDiv) -> (Poly, Poly) {
        let n = d.deg as usize;
        let mut u = Poly::ZERO;
        for i in 0..n {
            u.c[i] = u32::from(d.u[i]);
        }
        u.c[n] = 1;
        u.n = n + 1;
        let mut v = Poly::ZERO;
        for i in 0..n {
            v.c[i] = u32::from(d.v[i]);
        }
        v.n = n;
        v.trim();
        (u, v)
    }

    /// The divisor of monic `u` (`deg ≤ g`) and `v` (`deg v < deg u`).
    fn pack(&self, u: &Poly, v: &Poly) -> HDiv {
        debug_assert!(u.lead() == 1 && u.deg() <= self.genus as usize);
        debug_assert!(v.is_zero() || v.deg() < u.deg());
        let mut d = HDiv {
            deg: u.deg() as u8,
            ..HDiv::IDENTITY
        };
        for i in 0..u.deg() {
            d.u[i] = u.c[i] as u8;
        }
        for i in 0..v.n {
            d.v[i] = v.c[i] as u8;
        }
        d
    }

    /// `D` is a reduced divisor on this curve: `deg ≤ g`, coefficients
    /// below `p`, unused slots zero, and `u | f − v²`.
    pub fn is_valid(&self, d: &HDiv) -> bool {
        let n = d.deg as usize;
        if n > self.genus as usize {
            return false;
        }
        let p = self.k.p;
        if d.u.iter().chain(d.v.iter()).any(|&c| u32::from(c) >= p) {
            return false;
        }
        if d.u[n..].iter().chain(d.v[n..].iter()).any(|&c| c != 0) {
            return false;
        }
        let (u, v) = self.polys(d);
        let t = self.k.sub(&self.f, &self.k.mul(&v, &v));
        self.k.rem(&t, &u).is_zero()
    }

    /// `[P] − [∞]` for an affine point `P = (x, y)` on the curve.
    pub fn point(&self, x: u64, y: u64) -> Option<HDiv> {
        let p = u64::from(self.k.p);
        if x >= p || y >= p {
            return None;
        }
        let fx = self.k.eval(&self.f, x as u32);
        if (y * y % p) as u32 != fx {
            return None;
        }
        let mut d = HDiv {
            deg: 1,
            ..HDiv::IDENTITY
        };
        d.u[0] = ((p - x) % p) as u8;
        d.v[0] = y as u8;
        Some(d)
    }

    /// A random divisor: the sum of `g` random affine points (with
    /// either ordinate), from a SplitMix64 stream `state`.
    pub fn random_divisor(&self, state: &mut u64) -> HDiv {
        let p = self.k.p;
        let mut d = HDiv::IDENTITY;
        let mut placed = 0;
        while placed < self.genus {
            *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let h = mix64(*state);
            let x = (h % u64::from(p)) as u32;
            let s = self.k.sqrt[self.k.eval(&self.f, x) as usize];
            if s == u16::MAX {
                continue;
            }
            let y = if (h >> 63) & 1 == 1 {
                (p - u32::from(s)) % p
            } else {
                u32::from(s)
            };
            let pt = self
                .point(u64::from(x), u64::from(y))
                .expect("a lifted point lies on the curve");
            d = self.cantor(d, pt);
            placed += 1;
        }
        d
    }

    /// Cantor's composition and reduction, uncounted.
    pub fn cantor(&self, a: HDiv, b: HDiv) -> HDiv {
        if a.is_identity() {
            return b;
        }
        if b.is_identity() {
            return a;
        }
        let k = &self.k;
        let (u1, v1) = self.polys(&a);
        let (u2, v2) = self.polys(&b);
        let (d1, e1, e2) = k.xgcd(&u1, &u2);
        let (u, v) = if d1.n == 1 {
            // gcd(u₁, u₂) = 1: v ≡ vᵢ (mod uᵢ) by CRT, e₁u₁ ≡ 1 (mod u₂).
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
        self.reduce(u, v)
    }

    fn reduce(&self, mut u: Poly, mut v: Poly) -> HDiv {
        let k = &self.k;
        let g = self.genus as usize;
        while u.deg() > g {
            let t = k.sub(&self.f, &k.mul(&v, &v));
            let (q, r) = k.divrem(&t, &u);
            debug_assert!(r.is_zero(), "(f − v²)/u is exact");
            let un = k.monic(&q);
            v = k.rem(&k.neg(&v), &un);
            u = un;
        }
        let u = k.monic(&u);
        self.pack(&u, &v)
    }

    /// `−D = (u, −v)`.
    pub fn negate(&self, d: HDiv) -> HDiv {
        let p = self.k.p;
        let mut n = d;
        for i in 0..d.deg as usize {
            n.v[i] = ((p - u32::from(d.v[i])) % p) as u8;
        }
        n
    }

    /// `[k]D`, uncounted.
    pub fn mul_uncounted(&self, d: HDiv, k: u64) -> HDiv {
        let mut ops = GroupOps::default();
        CountedGroup::mul(self, &mut ops, d, k)
    }

    /// The injective key (see the module notes).
    pub fn key_of(&self, d: &HDiv) -> u64 {
        let p = u64::from(self.k.p);
        let n = d.deg as usize;
        let mut idx = 0u64;
        for i in (0..n).rev() {
            idx = idx * p + u64::from(d.v[i]);
        }
        for i in (0..n).rev() {
            idx = idx * p + u64::from(d.u[i]);
        }
        self.offsets[n] + idx
    }

    /// The `MumfordDivisorP` of `d` on [`Self::to_big`]'s curve.
    pub fn to_mumford(&self, d: &HDiv) -> MumfordDivisorP {
        let p = BigUint::from(self.k.p);
        MumfordDivisorP {
            u: FpPoly::from_coeffs(
                d.u_coeffs().into_iter().map(BigUint::from).collect(),
                p.clone(),
            ),
            v: FpPoly::from_coeffs(d.v_coeffs().into_iter().map(BigUint::from).collect(), p),
        }
    }

    /// The word-size divisor of a reduced `MumfordDivisorP` on this curve.
    pub fn from_mumford(&self, m: &MumfordDivisorP) -> Option<HDiv> {
        let small = |c: &BigUint| c.to_u64().filter(|&x| x < u64::from(self.k.p));
        let u: Vec<u64> = m.u.coeffs.iter().map(small).collect::<Option<_>>()?;
        let v: Vec<u64> = m.v.coeffs.iter().map(small).collect::<Option<_>>()?;
        self.from_coeffs(&u, &v)
    }

    /// The divisor of `u` (with its leading 1, lowest degree first) and
    /// `v` (lowest first, `deg v < deg u`), checked to be reduced on this
    /// curve.
    pub fn from_coeffs(&self, u: &[u64], v: &[u64]) -> Option<HDiv> {
        let deg = u.len().checked_sub(1)?;
        if deg > self.genus as usize || u[deg] != 1 {
            return None;
        }
        let mut v = v.to_vec();
        while v.last() == Some(&0) {
            v.pop();
        }
        if v.len() > deg {
            return None;
        }
        let p = u64::from(self.k.p);
        if u.iter().chain(v.iter()).any(|&c| c >= p) {
            return None;
        }
        let mut d = HDiv {
            deg: deg as u8,
            ..HDiv::IDENTITY
        };
        for i in 0..deg {
            d.u[i] = u[i] as u8;
        }
        for (i, &c) in v.iter().enumerate() {
            d.v[i] = c as u8;
        }
        self.is_valid(&d).then_some(d)
    }

    // ── Point counts and the group order ────────────────────────────

    /// `#C(F_{p^k})` (the one point at infinity included), `1 ≤ k ≤ 3`, by
    /// enumeration: `O(p^k)` evaluations of `f`.
    pub fn count_points(&self, k: u32) -> u64 {
        assert!((1..=3).contains(&k), "point counts over F_p^k for k ≤ 3");
        let p = self.k.p;
        let is_sq = |a: u32| self.k.sqrt[a as usize] != u16::MAX;
        if k == 1 {
            let mut n = 1u64;
            for x in 0..p {
                let y2 = self.k.eval(&self.f, x);
                n += if y2 == 0 {
                    1
                } else if is_sq(y2) {
                    2
                } else {
                    0
                };
            }
            return n;
        }
        let e = ExtField::new(&self.k, k as usize);
        let mut n = 1u64;
        let mut x = [0u32; 3];
        let total = (p as u64).pow(k);
        for _ in 0..total {
            let y2 = e.eval(&self.f, &x);
            let nm = e.norm(&y2);
            n += if y2 == [0; 3] {
                1
            } else if is_sq(nm) {
                2
            } else {
                0
            };
            // Next x in base p.
            for c in x.iter_mut().take(k as usize) {
                *c += 1;
                if *c < p {
                    break;
                }
                *c = 0;
            }
        }
        n
    }

    /// `#J(F_p)`, given a prime `r` known to divide it (see the module
    /// notes).  `seed` drives the random divisors of the exponent test.
    pub fn jacobian_order(&self, r: u64, seed: u64) -> Result<u64, String> {
        let g = self.genus as usize;
        let p = i128::from(self.k.p);
        if r < 2 || !is_prime_u64(r) {
            return Err(format!("r = {r} is not prime"));
        }
        // L(T) = Σ c_k T^k: c_k for k < g by Newton from N_1 … N_{g−1}.
        let s: Vec<i128> = (1..g as u32)
            .map(|k| self.count_points(k) as i128 - p.pow(k) - 1)
            .collect();
        let mut c = vec![0i128; g];
        c[0] = 1;
        for k in 1..g {
            let num: i128 = (1..=k).map(|i| s[i - 1] * c[k - i]).sum();
            if num % k as i128 != 0 {
                return Err("point counts violate Newton integrality".into());
            }
            c[k] = num / k as i128;
        }
        // L(1) = K + c_g, with c_{g+j} = p^j c_{g−j}.
        let base: i128 =
            c.iter().sum::<i128>() + (1..=g).map(|j| p.pow(j as u32) * c[g - j]).sum::<i128>();
        let binom: i128 = (0..g as i128).fold(1, |acc, i| acc * (2 * g as i128 - i) / (i + 1));
        let bound = (binom as f64 * (p as f64).powf(g as f64 / 2.0)).ceil() as i128 + 1;
        let sp = (p as f64).sqrt();
        let lo = ((sp - 1.0).powi(2 * g as i32) * (1.0 - 1e-9))
            .floor()
            .max(1.0) as i128;
        let hi = ((sp + 1.0).powi(2 * g as i32) * (1.0 + 1e-9)).ceil() as i128;
        let from = (base - bound).max(lo).max(1);
        let to = (base + bound).min(hi).min(i128::from(u64::MAX));
        let r = i128::from(r);
        let mut state = seed;
        let tests: Vec<HDiv> = (0..32).map(|_| self.random_divisor(&mut state)).collect();
        let mut survivors = Vec::new();
        let mut n = (from + r - 1) / r * r;
        while n <= to {
            if tests
                .iter()
                .all(|d| self.mul_uncounted(*d, n as u64).is_identity())
            {
                survivors.push(n as u64);
            }
            n += r;
        }
        match survivors.as_slice() {
            [one] => Ok(*one),
            [] => Err("no multiple of r in the L-polynomial window kills the test divisors".into()),
            many => Err(format!(
                "#J is ambiguous: {} candidates ({:?}…) kill all 32 test divisors",
                many.len(),
                &many[..many.len().min(4)]
            )),
        }
    }
}

/// `F_{p^k} = F_p[t]/(m)`, `k ∈ {2, 3}`, for point counting.
struct ExtField<'a> {
    k: &'a Fp,
    deg: usize,
    /// `m = t^deg + Σ m[i] t^i`.
    m: [u32; 3],
    /// `t^{ip}`, the Frobenius images of the basis.
    frob: [[u32; 3]; 3],
}

impl<'a> ExtField<'a> {
    fn new(k: &'a Fp, deg: usize) -> ExtField<'a> {
        let p = k.p;
        let m = if deg == 2 {
            let nu = (2..p)
                .find(|&a| k.sqrt[a as usize] == u16::MAX)
                .expect("an odd prime has a non-residue");
            [p - nu, 0, 0]
        } else {
            // t³ + a t + b with no root in F_p is irreducible.
            let mut found = None;
            'search: for a in 0..p {
                for b in 1..p {
                    if (0..p).all(|x| !(x * x % p * x + a * x + b).is_multiple_of(p)) {
                        found = Some([b, a, 0]);
                        break 'search;
                    }
                }
            }
            found.expect("an irreducible cubic exists")
        };
        let mut e = ExtField {
            k,
            deg,
            m,
            frob: [[0; 3]; 3],
        };
        let mut t = [0u32; 3];
        t[1] = 1;
        let tp = e.pow(&t, u64::from(p));
        let mut acc = [1u32, 0, 0];
        for i in 0..deg {
            e.frob[i] = acc;
            acc = e.mul(&acc, &tp);
        }
        e
    }

    fn mul(&self, a: &[u32; 3], b: &[u32; 3]) -> [u32; 3] {
        let p = self.k.p;
        let d = self.deg;
        let mut prod = [0u32; 5];
        for i in 0..d {
            for j in 0..d {
                prod[i + j] += a[i] * b[j];
            }
        }
        for c in prod.iter_mut() {
            *c %= p;
        }
        for i in (d..2 * d - 1).rev() {
            let c = prod[i];
            if c == 0 {
                continue;
            }
            prod[i] = 0;
            for j in 0..d {
                prod[i - d + j] = (prod[i - d + j] + (p - c) * self.m[j]) % p;
            }
        }
        [prod[0], prod[1], prod[2]]
    }

    fn pow(&self, a: &[u32; 3], mut e: u64) -> [u32; 3] {
        let mut acc = [1u32, 0, 0];
        let mut b = *a;
        while e > 0 {
            if e & 1 == 1 {
                acc = self.mul(&acc, &b);
            }
            b = self.mul(&b, &b);
            e >>= 1;
        }
        acc
    }

    fn frobenius(&self, a: &[u32; 3]) -> [u32; 3] {
        let p = self.k.p;
        let mut out = [0u32; 3];
        for i in 0..self.deg {
            for (j, o) in out.iter_mut().enumerate().take(self.deg) {
                *o = (*o + a[i] * self.frob[i][j]) % p;
            }
        }
        out
    }

    /// `N(a) = a · a^p · …`, an element of `F_p`.
    fn norm(&self, a: &[u32; 3]) -> u32 {
        let mut acc = *a;
        let mut conj = *a;
        for _ in 1..self.deg {
            conj = self.frobenius(&conj);
            acc = self.mul(&acc, &conj);
        }
        debug_assert!(acc[1] == 0 && acc[2] == 0, "a norm lies in F_p");
        acc[0]
    }

    fn eval(&self, f: &Poly, x: &[u32; 3]) -> [u32; 3] {
        let p = self.k.p;
        let mut acc = [0u32; 3];
        for i in (0..f.n).rev() {
            acc = self.mul(&acc, x);
            acc[0] = (acc[0] + f.c[i]) % p;
        }
        acc
    }
}

impl CountedGroup for HCurve {
    type Elt = HDiv;
    fn identity(&self) -> HDiv {
        HDiv::IDENTITY
    }
    fn is_identity(&self, d: &HDiv) -> bool {
        d.is_identity()
    }
    fn add(&self, ops: &mut GroupOps, a: HDiv, b: HDiv) -> HDiv {
        if a == b {
            ops.doubles += 1;
        } else {
            ops.adds += 1;
        }
        self.cantor(a, b)
    }
    fn double(&self, ops: &mut GroupOps, a: HDiv) -> HDiv {
        ops.doubles += 1;
        self.cantor(a, a)
    }
    fn neg(&self, a: HDiv) -> HDiv {
        self.negate(a)
    }
    fn key(&self, d: &HDiv) -> u64 {
        self.key_of(d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryptanalysis::genus2_fp::Genus2Curve;
    use crate::cryptanalysis::hyperelliptic_ic_bench::thread_instance;
    use std::collections::HashSet;

    fn instances() -> Vec<(HCurve, HDiv, u64, HyperellipticCurveP)> {
        [(2u32, 41u64), (3, 23), (4, 17)]
            .iter()
            .map(|&(g, p)| {
                let (big, d1, n, _c) = thread_instance(p, g).expect("thread instance");
                let hc = HCurve::from_big(&big).unwrap();
                let gen = hc.from_mumford(&d1).expect("generator converts");
                (hc, gen, n.to_u64().unwrap(), big)
            })
            .collect()
    }

    #[test]
    fn cantor_agrees_with_the_biguint_reference() {
        for (hc, gen, r, big) in instances() {
            let mut st = 0x1234_5678u64 ^ hc.p();
            for i in 0..60u64 {
                let a = hc.random_divisor(&mut st);
                let b = if i % 5 == 0 {
                    a
                } else if i % 7 == 0 {
                    hc.negate(a)
                } else {
                    hc.random_divisor(&mut st)
                };
                assert!(hc.is_valid(&a) && hc.is_valid(&b));
                let (ma, mb) = (hc.to_mumford(&a), hc.to_mumford(&b));
                assert!(ma.is_reduced(&big));
                let sum = hc.cantor(a, b);
                assert!(hc.is_valid(&sum));
                assert_eq!(
                    hc.to_mumford(&sum),
                    ma.add(&mb, &big),
                    "add, g = {}",
                    hc.genus()
                );
                assert_eq!(hc.to_mumford(&hc.cantor(a, a)), ma.double(&big));
                assert_eq!(hc.to_mumford(&hc.negate(a)), ma.neg(&big));
                assert_eq!(hc.from_mumford(&ma), Some(a));
                let k = mix64(i ^ r) % r;
                assert_eq!(
                    hc.to_mumford(&hc.mul_uncounted(a, k)),
                    ma.scalar_mul(&BigUint::from(k), &big)
                );
            }
            // r·D₁ = 0 for the thread's generator, and D₁ ≠ 0.
            assert!(!gen.is_identity());
            assert!(hc.mul_uncounted(gen, r).is_identity());
            assert!(!hc.mul_uncounted(gen, r - 1).is_identity());
        }
    }

    #[test]
    fn keys_are_injective_and_split_negation_pairs() {
        for (hc, _gen, _r, _big) in instances() {
            let mut st = 99u64;
            let mut seen: std::collections::HashMap<u64, HDiv> = Default::default();
            for _ in 0..4000 {
                let d = hc.random_divisor(&mut st);
                if let Some(prev) = seen.insert(hc.key_of(&d), d) {
                    assert_eq!(prev, d, "key collision");
                }
                let n = hc.negate(d);
                if n != d {
                    assert_ne!(hc.key_of(&n), hc.key_of(&d));
                }
            }
            assert_eq!(hc.key_of(&HDiv::IDENTITY), 0);
        }
    }

    #[test]
    fn keys_cover_degree_shells_exactly_at_genus_two_p_eleven() {
        // Every reduced divisor of a small genus-2 Jacobian, by enumeration
        // of (u, v) pairs: the keys are distinct.
        let f = (1..11u64)
            .map(|c| [c, 1, 2, 3, 0, 1])
            .find(|f| HCurve::new(11, 2, f).is_ok())
            .expect("a squarefree thread model over F_11");
        let hc = HCurve::new(11, 2, &f).unwrap();
        let mut keys = HashSet::new();
        let mut count = 1u64;
        keys.insert(0u64);
        for deg in 1..=2usize {
            let total = 11u64.pow(2 * deg as u32);
            for idx in 0..total {
                let mut x = idx;
                let mut u = vec![0u64; deg + 1];
                let mut v = vec![0u64; deg];
                for c in u.iter_mut().take(deg) {
                    *c = x % 11;
                    x /= 11;
                }
                u[deg] = 1;
                for c in v.iter_mut() {
                    *c = x % 11;
                    x /= 11;
                }
                if let Some(d) = hc.from_coeffs(&u, &v) {
                    assert!(keys.insert(hc.key_of(&d)));
                    count += 1;
                }
            }
        }
        let g2 = Genus2Curve::new(11, f).unwrap();
        assert_eq!(count, g2.jacobian_order().unwrap());
    }

    #[test]
    fn largest_key_fits_a_word_at_genus_four_p_251() {
        let mut f = vec![0u64; 10];
        f[0] = 1;
        f[1] = 3;
        f[3] = 1;
        f[9] = 1;
        let hc = HCurve::new(251, 4, &f).unwrap();
        let top = HDiv {
            deg: 4,
            u: [250; 4],
            v: [250; 4],
        };
        let k = hc.key_of(&top) as u128;
        let expect: u128 = (0..4).map(|e| 251u128.pow(2 * e)).sum::<u128>() + 251u128.pow(8) - 1;
        assert_eq!(k, expect);
        assert!(expect < u128::from(u64::MAX));
    }

    #[test]
    fn genus_two_agrees_with_genus2_fp() {
        let f = [7u64, 1, 2, 3, 0, 1];
        let hc = HCurve::new(41, 2, &f).unwrap();
        let g2 = Genus2Curve::new(41, f).unwrap();
        let mut st = 5u64;
        for _ in 0..200 {
            let a = hc.random_divisor(&mut st);
            let b = hc.random_divisor(&mut st);
            let conv = |d: &HDiv| g2.divisor(&d.u_coeffs(), &d.v_coeffs()).unwrap();
            let (ga, gb) = (conv(&a), conv(&b));
            let s = hc.cantor(a, b);
            assert_eq!(conv(&s), g2.add(&ga, &gb));
            assert_eq!(conv(&hc.cantor(a, a)), g2.double(&ga));
        }
    }

    #[test]
    fn group_order_matches_genus2_fp_and_kills_the_generator() {
        let (hc, gen, r, big) = instances().remove(0);
        let order = hc.jacobian_order(r, 7).unwrap();
        let f: [u64; 6] = hc.f_coeffs().try_into().unwrap();
        let g2 = Genus2Curve::new(hc.p(), f).unwrap();
        assert_eq!(order, g2.jacobian_order().unwrap());
        assert_eq!(order % r, 0);
        assert!(hc.mul_uncounted(gen, order).is_identity());
        assert_eq!(
            BigUint::from(hc.count_points(1)),
            big.count_affine_plus_infinity()
        );
    }

    #[test]
    fn point_counts_agree_with_the_biguint_counter() {
        for (hc, _gen, _r, big) in instances() {
            assert_eq!(
                BigUint::from(hc.count_points(1)),
                big.count_affine_plus_infinity()
            );
        }
    }

    #[test]
    fn higher_genus_orders_are_unique_and_kill_random_divisors() {
        for (hc, gen, r, _big) in instances() {
            let order = hc.jacobian_order(r, 11).unwrap();
            assert_eq!(order % r, 0);
            let mut st = 1u64;
            for _ in 0..16 {
                let d = hc.random_divisor(&mut st);
                assert!(hc.mul_uncounted(d, order).is_identity());
            }
            assert!(hc.mul_uncounted(gen, order).is_identity());
        }
    }

    #[test]
    fn genus_three_order_matches_full_point_counts_by_newton() {
        // With all g counts the L-polynomial is determined; it must agree.
        let (hc, _gen, r, _big) = instances().remove(1);
        let p = hc.p() as i128;
        let s: Vec<i128> = (1..=3u32)
            .map(|k| hc.count_points(k) as i128 - p.pow(k) - 1)
            .collect();
        let mut c = [1i128, 0, 0, 0];
        for k in 1..=3 {
            let num: i128 = (1..=k).map(|i| s[i - 1] * c[k - i]).sum();
            c[k] = num / k as i128;
        }
        let l1 = c.iter().sum::<i128>() + p * c[2] + p * p * c[1] + p * p * p;
        assert_eq!(hc.jacobian_order(r, 3).unwrap() as i128, l1);
    }
}
