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
#[ignore = "107-second exhaustive conditional-expectation threshold witness"]
fn first_guard_compatible_auxiliary_child_accepts_next_scale() {
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
    let child = packet_child(&root, &root_packing, &root_parameters, 0, &ring, &mut meter);
    let child_packing = packing::pack(
        &child,
        &child_parameters,
        maximum_degree(&child),
        &mut meter,
    )
    .unwrap();
    assert_eq!(child.edges.len(), 512);
    assert_eq!(maximum_degree(&child), 32);
    assert_eq!(child_packing.packets.len(), 132);
    let grandchild = packet_child(
        &child,
        &child_packing,
        &child_parameters,
        0,
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
    assert_eq!(grandchild.edges.len(), 512);
    assert_eq!(maximum_degree(&grandchild), 32);
    let shape = packing_shape(&grandchild, &grandchild_packing, &grandchild_parameters);
    assert_eq!(shape.maximum_class, 10);
    assert_eq!(shape.padded_class_mass, 1_024);
    assert_eq!(shape.packets, 74);
    assert_eq!(shape.maximum_group_load, 261);
    assert!(shape.padded_class_mass <= 3 * grandchild.edges.len());
    assert!(shape.packets * grandchild_parameters.length <= 3 * grandchild.edges.len());
    assert!(shape.maximum_group_load * grandchild_parameters.radix <= 3 * grandchild.edges.len());
}
