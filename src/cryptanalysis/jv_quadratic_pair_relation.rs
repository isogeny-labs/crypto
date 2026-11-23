//! Bounded quartic splitting and checked zero/one/two-quadratic principal rows.
//! Existing single-remainder certificates and entrypoints remain unchanged.
use super::{principal_relation as single, quadratic_pair_graph::LargeTerm};
use crate::cryptanalysis::{
    jv_bitops as work,
    jv_cover::{BaseEl, Fld, Fq, Hyp, E2},
    jv_sieve::{FpPoly, FpRing},
    residual_walk::{is_prime_u64, pow_mod},
};
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};
pub use single::Remainder as QuadraticClass;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SplitOptions {
    pub max_attempts: u8,
}
impl Default for SplitOptions {
    fn default() -> Self {
        Self { max_attempts: 8 }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SplitAttempt {
    pub polynomial: [u64; 4],
    pub gcd: FpPoly,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SplitReport {
    pub p: u64,
    pub max_attempts: u8,
    pub normalized: FpPoly,
    pub squarefree_gcd: FpPoly,
    pub linear_gcd: Option<FpPoly>,
    pub frobenius_p: Option<FpPoly>,
    pub frobenius_p2: Option<FpPoly>,
    pub attempts: Vec<SplitAttempt>,
    pub factors: Option<[FpPoly; 2]>,
    pub refusal: Option<String>,
}
fn mul_mod(a: u64, b: u64, p: u64) -> u64 {
    work::hook_mul(p);
    ((a as u128 * b as u128) % p as u128) as u64
}
pub fn is_irreducible_quadratic(ring: &FpRing, u: &[u64]) -> bool {
    if u.len() != 3 || u[2] != 1 || u.iter().any(|&x| x >= ring.p) {
        return false;
    }
    let p = ring.p;
    let a = mul_mod(u[1], u[1], p);
    let b = mul_mod(4 % p, u[0], p);
    work::hook_add(p);
    let discriminant = (a + p - b) % p;
    discriminant != 0 && pow_mod(discriminant, (p - 1) / 2, p) == p - 1
}
fn split_with_draws(
    ring: &FpRing,
    tail: &[u64],
    options: SplitOptions,
    mut draw: impl FnMut() -> [u64; 4],
) -> Result<SplitReport, &'static str> {
    if !(5..=1511).contains(&ring.p)
        || !is_prime_u64(ring.p)
        || !(1..=16).contains(&options.max_attempts)
        || tail.len() != 5
        || tail[4] == 0
        || tail.iter().any(|&x| x >= ring.p)
    {
        return Err("odd small prime/canonical quartic/bounded attempts required");
    }
    let normalized = ring.monic(tail);
    let squarefree_gcd = ring.gcd(&normalized, &ring.deriv(&normalized));
    let mut report = SplitReport {
        p: ring.p,
        max_attempts: options.max_attempts,
        normalized: normalized.clone(),
        squarefree_gcd,
        linear_gcd: None,
        frobenius_p: None,
        frobenius_p2: None,
        attempts: vec![],
        factors: None,
        refusal: None,
    };
    if report.squarefree_gcd != vec![1] {
        report.refusal = Some("repeated quartic factor".into());
        return Ok(report);
    }
    let xp = ring.powmod_x(ring.p as u128, &normalized);
    let linear_gcd = ring.gcd(&normalized, &ring.sub(&xp, &[0, 1]));
    report.frobenius_p = Some(xp.clone());
    report.linear_gcd = Some(linear_gcd.clone());
    if linear_gcd != vec![1] {
        report.refusal = Some("quartic has Fp linear support".into());
        return Ok(report);
    }
    let xp2 = ring.powmod(&xp, ring.p as u128, &normalized);
    report.frobenius_p2 = Some(xp2.clone());
    if xp2 != vec![0, 1] {
        report.refusal = Some("quartic is not a product of degree-two factors".into());
        return Ok(report);
    }
    let exponent = (ring.p as u128 * ring.p as u128 - 1) / 2;
    for _ in 0..options.max_attempts {
        let r = draw();
        if r.iter().any(|&x| x >= ring.p) {
            return Err("split draw outside field");
        }
        work::words(6);
        let powered = ring.powmod(&r, exponent, &normalized);
        let factor = ring.gcd(&normalized, &ring.sub(&powered, &[1]));
        report.attempts.push(SplitAttempt {
            polynomial: r,
            gcd: factor.clone(),
        });
        if factor.len() != 3 {
            continue;
        }
        let (other, remainder) = ring.divrem(&normalized, &factor);
        if !remainder.is_empty() {
            return Err("quartic gcd did not divide exactly");
        }
        let mut factors = [factor, other];
        factors.sort();
        if factors[0] == factors[1]
            || !factors.iter().all(|u| is_irreducible_quadratic(ring, u))
            || ring.mul(&factors[0], &factors[1]) != normalized
        {
            return Err("split factor identity or irreducibility");
        }
        report.factors = Some(factors);
        return Ok(report);
    }
    report.refusal = Some("quartic split attempt cap".into());
    Ok(report)
}
/// All random draws use the existing counted modular sampler. Structural
/// refusals precede sampling; failed numerical attempts remain in the report.
pub fn split_quartic(
    ring: &FpRing,
    tail: &[u64],
    options: SplitOptions,
    rng: &mut StdRng,
) -> Result<SplitReport, &'static str> {
    split_with_draws(ring, tail, options, || {
        core::array::from_fn(|_| work::uniform_mod_u64(rng, 0, ring.p))
    })
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Certificate {
    pub terms: Vec<(usize, i8)>,
    pub remainders: Vec<QuadraticClass>,
    pub a: Vec<[u64; 2]>,
    pub c: Vec<[u64; 2]>,
    pub norm: Vec<u64>,
}
#[derive(Clone, Debug)]
pub struct Verified {
    certificate: Certificate,
}
impl Verified {
    pub fn certificate(&self) -> &Certificate {
        &self.certificate
    }
    pub fn into_certificate(self) -> Certificate {
        self.certificate
    }
    pub fn large_terms(&self) -> Vec<LargeTerm> {
        self.certificate
            .remainders
            .iter()
            .map(|r| LargeTerm {
                class: r.key,
                coefficient: r.coefficient,
            })
            .collect()
    }
}
fn canonical(f: &Fq, u: &[E2], v: &[E2]) -> QuadraticClass {
    let a = v.first().copied().unwrap_or(E2::ZERO);
    let b = v.get(1).copied().unwrap_or(E2::ZERO);
    let na = f.neg(&a);
    let nb = f.neg(&b);
    let key = [u[0].0[0], u[1].0[0], a.0[0], a.0[1], b.0[0], b.0[1]];
    let neg = [u[0].0[0], u[1].0[0], na.0[0], na.0[1], nb.0[0], nb.0[1]];
    work::words(18);
    if neg < key {
        QuadraticClass {
            key: neg,
            coefficient: -1,
        }
    } else {
        QuadraticClass {
            key,
            coefficient: 1,
        }
    }
}
/// The curve/field/base must already be admitted by the caller. This verifies
/// exact principal support and irreducible monic cofactor polynomials, without
/// requiring or exposing any discrete logarithm.
#[allow(clippy::too_many_arguments)] // Explicit mathematical certificate inputs.
pub fn check_equation(
    f: &Fq,
    h: &[E2],
    base: &[BaseEl],
    a: &[E2],
    c: &[E2],
    norm: &[u64],
    terms: &[(usize, i8)],
    factors: &[FpPoly],
) -> Result<Verified, &'static str> {
    let ring = FpRing::new(f.p);
    if factors.len() > 2
        || factors.iter().any(|u| !is_irreducible_quadratic(&ring, u))
        || factors.windows(2).any(|w| w[0] >= w[1])
        || norm.len() != 10
        || norm[9] == 0
        || norm.iter().any(|&x| x >= f.p)
        || terms.len() + 2 * factors.len() != 9
    {
        return Err("canonical principal/cofactor support shape");
    }
    let mut tail = vec![norm[9]];
    for factor in factors {
        tail = ring.mul(&tail, factor);
    }
    if factors.len() < 2 {
        let old = single::check_equation(f, h, base, a, c, norm, terms, &tail)?.into_certificate();
        return Ok(Verified {
            certificate: Certificate {
                terms: old.terms,
                remainders: old.remainder.into_iter().collect(),
                a: old.a,
                c: old.c,
                norm: old.norm,
            },
        });
    }
    if h.len() != 8
        || h.last() == Some(&E2::ZERO)
        || a.len() > 5
        || c.len() != 2
        || c[1] == E2::ZERO
        || terms.windows(2).any(|w| w[0].0 >= w[1].0)
        || h.iter()
            .chain(a)
            .chain(c)
            .any(|x| x.0.iter().any(|&v| v >= f.p))
    {
        return Err("degree/canonical principal input");
    }
    let actual = single::sub(
        f,
        &single::mul(f, a, a),
        &single::mul(f, &single::mul(f, c, c), h),
    );
    if actual != single::embed(norm) {
        return Err("principal norm identity");
    }
    let mut product = vec![E2::ONE];
    let mut previous_x = None;
    for &(index, sign) in terms {
        let point = base.get(index).ok_or("small point index")?;
        if ![-1, 1].contains(&sign)
            || point.x >= f.p
            || point.y == E2::ZERO
            || point.y.0.iter().any(|&x| x >= f.p)
            || previous_x.is_some_and(|x| point.x <= x)
        {
            return Err("distinct canonical small support");
        }
        previous_x = Some(point.x);
        let y = if sign == 1 { point.y } else { f.neg(&point.y) };
        let den = single::eval(f, c, point.x);
        if den == E2::ZERO
            || f.sq(&y) != single::eval(f, h, point.x)
            || f.add(&single::eval(f, a, point.x), &f.mul(&den, &y)) != E2::ZERO
        {
            return Err("point/ordinate/principal binding");
        }
        product = single::mul(f, &product, &[f.neg(&f.from_fp(point.x)), E2::ONE]);
        work::words(7);
    }
    if single::mul(f, &product, &single::embed(&tail)) != single::embed(norm) {
        return Err("complete distinct support factorization");
    }
    let mut remainders = Vec::new();
    for factor in factors {
        let u = single::embed(factor);
        let inverse = single::inverse_linear(f, c, &u)?;
        let v = single::modulo(f, &single::mul(f, &single::modulo(f, a, &u)?, &inverse), &u)?
            .iter()
            .map(|x| f.neg(x))
            .collect::<Vec<_>>();
        if !single::modulo(f, &single::sub(f, &single::mul(f, &v, &v), h), &u)?.is_empty() {
            return Err("Mumford remainder identity");
        }
        remainders.push(canonical(f, &u, &v));
    }
    Ok(Verified {
        certificate: Certificate {
            terms: terms.to_vec(),
            remainders,
            a: single::encode(a),
            c: single::encode(c),
            norm: norm.to_vec(),
        },
    })
}
/// Separate direct Cantor replay of a serialized certificate's group identity.
/// Algebraic norm/canonical binding still requires check_equation or an auditor.
pub fn replay_group(jac: &Hyp<Fq>, base: &[BaseEl], row: &Certificate) -> bool {
    if row.remainders.len() > 2
        || row.terms.len() + 2 * row.remainders.len() != 9
        || row.terms.windows(2).any(|w| w[0].0 >= w[1].0)
        || row.remainders.windows(2).any(|w| w[0].key >= w[1].key)
    {
        return false;
    }
    let mut acc = jac.identity();
    for &(index, sign) in &row.terms {
        let Some(point) = base.get(index) else {
            return false;
        };
        if ![-1, 1].contains(&sign) || !jac.is_valid(&point.d) {
            return false;
        }
        let d = if sign == 1 {
            point.d.clone()
        } else {
            jac.neg(&point.d)
        };
        acc = jac.add(&acc, &d);
    }
    for r in &row.remainders {
        if ![-1, 1].contains(&r.coefficient) || r.key.iter().any(|&x| x >= jac.f.p) {
            return false;
        }
        let mut d = r.divisor();
        if !jac.is_valid(&d) {
            return false;
        }
        if r.coefficient == -1 {
            d = jac.neg(&d);
        }
        acc = jac.add(&acc, &d);
    }
    jac.is_identity(&acc)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_split_keeps_all_unsuccessful_attempts() {
        let ring = FpRing::new(101);
        let tail = ring.mul(&[98, 0, 1], &[99, 0, 1]);
        let report = split_with_draws(&ring, &tail, SplitOptions::default(), || [0; 4]).unwrap();
        assert!(report.factors.is_none());
        assert_eq!(report.attempts.len(), 8);
        assert!(report
            .attempts
            .iter()
            .all(|a| a.polynomial == [0; 4] && a.gcd == vec![1]));
        assert_eq!(report.refusal.as_deref(), Some("quartic split attempt cap"));
    }
    #[test]
    fn structural_rejections_do_not_draw() {
        let ring = FpRing::new(101);
        let repeated = ring.mul(&[99, 0, 1], &[99, 0, 1]);
        let linear = ring.mul(&[99, 0, 1], &ring.mul(&[100, 1], &[99, 1]));
        for tail in [repeated, linear, vec![2, 0, 0, 0, 1]] {
            let report = split_with_draws(&ring, &tail, SplitOptions::default(), || {
                panic!("no EDF draw")
            })
            .unwrap();
            assert!(report.factors.is_none());
            assert!(report.attempts.is_empty());
            assert!(report.refusal.is_some());
        }
        assert!(split_with_draws(
            &ring,
            &[1, 0, 0, 0, 1],
            SplitOptions { max_attempts: 0 },
            || panic!("bad cap")
        )
        .is_err());
        assert!(split_with_draws(
            &ring,
            &[1, 0, 0, 0, 101],
            SplitOptions::default(),
            || panic!("bad field coefficient")
        )
        .is_err());
    }
}
