//! Frozen cubic RR9 remainder-tower constructor recovered for PR #4692.
//!
//! Provenance: the construction and variable order were preserved in
//! `isogeny-labs/crypto` PR #4816, commit `040ef64`, file
//! `research/prime31_rr9_and_closures_20261119/src/rr9_remainder_tower.rs`,
//! where they are explicitly labelled the frozen PR #4692 construction.
//! This module ports only that builder and independent evaluation helpers; it
//! does not import or claim the later branch's experimental outcomes.

use crate::algebra::{AffinePoint, Curve};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

type SparsePoly = BTreeMap<Vec<u32>, u64>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SparseTerm {
    pub exponents: Vec<u32>,
    pub coefficient: u64,
}

pub type SparseEquation = Vec<SparseTerm>;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VariableBlock {
    pub offset: usize,
    pub length: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TowerLayout {
    pub chart: VariableBlock,
    pub q0: Option<VariableBlock>,
    pub r0: Option<VariableBlock>,
    pub q1: VariableBlock,
    pub r1: VariableBlock,
    pub q2: VariableBlock,
    pub r2: VariableBlock,
    pub q3: VariableBlock,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TowerSystemSpec {
    pub schema: String,
    pub base_size: usize,
    pub variables: usize,
    pub equations: usize,
    pub max_input_degree: u32,
    pub predicate: String,
    pub layout: TowerLayout,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenTower {
    pub spec: TowerSystemSpec,
    pub equations: Vec<SparseEquation>,
}

fn add_mod(left: u64, right: u64, p: u64) -> u64 {
    ((u128::from(left) + u128::from(right)) % u128::from(p)) as u64
}

fn mul_mod(left: u64, right: u64, p: u64) -> u64 {
    (u128::from(left) * u128::from(right) % u128::from(p)) as u64
}

fn neg_mod(value: u64, p: u64) -> u64 {
    if value == 0 {
        0
    } else {
        p - value
    }
}

fn sp_const(value: u64, variables: usize, p: u64) -> SparsePoly {
    let value = value % p;
    if value == 0 {
        SparsePoly::new()
    } else {
        BTreeMap::from([(vec![0; variables], value)])
    }
}

fn sp_var(index: usize, variables: usize) -> SparsePoly {
    let mut exponents = vec![0; variables];
    exponents[index] = 1;
    BTreeMap::from([(exponents, 1)])
}

fn sp_add_scaled(destination: &mut SparsePoly, source: &SparsePoly, scale: u64, p: u64) {
    let scale = scale % p;
    if scale == 0 {
        return;
    }
    for (exponents, coefficient) in source {
        let term = mul_mod(*coefficient, scale, p);
        let next = add_mod(destination.get(exponents).copied().unwrap_or(0), term, p);
        if next == 0 {
            destination.remove(exponents);
        } else {
            destination.insert(exponents.clone(), next);
        }
    }
}

fn sp_mul(left: &SparsePoly, right: &SparsePoly, p: u64) -> SparsePoly {
    let mut output = SparsePoly::new();
    for (left_exp, left_coefficient) in left {
        for (right_exp, right_coefficient) in right {
            let exponents = left_exp
                .iter()
                .zip(right_exp)
                .map(|(left, right)| left + right)
                .collect::<Vec<_>>();
            let coefficient = mul_mod(*left_coefficient, *right_coefficient, p);
            let next = add_mod(output.get(&exponents).copied().unwrap_or(0), coefficient, p);
            if next == 0 {
                output.remove(&exponents);
            } else {
                output.insert(exponents, next);
            }
        }
    }
    output
}

fn sp_convolve(left: &[SparsePoly], right: &[SparsePoly], p: u64) -> Vec<SparsePoly> {
    let mut output = vec![SparsePoly::new(); left.len() + right.len() - 1];
    for (i, left) in left.iter().enumerate() {
        for (j, right) in right.iter().enumerate() {
            let product = sp_mul(left, right, p);
            sp_add_scaled(&mut output[i + j], &product, 1, p);
        }
    }
    output
}

fn build_m_expression(
    variables: usize,
    p: u64,
    curve_a: u64,
    residual: (u64, u64),
) -> Vec<SparsePoly> {
    let (r, s) = residual;
    let d0 = add_mod(mul_mod(r, r, p), curve_a, p);
    let a: [SparsePoly; 4] = std::array::from_fn(|index| sp_var(index, variables));
    let v = [
        sp_var(4, variables),
        sp_var(5, variables),
        sp_var(6, variables),
        sp_const(1, variables, p),
    ];
    let d = [
        sp_const(d0, variables, p),
        sp_const(r, variables, p),
        sp_const(1, variables, p),
    ];
    let x_minus_r = [
        sp_const(neg_mod(r, p), variables, p),
        sp_const(1, variables, p),
    ];
    let vv = sp_convolve(&v, &v, p);
    let mut expression = sp_convolve(&d, &vv, p);
    let aa = sp_convolve(&a, &a, p);
    let xaa = sp_convolve(&x_minus_r, &aa, p);
    let av = sp_convolve(&a, &v, p);
    for (index, term) in xaa.iter().enumerate() {
        sp_add_scaled(&mut expression[index], term, p - 1, p);
    }
    let minus_two_s = neg_mod(mul_mod(2, s, p), p);
    for (index, term) in av.iter().enumerate() {
        sp_add_scaled(&mut expression[index], term, minus_two_s, p);
    }
    assert_eq!(expression.len(), 9);
    expression
}

fn block(offset: usize, length: usize) -> VariableBlock {
    VariableBlock { offset, length }
}

pub fn build_frozen_tower(
    p: u64,
    curve_a: u64,
    residual: (u64, u64),
    h_b: &[u64],
) -> Result<FrozenTower, String> {
    if h_b.len() < 2 || h_b.last() != Some(&1) {
        return Err("H_B must be a nonconstant monic polynomial".into());
    }
    let base_size = h_b.len() - 1;
    let mut next_variable = 7usize;
    let mut allocate = |length: usize| {
        let offset = next_variable;
        next_variable += length;
        block(offset, length)
    };
    let q0 = (base_size > 8).then(|| allocate(base_size - 8));
    let r0 = (base_size >= 8).then(|| allocate(8));
    let q1 = allocate(7);
    let r1 = allocate(8);
    let q2 = allocate(7);
    let r2 = allocate(8);
    let q3 = allocate(7);
    let variables = next_variable;
    let layout = TowerLayout {
        chart: block(0, 7),
        q0,
        r0,
        q1,
        r1,
        q2,
        r2,
        q3,
    };

    let m = build_m_expression(variables, p, curve_a, residual);
    let mut equations = Vec::<SparsePoly>::new();
    let r0_expression = if base_size < 8 {
        (0..8)
            .map(|index| sp_const(h_b.get(index).copied().unwrap_or(0), variables, p))
            .collect::<Vec<_>>()
    } else {
        let r0_block = layout.r0.expect("allocated for B >= 8");
        let remainder = (0..8)
            .map(|index| sp_var(r0_block.offset + index, variables))
            .collect::<Vec<_>>();
        let mut quotient = layout
            .q0
            .map(|q0_block| {
                (0..q0_block.length)
                    .map(|index| sp_var(q0_block.offset + index, variables))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        quotient.push(sp_const(1, variables, p));
        let product = sp_convolve(&quotient, &m, p);
        for index in 0..base_size {
            let mut equation = product[index].clone();
            if index < 8 {
                sp_add_scaled(&mut equation, &remainder[index], 1, p);
            }
            let constant = sp_const(h_b[index], variables, p);
            sp_add_scaled(&mut equation, &constant, p - 1, p);
            equations.push(equation);
        }
        remainder
    };

    let mut previous = r0_expression;
    for (quotient_block, remainder_block) in [
        (layout.q1, Some(layout.r1)),
        (layout.q2, Some(layout.r2)),
        (layout.q3, None),
    ] {
        let quotient = (0..7)
            .map(|index| sp_var(quotient_block.offset + index, variables))
            .collect::<Vec<_>>();
        let remainder = remainder_block.map(|block| {
            (0..8)
                .map(|index| sp_var(block.offset + index, variables))
                .collect::<Vec<_>>()
        });
        let square = sp_convolve(&previous, &previous, p);
        let product = sp_convolve(&quotient, &m, p);
        for index in 0..15 {
            let mut equation = square[index].clone();
            sp_add_scaled(&mut equation, &product[index], p - 1, p);
            if let Some(remainder) = &remainder {
                if index < 8 {
                    sp_add_scaled(&mut equation, &remainder[index], p - 1, p);
                }
            }
            equations.push(equation);
        }
        if let Some(remainder) = remainder {
            previous = remainder;
        }
    }

    equations.retain(|equation| !equation.is_empty());
    let max_input_degree = equations
        .iter()
        .flat_map(|equation| equation.keys())
        .map(|exponents| exponents.iter().sum())
        .max()
        .unwrap_or(0);
    let equations = equations
        .into_iter()
        .map(|equation| {
            equation
                .into_iter()
                .map(|(exponents, coefficient)| SparseTerm {
                    exponents,
                    coefficient,
                })
                .collect()
        })
        .collect::<Vec<_>>();
    Ok(FrozenTower {
        spec: TowerSystemSpec {
            schema: "prime31.rr9-frozen-remainder-tower/v1".into(),
            base_size,
            variables,
            equations: equations.len(),
            max_input_degree,
            predicate: "M_(A,V,R)(X) divides H_B(X)^8".into(),
            layout,
        },
        equations,
    })
}

pub fn polynomial_from_roots(roots: &[u64], p: u64) -> Vec<u64> {
    let mut polynomial = vec![1];
    for root in roots {
        polynomial = poly_mul(&polynomial, &[neg_mod(*root % p, p), 1], p);
    }
    polynomial
}

fn poly_trim(polynomial: &mut Vec<u64>) {
    while polynomial.len() > 1 && polynomial.last() == Some(&0) {
        polynomial.pop();
    }
}

fn poly_add_scaled(destination: &mut Vec<u64>, source: &[u64], scale: u64, p: u64) {
    if destination.len() < source.len() {
        destination.resize(source.len(), 0);
    }
    for (index, coefficient) in source.iter().copied().enumerate() {
        destination[index] = add_mod(destination[index], mul_mod(coefficient, scale, p), p);
    }
    poly_trim(destination);
}

fn poly_mul(left: &[u64], right: &[u64], p: u64) -> Vec<u64> {
    let mut output = vec![0; left.len() + right.len() - 1];
    for (i, left) in left.iter().copied().enumerate() {
        for (j, right) in right.iter().copied().enumerate() {
            output[i + j] = add_mod(output[i + j], mul_mod(left, right, p), p);
        }
    }
    poly_trim(&mut output);
    output
}

fn poly_div_rem_monic(numerator: &[u64], denominator: &[u64], p: u64) -> (Vec<u64>, Vec<u64>) {
    assert_eq!(denominator.last(), Some(&1));
    let denominator_degree = denominator.len() - 1;
    let mut remainder = numerator.to_vec();
    poly_trim(&mut remainder);
    if remainder.len() <= denominator_degree {
        return (vec![0], remainder);
    }
    let mut quotient = vec![0; remainder.len() - denominator_degree];
    for position in (denominator_degree..remainder.len()).rev() {
        let coefficient = remainder[position];
        let shift = position - denominator_degree;
        quotient[shift] = coefficient;
        for index in 0..=denominator_degree {
            let subtract = neg_mod(mul_mod(coefficient, denominator[index], p), p);
            remainder[shift + index] = add_mod(remainder[shift + index], subtract, p);
        }
    }
    remainder.truncate(denominator_degree);
    poly_trim(&mut quotient);
    poly_trim(&mut remainder);
    (quotient, remainder)
}

pub fn concrete_m(p: u64, curve_a: u64, residual: (u64, u64), chart: &[u64; 7]) -> Vec<u64> {
    let (r, s) = residual;
    let a = chart[..4].to_vec();
    let v = vec![chart[4], chart[5], chart[6], 1];
    let d = vec![add_mod(mul_mod(r, r, p), curve_a, p), r, 1];
    let mut output = poly_mul(&d, &poly_mul(&v, &v, p), p);
    let x_minus_r = vec![neg_mod(r, p), 1];
    let xaa = poly_mul(&x_minus_r, &poly_mul(&a, &a, p), p);
    poly_add_scaled(&mut output, &xaa, p - 1, p);
    let av = poly_mul(&a, &v, p);
    poly_add_scaled(&mut output, &av, neg_mod(mul_mod(2, s, p), p), p);
    output.resize(9, 0);
    output
}

pub fn derive_tower_witness(
    tower: &FrozenTower,
    p: u64,
    curve_a: u64,
    residual: (u64, u64),
    h_b: &[u64],
    chart: [u64; 7],
) -> Result<Vec<u64>, String> {
    if tower.spec.base_size + 1 != h_b.len() {
        return Err("tower/H_B size mismatch".into());
    }
    let mut assignment = vec![0; tower.spec.variables];
    assignment[..7].copy_from_slice(&chart);
    let m = concrete_m(p, curve_a, residual, &chart);
    if m.len() != 9 || m[8] != 1 {
        return Err("chart did not produce a monic degree-eight M".into());
    }
    let mut previous = if tower.spec.base_size < 8 {
        let mut padded = h_b.to_vec();
        padded.resize(8, 0);
        padded
    } else {
        let (quotient, remainder) = poly_div_rem_monic(h_b, &m, p);
        if quotient.last() != Some(&1) {
            return Err("initial monic quotient invariant failed".into());
        }
        if let Some(block) = tower.spec.layout.q0 {
            if quotient.len() != block.length + 1 {
                return Err("initial quotient has unexpected degree".into());
            }
            assignment[block.offset..block.offset + block.length]
                .copy_from_slice(&quotient[..block.length]);
        }
        let block = tower.spec.layout.r0.expect("B >= 8 has R0");
        for index in 0..block.length {
            assignment[block.offset + index] = remainder.get(index).copied().unwrap_or(0);
        }
        let mut padded = remainder;
        padded.resize(8, 0);
        padded
    };
    for (quotient_block, remainder_block) in [
        (tower.spec.layout.q1, Some(tower.spec.layout.r1)),
        (tower.spec.layout.q2, Some(tower.spec.layout.r2)),
        (tower.spec.layout.q3, None),
    ] {
        let square = poly_mul(&previous, &previous, p);
        let (quotient, remainder) = poly_div_rem_monic(&square, &m, p);
        if quotient.len() > quotient_block.length {
            return Err("tower quotient exceeds its frozen seven coefficients".into());
        }
        for index in 0..quotient_block.length {
            assignment[quotient_block.offset + index] = quotient.get(index).copied().unwrap_or(0);
        }
        if let Some(block) = remainder_block {
            for index in 0..block.length {
                assignment[block.offset + index] = remainder.get(index).copied().unwrap_or(0);
            }
            previous = remainder;
            previous.resize(8, 0);
        } else if remainder.iter().any(|coefficient| *coefficient != 0) {
            return Err("chart does not satisfy M | H_B^8".into());
        }
    }
    Ok(assignment)
}

pub fn evaluate_equation(equation: &SparseEquation, assignment: &[u64], p: u64) -> u64 {
    equation.iter().fold(0, |sum, term| {
        let mut value = term.coefficient;
        for (variable, exponent) in term.exponents.iter().copied().enumerate() {
            for _ in 0..exponent {
                value = mul_mod(value, assignment[variable], p);
            }
        }
        add_mod(sum, value, p)
    })
}

pub fn verify_assignment(tower: &FrozenTower, assignment: &[u64], p: u64) -> bool {
    assignment.len() == tower.spec.variables
        && assignment.iter().all(|value| *value < p)
        && tower
            .equations
            .iter()
            .all(|equation| evaluate_equation(equation, assignment, p) == 0)
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedFactor {
    pub base_index: usize,
    pub sign: i8,
    pub multiplicity: u8,
}

fn poly_eval(polynomial: &[u64], value: u64, p: u64) -> u64 {
    polynomial.iter().rev().fold(0, |accumulator, coefficient| {
        add_mod(mul_mod(accumulator, value, p), *coefficient, p)
    })
}

/// Decode one RR9 chart into a signed eight-multiset and replay its EC sum.
///
/// This is a consumer-side check independent of the remainder-tower
/// auxiliaries.  Multiplicities are factored from `M`; signs are recovered
/// from Taylor/Hermite vanishing orders of
/// `(X-r)A(X) + (Y+s)V(X)` on the curve.  It handles repeated points,
/// opposite pairs, and a support abscissa equal to `r`.
pub fn decode_and_replay_chart(
    curve: Curve,
    base: &[AffinePoint],
    residual: AffinePoint,
    chart: &[u64; 7],
) -> Result<Vec<SignedFactor>, String> {
    curve.validate()?;
    if !curve.on_curve(residual) {
        return Err("RR9 residual is not on the curve".into());
    }
    if base.is_empty()
        || base
            .iter()
            .any(|point| !curve.on_curve(*point) || point.y == 0)
        || base
            .iter()
            .enumerate()
            .any(|(index, point)| base[..index].iter().any(|old| old.x == point.x))
    {
        return Err("RR9 base must contain distinct nonzero-y curve points".into());
    }

    let p = curve.p;
    let m = concrete_m(p, curve.a, (residual.x, residual.y), chart);
    if m.len() != 9 || m[8] != 1 {
        return Err("RR9 chart did not produce a monic degree-eight M".into());
    }

    let mut remaining = m;
    let mut multiplicities = vec![0u8; base.len()];
    let mut total_degree = 0u8;
    for (index, point) in base.iter().enumerate() {
        let factor = [curve.neg(point.x), 1];
        loop {
            if remaining.len() <= 1 {
                break;
            }
            let (quotient, remainder) = poly_div_rem_monic(&remaining, &factor, p);
            if remainder.iter().any(|coefficient| *coefficient != 0) {
                break;
            }
            remaining = quotient;
            multiplicities[index] = multiplicities[index]
                .checked_add(1)
                .ok_or_else(|| "RR9 factor multiplicity overflow".to_owned())?;
            total_degree = total_degree
                .checked_add(1)
                .ok_or_else(|| "RR9 total multiplicity overflow".to_owned())?;
        }
    }
    if total_degree != 8 || remaining != [1] {
        return Err("RR9 M does not factor completely over the declared support".into());
    }

    let a = [chart[0], chart[1], chart[2], chart[3]];
    let v = [chart[4], chart[5], chart[6], 1];
    let vanishing_order = |x0: u64, y0: u64, cap: usize| -> Result<u8, String> {
        let taylor_rhs = [
            curve.mul(y0, y0),
            curve.add(curve.mul(3, curve.mul(x0, x0)), curve.a),
            curve.mul(3, x0),
            1,
        ];
        let inverse_two_y = curve
            .inverse(curve.mul(2, y0))
            .ok_or_else(|| "RR9 Hermite expansion encountered zero y".to_owned())?;
        let mut y_series = vec![0; cap + 1];
        y_series[0] = y0;
        for order in 1..=cap {
            let rhs = taylor_rhs.get(order).copied().unwrap_or(0);
            let convolution = (1..order).fold(0, |sum, index| {
                curve.add(sum, curve.mul(y_series[index], y_series[order - index]))
            });
            y_series[order] = curve.mul(curve.sub(rhs, convolution), inverse_two_y);
        }

        let shift_cubic = |coefficients: &[u64; 4]| {
            vec![
                poly_eval(coefficients, x0, p),
                curve.add(
                    coefficients[1],
                    curve.add(
                        curve.mul(2, curve.mul(coefficients[2], x0)),
                        curve.mul(3, curve.mul(coefficients[3], curve.mul(x0, x0))),
                    ),
                ),
                curve.add(
                    coefficients[2],
                    curve.mul(3, curve.mul(coefficients[3], x0)),
                ),
                coefficients[3],
            ]
        };
        let a_series = shift_cubic(&a);
        let v_series = shift_cubic(&v);
        y_series[0] = curve.add(y_series[0], residual.y);
        let left = poly_mul(&[curve.sub(x0, residual.x), 1], &a_series, p);
        let right = poly_mul(&y_series, &v_series, p);
        for order in 0..=cap {
            if curve.add(
                left.get(order).copied().unwrap_or(0),
                right.get(order).copied().unwrap_or(0),
            ) != 0
            {
                return Ok(order as u8);
            }
        }
        Ok((cap + 1) as u8)
    };

    let mut decoded = Vec::new();
    let mut replay = None;
    let mut leaves = 0u8;
    for (index, point) in base.iter().copied().enumerate() {
        let multiplicity = multiplicities[index];
        if multiplicity == 0 {
            continue;
        }
        let mut positive = vanishing_order(point.x, point.y, usize::from(multiplicity) + 1)?;
        let negative_y = curve.neg(point.y);
        let mut negative = vanishing_order(point.x, negative_y, usize::from(multiplicity) + 1)?;

        // The function has one structural zero at -R; remove it before
        // interpreting the remaining divisor as the eight selected leaves.
        let residual_negative_y = curve.neg(residual.y);
        if point.x == residual.x && point.y == residual_negative_y {
            positive = positive
                .checked_sub(1)
                .ok_or_else(|| "RR9 chart misses its structural -R zero".to_owned())?;
        }
        if point.x == residual.x && negative_y == residual_negative_y {
            negative = negative
                .checked_sub(1)
                .ok_or_else(|| "RR9 chart misses its structural -R zero".to_owned())?;
        }
        if positive + negative != multiplicity {
            return Err(format!(
                "RR9 sign multiplicities disagree at base index {index}"
            ));
        }
        for (sign, count, selected) in [
            (1i8, positive, point),
            (
                -1i8,
                negative,
                AffinePoint {
                    x: point.x,
                    y: negative_y,
                },
            ),
        ] {
            if count == 0 {
                continue;
            }
            decoded.push(SignedFactor {
                base_index: index,
                sign,
                multiplicity: count,
            });
            for _ in 0..count {
                replay = curve.point_add(replay, Some(selected));
            }
            leaves += count;
        }
    }
    decoded.sort_unstable();
    if leaves != 8 || replay != Some(residual) {
        return Err("RR9 decoded signed multiset fails independent EC replay".into());
    }
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PRIME31_A, PRIME31_P};

    const RESIDUAL: (u64, u64) = (1_911_579_686, 548_820_096);
    const CHART: [u64; 7] = [
        455_835_358,
        518_652_820,
        754_716_693,
        932_383_545,
        62_510_441,
        1_884_059_825,
        1_142_049_404,
    ];

    #[test]
    fn frozen_six_rung_shapes_are_exact() {
        for (base_size, variables, equations) in [
            (4, 44, 45),
            (8, 52, 53),
            (12, 56, 57),
            (16, 60, 61),
            (24, 68, 69),
            (31, 75, 76),
        ] {
            let roots = (0..base_size as u64).collect::<Vec<_>>();
            let h = polynomial_from_roots(&roots, PRIME31_P);
            let tower = build_frozen_tower(PRIME31_P, PRIME31_A, RESIDUAL, &h).unwrap();
            assert_eq!(tower.spec.variables, variables);
            assert_eq!(tower.spec.equations, equations);
            assert_eq!(tower.spec.max_input_degree, 3);
        }
    }

    #[test]
    fn known_b8_chart_derives_unique_auxiliaries_and_mutation_fails() {
        let roots = [0, 1, 2, 3, 5, 6, 8, 9];
        let h = polynomial_from_roots(&roots, PRIME31_P);
        let tower = build_frozen_tower(PRIME31_P, PRIME31_A, RESIDUAL, &h).unwrap();
        let assignment =
            derive_tower_witness(&tower, PRIME31_P, PRIME31_A, RESIDUAL, &h, CHART).unwrap();
        assert!(verify_assignment(&tower, &assignment, PRIME31_P));
        let second =
            derive_tower_witness(&tower, PRIME31_P, PRIME31_A, RESIDUAL, &h, CHART).unwrap();
        assert_eq!(assignment, second);
        let mut mutated = assignment;
        mutated[0] = add_mod(mutated[0], 1, PRIME31_P);
        assert!(!verify_assignment(&tower, &mutated, PRIME31_P));

        let base = crate::prime31_curve().canonical_base(8).unwrap();
        let decoded = decode_and_replay_chart(
            crate::prime31_curve(),
            &base,
            AffinePoint {
                x: RESIDUAL.0,
                y: RESIDUAL.1,
            },
            &CHART,
        )
        .unwrap();
        assert_eq!(
            decoded
                .iter()
                .map(|factor| usize::from(factor.multiplicity))
                .sum::<usize>(),
            8
        );
    }
}
