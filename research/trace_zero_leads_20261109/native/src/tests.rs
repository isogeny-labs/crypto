use super::{fixture, window};
use full_trace_zero_backend::binary_ecc::{
    curve::{point_add, point_double, point_neg},
    BinaryPoint, F2mElement, TraceZeroAffineMode as Mode, TraceZeroVariety,
};
use full_trace_zero_backend::cryptanalysis::{
    binary_semaev_chain_sat::ChainedS3Encoding,
    sat::SolveResult,
    trace_zero_relations::{
        decompose_full_trace_zero_with_encoding as decompose,
        TraceZeroMembershipEncoding as Encoding, TraceZeroRelationDisposition as Disposition,
        TraceZeroRelationLimits as Limits,
    },
};
use num_bigint::BigUint;
use std::time::{Duration, Instant};

fn points(n: u32) -> Vec<BinaryPoint> {
    let c = fixture(n);
    let mut out = vec![BinaryPoint::Infinity];
    for x in 0..1u32 << n {
        for y in 0..1u32 << n {
            let p = BinaryPoint::Affine {
                x: F2mElement::from_biguint(&BigUint::from(x), n),
                y: F2mElement::from_biguint(&BigUint::from(y), n),
            };
            if c.is_on_curve(&p) {
                out.push(p);
            }
        }
    }
    out
}
fn q(c: &full_trace_zero_backend::binary_ecc::BinaryCurve, x: &F2mElement) -> u8 {
    x.mul(&x.half_trace(&c.irreducible).unwrap(), &c.irreducible)
        .trace(&c.irreducible)
}
fn subtrace(c: &full_trace_zero_backend::binary_ecc::BinaryCurve, x: &F2mElement) -> u8 {
    let mut orbit = vec![x.clone()];
    for _ in 1..c.m {
        orbit.push(orbit.last().unwrap().square(&c.irreducible));
    }
    let mut total = F2mElement::zero(c.m);
    for i in 0..orbit.len() {
        for j in i + 1..orbit.len() {
            total = total.add(&orbit[i].mul(&orbit[j], &c.irreducible));
        }
    }
    assert!(total.is_zero() || total == F2mElement::one(c.m));
    if total.is_zero() {
        0
    } else {
        1
    }
}

#[test]
fn quadratic_bit_matches_independent_subtrace_and_polar_form() {
    for n in [3, 5] {
        let c = fixture(n);
        for i in 0..1u32 << n {
            let x = F2mElement::from_biguint(&BigUint::from(i), n);
            assert_eq!(q(&c, &x), x.trace(&c.irreducible) ^ subtrace(&c, &x));
            for j in 0..1u32 << n {
                let z = F2mElement::from_biguint(&BigUint::from(j), n);
                assert_eq!(
                    q(&c, &x.add(&z)) ^ q(&c, &x) ^ q(&c, &z),
                    x.mul(&z, &c.irreducible).trace(&c.irreducible)
                        ^ (x.trace(&c.irreducible) & z.trace(&c.irreducible))
                );
            }
        }
    }
    for n in [83, 131] {
        let c = fixture(n);
        let BinaryPoint::Affine { x, .. } = &c.generator else {
            panic!()
        };
        assert_eq!(q(&c, x), x.trace(&c.irreducible) ^ subtrace(&c, x));
    }
}

#[test]
fn committed_integral_polarization_certificate_replays_and_rejects_tampering() {
    use full_trace_zero_backend::cryptanalysis::identity_certificate::{
        check_with_extra, IdentityCertificate, Verdict,
    };
    let mut c: IdentityCertificate =
        serde_json::from_str(include_str!("../../subtrace-polarization.identity.json")).unwrap();
    assert!(matches!(
        check_with_extra(&c, Some((0x7265706c6179, 32))).unwrap(),
        Verdict::Accept { .. }
    ));
    c.statement.degree_bound = 1;
    assert!(check_with_extra(&c, None).is_err());
}

#[test]
fn restricted_models_match_exact_point_set_and_each_other_exhaustively() {
    for (n, d) in [(3, 1), (5, 2), (5, 4), (7, 3)] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let basis = window(&c, d);
        let mut span = Vec::new();
        for word in 0..1usize << d {
            let mut x = F2mElement::zero(n);
            for (i, b) in basis.iter().enumerate() {
                if word >> i & 1 != 0 {
                    x = x.add(b);
                }
            }
            span.push(x);
        }
        let expected: Vec<_> = points(n)
            .into_iter()
            .filter(|p| matches!(p,BinaryPoint::Affine{x,..} if span.contains(x)) && v.contains(p))
            .collect();
        for mode in [Mode::Raw, Mode::Compact, Mode::Factored] {
            let s = v
                .affine_system_over_subspace_with_mode(&basis, mode, 2_000_000)
                .unwrap();
            let mut accepted = Vec::new();
            for word in 0..1usize << s.variables {
                let bits: Vec<_> = (0..s.variables).map(|i| word >> i & 1 != 0).collect();
                if s.accepts(&bits) {
                    let p = s.decode(&bits).unwrap();
                    assert!(expected.contains(&p));
                    assert_eq!(s.assignment(&p).unwrap(), bits);
                    assert!(!accepted.contains(&p));
                    accepted.push(p);
                }
            }
            assert_eq!(accepted.len(), expected.len());
            for p in &expected {
                assert!(s.accepts(&s.assignment(p).unwrap()));
            }
            println!("RESTRICTED n={n} d={d} mode={mode:?} points={} variables={} equations={} degree={}",expected.len(),s.variables,s.equations.len(),s.maximum_degree());
        }
    }
}

#[test]
fn actual_polynomial_windows_restrict_the_second_bit_before_elimination() {
    for n in [83, 131] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        for d in [8, 16] {
            let basis = window(&c, d);
            let p = v.subspace_profile(&basis, 2_000_000).unwrap();
            assert_eq!(p.polar_rank, 0);
            assert_eq!(p.quadratic_terms, 0);
            for i in 0..d {
                for j in 0..d {
                    assert_eq!(
                        basis[i]
                            .mul(&basis[j], &c.irreducible)
                            .trace(&c.irreducible),
                        0
                    );
                }
            }
            let compact = v.affine_system_over_subspace(&basis, 2_000_000).unwrap();
            assert!(compact.maximum_degree() <= 2);
            assert_eq!(compact.variables, d as u32 + n - 1);
            println!("WINDOW n={n} d={d} polar={} q_linear={} q_quadratic={} product_dim={} compact_vars={} compact_terms={}",
                p.polar_rank,p.linear_terms,p.quadratic_terms,p.product_span_dimension,compact.variables,compact.monomials());
        }
        let full = v
            .subspace_profile(&window(&c, n as usize - 1), 2_000_000)
            .unwrap();
        assert_eq!(full.polar_rank, n as usize - 1);
    }
}

#[test]
fn invalid_restrictions_are_rejected_without_approximating_the_base() {
    let c = fixture(5);
    let v = TraceZeroVariety::new(&c).unwrap();
    let b = window(&c, 2);
    assert!(v
        .subspace_profile(&[b[0].clone(), b[0].clone()], 2_000_000)
        .is_err());
    assert!(v
        .affine_system_over_subspace(&[F2mElement::one(5)], 2_000_000)
        .is_err());
    assert!(v.affine_system_over_subspace(&b, 0).is_err());
    let empty = v.affine_system_over_subspace(&[], 2_000_000).unwrap();
    assert!((0..1usize << empty.variables).all(|w| !empty.accepts(
        &(0..empty.variables)
            .map(|i| w >> i & 1 != 0)
            .collect::<Vec<_>>()
    )));
    let restricted = v.affine_system_over_subspace(&b, 2_000_000).unwrap();
    let outside = points(5).into_iter().find(|p| {
        v.contains(p) && !matches!(p, BinaryPoint::Infinity) && restricted.assignment(p).is_err()
    });
    assert!(outside.is_some());
}

#[test]
fn certified_domain_and_explicit_points_match_group_enumeration_including_sign_restrictions() {
    let c = fixture(5);
    let v = TraceZeroVariety::new(&c).unwrap();
    let subgroup: Vec<_> = points(5).into_iter().filter(|p| v.contains(p)).collect();
    let g = subgroup
        .iter()
        .find(|p| **p != BinaryPoint::Infinity)
        .unwrap()
        .clone();
    for base in [
        vec![g.clone()],
        vec![g.clone(), point_neg(&g)],
        vec![g.clone(), point_double(&c, &g)],
        vec![g.clone(), point_neg(&g), point_double(&c, &g)],
    ] {
        for m in [2, 3] {
            for t in &subgroup {
                let mut exists = false;
                for a in &base {
                    for b in &base {
                        let s = point_add(&c, a, b);
                        if m == 2 {
                            exists |= s == *t;
                        } else {
                            for z in &base {
                                exists |= point_add(&c, &s, z) == *t;
                            }
                        }
                    }
                }
                for encoding in [Encoding::ExplicitPoints, Encoding::CertifiedDomain] {
                    let result = decompose(&v, &base, t, m, Limits::default(), encoding);
                    assert_ne!(result.disposition, Disposition::Inconclusive, "{result:?}");
                    assert_eq!(result.disposition == Disposition::Found, exists);
                    if let Some(path) = result.indices {
                        assert_eq!(
                            path.iter()
                                .fold(BinaryPoint::Infinity, |s, &i| point_add(&c, &s, &base[i])),
                            *t
                        );
                    }
                    if encoding == Encoding::CertifiedDomain {
                        assert_eq!(result.full_membership_equations, 0);
                    }
                }
            }
        }
    }
    let bad = BinaryPoint::Affine {
        x: F2mElement::zero(5),
        y: F2mElement::one(5),
    };
    assert_eq!(
        decompose(
            &v,
            &[bad],
            &g,
            2,
            Limits::default(),
            Encoding::CertifiedDomain
        )
        .disposition,
        Disposition::Inconclusive
    );
    let limits = Limits {
        max_variables: 1,
        ..Limits::default()
    };
    assert_eq!(
        decompose(&v, &[g.clone()], &g, 3, limits, Encoding::CertifiedDomain).disposition,
        Disposition::Inconclusive
    );
}

#[test]
fn wide_certified_domain_needs_no_ordinate_witness_to_replay_the_identity_control() {
    for n in [83, 131] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        assert!(v.contains(&c.generator));
        let BinaryPoint::Affine { x, .. } = &c.generator else {
            panic!()
        };
        let mut e = ChainedS3Encoding::build(3, &c.irreducible, &c.b, Some(x), 1, 500_000).unwrap();
        e.constrain_summands(&[x.to_biguint()], 1_000_000).unwrap();
        e.solver.conflict_budget = 2_000;
        e.solver.deadline = Some(Instant::now() + Duration::from_secs(1));
        let result = e.solver.solve();
        assert_eq!(result, SolveResult::Sat);
        assert!(e.verify_model());
        assert!(e.decode_summands().iter().all(|z| z == x));
        let witness = [
            c.generator.clone(),
            point_neg(&c.generator),
            c.generator.clone(),
        ];
        assert!(witness.iter().all(|p| v.contains(p)));
        assert_eq!(
            witness
                .iter()
                .fold(BinaryPoint::Infinity, |s, p| point_add(&c, &s, p)),
            c.generator
        );
        println!("CERTIFIED_X_ONLY n={n} outcome={result:?} variables={} clauses={} xors={}; constructed singleton-domain correctness control",e.solver.n_vars(),e.solver.n_clauses(),e.solver.n_xors());
        let out = decompose(
            &v,
            &[c.generator.clone()],
            &point_double(&c, &c.generator),
            2,
            Limits {
                wall_budget: Some(Duration::from_secs(2)),
                conflict_budget: 2_000,
                ..Limits::default()
            },
            Encoding::CertifiedDomain,
        );
        assert_eq!(out.disposition, Disposition::Found, "{out:?}");
        assert_eq!(out.indices, Some(vec![0, 0]));
    }
}
