//! Finite dense controls for the controller-private coefficient reduction.
//! Gaussian elimination is cubic here and capped at dimension 64. This is
//! not an implementation or benchmark of the cited sparse black-box bound.
use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};
use serde_json::json;
use torus_s4::{curve25519_prime, Curve, Field, Point};
type Result<T> = std::result::Result<T, String>;
fn dot(a: &[BigUint], b: &[BigUint], f: &Field) -> Result<BigUint> {
    if a.len() != b.len() {
        return Err("dot shape mismatch".into());
    }
    Ok(a.iter()
        .zip(b)
        .fold(BigUint::zero(), |s, (a, b)| f.add(&s, &f.mul(a, b))))
}
fn verify(c: &[Vec<BigUint>], v: &[BigUint], f: &Field) -> Result<()> {
    let cols = c.first().ok_or("empty matrix")?.len();
    if c.len() != v.len()
        || v.iter().all(BigUint::is_zero)
        || c.iter().any(|r| r.len() != cols)
        || c.iter().flatten().chain(v).any(|x| x >= &f.p)
    {
        return Err("invalid kernel shape/value".into());
    }
    for j in 0..cols {
        let s = c
            .iter()
            .zip(v)
            .fold(BigUint::zero(), |s, (row, x)| f.add(&s, &f.mul(&row[j], x)));
        if !s.is_zero() {
            return Err("kernel certificate fails".into());
        }
    }
    Ok(())
}
fn kernel(c: &[Vec<BigUint>], f: &Field) -> Result<(Vec<BigUint>, usize)> {
    let rows = c.len();
    let cols = c.first().ok_or("empty matrix")?.len();
    if rows > 64
        || cols > 64
        || c.iter().any(|r| r.len() != cols)
        || c.iter().flatten().any(|x| x >= &f.p)
    {
        return Err("dense-control dimension/value limit".into());
    }
    let mut a: Vec<Vec<_>> = (0..cols)
        .map(|j| c.iter().map(|r| r[j].clone()).collect())
        .collect();
    let mut pivots = vec![];
    for j in 0..rows {
        let Some(p) = (pivots.len()..cols).find(|i| !a[*i][j].is_zero()) else {
            continue;
        };
        let rank = pivots.len();
        a.swap(rank, p);
        let inv = f.inv(&a[rank][j])?;
        for x in &mut a[rank] {
            *x = f.mul(x, &inv);
        }
        for i in 0..cols {
            if i != rank {
                let scale = a[i][j].clone();
                for k in 0..rows {
                    a[i][k] = f.sub(&a[i][k], &f.mul(&scale, &a[rank][k]));
                }
            }
        }
        pivots.push(j);
    }
    let free = (0..rows)
        .find(|j| !pivots.contains(j))
        .ok_or("no nonzero left kernel")?;
    let mut v = vec![BigUint::zero(); rows];
    v[free] = 1u32.into();
    for (i, p) in pivots.iter().enumerate() {
        v[*p] = f.sub(&BigUint::zero(), &a[i][free]);
    }
    verify(c, &v, f)?;
    Ok((v, pivots.len()))
}
fn extract(
    c: &[Vec<BigUint>],
    v: &[BigUint],
    a: &[BigUint],
    b: &[BigUint],
    f: &Field,
) -> Result<BigUint> {
    verify(c, v, f)?;
    let denominator = dot(v, b, f)?;
    if denominator.is_zero() {
        return Err("inconclusive: secret coefficient annihilated".into());
    }
    Ok(f.mul(
        &f.sub(&BigUint::zero(), &dot(v, a, f)?),
        &f.inv(&denominator)?,
    ))
}
fn matrix(rows: &[&[u32]]) -> Vec<Vec<BigUint>> {
    rows.iter()
        .map(|r| r.iter().map(|x| (*x).into()).collect())
        .collect()
}
fn negate(p: Point, curve: &Curve) -> Point {
    p.map(|(u, v)| (u, curve.f.sub(&BigUint::zero(), &v)))
}
fn centered_times(curve: &Curve, p: &Point, n: &BigUint, ell: &BigUint) -> Result<Point> {
    if let Some(s) = n.to_u64() {
        curve.times(p, s)
    } else {
        let s = (ell - n)
            .to_u64()
            .ok_or("fixture scalar is not short signed integer")?;
        Ok(negate(curve.times(p, s)?, curve))
    }
}
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        return Err("usage: rankfree_control NEW_DIR".into());
    }
    let out = std::path::Path::new(&args[1]);
    std::fs::create_dir(out)?;
    std::fs::write(
        out.join("pre-call.json"),
        serde_json::to_vec_pretty(
            &json!({"purpose":"finite reduction controls; known secrets; no DLP attack or sparse-LA measurement","dense_dimension_limit":64,"small_field":5,"exhaustive_tag_vectors":125,"sampling_or_solver_campaign":false}),
        )?,
    )?;
    let f = Field::new(5u32.into())?;
    let c = matrix(&[&[1, 0], &[1, 0], &[1, 0]]);
    let (v, rank) = kernel(&c, &f)?;
    let mut annihilated = 0;
    let mut recovered = 0;
    let mut leaky = 0;
    for b0 in 0u32..5 {
        for b1 in 0u32..5 {
            for b2 in 0u32..5 {
                let b = vec![b0.into(), b1.into(), b2.into()];
                // Conditional on this fixed row matrix/targets, the private b's are
                // exhaustive uniform. Fixture target logarithm is 2, secret is 3.
                let a: Vec<_> = b
                    .iter()
                    .map(|b| f.sub(&2u32.into(), &f.mul(b, &3u32.into())))
                    .collect();
                if dot(&v, &b, &f)?.is_zero() {
                    annihilated += 1;
                    assert!(extract(&c, &v, &a, &b, &f).is_err());
                } else {
                    assert_eq!(extract(&c, &v, &a, &b, &f)?, BigUint::from(3u32));
                    recovered += 1;
                }
                let mut bad = vec![
                    f.sub(&b[1], &b[2]),
                    f.sub(&b[2], &b[0]),
                    f.sub(&b[0], &b[1]),
                ];
                if bad.iter().all(BigUint::is_zero) {
                    bad = vec![1u32.into(), 4u32.into(), 0u32.into()];
                }
                verify(&c, &bad, &f)?;
                assert!(dot(&bad, &b, &f)?.is_zero());
                assert!(extract(&c, &bad, &a, &b, &f).is_err());
                leaky += 1;
            }
        }
    }
    if (annihilated, recovered, leaky) != (25, 100, 125) {
        return Err("finite probability oracle mismatch".into());
    }
    let ell = (BigUint::from(1u32) << 252usize)
        + BigUint::parse_bytes(b"27742317777372353535851937790883648493", 10).unwrap();
    let ef = Field::new(ell.clone())?;
    let curve = Curve {
        f: Field::new(curve25519_prime())?,
        a: 486662u32.into(),
    };
    let raw = Some((
        9u32.into(),
        curve.f.sqrt(&curve.rhs(&9u32.into()))?.ok_or("base lift")?,
    ));
    let p = curve.times(&raw, 8)?;
    if p.is_none() {
        return Err("zero generator fixture".into());
    }
    let secret = BigUint::from(1234567u32);
    let public = curve.times(&p, 1234567)?;
    let base_logs: Vec<BigUint> = [5u32, 17, 39].into_iter().map(BigUint::from).collect();
    let base: Vec<Point> = [5u64, 17, 39]
        .into_iter()
        .map(|x| curve.times(&p, x))
        .collect::<Result<_>>()?;
    let mut rows = vec![];
    for (name, c) in [
        (
            "full_column_rank",
            matrix(&[&[1, 0, 0], &[0, 1, 0], &[0, 0, 1], &[1, 1, 1]]),
        ),
        (
            "rank_one",
            matrix(&[&[1, 0, 0], &[1, 0, 0], &[2, 0, 0], &[3, 0, 0]]),
        ),
    ] {
        let (v, rank) = kernel(&c, &ef)?;
        let b: Vec<BigUint> = [3u32, 7, 11, 13].into_iter().map(BigUint::from).collect();
        let mut a = vec![];
        for (row, bi) in c.iter().zip(&b) {
            let ti = dot(row, &base_logs, &ef)?;
            let ai = ef.sub(&ti, &ef.mul(bi, &secret));
            let mut lhs = None;
            for (coefficient, point) in row.iter().zip(&base) {
                lhs = curve.add(&lhs, &centered_times(&curve, point, coefficient, &ell)?)?;
            }
            let rhs = curve.add(
                &centered_times(&curve, &p, &ai, &ell)?,
                &centered_times(&curve, &public, bi, &ell)?,
            )?;
            if lhs != rhs {
                return Err("actual-curve row certificate fails".into());
            }
            a.push(ai);
        }
        let candidate = extract(&c, &v, &a, &b, &ef)?;
        let point = curve.times(
            &p,
            candidate.to_u64().ok_or("unexpected fixture candidate")?,
        )?;
        if candidate != secret || point != public {
            return Err("actual-curve final scalar check fails".into());
        }
        let zeros = vec![BigUint::zero(); c.len()];
        assert!(extract(&c, &zeros, &a, &b, &ef).is_err());
        let mut wrong = v.clone();
        wrong[0] = ef.add(&wrong[0], &1u32.into());
        assert!(verify(&c, &wrong, &ef).is_err());
        rows.push(json!({"case":name,"matrix_rank":rank,"rows":c.len(),"columns":c[0].len(),"kernel":v.iter().map(ToString::to_string).collect::<Vec<_>>(),"actual_curve_rows_checked":true,"scalar_candidate":candidate.to_string(),"final_curve_check":true}));
    }
    std::fs::write(
        out.join("post-call.json"),
        serde_json::to_vec_pretty(
            &json!({"schema":"torus-reduction.rankfree-control/v1","F5_rank":rank,"F5_tag_vectors":125,"tag_blind_kernel_zero_denominator":annihilated,"tag_blind_kernel_correct_candidates":recovered,"tag_dependent_kernel_zero_denominator":leaky,"actual_curve_controls":rows,"known_secret_controls_only":true,"dense_elimination_cubic_not_a_sparse_complexity_claim":true,"pcc_standing":"unverified"}),
        )?,
    )?;
    Ok(())
}
