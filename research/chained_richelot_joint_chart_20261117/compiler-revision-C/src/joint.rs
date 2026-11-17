use crate::{fp2::Fp2Ctx, fp_poly::FpPoly};
use num_bigint::BigUint;
use num_traits::{ToPrimitive, Zero};

type E = FpPoly;
type Poly = Vec<E>;
type Result<T, Error = String> = std::result::Result<T, Error>;
const P: u64 = 101;

// Exact original struct and every method body, pinned by the extraction receipt.
include!("../reused-core/cubic_core.rs.inc");

struct Fixture {
    name: String,
    lambda: u64,
    cubic: [u64; 3],
    j: usize,
    delta: u64,
    source: [u64; 7],
    target: [u64; 7],
}
fn ensure(condition: bool, reason: impl Into<String>) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(reason.into())
    }
}
fn base(coefficients: &[u64]) -> FpPoly {
    FpPoly::from_coeffs(
        coefficients.iter().copied().map(BigUint::from).collect(),
        BigUint::from(P),
    )
}
fn coeffs(a: &FpPoly) -> Vec<u64> {
    a.coeffs.iter().map(|c| c.to_u64().unwrap()).collect()
}
fn lift(k: &CubicField, a: &FpPoly) -> Poly {
    a.coeffs.iter().map(|c| k.c(c.to_u64().unwrap())).collect()
}
fn scale(k: &CubicField, a: &Poly, c: &E) -> Poly {
    k.trim(a.iter().map(|x| k.mul(x, c)).collect())
}
fn descent(k: &CubicField, a: &Poly) -> Result<FpPoly> {
    ensure(
        a.iter().all(|c| c.degree().is_none_or(|d| d == 0)),
        "coefficient did not descend",
    )?;
    Ok(base(&a.iter().map(|c| k.base(c)).collect::<Vec<_>>()))
}
fn squarefree(k: &CubicField, a: &FpPoly) -> Result<()> {
    let derivative = descent(k, &k.derivative(&lift(k, a)))?;
    ensure(
        a.degree() == Some(6) && a.gcd(&derivative) == base(&[1]),
        "not a squarefree sextic",
    )
}
fn power(a: &FpPoly, n: usize) -> FpPoly {
    (0..n).fold(base(&[1]), |out, _| out.mul(a))
}
/// Independent base-sextic binary substitution using the inherited polynomial API.
fn binary(a: &FpPoly, shift: u64, forward: bool) -> FpPoly {
    let (upper, lower) = if forward {
        (base(&[1, shift]), base(&[0, 1]))
    } else {
        (base(&[1]), base(&[(P - shift) % P, 1]))
    };
    (0..=6).fold(base(&[]), |out, i| {
        out.add(
            &power(&upper, i)
                .mul(&power(&lower, 6 - i))
                .scalar_mul(&a.coeff(i)),
        )
    })
}
/// The ascending 3x3 coefficient matrix in PROOF_13, including a degree drop.
fn binary2(k: &CubicField, q: &Poly, a: u64) -> Poly {
    assert!(q.len() <= 3);
    let c = |i| q.get(i).cloned().unwrap_or_else(|| k.z());
    k.trim(vec![
        c(2),
        c(1).add(&k.mul(&k.c(2 * a), &c(2))),
        c(0).add(&k.mul(&k.c(a), &c(1)))
            .add(&k.mul(&k.c(a * a), &c(2))),
    ])
}
fn value(k: &CubicField, q: &Poly, x: &E) -> E {
    q.iter().rev().fold(k.z(), |out, c| k.mul(&out, x).add(c))
}
fn encoded(k: &CubicField, q: &Poly) -> Vec<u64> {
    q.iter().map(|c| k.encoding(c)).collect()
}
fn factors(k: &CubicField, orbit: &[E; 3], j: usize) -> [Poly; 3] {
    std::array::from_fn(|i| {
        let a = &orbit[i];
        let b = &orbit[(i + j) % 3];
        vec![k.mul(a, b).neg(), b.sub(a), k.c(1)]
    })
}
fn validate_factors(k: &CubicField, q: &[Poly; 3], source: &FpPoly) -> Result<()> {
    ensure(
        k.product(q) == lift(k, source),
        "source factor product mismatch",
    )
}
fn root_nonzero(a: u64) -> Option<u64> {
    (1..P).find(|y| y * y % P == a)
}
fn fixtures() -> Result<Vec<Fixture>> {
    let mut lines = include_str!("../fixtures-13.csv").lines();
    ensure(
        lines.next()
            == Some("name,lambda,g0,g1,g2,j,delta,f0,f1,f2,f3,f4,f5,f6,h0,h1,h2,h3,h4,h5,h6"),
        "fixture header",
    )?;
    let mut out = Vec::new();
    for line in lines {
        let columns: Vec<_> = line.split(',').collect();
        ensure(columns.len() == 21, "fixture width")?;
        let mut v = Vec::new();
        for text in &columns[1..] {
            let n: u64 = text.parse().map_err(|_| "fixture integer")?;
            ensure(
                n < P && n.to_string() == *text,
                "noncanonical fixture integer",
            )?;
            v.push(n);
        }
        ensure([1, 2].contains(&v[4]), "fixture matching offset")?;
        out.push(Fixture {
            name: columns[0].into(),
            lambda: v[0],
            cubic: v[1..4].try_into().unwrap(),
            j: v[4] as usize,
            delta: v[5],
            source: v[6..13].try_into().unwrap(),
            target: v[13..20].try_into().unwrap(),
        });
    }
    let names: Vec<_> = out.iter().map(|r| r.name.as_str()).collect();
    ensure(
        names
            == [
                "Cpi1_j1", "Cpi1_j2", "Cpi2_j1", "Cpi2_j2", "C10_j1", "C10_j2", "C20_j1", "C20_j2",
            ],
        "fixture population/order",
    )?;
    Ok(out)
}

pub fn run_unit() -> Result<()> {
    let mut candidates = 0;
    let mut completed = 0;
    let mut degenerate = 0;
    let mut controls = 0;
    let mut permutations = 0;
    let mut signs = 0;
    let mut missing = Vec::new();
    for row in fixtures()? {
        let k = CubicField::with_cubic(P, row.cubic)?;
        let t = FpPoly::x(BigUint::from(P));
        let t1 = k.pow(&t, P);
        let t2 = k.pow(&t1, P);
        let orbit = [t, t1, t2];
        ensure(
            k.pow(&orbit[2], P) == orbit[0]
                && orbit[0] != orbit[1]
                && orbit[1] != orbit[2]
                && orbit[2] != orbit[0],
            format!("{} cubic orbit", row.name),
        )?;
        let q = factors(&k, &orbit, row.j);
        let delta = k.determinant(&q);
        ensure(
            !delta.is_zero() && delta == k.c(row.delta),
            format!("{} original determinant", row.name),
        )?;
        let f = base(&row.source);
        let h = base(&row.target);
        squarefree(&k, &f)?;
        squarefree(&k, &h)?;
        ensure(
            scale(&k, &k.product(&q), &k.c(row.lambda)) == lift(&k, &f),
            format!("{} original source factors", row.name),
        )?;
        for (i, factor) in q.iter().enumerate() {
            for (c, coefficient) in factor.iter().enumerate() {
                ensure(
                    k.pow(coefficient, P) == q[(i + 1) % 3][c],
                    "original factor Frobenius",
                )?;
            }
        }
        let original_brackets = k.dual_factors(&q, &k.c(1));
        let old_raw = k.product(&k.dual_factors(&q, &delta));
        ensure(
            row.lambda != 0 && row.lambda != 1,
            "lambda control availability",
        )?;
        let original_target = scale(&k, &old_raw, &k.inv(&k.c(row.lambda)));
        ensure(
            original_target == lift(&k, &h),
            format!("{} original literal target", row.name),
        )?;
        println!(
            "original_joint13,{},q={:?},brackets={:?},delta={},target={:?}",
            row.name,
            q.each_ref().map(|p| encoded(&k, p)),
            original_brackets.each_ref().map(|p| encoded(&k, p)),
            k.encoding(&delta),
            coeffs(&descent(&k, &original_target)?)
        );
        if row.j == 1 {
            let d0 = k.determinant(&factors(&k, &orbit, 0));
            ensure(d0.is_zero(), "shift0 must have determinant0")?;
            println!(
                "control_joint13,{},j0,determinant0,rejected_before_inverse",
                row.name
            );
            degenerate += 1;
        }
        let mut choices = Vec::new();
        for a in 0..P {
            let fa = f.eval(&BigUint::from(a)).to_u64().unwrap();
            let ha = h.eval(&BigUint::from(a)).to_u64().unwrap();
            let b = root_nonzero(fa);
            let c = root_nonzero(ha);
            let eligible = b.is_some() && c.is_some();
            println!(
                "candidate_joint13,{},a={a},f={fa},H={ha},b={b:?},c={c:?},eligible={eligible}",
                row.name
            );
            if let (Some(b), Some(c)) = (b, c) {
                choices.push((a, b, c));
            }
            candidates += 1;
        }
        let Some(&(a, b, c)) = choices.first() else {
            println!(
                "bounded_absence_joint13,{},all101_candidates_exhausted",
                row.name
            );
            missing.push(row.name);
            continue;
        };
        let alpha: [E; 3] = std::array::from_fn(|i| value(&k, &q[i], &k.c(a)));
        ensure(alpha.iter().all(|v| !v.is_zero()), "nonbranch alpha")?;
        let s = k.mul(&k.mul(&alpha[0], &alpha[1]), &alpha[2]);
        ensure(
            s == k.mul(&k.c(b * b), &k.inv(&k.c(row.lambda))),
            "norm S=b^2/lambda",
        )?;
        let normalized: [Poly; 3] =
            std::array::from_fn(|i| scale(&k, &binary2(&k, &q[i], a), &k.inv(&alpha[i])));
        let delta_prime = k.determinant(&normalized);
        let predicted_delta = k.mul(&delta.neg(), &k.inv(&s));
        ensure(
            !delta_prime.is_zero() && delta_prime == predicted_delta,
            "transformed determinant sign/scale",
        )?;
        let bi2 = k.base(&k.inv(&k.c(b * b)));
        let ci2 = k.base(&k.inv(&k.c(c * c)));
        let f_model = binary(&f, a, true).scalar_mul(&BigUint::from(bi2));
        let g_model = binary(&h, a, true).scalar_mul(&BigUint::from(ci2));
        ensure(
            f_model.coeff(6) == BigUint::from(1u64) && g_model.coeff(6) == BigUint::from(1u64),
            "new model monicity",
        )?;
        squarefree(&k, &f_model)?;
        squarefree(&k, &g_model)?;
        validate_factors(&k, &normalized, &f_model)?;
        ensure(
            binary(&f_model, a, false).scalar_mul(&BigUint::from(b * b)) == f
                && binary(&g_model, a, false).scalar_mul(&BigUint::from(c * c)) == h,
            "inverse model substitution",
        )?;
        let transformed_roots: [E; 6] = std::array::from_fn(|i| {
            let old = if i < 3 {
                orbit[i].clone()
            } else {
                orbit[i - 3].neg()
            };
            let denominator = old.sub(&k.c(a));
            assert!(!denominator.is_zero());
            k.inv(&denominator)
        });
        let mut permutation = [0; 6];
        for (i, root) in transformed_roots.iter().enumerate() {
            ensure(
                !transformed_roots[..i].contains(root),
                "six distinct transformed branch roots",
            )?;
            ensure(
                value(&k, &lift(&k, &f_model), root).is_zero(),
                "transformed source branch root",
            )?;
            permutation[i] = transformed_roots
                .iter()
                .position(|r| *r == k.pow(root, P))
                .ok_or("branch Frobenius closure")?;
        }
        ensure(
            permutation == [1, 2, 0, 4, 5, 3],
            "transformed Frobenius permutation",
        )?;
        for (i, factor) in normalized.iter().enumerate() {
            let u = &transformed_roots[i];
            let v = &transformed_roots[3 + (i + row.j) % 3];
            let root_factor = vec![k.mul(u, v), u.add(v).neg(), k.c(1)];
            ensure(
                root_factor == *factor,
                "root vs coefficient factor transform",
            )?;
            for (index, coefficient) in factor.iter().enumerate() {
                ensure(
                    k.pow(coefficient, P) == normalized[(i + 1) % 3][index],
                    "normalized factor Frobenius",
                )?;
            }
        }
        let new_brackets = k.dual_factors(&normalized, &k.c(1));
        for (i, bracket) in new_brackets.iter().enumerate() {
            let denominator = k.mul(&alpha[(i + 1) % 3], &alpha[(i + 2) % 3]);
            ensure(!denominator.is_zero(), "bracket scale denominator")?;
            let predicted = scale(
                &k,
                &binary2(&k, &original_brackets[i], a),
                &k.inv(&denominator).neg(),
            );
            ensure(*bracket == predicted, "complete bracket covariance")?;
        }
        let raw_new = k.product(&k.dual_factors(&normalized, &delta_prime));
        let predicted_raw = binary(&h, a, true).scalar_mul(&BigUint::from(b * b));
        ensure(raw_new == lift(&k, &predicted_raw), "raw target covariance")?;
        let descended = descent(&k, &raw_new)?;
        let eta = b * c % P;
        let eta2 = eta * eta % P;
        ensure(
            eta != 0 && descended.coeff(6) == BigUint::from(eta2),
            "raw target leading square",
        )?;
        let leading_inverse = k.base(&k.inv(&k.c(eta2)));
        ensure(
            descended.scalar_mul(&BigUint::from(leading_inverse)) == g_model,
            "complete normalized monic target",
        )?;
        let reorderings = [
            ([0, 1, 2], false),
            ([0, 2, 1], true),
            ([1, 0, 2], true),
            ([1, 2, 0], false),
            ([2, 0, 1], false),
            ([2, 1, 0], true),
        ];
        for (order, odd) in reorderings {
            let triple = order.map(|i| normalized[i].clone());
            let det = k.determinant(&triple);
            ensure(
                det == if odd {
                    delta_prime.neg()
                } else {
                    delta_prime.clone()
                },
                "reordering determinant parity",
            )?;
            let reordered = k.product(&k.dual_factors(&triple, &det));
            ensure(
                reordered == raw_new,
                "valid factor reordering changed target",
            )?;
            println!(
                "valid_permutation_joint13,{},order={order:?},odd={odd},delta={},raw_target={:?}",
                row.name,
                k.encoding(&det),
                coeffs(&descent(&k, &reordered)?)
            );
            permutations += 1;
        }
        for bb in [b, P - b] {
            for cc in [c, P - c] {
                let bs = k.base(&k.inv(&k.c(bb * bb)));
                let cs = k.base(&k.inv(&k.c(cc * cc)));
                let signed_eta = bb * cc % P;
                ensure(
                    binary(&f, a, true).scalar_mul(&BigUint::from(bs)) == f_model
                        && binary(&h, a, true).scalar_mul(&BigUint::from(cs)) == g_model
                        && signed_eta * signed_eta % P == eta2,
                    "valid sign choice changed models",
                )?;
                println!("valid_sign_joint13,{},b={bb},c={cc},scale_bc={signed_eta},leading_square={eta2}",row.name);
                signs += 1;
            }
        }
        ensure(old_raw != lift(&k, &h), "omitted lambda control undetected")?;
        println!(
            "control_joint13,{},omit_lambda,actual={:?},expected={:?},detected",
            row.name,
            coeffs(&descent(&k, &old_raw)?),
            row.target
        );
        controls += 1;
        let bad_raw = k.product(&k.dual_factors(&normalized, &delta_prime.neg()));
        let bad_model = descent(&k, &bad_raw)?.scalar_mul(&BigUint::from(leading_inverse));
        ensure(
            bad_model != g_model,
            "wrong determinant sign control undetected",
        )?;
        println!("control_joint13,{},flip_Delta_prime,actual={:?},expected={:?},fixed_normalization_detected",row.name,coeffs(&bad_model),coeffs(&g_model));
        controls += 1;
        let mut bad_factors = normalized.clone();
        bad_factors[0][0] = bad_factors[0][0].add(&k.c(1));
        let reason = validate_factors(&k, &bad_factors, &f_model)
            .expect_err("mutated source factor must reject");
        println!("control_joint13,{},mutate_Q0_constant,rejection={reason},actual_product={:?},expected={:?}",row.name,encoded(&k,&k.product(&bad_factors)),coeffs(&f_model));
        controls += 1;
        let canonical_scale = root_nonzero(eta2).ok_or("constructor target scale absent")?;
        println!("model_joint13,{},eligible_points={},a={a},b={b},c={c},S={},Delta_prime={},bc={eta},constructor_scale={canonical_scale},source={:?},target={:?},Q={:?},B_prime={:?},roots={:?},Frobenius={permutation:?}",row.name,choices.len(),k.encoding(&s),k.encoding(&delta_prime),coeffs(&f_model),coeffs(&g_model),normalized.each_ref().map(|q|encoded(&k,q)),new_brackets.each_ref().map(|q|encoded(&k,q)),transformed_roots.each_ref().map(|v|k.encoding(v)));
        completed += 1;
    }
    ensure(
        missing.is_empty(),
        format!("bounded joint-chart absence {missing:?}; complete808-point universe retained"),
    )?;
    ensure(
        (
            completed,
            candidates,
            permutations,
            signs,
            degenerate,
            controls,
        ) == (8, 808, 48, 32, 4, 24),
        "unit coverage",
    )?;
    println!("joint13=accepted;models={completed};candidates={candidates};valid_permutations={permutations};valid_signs={signs};degenerate_controls={degenerate};semantic_controls={controls};field_core=reused;factor_bridge_only;unsigned");
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn frozen_joint_chart_factor_covariance_13() {
        super::run_unit().unwrap();
    }
}
