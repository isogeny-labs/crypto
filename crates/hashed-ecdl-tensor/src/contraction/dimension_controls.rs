//! Controls binding numeric dimension expressions to native source routines.

use super::*;
use crate::admission::dimensions::{
    audit_arithmetic_widths, audit_node_dimensions, audit_recursion_composition,
    audit_uniform_history, path_precision_envelope, symbolic_history_certificate, NodeDimensions,
    UNIFORM_HISTORY_LAMBDA_THRESHOLD, UNIFORM_HISTORY_SCALE_FLOOR,
};
use crate::contraction::{Edge, Tensor, TensorKind, TensorVertex};
use crate::network::FnPredicate;
use crate::symbolic_history_certificate::{verify as verify_symbolic_history, Certificate};
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
fn weighted_tuple_exponent_covers_exact_small_enumerations() {
    let mut meter = Meter::new(Limits::default());
    for variables_per_weight in [2usize, 4, 6] {
        for precision in 2usize..=7 {
            let input = NodeDimensions {
                edges: 64u8.into(),
                packets: (variables_per_weight / 2).into(),
                a: 1,
                u: 1,
                coordinate: 0,
                precision: precision.into(),
            };
            let audit = audit_node_dimensions(&input).unwrap();
            assert_eq!(audit.modes_per_weight_bound, variables_per_weight.into());
            let modes: Vec<_> = (1..precision)
                .flat_map(|weight| {
                    (0..variables_per_weight).map(move |_| Mode {
                        packet: 0,
                        source: 1,
                        target: 0,
                        weight,
                        rows: Vec::new(),
                        columns: Vec::new(),
                    })
                })
                .collect();
            let mut beta = vec![0usize; modes.len()];
            let mut usage = vec![0usize; 2];
            let bounds = vec![precision - 1, 0];
            let mut weight = 0usize;
            let mut tuples = BigUint::one();
            while next_tuple(
                &mut beta,
                &modes,
                &mut usage,
                &bounds,
                &mut weight,
                precision,
                2,
                &mut meter,
            )
            .unwrap()
            {
                tuples += BigUint::one();
            }
            let exponent = audit.mode_tuple_log2_bound.to_usize().unwrap();
            assert!(tuples <= (BigUint::one() << exponent));
        }
    }
}

#[test]
fn active_call_work_envelope_fixture_is_exact() {
    let mut input = dimensions(64, 2, 2, 0, 128);
    input.packets = 8u8.into();
    let audit = audit_node_dimensions(&input).unwrap();
    assert_eq!(audit.interaction_distance, 4u8.into());
    assert_eq!(audit.modes_per_weight_bound, 16u8.into());
    assert_eq!(audit.attempted_modes_bound, 2_032u16.into());
    assert_eq!(audit.mode_tuple_log2_bound, 230u8.into());
    assert_eq!(audit.active_call_child_invocations_log2_bound, 248u8.into());
    assert_eq!(audit.packet_description_slots_bound, 20_456u16.into());
    assert_eq!(audit.packet_entry_choice_log2_bound, 44u8.into());
    assert_eq!(audit.precision_md_over_u_coefficient, BigUint::one());

    input.precision = 129u8.into();
    assert_eq!(
        audit_node_dimensions(&input)
            .unwrap()
            .precision_md_over_u_coefficient,
        2u8.into()
    );
}

#[test]
fn arithmetic_width_fixture_is_exact() {
    let mut input = dimensions(64, 2, 2, 0, 128);
    input.packets = 8u8.into();
    let audit = audit_arithmetic_widths(&input, &32u8.into()).unwrap();
    assert_eq!(audit.extension_degree, 32u8.into());
    assert_eq!(audit.coefficient_bits_bound, 128u8.into());
    assert_eq!(
        audit.ring_multiplication_temporary_bits_bound,
        257u16.into()
    );
    assert_eq!(audit.binomial_integer_bits_bound, 257u16.into());
    assert_eq!(audit.maximum_multiplicity_bound, 256u16.into());
    assert_eq!(audit.rational_multiplier_bits_bound, 3_649u16.into());
    assert_eq!(audit.rational_accumulator_bits_bound, 3_503_179u32.into());
    assert!(!audit.interpolation_ring_operations_bound.is_zero());
    assert!(!audit.recovery_ring_operations_bound.is_zero());
    assert!(!audit.recovery_rational_operations_bound.is_zero());
    assert!(audit_arithmetic_widths(&input, &BigUint::zero()).is_err());
}

#[test]
fn rational_width_envelope_covers_exact_small_grid_recovery() {
    let mut meter = Meter::new(Limits::default());
    let mut saw_width_above_coefficient_precision = false;
    for a in 1usize..=3 {
        let length = 1usize << a;
        for u in 1usize..=a {
            let edges = length.div_ceil(3).max(u);
            let parameters = Parameters {
                radix: 2,
                u,
                a,
                length,
                array: length * length,
            };
            let input = dimensions(edges, a as u32, u as u32, 0, 7);
            let audit = audit_arithmetic_widths(&input, &4u8.into()).unwrap();
            let points = deformation::grid(&parameters, &meter).unwrap();
            let alpha = BigInt::one() << (a + 1);
            let coefficient = BigRational::from_integer(BigInt::from(127u8));
            let mut accumulator = BigRational::zero();
            for &t in &points {
                let qt = multiplicity(edges, &parameters, t).unwrap();
                if qt == 0 {
                    continue;
                }
                let mut e_zero = BigRational::one();
                for &q in &points {
                    if q == 0 || q == t {
                        continue;
                    }
                    let qq = multiplicity(edges, &parameters, q).unwrap();
                    if qq == 0 {
                        continue;
                    }
                    let fraction = BigRational::new(BigInt::from(-q), BigInt::from(t - q));
                    e_zero *= rational_power(fraction, qq, &mut meter).unwrap();
                }
                for value in [e_zero.numer(), e_zero.denom()] {
                    assert!(BigUint::from(value.bits()) <= audit.rational_multiplier_bits_bound);
                }
                let ratio = BigRational::new(BigInt::from(-t), alpha.clone());
                let mut multiplier = e_zero;
                for _ in 0..qt {
                    accumulator += &multiplier * &coefficient;
                    for value in [accumulator.numer(), accumulator.denom()] {
                        let bits = BigUint::from(value.bits());
                        saw_width_above_coefficient_precision |= bits > input.precision;
                        assert!(bits <= audit.rational_accumulator_bits_bound);
                    }
                    multiplier *= &ratio;
                    for value in [multiplier.numer(), multiplier.denom()] {
                        assert!(
                            BigUint::from(value.bits()) <= audit.rational_multiplier_bits_bound
                        );
                    }
                }
            }
        }
    }
    assert!(saw_width_above_coefficient_precision);
}

#[test]
fn node_precision_covers_exact_rational_loss_on_small_source_grids() {
    let meter = Meter::new(Limits::default());
    for u in 1usize..=3 {
        for a in 2 * u..=2 * u + 2 {
            let parameters = Parameters {
                radix: 2,
                u,
                a,
                length: 1 << a,
                array: 1,
            };
            let points = deformation::grid(&parameters, &meter).unwrap();
            for edges in [u, 2 * u + 1, 17] {
                for &t in &points {
                    let qt = multiplicity(edges, &parameters, t).unwrap();
                    if qt == 0 {
                        continue;
                    }
                    let mut e_zero_valuation = 0i64;
                    for &q in &points {
                        let qq = multiplicity(edges, &parameters, q).unwrap();
                        if q == t || qq == 0 {
                            continue;
                        }
                        let difference_valuation = deformation::valuation(t - q);
                        assert!(difference_valuation <= a);
                        e_zero_valuation += i64::try_from(qq).unwrap()
                            * (i64::try_from(deformation::valuation(q)).unwrap()
                                - i64::try_from(difference_valuation).unwrap());
                    }
                    let coordinate = deformation::valuation(t);
                    let a_gap = a - coordinate;
                    let active_layers = u - coordinate;
                    let scaled_length = parameters.length >> coordinate;
                    let layer_loss: usize = (1..=active_layers)
                        .map(|r| 2 * scaled_length * (r + 1) / (1 << r) - a_gap - 2)
                        .sum();
                    let h = (3 * edges).div_ceil(u);
                    assert_eq!(
                        -e_zero_valuation,
                        i64::try_from(h * layer_loss).unwrap(),
                        "exact denominator loss at edges={edges}, u={u}, a={a}, t={t}"
                    );
                    let coordinate = i64::try_from(coordinate).unwrap();
                    let evaluation_step = coordinate - i64::try_from(a + 1).unwrap();
                    let final_multiplier_valuation =
                        e_zero_valuation + i64::try_from(qt - 1).unwrap() * evaluation_step;
                    let rational_loss =
                        usize::try_from(-final_multiplier_valuation.min(0)).unwrap();
                    let node_loss = 8 * h * scaled_length;
                    assert!(
                        rational_loss < node_loss,
                        "edges={edges}, u={u}, a={a}, t={t}"
                    );
                    assert!(node_loss <= child_precision(edges, &parameters, t, 7).unwrap() - 7);
                }
            }
        }
    }
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
fn uniform_history_threshold_covers_every_possible_active_depth() {
    let audit = audit_uniform_history(UNIFORM_HISTORY_LAMBDA_THRESHOLD).unwrap();
    assert!(audit.threshold_proof_applies);
    assert_eq!(audit.leaf_divisor, 8_192);
    assert_eq!(audit.root_u, 512);
    assert_eq!(audit.scale_floor, 32);
    assert_eq!(audit.inherited_precision_constant, 81);
    assert_eq!(audit.normalized_precision_constant, 34);
    assert_eq!(audit.depths.len(), 5);
    assert_eq!(
        audit.depths.iter().map(|case| case.u).collect::<Vec<_>>(),
        vec![512, 256, 128, 64, 32]
    );
    assert!(audit.depths.iter().all(|case| {
        case.shrink_factor < BigUint::from(audit.leaf_divisor)
            && case.u >= UNIFORM_HISTORY_SCALE_FLOOR
            && case.inherited_absorption_holds
            && case.evaluation_absorption_holds
    }));
    assert!(audit.all_absorption_checks_hold);

    // If d<=4, the threshold itself gives u>=2^(26-17-4)=32.  If d>=5,
    // activity gives lambda>2^(6d), hence u>=2^(5d-17)>=2^d>=32.
    assert_eq!(UNIFORM_HISTORY_LAMBDA_THRESHOLD >> (17 + 4), 32);
    // (5d-17)-d=4d-17 is already positive at d=5 and then increases.
    assert!(4 * 5 - 17 > 0);

    // Thus 2^d<=u.  From lambda<2^(17+d)(u+1) and u+1<=2u we get
    // h<512u.  The u=32 base cases below, followed by the fact that each
    // right side doubles while the polynomial left side grows by less than
    // two, establish both absorptions for every larger u.  For the evaluation
    // check, (A+2u+1)/2^A decreases with every A>=0, so A=0 is worst.
    let u = BigUint::from(UNIFORM_HISTORY_SCALE_FLOOR);
    assert!(
        BigUint::from(81u8) * BigUint::from(512u16) * &u * &u
            <= (BigUint::one() << (UNIFORM_HISTORY_SCALE_FLOOR + 1) as usize)
    );
    assert!(
        BigUint::from(8u8) * &u * (BigUint::from(2u8) * &u + BigUint::one())
            <= (BigUint::one() << (UNIFORM_HISTORY_SCALE_FLOOR + 1) as usize)
    );
    let next_u = &u + BigUint::one();
    assert!(&next_u * &next_u < BigUint::from(2u8) * &u * &u);
    assert!(
        &next_u * (BigUint::from(2u8) * &u + BigUint::from(3u8))
            < BigUint::from(2u8) * &u * (BigUint::from(2u8) * &u + BigUint::one())
    );
    assert!(
        BigUint::from(2u8) * &u + BigUint::from(2u8)
            <= BigUint::from(2u8) * (BigUint::from(2u8) * &u + BigUint::one())
    );
}

#[test]
fn smaller_root_log_size_exposes_the_unabsorbed_deep_history() {
    let audit = audit_uniform_history(1 << 23).unwrap();
    assert!(!audit.threshold_proof_applies);
    assert_eq!(audit.depths.last().unwrap().u, 8);
    assert!(!audit.all_absorption_checks_hold);
    assert!(audit
        .depths
        .iter()
        .any(|case| !case.inherited_absorption_holds));
}

#[test]
fn recursion_composition_has_depth_independent_constants() {
    let audit = audit_recursion_composition(UNIFORM_HISTORY_LAMBDA_THRESHOLD).unwrap();
    assert_eq!(audit.root_u, 512);
    assert_eq!(audit.local_child_exponent_constant, 96);
    assert_eq!(audit.path_sum_constant, 3);
    assert_eq!(audit.active_tree_exponent_constant, 288);
    assert_eq!(audit.leaf_exponent_constant, 1);
    assert!(audit.leaf_scale_absorbed);
    assert_eq!(audit.packet_entry_exponent_constant, 1);
    assert_eq!(audit.total_root_u_exponent_constant, 290);
    assert_eq!(audit.total_root_log_exponent_constant, 148_480);
    assert!(audit.packet_entry_absorbed);
    assert!(audit
        .depths
        .iter()
        .all(|case| case.geometric_scale_comparison_holds));
    assert!(audit.all_composition_checks_hold);
    assert!(audit_recursion_composition(1 << 23).is_err());

    // Local tuple constant: nD<=6M and p*u<=34MD give np<=204M^2/u.
    // sqrt(204)<15, and the ceiling contributes at most five more units.
    assert!(204u16 < 15u16.pow(2));
    assert_eq!(5 * 15 + 5, 80);
    // Active-node, sample, and radix terms use the remaining 16 units.
    assert_eq!(80 + 4 + 5 + 6, 95);
    assert!(audit.local_child_exponent_constant >= 95);
}

#[test]
fn symbolic_history_certificate_has_independent_exact_replay() {
    let produced = symbolic_history_certificate(UNIFORM_HISTORY_LAMBDA_THRESHOLD).unwrap();
    let serialized = serde_json::to_vec(&produced).unwrap();
    assert_eq!(serialized.len(), 2_324);
    assert_eq!(
        serialized,
        include_bytes!("../../examples/symbolic-history-certificate.json")
            .strip_suffix(b"\n")
            .unwrap()
    );
    let certificate: Certificate = serde_json::from_slice(&serialized).unwrap();
    assert_eq!(serde_json::to_vec(&certificate).unwrap(), serialized);
    let summary = verify_symbolic_history(&certificate).unwrap();
    assert_eq!(summary.lambda, UNIFORM_HISTORY_LAMBDA_THRESHOLD);
    assert_eq!(summary.root_u, 512);
    assert_eq!(summary.active_depths, 5);
    assert_eq!(summary.tuple_exponent_constant, 80);
    assert_eq!(summary.total_root_u_exponent_constant, 290);
    assert_eq!(summary.total_root_log_exponent_constant, 148_480);

    let mut bad_schema = certificate.clone();
    bad_schema.schema += 1;
    assert_eq!(
        verify_symbolic_history(&bad_schema).unwrap_err(),
        "symbolic history context"
    );

    let mut missing_depth = certificate.clone();
    missing_depth.depths.pop();
    assert_eq!(
        verify_symbolic_history(&missing_depth).unwrap_err(),
        "symbolic history depth missing"
    );

    let mut bad_precision = certificate.clone();
    bad_precision.depths[2].current_a += 1;
    assert_eq!(
        verify_symbolic_history(&bad_precision).unwrap_err(),
        "symbolic history depth mismatch"
    );

    let mut bad_composition = certificate;
    bad_composition.mode_precision_product_constant += 1;
    assert_eq!(
        verify_symbolic_history(&bad_composition).unwrap_err(),
        "symbolic history constants"
    );
    bad_composition.mode_precision_product_constant -= 1;
    bad_composition.total_root_u_exponent_constant += 1;
    assert_eq!(
        verify_symbolic_history(&bad_composition).unwrap_err(),
        "symbolic history composition mismatch"
    );
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
