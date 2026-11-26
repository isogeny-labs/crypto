//! Index calculus for a given prime-order subgroup of an imaginary
//! hyperelliptic Jacobian over a prime field.
//!
//! Generate a factor base of low-degree places, mask each target combination
//! with a random linear combination of the base, and collect smooth reduced
//! divisors. A primitive integer dependence cancels the factor-base
//! coefficients exactly, including when the generator is divisible by its
//! order in the ambient Jacobian. The solver verifies the recovered scalar on
//! the original curve.
//!
//! The masks and integer elimination prioritize a direct implementation of
//! the growing-genus construction. The work limits are explicit and dense
//! storage is quadratic in the size of the factor base.

mod factor_base;
mod kernel;

use num_bigint::{BigInt, BigUint, RandBigInt};
use num_integer::Integer;
use num_traits::{One, ToPrimitive, Zero};
use rand::Rng;

use super::curve_traits::arith::is_prime;
use crate::prime_hyperelliptic::fp2::is_prime_u64;
use crate::prime_hyperelliptic::{HyperellipticCurveP, MumfordDivisorP, PrimeHyperellipticError};

use factor_base::{build, smooth_coordinates, FactorBaseError};
use kernel::primitive_dependency;

/// Resource limits and factor-base degree for one solve.
#[derive(Clone, Copy, Debug)]
pub struct OrderFreeIcConfig {
    /// Override the default degree cutoff with a value in 1..=genus.
    pub smooth_degree: Option<u32>,
    /// Maximum monic polynomials to enumerate when building the base, including
    /// reducible and inert candidates.
    pub max_base_polynomials: usize,
    /// Maximum random relation trials across all batches.
    pub max_relation_trials: usize,
}

impl Default for OrderFreeIcConfig {
    fn default() -> Self {
        Self {
            smooth_degree: None,
            max_base_polynomials: 256,
            max_relation_trials: 4_096,
        }
    }
}

/// A verified answer and the sizes of the collection stages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderFreeIcSolution {
    pub scalar: BigUint,
    pub factor_base_size: usize,
    pub relation_trials: usize,
}

/// Inputs outside the solver's domain or a run that exhausted its limits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrderFreeIcError {
    InvalidCurveOrDivisor(PrimeHyperellipticError),
    InvalidConfiguration,
    InvalidSubgroup,
    FactorBaseBudgetExceeded,
    FactorBaseRootSearchExhausted,
    EmptyFactorBase,
    RelationBudgetExceeded,
    RelationCancellationFailed,
    ScalarVerificationFailed,
}

impl From<PrimeHyperellipticError> for OrderFreeIcError {
    fn from(error: PrimeHyperellipticError) -> Self {
        Self::InvalidCurveOrDivisor(error)
    }
}

impl From<FactorBaseError> for OrderFreeIcError {
    fn from(error: FactorBaseError) -> Self {
        match error {
            FactorBaseError::BudgetExceeded => Self::FactorBaseBudgetExceeded,
            FactorBaseError::RootSearchExhausted => Self::FactorBaseRootSearchExhausted,
            FactorBaseError::InvalidDivisor(error) => error.into(),
        }
    }
}

fn default_smooth_degree(curve: &HyperellipticCurveP) -> u32 {
    let genus = f64::from(curve.genus);
    let log_p = (curve.p.bits() - 1) as f64 * std::f64::consts::LN_2;
    let split_place_cutoff = (256.0 * (genus + 1.0).powi(2)).ln() / log_p;
    let smoothness_cutoff = (genus * (4.0 * genus).ln() / log_p).sqrt();
    (split_place_cutoff.max(smoothness_cutoff).ceil() as u32).clamp(1, curve.genus)
}

/// Solve the discrete logarithm on a genus-at-least-two curve.
///
/// The caller supplies the prime subgroup order and a target in that
/// subgroup. Orders larger than a machine word get the repository's
/// probable-prime screen. The order of the full Jacobian is unnecessary.
/// A cutoff below the default can make the sampled base generate a proper
/// subgroup, so the caller may need to increase the degree or trial budget.
/// Every returned scalar is checked against the input target.
pub fn solve_order_free(
    curve: &HyperellipticCurveP,
    generator: &MumfordDivisorP,
    target: &MumfordDivisorP,
    order: &BigUint,
    config: OrderFreeIcConfig,
    rng: &mut impl Rng,
) -> Result<OrderFreeIcSolution, OrderFreeIcError> {
    if curve.genus < 2
        || config.max_base_polynomials == 0
        || config.max_relation_trials == 0
        || config
            .smooth_degree
            .is_some_and(|degree| degree == 0 || degree > curve.genus)
    {
        return Err(OrderFreeIcError::InvalidConfiguration);
    }
    if curve.p.to_usize().is_none() {
        return Err(OrderFreeIcError::FactorBaseBudgetExceeded);
    }
    curve.validate()?;
    generator.validate(curve)?;
    target.validate(curve)?;

    let upper_bound = (&curve.p + BigUint::one()).pow(curve.genus).pow(2);
    let prime_order =
        order <= &upper_bound && order.to_u64().map_or_else(|| is_prime(order), is_prime_u64);
    if !prime_order
        || generator.is_identity()
        || !generator.scalar_mul_fast(order, curve).is_identity()
        || !target.scalar_mul_fast(order, curve).is_identity()
    {
        return Err(OrderFreeIcError::InvalidSubgroup);
    }
    if target.is_identity() {
        return Ok(OrderFreeIcSolution {
            scalar: BigUint::zero(),
            factor_base_size: 0,
            relation_trials: 0,
        });
    }

    let degree = config
        .smooth_degree
        .unwrap_or_else(|| default_smooth_degree(curve));
    let base = build(curve, degree, config.max_base_polynomials, rng)?;
    if base.is_empty() {
        return Err(OrderFreeIcError::EmptyFactorBase);
    }
    let mask_bound = BigUint::from(base.len()) * 2u32 * upper_bound.pow(2);
    let mask_bits = mask_bound.bits();
    let needed = base.len() + 1;
    let modulus = BigInt::from(order.clone());
    let mut relations = Vec::new();
    let mut labels = Vec::new();

    for trial in 1..=config.max_relation_trials {
        let alpha = rng.gen_biguint_below(order);
        let beta = rng.gen_biguint_below(order);
        let mut candidate = generator
            .scalar_mul_fast(&alpha, curve)
            .add_fast(&target.scalar_mul_fast(&beta, curve), curve);
        let mut relation = Vec::with_capacity(base.len());
        for place in &base {
            let coefficient = rng.gen_biguint(mask_bits);
            candidate = candidate.add_fast(&place.scalar_mul_fast(&coefficient, curve), curve);
            relation.push(-BigInt::from(coefficient));
        }
        let Some(coordinates) = smooth_coordinates(&candidate, &base) else {
            continue;
        };
        for (entry, coordinate) in relation.iter_mut().zip(coordinates) {
            *entry += coordinate;
        }
        relations.push(relation);
        labels.push((BigInt::from(alpha), BigInt::from(beta)));

        if relations.len() < needed {
            continue;
        }

        let weights =
            primitive_dependency(&relations).ok_or(OrderFreeIcError::RelationCancellationFailed)?;
        let mut alpha_sum = BigInt::zero();
        let mut beta_sum = BigInt::zero();
        for (weight, (alpha, beta)) in weights.iter().zip(&labels) {
            alpha_sum += weight * alpha;
            beta_sum += weight * beta;
        }
        let beta = beta_sum.mod_floor(&modulus).to_biguint().unwrap();
        if beta.is_zero() {
            relations.clear();
            labels.clear();
            continue;
        }
        let numerator = (-alpha_sum).mod_floor(&modulus).to_biguint().unwrap();
        let inverse = beta.modpow(&(order - 2u32), order);
        let scalar = numerator * inverse % order;
        if generator.scalar_mul_fast(&scalar, curve) != *target {
            return Err(OrderFreeIcError::ScalarVerificationFailed);
        }
        return Ok(OrderFreeIcSolution {
            scalar,
            factor_base_size: base.len(),
            relation_trials: trial,
        });
    }

    Err(OrderFreeIcError::RelationBudgetExceeded)
}

#[cfg(test)]
mod tests;
