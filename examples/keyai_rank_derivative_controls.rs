//! Correctness integration of the KeyAI derivative-support and cancellation controls.
//! Run with `cargo test --release --example keyai_rank_derivative_controls`.
//! This deterministic public fixture records no timing or performance comparison.

use crypto_lib::cryptanalysis::{
    challenge_ladder::is_prime_u64,
    ic_boundary::{
        CountedGroup, FactorBase, GroupOps, IncrementalGauss, PrimeCurve, PrimePoint,
        RelationSolver, RowStatus,
    },
    ic_framework::{
        stages::InstanceCtx,
        useful_rank::{fixed_target_row, replay, AuditedMatrix},
    },
    semaev_prime_poly::{
        derivative, poly_div_exact, poly_gcd, poly_mul, poly_powmod, poly_rem, poly_support_holds,
        poly_support_jacobian_at_root, Fp,
    },
};
use serde_json::{json, Value};
use std::collections::BTreeSet;

const P: u64 = 2_147_483_647;
const R: u64 = 2_147_446_831;

fn curve() -> PrimeCurve {
    PrimeCurve {
        p: P,
        a: 847_514_846,
        b: 2_087_444_724,
    }
}

fn generator() -> PrimePoint {
    PrimePoint::affine(625_037_715, 2_100_627_251)
}

/// The known construction scalars are not stored in the factor base or given to
/// the matrix. Only the public +/-G column is subsequently used as an anchor.
fn fixture_base(c: &PrimeCurve) -> Result<FactorBase<PrimePoint>, String> {
    let mut ops = GroupOps::default();
    let mut points = Vec::new();
    let mut columns = Vec::new();
    let mut coefficients = Vec::new();
    for column in 0..9 {
        let point = c.mul(&mut ops, generator(), column as u64 + 1);
        let negative = c.neg(point);
        let canonical = if point.y < negative.y {
            point
        } else {
            negative
        };
        points.extend([canonical, c.neg(canonical)]);
        columns.extend([column, column]);
        coefficients.extend([1, R - 1]);
    }
    FactorBase::from_column_map(
        "Prime31 public +/-[1..9]G correctness fixture; min-y columns".into(),
        points,
        columns,
        coefficients,
        9,
        |point| c.key(point),
        |point| c.key(&c.neg(*point)),
        |point| point.x,
    )
}

fn polynomial_from_roots(f: Fp, roots: impl IntoIterator<Item = u64>) -> Vec<u64> {
    roots.into_iter().fold(vec![1], |product, root| {
        poly_mul(f, &product, &[f.neg(root), 1])
    })
}

/// Three coefficient-only formulations, followed separately by signed group
/// replay and audited row insertion. None of these support tests finds a g.
fn compare_support_forms(f: Fp, support: &[u64], candidate: &[u64]) -> Result<bool, String> {
    let derivative_form = poly_support_holds(f, support, candidate)?;
    let powered_form = poly_powmod(f, support, (candidate.len() - 1) as u64, candidate) == [0];
    let common = poly_gcd(f, candidate, &derivative(f, candidate));
    let radical = poly_div_exact(f, candidate, &common);
    let radical_form = poly_rem(f, support, &radical) == [0];
    if derivative_form != powered_form || derivative_form != radical_form {
        return Err("derivative, powered and radical support forms disagree".into());
    }
    Ok(derivative_form)
}

fn require(condition: bool, message: &str) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn cancellation_control() -> Result<Value, String> {
    let c = curve();
    let g = generator();
    let f = Fp::new(P);
    let mut ops = GroupOps::default();
    require(
        is_prime_u64(P) && is_prime_u64(R),
        "fixture moduli must be prime",
    )?;
    require(c.is_on_curve(g), "fixture generator off curve")?;
    require(
        c.is_identity(&c.mul(&mut ops, g, R)),
        "fixture subgroup order mismatch",
    )?;
    let target = c.neg(c.mul(&mut ops, g, 2));
    require(
        !c.is_identity(&target),
        "cancellation target must be nonzero",
    )?;
    let fb = fixture_base(&c)?;
    let ctx = InstanceCtx {
        group: &c,
        generator: g,
        target,
        r: R,
        cofactor: 1,
        group_order: R,
        name: "prime31-keyai-fixed-nonzero-cancellation".into(),
        field_degree: None,
    };
    let mut matrix = AuditedMatrix::new(
        &ctx,
        &fb,
        Box::new(IncrementalGauss::new(fb.columns + 1, R)),
    )?;
    let a = fb
        .index_of_key(c.key(&g))
        .ok_or("missing generator witness point")?;
    let support = polynomial_from_roots(f, fb.abscissa_list.iter().copied());
    let mut x_tuples = BTreeSet::new();
    let mut signed_witnesses = BTreeSet::new();
    let mut candidates = BTreeSet::new();
    let mut normalized_rows = BTreeSet::new();
    let mut jacobian_checks = 0;

    // 32 ordered coordinate tuples, each with both global signed recoveries.
    // Every witness is A,A,B1,-B1,...,B7,-B7, so it has sixteen leaves.
    for rotation in 0..8 {
        for swap in 1..5 {
            let mut selected: Vec<usize> = (1..9).collect();
            selected.rotate_left(rotation);
            selected.swap(0, swap);
            let mut witness = vec![a, a];
            for &column in selected.iter().take(7) {
                witness.extend([2 * column, 2 * column + 1]);
            }
            for globally_negated in [false, true] {
                let signed: Vec<_> = witness
                    .iter()
                    .map(|&index| {
                        if globally_negated {
                            fb.neg_index[index]
                        } else {
                            index
                        }
                    })
                    .collect();
                let target_sign = if globally_negated { -1 } else { 1 };
                let coordinates: Vec<_> = signed.iter().map(|&i| fb.points[i].x).collect();
                x_tuples.insert(coordinates.clone());
                signed_witnesses.insert((signed.clone(), target_sign));
                let candidate = polynomial_from_roots(f, coordinates);
                require(
                    compare_support_forms(f, &support, &candidate)?,
                    "supported witness rejected",
                )?;
                if candidates.insert(candidate.clone()) {
                    let jacobian = poly_support_jacobian_at_root(f, &support, &candidate)?;
                    let mut jacobian_matrix = IncrementalGauss::new(16, P);
                    for row in jacobian {
                        jacobian_matrix.add_row(row, 0);
                    }
                    require(
                        jacobian_matrix.rank() == 16,
                        "squarefree-support Jacobian is singular",
                    )?;
                    jacobian_checks += 1;
                }
                let sum = signed
                    .iter()
                    .fold(c.identity(), |sum, &i| c.add(&mut ops, sum, fb.points[i]));
                let expected_sum = if target_sign == 1 {
                    c.neg(target)
                } else {
                    target
                };
                require(
                    sum == expected_sum,
                    "signed cancellation witness failed public replay",
                )?;
                let row = fixed_target_row(&ctx, &fb, &signed, target_sign)?;
                normalized_rows.insert(row.clone());
                let expected_status = if matrix.rank() == 0 {
                    RowStatus::Independent
                } else {
                    RowStatus::Dependent
                };
                require(
                    matrix.add_row(row, 0) == expected_status,
                    "cancellation rank changed",
                )?;
            }
        }
    }
    require(
        x_tuples.len() == 32 && signed_witnesses.len() == 64,
        "fixture witness count mismatch",
    )?;
    require(
        normalized_rows.len() == 1,
        "fixed-target row normalization disagrees",
    )?;
    require(candidates.len() == 8, "candidate-polynomial count mismatch")?;
    require(
        matrix.audit.rank == 1 && matrix.audit.base_rank == 1,
        "cancellation family rank is not one",
    )?;
    require(
        matrix.audit.duplicate_rows == 63,
        "duplicate accounting mismatch",
    )?;
    require(
        !matrix.audit.target_is_determined(),
        "unanchored cancellation family pins the target",
    )?;
    require(
        matrix.pinned(fb.columns).is_none(),
        "unanchored matrix returned a target scalar",
    )?;
    replay(&ctx, &fb, &matrix.audit)?;
    let unanchored_audit = matrix.audit.clone();

    // The same row already present contributes no additional useful rank.
    let repeated_row = normalized_rows.iter().next().unwrap().clone();
    require(
        matrix.add_row(repeated_row, 0) == RowStatus::Dependent,
        "repeat added rank",
    )?;
    let repeat_rank = matrix.rank();

    // Add the explicit public generator anchor only now. Other construction
    // logs never enter the matrix, and the expected target scalar is not input.
    let mut anchor = vec![0; fb.columns + 1];
    anchor[fb.col_of[a]] = fb.coef_of[a];
    require(
        matrix.add_row(anchor, 1) == RowStatus::Independent,
        "generator anchor did not add rank",
    )?;
    let recovered = matrix
        .pinned(fb.columns)
        .ok_or("anchored target is not pinned")?;
    require(
        matrix.audit.target_is_determined(),
        "rank diagnostic missed determined target",
    )?;
    require(
        c.mul(&mut ops, g, recovered) == target,
        "recovered scalar failed public replay",
    )?;
    require(
        recovered == R - 2 && recovered != P - 2,
        "recovery used the wrong modulus",
    )?;
    replay(&ctx, &fb, &matrix.audit)?;

    // A missing algebraic support root must fail all three formulations.
    let first_candidate = candidates.iter().next().unwrap();
    let mut outside = 0;
    while fb.abscissa_list.contains(&outside) {
        outside += 1;
    }
    let without_a = poly_div_exact(f, first_candidate, &[f.neg(g.x), 1]);
    let unsupported = poly_mul(f, &without_a, &[f.neg(outside), 1]);
    require(
        !compare_support_forms(f, &support, &unsupported)?,
        "unsupported root was accepted",
    )?;
    require(
        poly_support_jacobian_at_root(f, &support, &unsupported).is_err(),
        "off-root Jacobian accepted",
    )?;

    Ok(json!({
        "schema": "keyai-rank-derivative-controls/v1",
        "scope": "deterministic_public_prime31_correctness_fixture",
        "coordinate_modulus": P,
        "subgroup_order": R,
        "curve": {"a": c.a, "b": c.b},
        "generator": {"x": g.x, "y": g.y},
        "target": {"x": target.x, "y": target.y},
        "summands": 16,
        "raw_ordered_coordinate_tuples": x_tuples.len(),
        "signed_witnesses": signed_witnesses.len(),
        "distinct_candidate_polynomials": candidates.len(),
        "distinct_normalized_rows": normalized_rows.len(),
        "independent_relation_rows": unanchored_audit.independent_rows,
        "dependent_relation_rows": unanchored_audit.dependent_rows,
        "coefficient_rank": unanchored_audit.base_rank,
        "rank_including_target": unanchored_audit.rank,
        "target_determined_before_anchor": unanchored_audit.target_is_determined(),
        "rank_after_repeated_row": repeat_rank,
        "generator_anchors_added": 1,
        "rank_after_anchor": matrix.rank(),
        "target_determined_after_anchor": matrix.audit.target_is_determined(),
        "recovered_scalar": recovered,
        "jacobian_matrices_checked": jacobian_checks,
        "jacobian_rank": 16,
        "support_forms": ["derivative", "powered", "radical"],
        "unsupported_root_rejected": true,
        "unanchored_audit": unanchored_audit,
        "anchored_audit": matrix.audit
    }))
}

fn main() -> Result<(), String> {
    println!(
        "{}",
        serde_json::to_string_pretty(&cancellation_control()?).map_err(|e| e.to_string())?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime31_cancellation_family_support_and_anchored_recovery() {
        let report = cancellation_control().unwrap();
        assert_eq!(report["signed_witnesses"], 64);
        assert_eq!(report["distinct_normalized_rows"], 1);
        assert_eq!(report["recovered_scalar"], R - 2);
    }

    #[test]
    fn cofactor_and_global_sign_preserve_fixed_target_rows() {
        // E/F13 has 20 points, with G=(4,4) of order five: h=4 is an
        // actual cofactor, not a synthetic clearing multiplier on Prime31.
        let c = PrimeCurve { p: 13, a: 1, b: 0 };
        let g = PrimePoint::affine(4, 4);
        let mut ops = GroupOps::default();
        let twice_g = c.mul(&mut ops, g, 2);
        assert!(c.is_identity(&c.mul(&mut ops, g, 5)));
        let order = 1
            + (0..13)
                .flat_map(|x| (0..13).map(move |y| PrimePoint::affine(x, y)))
                .filter(|&point| c.is_on_curve(point))
                .count();
        assert_eq!(order, 20);
        let fb = FactorBase::from_column_map(
            "F13 genuine cofactor-four control".into(),
            vec![g, c.neg(g), twice_g, c.neg(twice_g)],
            vec![0, 0, 1, 1],
            vec![1, 4, 1, 4],
            2,
            |point| c.key(point),
            |point| c.key(&c.neg(*point)),
            |point| point.x,
        )
        .unwrap();
        let target = c.neg(twice_g);
        let ctx = InstanceCtx {
            group: &c,
            generator: g,
            target,
            r: 5,
            cofactor: 4,
            group_order: 20,
            name: "cofactor normalization control".into(),
            field_degree: None,
        };
        let a = fb.index_of_key(c.key(&g)).unwrap();
        let row = fixed_target_row(&ctx, &fb, &[a, a], 1).unwrap();
        let opposite =
            fixed_target_row(&ctx, &fb, &[fb.neg_index[a], fb.neg_index[a]], -1).unwrap();
        assert_eq!(row, opposite);
        let mut matrix =
            AuditedMatrix::new(&ctx, &fb, Box::new(IncrementalGauss::new(3, 5))).unwrap();
        assert_eq!(matrix.add_row(row, 0), RowStatus::Independent);
        assert!(!matrix.audit.target_is_determined());
        assert_eq!(matrix.add_row(vec![1, 0, 0], 4), RowStatus::Independent);
        assert_eq!(matrix.pinned(2), Some(3));
        assert!(matrix.audit.target_is_determined());
        replay(&ctx, &fb, &matrix.audit).unwrap();
    }

    #[test]
    fn nonduplicate_dependency_rhs_and_invalid_row_controls() {
        let c = curve();
        let fb = fixture_base(&c).unwrap();
        let mut ops = GroupOps::default();
        let q = c.mul(&mut ops, generator(), 7);
        let ctx = InstanceCtx {
            group: &c,
            generator: generator(),
            target: q,
            r: R,
            cofactor: 1,
            group_order: R,
            name: "full equation accounting".into(),
            field_degree: None,
        };
        let mut matrix =
            AuditedMatrix::new(&ctx, &fb, Box::new(IncrementalGauss::new(10, R))).unwrap();
        let i1 = fb.index_of_key(c.key(&generator())).unwrap();
        let p2 = c.mul(&mut ops, generator(), 2);
        let i2 = fb.index_of_key(c.key(&p2)).unwrap();
        let mut v1 = vec![0; 10];
        v1[fb.col_of[i1]] = fb.coef_of[i1];
        let mut v2 = vec![0; 10];
        v2[fb.col_of[i2]] = fb.coef_of[i2];
        let v3: Vec<_> = v1
            .iter()
            .zip(&v2)
            .map(|(&x, &y)| ((x as u128 + y as u128) % R as u128) as u64)
            .collect();
        assert_eq!(matrix.add_row(v1.clone(), 1), RowStatus::Independent);
        assert_eq!(matrix.add_row(v2, 2), RowStatus::Independent);
        assert_eq!(matrix.add_row(v3, 3), RowStatus::Dependent);
        assert_eq!(matrix.audit.duplicate_rows, 0);
        assert_eq!(matrix.audit.dependent_rows, 1);
        assert!(!matrix.audit.target_is_determined());

        // Same base coefficients, different target coefficient and RHS: the
        // second full equation must be inserted because it determines Q.
        let mut with_target = v1.clone();
        with_target[9] = 1;
        assert_eq!(matrix.add_row(with_target, 8), RowStatus::Independent);
        assert_eq!(matrix.pinned(9), Some(7));
        assert!(matrix.audit.target_is_determined());
        replay(&ctx, &fb, &matrix.audit).unwrap();

        assert_eq!(matrix.add_row(vec![0; 10], 0), RowStatus::Dependent);
        assert_eq!(matrix.add_row(v1, 2), RowStatus::Inconsistent);
        assert_eq!(matrix.add_row(vec![0; 10], 1), RowStatus::Inconsistent);
        assert!(!matrix.audit.target_is_determined());
        assert!(replay(&ctx, &fb, &matrix.audit).is_err());
    }

    #[test]
    fn malformed_fixed_target_inputs_are_rejected() {
        let c = curve();
        let mut fb = fixture_base(&c).unwrap();
        let mut ctx = InstanceCtx {
            group: &c,
            generator: generator(),
            target: generator(),
            r: R,
            cofactor: 1,
            group_order: R,
            name: "input guard".into(),
            field_degree: None,
        };
        assert!(fixed_target_row(&ctx, &fb, &[0], 0).is_err());
        assert!(fixed_target_row(&ctx, &fb, &[fb.points.len()], 1).is_err());
        ctx.cofactor = R;
        assert!(fixed_target_row(&ctx, &fb, &[0], 1).is_err());
        ctx.cofactor = 1;
        ctx.r = 15;
        assert!(fixed_target_row(&ctx, &fb, &[0], 1).is_err());
        ctx.r = R;
        fb.col_of[0] = fb.columns;
        assert!(fixed_target_row(&ctx, &fb, &[0], 1).is_err());
    }
}
