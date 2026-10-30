//! Certified finite fields with canonical integer polynomial-basis encoding.
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::Value;
pub type N = BigUint;
pub type Result<T> = std::result::Result<T, String>;
pub fn number(v: &Value) -> Result<N> {
    if let Some(n) = v.as_u64() {
        return Ok(n.into());
    }
    let s = v
        .as_str()
        .ok_or("expected unsigned integer or exact string")?;
    let (s, radix) = s.strip_prefix("0x").map(|s| (s, 16)).unwrap_or((s, 10));
    N::parse_bytes(s.as_bytes(), radix).ok_or("invalid exact integer".into())
}
fn prime64(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n % p == 0 {
            return n == p;
        }
    }
    let d = (n - 1) >> (n - 1).trailing_zeros();
    let power = |mut a: u64, mut e: u64| {
        let mut r = 1u64;
        while e > 0 {
            if e & 1 != 0 {
                r = (r as u128 * a as u128 % n as u128) as u64;
            }
            a = (a as u128 * a as u128 % n as u128) as u64;
            e >>= 1;
        }
        r
    };
    for a in [2, 325, 9375, 28178, 450775, 9780504, 1795265022] {
        if a % n == 0 {
            continue;
        }
        let mut x = power(a % n, d);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut pass = false;
        for _ in 1..(n - 1).trailing_zeros() {
            x = (x as u128 * x as u128 % n as u128) as u64;
            if x == n - 1 {
                pass = true;
                break;
            }
        }
        if !pass {
            return false;
        }
    }
    true
}
pub fn certify_prime(n: &N, certificate: Option<&Value>, depth: usize) -> Result<()> {
    if let Some(v) = n.to_u64() {
        return if prime64(v) {
            Ok(())
        } else {
            Err("characteristic is not prime".into())
        };
    }
    if depth > 128 {
        return Err("prime certificate nesting exceeds 128".into());
    }
    let c = certificate.ok_or("characteristics >=2^64 require a Pocklington certificate")?;
    if let Some(v) = c.get("n") {
        if number(v)? != *n {
            return Err("certificate n mismatch".into());
        }
    }
    let factors = c["factors"]
        .as_array()
        .filter(|a| !a.is_empty())
        .ok_or("missing Pocklington factors")?;
    let mut seen = std::collections::BTreeSet::new();
    let mut product = N::one();
    let nm = n - N::one();
    for f in factors {
        let q = number(&f["q"])?;
        let e = f["e"].as_u64().ok_or("certificate exponent")?;
        let a = number(&f["a"])?;
        if q < N::from(2u8)
            || q >= *n
            || !seen.insert(q.clone())
            || e == 0
            || e > n.bits()
            || a.is_zero()
            || a >= *n
        {
            return Err("invalid Pocklington factor/witness".into());
        }
        certify_prime(&q, f.get("certificate"), depth + 1)?;
        product *= q.pow(u32::try_from(e).map_err(|_| "certificate exponent too large")?);
        if &nm % &product != N::zero() {
            return Err("certificate factors do not divide n-1".into());
        }
        let z = a.modpow(&(&nm / &q), n);
        let difference = if z.is_zero() {
            n - N::one()
        } else {
            z - N::one()
        };
        if a.modpow(&nm, n) != N::one() || difference.gcd(n) != N::one() {
            return Err("Pocklington witness failed".into());
        }
    }
    if &product * &product <= *n {
        return Err("Pocklington factor product <= sqrt(n)".into());
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Field {
    pub p: N,
    pub degree: usize,
    pub order: N,
    pub modulus: Vec<N>,
    binary_mask: N,
}
impl Field {
    pub fn prime_subfield(&self) -> Self {
        Self {
            p: self.p.clone(),
            degree: 1,
            order: self.p.clone(),
            modulus: vec![],
            binary_mask: N::zero(),
        }
    }
    pub fn prime(p: N, certificate: Option<&Value>) -> Result<Self> {
        certify_prime(&p, certificate, 0)?;
        Ok(Self {
            order: p.clone(),
            p,
            degree: 1,
            modulus: vec![],
            binary_mask: N::zero(),
        })
    }
    pub fn binary(degree: usize, mask: N) -> Result<Self> {
        if degree == 0 || degree > 4096 || mask.bits() != degree as u64 + 1 || !mask.bit(0) {
            return Err("invalid binary modulus".into());
        }
        let modulus = (0..=degree)
            .map(|i| N::from(u8::from(mask.bit(i as u64))))
            .collect();
        Self::extension(N::from(2u8), modulus, None)
    }
    pub fn extension(p: N, modulus: Vec<N>, certificate: Option<&Value>) -> Result<Self> {
        certify_prime(&p, certificate, 0)?;
        if modulus.len() < 2
            || modulus.len() > 4097
            || modulus.last() != Some(&N::one())
            || modulus.iter().any(|a| a >= &p)
        {
            return Err("extension modulus must be canonical and monic".into());
        }
        let degree = modulus.len() - 1;
        let binary_mask = if p == N::from(2u8) {
            modulus
                .iter()
                .enumerate()
                .filter(|(_, a)| !a.is_zero())
                .fold(N::zero(), |v, (i, _)| v | (N::one() << i))
        } else {
            N::zero()
        };
        let field = Self {
            order: p.pow(degree as u32),
            p,
            degree,
            modulus,
            binary_mask,
        };
        if field.p == N::from(2u8) {
            let reduce = |mut a: N| {
                while a.bits() >= field.binary_mask.bits() {
                    let j = (a.bits() - field.binary_mask.bits()) as usize;
                    a ^= &field.binary_mask << j;
                }
                a
            };
            let gcd = |mut a: N, mut b: N| {
                while !b.is_zero() {
                    while !a.is_zero() && a.bits() >= b.bits() {
                        let j = (a.bits() - b.bits()) as usize;
                        a ^= &b << j;
                    }
                    std::mem::swap(&mut a, &mut b);
                }
                a
            };
            let x = reduce(N::from(2u8));
            let mut h = x.clone();
            let mut d = degree;
            let mut factors = Vec::new();
            let mut r = 2;
            while r * r <= d {
                if d % r == 0 {
                    factors.push(r);
                    while d % r == 0 {
                        d /= r;
                    }
                }
                r += 1;
            }
            if d > 1 {
                factors.push(d);
            }
            for i in 1..=degree {
                let mut square = N::zero();
                for j in 0..h.bits() {
                    if h.bit(j) {
                        square.set_bit(2 * j, true);
                    }
                }
                h = reduce(square);
                if factors.iter().any(|r| i == degree / r)
                    && gcd(&h ^ &x, field.binary_mask.clone()) != N::one()
                {
                    return Err("reducible binary modulus".into());
                }
            }
            if h != x {
                return Err("binary modulus fails Frobenius identity".into());
            }
            return Ok(field);
        }
        // Rabin's irreducibility test in F_p[X], independent of inverses
        // in the extension being certified.
        let base = Self::prime(field.p.clone(), certificate)?;
        let x = vec![N::zero(), N::one()];
        let mut h = x.clone();
        let mut factors = Vec::new();
        let mut d = degree;
        let mut r = 2;
        while r * r <= d {
            if d % r == 0 {
                factors.push(r);
                while d % r == 0 {
                    d /= r;
                }
            }
            r += 1;
        }
        if d > 1 {
            factors.push(d);
        }
        for i in 1..=degree {
            h = super::poly::powmod(&base, &h, &field.p, &field.modulus);
            if factors.iter().any(|r| i == degree / r)
                && super::poly::gcd(
                    &base,
                    field.modulus.clone(),
                    super::poly::sub(&base, &h, &x),
                )
                .len()
                    != 1
            {
                return Err("reducible extension modulus".into());
            }
        }
        if !super::poly::sub(&base, &h, &super::poly::rem(&base, x, &field.modulus)).is_empty() {
            return Err("extension modulus fails Frobenius identity".into());
        }
        Ok(field)
    }
    pub fn validate(&self, a: &N) -> Result<()> {
        if a < &self.order {
            Ok(())
        } else {
            Err("field element outside canonical range".into())
        }
    }
    pub fn digits(&self, a: &N) -> Vec<N> {
        let mut v = a.clone();
        (0..self.degree)
            .map(|_| {
                let r = &v % &self.p;
                v /= &self.p;
                r
            })
            .collect()
    }
    pub fn encode(&self, digits: &[N]) -> N {
        digits.iter().rev().fold(N::zero(), |v, a| v * &self.p + a)
    }
    pub fn add(&self, a: &N, b: &N) -> N {
        if self.p == N::from(2u8) {
            return a ^ b;
        }
        if self.degree == 1 {
            return (a + b) % &self.p;
        }
        self.encode(
            &self
                .digits(a)
                .iter()
                .zip(self.digits(b))
                .map(|(a, b)| (a + b) % &self.p)
                .collect::<Vec<_>>(),
        )
    }
    pub fn neg(&self, a: &N) -> N {
        if self.p == N::from(2u8) {
            return a.clone();
        }
        if self.degree == 1 {
            return if a.is_zero() { N::zero() } else { &self.p - a };
        }
        self.encode(
            &self
                .digits(a)
                .iter()
                .map(|v| if v.is_zero() { N::zero() } else { &self.p - v })
                .collect::<Vec<_>>(),
        )
    }
    pub fn sub(&self, a: &N, b: &N) -> N {
        self.add(a, &self.neg(b))
    }
    pub fn mul(&self, a: &N, b: &N) -> N {
        if self.degree == 1 {
            return (a * b) % &self.p;
        }
        if self.p == N::from(2u8) {
            let (mut a, mut b, mut out) = (a.clone(), b.clone(), N::zero());
            while !b.is_zero() {
                if b.bit(0) {
                    out ^= &a;
                }
                b >>= 1;
                a <<= 1;
                if a.bit(self.degree as u64) {
                    a ^= &self.binary_mask;
                }
            }
            return out;
        }
        let (a, b) = (self.digits(a), self.digits(b));
        let mut out = vec![N::zero(); 2 * self.degree - 1];
        for (i, a) in a.iter().enumerate() {
            for (j, b) in b.iter().enumerate() {
                out[i + j] = (&out[i + j] + a * b) % &self.p;
            }
        }
        for i in (self.degree..out.len()).rev() {
            let top = out[i].clone();
            for j in 0..self.degree {
                let v = &top * &self.modulus[j] % &self.p;
                out[i - self.degree + j] = (&out[i - self.degree + j] + &self.p - v) % &self.p;
            }
        }
        self.encode(&out[..self.degree])
    }
    pub fn pow(&self, a: &N, e: &N) -> N {
        let (mut a, mut e, mut out) = (a.clone(), e.clone(), N::one());
        while !e.is_zero() {
            if e.bit(0) {
                out = self.mul(&out, &a);
            }
            e >>= 1;
            if !e.is_zero() {
                a = self.mul(&a, &a);
            }
        }
        out
    }
    pub fn inv(&self, a: &N) -> N {
        assert!(!a.is_zero());
        if self.degree > 1 && self.p == N::from(2u8) {
            let (mut u, mut v, mut g, mut h) =
                (a.clone(), self.binary_mask.clone(), N::one(), N::zero());
            while u != N::one() {
                assert!(!u.is_zero());
                if u.bits() < v.bits() {
                    std::mem::swap(&mut u, &mut v);
                    std::mem::swap(&mut g, &mut h);
                }
                let j = (u.bits() - v.bits()) as usize;
                u ^= &v << j;
                g ^= &h << j;
            }
            while g.bits() >= self.binary_mask.bits() {
                let j = (g.bits() - self.binary_mask.bits()) as usize;
                g ^= &self.binary_mask << j;
            }
            return g;
        }
        self.pow(a, &(&self.order - N::from(2u8)))
    }
    pub fn element(&self, v: &Value) -> Result<N> {
        if let Some(object) = v.as_object() {
            if object.len() != 1 || !object.contains_key("scalar") {
                return Err("scalar coefficient object must contain only scalar".into());
            }
        }
        let a = if let Some(v) = v.get("scalar") {
            let signed = if let Some(i) = v.as_i64() {
                num_bigint::BigInt::from(i)
            } else if v.as_str().is_some_and(|s| s.starts_with('-')) {
                num_bigint::BigInt::parse_bytes(v.as_str().unwrap().as_bytes(), 10)
                    .ok_or("invalid signed scalar")?
            } else {
                num_bigint::BigInt::from(number(v)?)
            };
            let p = num_bigint::BigInt::from(self.p.clone());
            ((signed % &p + &p) % p).to_biguint().unwrap()
        } else {
            number(v)?
        };
        self.validate(&a)?;
        Ok(a)
    }
    pub fn from_json(v: &Value) -> Result<Self> {
        match v["kind"]
            .as_str()
            .unwrap_or(if v.get("characteristic").is_some() {
                "extension"
            } else {
                "binary"
            }) {
            "prime" => Self::prime(number(&v["characteristic"])?, v.get("prime_certificate")),
            "binary" => {
                if let Some(p) = v.get("characteristic") {
                    if number(p)? != N::from(2u8) {
                        return Err("binary characteristic must be two".into());
                    }
                }
                let d = v["degree"].as_u64().unwrap_or(83) as usize;
                let mask = if let Some(m) = v.get("modulus") {
                    number(m)?
                } else if d == 83 {
                    (N::one() << 83) | (N::one() << 45) | N::from(7u8)
                } else {
                    return Err("binary modulus required".into());
                };
                Self::binary(d, mask)
            }
            "extension" => {
                let p = number(&v["characteristic"])?;
                let m = v["modulus"]
                    .as_array()
                    .ok_or("explicit extension modulus required")?
                    .iter()
                    .map(number)
                    .collect::<Result<Vec<_>>>()?;
                if let Some(d) = v["degree"].as_u64() {
                    if d as usize + 1 != m.len() {
                        return Err("extension degree mismatch".into());
                    }
                }
                Self::extension(p, m, v.get("prime_certificate"))
            }
            _ => Err("unknown field kind".into()),
        }
    }
}
