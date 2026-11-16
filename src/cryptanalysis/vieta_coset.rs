//! Exact two-point decomposition through Vieta remainders and a PKM coset.
//!
//! For R=(t,r), put s=L^2-t and v=t*L^2+2*r*L+A+t^2. Reduction of
//! Z^m modulo Z^2-s*Z+v yields a(L)*Z+b(L). Distinct abscissae in the
//! coset are obtained from gcd(a,b-c^m). Repeated abscissae require the
//! separate discriminant branch; requiring a repeated quadratic to divide
//! the squarefree membership polynomial would incorrectly discard them.
//!
//! No interpolation, summation polynomial, or factor-base enumeration occurs
//! in the finite-target candidate. Dense polynomial arithmetic is quadratic;
//! small input degrees do not imply a small elimination/last-fall degree.
//! This is public-data research code, not constant-time cryptographic code.
//! Prime moduli are a caller precondition, not proved by these constructors.

use num_bigint::BigUint;
use num_traits::Zero;
use serde::Serialize;
use std::cell::Cell;
use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::time::{Duration, Instant};

pub trait PrimeField {
    type E: Clone + Debug + Eq + Ord + Hash;
    fn modulus(&self) -> BigUint;
    fn element(&self, n: u64) -> Self::E;
    fn add(&self, a: &Self::E, b: &Self::E) -> Self::E;
    fn sub(&self, a: &Self::E, b: &Self::E) -> Self::E;
    fn mul(&self, a: &Self::E, b: &Self::E) -> Self::E;
    fn hex(&self, a: &Self::E) -> String;
    fn canonical(&self, a: &Self::E) -> bool;
}

/// Canonical u64 coefficients; uses u128 reduction (not Montgomery).
#[derive(Clone, Debug)]
pub struct WordField(pub u64);
impl PrimeField for WordField {
    type E = u64;
    fn modulus(&self) -> BigUint {
        self.0.into()
    }
    fn element(&self, n: u64) -> u64 {
        n % self.0
    }
    fn add(&self, a: &u64, b: &u64) -> u64 {
        ((*a as u128 + *b as u128) % self.0 as u128) as u64
    }
    fn sub(&self, a: &u64, b: &u64) -> u64 {
        if a >= b {
            a - b
        } else {
            self.0 - (b - a)
        }
    }
    fn mul(&self, a: &u64, b: &u64) -> u64 {
        (*a as u128 * *b as u128 % self.0 as u128) as u64
    }
    fn hex(&self, a: &u64) -> String {
        format!("{a:x}")
    }
    fn canonical(&self, a: &u64) -> bool {
        *a < self.0
    }
}

/// Native Rust arbitrary precision, with the same algorithms as WordField.
#[derive(Clone, Debug)]
pub struct WideField(pub BigUint);
impl PrimeField for WideField {
    type E = BigUint;
    fn modulus(&self) -> BigUint {
        self.0.clone()
    }
    fn element(&self, n: u64) -> BigUint {
        BigUint::from(n) % &self.0
    }
    fn add(&self, a: &BigUint, b: &BigUint) -> BigUint {
        (a + b) % &self.0
    }
    fn sub(&self, a: &BigUint, b: &BigUint) -> BigUint {
        if a >= b {
            a - b
        } else {
            &self.0 - (b - a)
        }
    }
    fn mul(&self, a: &BigUint, b: &BigUint) -> BigUint {
        a * b % &self.0
    }
    fn hex(&self, a: &BigUint) -> String {
        a.to_str_radix(16)
    }
    fn canonical(&self, a: &BigUint) -> bool {
        a < &self.0
    }
}

/// Primitive totals include arithmetic inside inversion/sqrt. Do not add the
/// high-level call counters to their nested primitive totals when pricing.
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct Work {
    pub adds: u64,
    pub muls: u64,
    pub inversions: u64,
    pub square_roots: u64,
    pub polynomial_products: u64,
    pub polynomial_divisions: u64,
    pub split_attempts: u64,
    pub group_adds: u64,
    pub membership_lookups: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_muls: u64,
    pub max_split_attempts: u64,
    pub timeout: Duration,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            max_muls: 1_000_000_000,
            max_split_attempts: 4096,
            timeout: Duration::from_secs(120),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Invalid(&'static str),
    Budget(&'static str),
    Inconsistent(&'static str),
}
pub type Result<T> = std::result::Result<T, Error>;
pub type Point<E> = Option<(E, E)>;
pub type Relation<E> = ((E, E), (E, E));
type Poly<E> = Vec<E>;

/// One independently budgeted stage. Operation limits are checked at loop
/// boundaries, including each dense multiplication row and division step.
pub struct Arithmetic<F: PrimeField> {
    pub field: F,
    work: Cell<Work>,
    limits: Limits,
    started: Instant,
}
impl<F: PrimeField> Arithmetic<F> {
    pub fn new(field: F, limits: Limits) -> Result<Self> {
        let p = field.modulus();
        if p <= BigUint::from(3u8) || !p.bit(0) {
            return Err(Error::Invalid("need an odd prime >3"));
        }
        Ok(Self {
            field,
            work: Cell::new(Work::default()),
            limits,
            started: Instant::now(),
        })
    }
    pub fn work(&self) -> Work {
        self.work.get()
    }
    fn count(&self, f: impl FnOnce(&mut Work)) {
        let mut w = self.work.get();
        f(&mut w);
        self.work.set(w);
    }
    pub fn check(&self) -> Result<()> {
        if self.work.get().muls > self.limits.max_muls {
            return Err(Error::Budget("field multiplication limit"));
        }
        if self.work.get().split_attempts > self.limits.max_split_attempts {
            return Err(Error::Budget("root split limit"));
        }
        if self.started.elapsed() > self.limits.timeout {
            return Err(Error::Budget("stage deadline"));
        }
        Ok(())
    }
    fn n(&self, n: u64) -> F::E {
        self.field.element(n)
    }
    fn zero(&self) -> F::E {
        self.n(0)
    }
    fn one(&self) -> F::E {
        self.n(1)
    }
    fn add(&self, a: &F::E, b: &F::E) -> F::E {
        self.count(|w| w.adds += 1);
        self.field.add(a, b)
    }
    fn sub(&self, a: &F::E, b: &F::E) -> F::E {
        self.count(|w| w.adds += 1);
        self.field.sub(a, b)
    }
    fn neg(&self, a: &F::E) -> F::E {
        self.sub(&self.zero(), a)
    }
    fn mul(&self, a: &F::E, b: &F::E) -> F::E {
        self.count(|w| w.muls += 1);
        self.field.mul(a, b)
    }
    fn pow(&self, a: &F::E, e: &BigUint) -> Result<F::E> {
        let mut r = self.one();
        for i in (0..e.bits()).rev() {
            self.check()?;
            r = self.mul(&r, &r);
            if e.bit(i) {
                r = self.mul(&r, a);
            }
        }
        Ok(r)
    }
    fn inv(&self, a: &F::E) -> Result<F::E> {
        if *a == self.zero() {
            return Err(Error::Invalid("inverse of zero"));
        }
        self.count(|w| w.inversions += 1);
        self.pow(a, &(self.field.modulus() - 2u8))
    }
    fn sqrt(&self, a: &F::E) -> Result<Option<F::E>> {
        self.count(|w| w.square_roots += 1);
        if *a == self.zero() {
            return Ok(Some(self.zero()));
        }
        let p = self.field.modulus();
        if self.pow(a, &((&p - 1u8) >> 1))? != self.one() {
            return Ok(None);
        }
        let mut q = &p - 1u8;
        let mut s = 0;
        while !q.bit(0) {
            q >>= 1;
            s += 1;
        }
        if s == 1 {
            return Ok(Some(self.pow(a, &((&p + 1u8) >> 2))?));
        }
        let mut z = self.n(2);
        while self.pow(&z, &((&p - 1u8) >> 1))? == self.one() {
            self.check()?;
            z = self.add(&z, &self.one());
        }
        let mut c = self.pow(&z, &q)?;
        let mut x = self.pow(a, &((&q + 1u8) >> 1))?;
        let mut t = self.pow(a, &q)?;
        let mut m = s;
        while t != self.one() {
            self.check()?;
            let mut tt = t.clone();
            let mut i = 0;
            while tt != self.one() && i < m {
                tt = self.mul(&tt, &tt);
                i += 1;
            }
            if i == m {
                return Err(Error::Inconsistent("Tonelli-Shanks needs a prime modulus"));
            }
            let mut b = c.clone();
            for _ in 0..m - i - 1 {
                b = self.mul(&b, &b);
            }
            x = self.mul(&x, &b);
            c = self.mul(&b, &b);
            t = self.mul(&t, &c);
            m = i;
        }
        Ok(Some(x))
    }
    fn trim(&self, mut a: Poly<F::E>) -> Poly<F::E> {
        while a.len() > 1 && a.last() == Some(&self.zero()) {
            a.pop();
        }
        if a.is_empty() {
            a.push(self.zero());
        }
        a
    }
    fn is_zero(&self, a: &[F::E]) -> bool {
        a.len() == 1 && a[0] == self.zero()
    }
    fn padd(&self, a: &[F::E], b: &[F::E]) -> Poly<F::E> {
        let mut r = vec![self.zero(); a.len().max(b.len())];
        for (i, x) in a.iter().enumerate() {
            r[i] = self.add(&r[i], x);
        }
        for (i, x) in b.iter().enumerate() {
            r[i] = self.add(&r[i], x);
        }
        self.trim(r)
    }
    fn pscale(&self, a: &[F::E], c: &F::E) -> Poly<F::E> {
        self.trim(a.iter().map(|x| self.mul(x, c)).collect())
    }
    fn psub(&self, a: &[F::E], b: &[F::E]) -> Poly<F::E> {
        self.padd(a, &self.pscale(b, &self.neg(&self.one())))
    }
    fn pmul(&self, a: &[F::E], b: &[F::E]) -> Result<Poly<F::E>> {
        self.count(|w| w.polynomial_products += 1);
        let mut r = vec![self.zero(); a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            self.check()?;
            if *x == self.zero() {
                continue;
            }
            for (j, y) in b.iter().enumerate() {
                if *y != self.zero() {
                    r[i + j] = self.add(&r[i + j], &self.mul(x, y));
                }
            }
        }
        Ok(self.trim(r))
    }
    fn divrem(&self, a: &[F::E], b: &[F::E]) -> Result<(Poly<F::E>, Poly<F::E>)> {
        self.count(|w| w.polynomial_divisions += 1);
        if self.is_zero(b) {
            return Err(Error::Invalid("zero polynomial divisor"));
        }
        let mut r = self.trim(a.to_vec());
        if r.len() < b.len() {
            return Ok((vec![self.zero()], r));
        }
        let mut q = vec![self.zero(); r.len() - b.len() + 1];
        let inv = self.inv(b.last().unwrap())?;
        while !self.is_zero(&r) && r.len() >= b.len() {
            self.check()?;
            let k = r.len() - b.len();
            let c = self.mul(r.last().unwrap(), &inv);
            q[k] = c.clone();
            for (i, v) in b.iter().enumerate() {
                r[i + k] = self.sub(&r[i + k], &self.mul(&c, v));
            }
            r = self.trim(r);
        }
        Ok((self.trim(q), r))
    }
    fn gcd(&self, a: &[F::E], b: &[F::E]) -> Result<Poly<F::E>> {
        let (mut a, mut b) = (self.trim(a.to_vec()), self.trim(b.to_vec()));
        while !self.is_zero(&b) {
            let r = self.divrem(&a, &b)?.1;
            a = b;
            b = r;
        }
        if self.is_zero(&a) {
            return Ok(a);
        }
        Ok(self.pscale(&a, &self.inv(a.last().unwrap())?))
    }
    fn powmod(&self, a: &[F::E], e: &BigUint, f: &[F::E]) -> Result<Poly<F::E>> {
        let a = self.divrem(a, f)?.1;
        let mut r = vec![self.one()];
        for i in (0..e.bits()).rev() {
            r = self.divrem(&self.pmul(&r, &r)?, f)?.1;
            if e.bit(i) {
                r = self.divrem(&self.pmul(&r, &a)?, f)?.1;
            }
        }
        Ok(r)
    }
    fn eval(&self, a: &[F::E], x: &F::E) -> F::E {
        a.iter()
            .rev()
            .fold(self.zero(), |v, c| self.add(&self.mul(&v, x), c))
    }
    fn roots(&self, a: &[F::E], seed: &mut u64) -> Result<Vec<F::E>> {
        if a.len() == 1 {
            if self.is_zero(a) {
                return Err(Error::Inconsistent("unconstrained root polynomial"));
            }
            return Ok(vec![]);
        }
        let xp = self.powmod(&[self.zero(), self.one()], &self.field.modulus(), a)?;
        let f = self.gcd(a, &self.psub(&xp, &[self.zero(), self.one()]))?;
        let mut pending = vec![f];
        let mut out = vec![];
        while let Some(g) = pending.pop() {
            self.check()?;
            if g.len() == 1 {
                continue;
            }
            if g.len() == 2 {
                out.push(self.mul(&self.neg(&g[0]), &self.inv(&g[1])?));
                continue;
            }
            loop {
                self.count(|w| w.split_attempts += 1);
                self.check()?;
                let c = self.random_element(seed);
                let w = self.powmod(&[c, self.one()], &((self.field.modulus() - 1u8) >> 1), &g)?;
                let h = self.gcd(&g, &self.psub(&w, &[self.one()]))?;
                if h.len() > 1 && h.len() < g.len() {
                    pending.push(self.divrem(&g, &h)?.0);
                    pending.push(h);
                    break;
                }
            }
        }
        out.sort();
        out.dedup();
        Ok(out)
    }
    pub fn random_element(&self, seed: &mut u64) -> F::E {
        let mut v = self.zero();
        for _ in 0..self.field.modulus().bits().div_ceil(32) {
            *seed = seed.wrapping_add(0x9e3779b97f4a7c15);
            let mut z = *seed;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
            z ^= z >> 31;
            v = self.add(&self.mul(&v, &self.n(1u64 << 32)), &self.n(z >> 32));
        }
        v
    }
}

#[derive(Clone, Debug)]
pub struct Curve<E> {
    pub a: E,
    pub b: E,
}
impl<F: PrimeField> Arithmetic<F> {
    pub fn validate_curve(&self, e: &Curve<F::E>) -> Result<()> {
        if !self.field.canonical(&e.a) || !self.field.canonical(&e.b) {
            return Err(Error::Invalid("noncanonical curve coefficients"));
        }
        let disc = self.add(
            &self.mul(&self.n(4), &self.mul(&self.mul(&e.a, &e.a), &e.a)),
            &self.mul(&self.n(27), &self.mul(&e.b, &e.b)),
        );
        if disc == self.zero() {
            return Err(Error::Invalid("singular curve"));
        }
        Ok(())
    }
    pub fn on_curve(&self, e: &Curve<F::E>, p: &Point<F::E>) -> bool {
        p.as_ref().is_none_or(|(x, y)| {
            self.field.canonical(x)
                && self.field.canonical(y)
                && self.mul(y, y)
                    == self.add(
                        &self.add(&self.mul(&self.mul(x, x), x), &self.mul(&e.a, x)),
                        &e.b,
                    )
        })
    }
    pub fn negate(&self, p: &Point<F::E>) -> Point<F::E> {
        p.as_ref().map(|(x, y)| (x.clone(), self.neg(y)))
    }
    pub fn group_add(
        &self,
        e: &Curve<F::E>,
        p: &Point<F::E>,
        q: &Point<F::E>,
    ) -> Result<Point<F::E>> {
        self.count(|w| w.group_adds += 1);
        self.check()?;
        let (Some((x, y)), Some((u, v))) = (p, q) else {
            return Ok(if p.is_none() { q.clone() } else { p.clone() });
        };
        if x == u && self.add(y, v) == self.zero() {
            return Ok(None);
        }
        let (n, d) = if x == u {
            (
                self.add(&self.mul(&self.n(3), &self.mul(x, x)), &e.a),
                self.mul(&self.n(2), y),
            )
        } else {
            (self.sub(v, y), self.sub(u, x))
        };
        let l = self.mul(&n, &self.inv(&d)?);
        Ok(Some(self.with_slope(x, y, u, &l)))
    }
    fn with_slope(&self, x: &F::E, y: &F::E, u: &F::E, l: &F::E) -> (F::E, F::E) {
        let z = self.sub(&self.sub(&self.mul(l, l), x), u);
        let w = self.sub(&self.mul(l, &self.sub(x, &z)), y);
        (z, w)
    }
    pub fn random_point(&self, e: &Curve<F::E>, seed: &mut u64) -> Result<(F::E, F::E)> {
        loop {
            self.check()?;
            let x = self.random_element(seed);
            let rhs = self.add(
                &self.add(&self.mul(&self.mul(&x, &x), &x), &self.mul(&e.a, &x)),
                &e.b,
            );
            if let Some(y) = self.sqrt(&rhs)? {
                return Ok((x, y));
            }
        }
    }
}

/// x lies in c*mu_m; power-of-two m makes every membership gate quadratic.
/// m is bounded explicitly to keep an accidental request from allocating an
/// unbounded dense polynomial. Larger budgets require an explicit new build.
#[derive(Clone, Debug)]
pub struct Coset<E> {
    m: usize,
    c: E,
    cm: E,
    modulus: BigUint,
}
impl<E> Coset<E> {
    pub fn size(&self) -> usize {
        self.m
    }
    pub fn representative(&self) -> &E {
        &self.c
    }
}
impl<F: PrimeField> Arithmetic<F> {
    pub fn coset(&self, m: usize, c: F::E) -> Result<Coset<F::E>> {
        if !m.is_power_of_two() || m > 16384 {
            return Err(Error::Invalid("m must be a power of two <=16384"));
        }
        if !self.field.canonical(&c)
            || c == self.zero()
            || (self.field.modulus() - 1u8) % m != BigUint::zero()
        {
            return Err(Error::Invalid(
                "nonzero canonical coset and m | p-1 required",
            ));
        }
        let cm = self.pow(&c, &BigUint::from(m))?;
        Ok(Coset {
            m,
            c,
            cm,
            modulus: self.field.modulus(),
        })
    }
    pub fn factor_base(&self, e: &Curve<F::E>, coset: &Coset<F::E>) -> Result<Vec<(F::E, F::E)>> {
        self.validate_curve(e)?;
        if coset.modulus != self.field.modulus() {
            return Err(Error::Invalid("coset field mismatch"));
        }
        let exp = (self.field.modulus() - 1u8) / coset.m;
        let mut omega = self.one();
        if coset.m > 1 {
            let mut h = self.n(2);
            loop {
                self.check()?;
                omega = self.pow(&h, &exp)?;
                if self.pow(&omega, &BigUint::from(coset.m / 2))? != self.one() {
                    break;
                }
                h = self.add(&h, &self.one());
            }
        }
        let mut x = coset.c.clone();
        let mut out = vec![];
        for _ in 0..coset.m {
            self.check()?;
            let rhs = self.add(
                &self.add(&self.mul(&self.mul(&x, &x), &x), &self.mul(&e.a, &x)),
                &e.b,
            );
            if let Some(y) = self.sqrt(&rhs)? {
                out.push((x.clone(), y.clone()));
                if y != self.zero() {
                    out.push((x.clone(), self.neg(&y)));
                }
            }
            x = self.mul(&x, &omega);
        }
        out.sort();
        Ok(out)
    }

    /// Complete ordered finite-point pairs. Identity target uses the explicit
    /// P,-P branch and enumerates the factor base; it is reported separately.
    pub fn solve(
        &self,
        e: &Curve<F::E>,
        coset: &Coset<F::E>,
        target: &Point<F::E>,
        seed: u64,
    ) -> Result<Vec<Relation<F::E>>> {
        self.validate_curve(e)?;
        if coset.modulus != self.field.modulus() {
            return Err(Error::Invalid("coset field mismatch"));
        }
        if !self.on_curve(e, target) {
            return Err(Error::Invalid("target off curve"));
        }
        let Some((t, r)) = target else {
            return Ok(self
                .factor_base(e, coset)?
                .into_iter()
                .map(|p| {
                    let q = (p.0.clone(), self.neg(&p.1));
                    (p, q)
                })
                .collect());
        };
        let s = vec![self.neg(t), self.zero(), self.one()];
        let v = vec![
            self.add(&e.a, &self.mul(t, t)),
            self.mul(&self.n(2), r),
            t.clone(),
        ];
        // Binary composed powering in the quadratic algebra. Each square
        // sends (a,b) to (a^2*s+2ab,b^2-a^2*v).
        let (mut a, mut b) = (vec![self.zero()], vec![self.one()]);
        let (mut u, mut w) = (vec![self.one()], vec![self.zero()]);
        let mut power = coset.m;
        while power > 0 {
            if power & 1 == 1 {
                (a, b) = self.qmul((&a, &b), (&u, &w), &s, &v)?;
            }
            power >>= 1;
            if power > 0 {
                (u, w) = self.qmul((&u, &w), (&u, &w), &s, &v)?;
            }
        }
        let g = self.gcd(&a, &self.psub(&b, std::slice::from_ref(&coset.cm)))?;
        let delta = self.psub(&self.pmul(&s, &s)?, &self.pscale(&v, &self.n(4)));
        let inv2 = self.inv(&self.n(2))?;
        let half_s = self.pscale(&s, &inv2);
        let diagonal = self.powmod(&half_s, &BigUint::from(coset.m), &delta)?;
        let diagonal = self.gcd(
            &delta,
            &self.psub(&diagonal, std::slice::from_ref(&coset.cm)),
        )?;
        let mut rng = seed;
        let mut out = BTreeSet::new();
        for (f, repeated) in [(&g, false), (&diagonal, true)] {
            for l in self.roots(f, &mut rng)? {
                let sum = self.eval(&s, &l);
                let disc = self.eval(&delta, &l);
                if repeated != (disc == self.zero()) {
                    continue;
                }
                let Some(root) = self.sqrt(&disc)? else {
                    return Err(Error::Inconsistent(
                        "split coset gave nonsquare discriminant",
                    ));
                };
                let x = self.mul(&self.add(&sum, &root), &inv2);
                let y = self.sub(&sum, &x);
                let p = (x.clone(), self.sub(&self.mul(&l, &self.sub(&x, t)), r));
                let q = (y.clone(), self.sub(&self.mul(&l, &self.sub(&y, t)), r));
                // Returned witnesses are checked by the group law too. Tests
                // compare completeness against enumeration independently.
                if self.pow(&x, &BigUint::from(coset.m))? != coset.cm
                    || self.pow(&y, &BigUint::from(coset.m))? != coset.cm
                    || !self.on_curve(e, &Some(p.clone()))
                    || !self.on_curve(e, &Some(q.clone()))
                    || self.group_add(e, &Some(p.clone()), &Some(q.clone()))? != *target
                {
                    return Err(Error::Inconsistent("invalid extracted witness"));
                }
                out.insert((p.clone(), q.clone()));
                out.insert((q, p));
            }
        }
        self.check()?;
        Ok(out.into_iter().collect())
    }
    fn qmul(
        &self,
        (a, b): (&[F::E], &[F::E]),
        (d, e): (&[F::E], &[F::E]),
        s: &[F::E],
        v: &[F::E],
    ) -> Result<(Poly<F::E>, Poly<F::E>)> {
        let ad = self.pmul(a, d)?;
        let linear = self.padd(
            &self.padd(&self.pmul(&ad, s)?, &self.pmul(a, e)?),
            &self.pmul(b, d)?,
        );
        let constant = self.psub(&self.pmul(b, e)?, &self.pmul(&ad, v)?);
        Ok((linear, constant))
    }

    /// Independent scalar subtraction control (complete ordered pairs).
    pub fn lookup_scalar(
        &self,
        e: &Curve<F::E>,
        base: &[(F::E, F::E)],
        target: &Point<F::E>,
    ) -> Result<Vec<Relation<F::E>>> {
        let set: HashSet<_> = base.iter().cloned().collect();
        let mut out = vec![];
        for p in base {
            let q = self.group_add(e, target, &self.negate(&Some(p.clone())))?;
            self.count(|w| w.membership_lookups += 1);
            if let Some(q) = q {
                if set.contains(&q) {
                    out.push((p.clone(), q));
                }
            }
        }
        out.sort();
        Ok(out)
    }

    /// Montgomery's simultaneous-inversion lookup: one inverse for all
    /// ordinary R-P lanes, exceptional x matches handled by group_add.
    /// This is a matched generic control; the repository's Montgomery u64
    /// engine must additionally be measured before any fastest-backend claim.
    pub fn lookup_batched(
        &self,
        e: &Curve<F::E>,
        base: &[(F::E, F::E)],
        set: &HashSet<(F::E, F::E)>,
        target: &Point<F::E>,
    ) -> Result<Vec<Relation<F::E>>> {
        let Some((x, y)) = target else {
            let mut out = vec![];
            for p in base {
                self.check()?;
                let q = (p.0.clone(), self.neg(&p.1));
                self.count(|w| w.membership_lookups += 1);
                if set.contains(&q) {
                    out.push((p.clone(), q));
                }
            }
            out.sort();
            return Ok(out);
        };
        let mut prefix = vec![];
        let mut lanes = vec![];
        let mut product = self.one();
        let mut out = vec![];
        for (i, (u, _)) in base.iter().enumerate() {
            self.check()?;
            if x != u {
                prefix.push(product.clone());
                let d = self.sub(u, x);
                product = self.mul(&product, &d);
                lanes.push(i);
            } else {
                if let Some(q) = self.group_add(e, target, &self.negate(&Some(base[i].clone())))? {
                    self.count(|w| w.membership_lookups += 1);
                    if set.contains(&q) {
                        out.push((base[i].clone(), q));
                    }
                }
            }
        }
        if !lanes.is_empty() {
            let mut inv = self.inv(&product)?;
            for (&i, pre) in lanes.iter().zip(prefix.iter()).rev() {
                self.check()?;
                self.count(|w| w.group_adds += 1);
                let (u, v) = &base[i];
                let d = self.sub(u, x);
                let di = self.mul(&inv, pre);
                inv = self.mul(&inv, &d);
                let l = self.mul(&self.sub(&self.neg(v), y), &di);
                let q = self.with_slope(x, y, u, &l);
                self.count(|w| w.membership_lookups += 1);
                if set.contains(&q) {
                    out.push((base[i].clone(), q));
                }
            }
        }
        out.sort();
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::One;
    fn context(p: u64) -> Arithmetic<WordField> {
        Arithmetic::new(WordField(p), Limits::default()).unwrap()
    }
    #[test]
    fn exhaustive_all_targets_including_two_torsion_diagonals_and_identity() {
        for (p, a, b) in [(17, 2, 2), (97, 2, 3), (257, 1, 1)] {
            let f = context(p);
            let e = Curve { a, b };
            let mut targets = vec![None];
            for x in 0..p {
                for y in 0..p {
                    let q = Some((x, y));
                    if f.on_curve(&e, &q) {
                        targets.push(q);
                    }
                }
            }
            for m in [1, 2, 4, 8, 16] {
                if (p - 1) % m != 0 {
                    continue;
                }
                for c in [1, 2] {
                    let coset = f.coset(m as usize, c).unwrap();
                    let base = f.factor_base(&e, &coset).unwrap();
                    let set = base.iter().cloned().collect();
                    for target in &targets {
                        // New stage per query keeps limits independent.
                        let f = context(p);
                        let expected = f.lookup_scalar(&e, &base, target).unwrap();
                        let batched = f.lookup_batched(&e, &base, &set, target).unwrap();
                        let actual = f.solve(&e, &coset, target, 42).unwrap();
                        assert_eq!(actual, expected, "p={p} m={m} c={c} target={target:?}");
                        assert_eq!(batched, expected);
                    }
                }
            }
        }
    }
    #[test]
    fn word_and_wide_agree() {
        let f = context(65537);
        let e = Curve { a: 2, b: 3 };
        let c = f.coset(32, 3).unwrap();
        let base = f.factor_base(&e, &c).unwrap();
        let wide = Arithmetic::new(WideField(65537u32.into()), Limits::default()).unwrap();
        let ew = Curve {
            a: 2u8.into(),
            b: 3u8.into(),
        };
        let cw = wide.coset(32, 3u8.into()).unwrap();
        let bw = wide.factor_base(&ew, &cw).unwrap();
        assert_eq!(
            bw,
            base.iter()
                .map(|(x, y)| ((*x).into(), (*y).into()))
                .collect::<Vec<_>>()
        );
        for p in base.iter().take(8) {
            let t = f.group_add(&e, &Some(*p), &Some(base[0])).unwrap();
            let tw = t.map(|(x, y)| (x.into(), y.into()));
            let a = f.solve(&e, &c, &t, 1).unwrap();
            let b = wide.solve(&ew, &cw, &tw, 1).unwrap();
            assert_eq!(
                b,
                a.iter()
                    .map(|((x, y), (u, v))| (
                        ((*x).into(), (*y).into()),
                        ((*u).into(), (*v).into())
                    ))
                    .collect::<Vec<_>>()
            );
        }
    }
    #[test]
    fn inputs_and_budgets_fail_explicitly() {
        let small = context(17);
        let partial = vec![(5, 1)];
        let set = partial.iter().cloned().collect();
        assert!(small
            .lookup_batched(&Curve { a: 2, b: 2 }, &partial, &set, &None)
            .unwrap()
            .is_empty());
        let f = context(97);
        assert!(f.coset(3, 1).is_err());
        assert!(f.coset(64, 1).is_err());
        assert!(f.coset(4, 0).is_err());
        assert!(f.coset(4, 97).is_err());
        let c = f.coset(4, 1).unwrap();
        assert!(f.solve(&Curve { a: 0, b: 0 }, &c, &None, 0).is_err());
        assert!(f
            .solve(&Curve { a: 2, b: 3 }, &c, &Some((0, 0)), 0)
            .is_err());
        let b = Arithmetic::new(
            WordField(97),
            Limits {
                max_muls: 0,
                ..Limits::default()
            },
        )
        .unwrap();
        assert!(matches!(
            b.factor_base(&Curve { a: 2, b: 3 }, &c),
            Err(Error::Budget(_))
        ));
        assert!(context(257).factor_base(&Curve { a: 2, b: 3 }, &c).is_err());
        let f = Arithmetic::new(
            WordField(97),
            Limits {
                max_split_attempts: 0,
                ..Limits::default()
            },
        )
        .unwrap();
        assert!(matches!(
            f.roots(&[96, 0, 1], &mut 1),
            Err(Error::Budget("root split limit"))
        ));
    }

    #[test]
    fn larger_cosets_correctness_only() {
        // Development checks, no timing comparison. Independent scalar and
        // simultaneous-inversion controls must agree on the complete output.
        for (p, m) in [(2013265921, 1024), (4179340454199820289, 4096)] {
            let f = context(p);
            let e = Curve { a: 2, b: 3 };
            let c = f.coset(m, 1).unwrap();
            let base = f.factor_base(&e, &c).unwrap();
            let set = base.iter().cloned().collect();
            let mut seed = 19;
            for target in [
                f.group_add(&e, &Some(base[0]), &Some(base[1])).unwrap(),
                Some(f.random_point(&e, &mut seed).unwrap()),
            ] {
                let a = f.solve(&e, &c, &target, 37).unwrap();
                assert_eq!(a, f.lookup_scalar(&e, &base, &target).unwrap());
                assert_eq!(a, f.lookup_batched(&e, &base, &set, &target).unwrap());
            }
        }
        let p = (BigUint::one() << 224usize) - (BigUint::one() << 96usize) + 1u8;
        let f = Arithmetic::new(WideField(p.clone()), Limits::default()).unwrap();
        let b = BigUint::parse_bytes(
            b"b4050a850c04b3abf54132565044b0b7d7bfd8ba270b39432355ffb4",
            16,
        )
        .unwrap();
        let e = Curve { a: p - 3u8, b };
        let c = f.coset(1024, 1u8.into()).unwrap();
        let base = f.factor_base(&e, &c).unwrap();
        let set = base.iter().cloned().collect();
        let target = f
            .group_add(&e, &Some(base[0].clone()), &Some(base[1].clone()))
            .unwrap();
        let a = f.solve(&e, &c, &target, 38).unwrap();
        assert!(!a.is_empty());
        assert_eq!(a, f.lookup_scalar(&e, &base, &target).unwrap());
        assert_eq!(a, f.lookup_batched(&e, &base, &set, &target).unwrap());
    }
    #[test]
    fn p224_known_answer_wide_field() {
        let p = (BigUint::one() << 224usize) - (BigUint::one() << 96usize) + 1u8;
        let f = Arithmetic::new(WideField(p.clone()), Limits::default()).unwrap();
        let b = BigUint::parse_bytes(
            b"b4050a850c04b3abf54132565044b0b7d7bfd8ba270b39432355ffb4",
            16,
        )
        .unwrap();
        let e = Curve { a: p - 3u8, b };
        let c = f.coset(16, 1u8.into()).unwrap();
        let base = f.factor_base(&e, &c).unwrap();
        assert!(base.len() > 2);
        for i in 0..4 {
            let t = f
                .group_add(
                    &e,
                    &Some(base[i].clone()),
                    &Some(base[(i + 1) % base.len()].clone()),
                )
                .unwrap();
            assert_eq!(
                f.solve(&e, &c, &t, 21).unwrap(),
                f.lookup_scalar(&e, &base, &t).unwrap()
            );
        }
    }
}
