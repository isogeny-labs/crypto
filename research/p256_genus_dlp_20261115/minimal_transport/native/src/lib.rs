//! Source candidate. Numerical qualification requires the repository checkpoint
//! and the user-selected RunPod route; compilation does not qualify its output.
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{One, Signed, Zero};
use p256_folded_walk::{solve, toy_fixture, Config, EllipticGroup, Group, MixedGroup};
use p256_genus_check::{sub, transfer, Divisor, Point};
use p256_transfer_search::Curve;
use serde_json::{json, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Eisenstein {
    a: BigInt,
    b: BigInt,
}
impl Eisenstein {
    fn norm(&self) -> BigInt {
        &self.a * &self.a - &self.a * &self.b + &self.b * &self.b
    }
    fn mul(&self, y: &Self) -> Self {
        Self {
            a: &self.a * &y.a - &self.b * &y.b,
            b: &self.a * &y.b + &self.b * &y.a - &self.b * &y.b,
        }
    }
    fn conjugate(&self) -> Self {
        Self {
            a: &self.a - &self.b,
            b: -&self.b,
        }
    }
    fn rotate(&self) -> Self {
        Self {
            a: -&self.b,
            b: &self.a - &self.b,
        }
    }
    fn neg(&self) -> Self {
        Self {
            a: -&self.a,
            b: -&self.b,
        }
    }
}

/// Nearest integer, ties toward positive infinity; denominator is positive.
fn nearest(x: &BigInt, denominator: &BigInt) -> BigInt {
    let (q, r) = x.div_mod_floor(denominator);
    if 2u32 * r >= *denominator {
        q + 1u32
    } else {
        q
    }
}

#[derive(Clone, Debug)]
pub struct ShortVector {
    pub a: BigInt,
    pub b: BigInt,
    pub order: BigUint,
    pub lambda: BigUint,
    pub euclidean_steps: usize,
}
impl ShortVector {
    pub fn norm(&self) -> BigInt {
        &self.a * &self.a - &self.a * &self.b + &self.b * &self.b
    }
    pub fn validate(&self) -> Result<(), String> {
        let n = BigInt::from(self.order.clone());
        if self.order <= 3u32.into() || self.order.is_even() {
            return Err("order must be odd and greater than three".into());
        }
        if self.lambda <= BigUint::one()
            || self.lambda >= self.order
            || (&self.lambda * &self.lambda + &self.lambda + 1u32) % &self.order != BigUint::zero()
        {
            return Err("invalid canonical nontrivial cubic eigenvalue".into());
        }
        if self.a <= BigInt::zero() || self.b >= BigInt::zero() {
            return Err("vector is outside the chosen opposite-sign sector".into());
        }
        if self.norm() != n {
            return Err("norm is not exactly the supplied order".into());
        }
        if (&self.b + BigInt::from(self.lambda.clone()) * &self.a).mod_floor(&n) != BigInt::zero() {
            return Err("wrong finite eigenline".into());
        }
        let recovery = 2u32 * &self.a - &self.b;
        if self.a.gcd(&n) != BigInt::one() || recovery.gcd(&n) != BigInt::one() {
            return Err("noninvertible subgroup scaling or recovery multiplier".into());
        }
        Ok(())
    }
}

/// Eisenstein Euclid for the ideal (n, omega-lambda^2).
/// Prime order is a caller hypothesis, not inferred from a square root.
pub fn short_vector(order: &BigUint, lambda: &BigUint) -> Result<ShortVector, String> {
    if *order <= 3u32.into()
        || order.is_even()
        || *lambda <= BigUint::one()
        || lambda >= order
        || (lambda * lambda + lambda + 1u32) % order != BigUint::zero()
    {
        return Err("invalid order/eigenvalue input".into());
    }
    let mut x = Eisenstein {
        a: order.clone().into(),
        b: BigInt::zero(),
    };
    let mut y = Eisenstein {
        a: -BigInt::from(lambda * lambda % order),
        b: BigInt::one(),
    };
    let mut steps = 0;
    while !y.norm().is_zero() {
        if steps >= 1024 {
            return Err("Eisenstein Euclid budget exhausted".into());
        }
        let denominator = y.norm();
        if !denominator.is_positive() {
            return Err("nonpositive norm".into());
        }
        let numerator = x.mul(&y.conjugate());
        let q = Eisenstein {
            a: nearest(&numerator.a, &denominator),
            b: nearest(&numerator.b, &denominator),
        };
        let product = q.mul(&y);
        let r = Eisenstein {
            a: &x.a - product.a,
            b: &x.b - product.b,
        };
        if r.norm() >= denominator {
            return Err("Euclidean norm did not decrease".into());
        }
        x = y;
        y = r;
        steps += 1;
    }
    if x.norm() != BigInt::from(order.clone()) {
        return Err("ideal gcd has unexpected norm; no candidate accepted".into());
    }
    for _ in 0..3 {
        for unit in [x.clone(), x.neg()] {
            if unit.a.is_positive() && unit.b.is_negative() {
                let result = ShortVector {
                    a: unit.a,
                    b: unit.b,
                    order: order.clone(),
                    lambda: lambda.clone(),
                    euclidean_steps: steps,
                };
                result.validate()?;
                return Ok(result);
            }
        }
        x = x.rotate();
    }
    Err("no opposite-sign unit representative".into())
}

pub struct ShortMap {
    pub mixed: MixedGroup,
    pub vector: ShortVector,
}
impl ShortMap {
    pub fn new(curve: Curve, order: &BigUint) -> Result<Self, String> {
        let mixed = MixedGroup::new(curve, order);
        let vector = short_vector(order, &mixed.eigenvalue)?;
        Ok(Self { mixed, vector })
    }
    fn signed_source_mul(&self, k: &BigInt, p: &Point) -> Point {
        let point = self.mixed.curve.mul(k.magnitude(), p);
        if k.sign() == Sign::Minus {
            point.map(|(x, y)| (x, sub(&BigUint::zero(), &y, &self.mixed.curve.p)))
        } else {
            point
        }
    }
    pub fn image(&self, p: &Point) -> Divisor {
        let left = transfer(
            3,
            &self.signed_source_mul(&self.vector.a, p),
            &self.mixed.curve.p,
        );
        let right = self.mixed.rotate(&transfer(
            3,
            &self.signed_source_mul(&self.vector.b, p),
            &self.mixed.curve.p,
        ));
        self.mixed.add(&left, &right)
    }
}
impl Group for ShortMap {
    type Element = Divisor;
    fn zero(&self) -> Divisor {
        self.mixed.zero()
    }
    fn add(&self, a: &Divisor, b: &Divisor) -> Divisor {
        self.mixed.add(a, b)
    }
    fn neg(&self, a: &Divisor) -> Divisor {
        self.mixed.neg(a)
    }
    fn encode(&self, a: &Divisor) -> Vec<u8> {
        self.mixed.encode(a)
    }
    fn rotate(&self, a: &Divisor) -> Divisor {
        self.mixed.rotate(a)
    }
    fn lambda(&self) -> BigUint {
        self.mixed.lambda()
    }
    fn rotations(&self) -> usize {
        self.mixed.rotations()
    }
    fn valid(&self, a: &Divisor) -> bool {
        self.mixed.valid(a)
    }
}

/// Planned numerical qualification. Do not call before checkpoint admission.
pub fn qualification() -> Result<Value, String> {
    let mut rows = vec![];
    let mut paired_cases = 0;
    for field in [1009u64, 4093, 16381, 0] {
        let (curve, n, g, source_field) = if field == 0 {
            let (p, b, n, g) = p256_genus_check::parameters();
            (
                Curve {
                    a: &p - 3u32,
                    p: p.clone(),
                    b,
                },
                n,
                g,
                p.to_string(),
            )
        } else {
            let (curve, n, g, _) = toy_fixture(field).ok_or("frozen toy fixture unavailable")?;
            let expected = match field {
                1009 => 109u64,
                4093 => 823,
                _ => 4111,
            };
            if n != expected.into() {
                return Err("frozen toy subgroup changed".into());
            }
            (curve, n, g, field.to_string())
        };
        let source = EllipticGroup {
            curve: curve.clone(),
        };
        let short = ShortMap::new(curve.clone(), &n)?;
        let v = &short.vector;
        let image_g = short.image(&g);
        if image_g == short.zero() || short.mul(&n, &image_g) != short.zero() {
            return Err("image order check failed".into());
        }
        for scalar in [0u64, 1, 2, 37] {
            let q = source.mul(&(BigUint::from(scalar) % &n), &g);
            let image = short.image(&q);
            let scaled = curve.mul(v.a.magnitude(), &q);
            if image != short.mixed.image(&scaled)
                || !short.valid(&image)
                || short.rotate(&image) != short.mul(&v.lambda, &image)
            {
                return Err("map compatibility or eigenvalue check failed".into());
            }
        }
        if field != 0 {
            for scalar in [0u64, 1, 2, 17, 37, 43] {
                let k = BigUint::from(scalar) % &n;
                let q = source.mul(&k, &g);
                for seed in [7u64, 19, 20261115] {
                    let cfg = Config {
                        seed,
                        ..Config::default()
                    };
                    let a = solve(&source, &n, &g, &q, &cfg);
                    let b = solve(&short, &n, &image_g, &short.image(&q), &cfg);
                    if a.scalar != Some(k.to_string())
                        || b.scalar != Some(k.to_string())
                        || !a.scalar_replay_matches
                        || !b.scalar_replay_matches
                    {
                        return Err(format!(
                            "public replay failed: p={field}, seed={seed}, k={k}"
                        ));
                    }
                    paired_cases += 1;
                }
            }
        }
        let mut damaged = v.clone();
        damaged.b += 1u32;
        if damaged.validate().is_ok() {
            return Err("corrupt vector accepted".into());
        }
        rows.push(
            json!({"p":source_field,"n":n.to_string(),"lambda":v.lambda.to_string(),
            "a":v.a.to_string(),"b":v.b.to_string(),"norm":v.norm().to_string(),
            "a_bits":v.a.magnitude().bits(),"b_bits":v.b.magnitude().bits(),
            "euclidean_steps":v.euclidean_steps,"map_scalars_checked":[0,1,2,37]}),
        );
    }
    for (n, l) in [(3u64, 1u64), (19, 1), (19, 2)] {
        if short_vector(&n.into(), &l.into()).is_ok() {
            return Err("invalid parameters accepted".into());
        }
    }
    Ok(
        json!({"schema":"p256-minimal-transport-qualification/v1","rows":rows,
        "paired_public_toy_cases":paired_cases,"timing_measured":false,
        "complete_cost_measured":false,"standing":"producer checks only; no signed geometric or performance verification"}),
    )
}
