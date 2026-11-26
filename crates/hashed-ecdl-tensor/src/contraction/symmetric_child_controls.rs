//! Small exact controls for the packet-child shape of the symmetric source family.
//! These finite radix-two models are diagnostics, not asymptotic closure proofs.

use super::*;
use crate::contraction::{Edge, Tensor, TensorKind, TensorVertex};
use crate::network::FnPredicate;
use crate::Limits;
use std::sync::Arc;

fn complete_multigraph(vertices: usize, multiplicity: usize, precision: usize) -> Bipartite {
    let mut left_ports = vec![Vec::new(); vertices];
    let mut right_ports = vec![Vec::new(); vertices];
    let mut edges = Vec::with_capacity(vertices * vertices * multiplicity);
    for left in 0..vertices {
        for right in 0..vertices {
            for _ in 0..multiplicity {
                let edge = edges.len();
                let left_port = left_ports[left].len();
                let right_port = right_ports[right].len();
                left_ports[left].push(edge);
                right_ports[right].push(edge);
                edges.push(Edge {
                    left: (left, left_port),
                    right: (right, right_port),
                });
            }
        }
    }
    let make_vertices = |ports: Vec<Vec<usize>>| {
        ports
            .into_iter()
            .map(|edges| TensorVertex {
                tensor: Arc::new(Tensor {
                    creation_precision: precision,
                    kind: TensorKind::Boolean(Arc::new(FnPredicate::new(edges.len(), |_, _| {
                        Ok(true)
                    }))),
                }),
                edges,
            })
            .collect()
    };
    let graph = Bipartite {
        left: make_vertices(left_ports),
        right: make_vertices(right_ports),
        edges,
    };
    graph.validate().unwrap();
    graph
}

fn maximum_degree(graph: &Bipartite) -> usize {
    graph
        .left
        .iter()
        .chain(&graph.right)
        .map(|vertex| vertex.edges.len())
        .max()
        .unwrap_or(0)
}

#[derive(Debug, PartialEq, Eq)]
struct PackingShape {
    maximum_class: usize,
    padded_class_mass: usize,
    packets: usize,
    maximum_group_load: usize,
}

fn packing_shape(graph: &Bipartite, packing: &Packing, parameters: &Parameters) -> PackingShape {
    let labels = parameters.radix.pow(parameters.a as u32);
    let mut classes = vec![0usize; parameters.array];
    for edge in &graph.edges {
        classes[packing.row_labels[edge.left.0] * labels + packing.column_labels[edge.right.0]] +=
            1;
    }
    let maximum_class = classes.into_iter().max().unwrap_or(0);
    let padded_class_mass =
        maximum_class.div_ceil(parameters.length) * parameters.length * parameters.array;
    let mut maximum_group_load = 0;
    for coordinate in 0..parameters.a {
        for (vertices, labels) in [
            (&graph.left, &packing.row_labels),
            (&graph.right, &packing.column_labels),
        ] {
            let mut loads = vec![0usize; parameters.radix];
            for (vertex, &label) in vertices.iter().zip(labels) {
                loads[packing::digit(label, coordinate, parameters.radix)] += vertex.edges.len();
            }
            maximum_group_load = maximum_group_load.max(loads.into_iter().max().unwrap_or(0));
        }
    }
    PackingShape {
        maximum_class,
        padded_class_mass,
        packets: packing.packets.len(),
        maximum_group_load,
    }
}

fn packet_child(
    graph: &Bipartite,
    packing: &Packing,
    parameters: &Parameters,
    group: usize,
    ring: &Ring,
    meter: &mut Meter,
) -> Bipartite {
    let kernel = Cauchy::new(parameters, 1, &ring.one(), ring, meter).unwrap();
    let modes = deformation::modes(graph, packing, 0, parameters, &kernel, meter).unwrap();
    let beta = vec![0; modes.len()];
    deformation::child(
        graph, packing, 0, parameters, &kernel, &modes, &beta, group, meter,
    )
    .unwrap()
}

#[test]
fn under_scale_auxiliary_child_exposes_packet_padding_boundary() {
    let mut meter = Meter::new(Limits {
        steps: 1_000_000_000,
        ..Limits::default()
    });
    let precision = 7;
    let ring = Ring::new(4, precision, &mut meter).unwrap();
    let root_parameters = Parameters {
        radix: 2,
        u: 1,
        a: 1,
        length: 2,
        array: 4,
    };
    let child_parameters = Parameters {
        radix: 2,
        u: 1,
        a: 2,
        length: 4,
        array: 16,
    };
    let grandchild_parameters = Parameters {
        radix: 2,
        u: 1,
        a: 3,
        length: 8,
        array: 64,
    };
    let root = complete_multigraph(4, root_parameters.length, precision);
    let root_packing =
        packing::pack(&root, &root_parameters, maximum_degree(&root), &mut meter).unwrap();
    assert_eq!(root_packing.packets.len(), 16);
    let child = packet_child(&root, &root_packing, &root_parameters, 0, &ring, &mut meter);
    let child_packing = packing::pack(
        &child,
        &child_parameters,
        maximum_degree(&child),
        &mut meter,
    )
    .unwrap();
    assert_eq!(child.edges.len(), 32);
    assert_eq!(child_packing.packets.len(), 10);
    let grandchild = packet_child(
        &child,
        &child_packing,
        &child_parameters,
        0,
        &ring,
        &mut meter,
    );
    assert_eq!(grandchild.edges.len(), 32);
    assert_eq!(maximum_degree(&grandchild), 8);
    let error = packing::pack(
        &grandchild,
        &grandchild_parameters,
        maximum_degree(&grandchild),
        &mut meter,
    )
    .err()
    .expect("under-scale packing must be rejected");
    assert_eq!(
        error,
        crate::Error::Integrity("packet padding bound".into())
    );
    let labels = grandchild_parameters
        .radix
        .pow(grandchild_parameters.a as u32);
    let (rows, columns) = packing::conditional_labels(&grandchild, labels, &mut meter).unwrap();
    let forced = packing::pack_labels(
        &grandchild,
        &grandchild_parameters,
        rows,
        columns,
        false,
        &mut meter,
    )
    .unwrap();
    let shape = packing_shape(&grandchild, &forced, &grandchild_parameters);
    assert_eq!(shape.maximum_class, 2);
    assert_eq!(shape.padded_class_mass, 512);
    assert_eq!(shape.packets, 16);
    assert_eq!(shape.maximum_group_load, 28);
    assert!(shape.padded_class_mass > 3 * grandchild.edges.len());
}

#[test]
fn first_guard_boundary_auxiliary_child_paths_accept_next_scale() {
    let mut meter = Meter::new(Limits {
        steps: 1_000_000_000,
        ..Limits::default()
    });
    let precision = 7;
    let ring = Ring::new(4, precision, &mut meter).unwrap();
    let root_parameters = Parameters {
        radix: 2,
        u: 1,
        a: 1,
        length: 2,
        array: 4,
    };
    let child_parameters = Parameters {
        radix: 2,
        u: 1,
        a: 2,
        length: 4,
        array: 16,
    };
    let grandchild_parameters = Parameters {
        radix: 2,
        u: 1,
        a: 3,
        length: 8,
        array: 64,
    };
    let root = complete_multigraph(16, root_parameters.length, precision);
    assert_eq!(
        root.edges.len(),
        grandchild_parameters.length * grandchild_parameters.array
    );
    let root_packing =
        packing::pack(&root, &root_parameters, maximum_degree(&root), &mut meter).unwrap();
    assert_eq!(root_packing.packets.len(), 256);
    let mut observed = Vec::new();
    for group in 0..root_parameters.radix {
        let child = packet_child(
            &root,
            &root_packing,
            &root_parameters,
            group,
            &ring,
            &mut meter,
        );
        let child_packing = packing::pack(
            &child,
            &child_parameters,
            maximum_degree(&child),
            &mut meter,
        )
        .unwrap();
        assert_eq!(child.edges.len(), 512);
        assert_eq!(maximum_degree(&child), 32);
        for next_group in 0..child_parameters.radix {
            let grandchild = packet_child(
                &child,
                &child_packing,
                &child_parameters,
                next_group,
                &ring,
                &mut meter,
            );
            let grandchild_packing = packing::pack(
                &grandchild,
                &grandchild_parameters,
                maximum_degree(&grandchild),
                &mut meter,
            )
            .unwrap();
            observed.push((
                group,
                next_group,
                child_packing.packets.len(),
                grandchild.edges.len(),
                maximum_degree(&grandchild),
                packing_shape(&grandchild, &grandchild_packing, &grandchild_parameters),
            ));
        }
    }
    assert_eq!(
        observed,
        [
            (
                0,
                0,
                132,
                512,
                32,
                PackingShape {
                    maximum_class: 10,
                    padded_class_mass: 1_024,
                    packets: 74,
                    maximum_group_load: 261,
                },
            ),
            (
                0,
                1,
                132,
                512,
                32,
                PackingShape {
                    maximum_class: 10,
                    padded_class_mass: 1_024,
                    packets: 78,
                    maximum_group_load: 261,
                },
            ),
            (
                1,
                0,
                134,
                516,
                32,
                PackingShape {
                    maximum_class: 10,
                    padded_class_mass: 1_024,
                    packets: 77,
                    maximum_group_load: 260,
                },
            ),
            (
                1,
                1,
                134,
                508,
                32,
                PackingShape {
                    maximum_class: 10,
                    padded_class_mass: 1_024,
                    packets: 77,
                    maximum_group_load: 262,
                },
            ),
        ]
    );
    assert_eq!(
        observed
            .iter()
            .filter(|(_, _, _, edges, _, _)| {
                *edges >= grandchild_parameters.length * grandchild_parameters.array
            })
            .count(),
        3
    );
    for (_, _, _, edges, _, shape) in observed {
        assert!(shape.padded_class_mass <= 3 * edges);
        assert!(shape.packets * grandchild_parameters.length <= 3 * edges);
        assert!(shape.maximum_group_load * grandchild_parameters.radix <= 3 * edges);
    }
}

#[test]
fn four_level_auxiliary_paths_reach_next_length_array_boundary() {
    let mut meter = Meter::new(Limits {
        steps: 10_000_000_000,
        vector: 10_000_000,
        ..Limits::default()
    });
    let precision = 7;
    let ring = Ring::new(4, precision, &mut meter).unwrap();
    let parameters = [
        Parameters {
            radix: 2,
            u: 1,
            a: 1,
            length: 2,
            array: 4,
        },
        Parameters {
            radix: 2,
            u: 1,
            a: 2,
            length: 4,
            array: 16,
        },
        Parameters {
            radix: 2,
            u: 1,
            a: 3,
            length: 8,
            array: 64,
        },
        Parameters {
            radix: 2,
            u: 1,
            a: 4,
            length: 16,
            array: 256,
        },
        Parameters {
            radix: 2,
            u: 1,
            a: 5,
            length: 32,
            array: 1_024,
        },
    ];
    let root = complete_multigraph(128, parameters[0].length, precision);
    assert_eq!(root.edges.len(), 32_768);
    let root_packing =
        packing::pack(&root, &parameters[0], maximum_degree(&root), &mut meter).unwrap();
    let mut frontier = vec![(0usize, root, root_packing)];
    for depth in 0..4 {
        let mut next = Vec::new();
        for (path, graph, packing) in frontier {
            for group in 0..parameters[depth].radix {
                let child = packet_child(
                    &graph,
                    &packing,
                    &parameters[depth],
                    group,
                    &ring,
                    &mut meter,
                );
                let child_packing = packing::pack(
                    &child,
                    &parameters[depth + 1],
                    maximum_degree(&child),
                    &mut meter,
                )
                .unwrap();
                next.push(((path << 1) | group, child, child_packing));
            }
        }
        frontier = next;
    }
    assert_eq!(frontier.len(), 16);
    let observed: Vec<_> = frontier
        .into_iter()
        .map(|(path, graph, packing)| {
            (
                path,
                graph.edges.len(),
                maximum_degree(&graph),
                packing_shape(&graph, &packing, &parameters[4]),
            )
        })
        .collect();
    assert_eq!(
        observed.iter().map(|entry| entry.0).collect::<Vec<_>>(),
        (0..16).collect::<Vec<_>>()
    );
    assert_eq!(
        observed.iter().map(|entry| entry.1).collect::<Vec<_>>(),
        vec![
            32_759, 32_767, 32_786, 32_760, 32_768, 32_764, 32_774, 32_766, 32_773, 32_755, 32_779,
            32_765, 32_772, 32_766, 32_758, 32_776,
        ]
    );
    assert!(observed.iter().all(|entry| entry.2 == 256));
    assert_eq!(
        observed
            .iter()
            .map(|entry| entry.3.maximum_class)
            .collect::<Vec<_>>(),
        vec![36, 40, 37, 38, 40, 36, 36, 37, 36, 36, 37, 38, 39, 38, 37, 37]
    );
    assert!(observed
        .iter()
        .all(|entry| entry.3.padded_class_mass == 65_536));
    assert_eq!(
        observed
            .iter()
            .map(|entry| entry.3.packets)
            .collect::<Vec<_>>(),
        vec![
            1_205, 1_204, 1_188, 1_214, 1_200, 1_206, 1_207, 1_205, 1_206, 1_192, 1_195, 1_190,
            1_202, 1_201, 1_198, 1_204,
        ]
    );
    assert_eq!(
        observed
            .iter()
            .map(|entry| entry.3.maximum_group_load)
            .collect::<Vec<_>>(),
        vec![
            16_414, 16_417, 16_438, 16_455, 16_447, 16_463, 16_422, 16_433, 16_431, 16_430, 16_436,
            16_433, 16_430, 16_419, 16_406, 16_420,
        ]
    );
    let length_array = parameters[4].length * parameters[4].array;
    assert_eq!(
        observed
            .iter()
            .filter(|entry| entry.1 >= length_array)
            .count(),
        7
    );
    for (_, edges, _, shape) in observed {
        assert!(shape.maximum_class * parameters[4].array <= 2 * edges);
        assert!(shape.padded_class_mass <= 3 * edges);
        assert!(shape.packets * parameters[4].length <= 3 * edges);
        assert!(shape.maximum_group_load * parameters[4].radix <= 3 * edges);
    }
}
