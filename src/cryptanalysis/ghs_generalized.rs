//! Exact generalized Hess profiles over every proper binary subfield.
//!
//! The order polynomials have coefficients in F2, even when the base field
//! is larger. This matters: taking a linear span over the base field, or
//! adjoining the constant 1 to a span of coefficients, gives wrong genera.
//! These profiles are construction inputs, not subgroup certificates.

use super::ghs_screen::{GhsCurveInput, validate_input};
use crate::binary_ecc::{F2mElement, IrreduciblePoly};
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::Serialize;
use std::collections::BTreeMap;

fn degree(p: &BigUint) -> u32 {
    p.bits().saturating_sub(1) as u32
}

pub fn polynomial_div_rem(a: &BigUint, b: &BigUint) -> Result<(BigUint, BigUint), String> {
    if b.is_zero() {
        return Err("zero polynomial divisor".into());
    }
    let mut rem = a.clone();
    let mut quo = BigUint::zero();
    while !rem.is_zero() && rem.bits() >= b.bits() {
        let shift = (rem.bits() - b.bits()) as usize;
        quo ^= BigUint::one() << shift;
        rem ^= b << shift;
    }
    Ok((quo, rem))
}

fn polynomial_mul(mut a: BigUint, mut b: BigUint) -> BigUint {
    let mut out = BigUint::zero();
    while !b.is_zero() {
        if b.bit(0) {
            out ^= &a;
        }
        a <<= 1;
        b >>= 1;
    }
    out
}

pub fn polynomial_lcm(a: &BigUint, b: &BigUint) -> BigUint {
    let (mut x, mut y) = (a.clone(), b.clone());
    while !y.is_zero() {
        let rem = polynomial_div_rem(&x, &y).expect("nonzero divisor").1;
        x = y;
        y = rem;
    }
    polynomial_mul(
        polynomial_div_rem(a, &x).expect("gcd is nonzero").0,
        b.clone(),
    )
}

pub fn frobenius(mut x: F2mElement, base_degree: u32, irr: &IrreduciblePoly) -> F2mElement {
    for _ in 0..base_degree {
        x = x.square(irr);
    }
    x
}

pub fn relative_trace(x: &F2mElement, base_degree: u32, irr: &IrreduciblePoly) -> F2mElement {
    let mut out = F2mElement::zero(irr.degree);
    let mut term = x.clone();
    for _ in 0..irr.degree / base_degree {
        out = out.add(&term);
        term = frobenius(term, base_degree, irr);
    }
    out
}

/// First binary Krylov dependence, including repeated factors of T^n-1.
pub fn frobenius_order(
    x: &F2mElement,
    base_degree: u32,
    irr: &IrreduciblePoly,
) -> Result<BigUint, String> {
    if base_degree == 0 || !irr.degree.is_multiple_of(base_degree) {
        return Err("base degree must divide the absolute degree".into());
    }
    let mut pivots: BTreeMap<u64, (BigUint, BigUint)> = BTreeMap::new();
    let mut term = x.clone();
    for i in 0..=irr.degree / base_degree {
        let mut value = term.to_biguint();
        let mut relation = BigUint::one() << i as usize;
        while !value.is_zero() {
            let pivot = value.bits() - 1;
            if let Some((row, witness)) = pivots.get(&pivot) {
                value ^= row;
                relation ^= witness;
            } else {
                pivots.insert(pivot, (value.clone(), relation.clone()));
                break;
            }
        }
        if value.is_zero() {
            return Ok(relation);
        }
        term = frobenius(term, base_degree, irr);
    }
    Err("Frobenius orbit did not close".into())
}

pub fn linearized(
    f: &BigUint,
    x: &F2mElement,
    base_degree: u32,
    irr: &IrreduciblePoly,
) -> F2mElement {
    let mut out = F2mElement::zero(irr.degree);
    let mut term = x.clone();
    for i in 0..f.bits() {
        if f.bit(i) {
            out = out.add(&term);
        }
        term = frobenius(term, base_degree, irr);
    }
    out
}

/// Binary polynomial bitsets and field elements are canonical hexadecimal.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct GeneralizedProfile {
    pub base_degree: u32,
    pub relative_degree: u32,
    pub c: String,
    pub d: String,
    pub f1: String,
    pub f2: String,
    pub s1: u32,
    pub s2: u32,
    pub compositum_rank: u32,
    pub absolute_trace_a: u8,
    pub genus: String,
    pub descends: bool,
    pub hyperelliptic_sufficient: bool,
}

#[derive(Clone, Debug)]
pub struct GeneralizedContext {
    pub input: GhsCurveInput,
    pub irr: IrreduciblePoly,
    pub a: F2mElement,
    pub beta: F2mElement,
    pub delta: u8,
}

impl GeneralizedContext {
    pub fn new(input: &GhsCurveInput) -> Result<Self, String> {
        let irr = validate_input(input).map_err(|e| e.to_string())?;
        let a = F2mElement::from_biguint(&input.a, input.absolute_degree);
        let b = F2mElement::from_biguint(&input.b, input.absolute_degree);
        let beta = frobenius(b, input.absolute_degree - 1, &irr);
        let trace = relative_trace(&a, 1, &irr);
        let delta = u8::from(!trace.is_zero());
        Ok(Self {
            input: input.clone(),
            irr,
            a,
            beta,
            delta,
        })
    }

    pub fn profile(&self, base_degree: u32, c: &BigUint) -> Result<GeneralizedProfile, String> {
        let n = self.input.absolute_degree;
        if base_degree == 0 || base_degree >= n || !n.is_multiple_of(base_degree) {
            return Err("base degree must be a proper positive divisor".into());
        }
        if c.is_zero() || c.bits() > u64::from(n) {
            return Err("c must be a nonzero canonical field element".into());
        }
        let c = F2mElement::from_biguint(c, n);
        let d = self.beta.mul(
            &c.flt_inverse(&self.irr).ok_or("c is not invertible")?,
            &self.irr,
        );
        let f1 = frobenius_order(&c, base_degree, &self.irr)?;
        let f2 = frobenius_order(&d, base_degree, &self.irr)?;
        let mut f = polynomial_lcm(&f1, &f2);
        if self.delta == 1 {
            f = polynomial_lcm(&f, &BigUint::from(3u8));
        }
        let (s1, s2, t) = (degree(&f1), degree(&f2), degree(&f));
        let genus = (BigUint::one() << t as usize) + BigUint::one()
            - (BigUint::one() << (t - s1) as usize)
            - (BigUint::one() << (t - s2) as usize);
        let descends = self.delta == 0
            || !relative_trace(&c, base_degree, &self.irr).is_zero()
            || !relative_trace(&d, base_degree, &self.irr).is_zero();
        let fixed = |x: &F2mElement| frobenius(x.clone(), base_degree, &self.irr) == *x;
        Ok(GeneralizedProfile {
            base_degree,
            relative_degree: n / base_degree,
            c: format!("0x{:x}", c.to_biguint()),
            d: format!("0x{:x}", d.to_biguint()),
            f1: format!("0x{f1:x}"),
            f2: format!("0x{f2:x}"),
            s1,
            s2,
            compositum_rank: t,
            absolute_trace_a: self.delta,
            genus: genus.to_string(),
            descends,
            hyperelliptic_sufficient: fixed(&c) || fixed(&d),
        })
    }

    /// Generate from a Frobenius kernel using L_(T^n-1)/f. The returned
    /// element's *exact* order may be a proper divisor of the requested f.
    pub fn kernel_element(
        &self,
        base_degree: u32,
        f: &BigUint,
        z: &BigUint,
    ) -> Result<F2mElement, String> {
        if base_degree == 0 || !self.input.absolute_degree.is_multiple_of(base_degree) {
            return Err("base degree must divide the absolute degree".into());
        }
        if z.bits() > u64::from(self.input.absolute_degree) {
            return Err("seed must be a canonical field element".into());
        }
        let full = (BigUint::one() << (self.input.absolute_degree / base_degree) as usize)
            ^ BigUint::one();
        let (quotient, rem) = polynomial_div_rem(&full, f)?;
        if !rem.is_zero() {
            return Err("requested order polynomial does not divide T^n-1".into());
        }
        Ok(linearized(
            &quotient,
            &F2mElement::from_biguint(z, self.irr.degree),
            base_degree,
            &self.irr,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn context(n: u32, modulus: u64, a: u64, b: u64) -> GeneralizedContext {
        GeneralizedContext::new(&GhsCurveInput {
            absolute_degree: n,
            modulus: modulus.into(),
            a: a.into(),
            b: b.into(),
        })
        .unwrap()
    }

    #[test]
    fn generalized_profiles_match_independent_character_sum() {
        // Independently enumerate binary AS characters. A nonconstant
        // A/x+B*x character has genus 1 precisely when A and B are nonzero.
        for ctx in [context(4, 0x13, 0, 2), context(6, 0x43, 0, 7)] {
            for s in 1..ctx.irr.degree {
                if !ctx.irr.degree.is_multiple_of(s) {
                    continue;
                }
                for ci in 1..1u64 << ctx.irr.degree {
                    let p = ctx.profile(s, &ci.into()).unwrap();
                    let c = F2mElement::from_biguint(&ci.into(), ctx.irr.degree);
                    let d = ctx.beta.mul(&c.flt_inverse(&ctx.irr).unwrap(), &ctx.irr);
                    let mut pairs = vec![(BigUint::zero(), BigUint::zero())];
                    let (mut c0, mut d0) = (c, d);
                    for _ in 0..ctx.irr.degree / s {
                        let shift = (c0.to_biguint(), d0.to_biguint());
                        let mut extended = pairs.clone();
                        extended.extend(pairs.iter().map(|(a, b)| (a ^ &shift.0, b ^ &shift.1)));
                        extended.sort();
                        extended.dedup();
                        pairs = extended;
                        c0 = frobenius(c0, s, &ctx.irr);
                        d0 = frobenius(d0, s, &ctx.irr);
                    }
                    let independent = pairs
                        .iter()
                        .filter(|(a, b)| !a.is_zero() && !b.is_zero())
                        .count();
                    assert_eq!(p.genus, independent.to_string());
                }
            }
        }
    }

    #[test]
    fn preserves_repeated_factors_and_kernel_images() {
        let ctx = context(8, 0x11b, 1, 2);
        for f in [1u64, 3, 5, 15, 17, 51, 85, 255, 257] {
            let full = BigUint::from(257u64);
            if !polynomial_div_rem(&full, &f.into()).unwrap().1.is_zero() {
                continue;
            }
            for z in 0..256u64 {
                let u = ctx.kernel_element(1, &f.into(), &z.into()).unwrap();
                assert!(linearized(&f.into(), &u, 1, &ctx.irr).is_zero());
                let exact = frobenius_order(&u, 1, &ctx.irr).unwrap();
                assert!(polynomial_div_rem(&f.into(), &exact).unwrap().1.is_zero());
            }
        }
    }

    #[test]
    fn base_larger_than_f2_and_trace_obstruction() {
        let ctx = context(4, 0x13, 0, 6);
        let p = ctx.profile(2, &1u8.into()).unwrap();
        assert_eq!(p.compositum_rank, 1);
        assert_eq!(p.genus, "1");
        let ctx = context(4, 0x13, 8, 1);
        let p = ctx.profile(2, &1u8.into()).unwrap();
        assert_eq!(p.absolute_trace_a, 1);
        assert!(!p.descends);
    }
}
