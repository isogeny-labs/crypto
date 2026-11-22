//! Explicit prime-group transport for the actual Curve25519 cofactor.
use crate::Result;
use num_bigint::BigUint;
use torus_s4::{curve25519_prime, decimal, Curve, Point};
pub fn ell() -> BigUint {
    (BigUint::from(1u32) << 252usize)
        + decimal("27742317777372353535851937790883648493").expect("constant")
}
pub fn scalar(curve: &Curve, p: &Point, n: &BigUint) -> Result<Point> {
    let mut r = None;
    let mut a = p.clone();
    for i in 0..n.bits() {
        if n.bit(i) {
            r = curve.add(&r, &a)?;
        }
        if i + 1 < n.bits() {
            a = curve.add(&a, &a)?;
        }
    }
    Ok(r)
}
pub struct Transport {
    pub prime_target: Point,
    pub torsion_generator: Point,
    pub raw_sections: Vec<Point>,
}
pub fn sections(curve: &Curve, raw_target: &Point) -> Result<Transport> {
    if !curve.contains(raw_target) {
        return Err("invalid raw target".into());
    }
    let q = curve.times(raw_target, 8)?;
    let result = sections_for_prime(curve, &q)?;
    if !result.raw_sections.contains(raw_target) {
        return Err("known raw target is absent from the eight-section fiber".into());
    }
    Ok(result)
}
pub fn sections_for_prime(curve: &Curve, prime_target: &Point) -> Result<Transport> {
    if curve.f.p != curve25519_prime()
        || curve.a != BigUint::from(486662u32)
        || !curve.contains(prime_target)
    {
        return Err("transport requires actual Curve25519".into());
    }
    let q = prime_target.clone();
    if q.is_none() {
        return Err("prime target is identity".into());
    }
    let ell = ell();
    if scalar(curve, &q, &ell)?.is_some() {
        return Err("cofactor-cleared target failed subgroup check".into());
    }
    let inverse = (BigUint::from(3u32) * &ell + 1u32) / 8u32;
    if (BigUint::from(8u32) * &inverse) % &ell != BigUint::from(1u32) {
        return Err("wrong inverse of cofactor".into());
    }
    let start = scalar(curve, &q, &inverse)?;
    let mut generator = None;
    for u in 2u32..=256 {
        let u = BigUint::from(u);
        if let Some(v) = curve.f.sqrt(&curve.rhs(&u))? {
            let t = scalar(curve, &Some((u, v)), &ell)?;
            if curve.times(&t, 8)?.is_none() && curve.times(&t, 4)?.is_some() {
                generator = t;
                break;
            }
        }
    }
    if generator.is_none() {
        return Err("bounded torsion-generator search exhausted".into());
    }
    let mut raw_sections = vec![];
    let mut r = start;
    for _ in 0..8 {
        if !curve.contains(&r) || curve.times(&r, 8)? != q || raw_sections.contains(&r) {
            return Err("invalid cofactor fiber".into());
        }
        raw_sections.push(r.clone());
        r = curve.add(&r, &generator)?;
    }
    Ok(Transport {
        prime_target: q,
        torsion_generator: generator,
        raw_sections,
    })
}
