use super::*;
use prime31_algebraic_ic::PRIME31;

// Test-only reference arithmetic uses separate formulas and u128 reductions.
// It enumerates only the declared finite leaf set, never F_p.
fn mul(a: u64, b: u64) -> u64 { ((a as u128 * b as u128) % M31 as u128) as u64 }
fn sub(a: u64, b: u64) -> u64 { (a + M31 - b) % M31 }
fn power(mut x: u64, mut n: u64) -> u64 {
    let mut y = 1;
    while n > 0 { if n & 1 == 1 { y = mul(y, x); } x = mul(x, x); n >>= 1; }
    y
}
fn reference_add(a: u64, left: Option<[u64; 2]>, right: Option<[u64; 2]>) -> Option<[u64; 2]> {
    let (p, q) = match (left, right) { (None, v) | (v, None) => return v, (Some(p), Some(q)) => (p, q) };
    if p[0] == q[0] && (p[1] + q[1]) % M31 == 0 { return None; }
    let (num, den) = if p == q { ((3 * mul(p[0], p[0]) + a) % M31, 2 * p[1] % M31) }
        else { (sub(q[1], p[1]), sub(q[0], p[0])) };
    assert_ne!(den, 0);
    let slope = mul(num, power(den, M31 - 2));
    let x = sub(sub(mul(slope, slope), p[0]), q[0]);
    Some([x, sub(mul(slope, sub(p[0], x)), p[1])])
}
fn selector(xs: &[u64]) -> Vec<u64> {
    let mut l = vec![1];
    for &x in xs {
        let mut next = vec![0; l.len() + 1];
        for (i, &c) in l.iter().enumerate() {
            next[i] = sub(next[i], mul(c, x));
            next[i + 1] = (next[i + 1] + c) % M31;
        }
        l = next;
    }
    l
}
fn fixture() -> (Problem, Vec<[u64; 2]>) {
    let g = [PRIME31.g.0, PRIME31.g.1];
    let twice = reference_add(PRIME31.a, Some(g), Some(g)).unwrap();
    let target = reference_add(PRIME31.a, Some(twice), Some(g)).unwrap();
    let l = selector(&[g[0], twice[0]]);
    (Problem { p: M31, a: PRIME31.a, b: PRIME31.b, target, selectors: [l.clone(), l.clone(), l] }, vec![g, twice])
}
fn brute(p: &Problem, points: &[Vec<[u64; 2]>; 3]) -> BTreeSet<SignedWitness> {
    let mut signed: [Vec<[u64; 2]>; 3] = std::array::from_fn(|_| Vec::new());
    for i in 0..3 { for point in &points[i] {
        signed[i].push(*point);
        if point[1] != 0 { signed[i].push([point[0], M31 - point[1]]); }
    } }
    let mut expected = BTreeSet::new();
    for &a in &signed[0] { for &b in &signed[1] { for &c in &signed[2] {
        if reference_add(p.a, reference_add(p.a, Some(a), Some(b)), Some(c)) == Some(p.target) {
            expected.insert(SignedWitness { points: [a, b, c] });
        }
    } } }
    expected
}
fn evaluate(poly: &Poly, values: &[u64]) -> u64 {
    poly.iter().fold(0, |sum, (m, c)| {
        let value = m.iter().zip(values).fold(*c, |v, (&e, &x)| mul(v, power(x, e as u64)));
        (sum + value) % M31
    })
}
fn scalar_s3(a: u64, b: u64, x: u64, y: u64, z: u64) -> u64 {
    let constant = sub(mul(sub(mul(x, y), a), sub(mul(x, y), a)), mul(4 * b % M31, (x + y) % M31));
    let middle = 2 * ((mul((x + y) % M31, (mul(x, y) + a) % M31) + 2 * b) % M31) % M31;
    (sub(mul(mul(sub(x, y), sub(x, y)), mul(z, z)), mul(middle, z)) + constant) % M31
}

#[test]
fn m31_selector_certificates_bind_original_equations_and_reject_corruption() {
    let (p, _) = fixture();
    for form in [Formulation::Expanded, Formulation::ChainLeft, Formulation::ChainRight] {
        let (charts, _) = build(&p, form, Preprocessing::RequireSelectorQuotient, Limits::default()).unwrap();
        for chart in charts {
            assert!(replay_preprocessing(&p, &chart.original, &chart));
            let mut changed = chart.clone();
            changed.preprocessing.as_mut().unwrap().relations[0].relation_index += 1;
            assert!(!replay_preprocessing(&p, &chart.original, &changed));
            let mut changed = chart.clone();
            let z = vec![0; changed.variables];
            changed.preprocessing.as_mut().unwrap().relations[0].cofactors[0] = vec![(z, 1)];
            assert!(!replay_preprocessing(&p, &chart.original, &changed));
            let mut wrong_selector = p.clone(); wrong_selector.selectors[0][0] = (wrong_selector.selectors[0][0] + 1) % M31;
            assert!(!replay_preprocessing(&wrong_selector, &chart.original, &chart));
            let mut original = chart.original.clone(); original[0] = vec![(vec![0; chart.variables], 1)];
            assert!(!replay_preprocessing(&p, &original, &chart));
        }
    }
}

#[test]
fn m31_s3_builder_matches_independent_scalar_formula() {
    let (p, _) = fixture();
    let (charts, _) = build(&p, Formulation::ChainLeft, Preprocessing::None, Limits::default()).unwrap();
    for values in [[0, 0, 0, 0], [1, 2, 3, 4], [M31 - 1, 17, 98, 65], [p.target[0], 7, 4, 13]] {
        assert_eq!(evaluate(&charts[0].original[0], &values), scalar_s3(p.a, p.b, values[0], values[1], values[3]));
        assert_eq!(evaluate(&charts[0].original[1], &values), scalar_s3(p.a, p.b, values[2], p.target[0], values[3]));
    }
}

#[test]
fn m31_expanded_s4_matches_independent_sylvester_determinant() {
    let (p, _) = fixture();
    let (charts, _) = build(&p, Formulation::Expanded, Preprocessing::None, Limits::default()).unwrap();
    let quadratic = |x, y| {
        let c = scalar_s3(p.a, p.b, x, y, 0);
        let plus = scalar_s3(p.a, p.b, x, y, 1);
        let minus = scalar_s3(p.a, p.b, x, y, M31 - 1);
        [c, mul(sub(plus, minus), (M31 + 1) / 2), mul(sub((plus + minus) % M31, 2 * c % M31), (M31 + 1) / 2)]
    };
    for xs in [[0, 0, 0], [1, 2, 3], [17, 17, p.target[0]], [M31 - 1, 25, 97]] {
        let f = quadratic(xs[0], xs[1]);
        let g = quadratic(xs[2], p.target[0]);
        let matrix = [[f[2], f[1], f[0], 0], [0, f[2], f[1], f[0]], [g[2], g[1], g[0], 0], [0, g[2], g[1], g[0]]];
        let mut determinant = 0;
        for a in 0..4 { for b in 0..4 { for c in 0..4 { for d in 0..4 {
            let perm = [a, b, c, d];
            if (0..4).any(|i| (i + 1..4).any(|j| perm[i] == perm[j])) { continue; }
            let inversions: usize = (0..4).map(|i| (i + 1..4).filter(|&j| perm[i] > perm[j]).count()).sum();
            let term = (0..4).fold(1, |v, i| mul(v, matrix[i][perm[i]]));
            determinant = if inversions % 2 == 0 { (determinant + term) % M31 } else { sub(determinant, term) };
        } } } }
        assert_eq!(evaluate(&charts[0].original[0], &xs), determinant);
    }
}

#[test]
fn m31_expanded_and_chained_signed_sets_equal_independent_finite_base_oracle() {
    let (p, points) = fixture();
    let expected = brute(&p, &[points.clone(), points.clone(), points]);
    assert!(!expected.is_empty());
    for form in [Formulation::Expanded, Formulation::ChainLeft, Formulation::ChainRight] {
        for pre in [Preprocessing::None, Preprocessing::RequireSelectorQuotient] {
            let actual = solve(&p, form, pre, Limits::default()).unwrap();
            assert_eq!(actual.status, Status::Complete, "{form:?}/{pre:?}: {:?}", actual.stop_reason);
            assert_eq!(actual.witnesses.iter().cloned().collect::<BTreeSet<_>>(), expected, "{form:?}/{pre:?}");
            for witness in &actual.witnesses { assert!(replay_witness(&p, witness)); }
            for chart in actual.charts {
                assert_eq!(chart.refused_enumeration_branches, 0);
                assert_eq!(chart.traversal.full_field_branches, 0);
                assert_eq!(chart.traversal.domain_evaluations, 0);
            }
        }
    }
}

#[test]
fn m31_infinity_intermediate_chart_recovers_canceling_pair() {
    let (mut p, points) = fixture();
    let g = points[0]; let twice = points[1];
    p.target = twice;
    p.selectors = [selector(&[g[0]]), selector(&[g[0]]), selector(&[twice[0]])];
    let expected = brute(&p, &[vec![g], vec![g], vec![twice]]);
    assert_eq!(expected.len(), 2);
    let actual = solve(&p, Formulation::ChainLeft, Preprocessing::RequireSelectorQuotient, Limits::default()).unwrap();
    assert_eq!(actual.status, Status::Complete, "{:?}", actual.stop_reason);
    assert!(actual.charts[0].algebraic_assignments.is_empty());
    assert_eq!(actual.charts[1].chart.name, "intermediate_at_infinity");
    assert!(!actual.charts[1].algebraic_assignments.is_empty());
    assert_eq!(actual.witnesses.into_iter().collect::<BTreeSet<_>>(), expected);
}

#[test]
fn m31_two_torsion_control_has_one_sign_and_an_infinite_pair() {
    // Separate exceptional-point unit control, not a benchmark replacement:
    // T=(-1,0) on y^2=x^3+2x+3, Q=(3,6), and T+T+Q=Q.
    let t = [M31 - 1, 0]; let q = [3, 6];
    let p = Problem { p: M31, a: 2, b: 3, target: q,
        selectors: [selector(&[t[0]]), selector(&[t[0]]), selector(&[q[0]])] };
    let expected = brute(&p, &[vec![t], vec![t], vec![q]]);
    assert_eq!(expected.len(), 1);
    for form in [Formulation::Expanded, Formulation::ChainLeft, Formulation::ChainRight] {
        let actual = solve(&p, form, Preprocessing::RequireSelectorQuotient, Limits::default()).unwrap();
        assert_eq!(actual.status, Status::Complete, "{form:?}: {:?}", actual.stop_reason);
        assert_eq!(actual.witnesses.into_iter().collect::<BTreeSet<_>>(), expected);
    }
}

#[test]
fn m31_complete_empty_is_separate_from_resource_stop() {
    let (mut p, points) = fixture();
    let g = points[0];
    p.target = reference_add(p.a, Some(points[1]), Some(points[1])).unwrap();
    p.selectors = [selector(&[g[0]]), selector(&[g[0]]), selector(&[g[0]])];
    assert!(brute(&p, &[vec![g], vec![g], vec![g]]).is_empty());
    for form in [Formulation::Expanded, Formulation::ChainLeft] {
        let report = solve(&p, form, Preprocessing::RequireSelectorQuotient, Limits::default()).unwrap();
        assert_eq!(report.status, Status::Complete, "{:?}", report.stop_reason);
        assert!(report.witnesses.is_empty());
    }
    let (p, _) = fixture();
    let limits = Limits { max_symbolic_monomials: 1, ..Limits::default() };
    let stopped = solve(&p, Formulation::Expanded, Preprocessing::None, limits).unwrap();
    assert_eq!(stopped.status, Status::Incomplete);
    assert!(stopped.stop_reason.unwrap().contains("admission"));
}

#[test]
fn m31_assignment_and_signed_output_caps_are_incomplete() {
    let (p, _) = fixture();
    for limits in [Limits { max_assignments: 1, ..Limits::default() }, Limits { max_witnesses: 1, ..Limits::default() }] {
        let report = solve(&p, Formulation::Expanded, Preprocessing::RequireSelectorQuotient, limits).unwrap();
        assert_eq!(report.status, Status::Incomplete);
        assert!(report.stop_reason.unwrap().contains("cap"));
        for witness in &report.witnesses { assert!(replay_witness(&p, witness)); }
    }
}

#[test]
fn m31_preprocessing_fallback_preserves_the_unchanged_system() {
    let (p, _) = fixture();
    for form in [Formulation::Expanded, Formulation::ChainLeft, Formulation::ChainRight] {
        let (raw, stats) = build(&p, form, Preprocessing::None, Limits::default()).unwrap();
        let limits = Limits { max_symbolic_products: stats.coefficient_multiplications, ..Limits::default() };
        let (fallback, _) = build(&p, form, Preprocessing::TrySelectorQuotient, limits).unwrap();
        assert_eq!(fallback.len(), raw.len());
        assert!(fallback[0].fallback_reason.is_some());
        assert!(fallback[0].preprocessing.is_none());
        assert_eq!(fallback[0].prepared, raw[0].original);
        for (prepared, source) in fallback.iter().zip(&raw) {
            assert!(replay_preprocessing(&p, &source.original, prepared));
        }
        assert!(matches!(build(&p, form, Preprocessing::RequireSelectorQuotient, limits), Err(Error::PreprocessingLimit(_))));
    }
}

#[test]
fn input_and_witness_corruption_are_rejected() {
    let (p, points) = fixture();
    let mut invalid = p.clone(); invalid.p = 31;
    assert!(invalid.validate().is_err());
    let mut invalid = p.clone(); invalid.selectors[0].pop();
    assert!(invalid.validate().is_err());
    let mut invalid = p.clone(); invalid.target[1] = (invalid.target[1] + 1) % M31;
    assert!(invalid.validate().is_err());
    let witness = SignedWitness { points: [points[0], points[0], points[0]] };
    assert!(replay_witness(&p, &witness));
    let mut damaged = witness; damaged.points[1][1] = (damaged.points[1][1] + 1) % M31;
    assert!(!replay_witness(&p, &damaged));
}
