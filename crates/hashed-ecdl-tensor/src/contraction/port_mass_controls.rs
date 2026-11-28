//! Source-object controls for endpoint mass and replicated packet vertices.
//! These forced component geometries do not assert production Setup admission.

use super::*;
use crate::contraction::{Edge, Tensor, TensorKind, TensorVertex};
use crate::network::FnPredicate;
use crate::Limits;
use serde_json::json;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::sync::Arc;

fn graph(m: usize, family: usize) -> Bipartite {
    let (nl, nr) = match family {
        0 => (m + 1, m + 1),
        1 => (2, 2),
        2 => (2, m + 1),
        3 => (m + 1, 2),
        4 => (4, 3),
        _ => unreachable!(),
    };
    let mut left_ports = vec![Vec::new(); nl];
    let mut right_ports = vec![Vec::new(); nr];
    let mut edges = Vec::new();
    for e in 0..m {
        let (l, r) = match family {
            0 => (e, e),
            1 => (0, 0),
            2 => (0, e),
            3 => (e, 0),
            4 => (e % 3, e % 2),
            _ => unreachable!(),
        };
        let lp = left_ports[l].len();
        let rp = right_ports[r].len();
        left_ports[l].push(e);
        right_ports[r].push(e);
        edges.push(Edge {
            left: (l, lp),
            right: (r, rp),
        });
    }
    let vertices = |ports: Vec<Vec<usize>>| {
        ports
            .into_iter()
            .map(|edges| TensorVertex {
                tensor: Arc::new(Tensor {
                    creation_precision: 3,
                    kind: TensorKind::Boolean(Arc::new(FnPredicate::new(edges.len(), |_, _| {
                        Ok(true)
                    }))),
                }),
                edges,
            })
            .collect()
    };
    let out = Bipartite {
        left: vertices(left_ports),
        right: vertices(right_ports),
        edges,
    };
    out.validate().unwrap();
    out
}

fn labels(n: usize, alphabet: usize, schedule: usize, side: usize) -> Vec<usize> {
    (0..n)
        .map(|v| match schedule {
            0 => 0,
            1 => (v + side) % alphabet,
            2 => (7 * v + 3 * side + 1) % alphabet,
            3 => alphabet - 1 - (v + side) % alphabet,
            _ => unreachable!(),
        })
        .collect()
}

fn tuple_states(
    graph: &Bipartite,
    packing: &Packing,
    p: &Parameters,
    coordinate: usize,
    modes: &[Mode],
    precision: usize,
    meter: &mut Meter,
) -> Vec<Vec<usize>> {
    let mut bounds = vec![0usize; packing.packets.len() * p.radix];
    for (packet_id, packet) in packing.packets.iter().enumerate() {
        for &e in packet.slots.iter().flatten() {
            let target = packing::digit(
                packing.row_labels[graph.edges[e].left.0],
                coordinate,
                p.radix,
            );
            bounds[packet_id * p.radix + target] += 1;
        }
    }
    let mut states = BTreeSet::new();
    let mut beta = vec![0; modes.len()];
    states.insert(beta.clone());
    if let Some(first) = modes.first() {
        let group = first.packet * p.radix + first.target;
        let cap = ((precision - 1) / first.weight).min(bounds[group]);
        assert!(cap > 0, "a retained first mode must have a real target row");
        let mut usage = vec![0; bounds.len()];
        let mut weight = 0;
        for exponent in 1..=cap {
            assert!(next_tuple(
                &mut beta,
                modes,
                &mut usage,
                &bounds,
                &mut weight,
                precision,
                p.radix,
                meter
            )
            .unwrap());
            assert_eq!(beta[0], exponent);
            assert!(beta.iter().skip(1).all(|&e| e == 0));
            if exponent == 1 || exponent == cap {
                states.insert(beta.clone());
            }
        }
    }
    for beta in &states {
        let mut usage = vec![0; bounds.len()];
        let mut weight = 0;
        for (mode, &exponent) in modes.iter().zip(beta) {
            weight += exponent * mode.weight;
            usage[mode.packet * p.radix + mode.target] += exponent;
        }
        assert!(weight < precision);
        assert!(usage.iter().zip(&bounds).all(|(n, bound)| n <= bound));
    }
    states.into_iter().collect()
}

fn check_mode_placement(
    graph: &Bipartite,
    packing: &Packing,
    p: &Parameters,
    coordinate: usize,
    modes: &[Mode],
    precision: usize,
) -> usize {
    for mode in modes {
        assert!(mode.packet < packing.packets.len());
        assert!(mode.source < p.radix);
        assert!(mode.target < p.radix);
        assert!((1..precision).contains(&mode.weight));
        assert!(!mode.rows.is_empty());
        assert!(!mode.columns.is_empty());
        let packet = &packing.packets[mode.packet];
        for &(slot, ref coefficient) in &mode.rows {
            assert!(!coefficient.is_zero());
            let edge = packet.slots[slot].expect("mode row must name a packet edge");
            assert_eq!(
                packing::digit(
                    packing.row_labels[graph.edges[edge].left.0],
                    coordinate,
                    p.radix,
                ),
                mode.target
            );
        }
        for &(slot, ref coefficient) in &mode.columns {
            assert!(!coefficient.is_zero());
            let edge = packet.slots[slot].expect("mode column must name a packet edge");
            assert_eq!(
                packing::digit(
                    packing.column_labels[graph.edges[edge].right.0],
                    coordinate,
                    p.radix,
                ),
                mode.source
            );
        }
    }
    modes.len()
}

struct Observation {
    edges: Vec<usize>,
    old_vertices: usize,
    old_isolates: usize,
    packet_vertices: usize,
    isolated_packets: usize,
    maximum_degree: usize,
}

fn observe(
    graph: &Bipartite,
    packing: &Packing,
    p: &Parameters,
    coordinate: usize,
    kernel: &Cauchy,
    modes: &[Mode],
    beta: &[usize],
    meter: &mut Meter,
) -> Observation {
    // Independent edge-endpoint and vertex-degree histograms.
    let mut endpoint_histogram = vec![0usize; p.radix];
    let mut row_only = endpoint_histogram.clone();
    for edge in &graph.edges {
        let l = packing::digit(packing.row_labels[edge.left.0], coordinate, p.radix);
        let r = packing::digit(packing.column_labels[edge.right.0], coordinate, p.radix);
        endpoint_histogram[l] += 1;
        endpoint_histogram[r] += 1;
        row_only[l] += 1;
    }
    let mut degree_histogram = vec![0usize; p.radix];
    for (vertices, labels) in [
        (&graph.left, &packing.row_labels),
        (&graph.right, &packing.column_labels),
    ] {
        for (v, vertex) in vertices.iter().enumerate() {
            degree_histogram[packing::digit(labels[v], coordinate, p.radix)] += vertex.edges.len();
        }
    }
    assert_eq!(endpoint_histogram, degree_histogram);
    if !graph.edges.is_empty() {
        assert_ne!(
            row_only, endpoint_histogram,
            "wrong row-only model accepted"
        );
        assert_ne!(
            graph.edges.len(),
            endpoint_histogram.iter().sum::<usize>(),
            "wrong edge-once model accepted"
        );
    }

    let mut packet_of = vec![usize::MAX; graph.edges.len()];
    for (packet_id, packet) in packing.packets.iter().enumerate() {
        for &e in packet.slots.iter().flatten() {
            assert_eq!(packet_of[e], usize::MAX);
            packet_of[e] = packet_id;
        }
    }
    assert!(packet_of.iter().all(|&v| v != usize::MAX));
    let mut out = Observation {
        edges: Vec::new(),
        old_vertices: 0,
        old_isolates: 0,
        packet_vertices: 0,
        isolated_packets: 0,
        maximum_degree: 0,
    };
    let mut prefix = 0usize;
    for (group, &expected_edges) in endpoint_histogram.iter().enumerate() {
        let child = deformation::child(
            graph, packing, coordinate, p, kernel, modes, beta, group, meter,
        )
        .unwrap();
        assert_eq!(child.edges.len(), expected_edges);
        assert_eq!(child.right.len(), packing.packets.len());
        let mut selected = Vec::new();
        let mut row_position = vec![None; graph.left.len()];
        let mut column_position = vec![None; graph.right.len()];
        for (vertices, labels, positions) in [
            (&graph.left, &packing.row_labels, &mut row_position),
            (&graph.right, &packing.column_labels, &mut column_position),
        ] {
            for (v, vertex) in vertices.iter().enumerate() {
                if packing::digit(labels[v], coordinate, p.radix) == group {
                    positions[v] = Some(selected.len());
                    selected.push(vertex);
                }
            }
        }
        assert_eq!(selected.len(), child.left.len());
        for (old, retained) in selected.iter().zip(&child.left) {
            assert!(Arc::ptr_eq(&old.tensor, &retained.tensor));
            assert_eq!(retained.tensor.creation_precision, 3);
            assert_eq!(old.edges.len(), retained.edges.len());
        }
        for packet in &child.right {
            assert_eq!(packet.tensor.creation_precision, kernel.ring.precision());
        }
        // Check endpoint identity and packet ownership, including parallel ports.
        let mut expected = Vec::new();
        for (e, edge) in graph.edges.iter().enumerate() {
            if let Some(v) = row_position[edge.left.0] {
                expected.push((v, edge.left.1, packet_of[e]));
            }
            if let Some(v) = column_position[edge.right.0] {
                expected.push((v, edge.right.1, packet_of[e]));
            }
        }
        let mut actual: Vec<_> = child
            .edges
            .iter()
            .map(|e| (e.left.0, e.left.1, e.right.0))
            .collect();
        expected.sort_unstable();
        actual.sort_unstable();
        assert_eq!(expected, actual);
        prefix += child.edges.len();
        assert!(prefix <= 2 * graph.edges.len());
        out.old_vertices += child.left.len();
        out.old_isolates += child.left.iter().filter(|v| v.edges.is_empty()).count();
        out.packet_vertices += child.right.len();
        out.isolated_packets += child.right.iter().filter(|v| v.edges.is_empty()).count();
        out.maximum_degree = out.maximum_degree.max(
            child
                .left
                .iter()
                .chain(&child.right)
                .map(|v| v.edges.len())
                .max()
                .unwrap_or(0),
        );
        out.edges.push(child.edges.len());
    }
    assert_eq!(out.edges.iter().sum::<usize>(), 2 * graph.edges.len());
    assert_eq!(
        out.edges
            .iter()
            .map(|n| n * kernel.ring.precision())
            .sum::<usize>(),
        2 * graph.edges.len() * kernel.ring.precision()
    );
    assert_eq!(out.old_vertices, graph.left.len() + graph.right.len());
    assert_eq!(
        out.old_isolates,
        graph
            .left
            .iter()
            .chain(&graph.right)
            .filter(|v| v.edges.is_empty())
            .count()
    );
    assert_eq!(out.packet_vertices, p.radix * packing.packets.len());
    out
}

fn emit(writer: &mut Option<BufWriter<std::fs::File>>, row: serde_json::Value) {
    if let Some(file) = writer {
        serde_json::to_writer(&mut *file, &row).unwrap();
        writeln!(file).unwrap();
    }
}

fn coverage_error(error: &crate::Error, expected: &str) -> bool {
    matches!(error, crate::Error::Integrity(message) if message == expected)
}

#[test]
fn child_port_mass_source_controls() {
    let mut writer = std::env::var_os("PORT_MASS_ROWS").map(|path| {
        BufWriter::new(
            OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(path)
                .unwrap(),
        )
    });
    let mut meter = Meter::new(Limits::default());
    let ring = Ring::new(4, 7, &mut meter).unwrap();
    let mut contexts = 0usize;
    let mut tuples = 0usize;
    let mut base_group_calls = 0usize;
    let mut nonempty_tuple_controls = 0usize;
    let mut mode_placement_checks = 0usize;
    for (radix, a) in [
        (2usize, 1usize),
        (2, 2),
        (2, 3),
        (3, 1),
        (3, 2),
        (4, 1),
        (4, 2),
        (64, 1),
    ] {
        let p = Parameters {
            radix,
            a,
            u: a,
            length: 1 << a,
            array: radix.pow((2 * a) as u32),
        };
        let alphabet = radix.pow(a as u32);
        for m in [0usize, 1, 2, 3, 4, 8, 12, 16] {
            for family in 0..5 {
                let graph = graph(m, family);
                for schedule in 0..4 {
                    let packing = packing::pack_labels(
                        &graph,
                        &p,
                        labels(graph.left.len(), alphabet, schedule, 0),
                        labels(graph.right.len(), alphabet, schedule, 1),
                        false,
                        &mut meter,
                    )
                    .unwrap();
                    for coordinate in 0..a {
                        let t = 1i64 << coordinate;
                        let kernel = Cauchy::new(&p, t, &ring.one(), &ring, &mut meter).unwrap();
                        let modes = deformation::modes(
                            &graph, &packing, coordinate, &p, &kernel, &mut meter,
                        )
                        .unwrap();
                        mode_placement_checks += check_mode_placement(
                            &graph,
                            &packing,
                            &p,
                            coordinate,
                            &modes,
                            ring.precision(),
                        );
                        let states = tuple_states(
                            &graph,
                            &packing,
                            &p,
                            coordinate,
                            &modes,
                            ring.precision(),
                            &mut meter,
                        );
                        let mut first_histogram = None;
                        for beta in states {
                            let observed = observe(
                                &graph, &packing, &p, coordinate, &kernel, &modes, &beta,
                                &mut meter,
                            );
                            if let Some(first) = &first_histogram {
                                assert_eq!(first, &observed.edges);
                            } else {
                                first_histogram = Some(observed.edges.clone());
                            }
                            let active_beta: Vec<_> = beta
                                .iter()
                                .enumerate()
                                .filter(|(_, e)| **e != 0)
                                .map(|(i, e)| (i, *e))
                                .collect();
                            emit(
                                &mut writer,
                                json!({"kind":"base_tuple","radix":radix,"a":a,
                                "coordinate":coordinate,"M":m,"family":family,"schedule":schedule,
                                "modes":modes.len(),"active_beta":active_beta,"group_edges":observed.edges,
                                "old_vertices":observed.old_vertices,"old_isolates":observed.old_isolates,
                                "packet_vertices":observed.packet_vertices,
                                "total_vertices":observed.old_vertices+observed.packet_vertices,
                                "isolated_packets":observed.isolated_packets,"precision":7,"checks_passed":true}),
                            );
                            tuples += 1;
                            base_group_calls += radix;
                            nonempty_tuple_controls += usize::from(m != 0);
                        }
                        contexts += 1;
                    }
                }
            }
        }
    }
    assert_eq!(contexts, 2080);
    assert!(base_group_calls <= 46_560);
    assert!(mode_placement_checks > 0);

    // Balanced actual radix64, with zero modes; still forced component parameters.
    let p = Parameters {
        radix: 64,
        a: 1,
        u: 1,
        length: 2,
        array: 4096,
    };
    let balanced = graph(8192, 0);
    let packing = packing::pack_labels(
        &balanced,
        &p,
        (0..balanced.left.len()).map(|v| v % 64).collect(),
        (0..balanced.right.len()).map(|v| (v / 64) % 64).collect(),
        true,
        &mut meter,
    )
    .unwrap();
    let kernel = Cauchy::new(&p, 1, &ring.zero(), &ring, &mut meter).unwrap();
    let modes = deformation::modes(&balanced, &packing, 0, &p, &kernel, &mut meter).unwrap();
    assert!(modes.is_empty());
    let observed = observe(&balanced, &packing, &p, 0, &kernel, &modes, &[], &mut meter);
    assert!(observed.edges.iter().all(|&n| n == 256 && 8 * n <= 8192));
    assert_eq!(packing.packets.len(), 4096);
    assert_eq!(observed.packet_vertices, 262_144);
    assert_eq!(observed.isolated_packets, 254_016);
    assert_eq!(observed.old_vertices, 16_386);
    assert_eq!(observed.old_isolates, 2);
    assert!(observed.maximum_degree <= 4);
    emit(
        &mut writer,
        json!({"kind":"balanced","M":8192,"group_edges":observed.edges,
        "packets":4096,"old_vertices":observed.old_vertices,"old_isolates":observed.old_isolates,
        "packet_vertices":observed.packet_vertices,"total_vertices":observed.old_vertices+observed.packet_vertices,
        "isolated_packets":observed.isolated_packets,"maximum_degree":observed.maximum_degree,
        "group_calls":64,"packing_bounds_enforced":true,"checks_passed":true}),
    );

    // Intentionally bypass valid packing coverage in the two packet mutants.
    let p = Parameters {
        radix: 2,
        a: 2,
        u: 2,
        length: 4,
        array: 16,
    };
    let graph = graph(3, 4);
    let packing = packing::pack_labels(
        &graph,
        &p,
        labels(graph.left.len(), 4, 1, 0),
        labels(graph.right.len(), 4, 1, 1),
        false,
        &mut meter,
    )
    .unwrap();
    let kernel = Cauchy::new(&p, 1, &ring.one(), &ring, &mut meter).unwrap();
    let modes = deformation::modes(&graph, &packing, 0, &p, &kernel, &mut meter).unwrap();
    let beta = vec![0; modes.len()];
    let bad_beta = vec![0; modes.len() + 1];
    let beta_error = deformation::child(
        &graph, &packing, 0, &p, &kernel, &modes, &bad_beta, 0, &mut meter,
    )
    .err()
    .expect("malformed beta must be rejected");
    assert_eq!(
        beta_error,
        crate::Error::Invalid("mode exponent tuple".into())
    );
    emit(
        &mut writer,
        json!({"kind":"negative","id":"beta_length",
        "error":beta_error.to_string(),"checks_passed":true}),
    );
    let mut negative_calls = 1usize;
    for id in ["omitted_packet_occurrence", "duplicate_packet_occurrence"] {
        let mut mutant = Packing {
            packets: packing.packets.clone(),
            row_labels: packing.row_labels.clone(),
            column_labels: packing.column_labels.clone(),
        };
        if id == "omitted_packet_occurrence" {
            let slot = mutant
                .packets
                .iter_mut()
                .flat_map(|p| p.slots.iter_mut())
                .find(|s| s.is_some())
                .unwrap();
            *slot = None;
        } else {
            mutant.packets.push(mutant.packets[0].clone());
        }
        let mut refused = Vec::new();
        for group in 0..p.radix {
            negative_calls += 1;
            let result = deformation::child(
                &graph, &mutant, 0, &p, &kernel, &modes, &beta, group, &mut meter,
            );
            if let Err(error) = result {
                let expected = if id == "omitted_packet_occurrence" {
                    "unattached bipartite port"
                } else {
                    "bipartite port ordering"
                };
                assert!(
                    coverage_error(&error, expected),
                    "unexpected negative error:{error}"
                );
                refused.push(group);
            }
        }
        assert!(!refused.is_empty(), "malformed coverage accepted:{id}");
        emit(
            &mut writer,
            json!({"kind":"negative","id":id,"refused_groups":refused,"checks_passed":true}),
        );
    }
    let mut limited = Meter::new(Limits {
        vector: 0,
        ..Limits::default()
    });
    let limit_error = deformation::child(
        &graph,
        &packing,
        0,
        &p,
        &kernel,
        &modes,
        &beta,
        0,
        &mut limited,
    )
    .err()
    .expect("zero vector limit must reject a nonempty child");
    negative_calls += 1;
    assert!(matches!(&limit_error, crate::Error::Limit(_)));
    assert!(!coverage_error(&limit_error, "unattached bipartite port"));
    assert!(!coverage_error(&limit_error, "bipartite port ordering"));
    emit(
        &mut writer,
        json!({"kind":"negative","id":"limit_is_not_coverage",
        "error":limit_error.to_string(),"coverage_classification":false,"checks_passed":true}),
    );
    emit(
        &mut writer,
        json!({"kind":"summary","base_contexts":contexts,"distinct_context_tuples":tuples,
        "base_group_calls":base_group_calls,"balanced_group_calls":64,"negative_child_probes":negative_calls,
        "positive_M_model_controls":nonempty_tuple_controls,"mode_placement_checks":mode_placement_checks,
        "meter_steps_diagnostic":meter.used,
        "scope":"forced source-object accounting; no Setup admission,contraction value or timing claim",
        "checks_passed":true}),
    );
    if let Some(file) = &mut writer {
        file.flush().unwrap();
    }
}
