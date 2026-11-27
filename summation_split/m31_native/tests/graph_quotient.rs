use m31_projective_preclosure::f4::{self, Limits, VerificationStatus};
use m31_projective_preclosure::graph_quotient::GraphQuotient;
use m31_projective_preclosure::{Error, Poly, Ring, M31};
use std::collections::BTreeSet;

#[test]
fn representative_keeps_the_tensor_basis_for_the_quadratic_tie() {
    let graph = GraphQuotient::new(&[[0, 0, 0, 1], [1, 0, 0, 0]]).unwrap();
    let [t, u, g, z0, z1, h] = std::array::from_fn(|index| Poly::variable(graph.nvars(), index));

    assert_eq!(graph.representative(&t.mul(&u).unwrap()).unwrap(), g);
    let gz1 = g.mul(&z1).unwrap();
    assert_eq!(graph.representative(&z0.mul(&h).unwrap()).unwrap(), gz1);
    assert_eq!(graph.representative(&gz1).unwrap(), gz1);

    let basis = graph.basis_monomials();
    assert_eq!(basis.len(), 16);
    assert_eq!(basis.iter().collect::<BTreeSet<_>>().len(), basis.len());
    for exponents in basis {
        let monomial = Poly::from_terms(graph.nvars(), [(exponents, 1)]).unwrap();
        assert_eq!(graph.representative(&monomial).unwrap(), monomial);
    }
}

#[test]
fn preclosed_generators_work_with_the_ambient_solver() {
    let squares = [[M31 + 2, 3, M31 - 5, 7], [0, 0, 1, M31 - 1]];
    for (q, width, cubic_count) in [(1, 4, 20), (2, 6, 56)] {
        let graph = GraphQuotient::new(&squares[..q]).unwrap();
        assert_eq!(graph.nvars(), width);
        let t = Poly::variable(width, 0);
        let u = Poly::variable(width, 1);
        let g = Poly::variable(width, 2);
        let z0 = Poly::variable(width, 3);
        let delta0 = Poly::constant(width, 2)
            .add(&t.scale(3))
            .unwrap()
            .sub(&u.scale(5))
            .unwrap()
            .add(&g.scale(7))
            .unwrap();
        let mut base = vec![
            t.pow(2).unwrap().sub(&t).unwrap(),
            u.pow(2).unwrap().sub(&u).unwrap(),
            t.mul(&u).unwrap().sub(&g).unwrap(),
            z0.pow(2).unwrap().sub(&delta0).unwrap(),
        ];
        if q == 2 {
            let z1 = Poly::variable(width, 4);
            let h = Poly::variable(width, 5);
            base.extend([
                z1.pow(2).unwrap().sub(&u.sub(&g).unwrap()).unwrap(),
                z0.mul(&z1).unwrap().sub(&h).unwrap(),
            ]);
        }
        assert_eq!(graph.base_generators().unwrap(), base);

        let terminal = z0.pow(5).unwrap().add(&t).unwrap();
        let preclosed = graph.preclosed_generators(&terminal).unwrap();
        let (reduced_terminal, base_with_cubics) = preclosed.split_last().unwrap();
        let cubics = &base_with_cubics[base.len()..];
        assert_eq!(cubics.len(), cubic_count);
        assert!(reduced_terminal.degree() <= 2);
        let basis: BTreeSet<_> = graph.basis_monomials().into_iter().collect();
        assert_eq!(basis.len(), 4 << q);
        let mut leading_cubics = BTreeSet::new();
        for row in cubics {
            assert_eq!(row.degree(), 3);
            let lead = row
                .terms
                .iter()
                .filter(|(monomial, _)| monomial.iter().sum::<u64>() == 3)
                .collect::<Vec<_>>();
            assert_eq!(lead.len(), 1);
            assert_eq!(*lead[0].1, 1);
            assert!(leading_cubics.insert(lead[0].0));
            assert!(row
                .terms
                .keys()
                .all(|monomial| monomial == lead[0].0 || basis.contains(monomial)));
        }

        let ring = Ring::new(0, graph.nvars()).unwrap();
        let limits = Limits::default();
        let base_report = f4::compute(&ring, &base, limits).unwrap();
        assert_eq!(
            f4::verify(&ring, base_with_cubics, &base_report, limits)
                .unwrap()
                .status,
            VerificationStatus::Verified
        );

        base.push(terminal);
        let report = f4::compute(&ring, &preclosed, limits).unwrap();
        assert_eq!(
            f4::verify(&ring, &base, &report, limits).unwrap().status,
            VerificationStatus::Verified
        );
    }
}

#[test]
fn representative_handles_large_powers_and_invalid_inputs() {
    assert!(matches!(GraphQuotient::new(&[]), Err(Error::Invalid(_))));
    assert!(matches!(
        GraphQuotient::new(&[[0; 4]; 3]),
        Err(Error::Invalid(_))
    ));
    let graph = GraphQuotient::new(&[[1, 0, 0, 0]; 2]).unwrap();
    assert!(matches!(
        graph.representative(&Poly::zero(4)),
        Err(Error::Invalid(_))
    ));

    let width = graph.nvars();
    let high_power = Poly::from_terms(width, [(vec![0, 0, 0, 0, 0, u64::MAX], 1)]).unwrap();
    assert_eq!(
        graph.representative(&high_power).unwrap(),
        Poly::variable(width, 5)
    );
    let mut overflow = high_power;
    overflow.terms.insert(vec![1, 0, 0, 0, 0, u64::MAX], 1);
    assert_eq!(
        graph.representative(&overflow),
        Err(Error::ExponentOverflow)
    );
}
