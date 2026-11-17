//! Fixed continuous-relaxation controls: always check the actual curve branches.
use super::public_jets::{interpolate, kernel, norm};
use super::signed_hankel::{hankel, weights};
use super::*;
pub(super) fn control() -> Value {
    let f = vec![2, 2, 0, 1];
    let points = vec![(0, 6), (3, 1), (5, 1), (6, 3), (10, 11), (13, 10)];
    let xs = points.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    let g = roots(&xs);
    let w = weights(&points);
    let m = hankel(&points, &w);
    let d = det(&m);
    assert_eq!(d, 1);
    let adj = adjugate(&m);
    let mut rows = vec![];
    let mut repaired = 0;
    let mut off_curve = 0;
    let mut valid = 0;
    let mut no_direction = 0;
    let mut eligible_repaired = 0;
    let mut eligible_off_curve = 0;
    let mut eligible_valid = 0;
    for (t, (x, y)) in points.iter().enumerate() {
        let v = vec![1, *x];
        let av = mv(&adj, &v);
        let q = v.iter().zip(&av).fold(0, |s, (a, b)| add(s, mul(*a, *b)));
        let coefficient = mul(w[t], q);
        let target_fixed = t == 0;
        let allowed_delta = sub(0, mul(2, *y));
        let mut sign_flip = points.clone();
        sign_flip[t].1 = sub(0, *y);
        let allowed_det = det(&hankel(&sign_flip, &w));
        assert_eq!(allowed_det, add(d, mul(coefficient, allowed_delta)));
        if coefficient == 0 {
            no_direction += 1;
            // Check all field corrections for the frozen zero-coefficient case.
            for delta in 0..P {
                let mut changed = points.clone();
                changed[t].1 = add(*y, delta);
                assert_eq!(det(&hankel(&changed, &w)), d);
            }
            rows.push(json!({"index":t,"x":x,"old_y":y,"cofactor_quadratic":q,
                "linear_coefficient":coefficient,"repairable":false,"allowed_flip_determinant":allowed_det,
                "fixed_target_coordinate":target_fixed}));
            continue;
        }
        repaired += 1;
        if !target_fixed {
            eligible_repaired += 1;
        }
        let delta = sub(0, mul(d, inv(coefficient)));
        let y_new = add(*y, delta);
        let mut changed = points.clone();
        changed[t].1 = y_new;
        let new_m = hankel(&changed, &w);
        assert_eq!(det(&new_m), 0);
        assert_eq!(rank(&new_m), 1);
        let on_curve = mul(y_new, y_new) == eval(&f, *x);
        let allowed_branch = delta == 0 || delta == allowed_delta;
        assert_eq!(on_curve, allowed_branch);
        if on_curve {
            valid += 1;
            if !target_fixed {
                eligible_valid += 1;
            }
        } else {
            off_curve += 1;
            if !target_fixed {
                eligible_off_curve += 1;
            }
        }
        let basis = kernel(new_m.clone());
        assert_eq!(basis.len(), 1);
        let b = trim(basis[0].clone());
        let s = interpolate(&changed);
        let a = remainder(&pm(&s, &b), &g);
        assert!(a.len() <= 4);
        assert!(b.len() <= 2);
        let n = norm(&f, &a, &b);
        assert!(!n.is_empty());
        let c = coeff(&n, 6);
        let exact_norm = c != 0 && n == scale(&g, c);
        assert_eq!(exact_norm, on_curve);
        for (xx, yy) in &changed {
            assert_eq!(eval(&a, *xx), mul(*yy, eval(&b, *xx)));
        }
        rows.push(json!({"index":t,"x":x,"old_y":y,"cofactor_quadratic":q,
            "linear_coefficient":coefficient,"repairable":true,"delta":delta,"new_y":y_new,
            "new_matrix":new_m,"new_determinant":0,"new_rank":1,
            "on_curve":on_curve,"allowed_branch":allowed_branch,
            "allowed_flip_determinant":allowed_det,"fixed_target_coordinate":target_fixed,
            "A":a,"B":b,"norm":n,"c":c,"exact_norm_pass":exact_norm}));
    }
    assert_eq!((repaired, off_curve, valid, no_direction), (5, 4, 1, 1));
    assert_eq!(
        (eligible_repaired, eligible_off_curve, eligible_valid),
        (4, 3, 1)
    );
    json!({"schema":"norm.continuous-branch-repair/v1","p":P,"curve_coefficients":f,
        "public_points":points,"initial_matrix":m,"initial_determinant":d,"weights":w,
        "coordinate_controls":rows,"all_coordinates":{"repairs":repaired,"off_curve":off_curve,
            "valid_branches":valid,"zero_coefficient":no_direction},
        "target_fixed_at_x0":{"repairs":eligible_repaired,"off_curve":eligible_off_curve,
            "valid_branches":eligible_valid,"zero_coefficient":1},
        "physical_artifact_invocations":1,"random_branch_search_executed":false,"timing_measured":false,
        "scope":"one frozen supplied-branch algebra correctness control; no yield, global infeasibility or sub-exponential search bound",
        "standing":"unsigned producer evidence"})
}
pub(super) fn write(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = serde_json::to_vec_pretty(&control())?;
    let mut out = OpenOptions::new().create_new(true).write(true).open(path)?;
    out.write_all(&bytes)?;
    println!("5 continuous rank repairs:4 off-curve,1 valid;1 zero-coefficient direction; exact norms checked");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_continuous_repairs_and_full_norms() {
        control();
    }
    #[test]
    fn square_constraint_is_exact_for_all_field_pairs() {
        for z in 0..P {
            for delta in 0..P {
                assert_eq!(
                    power(add(z, delta), 2) == power(z, 2),
                    delta == 0 || delta == sub(0, mul(2, z))
                );
            }
        }
    }
}
