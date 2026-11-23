//! Controls binding numeric dimension expressions to native source routines.

use super::*;
use crate::admission::dimensions::{
    audit_node_dimensions, path_precision_envelope, NodeDimensions,
};
use crate::contraction::{Edge, Tensor, TensorKind, TensorVertex};
use crate::network::FnPredicate;
use crate::Limits;
use std::sync::Arc;

fn dimensions(m: usize, a: u32, u: u32, i: u32, p: usize) -> NodeDimensions {
    NodeDimensions {
        edges: m.into(),
        packets: 1u8.into(),
        a,
        u,
        coordinate: i,
        precision: p.into(),
    }
}

#[test]
fn closed_jet_counts_equal_native_grid_and_multiplicities() {
    let meter = Meter::new(Limits::default());
    for a in 1..=6 {
        for u in 1..=a {
            for m in [(1usize << a).div_ceil(3).max(u as usize), 127] {
                let parameters = Parameters {
                    radix: 2,
                    u: u as usize,
                    a: a as usize,
                    length: 1 << a,
                    array: 1 << (2 * a),
                };
                let audit = audit_node_dimensions(&dimensions(m, a, u, 0, 7)).unwrap();
                let mut count = 0usize;
                let mut coefficients = 0usize;
                for t in deformation::grid(&parameters, &meter).unwrap() {
                    let qt = multiplicity(m, &parameters, t).unwrap();
                    count += usize::from(qt != 0);
                    coefficients += qt;
                }
                assert_eq!(audit.active_nodes, count.into());
                assert_eq!(audit.retained_jet_coefficients, coefficients.into());
                assert!(audit.retained_jet_coefficients <= BigUint::from(8 * m * (1 << a)));
            }
        }
    }
}

#[test]
fn mode_envelopes_cover_native_zero_filtering_and_each_weight() {
    let mut meter = Meter::new(Limits {
        steps: 100_000_000,
        ..Limits::default()
    });
    let tensor = Arc::new(Tensor {
        creation_precision: 1,
        kind: TensorKind::Boolean(Arc::new(FnPredicate::new(1, |_, _| Ok(true)))),
    });
    let mut saw_discarded_modes = false;
    let mut saw_retained_modes = false;
    for a in 1..=3 {
        let length = 1usize << a;
        // Dense classes avoid relying on unqualified sparse packing bounds.
        let m = length.pow(3);
        let vertices: Vec<_> = (0..m)
            .map(|edge| TensorVertex {
                tensor: tensor.clone(),
                edges: vec![edge],
            })
            .collect();
        let graph = Bipartite {
            left: vertices.clone(),
            right: vertices,
            edges: (0..m)
                .map(|v| Edge {
                    left: (v, 0),
                    right: (v, 0),
                })
                .collect(),
        };
        graph.validate().unwrap();
        for u in 1..=a {
            let parameters = Parameters {
                radix: 2,
                u: u as usize,
                a: a as usize,
                length,
                array: length * length,
            };
            let packing = packing::pack_labels(
                &graph,
                &parameters,
                (0..m).map(|v| v / length % length).collect(),
                (0..m).map(|v| v / (length * length)).collect(),
                false,
                &mut meter,
            )
            .unwrap();
            for i in 0..u {
                for p in [1, 3, 7] {
                    let mut input = dimensions(m, a, u, i, p);
                    input.packets = packing.packets.len().into();
                    let audit = audit_node_dimensions(&input).unwrap();
                    let ring = Ring::new(4, p, &mut meter).unwrap();
                    for z in [ring.zero(), ring.one()] {
                        let kernel =
                            Cauchy::new(&parameters, 1i64 << i, &z, &ring, &mut meter).unwrap();
                        let modes = deformation::modes(
                            &graph,
                            &packing,
                            i as usize,
                            &parameters,
                            &kernel,
                            &mut meter,
                        )
                        .unwrap();
                        assert!(BigUint::from(modes.len()) <= audit.attempted_modes_bound);
                        for weight in 1..p {
                            let n = modes.iter().filter(|mode| mode.weight == weight).count();
                            assert!(BigUint::from(n) <= audit.modes_per_weight_bound);
                        }
                        saw_discarded_modes |=
                            BigUint::from(modes.len()) < audit.attempted_modes_bound;
                        saw_retained_modes |= !modes.is_empty();
                    }
                }
            }
        }
    }
    assert!(saw_discarded_modes && saw_retained_modes);
}

#[test]
fn root_precision_envelope_covers_all_active_scales_and_native_increments() {
    // Actual source scales at astronomical sizes, without allocating a graph.
    for root_u in [1u64, 3, 8] {
        let lambda = 131072 * root_u;
        let s = BigUint::one() << lambda as usize;
        let initial = &s + BigUint::one();
        let audit = path_precision_envelope(&s, &initial).unwrap();
        let mut m = s.clone();
        let mut p = initial;
        let mut a = 0u64;
        for depth in 0..64 {
            let u = root_u >> depth;
            if u == 0 {
                break;
            }
            a += 2 * u;
            let length = BigUint::one() << a as usize;
            let h = (BigUint::from(3u8) * &m + BigUint::from(u - 1)) / BigUint::from(u);
            // i=0 is the largest precision increment at this call.
            p += BigUint::from(8u8) * &h * &length + BigUint::from(8u8) * &m * BigUint::from(a + 1);
            assert!(a <= audit.packet_exponent_bound);
            assert!(length <= audit.packet_length_bound);
            assert!(p <= audit.precision_bound);
            m >>= 3usize;
        }
    }
    // Bind the increment expression to the real usize routine at small shapes.
    for m in 1usize..=64 {
        for u in 1..=m.min(4) {
            let parameters = Parameters {
                radix: 2,
                u,
                a: 2 * u,
                length: 1 << (2 * u),
                array: 1,
            };
            for i in 0..u {
                let actual = child_precision(m, &parameters, 1i64 << i, 7).unwrap();
                let expected = 7
                    + 8 * (3 * m).div_ceil(u) * (parameters.length >> i)
                    + 8 * m * (parameters.a + 1);
                assert_eq!(actual, expected);
            }
        }
    }
}

#[test]
fn dimension_checks_reject_missing_premises_and_cover_unit_precision() {
    let valid = dimensions(3, 2, 2, 1, 1);
    let audit = audit_node_dimensions(&valid).unwrap();
    assert!(audit.attempted_modes_bound.is_zero());
    for mutate in 0..7 {
        let mut invalid = valid.clone();
        match mutate {
            0 => invalid.edges = 0u8.into(),
            1 => invalid.packets = 0u8.into(),
            2 => invalid.precision = 0u8.into(),
            3 => invalid.u = 0,
            4 => invalid.u = 3,
            5 => invalid.coordinate = 2,
            _ => invalid.packets = 3u8.into(),
        }
        assert!(audit_node_dimensions(&invalid).is_err());
    }
    assert!(path_precision_envelope(&1u8.into(), &1u8.into()).is_err());
    assert!(path_precision_envelope(&2u8.into(), &0u8.into()).is_err());
}

#[test]
fn retained_jet_lengths_do_not_account_for_interpolation_vector_capacity() {
    let mut meter = Meter::new(Limits::default());
    let parameters = Parameters {
        radix: 2,
        u: 2,
        a: 2,
        length: 4,
        array: 16,
    };
    let audit = audit_node_dimensions(&dimensions(2, 2, 2, 0, 7)).unwrap();
    let ring = Ring::new(4, 7, &mut meter).unwrap();
    let samples = audit.samples_per_node.to_usize().unwrap();
    let nodes: Vec<_> = (0..samples)
        .map(|j| ring.point(&j.into()).unwrap())
        .collect();
    let values = vec![ring.one(); samples];
    let mut retained = Vec::new();
    for t in deformation::grid(&parameters, &meter).unwrap() {
        let qt = multiplicity(2, &parameters, t).unwrap();
        if qt == 0 {
            continue;
        }
        // The same existing interpolation/resize/truncate operations as active.
        let mut coefficients = interpolate(&nodes, &values, &ring, &mut meter).unwrap();
        coefficients.resize(qt, ring.zero());
        coefficients.truncate(qt);
        assert!(coefficients.capacity() >= samples);
        assert!(coefficients.capacity() > coefficients.len());
        retained.push(coefficients);
    }
    let live: usize = retained.iter().map(Vec::len).sum();
    let slots: usize = retained.iter().map(Vec::capacity).sum();
    assert_eq!(BigUint::from(live), audit.retained_jet_coefficients);
    assert!(BigUint::from(slots) >= audit.retained_jet_vector_slots_lower_bound);
    assert!(slots > live);
    // Concrete source-shape fixture: 30 live Elements, at least 90 outer slots.
    assert_eq!((live, samples), (30, 15));
    assert!(slots >= 90);
}
