use prime31_aux_matched::bases::prime31_fixture;
use prime31_aux_matched::collector::{from_replay, to_replay, PublicTarget};
use prime31_aux_matched::kernel_fibers::{build_fiber, KernelChoice};
use prime31_aux_matched::replay::Point;
use prime31_aux_matched::target_transport::{self as transfer, TargetArm, TransferWork};
use prime31_aux_matched::{CountedGroup, GroupOps, PrimePoint};

fn public_target() -> PublicTarget {
    let (e, order, g) = prime31_fixture();
    PublicTarget {
        p: e.p,
        a: e.a,
        b: e.b,
        order,
        generator: to_replay(g),
        target: Point::affine(984_837_385, 1_142_774_582),
    }
}

#[test]
fn certified_target_route_preserves_public_points_and_addition() {
    let (e, n, g) = prime31_fixture();
    let e1 = transfer::codomain();
    let mut work = TransferWork::default();
    let mapped_g = transfer::image_point(g, &mut work).unwrap();
    assert_eq!(mapped_g, PrimePoint::affine(1_767_020_261, 634_125_456));
    let mut ops = GroupOps::default();
    assert!(e1.mul(&mut ops, mapped_g, n).infinity);
    for (a, b) in [(1, 7), (31, 313), (1001, 12345)] {
        let p = e.mul(&mut ops, g, a);
        let q = e.mul(&mut ops, g, b);
        let sum = e.add(&mut ops, p, q);
        let left = transfer::image_point(sum, &mut work).unwrap();
        let pp = transfer::image_point(p, &mut work).unwrap();
        let qq = transfer::image_point(q, &mut work).unwrap();
        let right = e1.add(&mut ops, pp, qq);
        assert_eq!(left, right);
    }
    assert!(
        transfer::image_point(e.identity(), &mut work)
            .unwrap()
            .infinity
    );
    assert_eq!(transfer::image_x(transfer::ALPHA, &mut work).unwrap(), None);
    assert!(transfer::image_point(PrimePoint::affine(1, 1), &mut work).is_err());
    assert!(transfer::image_x(transfer::P, &mut work).is_err());
}

#[test]
fn four_complete_fibers_keep_their_rational_columns_under_transport() {
    for (kernel, source_count, native_count) in [
        (KernelChoice::AplusCyclic, 518, 506),
        (KernelChoice::AminusH0, 508, 508),
        (KernelChoice::AminusH1, 512, 519),
        (KernelChoice::AminusH2, 515, 533),
    ] {
        let fiber = build_fiber(kernel, 1, &mut GroupOps::default()).unwrap();
        let original = transfer::prepare(&fiber, &public_target(), TargetArm::Original).unwrap();
        let native =
            transfer::prepare(&fiber, &public_target(), TargetArm::IsogenousNative).unwrap();
        let transported =
            transfer::prepare(&fiber, &public_target(), TargetArm::IsogenousTransported).unwrap();
        assert_eq!(original.factor_base().unwrap().columns, source_count);
        assert_eq!(native.factor_base().unwrap().columns, native_count);
        assert_eq!(transported.factor_base().unwrap().columns, source_count);
        assert_eq!(transported.v.len(), 1024);
        assert!(transported.dropped_kernel_poles.is_empty());
        assert_eq!(transported.duplicate_image_values, 0);
        assert!(transported.tower.is_none());
        let encoded = serde_json::to_string(&transported).unwrap();
        let decoded: transfer::PreparedBase = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.factor_base().unwrap().columns, source_count);
        // A signed relation's sum must survive canonical sign/permutation changes.
        let mut row = vec![0; source_count];
        row[0] = 2;
        row[source_count / 2] = -1;
        row[source_count - 1] = 1;
        let mapped_row = transfer::transport_row(&row, &transported.column_transport).unwrap();
        let sum = |base: &transfer::PreparedBase, coefficients: &[i64]| {
            let e = base.curve.production();
            let mut ops = GroupOps::default();
            let mut sum = e.identity();
            for (&q, &coefficient) in base.canonical_points.iter().zip(coefficients) {
                if coefficient == 0 {
                    continue;
                }
                let q = if coefficient > 0 {
                    q.production()
                } else {
                    e.neg(q.production())
                };
                let term = e.mul(&mut ops, q, coefficient.unsigned_abs());
                sum = e.add(&mut ops, sum, term);
            }
            sum
        };
        assert_eq!(
            transfer::image_point(sum(&original, &row), &mut TransferWork::default()).unwrap(),
            sum(&transported, &mapped_row)
        );
        let actual_q = from_replay(transported.public_target.target);
        assert_eq!(
            actual_q,
            transfer::image_point(
                from_replay(public_target().target),
                &mut TransferWork::default()
            )
            .unwrap()
        );
    }
}

#[test]
fn transported_column_map_rejects_duplicates_and_bad_signs() {
    let columns = vec![
        transfer::ColumnTransport {
            source_column: 0,
            destination_column: 1,
            sign: -1,
        },
        transfer::ColumnTransport {
            source_column: 1,
            destination_column: 0,
            sign: 1,
        },
    ];
    assert_eq!(
        transfer::transport_row(&[3, -2], &columns).unwrap(),
        vec![-2, -3]
    );
    let mut bad = columns.clone();
    bad[1].destination_column = 1;
    assert!(transfer::transport_row(&[3, -2], &bad).is_err());
    bad = columns;
    bad[0].sign = 0;
    assert!(transfer::transport_row(&[3, -2], &bad).is_err());
}
