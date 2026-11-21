use std::cell::Cell;
use crate::rng::mix64;
pub const MAX_GENUS: usize = 5;
/// Polynomial capacity: the largest Cantor intermediate at `g = 5` is
/// `v²` with `deg v ≤ 2g − 1`, 19 coefficients.
pub const CAP: usize = 24;

// ── polynomials over F_p on the stack ───────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Poly {
    pub c: [u64; CAP],
    pub n: usize,
}

impl Poly {
    pub const ZERO: Poly = Poly { c: [0; CAP], n: 0 };

    pub fn constant(a: u64) -> Poly {
        let mut q = Poly::ZERO;
        if a != 0 {
            q.c[0] = a;
            q.n = 1;
        }
        q
    }
    pub fn from_slice(s: &[u64]) -> Poly {
        assert!(s.len() <= CAP, "polynomial past capacity");
        let mut q = Poly::ZERO;
        q.c[..s.len()].copy_from_slice(s);
        q.n = s.len();
        q.trim();
        q
    }
    pub fn trim(&mut self) {
        while self.n > 0 && self.c[self.n - 1] == 0 {
            self.n -= 1;
        }
    }
    pub fn is_zero(&self) -> bool {
        self.n == 0
    }
    pub fn deg(&self) -> usize {
        self.n.saturating_sub(1)
    }
    pub fn lead(&self) -> u64 {
        if self.n == 0 {
            0
        } else {
            self.c[self.n - 1]
        }
    }
}

/// `F_p` with a tally of the multiplications and inversions spent in the
/// group law.
#[derive(Clone, Debug)]
pub struct Fk {
    pub p: u64,
    pub mul: Cell<u64>,
    pub inv: Cell<u64>,
}

impl Fk {
    #[inline]
    pub fn tick(&self, n: usize) {
        self.mul.set(self.mul.get() + n as u64);
    }
    pub fn inv(&self, a: u64) -> u64 {
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
    pub fn add(&self, a: &Poly, b: &Poly) -> Poly {
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
    pub fn sub(&self, a: &Poly, b: &Poly) -> Poly {
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
    pub fn neg(&self, a: &Poly) -> Poly {
        let mut q = *a;
        for i in 0..a.n {
            q.c[i] = (self.p - a.c[i]) % self.p;
        }
        q
    }
    pub fn scale(&self, a: &Poly, s: u64) -> Poly {
        let mut q = Poly::ZERO;
        for i in 0..a.n {
            q.c[i] = a.c[i] * s % self.p;
        }
        self.tick(a.n);
        q.n = a.n;
        q.trim();
        q
    }
    /// Schoolbook product, `a.n · b.n` multiplications.
    pub fn mul(&self, a: &Poly, b: &Poly) -> Poly {
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
    /// `(a div b, a mod b)`, `b ≠ 0`.
    pub fn divrem(&self, a: &Poly, b: &Poly) -> (Poly, Poly) {
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
    pub fn rem(&self, a: &Poly, b: &Poly) -> Poly {
        self.divrem(a, b).1
    }
    pub fn monic(&self, a: &Poly) -> Poly {
        if a.is_zero() || a.lead() == 1 {
            return *a;
        }
        self.scale(a, self.inv(a.lead()))
    }
    /// `(g, s, t)` with `s·a + t·b = g`, `g` monic.
    pub fn xgcd(&self, a: &Poly, b: &Poly) -> (Poly, Poly, Poly) {
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
    pub fn eval(&self, a: &Poly, x: u64) -> u64 {
        let mut acc = 0;
        for i in (0..a.n).rev() {
            acc = (acc * x + a.c[i]) % self.p;
        }
        acc
    }
}

pub fn pow_mod(mut a: u64, mut e: u64, p: u64) -> u64 {
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

/// A square root of `a` modulo the odd prime `p` (Tonelli–Shanks), uncounted
/// (instance set-up only).
pub fn sqrt_mod(a: u64, p: u64) -> Option<u64> {
    if a == 0 {
        return Some(0);
    }
    if pow_mod(a, (p - 1) / 2, p) != 1 {
        return None;
    }
    if p % 4 == 3 {
        return Some(pow_mod(a, p.div_ceil(4), p));
    }
    let (mut q, mut s) = (p - 1, 0u32);
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }
    let mut z = 2;
    while pow_mod(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }
    let (mut m, mut c, mut t, mut r) = (
        s,
        pow_mod(z, q, p),
        pow_mod(a, q, p),
        pow_mod(a, q.div_ceil(2), p),
    );
    while t != 1 {
        let mut i = 0;
        let mut tt = t;
        while tt != 1 {
            tt = tt * tt % p;
            i += 1;
        }
        let b = pow_mod(c, 1 << (m - i - 1), p);
        m = i;
        c = b * b % p;
        t = t * c % p;
        r = r * b % p;
    }
    Some(r)
}

// ── the Jacobian ────────────────────────────────────────────────────────

/// A reduced Mumford divisor `(u, v)`, `u` monic of degree `deg ≤ g` (its
/// non-leading coefficients stored), `deg v < deg u`; unused slots are zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct JDiv {
    pub deg: u8,
    pub u: [u32; MAX_GENUS],
    pub v: [u32; MAX_GENUS],
}

impl JDiv {
    pub const IDENTITY: JDiv = JDiv {
        deg: 0,
        u: [0; MAX_GENUS],
        v: [0; MAX_GENUS],
    };
    pub fn is_identity(&self) -> bool {
        self.deg == 0
    }
}

/// `y² = f(x)` of genus `g ≤ 5` over `F_p`, `deg f = 2g + 1`, `f`
/// squarefree, with Cantor's arithmetic and a tally of its field work.
#[derive(Clone, Debug)]
pub struct JCurve {
    pub k: Fk,
    genus: u8,
    pub f: Poly,
}

impl JCurve {
    pub fn new(p: u64, genus: u32, f: &[u64]) -> Result<JCurve, String> {
        if !(3..(1 << 31)).contains(&p) || !is_prime(p) {
            return Err(format!("p must be an odd prime below 2^31, not {p}"));
        }
        if !(1..=MAX_GENUS as u32).contains(&genus) {
            return Err(format!("genus must be 1 to {MAX_GENUS}, not {genus}"));
        }
        if ((p as f64).sqrt() + 1.0).powi(2 * genus as i32) >= 9.0e18 {
            return Err(format!("#J does not fit 63 bits at p = {p}, g = {genus}"));
        }
        if f.iter().any(|&c| c >= p) {
            return Err("coefficient not reduced".into());
        }
        if f.len() != 2 * genus as usize + 2 || f[f.len() - 1] == 0 {
            return Err(format!("deg f must be 2g + 1 = {}", 2 * genus + 1));
        }
        let k = Fk {
            p,
            mul: Cell::new(0),
            inv: Cell::new(0),
        };
        let fp = Poly::from_slice(f);
        let mut d = Poly::ZERO;
        for i in 1..fp.n {
            d.c[i - 1] = fp.c[i] * (i as u64 % p) % p;
        }
        d.n = fp.n - 1;
        d.trim();
        if d.is_zero() || k.xgcd(&fp, &d).0.n != 1 {
            return Err("f is not squarefree".into());
        }
        k.mul.set(0);
        k.inv.set(0);
        Ok(JCurve {
            k,
            genus: genus as u8,
            f: fp,
        })
    }

    pub fn p(&self) -> u64 {
        self.k.p
    }
    pub fn genus(&self) -> u32 {
        u32::from(self.genus)
    }
    pub fn f_coeffs(&self) -> Vec<u64> {
        self.f.c[..self.f.n].to_vec()
    }
    /// `F_p` multiplications spent in the group law since construction.
    pub fn fmul(&self) -> u64 {
        self.k.mul.get()
    }
    /// `F_p` inversions spent in the group law since construction.
    pub fn finv(&self) -> u64 {
        self.k.inv.get()
    }

    pub fn polys(&self, d: &JDiv) -> (Poly, Poly) {
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

    pub fn pack(&self, u: &Poly, v: &Poly) -> JDiv {
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

    /// `u`, lowest degree first, with its leading 1.
    pub fn u_coeffs(&self, d: &JDiv) -> Vec<u64> {
        let mut out: Vec<u64> = d.u[..d.deg as usize]
            .iter()
            .map(|&c| u64::from(c))
            .collect();
        out.push(1);
        out
    }
    /// `v`, lowest degree first, `deg` entries.
    pub fn v_coeffs(&self, d: &JDiv) -> Vec<u64> {
        d.v[..d.deg as usize]
            .iter()
            .map(|&c| u64::from(c))
            .collect()
    }

    /// `D` is a reduced divisor on this curve.
    pub fn is_valid(&self, d: &JDiv) -> bool {
        let n = d.deg as usize;
        if n > self.genus as usize {
            return false;
        }
        let p = self.k.p;
        if d.u.iter().chain(d.v.iter()).any(|&c| u64::from(c) >= p) {
            return false;
        }
        if d.u[n..].iter().chain(d.v[n..].iter()).any(|&c| c != 0) {
            return false;
        }
        let (u, v) = self.polys(d);
        let t = self.k.sub(&self.f, &self.k.mul(&v, &v));
        self.k.rem(&t, &u).is_zero()
    }

    /// `[P] − [∞]` for an affine point.
    pub fn point(&self, x: u64, y: u64) -> Option<JDiv> {
        let p = self.k.p;
        if x >= p || y >= p || y * y % p != self.k.eval(&self.f, x) {
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

    /// The sum of `g` random affine points, from a SplitMix64 stream.
    pub fn random_divisor(&self, state: &mut u64) -> JDiv {
        let p = self.k.p;
        let mut d = JDiv::IDENTITY;
        let mut placed = 0;
        while placed < self.genus {
            *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let h = mix64(*state);
            let x = h % p;
            let Some(s) = sqrt_mod(self.k.eval(&self.f, x), p) else {
                continue;
            };
            let y = if (h >> 63) & 1 == 1 { (p - s) % p } else { s };
            let pt = self.point(x, y).expect("a lifted point lies on the curve");
            d = self.cantor(d, pt);
            placed += 1;
        }
        d
    }

    /// Cantor's composition and reduction.
    pub fn cantor(&self, a: JDiv, b: JDiv) -> JDiv {
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
            let u = k.mul(&u1, &u2);
            let t = k.mul(&k.mul(&e1, &u1), &k.sub(&v2, &v1));
            (u, k.rem(&k.add(&v1, &t), &u))
        } else {
            let (d, c1, c2) = k.xgcd(&d1, &k.add(&v1, &v2));
            let s1 = k.mul(&c1, &e1);
            let s2 = k.mul(&c1, &e2);
            let (u, ur) = k.divrem(&k.mul(&u1, &u2), &k.mul(&d, &d));
            debug_assert!(ur.is_zero(), "u₁u₂/d² is exact");
            // (s₁u₁v₂ + s₂u₂v₁ + c₂(v₁v₂ + f)) / d; degree at most 4g − 3.
            let t1 = k.mul(&k.mul(&s1, &u1), &v2);
            let t2 = k.mul(&k.mul(&s2, &u2), &v1);
            let t3 = k.mul(&c2, &k.add(&k.mul(&v1, &v2), &self.f));
            let (vq, vr) = k.divrem(&k.add(&k.add(&t1, &t2), &t3), &d);
            debug_assert!(vr.is_zero(), "the v numerator divides by d");
            (u, k.rem(&vq, &u))
        };
        self.reduce(u, v)
    }

    pub fn reduce(&self, mut u: Poly, mut v: Poly) -> JDiv {
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

    pub fn negate(&self, d: JDiv) -> JDiv {
        let p = self.k.p;
        let mut n = d;
        for i in 0..d.deg as usize {
            n.v[i] = ((p - u64::from(d.v[i])) % p) as u32;
        }
        n
    }

    /// `[k]D`, left-to-right.
    pub fn mul_ltr(&self, d: JDiv, k: u64) -> JDiv {
        let mut acc = JDiv::IDENTITY;
        for i in (0..64 - k.leading_zeros()).rev() {
            acc = self.cantor(acc, acc);
            if (k >> i) & 1 == 1 {
                acc = self.cantor(acc, d);
            }
        }
        acc
    }

    /// `[k]D`, right-to-left: the independent path of the certificates.
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

    /// A 64-bit fingerprint of the divisor (not an injection).
    pub fn key_of(&self, d: &JDiv) -> u64 {
        let mut h = 0x243F_6A88_85A3_08D3u64 ^ u64::from(d.deg);
        for i in 0..d.deg as usize {
            h = mix64(h ^ (u64::from(d.u[i]) << 32 | u64::from(d.v[i])));
        }
        h
    }
}

pub fn is_prime(p: u64) -> bool {
    if p < 2 {
        return false;
    }
    let mut d = 2u64;
    while d * d <= p {
        if p % d == 0 {
            return false;
        }
        d += 1;
    }
    true
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GroupOps {
    pub adds: u64,
    pub doubles: u64,
    pub scalar_mults: u64,
}

impl JCurve {
    pub fn add(&self, ops: &mut GroupOps, a: JDiv, b: JDiv) -> JDiv {
        if a == b {
            ops.doubles += 1;
        } else {
            ops.adds += 1;
        }
        self.cantor(a, b)
    }
    pub fn double(&self, ops: &mut GroupOps, a: JDiv) -> JDiv {
        ops.doubles += 1;
        self.cantor(a, a)
    }
    pub fn neg(&self, a: JDiv) -> JDiv {
        self.negate(a)
    }
    pub fn key(&self, d: &JDiv) -> u64 {
        self.key_of(d)
    }
    /// `[k]P` by double-and-add, counted exactly.
    pub fn smul(&self, ops: &mut GroupOps, p: JDiv, k: u64) -> JDiv {
        ops.scalar_mults += 1;
        if k == 0 || p.is_identity() {
            return JDiv::IDENTITY;
        }
        let bits = 64 - k.leading_zeros();
        let mut acc = p;
        for i in (0..bits - 1).rev() {
            acc = self.double(ops, acc);
            if (k >> i) & 1 == 1 {
                acc = self.add(ops, acc, p);
            }
        }
        acc
    }
}

#[derive(Clone, Debug)]
pub struct Instance {
    pub curve: JCurve,
    pub p: u64,
    pub genus: u32,
    pub order: u64,
    pub h: u64,
    pub r: u64,
    pub g: JDiv,
    pub t_secret: u64,
    pub target: JDiv,
}

pub fn instance_from(
    curve: JCurve,
    order: u64,
    h: u64,
    r: u64,
    target_seed: u64,
) -> Result<Instance, String> {
    let mut st = target_seed ^ 0x1234_5678_9ABC_DEF0;
    let mut g = JDiv::IDENTITY;
    for _ in 0..64 {
        let d = curve.random_divisor(&mut st);
        let cand = curve.mul_ltr(d, h);
        if !cand.is_identity() && curve.mul_ltr(cand, r).is_identity() {
            g = cand;
            break;
        }
    }
    if g.is_identity() {
        return Err("no generator of order r".into());
    }
    st = mix64(st);
    let t_secret = 1 + mix64(st) % (r - 1);
    let target = curve.mul_rtl(g, t_secret);
    Ok(Instance {
        p: curve.p(),
        genus: curve.genus(),
        curve,
        order,
        h,
        r,
        g,
        t_secret,
        target,
    })
}
