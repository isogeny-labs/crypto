//! Independent coupled-port and nested-precision controls for the source path.

use super::*;
use crate::contraction::{Edge, Tensor, TensorKind, TensorVertex};
use crate::network::FnPredicate;
use crate::Limits;
use std::sync::Arc;

fn evaluate_generated_modes(
    graph: &Bipartite,
    packing: &Packing,
    parameters: &Parameters,
    coordinate: usize,
    kernel: &deformation::Cauchy,
    modes: &[deformation::Mode],
    meter: &mut Meter,
) -> Result<Element> {
    let groups = packing.packets.len() * parameters.radix;
    let mut bounds = vec![0usize; groups];
    for (packet_id, packet) in packing.packets.iter().enumerate() {
        for &edge in packet.slots.iter().flatten() {
            let group = packing::digit(
                packing.row_labels[graph.edges[edge].left.0],
                coordinate,
                parameters.radix,
            );
            bounds[packet_id * parameters.radix + group] += 1;
        }
    }
    let mut usage = vec![0usize; groups];
    let mut beta = vec![0usize; modes.len()];
    let mut weight = 0usize;
    let mut total = kernel.ring.zero();
    loop {
        let mut product = kernel.ring.one();
        for group in 0..parameters.radix {
            let child = deformation::child(
                graph, packing, coordinate, parameters, kernel, modes, &beta, group, meter,
            )?;
            let value = brute_tensor(&child, &kernel.ring, &mut Statistics::default(), meter)?;
            product = kernel.ring.mul(&product, &value, meter)?;
        }
        total = kernel.ring.add(&total, &product, meter)?;
        if !next_tuple(
            &mut beta,
            modes,
            &mut usage,
            &bounds,
            &mut weight,
            kernel.ring.precision(),
            parameters.radix,
            meter,
        )? {
            return Ok(total);
        }
    }
}

fn direct_deformed_contraction(
    graph: &Bipartite,
    packing: &Packing,
    kernel: &deformation::Cauchy,
    meter: &mut Meter,
) -> Result<Element> {
    let incidences = graph
        .edges
        .len()
        .checked_mul(2)
        .ok_or_else(|| overflow("direct deformation incidences"))?;
    let assignments = 1usize
        .checked_shl(u32::try_from(incidences).map_err(|_| overflow("direct deformation shift"))?)
        .ok_or_else(|| overflow("direct deformation assignments"))?;
    let mut total = kernel.ring.zero();
    for mask in 0..assignments {
        let left_bits: Vec<_> = (0..graph.edges.len())
            .map(|edge| mask & (1usize << edge) != 0)
            .collect();
        let right_bits: Vec<_> = (0..graph.edges.len())
            .map(|edge| mask & (1usize << (graph.edges.len() + edge)) != 0)
            .collect();
        let mut product = kernel.ring.one();
        for vertex in &graph.left {
            let bits: Vec<_> = vertex.edges.iter().map(|&edge| left_bits[edge]).collect();
            let value = vertex.tensor.entry(&bits, &kernel.ring, meter)?;
            product = kernel.ring.mul(&product, &value, meter)?;
        }
        for vertex in &graph.right {
            let bits: Vec<_> = vertex.edges.iter().map(|&edge| right_bits[edge]).collect();
            let value = vertex.tensor.entry(&bits, &kernel.ring, meter)?;
            product = kernel.ring.mul(&product, &value, meter)?;
        }
        for packet in &packing.packets {
            for (row, &row_edge) in packet
                .slots
                .iter()
                .enumerate()
                .filter_map(|(slot, edge)| edge.as_ref().map(|edge| (slot, edge)))
            {
                let mut row_sum = kernel.ring.zero();
                for (column, &column_edge) in packet
                    .slots
                    .iter()
                    .enumerate()
                    .filter_map(|(slot, edge)| edge.as_ref().map(|edge| (slot, edge)))
                {
                    if right_bits[column_edge] {
                        let weight = kernel.weight(row, column, meter)?;
                        row_sum = kernel.ring.add(&row_sum, &weight, meter)?;
                    }
                }
                let factor = if left_bits[row_edge] {
                    row_sum
                } else {
                    kernel.ring.sub(&kernel.ring.one(), &row_sum, meter)?
                };
                product = kernel.ring.mul(&product, &factor, meter)?;
            }
        }
        total = kernel.ring.add(&total, &product, meter)?;
    }
    Ok(total)
}

fn predicate(arity: usize, table: u8) -> Arc<Tensor> {
    Arc::new(Tensor {
        creation_precision: 3,
        kind: TensorKind::Boolean(Arc::new(FnPredicate::new(arity, move |bits, _| {
            let index = bits
                .iter()
                .enumerate()
                .fold(0, |n, (i, bit)| n | (usize::from(*bit) << i));
            Ok(table & (1 << index) != 0)
        }))),
    })
}

fn two_port_graph(
    left_split: bool,
    right_split: bool,
    left_table: u8,
    right_table: u8,
) -> Bipartite {
    fn side(split: bool, table: u8) -> Vec<TensorVertex> {
        if split {
            vec![
                TensorVertex {
                    tensor: predicate(1, table & 3),
                    edges: vec![0],
                },
                TensorVertex {
                    tensor: predicate(1, (table >> 2) & 3),
                    edges: vec![1],
                },
            ]
        } else {
            vec![TensorVertex {
                tensor: predicate(2, table),
                edges: vec![0, 1],
            }]
        }
    }

    Bipartite {
        left: side(left_split, left_table),
        right: side(right_split, right_table),
        edges: (0..2)
            .map(|edge| Edge {
                left: if left_split { (edge, 0) } else { (0, edge) },
                right: if right_split { (edge, 0) } else { (0, edge) },
            })
            .collect(),
    }
}

fn partition_basis_graph(left_groups: &[usize], right_groups: &[usize], basis: usize) -> Bipartite {
    fn side(
        groups: &[usize],
        basis: usize,
        offset: usize,
    ) -> (Vec<TensorVertex>, Vec<(usize, usize)>) {
        let vertices = groups.iter().copied().max().map_or(0, |group| group + 1);
        let mut ports = vec![Vec::new(); vertices];
        let mut endpoints = vec![(0usize, 0usize); groups.len()];
        for (edge, &vertex) in groups.iter().enumerate() {
            endpoints[edge] = (vertex, ports[vertex].len());
            ports[vertex].push(edge);
        }
        let tensors = ports
            .into_iter()
            .map(|edges| {
                let assignment = edges
                    .iter()
                    .enumerate()
                    .fold(0usize, |value, (port, edge)| {
                        value | (((basis >> (offset + edge)) & 1) << port)
                    });
                TensorVertex {
                    tensor: predicate(edges.len(), 1u8 << assignment),
                    edges,
                }
            })
            .collect();
        (tensors, endpoints)
    }

    assert_eq!(left_groups.len(), right_groups.len());
    let edges = left_groups.len();
    let (left, left_endpoints) = side(left_groups, basis, 0);
    let (right, right_endpoints) = side(right_groups, basis, edges);
    Bipartite {
        left,
        right,
        edges: (0..edges)
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

fn direct_and_extracted(
    graph: &Bipartite,
    row_labels: &[usize],
    column_labels: &[usize],
    z: &Element,
    ring: &Ring,
    parameters: &Parameters,
    meter: &mut Meter,
) -> Result<(Element, Element)> {
    let packing = packing::pack_labels(
        graph,
        parameters,
        row_labels.to_vec(),
        column_labels.to_vec(),
        false,
        meter,
    )?;
    let t = 1i64;
    let coordinate = deformation::valuation(t);
    let kernel = deformation::Cauchy::new(parameters, t, z, ring, meter)?;
    let modes = deformation::modes(graph, &packing, coordinate, parameters, &kernel, meter)?;
    let direct = direct_deformed_contraction(graph, &packing, &kernel, meter)?;
    let extracted = evaluate_generated_modes(
        graph, &packing, parameters, coordinate, &kernel, &modes, meter,
    )?;
    Ok((direct, extracted))
}

fn forced(
    graph: &Bipartite,
    ring: &Ring,
    rows: Vec<usize>,
    columns: Vec<usize>,
    layers: usize,
    meter: &mut Meter,
) -> Result<Element> {
    let parameters = Parameters {
        radix: 2,
        u: 1,
        a: 2,
        length: 4,
        array: 16,
    };
    forced_at(
        graph,
        ring,
        rows,
        columns,
        layers,
        0,
        &parameters,
        &mut Vec::new(),
        meter,
    )
}

#[allow(clippy::too_many_arguments)]
fn forced_at(
    graph: &Bipartite,
    ring: &Ring,
    rows: Vec<usize>,
    columns: Vec<usize>,
    layers: usize,
    depth: usize,
    parameters: &Parameters,
    visits: &mut Vec<usize>,
    meter: &mut Meter,
) -> Result<Element> {
    if visits.len() <= depth {
        visits.resize(depth + 1, 0);
    }
    visits[depth] += 1;
    if layers == 0 || graph.edges.is_empty() {
        return brute_tensor(graph, ring, &mut Statistics::default(), meter);
    }
    let packing = packing::pack_labels(graph, parameters, rows, columns, false, meter)?;
    active(
        graph,
        ring,
        parameters,
        &packing,
        false,
        usize::MAX,
        &mut |child, child_ring, work| {
            forced_at(
                child,
                child_ring,
                vec![0; child.left.len()],
                vec![parameters.length - 1; child.right.len()],
                layers - 1,
                depth + 1,
                parameters,
                visits,
                work,
            )
        },
        meter,
    )
}

#[test]
fn coupled_packets_match_direct_counts_with_parallel_ports_and_quadratic_modes() {
    let mut meter = Meter::new(Limits {
        steps: 5_000_000_000,
        ..Limits::default()
    });
    let ring = Ring::new(4, 3, &mut meter).unwrap();
    // Two inputs of the same predicate are correlated. Treating their mode
    // sums independently would lose that correlation.
    let parallel = Bipartite {
        left: vec![TensorVertex {
            tensor: predicate(2, 6),
            edges: vec![0, 1],
        }],
        right: vec![TensorVertex {
            tensor: predicate(2, 6),
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
    };
    parallel.validate().unwrap();
    let expected = brute_tensor(&parallel, &ring, &mut Statistics::default(), &mut meter).unwrap();
    assert_eq!(expected, ring.integer(&2u8.into()));
    assert_eq!(
        forced(&parallel, &ring, vec![0], vec![3], 1, &mut meter).unwrap(),
        expected
    );

    // Both rows lie in the same target group and the columns occupy j=0,1
    // in one packet. Its degree-two incoming coefficient and outgoing form
    // powers must be retained, including the mixed term with coefficient 2.
    for right_tables in [[3u8, 3u8], [2, 1]] {
        let graph = Bipartite {
            left: (0..2)
                .map(|edge| TensorVertex {
                    tensor: predicate(1, 2),
                    edges: vec![edge],
                })
                .collect(),
            right: right_tables
                .iter()
                .enumerate()
                .map(|(edge, &table)| TensorVertex {
                    tensor: predicate(1, table),
                    edges: vec![edge],
                })
                .collect(),
            edges: vec![
                Edge {
                    left: (0, 0),
                    right: (0, 0),
                },
                Edge {
                    left: (1, 0),
                    right: (1, 0),
                },
            ],
        };
        graph.validate().unwrap();
        let expected = brute_tensor(&graph, &ring, &mut Statistics::default(), &mut meter).unwrap();
        let actual = forced(&graph, &ring, vec![0, 2], vec![3, 1], 1, &mut meter).unwrap();
        assert_eq!(actual, expected, "column predicates {right_tables:?}");
    }
}

#[test]
fn direct_deformation_matches_group_factorization_for_all_binary_tables() {
    let mut meter = Meter::new(Limits {
        steps: 5_000_000_000,
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
    let cases = [
        (0usize, 3usize, ring.zero()),
        (1, 2, ring.one()),
        (2, 2, ring.point(&2u8.into()).unwrap()),
    ];
    for (row_label, column_label, z) in cases {
        for left_table in 0..16u8 {
            for right_table in 0..16u8 {
                let graph = two_port_graph(false, false, left_table, right_table);
                let (expected, actual) = direct_and_extracted(
                    &graph,
                    &[row_label],
                    &[column_label],
                    &z,
                    &ring,
                    &parameters,
                    &mut meter,
                )
                .unwrap();
                assert_eq!(
                    actual, expected,
                    "labels={row_label}/{column_label}, tables={left_table}/{right_table}",
                );
            }
        }
    }

    let z = ring.point(&3u8.into()).unwrap();
    for (left_split, right_split, rows, columns) in [
        (true, false, vec![0, 1], vec![3]),
        (false, true, vec![0], vec![2, 3]),
        (true, true, vec![0, 1], vec![2, 3]),
    ] {
        for left_table in 0..16u8 {
            for right_table in 0..16u8 {
                let graph = two_port_graph(left_split, right_split, left_table, right_table);
                let (expected, actual) = direct_and_extracted(
                    &graph,
                    &rows,
                    &columns,
                    &z,
                    &ring,
                    &parameters,
                    &mut meter,
                )
                .unwrap();
                assert_eq!(
                    actual, expected,
                    "split={left_split}/{right_split}, tables={left_table}/{right_table}",
                );
            }
        }
    }
}

#[test]
fn direct_factorization_matches_every_three_edge_partition_basis() {
    let mut meter = Meter::new(Limits {
        steps: 20_000_000_000,
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
    let z = ring.point(&3u8.into()).unwrap();
    let partitions = [[0usize, 0, 0], [0, 0, 1], [0, 1, 0], [0, 1, 1], [0, 1, 2]];
    let mut cases = 0usize;
    for left_groups in &partitions {
        for right_groups in &partitions {
            for schedule in 0..4 {
                for basis in 0..64usize {
                    let graph = partition_basis_graph(left_groups, right_groups, basis);
                    let rows = refinement_labels(graph.left.len(), schedule, 0);
                    let columns = refinement_labels(graph.right.len(), schedule, 1);
                    let (expected, actual) = direct_and_extracted(
                        &graph,
                        &rows,
                        &columns,
                        &z,
                        &ring,
                        &parameters,
                        &mut meter,
                    )
                    .unwrap();
                    assert_eq!(
                        actual, expected,
                        "left={left_groups:?}, right={right_groups:?}, schedule={schedule}, basis={basis}",
                    );
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 6_400);
}

#[test]
fn generated_mode_removal_or_reassignment_changes_the_group_product() {
    let mut meter = Meter::new(Limits {
        steps: 5_000_000_000,
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
    let graph = Bipartite {
        left: vec![TensorVertex {
            tensor: predicate(2, 6),
            edges: vec![0, 1],
        }],
        right: vec![TensorVertex {
            tensor: predicate(2, 6),
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
    };
    let packing =
        packing::pack_labels(&graph, &parameters, vec![0], vec![3], false, &mut meter).unwrap();
    let t = 1i64;
    let z = ring.one();
    let coordinate = deformation::valuation(t);
    let kernel = deformation::Cauchy::new(&parameters, t, &z, &ring, &mut meter).unwrap();
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
    let expected = direct_deformed_contraction(&graph, &packing, &kernel, &mut meter).unwrap();
    let extracted = evaluate_generated_modes(
        &graph,
        &packing,
        &parameters,
        coordinate,
        &kernel,
        &modes,
        &mut meter,
    )
    .unwrap();
    assert_eq!(extracted, expected);
    let production = evaluate_at(
        &graph,
        &packing,
        &parameters,
        t,
        &z,
        &ring,
        false,
        usize::MAX,
        &mut |child, child_ring, work| {
            brute_tensor(child, child_ring, &mut Statistics::default(), work)
        },
        &mut meter,
    )
    .unwrap();
    assert_eq!(production, expected);

    let mut removal_detected = false;
    let mut reassignment_detected = false;
    for index in 0..modes.len() {
        let mut removed = deformation::modes(
            &graph,
            &packing,
            coordinate,
            &parameters,
            &kernel,
            &mut meter,
        )
        .unwrap();
        removed.remove(index);
        removal_detected |= evaluate_generated_modes(
            &graph,
            &packing,
            &parameters,
            coordinate,
            &kernel,
            &removed,
            &mut meter,
        )
        .map_or(true, |value| value != expected);

        let mut reassigned = deformation::modes(
            &graph,
            &packing,
            coordinate,
            &parameters,
            &kernel,
            &mut meter,
        )
        .unwrap();
        reassigned[index].target ^= 1;
        reassignment_detected |= evaluate_generated_modes(
            &graph,
            &packing,
            &parameters,
            coordinate,
            &kernel,
            &reassigned,
            &mut meter,
        )
        .map_or(true, |value| value != expected);
        if removal_detected && reassignment_detected {
            break;
        }
    }
    assert!(removal_detected);
    assert!(reassignment_detected);
}

#[test]
fn coupled_nested_contraction_preserves_old_packet_creation_precision() {
    let mut meter = Meter::new(Limits {
        steps: 5_000_000_000,
        ..Limits::default()
    });
    let ring = Ring::new(4, 3, &mut meter).unwrap();
    let graph = Bipartite {
        left: vec![TensorVertex {
            tensor: predicate(1, 2),
            edges: vec![0],
        }],
        right: vec![TensorVertex {
            tensor: predicate(1, 3),
            edges: vec![0],
        }],
        edges: vec![Edge {
            left: (0, 0),
            right: (0, 0),
        }],
    };
    graph.validate().unwrap();
    let expected = brute_tensor(&graph, &ring, &mut Statistics::default(), &mut meter).unwrap();
    assert_eq!(
        forced(&graph, &ring, vec![0], vec![3], 2, &mut meter).unwrap(),
        expected
    );
}

#[test]
fn three_nested_contractions_reach_the_third_child_and_match_direct_count() {
    let mut meter = Meter::new(Limits {
        steps: 20_000_000_000,
        ..Limits::default()
    });
    let ring = Ring::new(4, 3, &mut meter).unwrap();
    let graph = Bipartite {
        left: vec![TensorVertex {
            tensor: predicate(1, 2),
            edges: vec![0],
        }],
        right: vec![TensorVertex {
            tensor: predicate(1, 3),
            edges: vec![0],
        }],
        edges: vec![Edge {
            left: (0, 0),
            right: (0, 0),
        }],
    };
    graph.validate().unwrap();
    let expected = brute_tensor(&graph, &ring, &mut Statistics::default(), &mut meter).unwrap();
    let parameters = Parameters {
        radix: 2,
        u: 1,
        a: 1,
        length: 2,
        array: 4,
    };
    let mut visits = Vec::new();
    let actual = forced_at(
        &graph,
        &ring,
        vec![0],
        vec![1],
        3,
        0,
        &parameters,
        &mut visits,
        &mut meter,
    )
    .unwrap();
    assert_eq!(actual, expected);
    assert!(visits.get(3).copied().unwrap_or(0) > 0, "visits={visits:?}");
}

#[test]
fn both_taper_levels_match_direct_counts_for_crossing_and_local_ports() {
    let mut meter = Meter::new(Limits {
        steps: 5_000_000_000,
        ..Limits::default()
    });
    let ring = Ring::new(4, 3, &mut meter).unwrap();
    let parameters = Parameters {
        radix: 2,
        u: 2,
        a: 2,
        length: 4,
        array: 16,
    };
    // Existing forced controls retain only odd Hermite nodes (u=1).
    // Here both v2(t)=0 and v2(t)=1 contribute, selecting different label
    // coordinates and different multiplicities/child precisions.
    for (rows, columns) in [(0, 3), (1, 2), (2, 2)] {
        for (left_table, right_table) in [(6, 15), (9, 6), (15, 15)] {
            let graph = Bipartite {
                left: vec![TensorVertex {
                    tensor: predicate(2, left_table),
                    edges: vec![0, 1],
                }],
                right: vec![TensorVertex {
                    tensor: predicate(2, right_table),
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
            };
            let packing = packing::pack_labels(
                &graph,
                &parameters,
                vec![rows],
                vec![columns],
                false,
                &mut meter,
            )
            .unwrap();
            let expected =
                brute_tensor(&graph, &ring, &mut Statistics::default(), &mut meter).unwrap();
            let actual = active(
                &graph,
                &ring,
                &parameters,
                &packing,
                false,
                usize::MAX,
                &mut |child, child_ring, work| {
                    brute_tensor(child, child_ring, &mut Statistics::default(), work)
                },
                &mut meter,
            )
            .unwrap();
            assert_eq!(
                actual, expected,
                "labels {rows}/{columns}, tables {left_table}/{right_table}"
            );
        }
    }
}
