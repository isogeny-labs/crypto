use num_bigint::BigUint;
use torus_s4::{Curve, Field, Point};
use torus_s4_tree::{actual_four_control, TorusParameters, TreeInput, TreeSystem};
fn y(p: &Point, f: &Field) -> BigUint {
    match p {
        None => 1u32.into(),
        Some((u, _)) => f.mul(
            &f.sub(u, &1u32.into()),
            &f.inv(&f.add(u, &1u32.into())).unwrap(),
        ),
    }
}
#[test]
fn semaev_y_formula_covers_all_group_pairs_and_identity() {
    let f = Field::new(13u32.into()).unwrap();
    let c = Curve {
        f: f.clone(),
        a: 4u32.into(),
    };
    let mut points = vec![None];
    for u in 0u32..13 {
        if let Some(v) = f.sqrt(&c.rhs(&u.into())).unwrap() {
            points.push(Some((u.into(), v.clone())));
            let neg = f.sub(&0u32.into(), &v);
            if neg != v {
                points.push(Some((u.into(), neg)));
            }
        }
    }
    let one = BigUint::from(1u32);
    for p in &points {
        for q in &points {
            let z = y(&c.add(p, q).unwrap(), &f);
            let x = y(p, &f);
            let yy = y(q, &f);
            let a = f.mul(
                &f.sub(&c.a, &2u32.into()),
                &f.mul(&f.sub(&f.square(&x), &one), &f.sub(&f.square(&yy), &one)),
            );
            let value = f.sub(
                &f.add(
                    &f.mul(&a, &f.sub(&f.square(&z), &one)),
                    &f.scale(
                        &f.sub(
                            &f.add(&f.add(&f.square(&x), &f.square(&yy)), &f.square(&z)),
                            &one,
                        ),
                        4,
                    ),
                ),
                &f.scale(&f.mul(&f.mul(&x, &yy), &z), 8),
            );
            assert_eq!(value, BigUint::from(0u32));
        }
    }
}
#[test]
fn actual_four_point_system_and_mutation() {
    let (p, input, tree, points) = actual_four_control().unwrap();
    let assignment = tree.witness(&input, &points).unwrap();
    tree.system.check(&assignment).unwrap();
    assert_eq!(tree.system.equations.len(), tree.system.variables.len() + 1);
    assert!(tree.system.equations.iter().all(|f| f.degree() <= 2));
    println!(
        "actual M={} variables={} equations={} raw_coordinates={} cofactor_class_lower_bound={}",
        p.m,
        tree.system.variables.len(),
        tree.system.equations.len(),
        4 * p.m,
        (p.m - 1) / 2
    );
    let mut bad = assignment;
    bad[0] = (&bad[0] + 1u32) % &p.f.p;
    assert!(tree.system.check(&bad).is_err());
    for (j, point) in points.iter().enumerate() {
        let u = &point.as_ref().unwrap().0;
        let den = p.f.square(&p.f.add(&p.f.scale(u, 2), &486662u32.into()));
        let x = p.f.sub(
            &p.f.mul(
                &p.f.scale(&p.f.square(&486662u32.into()), 2),
                &p.f.inv(&den).unwrap(),
            ),
            &1u32.into(),
        );
        assert_eq!(p.f.chebyshev(p.m, &x), p.betas[j]);
    }
}
#[test]
fn identity_intermediate_and_multiple_arities() {
    let p = TorusParameters::actual_four().unwrap();
    let c = Curve {
        f: p.f.clone(),
        a: 486662u32.into(),
    };
    let first = p.point(1, 31).unwrap();
    let (u, v) = first.as_ref().unwrap();
    let neg = Some((u.clone(), p.f.sub(&0u32.into(), v)));
    let points = vec![first, neg, p.point(2, 43).unwrap(), p.point(3, 47).unwrap()];
    assert_eq!(c.add(&points[0], &points[1]).unwrap(), None);
    let input =
        TreeInput::from_points(p.f.p.clone(), c.a.clone(), p.m, p.betas.clone(), &points).unwrap();
    let tree = TreeSystem::compile(&input).unwrap();
    tree.system
        .check(&tree.witness(&input, &points).unwrap())
        .unwrap();
    for arity in 2..=6 {
        let points = (0..arity)
            .map(|i| p.point((i % 4 + 1) as u32, 101 + 3 * i as u128).unwrap())
            .collect::<Vec<_>>();
        let input =
            TreeInput::from_points(p.f.p.clone(), c.a.clone(), p.m, p.betas.clone(), &points)
                .unwrap();
        let tree = TreeSystem::compile(&input).unwrap();
        tree.system
            .check(&tree.witness(&input, &points).unwrap())
            .unwrap();
        assert_eq!(tree.system.equations.len(), tree.system.variables.len() + 1);
    }
}

#[test]
fn branch_norm_arithmetic_and_shifted_constructions() {
    let p = TorusParameters::actual_four().unwrap();
    let record = torus_s4_tree::energy::arithmetic_certificate(&p).unwrap();
    assert_eq!(record["unit_norm_pairs"], 12);
    assert_eq!(
        record["K_times_W0_less_than_admissible_parameter_bound"],
        true
    );
    for t in [
        BigUint::from(0u32),
        1u32.into(),
        2u32.into(),
        17u32.into(),
        p.f.p.clone(),
    ] {
        let shifted = p.shifted(p.shift_parameter(&t).unwrap()).unwrap();
        let points = (1u32..=4)
            .map(|j| shifted.point(j, 301 + j as u128))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let input = TreeInput::from_points(
            shifted.f.p.clone(),
            486662u32.into(),
            shifted.m,
            shifted.betas.clone(),
            &points,
        )
        .unwrap();
        let tree = TreeSystem::compile(&input).unwrap();
        tree.system
            .check(&tree.witness(&input, &points).unwrap())
            .unwrap();
    }
}

#[test]
fn actual_prime_target_has_eight_checked_raw_sections() {
    let (_, input, _, _) = actual_four_control().unwrap();
    let curve = Curve::from_case(&input.curve).unwrap();
    let raw = input.curve.point().unwrap();
    let fiber = torus_s4_tree::transport::sections(&curve, &raw).unwrap();
    assert_eq!(fiber.raw_sections.len(), 8);
    assert!(curve.times(&fiber.torsion_generator, 8).unwrap().is_none());
    assert!(curve.times(&fiber.torsion_generator, 4).unwrap().is_some());
    for r in &fiber.raw_sections {
        assert_eq!(curve.times(r, 8).unwrap(), fiber.prime_target);
    }
}
