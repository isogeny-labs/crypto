//! Fixed public-input arithmetic controls, not branch selection or a timed solver.
use super::*;
fn neg(a: &Poly) -> Poly {
    scale(a, P - 1)
}
fn norm(f: &Poly, a: &Poly, b: &Poly) -> Poly {
    ps(&pm(a, a), &pm(f, &pm(b, b)))
}
fn derivative(a: &Poly) -> Poly {
    trim(
        a.iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| mul(i as u64 % P, *c))
            .collect(),
    )
}
fn interpolate(points: &[(u64, u64)]) -> Poly {
    let mut s = vec![];
    for (i, (x, y)) in points.iter().enumerate() {
        let mut l = vec![1];
        let mut d = 1;
        for (j, (z, _)) in points.iter().enumerate() {
            if i != j {
                assert_ne!(x, z);
                l = pm(&l, &vec![sub(0, *z), 1]);
                d = mul(d, sub(*x, *z));
            }
        }
        s = pa(&s, &scale(&l, mul(*y, inv(d))));
    }
    for (x, y) in points {
        assert_eq!(eval(&s, *x), *y);
    }
    s
}
fn kernel(mut m: Mat) -> Vec<Vec<u64>> {
    let cols = m[0].len();
    let mut pivots = vec![];
    for c in 0..cols {
        let row = pivots.len();
        if row == m.len() {
            break;
        }
        if let Some(k) = (row..m.len()).find(|k| m[*k][c] != 0) {
            m.swap(row, k);
            let pivot = inv(m[row][c]);
            for x in &mut m[row] {
                *x = mul(*x, pivot);
            }
            for r in 0..m.len() {
                if r == row {
                    continue;
                }
                let factor = m[r][c];
                for j in 0..cols {
                    m[r][j] = sub(m[r][j], mul(factor, m[row][j]));
                }
            }
            pivots.push(c);
        }
    }
    (0..cols)
        .filter(|c| !pivots.contains(c))
        .map(|c| {
            let mut v = vec![0; cols];
            v[c] = 1;
            for (r, p) in pivots.iter().enumerate() {
                v[*p] = sub(0, m[r][c]);
            }
            v
        })
        .collect()
}
fn reconstruct(points: &[(u64, u64)]) -> Value {
    let f = vec![2, 2, 0, 1];
    for (x, y) in points {
        assert_eq!(mul(*y, *y), eval(&f, *x));
        assert_ne!(*y, 0);
    }
    let g = roots(&points.iter().map(|p| p.0).collect::<Vec<_>>());
    let s = interpolate(points);
    assert!(remainder(&ps(&pm(&s, &s), &f), &g).is_empty());
    let rows: Mat = points
        .iter()
        .map(|(x, y)| {
            vec![
                1,
                *x,
                power(*x, 2),
                power(*x, 3),
                sub(0, *y),
                sub(0, mul(*y, *x)),
            ]
        })
        .collect();
    let basis = kernel(rows.clone());
    let mut recovered = vec![];
    for v in &basis {
        assert!(mv(&rows, v).iter().all(|x| *x == 0));
        let a = trim(v[..4].to_vec());
        let b = trim(v[4..].to_vec());
        assert!(!a.is_empty() || !b.is_empty());
        assert!(remainder(&ps(&a, &pm(&s, &b)), &g).is_empty());
        let n = norm(&f, &a, &b);
        assert!(!n.is_empty());
        assert!(n.len() <= 7);
        let c = coeff(&n, 6);
        assert_ne!(c, 0);
        assert_eq!(n, scale(&g, c));
        for (x, y) in points {
            assert_eq!(eval(&a, *x), mul(*y, eval(&b, *x)));
        }
        recovered.push(json!({"A":a,"B":b,"norm":n,"c":c}));
    }
    json!({"public_points":points,"support":g,"sqrt_residue":s,
        "kernel_dimension":basis.len(),"reconstructed":recovered})
}
pub(super) fn control() -> Value {
    let f = vec![2, 2, 0, 1];
    let g = vec![0, 1];
    let a = vec![6];
    let b = vec![1];
    let v = vec![9];
    let w = vec![16, 0, 8];
    let old = norm(&f, &a, &b);
    assert_eq!(pa(&pm(&f, &v), &pm(&g, &w)), vec![1]);
    assert_eq!(coeff(&old, 0), 0); // public root residue, not an exact norm certificate
    let mut cases = vec![];
    let mut nonzero_residual = 0;
    for d0 in 0..P {
        for d1 in 0..P {
            let delta = trim(vec![d0, d1]);
            let h = neg(&scale(&pm(&v, &delta), inv(2)));
            let new_b = pa(&b, &pm(&g, &h));
            let new = norm(&f, &a, &new_b);
            let residual = ps(&ps(&new, &old), &pm(&g, &delta));
            let expected = pm(
                &pm(&g, &g),
                &ps(&neg(&pm(&f, &pm(&h, &h))), &pm(&w, &delta)),
            );
            assert_eq!(residual, expected);
            assert!(remainder(&residual, &pm(&g, &g)).is_empty());
            assert_eq!(eval(&new_b, 0), 1);
            // Separate point and derivative formula for the first quotient jet.
            assert_eq!(eval(&new, 0), eval(&old, 0));
            assert_eq!(
                sub(eval(&derivative(&new), 0), eval(&derivative(&old), 0)),
                d0
            );
            // Test full polynomial identities at every field value as an extra control.
            for x in 0..P {
                assert_eq!(
                    eval(&new, x),
                    sub(mul(6, 6), mul(eval(&f, x), power(eval(&new_b, x), 2)))
                );
            }
            if !residual.is_empty() {
                nonzero_residual += 1;
            }
            cases.push(json!({"delta":delta,"B_new":new_b,"full_residual":residual}));
        }
    }
    // Target c=1 using R=(A^2-f)/G, with the same explicit public inverse.
    let r = old.iter().skip(1).copied().collect::<Poly>();
    let delta = ps(&vec![1], &r);
    let h = neg(&scale(&pm(&v, &delta), inv(2)));
    let lifted_b = pa(&b, &pm(&g, &h));
    let target_residual = ps(&norm(&f, &a, &lifted_b), &g);
    assert!(remainder(&target_residual, &pm(&g, &g)).is_empty());
    assert!(!target_residual.is_empty());
    assert!(lifted_b.len() > g.len());
    let positive = reconstruct(&[(0, 6), (3, 1), (5, 1), (6, 3), (10, 11), (13, 7)]);
    assert_eq!(positive["kernel_dimension"], 1);
    let changed = reconstruct(&[(0, 6), (3, 1), (5, 1), (6, 3), (10, 11), (13, 10)]);
    // Do not infer impossibility for other branch assignments from this outcome.
    json!({"schema":"norm.public-jet-controls/v1","p":P,"curve_coefficients":f,
        "support":g,"A":a,"B":b,"V":v,"W":w,"cases":cases,
        "delta_cases":289,"nonzero_full_residuals":nonzero_residual,
        "target_c1":{"delta":delta,"B_new":lifted_b,"full_residual":target_residual,
            "mod_G_squared_pass":true,"exact_norm_pass":false,"degree_window_pass":false},
        "public_branch_reconstruction":{"positive":positive,"flipped_branch":changed},
        "attack_solver_executed":false,"finite_linear_reconstruction":true,
        "timing_measured":false,"scope":"fixed development controls; supplied public branches; no sign selection, DLP search, yield or scaling claim",
        "standing":"unsigned producer evidence"})
}
pub(super) fn write(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = control();
    let data = serde_json::to_vec_pretty(&result)?;
    let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;
    file.write_all(&data)?;
    println!(
        "289 fixed jet controls; full residual retained; supplied-branch reconstruction checked"
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn jet_and_public_reconstruction_controls() {
        control();
    }
    #[test]
    fn low_degree_residue_rigidity_control() {
        let g = vec![3, 4, 1];
        let a = vec![6, 7];
        for c in 0..P {
            let changed = pa(&a, &scale(&g, c));
            assert_eq!(remainder(&changed, &g), a);
            assert_eq!(changed.len() < g.len(), c == 0);
        }
    }
    #[test]
    fn characteristic_two_is_excluded() {
        fn half(p: u64) -> Option<u64> {
            (0..p).find(|x| 2 * x % p == 1)
        }
        assert_eq!(half(2), None);
        assert_eq!(half(17), Some(9));
    }
    #[test]
    fn gaussian_kernel_is_checked_against_original_rows() {
        let m = vec![vec![1, 2, 3], vec![2, 4, 6]];
        let basis = kernel(m.clone());
        assert_eq!(basis.len(), 2);
        for v in basis {
            assert_eq!(mv(&m, &v), vec![0, 0]);
        }
        assert!(kernel(vec![vec![1, 0], vec![0, 1]]).is_empty());
    }
}
