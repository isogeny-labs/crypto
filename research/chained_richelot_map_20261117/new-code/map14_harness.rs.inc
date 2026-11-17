// New protocol14 harness; all preceding correspondence method bodies are reused.
const P14: u64 = 101;
const SOURCE14: [u64; 7] = [97, 29, 96, 18, 87, 99, 1];
const TARGET14: [u64; 7] = [16, 53, 24, 80, 73, 57, 1];
const ROOTS14: [u64; 6] = [1008151, 720554, 352389, 357036, 975881, 738000];
const PAIRING14: super::Pairing = [(0, 4), (1, 5), (2, 3)];

#[derive(Clone, Copy, Debug)]
enum Mutation14 {
    None,
    DuplicateRoot,
    SourceConstant,
}

fn assessment14(old: bool, mutation: Mutation14) -> Result<Assessment> {
    let field = CubicField::with_cubic(P14, [3, 26, 3])?;
    let mut roots: Vec<_> = ROOTS14
        .iter()
        .map(|&n| {
            let w = field.element(n);
            if old {
                field.c(3).add(&field.inv(&w))
            } else {
                w
            }
        })
        .collect();
    let permutation = [1, 2, 0, 4, 5, 3];
    for (i, root) in roots.iter().enumerate() {
        assert_eq!(field.pow(root, P14), roots[permutation[i]]);
    }
    let pairs: Vec<_> = super::pairings()
        .into_iter()
        .filter(|pairing| super::stable(pairing, &permutation))
        .collect();
    assert_eq!(pairs.len(), 3);
    let mut source = if old {
        let inverse = field.base(&field.inv(&field.c(84)));
        [52, 0, 25, 0, 77, 0, 84]
            .into_iter()
            .map(|a| a * inverse % P14)
            .collect()
    } else {
        SOURCE14.to_vec()
    };
    match mutation {
        Mutation14::None => {}
        Mutation14::DuplicateRoot => roots[5] = roots[0].clone(),
        Mutation14::SourceConstant => source[0] = (source[0] + 1) % P14,
    }
    Ok(Assessment {
        field,
        roots,
        pairs,
        source,
        targets: vec![],
        degenerate: 0,
    })
}

fn prime_inverse14(a: u64) -> u64 {
    let field = CubicField::with_cubic(P14, [3, 26, 3]).unwrap();
    field.base(&field.inv(&field.c(a)))
}

fn signed14(c: &Balanced, d: &Class, sign: u64) -> Result<Class> {
    match sign {
        1 => Ok(d.clone()),
        100 => c.neg(d),
        _ => Err("conjugacy sign is not algebraically plus/minus one".into()),
    }
}

fn assert_model14(c: &Balanced, coefficients: &[u64]) {
    let expected = FpPoly::from_coeffs(
        coefficients.iter().copied().map(BigUint::from).collect(),
        BigUint::from(P14),
    );
    assert_eq!(c.equation_and_infinity().0, &expected);
}

fn run_unit14() -> Result<()> {
    use crate::balanced::{witnesses14, Bridge14};
    let map =
        RichelotMap::from_selected_pairing(assessment14(false, Mutation14::None)?, PAIRING14)?;
    assert_eq!(map.target_scale, 13);
    assert_model14(&map.source, &SOURCE14);
    assert_model14(&map.target, &TARGET14);
    let old = RichelotMap::from_selected_pairing(assessment14(true, Mutation14::None)?, PAIRING14)?;
    let source_bridge = Bridge14::new(&old.source, 3, 96)?;
    assert_model14(source_bridge.target(), &SOURCE14);
    let old_h_leading = old.target_scale * old.target_scale % P14 * prime_inverse14(84) % P14;
    let s = (1..P14)
        .find(|x| x * x % P14 == old_h_leading)
        .ok_or("old raw H leading has no prescribed base-field square root")?;
    let normalized_c = 4 * prime_inverse14(s) % P14;
    let target_bridge = Bridge14::new(&old.target, 3, normalized_c)?;
    assert_model14(target_bridge.target(), &TARGET14);
    // Forced minus from x-z=-(w-v)/(wv), fixed before divisor comparisons.
    let unsigned =
        96 * normalized_c % P14 * old.target_scale % P14 * prime_inverse14(map.target_scale) % P14;
    let kappa = (P14 - unsigned) % P14;
    assert!([1, P14 - 1].contains(&kappa));
    println!("CT14|normalization|old_scale={}|s={s}|c_normal={normalized_c}|new_scale={}|kappa={kappa}|source={SOURCE14:?}|target={TARGET14:?}",old.target_scale,map.target_scale);

    let witnesses = witnesses14(&map.source)?;
    assert_eq!(witnesses.len(), 17);
    let mut images = Vec::new();
    let mut closed_quadratic = 0;
    let mut conjugacies = 0;
    for (i, (label, d)) in witnesses.iter().enumerate() {
        println!("CT14|attempt|witness|i={i}|label={label}");
        map.source.check(d)?;
        let image = map.forward(d)?;
        map.target.check(&image)?;
        let (closed, route) = map.evaluate_closed(d, false)?;
        assert_eq!(closed, image, "forward closed mismatch at {label}");
        if matches!(
            label.as_str(),
            "quadratic_orbit" | "opposite_quadratic_orbit"
        ) {
            assert_eq!(route, closed_fibres::ClosedRoute::Quadratic);
            closed_quadratic += 1;
        }
        let dual = map.dual(&image)?;
        let doubled = map.source.mul(d, 2)?;
        assert_eq!(dual, doubled, "positive dual mismatch at {label}");
        assert_eq!(map.dual_closed(&image)?, dual);
        assert_eq!(map.forward(&dual)?, map.target.mul(&image, 2)?);
        assert_eq!(map.source.mul(&dual, 6159)?, *d);

        let old_d = source_bridge.backward(d)?;
        assert_eq!(source_bridge.forward(&old_d)?, *d);
        let old_image = old.forward(&old_d)?;
        let transported = signed14(&map.target, &target_bridge.forward(&old_image)?, kappa)?;
        assert_eq!(
            transported, image,
            "fixed-sign conjugacy mismatch at {label}"
        );
        let old_return = old.dual(&target_bridge.backward(&image)?)?;
        let chain_return = signed14(&map.source, &source_bridge.forward(&old_return)?, kappa)?;
        assert_eq!(
            chain_return, dual,
            "actual return-chain mismatch at {label}"
        );
        conjugacies += 1;
        println!("CT14|witness|i={i}|label={label}|source={d:?}|image={image:?}|dual={dual:?}|closed_route={route:?}");
        println!("CT14|conjugacy|i={i}|old_source={old_d:?}|old_image={old_image:?}|transported={transported:?}|chain_return={chain_return:?}");
        images.push(image);
    }
    assert_eq!(closed_quadratic, 2);
    let mut pairs = 0;
    for (i, ((_, left), left_image)) in witnesses.iter().zip(&images).enumerate() {
        for j in [i, (i + 1) % witnesses.len()] {
            println!("CT14|attempt|pair|i={i}|j={j}");
            let sum = map.source.add(left, &witnesses[j].1)?;
            map.source.check(&sum)?;
            let image = map.forward(&sum)?;
            let expected = map.target.add(left_image, &images[j])?;
            assert_eq!(image, expected, "homomorphism pair {i},{j}");
            assert_eq!(map.forward_closed(&sum)?, image);
            println!("CT14|pair|i={i}|j={j}|source_sum={sum:?}|image_sum={image:?}");
            pairs += 1;
        }
    }
    assert_eq!(pairs, 34);

    let mut scalar_cases = 0;
    let mut wrong_sign_controls = 0;
    for q in [109, 113] {
        println!("CT14|attempt|prime|q={q}");
        let mut chosen = None;
        for (i, (_, d)) in witnesses.iter().enumerate() {
            let g = map.source.mul(d, 12317 / q)?;
            if g != map.source.zero() {
                chosen = Some((i, g));
                break;
            }
        }
        let (index, g) = chosen.ok_or("no nonzero generator in frozen witness population")?;
        assert_eq!(map.source.mul(&g, q)?, map.source.zero());
        let image = map.forward(&g)?;
        assert_ne!(image, map.target.zero());
        assert_eq!(map.target.mul(&image, q)?, map.target.zero());
        let correct_dual = map.dual(&image)?;
        assert_eq!(correct_dual, map.source.mul(&g, 2)?);
        assert_ne!(map.source.neg(&correct_dual)?, map.source.mul(&g, 2)?);
        wrong_sign_controls += 1;
        println!("CT14|generator|q={q}|first_witness={index}|source={g:?}|image={image:?}");
        println!("CT14|control|wrong_dual_sign|q={q}|detected=true");
        for scalar in [0, 1, 2, 37, q - 1] {
            println!("CT14|attempt|scalar|q={q}|scalar={scalar}");
            let d = map.source.mul(&g, scalar)?;
            let actual = map.forward(&d)?;
            assert_eq!(actual, map.target.mul(&image, scalar)?);
            assert_eq!(map.forward_closed(&d)?, actual);
            let dual = map.dual(&actual)?;
            assert_eq!(dual, map.source.mul(&d, 2)?);
            assert_eq!(map.source.mul(&dual, 6159)?, d);
            println!(
                "CT14|scalar|q={q}|scalar={scalar}|source={d:?}|image={actual:?}|dual={dual:?}"
            );
            scalar_cases += 1;
        }
    }
    assert_eq!(scalar_cases, 10);
    assert_eq!(wrong_sign_controls, 2);
    let malformed = [
        (
            "repeated_pairing",
            Mutation14::None,
            [(0, 4), (1, 5), (2, 4)],
        ),
        ("duplicate_root", Mutation14::DuplicateRoot, PAIRING14),
        ("source_constant", Mutation14::SourceConstant, PAIRING14),
    ];
    let mut constructor_controls = 0;
    for (label, mutation, pairing) in malformed {
        println!("CT14|attempt|constructor_control|{label}");
        match RichelotMap::from_selected_pairing(assessment14(false, mutation)?, pairing) {
            Ok(_) => return Err(format!("malformed assessment accepted: {label}")),
            Err(error) => println!("CT14|control|{label}|detected=true|error={error}"),
        }
        constructor_controls += 1;
    }
    assert_eq!(constructor_controls, 3);
    println!("CT14|accepted|witnesses={}|ordered_pairs={pairs}|conjugacies={conjugacies}|closed_quadratic={closed_quadratic}|prime_generators=2|scalar_cases={scalar_cases}|constructor_controls={constructor_controls}|wrong_sign_controls={wrong_sign_controls}|kappa={kappa}|no_DLP_solver|unsigned",witnesses.len());
    Ok(())
}

#[cfg(test)]
mod tests_14 {
    #[test]
    fn frozen_joint_chart_divisor_map_14() {
        super::run_unit14().unwrap();
    }
}
