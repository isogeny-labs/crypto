//! Relation languages for the width comparison, all built by the same encoder
//! so that one thing changes at a time:
//!
//! * `GroupLawHash`: Bochum Lemma 11 with two hashed factor bases (m = 2, derived
//!   last slot), the crate's own decomposition language;
//! * `GroupLawSmallX`: the same group-law relation `A1 + A2 = R` with the
//!   small-x factor base `x < 2^l` (membership is free: the high bits are
//!   constant zero), the prime-field index-calculus convention;
//! * `SemaevS3SmallX`: Semaev's third summation polynomial `S3(x1, x2, x_R) = 0`
//!   on small-x abscissae, no ordinates and no curve equation (m = 2);
//! * `ChainedS3SmallX`: `S3(x1, x2, t) = 0` and `S3(t, x3, x_R) = 0` with one
//!   free intermediate abscissa `t` (m = 3), the sparse form of `S4`.
//!
//! Each builder returns the finished Boolean network; `accepts` evaluates it on
//! its free input bits through the built-in witness program, and `semaev_s3`
//! is the direct field evaluation used to check the gadget.

use crate::curve::{Curve, Point};
use crate::encoding::{Encoder, Multiplier, PointWords};
use crate::network::Network;
use crate::reduction::{decomposition_network_with_multiplier, FactorBase};
use crate::{invalid, BigUint, Meter, Result};
use num_traits::Zero;

/// The relation a network encodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Relation {
    /// Hashed group-law decomposition, m = 2.
    GroupLawHash,
    /// Group law with the small-x factor base, m = 2.
    GroupLawSmallX,
    /// Semaev S3 with the small-x factor base, m = 2.
    SemaevS3SmallX,
    /// Chained S3 (sparse S4) with the small-x factor base, m = 3.
    ChainedS3SmallX,
}

impl Relation {
    /// Every relation, in report order.
    pub const ALL: [Relation; 4] = [
        Relation::GroupLawHash,
        Relation::GroupLawSmallX,
        Relation::SemaevS3SmallX,
        Relation::ChainedS3SmallX,
    ];

    /// Stable name.
    pub fn name(self) -> &'static str {
        match self {
            Self::GroupLawHash => "grouplaw-hash",
            Self::GroupLawSmallX => "grouplaw-smallx",
            Self::SemaevS3SmallX => "semaev-s3-smallx",
            Self::ChainedS3SmallX => "chained-s3-smallx",
        }
    }

    /// Parse a stable name.
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|r| r.name() == name)
    }

    /// Number of factor-base summands.
    pub fn slots(self) -> usize {
        match self {
            Self::ChainedS3SmallX => 3,
            _ => 2,
        }
    }

    /// Whether the language uses hashed factor bases.
    pub fn uses_hash(self) -> bool {
        matches!(self, Self::GroupLawHash)
    }

    /// Free abscissa bits per slot for the small-x base, `ceil(width / m)`.
    pub fn low_bits(self, width: usize) -> usize {
        width.div_ceil(self.slots())
    }
}

fn affine_x(target: &Point) -> Result<&BigUint> {
    match target {
        Point::Affine { x, .. } => Ok(x),
        Point::Infinity => Err(invalid("target must be affine")),
    }
}

/// Bochum's language with the chosen multiplier gadget.
pub fn group_law_hash(
    curve: &Curve,
    bases: &[FactorBase],
    target: &Point,
    multiplier: Multiplier,
    meter: &mut Meter,
) -> Result<Network> {
    decomposition_network_with_multiplier(curve, bases, target, multiplier, meter)
}

/// `A1 + A2 = R` with `A1` a free small-x point and `A2 = R - A1` derived and
/// held to the small-x base. Free bits: `low_bits` of `x1`, then `y1`.
pub fn group_law_small_x(
    curve: &Curve,
    low_bits: usize,
    target: &Point,
    multiplier: Multiplier,
    meter: &mut Meter,
) -> Result<Network> {
    if !curve.in_subgroup(target, meter)? {
        return Err(invalid("target outside the subgroup"));
    }
    let width = curve.modulus().bits() as usize;
    let mut encoder = Encoder::new();
    encoder.set_multiplier(multiplier);
    let first = encoder.input_point_small_x(curve, low_bits, meter)?;
    let negated = PointWords {
        infinity: first.infinity.clone(),
        x: first.x.clone(),
        y: encoder.modular_subtract(
            &Encoder::constant(width, &BigUint::zero())?,
            &first.y,
            curve.modulus(),
            meter,
        )?,
    };
    let second = encoder.point_add(
        curve,
        &Encoder::constant_point(curve, target)?,
        &negated,
        meter,
    )?;
    encoder.assert_small_x(&second, low_bits, meter)?;
    encoder.finish(meter)
}

/// `S3(x1, x2, x_R) = 0` on two free small-x abscissae. Free bits: `low_bits`
/// of `x1`, then `low_bits` of `x2`.
pub fn semaev_s3_small_x(
    curve: &Curve,
    low_bits: usize,
    target: &Point,
    multiplier: Multiplier,
    meter: &mut Meter,
) -> Result<Network> {
    let width = curve.modulus().bits() as usize;
    let target_x = Encoder::constant(width, affine_x(target)?)?;
    let mut encoder = Encoder::new();
    encoder.set_multiplier(multiplier);
    let x1 = encoder.input_small_x(curve, low_bits, meter)?;
    let x2 = encoder.input_small_x(curve, low_bits, meter)?;
    encoder.semaev_s3_zero(curve, &x1, &x2, &target_x, meter)?;
    encoder.finish(meter)
}

/// `S3(x1, x2, t) = 0` and `S3(t, x3, x_R) = 0` on three free small-x
/// abscissae and one free field-width intermediate `t`. Free bits: `low_bits`
/// of `x1`, `x2`, `x3`, then the `width` bits of `t`.
pub fn chained_s3_small_x(
    curve: &Curve,
    low_bits: usize,
    target: &Point,
    multiplier: Multiplier,
    meter: &mut Meter,
) -> Result<Network> {
    let width = curve.modulus().bits() as usize;
    let target_x = Encoder::constant(width, affine_x(target)?)?;
    let mut encoder = Encoder::new();
    encoder.set_multiplier(multiplier);
    let x1 = encoder.input_small_x(curve, low_bits, meter)?;
    let x2 = encoder.input_small_x(curve, low_bits, meter)?;
    let x3 = encoder.input_small_x(curve, low_bits, meter)?;
    let t = encoder.input_word(width, meter)?;
    encoder.semaev_s3_zero(curve, &x1, &x2, &t, meter)?;
    encoder.semaev_s3_zero(curve, &t, &x3, &target_x, meter)?;
    encoder.finish(meter)
}

/// Build the named relation. `bases` is used by `GroupLawHash` only.
pub fn build(
    relation: Relation,
    curve: &Curve,
    bases: &[FactorBase],
    target: &Point,
    multiplier: Multiplier,
    meter: &mut Meter,
) -> Result<Network> {
    let width = curve.modulus().bits() as usize;
    let low_bits = relation.low_bits(width);
    match relation {
        Relation::GroupLawHash => group_law_hash(curve, bases, target, multiplier, meter),
        Relation::GroupLawSmallX => group_law_small_x(curve, low_bits, target, multiplier, meter),
        Relation::SemaevS3SmallX => semaev_s3_small_x(curve, low_bits, target, multiplier, meter),
        Relation::ChainedS3SmallX => chained_s3_small_x(curve, low_bits, target, multiplier, meter),
    }
}

/// Direct field evaluation of Semaev's third summation polynomial for
/// `y^2 = x^3 + a x + b`, reduced into `[0, p)`.
pub fn semaev_s3(curve: &Curve, x1: &BigUint, x2: &BigUint, x3: &BigUint) -> BigUint {
    let p = curve.modulus();
    let a = curve.a();
    let b = curve.b();
    let sub = |u: &BigUint, v: &BigUint| (u + p - v % p) % p;
    let difference = sub(x1, x2);
    let term1 = &difference * &difference % p * x3 % p * x3 % p;
    let sum = (x1 + x2) % p;
    let product = x1 * x2 % p;
    let inner = (&sum * ((&product + a) % p) % p + (b << 1)) % p;
    let term2 = (inner * x3 % p) << 1;
    let product_minus_a = sub(&product, a);
    let term3 = &product_minus_a * &product_minus_a % p;
    let term4 = (b << 2) % p * &sum % p;
    let positive = (term1 + term3) % p;
    let negative = (term2 + term4) % p;
    sub(&positive, &negative)
}

/// Little-endian bits of a value.
pub fn bits_of(value: &BigUint, width: usize) -> Vec<bool> {
    (0..width).map(|i| value.bit(i as u64)).collect()
}

/// Whether the network accepts the full assignment determined by these free
/// input bits (through the built-in witness program and an independent
/// evaluation of every vertex).
pub fn accepts(network: &Network, bits: &[bool], meter: &mut Meter) -> Result<bool> {
    match network.witness_from_input_bits(bits, meter)? {
        Some(assignment) => network.evaluate(&assignment, meter),
        None => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reduction::HashSeed;
    use crate::Limits;

    fn meter() -> Meter {
        Meter::new(Limits {
            steps: 2_000_000_000,
            vector: 1 << 26,
            bits: 1 << 24,
        })
    }

    fn curve17(meter: &mut Meter) -> Curve {
        Curve::new(
            BigUint::from(17u8),
            BigUint::from(2u8),
            BigUint::from(2u8),
            BigUint::from(19u8),
            Point::Affine {
                x: BigUint::from(5u8),
                y: BigUint::from(1u8),
            },
            meter,
        )
        .unwrap()
    }

    fn affine_points(curve: &Curve, meter: &mut Meter) -> Vec<Point> {
        let p = curve.modulus();
        let mut points = Vec::new();
        for x in 0..17u8 {
            for y in 0..17u8 {
                let point = Point::Affine {
                    x: BigUint::from(x),
                    y: BigUint::from(y),
                };
                if curve.contains(&point) {
                    points.push(point);
                }
            }
        }
        assert_eq!(points.len(), 18);
        assert!(p.bits() == 5);
        let _ = meter;
        points
    }

    fn xy(point: &Point) -> (BigUint, BigUint) {
        match point {
            Point::Affine { x, y } => (x.clone(), y.clone()),
            Point::Infinity => panic!("affine expected"),
        }
    }

    #[test]
    fn direct_s3_vanishes_exactly_on_sums_and_differences() {
        let mut meter = meter();
        let curve = curve17(&mut meter);
        let points = affine_points(&curve, &mut meter);
        let mut checked = 0;
        for p1 in &points {
            for p2 in &points {
                let (x1, _) = xy(p1);
                let (x2, _) = xy(p2);
                if x1 == x2 {
                    continue;
                }
                let sum = curve.add(p1, p2, &mut meter).unwrap();
                let difference = curve
                    .add(p1, &curve.negate(p2).unwrap(), &mut meter)
                    .unwrap();
                let (xs, _) = xy(&sum);
                let (xd, _) = xy(&difference);
                assert!(semaev_s3(&curve, &x1, &x2, &xs).is_zero());
                assert!(semaev_s3(&curve, &x1, &x2, &xd).is_zero());
                let roots = (0..17u8)
                    .filter(|&x| semaev_s3(&curve, &x1, &x2, &BigUint::from(x)).is_zero())
                    .count();
                assert!(roots <= 2);
                checked += 1;
            }
        }
        assert!(checked > 200);
    }

    fn small_x_points(curve: &Curve, low_bits: usize, meter: &mut Meter) -> Vec<Point> {
        affine_points(curve, meter)
            .into_iter()
            .filter(|point| xy(point).0 < (BigUint::from(1u8) << low_bits))
            .collect()
    }

    #[test]
    fn s3_gadget_matches_direct_evaluation_for_both_multipliers() {
        let mut meter = meter();
        let curve = curve17(&mut meter);
        let low_bits = 2;
        let small = small_x_points(&curve, low_bits, &mut meter);
        let target = curve.add(&small[0], &small[2], &mut meter).unwrap();
        let target_x = xy(&target).0;
        let mut arities = Vec::new();
        for multiplier in [Multiplier::Fourier, Multiplier::Schoolbook] {
            let network =
                semaev_s3_small_x(&curve, low_bits, &target, multiplier, &mut meter).unwrap();
            assert_eq!(network.input_bit_count(), Some(2 * low_bits));
            arities.push(network.maximum_degree());
            let mut positives = 0;
            for x1 in 0..4u8 {
                for x2 in 0..4u8 {
                    let mut bits = bits_of(&BigUint::from(x1), low_bits);
                    bits.extend(bits_of(&BigUint::from(x2), low_bits));
                    let accepted = accepts(&network, &bits, &mut meter).unwrap();
                    let expected =
                        semaev_s3(&curve, &BigUint::from(x1), &BigUint::from(x2), &target_x)
                            .is_zero();
                    assert_eq!(accepted, expected, "{multiplier:?} x1={x1} x2={x2}");
                    positives += usize::from(accepted);
                }
            }
            assert!(positives >= 1);
        }
        assert!(arities[0] > arities[1], "{arities:?}");
        assert!(arities[1] <= 8, "{arities:?}");
    }

    #[test]
    fn group_law_small_x_matches_direct_count() {
        let mut meter = meter();
        let curve = curve17(&mut meter);
        let low_bits = 2;
        let small = small_x_points(&curve, low_bits, &mut meter);
        let target = curve.add(&small[0], &small[2], &mut meter).unwrap();
        let bound = BigUint::from(1u8) << low_bits;
        for multiplier in [Multiplier::Fourier, Multiplier::Schoolbook] {
            let network =
                group_law_small_x(&curve, low_bits, &target, multiplier, &mut meter).unwrap();
            assert_eq!(network.input_bit_count(), Some(low_bits + 5));
            let mut positives = 0;
            for point in &small {
                let (x, y) = xy(point);
                let mut bits = bits_of(&x, low_bits);
                bits.extend(bits_of(&y, 5));
                let accepted = accepts(&network, &bits, &mut meter).unwrap();
                let other = curve
                    .add(&target, &curve.negate(point).unwrap(), &mut meter)
                    .unwrap();
                let expected = matches!(&other, Point::Affine { x, .. } if x < &bound);
                assert_eq!(accepted, expected, "{multiplier:?} {point:?}");
                positives += usize::from(accepted);
            }
            assert!(positives >= 1);
            // Off-curve inputs are rejected.
            let mut bits = bits_of(&BigUint::from(1u8), low_bits);
            bits.extend(bits_of(&BigUint::from(3u8), 5));
            assert!(!accepts(&network, &bits, &mut meter).unwrap());
        }
    }

    #[test]
    fn chained_s3_matches_direct_evaluation() {
        let mut meter = meter();
        let curve = curve17(&mut meter);
        let low_bits = 2;
        let small = small_x_points(&curve, low_bits, &mut meter);
        let partial = curve.add(&small[0], &small[2], &mut meter).unwrap();
        let target = curve.add(&partial, &small[3], &mut meter).unwrap();
        let target_x = xy(&target).0;
        let network = chained_s3_small_x(
            &curve,
            low_bits,
            &target,
            Multiplier::Schoolbook,
            &mut meter,
        )
        .unwrap();
        assert_eq!(network.input_bit_count(), Some(3 * low_bits + 5));
        let mut positives = 0;
        for x1 in 0..4u8 {
            for x2 in 0..4u8 {
                for x3 in 0..4u8 {
                    for t in 0..17u8 {
                        let mut bits = bits_of(&BigUint::from(x1), low_bits);
                        bits.extend(bits_of(&BigUint::from(x2), low_bits));
                        bits.extend(bits_of(&BigUint::from(x3), low_bits));
                        bits.extend(bits_of(&BigUint::from(t), 5));
                        let accepted = accepts(&network, &bits, &mut meter).unwrap();
                        let expected = semaev_s3(
                            &curve,
                            &BigUint::from(x1),
                            &BigUint::from(x2),
                            &BigUint::from(t),
                        )
                        .is_zero()
                            && semaev_s3(&curve, &BigUint::from(t), &BigUint::from(x3), &target_x)
                                .is_zero();
                        assert_eq!(accepted, expected, "x1={x1} x2={x2} x3={x3} t={t}");
                        positives += usize::from(accepted);
                    }
                }
            }
        }
        assert!(positives >= 1);
    }

    #[test]
    fn hashed_language_is_multiplier_independent() {
        let mut meter = meter();
        let curve = curve17(&mut meter);
        let points = affine_points(&curve, &mut meter);
        let prime = BigUint::from(37u8);
        let bases = vec![
            FactorBase::new(
                &curve,
                prime.clone(),
                BigUint::from(20u8),
                HashSeed {
                    alpha: BigUint::from(3u8),
                    beta: BigUint::from(1u8),
                },
                &mut meter,
            )
            .unwrap(),
            FactorBase::new(
                &curve,
                prime,
                BigUint::from(20u8),
                HashSeed {
                    alpha: BigUint::from(5u8),
                    beta: BigUint::from(2u8),
                },
                &mut meter,
            )
            .unwrap(),
        ];
        let members: Vec<Vec<&Point>> = bases
            .iter()
            .map(|base| {
                points
                    .iter()
                    .filter(|point| base.contains(&curve, point, &mut meter).unwrap())
                    .collect()
            })
            .collect();
        assert!(!members[0].is_empty() && !members[1].is_empty());
        let target = curve.add(members[0][0], members[1][0], &mut meter).unwrap();
        let mut acceptance = Vec::new();
        for multiplier in [Multiplier::Fourier, Multiplier::Schoolbook] {
            let network = group_law_hash(&curve, &bases, &target, multiplier, &mut meter).unwrap();
            assert_eq!(network.input_bit_count(), Some(10));
            let accepted: Vec<bool> = points
                .iter()
                .map(|point| {
                    let (x, y) = xy(point);
                    let mut bits = bits_of(&x, 5);
                    bits.extend(bits_of(&y, 5));
                    accepts(&network, &bits, &mut meter).unwrap()
                })
                .collect();
            let expected: Vec<bool> = points
                .iter()
                .map(|point| {
                    let other = curve
                        .add(&target, &curve.negate(point).unwrap(), &mut meter)
                        .unwrap();
                    bases[0].contains(&curve, point, &mut meter).unwrap()
                        && bases[1].contains(&curve, &other, &mut meter).unwrap()
                })
                .collect();
            assert_eq!(accepted, expected, "{multiplier:?}");
            assert!(accepted.iter().any(|&a| a));
            acceptance.push(accepted);
        }
        assert_eq!(acceptance[0], acceptance[1]);
    }

    #[test]
    fn names_round_trip() {
        for relation in Relation::ALL {
            assert_eq!(Relation::parse(relation.name()), Some(relation));
        }
        assert_eq!(Relation::parse("nope"), None);
        assert_eq!(Relation::ChainedS3SmallX.low_bits(8), 3);
        assert_eq!(Relation::SemaevS3SmallX.low_bits(9), 5);
    }
}
