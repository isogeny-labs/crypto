//! Degree-eight adapter on unchanged native Engine arithmetic, not the old Ring.
use crate::Result;
use num_bigint::BigUint;
use num_traits::Zero;
use serde_json::{json, Value};
use torus_quotient_query::arithmetic::{c, encoded, trim, Engine, P};
use torus_s4_tree::Fp2;

pub fn powmod(e: &Engine, a: &[Fp2], exponent: &BigUint, modulus: &[Fp2]) -> Result<P> {
    if !(2..=9).contains(&modulus.len()) {
        return Err("new degree8 modulus admission".into());
    }
    let mut x = e.divrem(a, modulus)?.1;
    let mut r = vec![c(1)];
    for i in 0..exponent.bits() {
        if exponent.bit(i) {
            r = e.divrem(&e.product(&r, &x)?, modulus)?.1;
        }
        if i + 1 < exponent.bits() {
            x = e.divrem(&e.product(&x, &x)?, modulus)?.1;
        }
    }
    Ok(r)
}

pub fn roots(e: &Engine, input: &[Fp2]) -> Result<(Vec<(BigUint, usize)>, Value)> {
    if input.iter().any(|x| x.a >= e.f.p || !x.b.is_zero()) {
        return Err("noncanonical/non-Fp polynomial".into());
    }
    let p = trim(input.to_vec());
    if p.is_empty() {
        return Err("zero polynomial: degenerate family, unsupported".into());
    }
    if p.len() > 9 {
        return Err("new degree8 root admission".into());
    }
    if p.len() == 1 {
        return Ok((
            vec![],
            json!({"polynomial":encoded(&p),"fp_root_factor":encoded(&[c(1)]),"roots":[],"complete_fp_support":true}),
        ));
    }
    let monic = e.monic(&p)?;
    let frobenius = powmod(e, &[c(0), c(1)], &e.f.p, &monic)?;
    let factor = e.gcd(&monic, &e.psub(&frobenius, &[c(0), c(1)]))?;
    let mut stack = vec![factor.clone()];
    let mut support = vec![];
    while let Some(g) = stack.pop() {
        if g.len() <= 1 {
            continue;
        }
        if g.len() == 2 {
            let x = e.neg(&e.mul(&g[0], &e.inv(&g[1])?)?);
            if !x.b.is_zero() {
                return Err("nonreal root after Frobenius restriction".into());
            }
            support.push(x);
            continue;
        }
        let mut split = None;
        for shift in 0..64 {
            let line = [c(shift), c(1)];
            let h0 = e.gcd(&g, &line)?;
            if h0.len() > 1 && h0.len() < g.len() {
                split = Some(h0);
                break;
            }
            let y = powmod(e, &line, &((&e.f.p - 1u32) >> 1), &g)?;
            let h = e.gcd(&g, &e.psub(&y, &[c(1)]))?;
            if h.len() > 1 && h.len() < g.len() {
                split = Some(h);
                break;
            }
        }
        let h = split.ok_or("split64 cap: incomplete, not absence")?;
        let (other, rem) = e.divrem(&g, &h)?;
        if !rem.is_empty() {
            return Err("split reconstruction".into());
        }
        stack.push(h);
        stack.push(other);
    }
    support.sort_by(|a, b| a.a.cmp(&b.a));
    let mut reconstructed = vec![c(1)];
    let mut out = vec![];
    for x in support {
        let line = [e.neg(&x), c(1)];
        reconstructed = e.product(&reconstructed, &line)?;
        let mut remaining = p.clone();
        let mut multiplicity = 0;
        while remaining.len() >= 2 {
            let (q, r) = e.divrem(&remaining, &line)?;
            if !r.is_empty() {
                break;
            }
            multiplicity += 1;
            remaining = q;
        }
        if multiplicity == 0 {
            return Err("root multiplicity".into());
        }
        out.push((x.a, multiplicity));
    }
    if reconstructed != factor || !e.divrem(&p, &factor)?.1.is_empty() {
        return Err("complete Fp support reconstruction".into());
    }
    let receipt = json!({"polynomial":encoded(&p),"fp_root_factor":encoded(&factor),"roots":out.iter().map(|(x,m)|json!({"x":x.to_string(),"multiplicity":m})).collect::<Vec<_>>(),"complete_fp_support":true});
    Ok((out, receipt))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fp_support_keeps_multiplicity_and_rejects_nonsquare_factor() {
        let e = Engine::new(101u32.into(), 1_000_000).unwrap();
        let a = [c(100), c(1)];
        let b = [c(99), c(1)];
        let ns = [c(99), c(0), c(1)];
        let p = e
            .product(&e.product(&e.product(&a, &a).unwrap(), &b).unwrap(), &ns)
            .unwrap();
        let (xs, _) = roots(&e, &p).unwrap();
        assert_eq!(xs, vec![(1u32.into(), 2), (2u32.into(), 1)]);
        assert!(roots(&e, &[]).is_err());
        assert!(roots(&e, &[c(101)]).is_err());
        assert!(roots(&e, &[c(1)]).unwrap().0.is_empty());
    }
}
