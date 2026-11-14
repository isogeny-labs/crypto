//! Public barycentric Hankel representation controls; no branch search or timing.
use super::public_jets::{interpolate, kernel, norm};
use super::*;
pub(super) fn weights(points: &[(u64, u64)]) -> Vec<u64> {
    points
        .iter()
        .enumerate()
        .map(|(i, (x, _))| {
            let d = points
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .fold(1, |v, (_, (t, _))| mul(v, sub(*x, *t)));
            assert_ne!(d, 0);
            inv(d)
        })
        .collect()
}
pub(super) fn hankel(points: &[(u64, u64)], w: &[u64]) -> Mat {
    (0..2)
        .map(|j| {
            (0..2)
                .map(|k| {
                    points.iter().zip(w).fold(0, |s, ((x, y), v)| {
                        add(s, mul(mul(*v, *y), power(*x, j + k)))
                    })
                })
                .collect()
        })
        .collect()
}
fn full_rows(points: &[(u64, u64)]) -> Mat {
    points
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
        .collect()
}
fn compress(points: &[(u64, u64)]) -> Value {
    let f = vec![2, 2, 0, 1];
    let xs = points.iter().map(|(x, _)| *x).collect::<Vec<_>>();
    let g = roots(&xs);
    let s = interpolate(points);
    let w = weights(points);
    for (x, y) in points {
        assert_eq!(mul(*y, *y), eval(&f, *x));
        assert_ne!(*y, 0);
    }
    assert!(remainder(&ps(&pm(&s, &s), &f), &g).is_empty());
    let m = hankel(points, &w);
    assert_eq!(m[0][1], m[1][0]);
    let small_basis = kernel(m.clone());
    let rows = full_rows(points);
    let full_basis = kernel(rows.clone());
    assert_eq!(small_basis.len(), full_basis.len());
    assert_eq!(m.len(), 2);
    assert_eq!(rows.len(), 6);
    // Both directions of the actual kernel correspondence, no rank-only acceptance.
    for v in &full_basis {
        let b = trim(v[4..].to_vec());
        assert!(!b.is_empty());
        let a = trim(v[..4].to_vec());
        assert_eq!(mv(&m, &v[4..]), vec![0, 0]);
        assert_eq!(remainder(&pm(&s, &b), &g), a);
    }
    let mut candidates = vec![];
    for b in &small_basis {
        assert_ne!(*b, vec![0, 0]);
        assert_eq!(mv(&m, b), vec![0, 0]);
        let bp = trim(b.clone());
        let a = remainder(&pm(&s, &bp), &g);
        assert!(a.len() <= 4);
        assert!(bp.len() <= 2);
        let mut full = (0..4).map(|j| coeff(&a, j)).collect::<Vec<_>>();
        full.extend(b);
        assert_eq!(mv(&rows, &full), vec![0; 6]);
        let n = norm(&f, &a, &bp);
        let c = coeff(&n, 6);
        assert_ne!(c, 0);
        assert_eq!(n, scale(&g, c));
        for (x, y) in points {
            assert_eq!(eval(&a, *x), mul(*y, eval(&bp, *x)));
        }
        candidates.push(json!({"B_vector":b,"A":a,"B":bp,"c":c,"norm":n}));
    }
    let mut updates = vec![];
    for (t, (x, y)) in points.iter().enumerate() {
        let mut flipped = points.to_vec();
        flipped[t].1 = sub(0, *y);
        let changed = hankel(&flipped, &w);
        let delta = sub(flipped[t].1, *y);
        let v = vec![1, *x];
        let scalar = mul(delta, w[t]);
        for j in 0..2 {
            for k in 0..2 {
                assert_eq!(sub(changed[j][k], m[j][k]), mul(scalar, mul(v[j], v[k])));
            }
        }
        let adj = adjugate(&m);
        let av = mv(&adj, &v);
        let quad = v.iter().zip(&av).fold(0, |q, (a, b)| add(q, mul(*a, *b)));
        assert_eq!(det(&changed), add(det(&m), mul(scalar, quad)));
        updates.push(
            json!({"flipped_index":t,"x":x,"delta":delta,"outer_scale":scalar,
            "matrix":changed,"determinant":det(&changed),"cofactor_quadratic":quad}),
        );
    }
    // Wrong weight must fail the literal matrix and positive-candidate kernel check.
    let mut wrong = w.clone();
    wrong[0] = add(wrong[0], 1);
    let wrong_m = hankel(points, &wrong);
    assert_ne!(wrong_m, m);
    let rejected = small_basis.iter().all(|b| mv(&wrong_m, b) != vec![0, 0]);
    assert!(rejected);
    json!({"public_points":points,"weights":w,"sqrt_residue":s,"support":g,
        "full_rows":rows,"hankel":m,"full_kernel_dimension":full_basis.len(),
        "hankel_kernel_dimension":small_basis.len(),"hankel_determinant":det(&m),
        "reconstructed":candidates,"single_branch_update_controls":updates,
        "wrong_weight_matrix":wrong_m,"wrong_weight_candidate_checks":small_basis.len(),
        "wrong_weight_rejected":rejected})
}
pub(super) fn control() -> Value {
    let positive = compress(&[(0, 6), (3, 1), (5, 1), (6, 3), (10, 11), (13, 7)]);
    let flipped = compress(&[(0, 6), (3, 1), (5, 1), (6, 3), (10, 11), (13, 10)]);
    assert_eq!(positive["hankel_kernel_dimension"], 1);
    assert_eq!(flipped["hankel_kernel_dimension"], 0);
    json!({"schema":"norm.signed-hankel-controls/v1","p":17,"curve":{"a":2,"b":2},
        "positive":positive,"flipped_branch":flipped,"physical_artifact_invocations":1,
        "branch_search_executed":false,"timing_measured":false,
        "scope":"fixed finite equivalence and rank-one update controls; no sign selection, statistical yield, timing or asymptotic gain claim",
        "standing":"unsigned producer evidence"})
}
pub(super) fn write(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = control();
    let bytes = serde_json::to_vec_pretty(&result)?;
    let mut out = OpenOptions::new().write(true).create_new(true).open(path)?;
    out.write_all(&bytes)?;
    println!("public 6x6 / 2x2 kernel correspondence and branch update controls checked");
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn public_hankel_and_update_controls() {
        control();
    }
    #[test]
    fn cardinal_weights_annihilate_low_powers() {
        let points = [(0, 6), (3, 1), (5, 1), (6, 3), (10, 11), (13, 7)];
        let w = weights(&points);
        for j in 0..5 {
            assert_eq!(
                points
                    .iter()
                    .zip(&w)
                    .fold(0, |q, ((x, _), v)| add(q, mul(*v, power(*x, j)))),
                0
            );
        }
        assert_eq!(
            points
                .iter()
                .zip(&w)
                .fold(0, |q, ((x, _), v)| add(q, mul(*v, power(*x, 5)))),
            1
        );
    }
}
