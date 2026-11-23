//! Free-algebra arithmetic. No division by a polynomial value or unknown factor.
use crate::Result;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde_json::{json, Value};
use std::cell::Cell;
use torus_s4::{curve25519_prime, Field};
use torus_s4_tree::Fp2;
pub type P = Vec<Fp2>;
pub fn z() -> Fp2 {
    Fp2 {
        a: 0u32.into(),
        b: 0u32.into(),
    }
}
pub fn c(n: u64) -> Fp2 {
    Fp2 {
        a: n.into(),
        b: 0u32.into(),
    }
}
pub fn is_zero(x: &Fp2) -> bool {
    x.a.is_zero() && x.b.is_zero()
}
pub fn trim(mut a: P) -> P {
    while a.last().is_some_and(is_zero) {
        a.pop();
    }
    a
}
pub fn encoded(a: &[Fp2]) -> Value {
    json!(a
        .iter()
        .map(|x| [x.a.to_string(), x.b.to_string()])
        .collect::<Vec<_>>())
}

pub struct Engine {
    pub f: Field,
    work: Cell<u64>,
    slots: Cell<u64>,
    cap: u64,
}
impl Engine {
    pub fn new(p: BigUint, cap: u64) -> Result<Self> {
        if p != BigUint::from(101u32) && p != curve25519_prime() {
            return Err("unsupported modulus".into());
        }
        Ok(Self {
            f: Field::new(p)?,
            work: Cell::new(0),
            slots: Cell::new(0),
            cap,
        })
    }
    pub fn stats(&self) -> Value {
        json!({"fp2_multiplications":self.work.get(),"cumulative_reserved_coefficient_slots":self.slots.get(),"scope":"partial new arithmetic only; not total cold work or RSS"})
    }
    pub fn reserve(&self, n: usize) -> Result<()> {
        let next = self
            .slots
            .get()
            .checked_add(n as u64)
            .ok_or("reservation overflow")?;
        if next > 200_000_000 {
            return Err("cumulative reservation cap; incomplete".into());
        }
        self.slots.set(next);
        Ok(())
    }
    pub fn add(&self, a: &Fp2, b: &Fp2) -> Fp2 {
        Fp2 {
            a: self.f.add(&a.a, &b.a),
            b: self.f.add(&a.b, &b.b),
        }
    }
    pub fn sub(&self, a: &Fp2, b: &Fp2) -> Fp2 {
        Fp2 {
            a: self.f.sub(&a.a, &b.a),
            b: self.f.sub(&a.b, &b.b),
        }
    }
    pub fn neg(&self, a: &Fp2) -> Fp2 {
        self.sub(&z(), a)
    }
    pub fn conjugate(&self, a: &Fp2) -> Fp2 {
        Fp2 {
            a: a.a.clone(),
            b: self.f.sub(&0u32.into(), &a.b),
        }
    }
    pub fn mul(&self, a: &Fp2, b: &Fp2) -> Result<Fp2> {
        let next = self.work.get().checked_add(1).ok_or("work overflow")?;
        if next > self.cap {
            return Err("scalar work cap; incomplete".into());
        }
        self.work.set(next);
        Ok(a.mul(b, &self.f))
    }
    pub fn inv(&self, a: &Fp2) -> Result<Fp2> {
        a.inverse(&self.f)
    }
    pub fn pow(&self, a: &Fp2, n: &BigUint) -> Result<Fp2> {
        let mut r = Fp2::one();
        let mut x = a.clone();
        for i in 0..n.bits() {
            if n.bit(i) {
                r = self.mul(&r, &x)?;
            }
            if i + 1 < n.bits() {
                x = self.mul(&x, &x)?;
            }
        }
        Ok(r)
    }
    pub fn padd(&self, a: &[Fp2], b: &[Fp2]) -> P {
        (0..a.len().max(b.len()))
            .map(|i| self.add(a.get(i).unwrap_or(&z()), b.get(i).unwrap_or(&z())))
            .collect()
    }
    pub fn psub(&self, a: &[Fp2], b: &[Fp2]) -> P {
        trim(
            (0..a.len().max(b.len()))
                .map(|i| self.sub(a.get(i).unwrap_or(&z()), b.get(i).unwrap_or(&z())))
                .collect(),
        )
    }
    pub fn scale(&self, a: &[Fp2], b: &Fp2) -> Result<P> {
        a.iter().map(|v| self.mul(v, b)).collect()
    }
    pub fn school(&self, a: &[Fp2], b: &[Fp2]) -> Result<P> {
        if a.is_empty() || b.is_empty() {
            return Ok(vec![]);
        }
        self.reserve(a.len() + b.len() - 1)?;
        let mut r = vec![z(); a.len() + b.len() - 1];
        for (i, x) in a.iter().enumerate() {
            for (j, y) in b.iter().enumerate() {
                r[i + j] = self.add(&r[i + j], &self.mul(x, y)?);
            }
        }
        Ok(trim(r))
    }
    fn kara(&self, a: &[Fp2], b: &[Fp2]) -> Result<P> {
        let n = a.len();
        if n <= 8 {
            let mut r = self.school(a, b)?;
            r.resize(2 * n, z());
            return Ok(r);
        }
        let h = n / 2;
        self.reserve(2 * n)?;
        let lo = self.kara(&a[..h], &b[..h])?;
        let hi = self.kara(&a[h..], &b[h..])?;
        let sa = self.padd(&a[..h], &a[h..]);
        let sb = self.padd(&b[..h], &b[h..]);
        let mid = self.kara(&sa, &sb)?;
        let mut r = vec![z(); 2 * n];
        for i in 0..n {
            r[i] = self.add(&r[i], &lo[i]);
            r[i + n] = self.add(&r[i + n], &hi[i]);
            r[i + h] = self.add(&r[i + h], &self.sub(&self.sub(&mid[i], &lo[i]), &hi[i]));
        }
        Ok(r)
    }
    pub fn product(&self, a: &[Fp2], b: &[Fp2]) -> Result<P> {
        if a.is_empty() || b.is_empty() {
            return Ok(vec![]);
        }
        let n = a
            .len()
            .max(b.len())
            .checked_next_power_of_two()
            .ok_or("polynomial length overflow")?;
        if n > 2048 {
            return Err("polynomial scratch cap".into());
        }
        self.reserve(2 * n)?;
        let mut aa = a.to_vec();
        aa.resize(n, z());
        let mut bb = b.to_vec();
        bb.resize(n, z());
        let mut r = self.kara(&aa, &bb)?;
        r.truncate(a.len() + b.len() - 1);
        Ok(trim(r))
    }
    pub fn divrem(&self, a: &[Fp2], b: &[Fp2]) -> Result<(P, P)> {
        let b = trim(b.to_vec());
        if b.is_empty() {
            return Err("zero divisor polynomial".into());
        }
        let mut r = trim(a.to_vec());
        let mut q = vec![z(); r.len().saturating_sub(b.len()) + 1];
        let inverse = self.inv(b.last().unwrap())?;
        while r.len() >= b.len() {
            let i = r.len() - b.len();
            let v = self.mul(r.last().unwrap(), &inverse)?;
            q[i] = v.clone();
            for (j, bj) in b.iter().enumerate() {
                r[i + j] = self.sub(&r[i + j], &self.mul(&v, bj)?);
            }
            r = trim(r);
        }
        Ok((trim(q), r))
    }
    pub fn monic(&self, a: &[Fp2]) -> Result<P> {
        let a = trim(a.to_vec());
        if a.is_empty() {
            return Ok(a);
        }
        self.scale(&a, &self.inv(a.last().unwrap())?)
    }
    pub fn gcd(&self, a: &[Fp2], b: &[Fp2]) -> Result<P> {
        let mut a = trim(a.to_vec());
        let mut b = trim(b.to_vec());
        while !b.is_empty() {
            let r = self.divrem(&a, &b)?.1;
            a = b;
            b = r;
        }
        self.monic(&a)
    }
    pub fn eval(&self, a: &[Fp2], x: &Fp2) -> Result<Fp2> {
        let mut y = z();
        for v in a.iter().rev() {
            y = self.add(&self.mul(&y, x)?, v);
        }
        Ok(y)
    }
}

pub struct Ring<'a> {
    pub e: &'a Engine,
    pub s: P,
    pub d: usize,
    pub tau: P,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alg {
    pub a: P,
    pub b: P,
}
impl<'a> Ring<'a> {
    pub fn new(e: &'a Engine, s: P) -> Result<Self> {
        if !(2..=7).contains(&s.len())
            || s.last() != Some(&Fp2::one())
            || s.iter().any(|x| x.a >= e.f.p || x.b >= e.f.p)
        {
            return Err("noncanonical/nonmonic degree outside1..6".into());
        }
        let d = s.len() - 1;
        Ok(Self {
            e,
            s,
            d,
            tau: vec![z(); d],
        })
    }
    pub fn q(&self, a: &[Fp2]) -> Result<P> {
        let mut r = self.e.divrem(a, &self.s)?.1;
        r.resize(self.d, z());
        Ok(r)
    }
    pub fn constant(&self, x: Fp2) -> P {
        let mut a = vec![z(); self.d];
        a[0] = x;
        a
    }
    pub fn x(&self) -> Result<P> {
        self.q(&[z(), Fp2::one()])
    }
    pub fn add(&self, a: &[Fp2], b: &[Fp2]) -> P {
        self.e.padd(a, b)
    }
    pub fn sub(&self, a: &[Fp2], b: &[Fp2]) -> P {
        let mut r = self.e.psub(a, b);
        r.resize(self.d, z());
        r
    }
    pub fn mul(&self, a: &[Fp2], b: &[Fp2]) -> Result<P> {
        // Monic reduction: no coefficient-dependent inversion.
        let mut r = self.e.school(a, b)?;
        r.resize(2 * self.d - 1, z());
        for i in (self.d..r.len()).rev() {
            let v = r[i].clone();
            for j in 0..self.d {
                r[i - self.d + j] = self.e.sub(&r[i - self.d + j], &self.e.mul(&v, &self.s[j])?);
            }
        }
        r.truncate(self.d);
        Ok(r)
    }
    pub fn scale(&self, a: &[Fp2], x: &Fp2) -> Result<P> {
        self.e.scale(a, x)
    }
    pub fn pow(&self, a: &[Fp2], n: &BigUint) -> Result<P> {
        let mut r = self.constant(Fp2::one());
        let mut x = a.to_vec();
        for i in 0..n.bits() {
            if n.bit(i) {
                r = self.mul(&r, &x)?;
            }
            if i + 1 < n.bits() {
                x = self.mul(&x, &x)?;
            }
        }
        Ok(r)
    }
    pub fn chebyshev(&self, n: u64, x: &[Fp2]) -> Result<P> {
        let mut a = self.constant(Fp2::one());
        let mut b = x.to_vec();
        for bit in (0..64 - n.leading_zeros()).rev() {
            let ab = self.sub(&self.scale(&self.mul(&a, &b)?, &c(2))?, x);
            if (n >> bit) & 1 == 0 {
                b = ab;
                a = self.sub(
                    &self.scale(&self.mul(&a, &a)?, &c(2))?,
                    &self.constant(Fp2::one()),
                );
            } else {
                a = ab;
                b = self.sub(
                    &self.scale(&self.mul(&b, &b)?, &c(2))?,
                    &self.constant(Fp2::one()),
                );
            }
        }
        Ok(a)
    }
    pub fn zero(&self) -> Alg {
        Alg {
            a: vec![z(); self.d],
            b: vec![z(); self.d],
        }
    }
    pub fn one(&self) -> Alg {
        Alg {
            a: self.constant(Fp2::one()),
            b: vec![z(); self.d],
        }
    }
    pub fn v(&self) -> Alg {
        Alg {
            a: vec![z(); self.d],
            b: self.constant(Fp2::one()),
        }
    }
    pub fn aadd(&self, a: &Alg, b: &Alg) -> Alg {
        Alg {
            a: self.add(&a.a, &b.a),
            b: self.add(&a.b, &b.b),
        }
    }
    pub fn asub(&self, a: &Alg, b: &Alg) -> Alg {
        Alg {
            a: self.sub(&a.a, &b.a),
            b: self.sub(&a.b, &b.b),
        }
    }
    pub fn ascale(&self, a: &Alg, b: &Fp2) -> Result<Alg> {
        Ok(Alg {
            a: self.scale(&a.a, b)?,
            b: self.scale(&a.b, b)?,
        })
    }
    pub fn amul(&self, a: &Alg, b: &Alg) -> Result<Alg> {
        let ac = self.mul(&a.a, &b.a)?;
        let bd = self.mul(&a.b, &b.b)?;
        let cross = self.sub(
            &self.sub(
                &self.mul(&self.add(&a.a, &a.b), &self.add(&b.a, &b.b))?,
                &ac,
            ),
            &bd,
        );
        Ok(Alg {
            a: self.sub(&ac, &bd),
            b: self.add(&cross, &self.mul(&self.tau, &bd)?),
        })
    }
    pub fn apow(&self, a: &Alg, n: u64) -> Result<Alg> {
        let mut r = self.one();
        let mut x = a.clone();
        for i in 0..64 - n.leading_zeros() {
            if (n >> i) & 1 != 0 {
                r = self.amul(&r, &x)?;
            }
            if i + 1 < 64 - n.leading_zeros() {
                x = self.amul(&x, &x)?;
            }
        }
        Ok(r)
    }
    /// Independent four-term multiplication layout used only by direct controls.
    fn reference_mul(&self, a: &Alg, b: &Alg) -> Result<Alg> {
        let aa = self.mul(&a.a, &b.a)?;
        let ab = self.mul(&a.a, &b.b)?;
        let ba = self.mul(&a.b, &b.a)?;
        let bb = self.mul(&a.b, &b.b)?;
        Ok(Alg {
            a: self.sub(&aa, &bb),
            b: self.add(&self.add(&ab, &ba), &self.mul(&self.tau, &bb)?),
        })
    }
    pub fn direct(&self, t: &Alg, rho: &Fp2, start: u64, n: u64) -> Result<Alg> {
        if n > 65536 {
            return Err("product cap".into());
        }
        let mut w = self.e.pow(rho, &start.into())?;
        let mut r = self.one();
        for _ in 0..n {
            let factor = self.asub(t, &self.ascale(&self.one(), &w)?);
            r = self.reference_mul(&r, &factor)?;
            w = self.e.mul(&w, rho)?;
        }
        Ok(r)
    }
    fn baby(&self, n: usize, rho: &Fp2) -> Result<P> {
        if n == 0 {
            return Ok(vec![Fp2::one()]);
        }
        let m = n / 2;
        let b = self.baby(m, rho)?;
        let q = self.e.pow(rho, &m.into())?;
        let mut shift = Fp2::one();
        let mut bb = vec![];
        for v in &b {
            bb.push(self.e.mul(v, &shift)?);
            shift = self.e.mul(&shift, &q)?;
        }
        let mut r = self.e.product(&b, &bb)?;
        if n % 2 == 1 {
            r = self.e.product(
                &r,
                &[Fp2::one(), self.e.neg(&self.e.pow(rho, &(2 * m).into())?)],
            )?;
        }
        Ok(r)
    }
    pub fn geometric(&self, t: &Alg, rho: &Fp2, start: u64, n: u64) -> Result<Alg> {
        if t.a.len() != self.d
            || t.b.len() != self.d
            || self.tau.len() != self.d
            || t.a
                .iter()
                .chain(&t.b)
                .chain(&self.tau)
                .chain(std::iter::once(rho))
                .any(|x| x.a >= self.e.f.p || x.b >= self.e.f.p)
        {
            return Err("malformed/noncanonical algebra input".into());
        }
        if n > 65536 || start.checked_add(n).is_none() {
            return Err("product cap/overflow".into());
        }
        if n == 0 {
            return Ok(self.one());
        }
        let inv = self.e.inv(rho)?;
        let shifted = self.ascale(t, &self.e.pow(&inv, &start.into())?)?;
        let s = n.isqrt() as usize;
        let b = self.baby(s, rho)?;
        let mut powers = vec![self.one()];
        for _ in 0..s {
            powers.push(self.amul(powers.last().unwrap(), &shifted)?);
        }
        let q = self.e.pow(rho, &s.into())?;
        let qi = self.e.inv(&q)?;
        let len = s + 1;
        let mut tri = vec![];
        let mut v = Fp2::one();
        let mut step = Fp2::one();
        for _ in 0..2 * s {
            tri.push(v.clone());
            v = self.e.mul(&v, &step)?;
            step = self.e.mul(&step, &q)?;
        }
        let mut inv_tri = vec![];
        v = Fp2::one();
        step = Fp2::one();
        for _ in 0..len {
            inv_tri.push(v.clone());
            v = self.e.mul(&v, &step)?;
            step = self.e.mul(&step, &qi)?;
        }
        let mut outputs = vec![self.zero(); s];
        // One side of every chirp convolution is scalar. No dense Alg-polynomial multiplication.
        for coordinate in 0..2 * self.d {
            let mut a = vec![z(); len];
            for j in 0..len {
                let aj = if coordinate < self.d {
                    &powers[s - j].a[coordinate]
                } else {
                    &powers[s - j].b[coordinate - self.d]
                };
                a[s - j] = self.e.mul(&self.e.mul(aj, &b[j])?, &inv_tri[j])?;
            }
            let convolution = self.e.product(&a, &tri)?;
            for i in 0..s {
                let val = self
                    .e
                    .mul(convolution.get(len - 1 + i).unwrap_or(&z()), &inv_tri[i])?;
                if coordinate < self.d {
                    outputs[i].a[coordinate] = val;
                } else {
                    outputs[i].b[coordinate - self.d] = val;
                }
            }
        }
        let mut r = self.one();
        for value in outputs {
            r = self.amul(&r, &value)?;
        }
        let mut w = self.e.pow(rho, &(s * s).into())?;
        for _ in s * s..n as usize {
            r = self.amul(&r, &self.asub(&shifted, &self.ascale(&self.one(), &w)?))?;
            w = self.e.mul(&w, rho)?;
        }
        let exponent = BigUint::from(start) * n;
        self.ascale(&r, &self.e.pow(rho, &exponent)?)
    }
    pub fn h_interval(&self, rho: &Fp2, start: u64, n: u64) -> Result<P> {
        let p = self.geometric(&self.v(), rho, start, n)?;
        // tau and S are real, so conjugation changes rho to rho^-1 and fixes v.
        if self.tau.iter().chain(&self.s).any(|x| !x.b.is_zero())
            || rho.norm(&self.e.f) != BigUint::one()
        {
            return Err("conjugation contracts".into());
        }
        let conjugate = Alg {
            a: p.a.iter().map(|v| self.e.conjugate(v)).collect(),
            b: p.b.iter().map(|v| self.e.conjugate(v)).collect(),
        };
        let vi = Alg {
            a: self.tau.clone(),
            b: self.constant(self.e.neg(&Fp2::one())),
        };
        let inv4 = self.e.inv(&c(4))?;
        let normalizer = self.apow(&self.ascale(&vi, &inv4)?, n)?;
        let h = self.amul(&self.amul(&p, &conjugate)?, &normalizer)?;
        if h.b.iter().any(|v| !is_zero(v)) || h.a.iter().any(|v| !v.b.is_zero()) {
            return Err("geometric product failed base projection".into());
        }
        Ok(h.a)
    }
}
