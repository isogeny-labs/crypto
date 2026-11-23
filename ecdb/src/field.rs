//! Exact arithmetic in GF(p^k) = F_p[t]/(f), p = 2 included, and the
//! genus-1 invariants derived from a model.
//!
//! Elements are coefficient vectors in the basis 1, t, ..., t^{k-1}. This is
//! schoolbook arithmetic for catalogue checks (a few operations per curve),
//! not a performance path.

use anyhow::{anyhow, bail, Result};
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};

/// Canonical text encoding of a field element (`ec.fe_encoding`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encoding {
    Decimal,
    HexBitset,
    DecimalVector,
}

impl Encoding {
    pub fn sql_name(self) -> &'static str {
        match self {
            Encoding::Decimal => "decimal",
            Encoding::HexBitset => "hex_bitset",
            Encoding::DecimalVector => "decimal_vector",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Fq {
    pub p: BigInt,
    /// Monic defining polynomial, ascending, length k + 1.
    pub f: Vec<BigInt>,
    pub encoding: Encoding,
}

pub type Elt = Vec<BigInt>;

impl Fq {
    pub fn prime(p: BigInt) -> Fq {
        Fq { p, f: vec![BigInt::zero(), BigInt::one()], encoding: Encoding::Decimal }
    }

    /// GF(p^k) with modulus t^k + c_{k-1} t^{k-1} + ... + c_0 given as c_0..c_{k-1}.
    pub fn extension(p: BigInt, low: &[BigInt]) -> Fq {
        let mut f: Vec<BigInt> = low.iter().map(|c| c.mod_floor(&p)).collect();
        f.push(BigInt::one());
        Fq { p, f, encoding: Encoding::DecimalVector }
    }

    /// GF(2^m) in polynomial basis from the modulus bitset (bit i = t^i).
    pub fn binary(modulus: &BigUint) -> Fq {
        let m = modulus.bits() as usize - 1;
        let f = (0..=m).map(|i| BigInt::from(modulus.bit(i as u64) as u8)).collect();
        Fq { p: BigInt::from(2u8), f, encoding: Encoding::HexBitset }
    }

    pub fn degree(&self) -> usize {
        self.f.len() - 1
    }

    pub fn zero(&self) -> Elt {
        vec![BigInt::zero(); self.degree()]
    }

    pub fn from_int(&self, n: i64) -> Elt {
        let mut e = self.zero();
        e[0] = BigInt::from(n).mod_floor(&self.p);
        e
    }

    pub fn is_zero(&self, a: &Elt) -> bool {
        a.iter().all(|c| c.is_zero())
    }

    pub fn add(&self, a: &Elt, b: &Elt) -> Elt {
        a.iter().zip(b).map(|(x, y)| (x + y).mod_floor(&self.p)).collect()
    }

    pub fn sub(&self, a: &Elt, b: &Elt) -> Elt {
        a.iter().zip(b).map(|(x, y)| (x - y).mod_floor(&self.p)).collect()
    }

    pub fn neg(&self, a: &Elt) -> Elt {
        a.iter().map(|x| (-x).mod_floor(&self.p)).collect()
    }

    pub fn scale(&self, a: &Elt, n: i64) -> Elt {
        self.mul(a, &self.from_int(n))
    }

    pub fn mul(&self, a: &Elt, b: &Elt) -> Elt {
        let k = self.degree();
        let mut prod = vec![BigInt::zero(); 2 * k - 1];
        for (i, x) in a.iter().enumerate() {
            if x.is_zero() {
                continue;
            }
            for (j, y) in b.iter().enumerate() {
                if !y.is_zero() {
                    prod[i + j] += x * y;
                }
            }
        }
        self.reduce(prod)
    }

    fn reduce(&self, mut c: Vec<BigInt>) -> Elt {
        let k = self.degree();
        for i in (k..c.len()).rev() {
            let lead = c[i].mod_floor(&self.p);
            if lead.is_zero() {
                continue;
            }
            for j in 0..k {
                c[i - k + j] -= &lead * &self.f[j];
            }
            c[i] = BigInt::zero();
        }
        c.truncate(k);
        c.resize(k, BigInt::zero());
        c.iter().map(|x| x.mod_floor(&self.p)).collect()
    }

    /// Inverse by the extended Euclidean algorithm in F_p[t].
    pub fn inv(&self, a: &Elt) -> Result<Elt> {
        if self.is_zero(a) {
            bail!("inverse of zero");
        }
        let p = &self.p;
        // Invariant: s_i * a = r_i (mod f).
        let (mut r0, mut r1) = (self.f.clone(), poly_trim(a.clone()));
        let (mut s0, mut s1) = (vec![BigInt::zero()], vec![BigInt::one()]);
        while !poly_is_zero(&r1) {
            let (q, r) = poly_divmod(&r0, &r1, p)?;
            let s2 = poly_sub(&s0, &poly_mul(&q, &s1, p), p);
            r0 = std::mem::replace(&mut r1, r);
            s0 = std::mem::replace(&mut s1, s2);
        }
        if r0.len() != 1 {
            bail!("element is not invertible: the modulus is reducible");
        }
        let c = inv_mod(&r0[0], p)?;
        let out: Vec<BigInt> = s0.iter().map(|x| (x * &c).mod_floor(p)).collect();
        Ok(self.reduce(out))
    }

    pub fn div(&self, a: &Elt, b: &Elt) -> Result<Elt> {
        Ok(self.mul(a, &self.inv(b)?))
    }

    pub fn parse(&self, s: &str) -> Result<Elt> {
        match self.encoding {
            Encoding::Decimal => {
                let v: BigInt = s.parse().map_err(|_| anyhow!("bad decimal field element {s:?}"))?;
                Ok(vec![v.mod_floor(&self.p)])
            }
            Encoding::HexBitset => {
                let digits = s.strip_prefix("0x").ok_or_else(|| anyhow!("bad hex element {s:?}"))?;
                let v = BigUint::parse_bytes(digits.as_bytes(), 16).ok_or_else(|| anyhow!("bad hex element {s:?}"))?;
                if v.bits() as usize > self.degree() {
                    bail!("binary element {s} not reduced modulo the modulus");
                }
                Ok((0..self.degree()).map(|i| BigInt::from(v.bit(i as u64) as u8)).collect())
            }
            Encoding::DecimalVector => {
                let parts: Vec<&str> = s.split(',').collect();
                if parts.len() != self.degree() {
                    bail!("extension element {s:?} has {} coefficients, field degree {}", parts.len(), self.degree());
                }
                parts
                    .iter()
                    .map(|c| {
                        c.parse::<BigInt>().map(|v| v.mod_floor(&self.p)).map_err(|_| anyhow!("bad coefficient {c:?}"))
                    })
                    .collect()
            }
        }
    }

    pub fn format(&self, a: &Elt) -> String {
        match self.encoding {
            Encoding::Decimal => a[0].to_string(),
            Encoding::HexBitset => {
                let mut v = BigUint::zero();
                for (i, c) in a.iter().enumerate() {
                    if !c.is_zero() {
                        v.set_bit(i as u64, true);
                    }
                }
                format!("0x{}", v.to_str_radix(16))
            }
            Encoding::DecimalVector => a.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","),
        }
    }

    /// Decode a packed integer e_0 + e_1 p + ... (docs/curves/covers.json
    /// coefficient encoding) into the canonical element string.
    pub fn from_packed_hex(&self, s: &str) -> Result<String> {
        let digits = s.strip_prefix("0x").ok_or_else(|| anyhow!("bad packed element {s:?}"))?;
        let mut v = BigInt::from_biguint(
            Sign::Plus,
            BigUint::parse_bytes(digits.as_bytes(), 16).ok_or_else(|| anyhow!("bad packed element {s:?}"))?,
        );
        if self.encoding == Encoding::HexBitset {
            let e: Elt = (0..self.degree()).map(|i| BigInt::from(v.bit(i as u64) as u8)).collect();
            if v.bits() as usize > self.degree() {
                bail!("binary element {s} not reduced");
            }
            return Ok(self.format(&e));
        }
        let mut e = Vec::with_capacity(self.degree());
        for _ in 0..self.degree() {
            let (q, r) = v.div_mod_floor(&self.p);
            e.push(r);
            v = q;
        }
        if !v.is_zero() {
            bail!("packed element {s} exceeds the field size");
        }
        Ok(self.format(&e))
    }
}

fn poly_trim(mut v: Vec<BigInt>) -> Vec<BigInt> {
    while v.len() > 1 && v.last().is_some_and(|c| c.is_zero()) {
        v.pop();
    }
    if v.is_empty() {
        v.push(BigInt::zero());
    }
    v
}

fn poly_is_zero(v: &[BigInt]) -> bool {
    v.iter().all(|c| c.is_zero())
}

fn inv_mod(x: &BigInt, p: &BigInt) -> Result<BigInt> {
    let g = x.mod_floor(p).extended_gcd(p);
    if !g.gcd.is_one() {
        bail!("{x} is not invertible modulo {p}");
    }
    Ok(g.x.mod_floor(p))
}

fn poly_mul(a: &[BigInt], b: &[BigInt], p: &BigInt) -> Vec<BigInt> {
    let mut c = vec![BigInt::zero(); a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            c[i + j] += x * y;
        }
    }
    poly_trim(c.into_iter().map(|x| x.mod_floor(p)).collect())
}

fn poly_sub(a: &[BigInt], b: &[BigInt], p: &BigInt) -> Vec<BigInt> {
    let n = a.len().max(b.len());
    let c = (0..n)
        .map(|i| {
            let x = a.get(i).cloned().unwrap_or_default();
            let y = b.get(i).cloned().unwrap_or_default();
            (x - y).mod_floor(p)
        })
        .collect();
    poly_trim(c)
}

/// Quotient and remainder in F_p[t]; b is nonzero.
fn poly_divmod(a: &[BigInt], b: &[BigInt], p: &BigInt) -> Result<(Vec<BigInt>, Vec<BigInt>)> {
    let b = poly_trim(b.to_vec());
    let db = b.len() - 1;
    let lead_inv = inv_mod(&b[db], p)?;
    let mut r = poly_trim(a.iter().map(|x| x.mod_floor(p)).collect());
    if r.len() - 1 < db {
        return Ok((vec![BigInt::zero()], r));
    }
    let mut q = vec![BigInt::zero(); r.len() - db];
    while !poly_is_zero(&r) && r.len() > db {
        let shift = r.len() - 1 - db;
        let coef = (&r[r.len() - 1] * &lead_inv).mod_floor(p);
        for j in 0..=db {
            r[shift + j] = (&r[shift + j] - &coef * &b[j]).mod_floor(p);
        }
        q[shift] = coef;
        r = poly_trim(r);
    }
    Ok((poly_trim(q), r))
}

/// a-, b-, c-invariants, discriminant and j of a long Weierstrass model.
#[derive(Clone, Debug)]
pub struct Genus1Invariants {
    pub a: [Elt; 5],
    pub b2: Elt,
    pub b4: Elt,
    pub b6: Elt,
    pub b8: Elt,
    pub c4: Elt,
    pub c6: Elt,
    pub disc: Elt,
    pub j: Elt,
}

pub fn invariants_from_a(k: &Fq, a: [Elt; 5]) -> Result<Genus1Invariants> {
    let [a1, a2, a3, a4, a6] = &a;
    let m = |x: &Elt, y: &Elt| k.mul(x, y);
    let b2 = k.add(&m(a1, a1), &k.scale(a2, 4));
    let b4 = k.add(&k.scale(a4, 2), &m(a1, a3));
    let b6 = k.add(&m(a3, a3), &k.scale(a6, 4));
    // b8 = a1^2 a6 + 4 a2 a6 - a1 a3 a4 + a2 a3^2 - a4^2
    let b8 = {
        let t1 = m(&m(a1, a1), a6);
        let t2 = k.scale(&m(a2, a6), 4);
        let t3 = m(&m(a1, a3), a4);
        let t4 = m(a2, &m(a3, a3));
        let t5 = m(a4, a4);
        k.sub(&k.add(&k.sub(&k.add(&t1, &t2), &t3), &t4), &t5)
    };
    let c4 = k.sub(&m(&b2, &b2), &k.scale(&b4, 24));
    let c6 = k.add(&k.neg(&m(&m(&b2, &b2), &b2)), &k.sub(&k.scale(&m(&b2, &b4), 36), &k.scale(&b6, 216)));
    // Delta = -b2^2 b8 - 8 b4^3 - 27 b6^2 + 9 b2 b4 b6
    let disc = {
        let t1 = k.neg(&m(&m(&b2, &b2), &b8));
        let t2 = k.scale(&m(&m(&b4, &b4), &b4), 8);
        let t3 = k.scale(&m(&b6, &b6), 27);
        let t4 = k.scale(&m(&m(&b2, &b4), &b6), 9);
        k.add(&k.sub(&k.sub(&t1, &t2), &t3), &t4)
    };
    if k.is_zero(&disc) {
        bail!("singular model: discriminant is zero");
    }
    let j = k.div(&m(&m(&c4, &c4), &c4), &disc)?;
    Ok(Genus1Invariants { a, b2, b4, b6, b8, c4, c6, disc, j })
}

/// Long Weierstrass a-invariants of a supported genus-1 model, with the
/// birational map used. Coefficients are canonical element strings.
pub fn a_invariants(
    k: &Fq,
    model: &str,
    coeff: &serde_json::Map<String, serde_json::Value>,
) -> Result<([Elt; 5], &'static str)> {
    let get = |name: &str| -> Result<Elt> {
        let s = coeff.get(name).and_then(|v| v.as_str()).ok_or_else(|| anyhow!("missing coefficient {name}"))?;
        k.parse(s)
    };
    let zero = k.zero();
    let one = k.from_int(1);
    let montgomery = |a_m: Elt, b_m: Elt| -> Result<[Elt; 5]> {
        // B y^2 = x^3 + A x^2 + x  ->  (x/B, y/B): Y^2 = X^3 + (A/B) X^2 + (1/B^2) X
        let binv = k.inv(&b_m)?;
        Ok([zero.clone(), k.mul(&a_m, &binv), zero.clone(), k.mul(&binv, &binv), zero.clone()])
    };
    let twisted_edwards = |a_e: Elt, d_e: Elt| -> Result<[Elt; 5]> {
        // a x^2 + y^2 = 1 + d x^2 y^2  ~  Montgomery A = 2(a+d)/(a-d), B = 4/(a-d)
        let amd = k.sub(&a_e, &d_e);
        let inv = k.inv(&amd)?;
        let a_m = k.mul(&k.scale(&k.add(&a_e, &d_e), 2), &inv);
        let b_m = k.mul(&k.from_int(4), &inv);
        montgomery(a_m, b_m)
    };
    Ok(match model {
        "short_weierstrass" => ([zero.clone(), zero.clone(), zero.clone(), get("a")?, get("b")?], "identity"),
        "binary_weierstrass" => ([one, get("a")?, zero.clone(), zero.clone(), get("b")?], "identity"),
        "montgomery" => (montgomery(get("A")?, get("B")?)?, "(x, y) -> (x/B, y/B)"),
        "twisted_edwards" => (
            twisted_edwards(get("a")?, get("d")?)?,
            "Edwards -> Montgomery A = 2(a+d)/(a-d), B = 4/(a-d), then (x, y) -> (x/B, y/B)",
        ),
        "edwards" => {
            // x^2 + y^2 = c^2 (1 + d x^2 y^2) -> (x/c, y/c): twisted Edwards with a = 1, d' = d c^4
            let c = get("c")?;
            let c2 = k.mul(&c, &c);
            let d = k.mul(&get("d")?, &k.mul(&c2, &c2));
            (
                twisted_edwards(k.from_int(1), d)?,
                "(x, y) -> (x/c, y/c), then twisted Edwards a = 1, d' = d c^4 to Montgomery to Weierstrass",
            )
        }
        other => bail!("no a-invariant map for model {other}"),
    })
}

/// Bit length of |n|.
pub fn bits(n: &BigInt) -> u64 {
    n.abs().bits()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bi(s: &str) -> BigInt {
        s.parse().unwrap()
    }

    #[test]
    fn prime_field_j_matches_icv1_formula() {
        // registry: y^2 = x^3 + x + 13 over GF(59), j = 52
        let k = Fq::prime(bi("59"));
        let mut c = serde_json::Map::new();
        c.insert("a".into(), "1".into());
        c.insert("b".into(), "13".into());
        let (a, _) = a_invariants(&k, "short_weierstrass", &c).unwrap();
        let inv = invariants_from_a(&k, a).unwrap();
        assert_eq!(k.format(&inv.j), "52");
    }

    #[test]
    fn binary_j_is_inverse_of_b() {
        // registry: y^2 + xy = x^3 + 0x25 x^2 + 0x4 over GF(2)[t]/(0x11b), j = 0xcb
        let k = Fq::binary(&BigUint::parse_bytes(b"11b", 16).unwrap());
        let mut c = serde_json::Map::new();
        c.insert("a".into(), "0x25".into());
        c.insert("b".into(), "0x4".into());
        let (a, _) = a_invariants(&k, "binary_weierstrass", &c).unwrap();
        let inv = invariants_from_a(&k, a).unwrap();
        assert_eq!(k.format(&inv.j), "0xcb");
        assert_eq!(k.format(&inv.disc), "0x4");
    }

    #[test]
    fn extension_field_j() {
        // registry: y^2 = x^3 + a x + b over GF(5)[t]/(t^2 + 2), a = [3,1], b = [4,1], j = "4,4"
        let k = Fq::extension(bi("5"), &[bi("2"), bi("0")]);
        let mut c = serde_json::Map::new();
        c.insert("a".into(), "3,1".into());
        c.insert("b".into(), "4,1".into());
        let (a, _) = a_invariants(&k, "short_weierstrass", &c).unwrap();
        let inv = invariants_from_a(&k, a).unwrap();
        assert_eq!(k.format(&inv.j), "4,4");
    }

    #[test]
    fn inverse_round_trips() {
        let k = Fq::binary(&BigUint::parse_bytes(b"83", 16).unwrap());
        for v in 1..128u32 {
            let e = k.parse(&format!("0x{v:x}")).unwrap();
            let i = k.inv(&e).unwrap();
            assert_eq!(k.format(&k.mul(&e, &i)), "0x1");
        }
        let k = Fq::extension(bi("7"), &[bi("3"), bi("1"), bi("0")]); // t^3 + t + 3, irreducible mod 7
        let e = k.parse("2,5,6").unwrap();
        let i = k.inv(&e).unwrap();
        assert_eq!(k.format(&k.mul(&e, &i)), "1,0,0");
    }

    #[test]
    fn packed_elements_decode() {
        let k = Fq::extension(bi("5"), &[bi("2"), bi("0")]);
        assert_eq!(k.from_packed_hex("0x8").unwrap(), "3,1"); // 3 + 1*5
        let k = Fq::prime(bi("59"));
        assert_eq!(k.from_packed_hex("0x34").unwrap(), "52");
        let k = Fq::binary(&BigUint::parse_bytes(b"83", 16).unwrap());
        assert_eq!(k.from_packed_hex("0x26").unwrap(), "0x26");
    }
}
