use crate::field_b::*;
use crate::ring_b::*;
const INPUT: &str = include_str!("first-A-native/norm-quartic.stdout");
fn actual_coefficients() -> [R; 5] {
    let u = R::term(1, 0, 1);
    let v = R::term(0, 1, 1);
    let sections = [
        u.minus(&R::constant(5)),
        v.minus(&R::constant(6)),
        u.power(2).minus(&R::constant(4)),
        u.times(&v).minus(&R::constant(2)),
    ];
    let mut rows = Vec::new();
    for line in INPUT
        .lines()
        .filter(|s| s.starts_with("ACTUAL_PRIMITIVE_QUARTIC_COEFFICIENT\t"))
    {
        let field = |prefix: &str| {
            line.split('\t')
                .find_map(|s| s.strip_prefix(prefix))
                .unwrap()
        };
        let i: usize = field("T-power=").parse().unwrap();
        assert_eq!(i, rows.len());
        let c: Vec<u8> = field("vector=")
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        assert_eq!(c.len(), 4);
        assert!(c.iter().all(|&v| v < 7));
        rows.push(
            sections
                .iter()
                .zip(c)
                .fold(R::zero(), |a, (s, c)| a.plus(&s.scaled(c))),
        );
    }
    let coefficients: [R; 5] = rows.try_into().unwrap();
    assert!(!coefficients[4].0.is_empty());
    println!("ACTUAL_UNSCALED_QUARTIC\tcoefficients-ascending={coefficients:?}");
    coefficients
}
pub fn qualify() {
    let coefficients = actual_coefficients();
    let lead = &coefficients[4];
    // X=S4*(Y/X_source) gives this monic polynomial without denominators.
    let f: Vec<R> = coefficients
        .iter()
        .enumerate()
        .take(4)
        .map(|(i, c)| c.times(&lead.power(3 - i)))
        .chain(std::iter::once(R::constant(1)))
        .collect();
    let (a, b, c, d) = (&f[3], &f[2], &f[1], &f[0]);
    println!("ACTUAL_SCALED_PRIMITIVE_QUARTIC\tX=S4*Y/Xsource\tcoefficients-ascending={f:?}");
    let cubic = vec![
        b.times(d)
            .scaled(4)
            .minus(&a.power(2).times(d))
            .minus(&c.power(2)),
        a.times(c).minus(&d.scaled(4)),
        b.scaled(6),
        R::constant(1),
    ];
    println!("ACTUAL_PAIRING_CUBIC\tcoefficients-ascending={cubic:?}");
    let disc4 = discriminant("actual-scaled-quartic", &f);
    let disc3 = discriminant("actual-pairing-cubic", &cubic);
    assert!(!disc4.0.is_empty());
    assert_eq!(disc4, disc3);
    println!(
        "ACTUAL_RESOLVENT_DISCRIMINANTS\tequal=true\tnonzero=true\tfull-coefficients={disc4:?}"
    );
    let p = b.minus(&a.power(2).scaled(mul(3, inv(8 % 7))));
    let q = c
        .minus(&a.times(b).scaled(inv(2)))
        .plus(&a.power(3).scaled(inv(8 % 7)));
    let r = d
        .minus(&a.times(c).scaled(inv(4)))
        .plus(&a.power(2).times(b).scaled(inv(16 % 7)))
        .minus(&a.power(4).scaled(mul(3, inv(4))));
    assert!(!q.0.is_empty());
    let quotient = vec![
        q.power(2).scaled(6),
        p.power(2).minus(&r.scaled(4)),
        p.scaled(2),
        R::constant(1),
    ];
    let shift_by = b.minus(&a.power(2).scaled(inv(4)));
    let shifted = shift(&cubic, &shift_by);
    assert_eq!(shifted, quotient);
    println!("ACTUAL_COMPLEMENT_QUOTIENT\tz=(s+a/2)^2\tpairing-variable=z+b-a^2/4\tcoefficients-ascending={quotient:?}\tfull-shifted-cubic={shifted:?}");
    let half_a = a.scaled(inv(2));
    let six = poly_add(
        &poly_add(
            &linear_power(&half_a, 6),
            &linear_power(&half_a, 4)
                .iter()
                .map(|c| c.times(&p).scaled(2))
                .collect::<Vec<_>>(),
        ),
        &poly_add(
            &linear_power(&half_a, 2)
                .iter()
                .map(|c| c.times(&p.power(2).minus(&r.scaled(4))))
                .collect::<Vec<_>>(),
            &[q.power(2).scaled(6)],
        ),
    );
    println!("ACTUAL_PAIR_SUM_SEXTIC\tcoefficients-ascending={six:?}");
    let t = K::root();
    let minus_t = t.scaled(&R::constant(6));
    let two_t = t.scaled(&R::constant(2));
    let mut shifted = vec![K::constant(R::zero()); 5];
    for (n, c) in f.iter().enumerate() {
        for (j, slot) in shifted.iter_mut().enumerate().take(n + 1) {
            *slot = slot.plus(&minus_t.power(n - j, &f).scaled(&c.scaled(binomial(n, j))));
        }
    }
    let mut g = vec![K::constant(R::zero()); 4];
    g[3] = shifted[4].clone();
    for i in (0..3).rev() {
        g[i] = shifted[i + 1].plus(&two_t.times(&g[i + 1], &f));
    }
    let remainder = shifted[0].plus(&two_t.times(&g[0], &f));
    assert_eq!(remainder, K::constant(R::zero()));
    println!("ACTUAL_INCIDENT_PAIR_CUBIC\tG_t(s)=F(s-t)/(s-2t)\tcoefficients-ascending={g:?}\tdivision-remainder={remainder:?}");
    let mut complement = vec![K::constant(R::zero()); 4];
    for (n, c) in g.iter().enumerate() {
        for (j, slot) in complement.iter_mut().enumerate().take(n + 1) {
            let coefficient = a.scaled(6).power(n - j).scaled(mul(
                6,
                if j.is_multiple_of(2) {
                    binomial(n, j)
                } else {
                    mul(6, binomial(n, j))
                },
            ));
            *slot = slot.plus(&c.scaled(&coefficient));
        }
    }
    let product = kpoly_mul(&g, &complement, &f);
    let expected: Vec<K> = six.iter().cloned().map(K::constant).collect();
    let residuals: Vec<K> = product
        .iter()
        .zip(&expected)
        .map(|(a, b)| a.plus(&b.scaled(&R::constant(6))))
        .collect();
    println!("ACTUAL_INCIDENT_COMPLEMENT_FACTOR\tcomplement={complement:?}\tproduct={product:?}\tfull-residuals={residuals:?}");
    assert_eq!(product, expected);
    let mut mutated = six.clone();
    mutated[0] = mutated[0].plus(&R::constant(1));
    let mutant: Vec<K> = mutated.into_iter().map(K::constant).collect();
    assert_ne!(product, mutant);
    assert_eq!(product, expected);
    println!("REAL_SEXTIC_CONSTANT_MUTANT\t+1-rejected\tpristine-restored\tno-splitting-roots-required\tproper-normalization-bridge-separate");
}
