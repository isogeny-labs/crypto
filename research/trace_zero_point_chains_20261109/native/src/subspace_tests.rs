use full_trace_zero_backend::binary_ecc::{
    curve::{point_add, point_neg, scalar_mul},
    BinaryPoint, F2mElement, TraceZeroVariety,
};
use full_trace_zero_backend::cryptanalysis::{
    binary_semaev_chain_sat::ChainedS3Encoding,
    trace_zero_relations::{
        TraceZeroRelationDisposition as Disposition, TraceZeroRelationLimits as Limits,
    },
    trace_zero_subspace_relations::decompose_trace_zero_subspace,
};
use num_bigint::BigUint;
use trace_zero_leads::{fixture, window};

fn base(v: &TraceZeroVariety, d: usize) -> Vec<BinaryPoint> {
    let c = v.curve();
    let basis = window(c, d);
    let mut out = Vec::new();
    for word in 1..1usize << d {
        let mut x = F2mElement::zero(c.m);
        for (i, b) in basis.iter().enumerate() {
            if word >> i & 1 != 0 {
                x = x.add(b);
            }
        }
        let inv = x.flt_inverse(&c.irreducible).unwrap();
        let rhs = x.add(&inv.square(&c.irreducible));
        if rhs.trace(&c.irreducible) != 0 {
            continue;
        }
        let p = BinaryPoint::Affine {
            x: x.clone(),
            y: x.mul(&rhs.half_trace(&c.irreducible).unwrap(), &c.irreducible),
        };
        if v.point_trace(&p).unwrap() == BinaryPoint::Infinity {
            out.push(p.clone());
            out.push(point_neg(&p));
        }
    }
    out
}
fn exists(
    c: &full_trace_zero_backend::binary_ecc::BinaryCurve,
    b: &[BinaryPoint],
    t: &BinaryPoint,
    m: usize,
) -> bool {
    for p in b {
        for q in b {
            let s = point_add(c, p, q);
            if m == 2 && s == *t {
                return true;
            }
            if m == 3 && b.iter().any(|r| point_add(c, &s, r) == *t) {
                return true;
            }
        }
    }
    false
}

#[test]
fn implicit_algebraic_base_matches_every_small_target_without_a_point_table_input() {
    for n in [5, 7] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let all = base(&v, n as usize - 1);
        let g = &all[0];
        for d in [0, 2, 3] {
            let b = base(&v, d);
            let basis = window(&c, d);
            for m in [2, 3] {
                for k in 0..c.order.to_u32_digits()[0] {
                    let t = scalar_mul(&c, g, &BigUint::from(k));
                    let expected = exists(&c, &b, &t, m);
                    // Only a linear basis and target are supplied to the oracle.
                    let result =
                        decompose_trace_zero_subspace(&v, &basis, &t, m, Limits::default());
                    assert_ne!(
                        result.disposition,
                        Disposition::Inconclusive,
                        "n={n},d={d},m={m},k={k}: {result:?}"
                    );
                    assert_eq!(result.disposition == Disposition::Found, expected);
                    if let Some(points) = result.points {
                        assert_eq!(points.len(), m);
                        assert!(points.iter().all(|p| b.contains(p)));
                        assert_eq!(
                            points
                                .iter()
                                .fold(BinaryPoint::Infinity, |s, p| point_add(&c, &s, p)),
                            t
                        );
                    }
                }
            }
        }
    }
}
#[test]
fn implicit_domain_caps_and_bad_bases_stay_inconclusive() {
    let c = fixture(5);
    let v = TraceZeroVariety::new(&c).unwrap();
    let b = base(&v, 4);
    let t = &b[0];
    assert_eq!(
        decompose_trace_zero_subspace(&v, &[F2mElement::one(5)], t, 3, Limits::default())
            .disposition,
        Disposition::Inconclusive
    );
    assert_eq!(
        decompose_trace_zero_subspace(
            &v,
            &window(&c, 2),
            t,
            3,
            Limits {
                max_variables: 1,
                ..Limits::default()
            }
        )
        .disposition,
        Disposition::Inconclusive
    );
    assert_eq!(
        decompose_trace_zero_subspace(
            &v,
            &window(&c, 2),
            t,
            3,
            Limits {
                max_models: 0,
                ..Limits::default()
            }
        )
        .disposition,
        Disposition::Inconclusive
    );
}
#[test]
fn actual_wide_implicit_domain_constructs_without_enumerating_its_points() {
    for n in [83, 131] {
        let c = fixture(n);
        let v = TraceZeroVariety::new(&c).unwrap();
        let basis = window(&c, 16);
        let e = ChainedS3Encoding::build_for_trace_zero_subspace(
            3,
            &v,
            &basis,
            &c.generator,
            0,
            500_000,
            2_000_000,
        )
        .unwrap();
        println!("ALGEBRAIC_SUBSPACE n={n} d=16 m=3 variables={} s3_ands={} clauses={} xors={}; basis-only constructor, no finite base enumerated",e.solver.n_vars(),e.n_and_gates,e.solver.n_clauses(),e.solver.n_xors());
    }
}
