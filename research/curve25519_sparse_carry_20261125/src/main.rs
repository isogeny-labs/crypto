use std::collections::BTreeMap;
use std::env;

use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, ToPrimitive, Zero};
use serde::Serialize;

const RADIX: u32 = 8;
const LIMBS: usize = 85;
const CURVE_A: u32 = 486_662;
const CARRY_LIMIT: i64 = 16;

type Digits = Vec<(usize, i8)>;
type Polynomial = Vec<i64>;

#[derive(Clone)]
struct Point {
    x: BigUint,
    y: BigUint,
    digits: Digits,
}

#[derive(Serialize)]
struct Hit {
    x: [String; 4],
    digits: [Digits; 4],
    max_carry: i64,
    nonzero_carries: usize,
}

#[derive(Serialize)]
struct Summary {
    protocol: &'static str,
    radix: u32,
    limbs: usize,
    max_weight: usize,
    anchor_weight: usize,
    carry_limit: i64,
    candidate_coordinates: usize,
    curve_coordinates: usize,
    ordered_pairs: usize,
    both_outputs_sparse: usize,
    all_non_torsion: usize,
    small_carry: usize,
    hits: Vec<Hit>,
    excluded_torsion: Vec<[String; 4]>,
}

fn modulus() -> BigUint {
    (BigUint::one() << 255) - BigUint::from(19u32)
}

fn sub(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
    if a >= b {
        a - b
    } else {
        p - (b - a)
    }
}

fn mul(a: &BigUint, b: &BigUint, p: &BigUint) -> BigUint {
    (a * b) % p
}

fn inverse(a: &BigUint, p: &BigUint) -> BigUint {
    a.modpow(&(p - 2u32), p)
}

fn sqrt_mod(value: &BigUint, p: &BigUint, sqrt_minus_one: &BigUint) -> Option<BigUint> {
    let mut root = value.modpow(&((p + 3u32) >> 3), p);
    if mul(&root, &root, p) != *value {
        root = mul(&root, sqrt_minus_one, p);
    }
    (mul(&root, &root, p) == *value).then_some(root)
}

fn curve_y(x: &BigUint, p: &BigUint, sqrt_minus_one: &BigUint) -> Option<BigUint> {
    let x2 = mul(x, x, p);
    let rhs = (mul(&x2, x, p) + &x2 * CURVE_A + x) % p;
    sqrt_mod(&rhs, p, sqrt_minus_one)
}

fn coordinates(max_weight: usize, p: &BigUint) -> BTreeMap<BigUint, Digits> {
    let mut found = BTreeMap::new();
    found.insert(BigUint::zero(), vec![]);
    let powers: Vec<_> = (0..LIMBS)
        .map(|i| BigUint::from(RADIX).modpow(&BigUint::from(i), p))
        .collect();
    if max_weight >= 1 {
        for (i, power) in powers.iter().enumerate() {
            for sign in [-1, 1] {
                let value = if sign == 1 { power.clone() } else { p - power };
                found.entry(value).or_insert_with(|| vec![(i, sign)]);
            }
        }
    }
    if max_weight >= 2 {
        for i in 0..LIMBS {
            for j in (i + 1)..LIMBS {
                for sign_i in [-1, 1] {
                    for sign_j in [-1, 1] {
                        let left = if sign_i == 1 {
                            powers[i].clone()
                        } else {
                            p - &powers[i]
                        };
                        let right = if sign_j == 1 {
                            powers[j].clone()
                        } else {
                            p - &powers[j]
                        };
                        found
                            .entry((left + right) % p)
                            .or_insert_with(|| vec![(i, sign_i), (j, sign_j)]);
                    }
                }
            }
        }
    }
    found
}

fn double_x(x: &BigUint, p: &BigUint) -> Option<BigUint> {
    let x2 = mul(x, x, p);
    let inner = (&x2 + x * CURVE_A + BigUint::one()) % p;
    let denominator = (x * &inner * 4u32) % p;
    if denominator.is_zero() {
        return None;
    }
    let numerator = sub(&x2, &BigUint::one(), p);
    Some(mul(
        &mul(&numerator, &numerator, p),
        &inverse(&denominator, p),
        p,
    ))
}

fn non_torsion(x: &BigUint, p: &BigUint) -> bool {
    let mut current = Some(x.clone());
    for _ in 0..3 {
        current = current.and_then(|v| double_x(&v, p));
        if current.is_none() {
            return false;
        }
    }
    true
}

fn polynomial(digits: &Digits) -> Polynomial {
    let mut result = vec![0; digits.iter().map(|(i, _)| i + 1).max().unwrap_or(1)];
    for &(i, digit) in digits {
        result[i] = i64::from(digit);
    }
    result
}

fn polynomial_sub(a: &Polynomial, b: &Polynomial) -> Polynomial {
    (0..a.len().max(b.len()))
        .map(|i| a.get(i).copied().unwrap_or(0) - b.get(i).copied().unwrap_or(0))
        .collect()
}

fn polynomial_mul(a: &Polynomial, b: &Polynomial) -> Polynomial {
    let mut result = vec![0; a.len() + b.len() - 1];
    for (i, &av) in a.iter().enumerate() {
        for (j, &bv) in b.iter().enumerate() {
            result[i + j] += av * bv;
        }
    }
    result
}

fn folded_quartic(encodings: [&Digits; 4]) -> [i64; LIMBS] {
    let [x1, x2, x3, x4] = encodings.map(polynomial);
    let delta = polynomial_sub(&x1, &x2);
    let product_minus_one = polynomial_sub(&polynomial_mul(&x1, &x2), &vec![1]);
    let left = polynomial_mul(&polynomial_mul(&polynomial_mul(&delta, &delta), &x3), &x4);
    let right = polynomial_mul(&product_minus_one, &product_minus_one);
    let quartic = polynomial_sub(&left, &right);
    let mut folded = [0i64; LIMBS];
    for (i, coefficient) in quartic.into_iter().enumerate() {
        folded[i % LIMBS] += coefficient * 19i64.pow((i / LIMBS) as u32);
    }
    folded
}

fn cyclic_carries(folded: &[i64; LIMBS], p: &BigUint) -> Option<(i64, usize)> {
    let base = BigInt::from(RADIX);
    let p_signed = BigInt::from_biguint(Sign::Plus, p.clone());
    let mut value = BigInt::zero();
    let mut place = BigInt::one();
    for &h in folded {
        value += &place * h;
        place *= &base;
    }
    if &value % &p_signed != BigInt::zero() {
        return None;
    }
    let quotient = &value / &p_signed;
    let mut previous = quotient.clone();
    let mut max_carry = 0;
    let mut nonzero = 0;
    for (i, &h) in folded.iter().enumerate() {
        let numerator = BigInt::from(h) + if i == 0 { &previous * 19 } else { previous };
        if &numerator % &base != BigInt::zero() {
            return None;
        }
        previous = numerator / &base;
        let magnitude = previous.magnitude().to_i64()?;
        max_carry = max_carry.max(magnitude);
        nonzero += usize::from(!previous.is_zero());
    }
    (previous == quotient).then_some((max_carry, nonzero))
}

fn outputs(a: &Point, b: &Point, p: &BigUint) -> (BigUint, BigUint) {
    let delta = sub(&a.x, &b.x, p);
    let denominator = mul(&delta, &delta, p);
    let inverse_denominator = inverse(&denominator, p);
    let offset = (BigUint::from(CURVE_A) + &a.x + &b.x) % p;
    let coordinate = |y: BigUint| {
        let square = mul(&y, &y, p);
        sub(&mul(&square, &inverse_denominator, p), &offset, p)
    };
    (
        coordinate(sub(&a.y, &b.y, p)),
        coordinate((&a.y + &b.y) % p),
    )
}

fn search(max_weight: usize, anchor_weight: usize) -> Summary {
    let p = modulus();
    let sqrt_minus_one = BigUint::from(2u32).modpow(&((&p - 1u32) >> 2), &p);
    assert_eq!(mul(&sqrt_minus_one, &sqrt_minus_one, &p), &p - 1u32);
    let candidates = coordinates(max_weight, &p);
    let points: Vec<_> = candidates
        .iter()
        .filter_map(|(x, digits)| {
            curve_y(x, &p, &sqrt_minus_one).map(|y| Point {
                x: x.clone(),
                y,
                digits: digits.clone(),
            })
        })
        .collect();
    let mut summary = Summary {
        protocol: "curve25519-sparse-carry/v1",
        radix: RADIX,
        limbs: LIMBS,
        max_weight,
        anchor_weight,
        carry_limit: CARRY_LIMIT,
        candidate_coordinates: candidates.len(),
        curve_coordinates: points.len(),
        ordered_pairs: 0,
        both_outputs_sparse: 0,
        all_non_torsion: 0,
        small_carry: 0,
        hits: vec![],
        excluded_torsion: vec![],
    };
    for first in points
        .iter()
        .filter(|point| point.digits.len() <= anchor_weight)
    {
        for second in &points {
            if first.x == second.x {
                continue;
            }
            summary.ordered_pairs += 1;
            let (x3, x4) = outputs(first, second, &p);
            let (Some(d3), Some(d4)) = (candidates.get(&x3), candidates.get(&x4)) else {
                continue;
            };
            summary.both_outputs_sparse += 1;
            let values = [&first.x, &second.x, &x3, &x4];
            let names = values.map(ToString::to_string);
            if !values.iter().all(|x| non_torsion(x, &p)) {
                summary.excluded_torsion.push(names);
                continue;
            }
            summary.all_non_torsion += 1;
            let digit_sets = [&first.digits, &second.digits, d3, d4];
            let folded = folded_quartic(digit_sets);
            let (max_carry, nonzero_carries) = cyclic_carries(&folded, &p)
                .expect("actual curve points must satisfy the quartic and carry recurrence");
            if max_carry <= CARRY_LIMIT {
                summary.small_carry += 1;
                summary.hits.push(Hit {
                    x: names,
                    digits: digit_sets.map(Clone::clone),
                    max_carry,
                    nonzero_carries,
                });
            }
        }
    }
    summary
}

fn main() {
    let mut max_weight = 2;
    let mut anchor_weight = 1;
    let mut arguments = env::args().skip(1);
    while let Some(argument) = arguments.next() {
        let value = arguments.next().expect("each option requires an integer");
        match argument.as_str() {
            "--max-weight" => max_weight = value.parse().expect("invalid max-weight"),
            "--anchor-weight" => anchor_weight = value.parse().expect("invalid anchor-weight"),
            _ => panic!("unknown option: {argument}"),
        }
    }
    assert!(max_weight <= 2 && anchor_weight <= max_weight);
    println!(
        "{}",
        serde_json::to_string_pretty(&search(max_weight, anchor_weight)).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_torsion_control() {
        let p = modulus();
        let sqrt_minus_one = BigUint::from(2u32).modpow(&((&p - 1u32) >> 2), &p);
        assert!(curve_y(&BigUint::one(), &p, &sqrt_minus_one).is_some());
        assert!(!non_torsion(&BigUint::zero(), &p));
        assert!(!non_torsion(&BigUint::one(), &p));
        assert!(non_torsion(&BigUint::from(4u32), &p));
        let empty = vec![];
        let one = vec![(0, 1)];
        let folded = folded_quartic([&empty, &one, &one, &one]);
        assert_eq!(cyclic_carries(&folded, &p), Some((0, 0)));
        let zero_point = Point {
            x: BigUint::zero(),
            y: BigUint::zero(),
            digits: empty,
        };
        let one_point = Point {
            x: BigUint::one(),
            y: curve_y(&BigUint::one(), &p, &sqrt_minus_one).unwrap(),
            digits: one,
        };
        assert_eq!(
            outputs(&zero_point, &one_point, &p),
            (BigUint::one(), BigUint::one())
        );
    }

    #[test]
    fn nonzero_cyclic_carry_control() {
        let mut folded = [7i64; LIMBS];
        folded[0] = -11;
        assert_eq!(cyclic_carries(&folded, &modulus()), Some((1, LIMBS)));
    }

    #[test]
    fn coordinate_enumeration_sizes() {
        let p = modulus();
        assert_eq!(coordinates(1, &p).len(), 171);
        assert_eq!(coordinates(2, &p).len(), 14_451);
    }
}
