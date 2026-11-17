use full_trace_zero_backend::binary_ecc::{BinaryPoint, F2mElement, IrreduciblePoly};
use full_trace_zero_backend::cryptanalysis::{
    binary_semaev::binary_semaev_s3, binary_semaev_chain_sat::ChainedS3Encoding, sat::SolveResult,
};
use num_bigint::BigUint;
use std::{
    collections::BTreeSet,
    time::{Duration, Instant},
};
use trace_zero_leads::fixture;

fn element(x: u32) -> F2mElement {
    F2mElement::from_biguint(&BigUint::from(x), 3)
}
fn expected(
    m: usize,
    target: Option<u32>,
    mask: u32,
    irr: &IrreduciblePoly,
) -> BTreeSet<Vec<BigUint>> {
    let mut out = BTreeSet::new();
    let b = F2mElement::one(3);
    let codes = [0, 1, 3];
    for a in codes {
        for c in codes {
            for d in if m == 2 { vec![0] } else { codes.to_vec() } {
                let valid = if m == 2 {
                    target.map_or(a == c, |t| {
                        binary_semaev_s3(&element(a), &element(c), &element(t), &b, irr).is_zero()
                    })
                } else if mask == 1 {
                    target.is_some_and(|t| a == c && d == t)
                } else {
                    (0..8).any(|z| {
                        binary_semaev_s3(&element(a), &element(c), &element(z), &b, irr).is_zero()
                            && target.map_or(z == d, |t| {
                                binary_semaev_s3(&element(z), &element(d), &element(t), &b, irr)
                                    .is_zero()
                            })
                    })
                };
                if valid {
                    out.insert(if m == 2 {
                        vec![a.into(), c.into()]
                    } else {
                        vec![a.into(), c.into(), d.into()]
                    });
                }
            }
        }
    }
    out
}
#[test]
fn specialized_builds_preserve_every_small_projected_model_and_identity_pattern() {
    let irr = IrreduciblePoly {
        degree: 3,
        low_terms: vec![0, 1],
    };
    let b = F2mElement::one(3);
    let codes: Vec<BigUint> = [0u32, 1, 3].into_iter().map(BigUint::from).collect();
    for m in [2, 3] {
        for target in [None, Some(0), Some(3)] {
            for mask in 0..1u32 << (m - 2) {
                let truth = expected(m, target, mask, &irr);
                let tx = target.map(element);
                for mode in 0..3 {
                    let mut e = match mode {
                        0 => ChainedS3Encoding::build(m, &irr, &b, tx.as_ref(), mask, 10_000)
                            .unwrap(),
                        1 => ChainedS3Encoding::build_with_target_constants(
                            m,
                            &irr,
                            &b,
                            tx.as_ref(),
                            mask,
                            10_000,
                        )
                        .unwrap(),
                        _ => ChainedS3Encoding::build_for_domain(
                            m,
                            &irr,
                            &b,
                            tx.as_ref(),
                            mask,
                            10_000,
                            &codes,
                            100_000,
                        )
                        .unwrap(),
                    };
                    if mode != 2 {
                        e.constrain_summands(&codes, 100_000).unwrap();
                    }
                    let mut found = BTreeSet::new();
                    loop {
                        match e.solver.solve() {
                            SolveResult::Unsat => break,
                            SolveResult::Unknown => {
                                panic!("small finite completeness control capped")
                            }
                            SolveResult::Sat => {
                                assert!(e.verify_model());
                                let xs = e.decode_summands();
                                let key: Vec<_> = xs.iter().map(F2mElement::to_biguint).collect();
                                assert!(found.insert(key));
                                let bits = e.solver.model();
                                e.solver.reset_search();
                                e.solver.add_clause(
                                    (0..m * 3)
                                        .map(|i| {
                                            if bits[i] {
                                                -((i + 1) as i32)
                                            } else {
                                                (i + 1) as i32
                                            }
                                        })
                                        .collect(),
                                );
                            }
                        }
                    }
                    assert_eq!(
                        found, truth,
                        "m={m},target={target:?},mask={mask},mode={mode}"
                    );
                }
            }
        }
    }
}

#[test]
fn constant_target_and_domain_controls_keep_known_wide_witnesses() {
    for n in [83, 131] {
        let c = fixture(n);
        let BinaryPoint::Affine { x, .. } = &c.generator else {
            panic!()
        };
        let codes = [x.to_biguint()];
        let mut old =
            ChainedS3Encoding::build(3, &c.irreducible, &c.b, Some(x), 0, 500_000).unwrap();
        old.constrain_summands(&codes, 1_000_000).unwrap();
        let mut target = ChainedS3Encoding::build_with_target_constants(
            3,
            &c.irreducible,
            &c.b,
            Some(x),
            0,
            500_000,
        )
        .unwrap();
        target.constrain_summands(&codes, 1_000_000).unwrap();
        let mut domain = ChainedS3Encoding::build_for_domain(
            3,
            &c.irreducible,
            &c.b,
            Some(x),
            0,
            500_000,
            &codes,
            1_000_000,
        )
        .unwrap();
        assert!(target.solver.n_vars() < old.solver.n_vars());
        assert_eq!(
            domain.n_and_gates, 0,
            "all products have a known operand on this fixed-input control"
        );
        domain.solver.conflict_budget = 2000;
        domain.solver.deadline = Some(Instant::now() + Duration::from_secs(2));
        let outcome = domain.solver.solve();
        assert_ne!(outcome, SolveResult::Unsat);
        if outcome == SolveResult::Sat {
            assert!(domain.verify_model());
        }
        println!("STATIC_SPECIALIZATION n={n} m=3 affine baseline_vars={} target_vars={} domain_vars={} domain_ands={} outcome={outcome:?}; fixed public singleton correctness control",old.solver.n_vars(),target.solver.n_vars(),domain.solver.n_vars(),domain.n_and_gates);
    }
}

#[test]
fn specialization_admission_rejects_empty_or_out_of_width_domains() {
    let irr = IrreduciblePoly {
        degree: 3,
        low_terms: vec![0, 1],
    };
    let b = F2mElement::one(3);
    assert!(
        ChainedS3Encoding::build_for_domain(3, &irr, &b, Some(&b), 0, 10_000, &[], 1000).is_err()
    );
    assert!(ChainedS3Encoding::build_for_domain(
        3,
        &irr,
        &b,
        Some(&b),
        0,
        10_000,
        &[BigUint::from(8u32)],
        1000
    )
    .is_err());
    assert!(ChainedS3Encoding::build_for_domain(
        3,
        &irr,
        &b,
        Some(&b),
        0,
        10_000,
        &[BigUint::from(1u32)],
        0
    )
    .is_err());
}
