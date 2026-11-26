//! Independent coupled-port and nested-precision controls for the source path.

use super::*;
use crate::contraction::{Edge, Tensor, TensorKind, TensorVertex};
use crate::network::FnPredicate;
use crate::Limits;
use std::collections::BTreeSet;
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

fn direct_packet_basis_map(
    graph: &Bipartite,
    packing: &Packing,
    kernel: &deformation::Cauchy,
    meter: &mut Meter,
) -> Result<Vec<Element>> {
    let incidences = graph
        .edges
        .len()
        .checked_mul(2)
        .ok_or_else(|| overflow("direct basis incidences"))?;
    let assignments = 1usize
        .checked_shl(u32::try_from(incidences).map_err(|_| overflow("direct basis shift"))?)
        .ok_or_else(|| overflow("direct basis assignments"))?;
    let mut out = Vec::with_capacity(assignments);
    for mask in 0..assignments {
        let left_bits: Vec<_> = (0..graph.edges.len())
            .map(|edge| mask & (1usize << edge) != 0)
            .collect();
        let right_bits: Vec<_> = (0..graph.edges.len())
            .map(|edge| mask & (1usize << (graph.edges.len() + edge)) != 0)
            .collect();
        let mut product = kernel.ring.one();
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
        out.push(product);
    }
    Ok(out)
}

fn mode_expanded_basis_map(
    graph: &Bipartite,
    packing: &Packing,
    parameters: &Parameters,
    coordinate: usize,
    kernel: &deformation::Cauchy,
    modes: &[deformation::Mode],
    meter: &mut Meter,
) -> Result<Vec<Element>> {
    let incidences = graph
        .edges
        .len()
        .checked_mul(2)
        .ok_or_else(|| overflow("mode basis incidences"))?;
    let assignments = 1usize
        .checked_shl(u32::try_from(incidences).map_err(|_| overflow("mode basis shift"))?)
        .ok_or_else(|| overflow("mode basis assignments"))?;
    let mut out = Vec::with_capacity(assignments);
    for mask in 0..assignments {
        let left_bits: Vec<_> = (0..graph.edges.len())
            .map(|edge| mask & (1usize << edge) != 0)
            .collect();
        let right_bits: Vec<_> = (0..graph.edges.len())
            .map(|edge| mask & (1usize << (graph.edges.len() + edge)) != 0)
            .collect();
        let mut product = kernel.ring.one();
        for (packet_id, packet) in packing.packets.iter().enumerate() {
            for (row, &row_edge) in packet
                .slots
                .iter()
                .enumerate()
                .filter_map(|(slot, edge)| edge.as_ref().map(|edge| (slot, edge)))
            {
                let row_group = packing::digit(
                    packing.row_labels[graph.edges[row_edge].left.0],
                    coordinate,
                    parameters.radix,
                );
                let mut row_sum = kernel.ring.zero();
                for (column, &column_edge) in packet
                    .slots
                    .iter()
                    .enumerate()
                    .filter_map(|(slot, edge)| edge.as_ref().map(|edge| (slot, edge)))
                {
                    let column_group = packing::digit(
                        packing.column_labels[graph.edges[column_edge].right.0],
                        coordinate,
                        parameters.radix,
                    );
                    if column_group == row_group && right_bits[column_edge] {
                        let weight = kernel.weight(row, column, meter)?;
                        row_sum = kernel.ring.add(&row_sum, &weight, meter)?;
                    }
                }
                for mode in modes
                    .iter()
                    .filter(|mode| mode.packet == packet_id && mode.target == row_group)
                {
                    let Some((_, row_coefficient)) =
                        mode.rows.iter().find(|&&(slot, _)| slot == row)
                    else {
                        continue;
                    };
                    let mut form = BigUint::zero();
                    for (column, coefficient) in &mode.columns {
                        let column_edge = packet.slots[*column]
                            .ok_or_else(|| integrity("mode column packet slot"))?;
                        if right_bits[column_edge] {
                            form = (form + coefficient) % kernel.ring.modulus();
                        }
                    }
                    let term =
                        kernel
                            .ring
                            .mul(row_coefficient, &kernel.ring.integer(&form), meter)?;
                    row_sum = kernel.ring.add(&row_sum, &term, meter)?;
                }
                let factor = if left_bits[row_edge] {
                    row_sum
                } else {
                    kernel.ring.sub(&kernel.ring.one(), &row_sum, meter)?
                };
                product = kernel.ring.mul(&product, &factor, meter)?;
            }
        }
        out.push(product);
    }
    Ok(out)
}

#[allow(clippy::too_many_arguments)]
fn child_basis_map(
    graph: &Bipartite,
    packing: &Packing,
    parameters: &Parameters,
    coordinate: usize,
    kernel: &deformation::Cauchy,
    modes: &[deformation::Mode],
    beta: &[usize],
    group: usize,
    meter: &mut Meter,
) -> Result<Vec<(usize, Element)>> {
    let child = deformation::child(
        graph, packing, coordinate, parameters, kernel, modes, beta, group, meter,
    )?;
    let mut selected = Vec::new();
    for (side, vertices, labels) in [
        (false, &graph.left, &packing.row_labels),
        (true, &graph.right, &packing.column_labels),
    ] {
        for vertex in 0..vertices.len() {
            if packing::digit(labels[vertex], coordinate, parameters.radix) == group {
                selected.push((side, vertex));
            }
        }
    }
    assert_eq!(selected.len(), child.left.len());
    let mut bit_positions = Vec::with_capacity(child.edges.len());
    let mut unique_positions = BTreeSet::new();
    for edge in &child.edges {
        let (side, vertex) = selected[edge.left.0];
        let old_edge = if side {
            graph.right[vertex].edges[edge.left.1]
        } else {
            graph.left[vertex].edges[edge.left.1]
        };
        let position = old_edge + usize::from(side) * graph.edges.len();
        assert!(unique_positions.insert(position));
        bit_positions.push(position);
    }
    let assignments = 1usize
        .checked_shl(u32::try_from(child.edges.len()).map_err(|_| overflow("child basis shift"))?)
        .ok_or_else(|| overflow("child basis assignments"))?;
    let mut out = Vec::with_capacity(assignments);
    for mask in 0..assignments {
        let mut global_mask = 0usize;
        for (edge, &position) in bit_positions.iter().enumerate() {
            if mask & (1usize << edge) != 0 {
                global_mask |= 1usize << position;
            }
        }
        let mut product = kernel.ring.one();
        for vertex in &child.right {
            let bits: Vec<_> = vertex
                .edges
                .iter()
                .map(|&edge| mask & (1usize << edge) != 0)
                .collect();
            let value = vertex.tensor.entry(&bits, &kernel.ring, meter)?;
            product = kernel.ring.mul(&product, &value, meter)?;
        }
        out.push((global_mask, product));
    }
    out.sort_by_key(|(mask, _)| *mask);
    assert!(out.windows(2).all(|pair| pair[0].0 != pair[1].0));
    Ok(out)
}

fn extracted_basis_map(
    graph: &Bipartite,
    packing: &Packing,
    parameters: &Parameters,
    coordinate: usize,
    kernel: &deformation::Cauchy,
    modes: &[deformation::Mode],
    meter: &mut Meter,
) -> Result<Vec<Element>> {
    let groups = packing.packets.len() * parameters.radix;
    let mut bounds = vec![0usize; groups];
    for (packet_id, packet) in packing.packets.iter().enumerate() {
        for &edge in packet.slots.iter().flatten() {
            let target = packing::digit(
                packing.row_labels[graph.edges[edge].left.0],
                coordinate,
                parameters.radix,
            );
            bounds[packet_id * parameters.radix + target] += 1;
        }
    }
    let assignments = 1usize
        .checked_shl(u32::try_from(2 * graph.edges.len()).map_err(|_| overflow("basis map shift"))?)
        .ok_or_else(|| overflow("basis map assignments"))?;
    let mut total = vec![kernel.ring.zero(); assignments];
    let mut usage = vec![0usize; groups];
    let mut beta = vec![0usize; modes.len()];
    let mut weight = 0usize;
    loop {
        let mut product_map = vec![(0usize, kernel.ring.one())];
        for group in 0..parameters.radix {
            let child_map = child_basis_map(
                graph, packing, parameters, coordinate, kernel, modes, &beta, group, meter,
            )?;
            let mut next = Vec::with_capacity(product_map.len() * child_map.len());
            for (left_mask, left_value) in &product_map {
                for (right_mask, right_value) in &child_map {
                    assert_eq!(left_mask & right_mask, 0);
                    next.push((
                        left_mask | right_mask,
                        kernel.ring.mul(left_value, right_value, meter)?,
                    ));
                }
            }
            product_map = next;
        }
        assert_eq!(product_map.len(), assignments);
        for (mask, value) in product_map {
            total[mask] = kernel.ring.add(&total[mask], &value, meter)?;
        }
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

fn delta_predicate(arity: usize, selected: usize) -> Arc<Tensor> {
    Arc::new(Tensor {
        creation_precision: 3,
        kind: TensorKind::Boolean(Arc::new(FnPredicate::new(arity, move |bits, _| {
            let index = bits
                .iter()
                .enumerate()
                .fold(0, |value, (port, bit)| value | (usize::from(*bit) << port));
            Ok(index == selected)
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
                    tensor: delta_predicate(edges.len(), assignment),
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
    forced_at_observed(
        graph,
        ring,
        rows,
        columns,
        layers,
        depth,
        parameters,
        visits,
        &mut |_, _, _, _, _, _| Ok(()),
        meter,
    )
}

#[allow(clippy::too_many_arguments)]
fn forced_at_observed(
    graph: &Bipartite,
    ring: &Ring,
    rows: Vec<usize>,
    columns: Vec<usize>,
    layers: usize,
    depth: usize,
    parameters: &Parameters,
    visits: &mut Vec<usize>,
    observe: &mut impl FnMut(usize, &Bipartite, &Ring, &Parameters, &Packing, &mut Meter) -> Result<()>,
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
    observe(depth, graph, ring, parameters, &packing, meter)?;
    active(
        graph,
        ring,
        parameters,
        &packing,
        false,
        usize::MAX,
        &mut |child, child_ring, work| {
            forced_at_observed(
                child,
                child_ring,
                vec![0; child.left.len()],
                vec![parameters.length - 1; child.right.len()],
                layers - 1,
                depth + 1,
                parameters,
                visits,
                observe,
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
fn direct_factorization_matches_four_edge_partition_orbits() {
    const PARTITIONS: [[usize; 4]; 15] = [
        [0, 0, 0, 0],
        [0, 0, 0, 1],
        [0, 0, 1, 0],
        [0, 0, 1, 1],
        [0, 0, 1, 2],
        [0, 1, 0, 0],
        [0, 1, 0, 1],
        [0, 1, 0, 2],
        [0, 1, 1, 0],
        [0, 1, 1, 1],
        [0, 1, 1, 2],
        [0, 1, 2, 0],
        [0, 1, 2, 1],
        [0, 1, 2, 2],
        [0, 1, 2, 3],
    ];
    const PERMUTATIONS: [[usize; 4]; 24] = [
        [0, 1, 2, 3],
        [0, 1, 3, 2],
        [0, 2, 1, 3],
        [0, 2, 3, 1],
        [0, 3, 1, 2],
        [0, 3, 2, 1],
        [1, 0, 2, 3],
        [1, 0, 3, 2],
        [1, 2, 0, 3],
        [1, 2, 3, 0],
        [1, 3, 0, 2],
        [1, 3, 2, 0],
        [2, 0, 1, 3],
        [2, 0, 3, 1],
        [2, 1, 0, 3],
        [2, 1, 3, 0],
        [2, 3, 0, 1],
        [2, 3, 1, 0],
        [3, 0, 1, 2],
        [3, 0, 2, 1],
        [3, 1, 0, 2],
        [3, 1, 2, 0],
        [3, 2, 0, 1],
        [3, 2, 1, 0],
    ];

    fn normalize(values: [usize; 4]) -> [usize; 4] {
        let mut names = [usize::MAX; 4];
        let mut next = 0usize;
        values.map(|value| {
            if names[value] == usize::MAX {
                names[value] = next;
                next += 1;
            }
            names[value]
        })
    }

    fn canonical(left: [usize; 4], right: [usize; 4]) -> ([usize; 4], [usize; 4]) {
        PERMUTATIONS
            .iter()
            .map(|permutation| {
                (
                    normalize(permutation.map(|edge| left[edge])),
                    normalize(permutation.map(|edge| right[edge])),
                )
            })
            .min()
            .unwrap()
    }

    let representatives: BTreeSet<_> = PARTITIONS
        .iter()
        .flat_map(|&left| PARTITIONS.iter().map(move |&right| canonical(left, right)))
        .collect();
    assert_eq!(representatives.len(), 33);

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
    let z = ring.point(&3u8.into()).unwrap();
    let t = 1i64;
    let coordinate = deformation::valuation(t);
    let kernel = deformation::Cauchy::new(&parameters, t, &z, &ring, &mut meter).unwrap();
    let mut cases = 0usize;
    for (left_groups, right_groups) in representatives {
        for schedule in 0..4 {
            let prototype = partition_basis_graph(&left_groups, &right_groups, 0);
            let rows = refinement_labels(prototype.left.len(), schedule, 0);
            let columns = refinement_labels(prototype.right.len(), schedule, 1);
            let packing =
                packing::pack_labels(&prototype, &parameters, rows, columns, false, &mut meter)
                    .unwrap();
            let modes = deformation::modes(
                &prototype,
                &packing,
                coordinate,
                &parameters,
                &kernel,
                &mut meter,
            )
            .unwrap();
            let expected =
                direct_packet_basis_map(&prototype, &packing, &kernel, &mut meter).unwrap();
            let expanded = mode_expanded_basis_map(
                &prototype,
                &packing,
                &parameters,
                coordinate,
                &kernel,
                &modes,
                &mut meter,
            )
            .unwrap();
            let actual = extracted_basis_map(
                &prototype,
                &packing,
                &parameters,
                coordinate,
                &kernel,
                &modes,
                &mut meter,
            )
            .unwrap();
            assert_eq!(
                actual, expected,
                "left={left_groups:?}, right={right_groups:?}, schedule={schedule}",
            );
            assert_eq!(
                expanded, expected,
                "mode expansion: left={left_groups:?}, right={right_groups:?}, schedule={schedule}",
            );
            cases += expected.len();
        }
    }
    assert_eq!(cases, 33_792);
}

fn dynamic_set_partitions(edges: usize) -> Vec<Vec<usize>> {
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

fn dynamic_edge_permutations(edges: usize) -> Vec<Vec<usize>> {
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

fn dynamic_normalize_partition(values: impl Iterator<Item = usize>, edges: usize) -> Vec<usize> {
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

fn dynamic_partition_pair_orbits(edges: usize) -> BTreeSet<(Vec<usize>, Vec<usize>)> {
    let partitions = dynamic_set_partitions(edges);
    let permutations = dynamic_edge_permutations(edges);
    let mut out = BTreeSet::new();
    for left in &partitions {
        for right in &partitions {
            let canonical = permutations
                .iter()
                .map(|permutation| {
                    (
                        dynamic_normalize_partition(
                            permutation.iter().map(|&edge| left[edge]),
                            edges,
                        ),
                        dynamic_normalize_partition(
                            permutation.iter().map(|&edge| right[edge]),
                            edges,
                        ),
                    )
                })
                .min()
                .unwrap();
            out.insert(canonical);
        }
    }
    out
}

#[test]
fn direct_factorization_matches_five_edge_partition_orbits() {
    let representatives = dynamic_partition_pair_orbits(5);
    assert_eq!(representatives.len(), 91);
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
    let z = ring.point(&3u8.into()).unwrap();
    let t = 1i64;
    let coordinate = deformation::valuation(t);
    let kernel = deformation::Cauchy::new(&parameters, t, &z, &ring, &mut meter).unwrap();
    let mut cases = 0usize;
    for (left_groups, right_groups) in representatives {
        for schedule in 0..4 {
            let prototype = partition_basis_graph(&left_groups, &right_groups, 0);
            let rows = refinement_labels(prototype.left.len(), schedule, 0);
            let columns = refinement_labels(prototype.right.len(), schedule, 1);
            let packing =
                packing::pack_labels(&prototype, &parameters, rows, columns, false, &mut meter)
                    .unwrap();
            let modes = deformation::modes(
                &prototype,
                &packing,
                coordinate,
                &parameters,
                &kernel,
                &mut meter,
            )
            .unwrap();
            let expected =
                direct_packet_basis_map(&prototype, &packing, &kernel, &mut meter).unwrap();
            let expanded = mode_expanded_basis_map(
                &prototype,
                &packing,
                &parameters,
                coordinate,
                &kernel,
                &modes,
                &mut meter,
            )
            .unwrap();
            let actual = extracted_basis_map(
                &prototype,
                &packing,
                &parameters,
                coordinate,
                &kernel,
                &modes,
                &mut meter,
            )
            .unwrap();
            assert_eq!(
                actual, expected,
                "left={left_groups:?}, right={right_groups:?}, schedule={schedule}",
            );
            assert_eq!(
                expanded, expected,
                "mode expansion: left={left_groups:?}, right={right_groups:?}, schedule={schedule}",
            );
            cases += expected.len();
        }
    }
    assert_eq!(cases, 372_736);
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
    let mut captures = Vec::new();
    let actual = forced_at_observed(
        &graph,
        &ring,
        vec![0],
        vec![1],
        3,
        0,
        &parameters,
        &mut visits,
        &mut |depth, node, node_ring, node_parameters, packing, work| {
            let (bytes, envelope) =
                super::factorization_certificate::capture_executed_source_checked(
                    node,
                    packing,
                    node_parameters,
                    0,
                    node_ring,
                    work,
                )?;
            captures.push((
                depth,
                node.edges.len(),
                node_ring.precision(),
                bytes,
                envelope.modes,
                envelope.tuples,
                envelope.tuple_log2_bound,
            ));
            Ok(())
        },
        &mut meter,
    )
    .unwrap();
    assert_eq!(actual, expected);
    assert!(visits.get(3).copied().unwrap_or(0) > 0, "visits={visits:?}");
    let capture_counts: Vec<_> = (0..3)
        .map(|depth| captures.iter().filter(|capture| capture.0 == depth).count())
        .collect();
    let capture_bytes: Vec<usize> = (0..3)
        .map(|depth| {
            captures
                .iter()
                .filter(|capture| capture.0 == depth)
                .map(|capture| capture.3)
                .sum()
        })
        .collect();
    let unique: BTreeSet<_> = captures.iter().cloned().collect();
    assert_eq!(visits, [1, 20, 412, 9_080]);
    assert_eq!(capture_counts, visits[..3]);
    assert_eq!(capture_bytes, [641, 15_500, 374_096]);
    assert_eq!(captures.len(), 433);
    assert_eq!(capture_bytes.iter().sum::<usize>(), 390_237);
    assert_eq!(
        unique.into_iter().collect::<Vec<_>>(),
        [
            (0, 1, 3, 641, 1, 2, 15),
            (1, 1, 67, 775, 1, 2, 60),
            (2, 1, 131, 908, 1, 2, 85),
        ]
    );
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
