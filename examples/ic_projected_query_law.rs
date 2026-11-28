//! Native projected-query and verified-relation plumbing, not a complete IC solver.
use crypto_lib::cryptanalysis::{
    ic_boundary::{BinaryGroup, BinaryInstance, CountedGroup, GroupOps},
    koblitz_fast::FastPoint,
};

#[path = "ic_projected_query_law/core.rs"]
pub mod law;

/// Group calls are counted, including checks. Binary field work and membership
/// checks have no complete price here: this ledger is not an end-to-end metric.
pub struct BinaryBackend<'a> {
    pub instance: &'a BinaryInstance,
    pub operations: GroupOps,
}

impl law::Group for BinaryBackend<'_> {
    type Point = FastPoint;
    fn domain(&self) -> Vec<u64> {
        vec![
            u64::from(self.instance.n),
            self.instance.gf.irr,
            self.instance.a,
            self.instance.b,
            self.instance.group_order,
            self.instance.r,
            self.instance.cofactor,
            u64::from(self.instance.gf.n),
            u64::from(self.instance.fast.n),
            u64::from(self.instance.fast.field.n),
            self.instance.fast.field.irr,
            self.instance.fast.a,
            self.instance.fast.b,
        ]
    }
    fn order(&self) -> u64 {
        self.instance.group_order
    }
    fn identity(&self) -> FastPoint {
        FastPoint::INFINITY
    }
    fn contains(&self, point: FastPoint) -> bool {
        if self.instance.n != self.instance.fast.n
            || self.instance.gf.n != self.instance.n
            || self.instance.fast.field.n != self.instance.n
            || self.instance.gf.irr != self.instance.fast.field.irr
            || self.instance.a != self.instance.fast.a
            || self.instance.b != self.instance.fast.b
        {
            return false;
        }
        if point.infinity {
            return point == FastPoint::INFINITY;
        }
        self.instance.n < 64
            && point.x >> self.instance.n == 0
            && point.y >> self.instance.n == 0
            && self.instance.fast.is_on_curve(point)
    }
    fn add(&mut self, a: FastPoint, b: FastPoint) -> FastPoint {
        BinaryGroup(&self.instance.fast).add(&mut self.operations, a, b)
    }
    fn neg(&mut self, point: FastPoint) -> FastPoint {
        BinaryGroup(&self.instance.fast).neg(point)
    }
    fn mul(&mut self, point: FastPoint, scalar: u64) -> FastPoint {
        BinaryGroup(&self.instance.fast).mul(&mut self.operations, point, scalar)
    }
}

fn main() {
    eprintln!("Source support for checked projected queries and supplied relations; use cargo test --release --example ic_projected_query_law. No research run or performance result is emitted.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_lib::{
        binary_ecc::{
            curve::{point_add, point_neg, scalar_mul},
            BinaryCurve, BinaryPoint,
        },
        cryptanalysis::{
            ic_boundary::random_binary_instance, koblitz_index_calculus::points_with_x,
        },
    };
    use law::Group;
    use std::collections::{HashMap, HashSet};

    fn lift(point: &BinaryPoint) -> FastPoint {
        match point {
            BinaryPoint::Infinity => FastPoint::INFINITY,
            BinaryPoint::Affine { x, y } => FastPoint::affine(x.raw_bits()[0], y.raw_bits()[0]),
        }
    }

    fn lower(point: FastPoint, instance: &BinaryInstance) -> BinaryPoint {
        if point.infinity {
            BinaryPoint::Infinity
        } else {
            BinaryPoint::Affine {
                x: instance.gf.to_element(point.x),
                y: instance.gf.to_element(point.y),
            }
        }
    }

    #[test]
    fn original_e0_fixture_constructs_and_independently_checks_its_kernel() {
        use crypto_lib::cryptanalysis::{
            ic_boundary::ArtinSchreier, koblitz_fast::FastCurve,
            koblitz_index_calculus::find_irreducible_sparse, semaev_decomp::Gf2,
        };

        // Fixed public fixture from fixture-subgroups-enumerated-20261025.
        // Its already recorded ambient order is an input, not a new point count.
        let irreducible = find_irreducible_sparse(22).unwrap();
        let gf = Gf2::new(&irreducible);
        assert_eq!(gf.irr, 4_194_307);
        let generator = BinaryPoint::Affine {
            x: gf.to_element(97_814),
            y: gf.to_element(2_630_359),
        };
        let curve = BinaryCurve {
            m: 22,
            irreducible: irreducible.clone(),
            a: gf.to_element(0),
            b: gf.to_element(1),
            generator: generator.clone(),
            order: 991u64.into(),
            cofactor: 4232u64.into(),
        };
        let instance = BinaryInstance {
            name: "original-e0-conformance-fixture".into(),
            n: 22,
            irreducible,
            fast: FastCurve::new(&curve).unwrap(),
            artin_schreier: ArtinSchreier::new(&gf),
            gf,
            a: 0,
            b: 1,
            group_order: 4_193_912,
            r: 991,
            cofactor: 4232,
            generator: lift(&generator),
            koblitz: None,
        };
        assert!(curve.is_on_curve(&generator));
        assert_ne!(generator, BinaryPoint::Infinity);
        assert_eq!(
            scalar_mul(&curve, &generator, &991u64.into()),
            BinaryPoint::Infinity
        );
        let mut candidates: Vec<_> = (0..64)
            .flat_map(|x| points_with_x(&curve, &instance.gf.to_element(x)))
            .map(|point| lift(&point))
            .collect();
        let mut backend = BinaryBackend {
            instance: &instance,
            operations: GroupOps::default(),
        };
        assert_eq!(
            law::Law::certify_from_points(
                &mut backend,
                instance.generator,
                instance.r,
                instance.cofactor,
                &candidates,
            )
            .unwrap_err(),
            law::Error::IncompleteKernelSpan {
                found: 2116,
                expected: 4232
            }
        );
        // Low abscissae alone miss the other trace coset. Supply that coset
        // explicitly; the constructor itself does not discover missing points.
        let trace_one = (0..22)
            .map(|bit| 1u64 << bit)
            .find(|&x| instance.artin_schreier.trace(x) == 1)
            .unwrap();
        candidates.extend(
            (0..64)
                .flat_map(|x| points_with_x(&curve, &instance.gf.to_element(trace_one ^ x)))
                .map(|point| lift(&point)),
        );
        let law = law::Law::certify_from_points(
            &mut backend,
            instance.generator,
            instance.r,
            instance.cofactor,
            &candidates,
        )
        .unwrap();
        assert_eq!(law.kernel().len(), 4232);
        assert_eq!(
            law.kernel().iter().copied().collect::<HashSet<_>>().len(),
            4232
        );
        for &point in law.kernel() {
            let reference = lower(point, &instance);
            assert!(curve.is_on_curve(&reference));
            assert_eq!(
                scalar_mul(&curve, &reference, &4232u64.into()),
                BinaryPoint::Infinity
            );
        }
        // Known-scalar API conformance only; no unknown logarithm is discovered.
        for d in [0u64, 1, 123, 990] {
            let target = scalar_mul(&curve, &generator, &d.into());
            for offset in [0u64, 456, 990] {
                for index in [0, 2116, 4231] {
                    let query = law
                        .query(&mut backend, lift(&target), offset, index)
                        .unwrap();
                    let shifted = point_add(
                        &curve,
                        &target,
                        &scalar_mul(&curve, &generator, &offset.into()),
                    );
                    let expected =
                        point_add(&curve, &shifted, &lower(law.kernel()[index], &instance));
                    assert_eq!(query.point(), lift(&expected));
                    assert_eq!(
                        query.projected(),
                        lift(&scalar_mul(&curve, &expected, &4232u64.into()))
                    );
                    let recovered = law
                        .recover(
                            &mut backend,
                            &query,
                            &[law::Term {
                                point: query.point(),
                                log: 4232 * ((d + offset) % 991) % 991,
                                negative: false,
                            }],
                        )
                        .unwrap();
                    assert_eq!(recovered.scalar, d);
                }
            }
        }
    }

    #[test]
    fn binary_adapter_checks_full_query_coverage_and_relation_recovery_against_reference() {
        let mut instance = random_binary_instance(7, 202_610_260_001, 16).unwrap();
        let curve = BinaryCurve {
            m: instance.n,
            irreducible: instance.irreducible.clone(),
            a: instance.gf.to_element(instance.a),
            b: instance.gf.to_element(instance.b),
            generator: lower(instance.generator, &instance),
            order: instance.r.into(),
            cofactor: instance.cofactor.into(),
        };
        let mut reference_points = vec![BinaryPoint::Infinity];
        for x in 0..1u64 << instance.n {
            reference_points.extend(points_with_x(&curve, &instance.gf.to_element(x)));
        }
        // Independently enumerate this small domain; metadata alone is not the unit-test certificate.
        assert_eq!(reference_points.len() as u64, instance.group_order);
        let all: HashSet<_> = reference_points.iter().map(lift).collect();
        assert_eq!(all.len(), reference_points.len());
        let mut backend = BinaryBackend {
            instance: &instance,
            operations: GroupOps::default(),
        };
        assert!(!backend.contains(FastPoint::affine(1u64 << instance.n, 0)));
        let mut certified = None;
        for point in &reference_points {
            let theta = lift(&scalar_mul(&curve, point, &instance.r.into()));
            if let Ok(law) = law::Law::certify(
                &mut backend,
                instance.generator,
                instance.r,
                instance.cofactor,
                theta,
            ) {
                certified = Some(law);
                break;
            }
        }
        let law = certified
            .expect("this small binary curve must have a certified cyclic cofactor kernel");
        assert_eq!(law.kernel().len() as u64, instance.cofactor);
        let law = law::Law::certify_with_kernel(
            &mut backend,
            instance.generator,
            instance.r,
            instance.cofactor,
            law.kernel().to_vec(),
        )
        .unwrap();

        let constructed = law::Law::certify_from_points(
            &mut backend,
            instance.generator,
            instance.r,
            instance.cofactor,
            &reference_points.iter().map(lift).collect::<Vec<_>>(),
        )
        .unwrap();
        assert_eq!(
            constructed.kernel().iter().copied().collect::<HashSet<_>>(),
            law.kernel().iter().copied().collect()
        );
        let law = constructed;

        // Independent subgroup table, then projection logs from reference arithmetic only.
        let mut reference_logs = HashMap::new();
        let mut point = BinaryPoint::Infinity;
        for log in 0..instance.r {
            assert!(reference_logs.insert(lift(&point), log).is_none());
            point = point_add(&curve, &point, &curve.generator);
        }
        assert_eq!(point, BinaryPoint::Infinity);
        let mut projected_logs = HashMap::new();
        for point in &reference_points {
            let projected = scalar_mul(&curve, point, &instance.cofactor.into());
            assert_eq!(
                backend.mul(lift(point), instance.cofactor),
                lift(&projected)
            );
            projected_logs.insert(lift(point), reference_logs[&lift(&projected)]);
        }
        let target = lift(&scalar_mul(&curve, &curve.generator, &3u64.into()));
        let p1 = reference_points[1].clone();
        let p2 = reference_points[3].clone();
        let mut seen = HashSet::new();
        for offset in 0..instance.r {
            for j in 0..instance.cofactor as usize {
                let query = law.query(&mut backend, target, offset, j).unwrap();
                assert!(seen.insert(query.point()));
                let last = point_add(&curve, &lower(query.point(), &instance), &point_neg(&p1));
                let last = point_add(&curve, &last, &p2);
                let sum = point_add(&curve, &p1, &point_neg(&p2));
                assert_eq!(
                    point_add(&curve, &sum, &last),
                    lower(query.point(), &instance)
                );
                let terms: Vec<_> = [(&p1, false), (&p2, true), (&last, false)]
                    .into_iter()
                    .map(|(p, negative)| {
                        let point = lift(p);
                        law::Term {
                            point,
                            log: projected_logs[&point],
                            negative,
                        }
                    })
                    .collect();
                assert_eq!(law.recover(&mut backend, &query, &terms).unwrap().scalar, 3);
            }
        }
        assert_eq!(seen, all);
        assert!(backend.operations.scalar_mults > 0);
        assert!(backend.operations.adds > 0);
        assert!(backend.operations.doubles > 0);
        assert!(!BinaryGroup(&instance.fast).counts_field_ops());
        instance.a ^= 1;
        let inconsistent = BinaryBackend {
            instance: &instance,
            operations: GroupOps::default(),
        };
        assert!(!inconsistent.contains(FastPoint::INFINITY));
    }
}
