//! Production-side certificate controls for the standalone verifier.

use super::*;
use crate::admission::dimensions::{audit_node_dimensions, NodeDimensions};
use crate::child_provenance_certificate::{
    family_description_envelope as child_family_description_envelope,
    verify as verify_child_provenance, verify_family as verify_child_family,
    Certificate as ChildProvenanceCertificate, ChildEdge as ProvenanceChildEdge, OldTensorSource,
    ParentSide, SCHEMA as CHILD_PROVENANCE_SCHEMA,
};
use crate::contraction::{Edge, Tensor, TensorKind, TensorVertex};
use crate::factorization_certificate::{
    description_envelope, group_assignment_projection, verify as standalone_verify,
    verify_cauchy_mode_coverage, verify_cauchy_source, verify_ring_multiplication_witness,
    verify_source_dimensions, verify_with_ring as check, CauchyFactorWitness, CauchySourceWitness,
    Certificate, CertifiedElement, CertifiedMode, EntryTable, RingMultiplicationWitness,
    SourceDimensionWitness, SCHEMA,
};
use crate::network::FnPredicate;
use crate::node_refinement_certificate::{
    history_shrink_bundle_projection, history_shrink_projection, verify as verify_node_refinement,
    verify_history_shrink_projection, verify_strict as verify_strict_node_refinement,
    Certificate as NodeRefinementCertificate, SCHEMA as NODE_REFINEMENT_SCHEMA,
};
use crate::packing_provenance_certificate::{
    description_envelope as packing_description_envelope,
    label_projection as packing_label_projection, verify as verify_packing_provenance,
    Certificate as PackingProvenanceCertificate, EdgeEndpoints,
    SCHEMA as PACKING_PROVENANCE_SCHEMA,
};
use crate::Limits;
use std::collections::BTreeSet;
use std::sync::Arc;

fn cauchy_source_witness(kernel: &deformation::Cauchy, coordinate: usize) -> CauchySourceWitness {
    let (sample, base, numerator_unit, factors) = kernel.source_witness_parts();
    CauchySourceWitness {
        polynomial: kernel.ring.polynomial().clone(),
        lower_coefficients: (0..kernel.ring.degree())
            .map(|index| kernel.ring.polynomial().bit(index as u64))
            .collect(),
        a: u32::try_from(kernel.a).unwrap(),
        coordinate: u32::try_from(coordinate).unwrap(),
        center: kernel.t,
        sample: sample.coordinates().to_vec(),
        base: base.coordinates().to_vec(),
        numerator_unit: numerator_unit.coordinates().to_vec(),
        factors: factors
            .iter()
            .map(|(&difference, (valuation, inverse))| CauchyFactorWitness {
                difference,
                valuation: *valuation,
                inverse: inverse.coordinates().to_vec(),
            })
            .collect(),
    }
}

fn assert_source_count_bridge(
    certificate: &Certificate,
    parameters: &Parameters,
    coordinate: usize,
    require_normalized_precision: bool,
) -> bool {
    let dimensions = NodeDimensions {
        edges: certificate.edge_count.into(),
        packets: certificate.packets.len().into(),
        a: u32::try_from(parameters.a).unwrap(),
        u: u32::try_from(parameters.u).unwrap(),
        coordinate: u32::try_from(coordinate).unwrap(),
        precision: certificate.precision.into(),
    };
    let audit = audit_node_dimensions(&dimensions);
    let source = verify_source_dimensions(
        certificate,
        &SourceDimensionWitness {
            a: u32::try_from(parameters.a).unwrap(),
            u: u32::try_from(parameters.u).unwrap(),
            coordinate: u32::try_from(coordinate).unwrap(),
            require_normalized_precision,
        },
    );
    let packing_capacity = BigUint::from(certificate.packets.len())
        * BigUint::from(parameters.length)
        <= BigUint::from(3u8) * BigUint::from(certificate.edge_count);
    assert_eq!(audit.is_ok(), packing_capacity);
    let precision_admitted = audit
        .as_ref()
        .is_ok_and(|audit| audit.precision_md_over_u_coefficient <= BigUint::from(34u8));
    assert_eq!(
        source.is_ok(),
        packing_capacity && (!require_normalized_precision || precision_admitted)
    );
    let Ok(audit) = audit else {
        return false;
    };
    let source = source.unwrap();
    let envelope = description_envelope(certificate).unwrap();
    assert_eq!(audit.packet_length, parameters.length.into());
    assert_eq!(source.edges, dimensions.edges);
    assert_eq!(source.packets, dimensions.packets);
    assert_eq!(source.packet_length, audit.packet_length);
    assert_eq!(source.tile_period, audit.tiles);
    assert_eq!(source.interaction_distance, audit.interaction_distance);
    assert_eq!(source.precision, dimensions.precision);
    assert_eq!(source.normalized_precision_admitted, precision_admitted);
    assert_eq!(audit.packet_length, envelope.packet_length.into());
    assert_eq!(audit.tiles, envelope.tiles.into());
    assert_eq!(
        audit.modes_per_weight_bound,
        envelope.modes_per_weight_bound.into()
    );
    assert_eq!(
        audit.attempted_modes_bound,
        envelope.attempted_modes_bound.into()
    );
    assert_eq!(
        audit.mode_tuple_log2_bound,
        envelope.tuple_log2_bound.into()
    );
    assert_eq!(
        source.modes_per_weight_bound,
        envelope.modes_per_weight_bound
    );
    assert_eq!(source.attempted_modes_bound, envelope.attempted_modes_bound);
    assert_eq!(source.tuple_log2_bound, envelope.tuple_log2_bound);
    true
}

fn predicate(arity: usize, table: u8) -> Arc<Tensor> {
    Arc::new(Tensor {
        creation_precision: 3,
        kind: TensorKind::Boolean(Arc::new(FnPredicate::new(arity, move |bits, _| {
            let index = bits
                .iter()
                .enumerate()
                .fold(0, |value, (port, bit)| value | (usize::from(*bit) << port));
            Ok(table & (1 << index) != 0)
        }))),
    })
}

fn true_predicate(arity: usize) -> Arc<Tensor> {
    Arc::new(Tensor {
        creation_precision: 3,
        kind: TensorKind::Boolean(Arc::new(FnPredicate::new(arity, |_, _| Ok(true)))),
    })
}

fn fixture() -> Bipartite {
    Bipartite {
        left: vec![TensorVertex {
            tensor: predicate(2, 6),
            edges: vec![0, 1],
        }],
        right: vec![TensorVertex {
            tensor: predicate(2, 9),
            edges: vec![0, 1],
        }],
        edges: vec![
            Edge {
                left: (0, 0),
                right: (0, 0),
            },
            Edge {
                left: (0, 1),
                right: (0, 1),
            },
        ],
    }
}

fn partition_graph(left_groups: &[usize], right_groups: &[usize]) -> Bipartite {
    fn side(groups: &[usize]) -> (Vec<TensorVertex>, Vec<(usize, usize)>) {
        let vertices = groups.iter().copied().max().map_or(0, |group| group + 1);
        let mut ports = vec![Vec::new(); vertices];
        let mut endpoints = vec![(0usize, 0usize); groups.len()];
        for (edge, &vertex) in groups.iter().enumerate() {
            endpoints[edge] = (vertex, ports[vertex].len());
            ports[vertex].push(edge);
        }
        let tensors = ports
            .into_iter()
            .map(|edges| TensorVertex {
                tensor: true_predicate(edges.len()),
                edges,
            })
            .collect();
        (tensors, endpoints)
    }

    assert_eq!(left_groups.len(), right_groups.len());
    let (left, left_endpoints) = side(left_groups);
    let (right, right_endpoints) = side(right_groups);
    Bipartite {
        left,
        right,
        edges: (0..left_groups.len())
            .map(|edge| Edge {
                left: left_endpoints[edge],
                right: right_endpoints[edge],
            })
            .collect(),
    }
}

fn refinement_labels(vertices: usize, schedule: usize, side: usize) -> Vec<usize> {
    (0..vertices)
        .map(|vertex| match schedule {
            0 => 0,
            1 => (vertex + side) % 4,
            2 => (3 * vertex + side + 1) % 4,
            3 => 3 - (vertex + side) % 4,
            _ => unreachable!(),
        })
        .collect()
}

fn set_partitions(edges: usize) -> Vec<Vec<usize>> {
    fn visit(position: usize, maximum: usize, current: &mut [usize], out: &mut Vec<Vec<usize>>) {
        if position == current.len() {
            out.push(current.to_vec());
            return;
        }
        for group in 0..=maximum + 1 {
            current[position] = group;
            visit(position + 1, maximum.max(group), current, out);
        }
    }

    assert!(edges > 0);
    let mut out = Vec::new();
    let mut current = vec![0usize; edges];
    visit(1, 0, &mut current, &mut out);
    out
}

fn edge_permutations(edges: usize) -> Vec<Vec<usize>> {
    fn visit(position: usize, current: &mut [usize], out: &mut Vec<Vec<usize>>) {
        if position == current.len() {
            out.push(current.to_vec());
            return;
        }
        for next in position..current.len() {
            current.swap(position, next);
            visit(position + 1, current, out);
            current.swap(position, next);
        }
    }

    let mut out = Vec::new();
    let mut current: Vec<_> = (0..edges).collect();
    visit(0, &mut current, &mut out);
    out
}

fn normalize_partition(values: impl Iterator<Item = usize>, edges: usize) -> Vec<usize> {
    let mut names = vec![usize::MAX; edges];
    let mut next = 0usize;
    values
        .map(|value| {
            if names[value] == usize::MAX {
                names[value] = next;
                next += 1;
            }
            names[value]
        })
        .collect()
}

fn partition_pair_orbits(edges: usize) -> BTreeSet<(Vec<usize>, Vec<usize>)> {
    let partitions = set_partitions(edges);
    let permutations = edge_permutations(edges);
    let mut out = BTreeSet::new();
    for left in &partitions {
        for right in &partitions {
            let canonical = permutations
                .iter()
                .map(|permutation| {
                    (
                        normalize_partition(permutation.iter().map(|&edge| left[edge]), edges),
                        normalize_partition(permutation.iter().map(|&edge| right[edge]), edges),
                    )
                })
                .min()
                .unwrap();
            out.insert(canonical);
        }
    }
    out
}

fn producer_bounds(certificate: &Certificate) -> Vec<usize> {
    let mut out = vec![0usize; certificate.packets.len() * certificate.radix];
    for (packet, slots) in certificate.packets.iter().enumerate() {
        for &edge in slots.iter().flatten() {
            out[packet * certificate.radix + certificate.left_groups[edge]] += 1;
        }
    }
    out
}

fn packing_provenance(
    graph: &Bipartite,
    packing: &Packing,
    parameters: &Parameters,
    coordinate: usize,
    source_degree_cap: Option<usize>,
) -> PackingProvenanceCertificate {
    PackingProvenanceCertificate {
        schema: PACKING_PROVENANCE_SCHEMA,
        radix: parameters.radix,
        packet_exponent: u32::try_from(parameters.a).unwrap(),
        coordinate: u32::try_from(coordinate).unwrap(),
        conditional_labels: source_degree_cap.is_some(),
        source_degree_cap,
        edges: graph
            .edges
            .iter()
            .map(|edge| EdgeEndpoints {
                left: edge.left.0,
                left_port: edge.left.1,
                right: edge.right.0,
                right_port: edge.right.1,
            })
            .collect(),
        row_labels: packing.row_labels.clone(),
        column_labels: packing.column_labels.clone(),
        row_creation_precisions: graph
            .left
            .iter()
            .map(|vertex| vertex.tensor.creation_precision)
            .collect(),
        column_creation_precisions: graph
            .right
            .iter()
            .map(|vertex| vertex.tensor.creation_precision)
            .collect(),
        packet_pairs: packing
            .packets
            .iter()
            .map(|packet| packet.pairs.clone())
            .collect(),
    }
}

fn child_provenance(
    graph: &Bipartite,
    packing: &Packing,
    coordinate: usize,
    parameters: &Parameters,
    child: &Bipartite,
    beta: &[usize],
    group: usize,
) -> ChildProvenanceCertificate {
    let mut old_tensor_sources = Vec::new();
    let mut expected_old_tensors = Vec::new();
    for (side, vertices, labels) in [
        (ParentSide::Row, &graph.left, &packing.row_labels),
        (ParentSide::Column, &graph.right, &packing.column_labels),
    ] {
        for (vertex, (tensor_vertex, &label)) in vertices.iter().zip(labels).enumerate() {
            if packing::digit(label, coordinate, parameters.radix) == group {
                old_tensor_sources.push(OldTensorSource { side, vertex });
                expected_old_tensors.push(&tensor_vertex.tensor);
            }
        }
    }
    assert_eq!(expected_old_tensors.len(), child.left.len());
    for (expected, actual) in expected_old_tensors.iter().zip(&child.left) {
        assert!(Arc::ptr_eq(expected, &actual.tensor));
    }
    assert!(child
        .right
        .iter()
        .all(|vertex| matches!(vertex.tensor.kind, TensorKind::Packet(_))));
    ChildProvenanceCertificate {
        schema: CHILD_PROVENANCE_SCHEMA,
        group,
        beta: beta.to_vec(),
        old_vertex_degrees: child.left.iter().map(|vertex| vertex.edges.len()).collect(),
        old_tensor_sources,
        old_creation_precisions: child
            .left
            .iter()
            .map(|vertex| vertex.tensor.creation_precision)
            .collect(),
        packet_vertex_degrees: child
            .right
            .iter()
            .map(|vertex| vertex.edges.len())
            .collect(),
        packet_creation_precisions: child
            .right
            .iter()
            .map(|vertex| vertex.tensor.creation_precision)
            .collect(),
        edges: child
            .edges
            .iter()
            .map(|edge| ProvenanceChildEdge {
                left_vertex: edge.left.0,
                left_port: edge.left.1,
                right_vertex: edge.right.0,
                right_port: edge.right.1,
            })
            .collect(),
    }
}

fn produce(
    graph: &Bipartite,
    packing: &Packing,
    parameters: &Parameters,
    coordinate: usize,
    kernel: &deformation::Cauchy,
    modes: &[deformation::Mode],
    source_degree_cap: Option<usize>,
    meter: &mut Meter,
) -> Result<Certificate> {
    let left_groups: Vec<_> = graph
        .edges
        .iter()
        .map(|edge| {
            packing::digit(
                packing.row_labels[edge.left.0],
                coordinate,
                parameters.radix,
            )
        })
        .collect();
    let right_groups: Vec<_> = graph
        .edges
        .iter()
        .map(|edge| {
            packing::digit(
                packing.column_labels[edge.right.0],
                coordinate,
                parameters.radix,
            )
        })
        .collect();
    let packets: Vec<_> = packing
        .packets
        .iter()
        .map(|packet| packet.slots.clone())
        .collect();
    let mut weights = Vec::new();
    for slots in &packets {
        let mut rows = Vec::new();
        for row in 0..slots.len() {
            let mut columns = Vec::new();
            for column in 0..slots.len() {
                columns.push(CertifiedElement::encode(kernel.weight(row, column, meter)?));
            }
            rows.push(columns);
        }
        weights.push(rows);
    }
    let certified_modes: Vec<_> = modes
        .iter()
        .map(|mode| CertifiedMode {
            packet: mode.packet,
            source: mode.source,
            target: mode.target,
            weight: mode.weight,
            rows: mode
                .rows
                .iter()
                .map(|(slot, value)| (*slot, CertifiedElement::encode(value.clone())))
                .collect(),
            columns: mode.columns.clone(),
        })
        .collect();
    let mut certificate = Certificate {
        schema: SCHEMA,
        degree: kernel.ring.degree(),
        radix: parameters.radix,
        precision: kernel.ring.precision(),
        tiles: 1usize
            .checked_shl(
                u32::try_from(
                    coordinate
                        .checked_add(1)
                        .ok_or_else(|| overflow("certificate tile period"))?,
                )
                .map_err(|_| overflow("certificate tile period"))?,
            )
            .ok_or_else(|| overflow("certificate tile period"))?,
        edge_count: graph.edges.len(),
        packets,
        left_groups,
        right_groups,
        weights,
        modes: certified_modes,
        tuples: Vec::new(),
        entries: Vec::new(),
    };
    let group_bounds = producer_bounds(&certificate);
    let mut beta = vec![0usize; modes.len()];
    let mut usage = vec![0usize; group_bounds.len()];
    let mut weight = 0usize;
    let mut child_certificates = Vec::new();
    loop {
        certificate.tuples.push(beta.clone());
        for group in 0..parameters.radix {
            let child = deformation::child(
                graph, packing, coordinate, parameters, kernel, modes, &beta, group, meter,
            )?;
            child_certificates.push(child_provenance(
                graph, packing, coordinate, parameters, &child, &beta, group,
            ));
            for (packet, vertex) in child.right.iter().enumerate() {
                let assignments = 1usize
                    .checked_shl(
                        u32::try_from(vertex.edges.len())
                            .map_err(|_| overflow("certificate packet table"))?,
                    )
                    .ok_or_else(|| overflow("certificate packet table"))?;
                let mut values = Vec::with_capacity(assignments);
                for mask in 0..assignments {
                    let bits: Vec<_> = vertex
                        .edges
                        .iter()
                        .enumerate()
                        .map(|(port, _)| mask & (1usize << port) != 0)
                        .collect();
                    values.push(CertifiedElement::encode(vertex.tensor.entry(
                        &bits,
                        &kernel.ring,
                        meter,
                    )?));
                }
                certificate.entries.push(EntryTable {
                    beta: beta.clone(),
                    group,
                    packet,
                    values,
                });
            }
        }
        if !next_tuple(
            &mut beta,
            modes,
            &mut usage,
            &group_bounds,
            &mut weight,
            kernel.ring.precision(),
            parameters.radix,
            meter,
        )? {
            let provenance =
                packing_provenance(graph, packing, parameters, coordinate, source_degree_cap);
            let serialized_provenance = serde_json::to_vec(&provenance)
                .map_err(|_| invalid("packing provenance serialization"))?;
            let parsed_provenance: PackingProvenanceCertificate =
                serde_json::from_slice(&serialized_provenance)
                    .map_err(|_| invalid("packing provenance parsing"))?;
            if serde_json::to_vec(&parsed_provenance)
                .map_err(|_| invalid("packing provenance serialization"))?
                != serialized_provenance
            {
                return Err(integrity("packing provenance round trip"));
            }
            if packing_label_projection(&parsed_provenance)
                .map_err(|_| integrity("packing label projection"))?
                != packing_label_projection(&provenance)
                    .map_err(|_| integrity("packing label projection"))?
            {
                return Err(integrity("packing label projection"));
            }
            verify_packing_provenance(&certificate, &parsed_provenance, meter)
                .map_err(|_| integrity("packing provenance replay"))?;
            if serialized_provenance.len()
                > packing_description_envelope(&parsed_provenance)
                    .map_err(|_| integrity("packing provenance description"))?
                    .json_bytes_bound
            {
                return Err(integrity("packing provenance description"));
            }
            let expected_children = certificate
                .tuples
                .len()
                .checked_mul(parameters.radix)
                .ok_or_else(|| overflow("child provenance coverage"))?;
            if child_certificates.len() != expected_children {
                return Err(integrity("child provenance coverage"));
            }
            meter.vector(child_certificates.len(), "child provenance certificates")?;
            let mut parsed_children = Vec::with_capacity(child_certificates.len());
            let mut serialized_child_bytes = 0usize;
            for child in child_certificates {
                let serialized_child = serde_json::to_vec(&child)
                    .map_err(|_| invalid("child provenance serialization"))?;
                serialized_child_bytes = serialized_child_bytes
                    .checked_add(serialized_child.len())
                    .ok_or_else(|| overflow("child provenance description"))?;
                let parsed_child: ChildProvenanceCertificate =
                    serde_json::from_slice(&serialized_child)
                        .map_err(|_| invalid("child provenance parsing"))?;
                if serde_json::to_vec(&parsed_child)
                    .map_err(|_| invalid("child provenance serialization"))?
                    != serialized_child
                {
                    return Err(integrity("child provenance round trip"));
                }
                parsed_children.push(parsed_child);
            }
            verify_child_family(&parsed_provenance, &certificate, &parsed_children, meter)
                .map_err(|_| integrity("child provenance family replay"))?;
            if serialized_child_bytes
                > child_family_description_envelope(&parsed_children)
                    .map_err(|_| integrity("child provenance family description"))?
                    .json_bytes_bound
            {
                return Err(integrity("child provenance family description"));
            }
            return Ok(certificate);
        }
    }
}

pub(super) fn capture_executed_source_checked(
    graph: &Bipartite,
    packing: &Packing,
    parameters: &Parameters,
    coordinate: usize,
    ring: &Ring,
    meter: &mut Meter,
) -> Result<(usize, crate::factorization_certificate::DescriptionEnvelope)> {
    let t = 1i64
        .checked_shl(u32::try_from(coordinate).map_err(|_| overflow("certificate valuation"))?)
        .ok_or_else(|| overflow("certificate valuation"))?;
    let kernel = deformation::Cauchy::new(parameters, t, &ring.one(), ring, meter)?;
    let modes = deformation::modes(graph, packing, coordinate, parameters, &kernel, meter)?;
    let produced = produce(
        graph, packing, parameters, coordinate, &kernel, &modes, None, meter,
    )?;
    let serialized = serde_json::to_vec(&produced)
        .map_err(|_| invalid("executed source certificate serialization"))?;
    let certificate: Certificate = serde_json::from_slice(&serialized)
        .map_err(|_| invalid("executed source certificate parsing"))?;
    standalone_verify(&certificate, meter)
        .map_err(|_| integrity("executed source certificate verification"))?;
    let cauchy_witness = cauchy_source_witness(&kernel, coordinate);
    let serialized_cauchy = serde_json::to_vec(&cauchy_witness)
        .map_err(|_| invalid("executed Cauchy witness serialization"))?;
    let cauchy_witness: CauchySourceWitness = serde_json::from_slice(&serialized_cauchy)
        .map_err(|_| invalid("executed Cauchy witness parsing"))?;
    verify_cauchy_source(&certificate, &cauchy_witness, meter)
        .map_err(|_| integrity("executed Cauchy witness replay"))?;
    let provenance = packing_provenance(graph, packing, parameters, coordinate, None);
    verify_cauchy_mode_coverage(
        &certificate,
        &cauchy_witness,
        &provenance.packet_pairs,
        meter,
    )
    .map_err(|_| integrity("executed Cauchy mode coverage"))?;
    if !assert_source_count_bridge(&certificate, parameters, coordinate, false) {
        return Err(invalid("executed source certificate dimensions"));
    }
    let envelope = description_envelope(&certificate)
        .map_err(|_| integrity("executed source certificate envelope"))?;
    Ok((serialized.len(), envelope))
}

#[test]
fn independent_certificate_checks_all_six_factorization_obligations() {
    let mut meter = Meter::new(Limits {
        steps: 10_000_000_000,
        ..Limits::default()
    });
    let ring = Ring::new(4, 5, &mut meter).unwrap();
    let parameters = Parameters {
        radix: 2,
        u: 1,
        a: 2,
        length: 4,
        array: 16,
    };
    let graph = fixture();
    let packing =
        packing::pack_labels(&graph, &parameters, vec![0], vec![3], false, &mut meter).unwrap();
    let coordinate = 0usize;
    let kernel = deformation::Cauchy::new(&parameters, 1, &ring.one(), &ring, &mut meter).unwrap();
    let modes = deformation::modes(
        &graph,
        &packing,
        coordinate,
        &parameters,
        &kernel,
        &mut meter,
    )
    .unwrap();
    assert!(!modes.is_empty());
    let produced = produce(
        &graph,
        &packing,
        &parameters,
        coordinate,
        &kernel,
        &modes,
        None,
        &mut meter,
    )
    .unwrap();
    let serialized = serde_json::to_vec(&produced).unwrap();
    assert_eq!(serialized.len(), 2_172);
    let certificate: Certificate = serde_json::from_slice(&serialized).unwrap();
    assert!(!assert_source_count_bridge(
        &certificate,
        &parameters,
        coordinate,
        true,
    ));
    assert_eq!(serde_json::to_vec(&certificate).unwrap(), serialized);
    assert!(serialized.len() <= description_envelope(&certificate).unwrap().json_bytes_bound);
    let mut standalone_meter = Meter::new(Limits {
        steps: 10_000_000_000,
        ..Limits::default()
    });
    standalone_verify(&certificate, &mut standalone_meter).unwrap();
    check(&certificate, &ring, &mut meter).unwrap();
    let cauchy_witness = cauchy_source_witness(&kernel, coordinate);
    let serialized_cauchy = serde_json::to_vec(&cauchy_witness).unwrap();
    let cauchy_witness: CauchySourceWitness = serde_json::from_slice(&serialized_cauchy).unwrap();
    assert_eq!(
        serde_json::to_vec(&cauchy_witness).unwrap(),
        serialized_cauchy
    );
    let cauchy_summary = verify_cauchy_source(&certificate, &cauchy_witness, &mut meter).unwrap();
    assert_eq!(cauchy_summary.grid_points, 8);
    assert_eq!(cauchy_summary.factors, 7);
    assert_eq!(cauchy_summary.weights, 16 * certificate.packets.len());
    assert!(cauchy_summary.mode_rows > 0);
    assert!(cauchy_summary.mode_columns > 0);

    let left = ring.point(&BigUint::from(15u8)).unwrap();
    let right = ring.point(&BigUint::from(15u8)).unwrap();
    let product = ring.mul(&left, &right, &mut meter).unwrap();
    let multiplication_witness = RingMultiplicationWitness {
        degree: ring.degree(),
        precision: ring.precision(),
        polynomial: ring.polynomial().clone(),
        lower_coefficients: (0..ring.degree())
            .map(|index| ring.polynomial().bit(index as u64))
            .collect(),
        left: left.coordinates().to_vec(),
        right: right.coordinates().to_vec(),
        product: product.coordinates().to_vec(),
    };
    let serialized_multiplication = serde_json::to_vec(&multiplication_witness).unwrap();
    let multiplication_witness: RingMultiplicationWitness =
        serde_json::from_slice(&serialized_multiplication).unwrap();
    let multiplication_summary =
        verify_ring_multiplication_witness(&multiplication_witness, &mut meter).unwrap();
    let fixture_values: Vec<BigUint> = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../research/curve25519_subex_pivot_20261125/ring_multiplication_witness.csv"
    ))
    .trim()
    .split(',')
    .map(|value| BigUint::parse_bytes(value.as_bytes(), 10).unwrap())
    .collect();
    let mut observed_values = vec![
        BigUint::from(multiplication_witness.degree),
        BigUint::from(multiplication_witness.precision),
        multiplication_witness.polynomial.clone(),
    ];
    observed_values.extend(
        multiplication_witness
            .lower_coefficients
            .iter()
            .map(|&value| BigUint::from(u8::from(value))),
    );
    observed_values.extend(multiplication_witness.left.iter().cloned());
    observed_values.extend(multiplication_witness.right.iter().cloned());
    observed_values.extend(multiplication_witness.product.iter().cloned());
    observed_values.push(BigUint::from(multiplication_summary.convolution_products));
    observed_values.push(BigUint::from(multiplication_summary.reductions));
    assert_eq!(observed_values, fixture_values);
    let mut bad_multiplication = multiplication_witness.clone();
    bad_multiplication.product[0] ^= BigUint::from(1u8);
    assert_eq!(
        verify_ring_multiplication_witness(&bad_multiplication, &mut meter).unwrap_err(),
        "ring multiplication product"
    );

    let mut bad_cauchy = cauchy_witness.clone();
    bad_cauchy.polynomial ^= BigUint::from(2u8);
    assert_eq!(
        verify_cauchy_source(&certificate, &bad_cauchy, &mut meter).unwrap_err(),
        "Cauchy witness polynomial"
    );
    let mut bad_cauchy = cauchy_witness.clone();
    bad_cauchy.lower_coefficients[0] ^= true;
    assert_eq!(
        verify_cauchy_source(&certificate, &bad_cauchy, &mut meter).unwrap_err(),
        "Cauchy witness polynomial coefficients"
    );
    let mut bad_cauchy = cauchy_witness.clone();
    bad_cauchy.factors[0].valuation += 1;
    assert_eq!(
        verify_cauchy_source(&certificate, &bad_cauchy, &mut meter).unwrap_err(),
        "Cauchy witness factor valuation"
    );
    let mut bad_cauchy = cauchy_witness.clone();
    bad_cauchy.factors.pop();
    assert_eq!(
        verify_cauchy_source(&certificate, &bad_cauchy, &mut meter).unwrap_err(),
        "Cauchy witness factor coverage"
    );
    let mut bad_cauchy = cauchy_witness.clone();
    bad_cauchy.factors[0].inverse[0] =
        (&bad_cauchy.factors[0].inverse[0] + BigUint::from(1u8)) % ring.modulus();
    assert_eq!(
        verify_cauchy_source(&certificate, &bad_cauchy, &mut meter).unwrap_err(),
        "Cauchy witness factor inverse"
    );
    let mut bad_cauchy = cauchy_witness.clone();
    bad_cauchy.numerator_unit[0] =
        (&bad_cauchy.numerator_unit[0] + BigUint::from(1u8)) % ring.modulus();
    assert_eq!(
        verify_cauchy_source(&certificate, &bad_cauchy, &mut meter).unwrap_err(),
        "Cauchy witness numerator unit"
    );
    let mut bad_cauchy = cauchy_witness.clone();
    bad_cauchy.coordinate += 1;
    assert_eq!(
        verify_cauchy_source(&certificate, &bad_cauchy, &mut meter).unwrap_err(),
        "Cauchy witness active coordinate"
    );
    let provenance = packing_provenance(&graph, &packing, &parameters, coordinate, None);
    let provenance_summary =
        verify_packing_provenance(&certificate, &provenance, &mut meter).unwrap();
    assert_eq!(provenance_summary.edges, 2);
    assert_eq!(provenance_summary.packet_length, 4);
    assert!(!provenance_summary.source_capacity_admitted);
    assert!(!provenance_summary.source_length_mass_admitted);
    assert!(!provenance_summary.source_guard_closure_admitted);
    let mode_summary = verify_cauchy_mode_coverage(
        &certificate,
        &cauchy_witness,
        &provenance.packet_pairs,
        &mut meter,
    )
    .unwrap();
    assert!(mode_summary.attempted_modes >= mode_summary.retained_modes);
    assert_eq!(mode_summary.retained_modes, certificate.modes.len());
    assert_eq!(mode_summary.mode_rows, cauchy_summary.mode_rows);
    assert_eq!(mode_summary.mode_columns, cauchy_summary.mode_columns);

    let mut bad_provenance = provenance;
    bad_provenance.packet_pairs[0][0].0 ^= 1;
    assert_eq!(
        verify_cauchy_mode_coverage(
            &certificate,
            &cauchy_witness,
            &bad_provenance.packet_pairs,
            &mut meter,
        )
        .unwrap_err(),
        "Cauchy mode header: expected=0, packet=1, tile=0, weight=1"
    );
    assert_eq!(
        verify_packing_provenance(&certificate, &bad_provenance, &mut meter).unwrap_err(),
        "packing provenance replay"
    );

    let conditional_parameters = Parameters {
        radix: 2,
        u: 1,
        a: 1,
        length: 2,
        array: 4,
    };
    let conditional_vertices: Vec<_> = (0..3)
        .map(|edge| TensorVertex {
            tensor: true_predicate(1),
            edges: vec![edge],
        })
        .collect();
    let conditional_graph = Bipartite {
        left: conditional_vertices.clone(),
        right: conditional_vertices,
        edges: (0..3)
            .map(|vertex| Edge {
                left: (vertex, 0),
                right: (vertex, 0),
            })
            .collect(),
    };
    conditional_graph.validate().unwrap();
    let conditional_packing =
        packing::pack(&conditional_graph, &conditional_parameters, 2, &mut meter).unwrap();
    let conditional_kernel =
        deformation::Cauchy::new(&conditional_parameters, 1, &ring.one(), &ring, &mut meter)
            .unwrap();
    let conditional_modes = deformation::modes(
        &conditional_graph,
        &conditional_packing,
        0,
        &conditional_parameters,
        &conditional_kernel,
        &mut meter,
    )
    .unwrap();
    let conditional_certificate = produce(
        &conditional_graph,
        &conditional_packing,
        &conditional_parameters,
        0,
        &conditional_kernel,
        &conditional_modes,
        Some(2),
        &mut meter,
    )
    .unwrap();
    let conditional_provenance = packing_provenance(
        &conditional_graph,
        &conditional_packing,
        &conditional_parameters,
        0,
        Some(2),
    );
    let conditional_summary = verify_packing_provenance(
        &conditional_certificate,
        &conditional_provenance,
        &mut meter,
    )
    .unwrap();
    assert!(conditional_summary.conditional_labels_verified);
    assert!(conditional_summary.source_bounds_verified);
    assert!(conditional_summary.source_capacity_admitted);
    assert!(!conditional_summary.source_length_mass_admitted);
    assert!(!conditional_summary.source_guard_closure_admitted);
    let group_projection = group_assignment_projection(&conditional_certificate).unwrap();
    assert_eq!(group_projection[0], 1);
    assert_eq!(group_projection[1], conditional_certificate.radix);
    assert_eq!(group_projection[2], conditional_certificate.edge_count);
    assert_eq!(
        &group_projection[5..5 + conditional_certificate.edge_count],
        conditional_certificate.left_groups.as_slice()
    );
    assert_eq!(
        &group_projection[5 + conditional_certificate.edge_count..],
        conditional_certificate.right_groups.as_slice()
    );
    let mut invalid_group_projection = conditional_certificate.clone();
    invalid_group_projection.left_groups[0] = invalid_group_projection.radix;
    assert_eq!(
        group_assignment_projection(&invalid_group_projection).unwrap_err(),
        "factorization group projection context"
    );
    let mut missing_degree_cap = conditional_provenance.clone();
    missing_degree_cap.source_degree_cap = None;
    assert_eq!(
        verify_packing_provenance(&conditional_certificate, &missing_degree_cap, &mut meter)
            .unwrap_err(),
        "packing provenance context"
    );
    let label_fixture_values: Vec<usize> = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../research/curve25519_subex_pivot_20261125/packing_label_witness.csv"
    ))
    .trim()
    .split(',')
    .map(|value| value.parse().unwrap())
    .collect();
    let observed_label_values = packing_label_projection(&conditional_provenance).unwrap();
    assert_eq!(observed_label_values, label_fixture_values);
    let mut bad_degree_cap = conditional_provenance.clone();
    bad_degree_cap.source_degree_cap = Some(0);
    assert_eq!(
        verify_packing_provenance(&conditional_certificate, &bad_degree_cap, &mut meter)
            .unwrap_err(),
        "packing provenance source degree cap"
    );

    let zero_beta = vec![0usize; conditional_modes.len()];
    let mut first_child_provenance = None;
    let mut conditional_children = Vec::new();
    for beta in &conditional_certificate.tuples {
        for group in 0..conditional_parameters.radix {
            let child = deformation::child(
                &conditional_graph,
                &conditional_packing,
                0,
                &conditional_parameters,
                &conditional_kernel,
                &conditional_modes,
                beta,
                group,
                &mut meter,
            )
            .unwrap();
            let produced_child = child_provenance(
                &conditional_graph,
                &conditional_packing,
                0,
                &conditional_parameters,
                &child,
                beta,
                group,
            );
            let serialized_child = serde_json::to_vec(&produced_child).unwrap();
            let parsed_child: ChildProvenanceCertificate =
                serde_json::from_slice(&serialized_child).unwrap();
            assert_eq!(serde_json::to_vec(&parsed_child).unwrap(), serialized_child);
            let summary = verify_child_provenance(
                &conditional_provenance,
                &conditional_certificate,
                &parsed_child,
                &mut meter,
            )
            .unwrap();
            assert_eq!(summary.edges, child.edges.len());
            assert_eq!(
                summary.edges,
                summary.row_group_edges + summary.column_group_edges
            );
            assert_eq!(
                summary.packet_entry_tables,
                conditional_certificate.packets.len()
            );
            assert!(summary.packet_entry_values >= summary.packet_entry_tables);
            if beta == &zero_beta
                && !parsed_child.edges.is_empty()
                && first_child_provenance.is_none()
            {
                first_child_provenance = Some(parsed_child.clone());
            }
            conditional_children.push(parsed_child);
        }
    }
    let family_summary = verify_child_family(
        &conditional_provenance,
        &conditional_certificate,
        &conditional_children,
        &mut meter,
    )
    .unwrap();
    assert_eq!(
        family_summary.children,
        conditional_certificate.tuples.len() * conditional_parameters.radix
    );
    assert_eq!(
        family_summary.child_edges,
        conditional_certificate.tuples.len() * 2 * conditional_graph.edges.len()
    );
    assert!(!family_summary.eightfold_edge_shrink_admitted);
    assert!(family_summary.rounded_height_child_bound_verified);
    assert!(!family_summary.packet_degree_guard_admitted);
    assert!(!family_summary.strict_source_refinement_admitted);
    let packing_description = packing_description_envelope(&conditional_provenance).unwrap();
    assert_eq!(
        serde_json::to_vec(&conditional_provenance).unwrap().len(),
        414
    );
    assert_eq!(packing_description.scalar_atoms, 34);
    assert_eq!(packing_description.json_bytes_bound, 4_672);
    assert_eq!(packing_description.canonical_binary_bits_bound, 2_564);
    let child_description = child_family_description_envelope(&conditional_children).unwrap();
    assert_eq!(child_description.certificates, 8);
    assert_eq!(child_description.scalar_atoms, 256);
    assert_eq!(child_description.json_bytes_bound, 37_440);
    assert_eq!(child_description.canonical_binary_bits_bound, 21_824);
    assert_eq!(
        conditional_children
            .iter()
            .map(|child| serde_json::to_vec(child).unwrap().len())
            .sum::<usize>(),
        3_764
    );
    let node = NodeRefinementCertificate {
        schema: NODE_REFINEMENT_SCHEMA,
        source: SourceDimensionWitness {
            a: 1,
            u: 1,
            coordinate: 0,
            require_normalized_precision: true,
        },
        factorization: conditional_certificate.clone(),
        packing: conditional_provenance.clone(),
        children: conditional_children.clone(),
    };
    let serialized_node = serde_json::to_vec(&node).unwrap();
    let parsed_node: NodeRefinementCertificate = serde_json::from_slice(&serialized_node).unwrap();
    assert_eq!(serde_json::to_vec(&parsed_node).unwrap(), serialized_node);
    let node_summary = verify_node_refinement(&parsed_node, &mut meter).unwrap();
    assert!(node_summary.normalized_precision_admitted);
    assert!(!node_summary.source_guard_closure_admitted);
    assert!(!node_summary.source_length_mass_admitted);
    assert!(node_summary.rounded_height_child_bound_verified);
    assert!(!node_summary.history_shrink_derived_from_packing);
    assert!(!node_summary.strict_source_refinement_admitted);
    let history_projection = history_shrink_projection(&node_summary);
    assert_eq!(history_projection[2], node_summary.edges);
    assert_eq!(
        history_projection[7],
        BigUint::from(node_summary.maximum_child_edges)
    );
    assert_eq!(
        verify_history_shrink_projection(&history_projection).unwrap_err(),
        "node refinement history projection context"
    );
    let exact_history_projection = [
        BigUint::from(1u8),
        BigUint::from(64u8),
        BigUint::from(32u8),
        BigUint::from(2u8),
        BigUint::from(4u8),
        BigUint::from(8u8),
        BigUint::from(8u8),
        BigUint::from(1u8),
    ];
    verify_history_shrink_projection(&exact_history_projection).unwrap();
    for (index, replacement) in [(2, 7u8), (6, 9u8), (7, 2u8)] {
        let mut invalid = exact_history_projection.clone();
        invalid[index] = BigUint::from(replacement);
        assert_eq!(
            verify_history_shrink_projection(&invalid).unwrap_err(),
            "node refinement history projection arithmetic"
        );
    }
    assert_eq!(
        verify_history_shrink_projection(&exact_history_projection[..7]).unwrap_err(),
        "node refinement history projection context"
    );
    let history_bundle =
        history_shrink_bundle_projection(&node_summary, &conditional_provenance).unwrap();
    assert_eq!(
        &history_bundle[..history_projection.len()],
        history_projection.as_slice()
    );
    assert_eq!(
        history_bundle[history_projection.len()..],
        packing_label_projection(&conditional_provenance)
            .unwrap()
            .into_iter()
            .map(BigUint::from)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        verify_strict_node_refinement(&parsed_node, &mut meter).unwrap_err(),
        "node refinement strict source guards"
    );
    let mut bad_node_context = parsed_node;
    bad_node_context.source.require_normalized_precision = false;
    assert_eq!(
        verify_node_refinement(&bad_node_context, &mut meter).unwrap_err(),
        "node refinement context"
    );
    let mut missing_child = conditional_children.clone();
    missing_child.pop();
    assert_eq!(
        verify_child_family(
            &conditional_provenance,
            &conditional_certificate,
            &missing_child,
            &mut meter
        )
        .unwrap_err(),
        "child provenance family coverage"
    );
    let mut duplicate_child = conditional_children;
    let repeated_child = duplicate_child[0].clone();
    *duplicate_child.last_mut().unwrap() = repeated_child;
    assert_eq!(
        verify_child_family(
            &conditional_provenance,
            &conditional_certificate,
            &duplicate_child,
            &mut meter
        )
        .unwrap_err(),
        "child provenance family duplicate"
    );
    let first_child = first_child_provenance.unwrap();
    let mut bad_child = first_child.clone();
    bad_child.edges[0].right_port += 1;
    assert_eq!(
        verify_child_provenance(
            &conditional_provenance,
            &conditional_certificate,
            &bad_child,
            &mut meter
        )
        .unwrap_err(),
        "child provenance replay"
    );
    let mut bad_packet_precision = first_child.clone();
    bad_packet_precision.packet_creation_precisions[0] += 1;
    assert_eq!(
        verify_child_provenance(
            &conditional_provenance,
            &conditional_certificate,
            &bad_packet_precision,
            &mut meter
        )
        .unwrap_err(),
        "child provenance replay"
    );
    let mut bad_parent_precision = conditional_provenance.clone();
    let source = &first_child.old_tensor_sources[0];
    match source.side {
        ParentSide::Row => bad_parent_precision.row_creation_precisions[source.vertex] += 1,
        ParentSide::Column => {
            bad_parent_precision.column_creation_precisions[source.vertex] += 1;
        }
    }
    assert_eq!(
        verify_child_provenance(
            &bad_parent_precision,
            &conditional_certificate,
            &first_child,
            &mut meter
        )
        .unwrap_err(),
        "child provenance replay"
    );
    let mut bad_beta = first_child;
    bad_beta.beta.push(0);
    assert_eq!(
        verify_child_provenance(
            &conditional_provenance,
            &conditional_certificate,
            &bad_beta,
            &mut meter
        )
        .unwrap_err(),
        "child provenance context"
    );

    let mut bad_conditional_labels = conditional_provenance;
    bad_conditional_labels.row_labels[0] ^= 1;
    assert_eq!(
        verify_packing_provenance(
            &conditional_certificate,
            &bad_conditional_labels,
            &mut meter
        )
        .unwrap_err(),
        "packing provenance conditional labels"
    );

    let mut bad_context = certificate.clone();
    bad_context.schema += 1;
    assert_eq!(
        check(&bad_context, &ring, &mut meter).unwrap_err(),
        "certificate context"
    );

    let mut bad_tiles = certificate.clone();
    bad_tiles.tiles = 1;
    assert_eq!(
        check(&bad_tiles, &ring, &mut meter).unwrap_err(),
        "certificate tile period"
    );
    let mut bad_tiles = certificate.clone();
    bad_tiles.tiles = 3;
    assert_eq!(
        check(&bad_tiles, &ring, &mut meter).unwrap_err(),
        "certificate tile period"
    );

    let mut bad_element = certificate.clone();
    bad_element.entries[0].values[0].0[0] = ring.modulus().clone();
    assert_eq!(
        check(&bad_element, &ring, &mut meter).unwrap_err(),
        "noncanonical certificate element"
    );

    let mut bad_count = certificate.clone();
    bad_count.radix = usize::MAX;
    assert_eq!(
        check(&bad_count, &ring, &mut meter).unwrap_err(),
        "entry table count overflow"
    );

    let mut bad_coverage = certificate.clone();
    let repeated = bad_coverage
        .packets
        .iter()
        .flatten()
        .flatten()
        .copied()
        .next()
        .unwrap();
    let empty = bad_coverage
        .packets
        .iter_mut()
        .flat_map(|slots| slots.iter_mut())
        .find(|slot| slot.is_none())
        .unwrap();
    *empty = Some(repeated);
    assert_eq!(
        check(&bad_coverage, &ring, &mut meter).unwrap_err(),
        "packet coverage"
    );

    let mut bad_mode = certificate.clone();
    bad_mode.modes[0].target ^= 1;
    assert!(matches!(
        check(&bad_mode, &ring, &mut meter).unwrap_err().as_str(),
        "mode header" | "mode row placement" | "crossing mode series"
    ));

    let mut duplicate_mode_slot = certificate.clone();
    let repeated_row = duplicate_mode_slot.modes[0].rows[0].clone();
    duplicate_mode_slot.modes[0].rows.push(repeated_row);
    assert_eq!(
        check(&duplicate_mode_slot, &ring, &mut meter).unwrap_err(),
        "mode row duplicate"
    );

    let mut duplicate_mode = certificate.clone();
    duplicate_mode.modes.push(duplicate_mode.modes[0].clone());
    assert_eq!(
        check(&duplicate_mode, &ring, &mut meter).unwrap_err(),
        "mode tile duplicate"
    );

    let mut bad_tuples = certificate.clone();
    bad_tuples.tuples.pop();
    assert_eq!(
        check(&bad_tuples, &ring, &mut meter).unwrap_err(),
        "weighted tuple coverage"
    );

    let mut bad_entry = certificate.clone();
    let changed = ring
        .add(
            &bad_entry.entries[0].values[0].decode(&ring).unwrap(),
            &ring.one(),
            &mut meter,
        )
        .unwrap();
    bad_entry.entries[0].values[0] = CertifiedElement::encode(changed);
    assert!(check(&bad_entry, &ring, &mut meter)
        .unwrap_err()
        .starts_with("packet coefficient table:"));
}

fn check_strict_source_guard_exact_boundary(
    packet_exponent: usize,
    edges: usize,
    expected_projection: Option<&str>,
) -> (usize, usize, u64) {
    let mut meter = Meter::new(Limits {
        steps: 100_000_000,
        ..Limits::default()
    });
    let parameters = Parameters {
        radix: 2,
        u: 1,
        a: packet_exponent,
        length: 1usize << packet_exponent,
        array: 2usize.pow((2 * packet_exponent) as u32),
    };
    let endpoints: Vec<_> = (0..edges).collect();
    let graph = partition_graph(&endpoints, &endpoints);
    graph.validate().unwrap();
    let packing = packing::pack(&graph, &parameters, 1, &mut meter).unwrap();
    let certificate = Certificate {
        schema: SCHEMA,
        degree: 1,
        radix: parameters.radix,
        precision: 1,
        tiles: 2,
        edge_count: graph.edges.len(),
        packets: packing
            .packets
            .iter()
            .map(|packet| packet.slots.clone())
            .collect(),
        left_groups: graph
            .edges
            .iter()
            .map(|edge| packing::digit(packing.row_labels[edge.left.0], 0, parameters.radix))
            .collect(),
        right_groups: graph
            .edges
            .iter()
            .map(|edge| packing::digit(packing.column_labels[edge.right.0], 0, parameters.radix))
            .collect(),
        weights: Vec::new(),
        modes: Vec::new(),
        tuples: Vec::new(),
        entries: Vec::new(),
    };
    let provenance = packing_provenance(&graph, &packing, &parameters, 0, Some(1));
    let summary = verify_packing_provenance(&certificate, &provenance, &mut meter).unwrap();
    assert_eq!(summary.edges, edges);
    assert!(summary.conditional_labels_verified);
    assert!(summary.source_bounds_verified);
    assert_eq!(summary.padded_class_height, summary.maximum_class);
    assert!(summary.source_length_mass_admitted);
    assert!(summary.source_guard_closure_admitted);
    if let Some(expected_projection) = expected_projection {
        let projection = packing_label_projection(&provenance).unwrap();
        let fixture_values: Vec<usize> = expected_projection
            .trim()
            .split(',')
            .map(|value| value.parse().unwrap())
            .collect();
        assert_eq!(projection, fixture_values);
    }
    (summary.packets, summary.maximum_class, meter.used)
}

#[test]
fn strict_source_guard_projection_accepts_nontrivial_exact_boundary() {
    let _ = check_strict_source_guard_exact_boundary(
        1,
        32,
        Some(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../research/curve25519_subex_pivot_20261125/packing_label_source_guard_witness.csv"
        ))),
    );
}

#[test]
fn strict_source_guard_projection_accepts_larger_exact_boundaries() {
    for (packet_exponent, edges) in [(2, 512), (3, 8_192), (4, 131_072)] {
        let (packets, maximum_class, steps) =
            check_strict_source_guard_exact_boundary(packet_exponent, edges, None);
        let label_classes = 2usize.pow((2 * packet_exponent) as u32);
        assert_eq!(maximum_class, edges / label_classes);
        eprintln!(
            "exact-boundary a={packet_exponent} M={edges} D={label_classes} \
             max_class={maximum_class} packets={packets} charged_steps={steps}"
        );
    }
}

#[test]
fn serialized_certificate_covers_every_three_edge_partition_shape() {
    let mut meter = Meter::new(Limits {
        steps: 100_000_000_000,
        ..Limits::default()
    });
    let ring = Ring::new(4, 5, &mut meter).unwrap();
    let parameters = Parameters {
        radix: 2,
        u: 1,
        a: 2,
        length: 4,
        array: 16,
    };
    let kernel = deformation::Cauchy::new(&parameters, 1, &ring.one(), &ring, &mut meter).unwrap();
    let partitions = [[0usize, 0, 0], [0, 0, 1], [0, 1, 0], [0, 1, 1], [0, 1, 2]];
    let mut cases = 0usize;
    let mut total_bytes = 0usize;
    let mut maximum_bytes = 0usize;
    let mut maximum_case = None;
    let mut source_admitted = 0usize;
    for left_groups in &partitions {
        for right_groups in &partitions {
            for schedule in 0..4 {
                let graph = partition_graph(left_groups, right_groups);
                let rows = refinement_labels(graph.left.len(), schedule, 0);
                let columns = refinement_labels(graph.right.len(), schedule, 1);
                let packing =
                    packing::pack_labels(&graph, &parameters, rows, columns, false, &mut meter)
                        .unwrap();
                let modes =
                    deformation::modes(&graph, &packing, 0, &parameters, &kernel, &mut meter)
                        .unwrap();
                let produced = produce(
                    &graph,
                    &packing,
                    &parameters,
                    0,
                    &kernel,
                    &modes,
                    None,
                    &mut meter,
                )
                .unwrap();
                let serialized = serde_json::to_vec(&produced).unwrap();
                total_bytes += serialized.len();
                if serialized.len() > maximum_bytes {
                    maximum_bytes = serialized.len();
                    maximum_case = Some((*left_groups, *right_groups, schedule));
                }
                let certificate: Certificate = serde_json::from_slice(&serialized).unwrap();
                source_admitted += usize::from(assert_source_count_bridge(
                    &certificate,
                    &parameters,
                    0,
                    true,
                ));
                assert!(
                    serialized.len()
                        <= description_envelope(&certificate).unwrap().json_bytes_bound
                );
                standalone_verify(&certificate, &mut meter).unwrap_or_else(|error| {
                    panic!(
                        "left={left_groups:?}, right={right_groups:?}, schedule={schedule}: {error}"
                    )
                });
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 100);
    assert_eq!(total_bytes, 318_734);
    assert_eq!(maximum_bytes, 10_146);
    assert_eq!(maximum_case, Some(([0, 1, 1], [0, 1, 1], 3)));
    assert_eq!(source_admitted, 52);
}

#[test]
fn serialized_certificate_covers_four_edge_partition_orbits() {
    let representatives = partition_pair_orbits(4);
    assert_eq!(representatives.len(), 33);
    let mut meter = Meter::new(Limits {
        steps: 500_000_000_000,
        ..Limits::default()
    });
    let ring = Ring::new(4, 5, &mut meter).unwrap();
    let parameters = Parameters {
        radix: 2,
        u: 1,
        a: 2,
        length: 4,
        array: 16,
    };
    let kernel = deformation::Cauchy::new(&parameters, 1, &ring.one(), &ring, &mut meter).unwrap();
    let mut cases = 0usize;
    let mut total_bytes = 0usize;
    let mut maximum_bytes = 0usize;
    let mut maximum_case = None;
    let mut source_admitted = 0usize;
    for (left_groups, right_groups) in representatives {
        for schedule in 0..4 {
            let graph = partition_graph(&left_groups, &right_groups);
            let rows = refinement_labels(graph.left.len(), schedule, 0);
            let columns = refinement_labels(graph.right.len(), schedule, 1);
            let packing =
                packing::pack_labels(&graph, &parameters, rows, columns, false, &mut meter)
                    .unwrap();
            let modes =
                deformation::modes(&graph, &packing, 0, &parameters, &kernel, &mut meter).unwrap();
            let produced = produce(
                &graph,
                &packing,
                &parameters,
                0,
                &kernel,
                &modes,
                None,
                &mut meter,
            )
            .unwrap();
            let serialized = serde_json::to_vec(&produced).unwrap();
            total_bytes += serialized.len();
            if serialized.len() > maximum_bytes {
                maximum_bytes = serialized.len();
                maximum_case = Some((left_groups.clone(), right_groups.clone(), schedule));
            }
            let certificate: Certificate = serde_json::from_slice(&serialized).unwrap();
            source_admitted += usize::from(assert_source_count_bridge(
                &certificate,
                &parameters,
                0,
                true,
            ));
            assert!(
                serialized.len() <= description_envelope(&certificate).unwrap().json_bytes_bound
            );
            standalone_verify(&certificate, &mut meter).unwrap_or_else(|error| {
                panic!("left={left_groups:?}, right={right_groups:?}, schedule={schedule}: {error}")
            });
            cases += 1;
        }
    }
    assert_eq!(cases, 132);
    assert_eq!(total_bytes, 792_556);
    assert_eq!(maximum_bytes, 20_777);
    assert_eq!(maximum_case, Some((vec![0, 0, 1, 1], vec![0, 0, 1, 1], 3)));
    assert_eq!(source_admitted, 87);
}

#[test]
fn serialized_certificate_covers_five_edge_partition_orbits() {
    let representatives = partition_pair_orbits(5);
    assert_eq!(representatives.len(), 91);
    let mut meter = Meter::new(Limits {
        steps: 2_000_000_000_000,
        ..Limits::default()
    });
    let ring = Ring::new(4, 5, &mut meter).unwrap();
    let parameters = Parameters {
        radix: 2,
        u: 1,
        a: 2,
        length: 4,
        array: 16,
    };
    let kernel = deformation::Cauchy::new(&parameters, 1, &ring.one(), &ring, &mut meter).unwrap();
    let mut cases = 0usize;
    let mut total_bytes = 0usize;
    let mut maximum_bytes = 0usize;
    let mut maximum_case = None;
    let mut maximum_json_bound = 0usize;
    let mut maximum_binary_bits_bound = 0usize;
    let mut source_admitted = 0usize;
    for (left_groups, right_groups) in representatives {
        for schedule in 0..4 {
            let graph = partition_graph(&left_groups, &right_groups);
            let rows = refinement_labels(graph.left.len(), schedule, 0);
            let columns = refinement_labels(graph.right.len(), schedule, 1);
            let packing =
                packing::pack_labels(&graph, &parameters, rows, columns, false, &mut meter)
                    .unwrap();
            let modes =
                deformation::modes(&graph, &packing, 0, &parameters, &kernel, &mut meter).unwrap();
            let produced = produce(
                &graph,
                &packing,
                &parameters,
                0,
                &kernel,
                &modes,
                None,
                &mut meter,
            )
            .unwrap();
            let serialized = serde_json::to_vec(&produced).unwrap();
            total_bytes += serialized.len();
            if serialized.len() > maximum_bytes {
                maximum_bytes = serialized.len();
                maximum_case = Some((left_groups.clone(), right_groups.clone(), schedule));
            }
            let certificate: Certificate = serde_json::from_slice(&serialized).unwrap();
            source_admitted += usize::from(assert_source_count_bridge(
                &certificate,
                &parameters,
                0,
                true,
            ));
            let envelope = description_envelope(&certificate).unwrap();
            assert!(serialized.len() <= envelope.json_bytes_bound);
            maximum_json_bound = maximum_json_bound.max(envelope.json_bytes_bound);
            maximum_binary_bits_bound =
                maximum_binary_bits_bound.max(envelope.canonical_binary_bits_bound);
            standalone_verify(&certificate, &mut meter).unwrap_or_else(|error| {
                panic!("left={left_groups:?}, right={right_groups:?}, schedule={schedule}: {error}")
            });
            cases += 1;
        }
    }
    assert_eq!(cases, 364);
    assert_eq!(total_bytes, 3_652_442);
    assert_eq!(maximum_bytes, 43_460);
    assert_eq!(
        maximum_case,
        Some((vec![0, 0, 0, 1, 1], vec![0, 0, 0, 1, 1], 3))
    );
    assert_eq!(maximum_json_bound, 32_519_488);
    assert_eq!(maximum_binary_bits_bound, 19_154_282);
    assert_eq!(source_admitted, 146);
}
