//! Independent finite checker for a produced factorization certificate.

use super::*;
use crate::contraction::{Edge, Tensor, TensorKind, TensorVertex};
use crate::network::FnPredicate;
use crate::Limits;
use std::collections::BTreeSet;
use std::sync::Arc;

#[derive(Clone)]
struct CertifiedMode {
    packet: usize,
    source: usize,
    target: usize,
    weight: usize,
    rows: Vec<(usize, Element)>,
    columns: Vec<(usize, BigUint)>,
}

#[derive(Clone)]
struct EntryTable {
    beta: Vec<usize>,
    group: usize,
    packet: usize,
    values: Vec<Element>,
}

#[derive(Clone)]
struct Certificate {
    radix: usize,
    precision: usize,
    edge_count: usize,
    packets: Vec<Vec<Option<usize>>>,
    left_groups: Vec<usize>,
    right_groups: Vec<usize>,
    weights: Vec<Vec<Vec<Element>>>,
    modes: Vec<CertifiedMode>,
    tuples: Vec<Vec<usize>>,
    entries: Vec<EntryTable>,
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

fn bounds(certificate: &Certificate) -> Vec<usize> {
    let mut out = vec![0usize; certificate.packets.len() * certificate.radix];
    for (packet, slots) in certificate.packets.iter().enumerate() {
        for &edge in slots.iter().flatten() {
            out[packet * certificate.radix + certificate.left_groups[edge]] += 1;
        }
    }
    out
}

fn produce(
    graph: &Bipartite,
    packing: &Packing,
    parameters: &Parameters,
    coordinate: usize,
    kernel: &deformation::Cauchy,
    modes: &[deformation::Mode],
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
                columns.push(kernel.weight(row, column, meter)?);
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
            rows: mode.rows.clone(),
            columns: mode.columns.clone(),
        })
        .collect();
    let mut certificate = Certificate {
        radix: parameters.radix,
        precision: kernel.ring.precision(),
        edge_count: graph.edges.len(),
        packets,
        left_groups,
        right_groups,
        weights,
        modes: certified_modes,
        tuples: Vec::new(),
        entries: Vec::new(),
    };
    let group_bounds = bounds(&certificate);
    let mut beta = vec![0usize; modes.len()];
    let mut usage = vec![0usize; group_bounds.len()];
    let mut weight = 0usize;
    loop {
        certificate.tuples.push(beta.clone());
        for group in 0..parameters.radix {
            let child = deformation::child(
                graph, packing, coordinate, parameters, kernel, modes, &beta, group, meter,
            )?;
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
                    values.push(vertex.tensor.entry(&bits, &kernel.ring, meter)?);
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
            return Ok(certificate);
        }
    }
}

fn independent_tuples(certificate: &Certificate) -> BTreeSet<Vec<usize>> {
    fn visit(
        index: usize,
        certificate: &Certificate,
        bounds: &[usize],
        beta: &mut [usize],
        usage: &mut [usize],
        weight: usize,
        out: &mut BTreeSet<Vec<usize>>,
    ) {
        if index == certificate.modes.len() {
            out.insert(beta.to_vec());
            return;
        }
        let mode = &certificate.modes[index];
        let group = mode.packet * certificate.radix + mode.target;
        let group_capacity = bounds[group] - usage[group];
        let weight_capacity = (certificate.precision - 1 - weight) / mode.weight;
        for exponent in 0..=group_capacity.min(weight_capacity) {
            beta[index] = exponent;
            usage[group] += exponent;
            visit(
                index + 1,
                certificate,
                bounds,
                beta,
                usage,
                weight + exponent * mode.weight,
                out,
            );
            usage[group] -= exponent;
        }
        beta[index] = 0;
    }

    let group_bounds = bounds(certificate);
    let mut beta = vec![0usize; certificate.modes.len()];
    let mut usage = vec![0usize; group_bounds.len()];
    let mut out = BTreeSet::new();
    visit(
        0,
        certificate,
        &group_bounds,
        &mut beta,
        &mut usage,
        0,
        &mut out,
    );
    out
}

fn expected_entry(
    certificate: &Certificate,
    beta: &[usize],
    group: usize,
    packet: usize,
    mask: usize,
    ring: &Ring,
    meter: &mut Meter,
) -> Result<Element> {
    let slots = &certificate.packets[packet];
    let rows: Vec<_> = slots
        .iter()
        .enumerate()
        .filter_map(|(slot, edge)| {
            edge.filter(|&edge| certificate.left_groups[edge] == group)
                .map(|_| slot)
        })
        .collect();
    let columns: Vec<_> = slots
        .iter()
        .enumerate()
        .filter_map(|(slot, edge)| {
            edge.filter(|&edge| certificate.right_groups[edge] == group)
                .map(|_| slot)
        })
        .collect();
    let bits: Vec<_> = (0..rows.len() + columns.len())
        .map(|bit| mask & (1usize << bit) != 0)
        .collect();
    let active_incoming: Vec<_> = certificate
        .modes
        .iter()
        .enumerate()
        .filter(|(mode, description)| {
            beta[*mode] != 0 && description.packet == packet && description.target == group
        })
        .collect();
    let mut constants = Vec::new();
    let mut incoming = vec![Vec::new(); active_incoming.len()];
    for (row_index, &row) in rows.iter().enumerate() {
        let mut local = ring.zero();
        for (column_index, &column) in columns.iter().enumerate() {
            if bits[rows.len() + column_index] {
                local = ring.add(&local, &certificate.weights[packet][row][column], meter)?;
            }
        }
        constants.push(if bits[row_index] {
            local
        } else {
            ring.sub(&ring.one(), &local, meter)?
        });
        for (active, (_, mode)) in active_incoming.iter().enumerate() {
            let coefficient = mode
                .rows
                .iter()
                .find(|&&(slot, _)| slot == row)
                .map(|(_, value)| value.clone())
                .unwrap_or_else(|| ring.zero());
            incoming[active].push(if bits[row_index] {
                coefficient
            } else {
                ring.sub(&ring.zero(), &coefficient, meter)?
            });
        }
    }
    let choice_radix = active_incoming.len() + 1;
    let assignments = choice_radix.pow(u32::try_from(rows.len()).unwrap());
    let mut coefficient = ring.zero();
    for mut choices in 0..assignments {
        let mut counts = vec![0usize; active_incoming.len()];
        let mut product = ring.one();
        for (row, constant) in constants.iter().enumerate() {
            let choice = choices % choice_radix;
            choices /= choice_radix;
            let factor = if choice == 0 {
                constant
            } else {
                counts[choice - 1] += 1;
                &incoming[choice - 1][row]
            };
            product = ring.mul(&product, factor, meter)?;
        }
        if active_incoming
            .iter()
            .zip(&counts)
            .all(|((mode, _), &count)| beta[*mode] == count)
        {
            coefficient = ring.add(&coefficient, &product, meter)?;
        }
    }
    for (mode, description) in certificate.modes.iter().enumerate() {
        if beta[mode] == 0 || description.packet != packet || description.source != group {
            continue;
        }
        let mut form = BigUint::zero();
        for (column_index, &column) in columns.iter().enumerate() {
            if !bits[rows.len() + column_index] {
                continue;
            }
            if let Some((_, value)) = description
                .columns
                .iter()
                .find(|&&(slot, _)| slot == column)
            {
                form = (form + value) % ring.modulus();
            }
        }
        let power = ring.pow(&ring.integer(&form), beta[mode], meter)?;
        coefficient = ring.mul(&coefficient, &power, meter)?;
    }
    Ok(coefficient)
}

fn check(
    certificate: &Certificate,
    ring: &Ring,
    meter: &mut Meter,
) -> std::result::Result<(), String> {
    if certificate.left_groups.len() != certificate.edge_count
        || certificate.right_groups.len() != certificate.edge_count
        || certificate
            .left_groups
            .iter()
            .chain(&certificate.right_groups)
            .any(|&group| group >= certificate.radix)
    {
        return Err("incidence group partition".into());
    }
    if certificate.weights.len() != certificate.packets.len() {
        return Err("packet weight count".into());
    }
    let mut occurrences = vec![0usize; certificate.edge_count];
    for slots in &certificate.packets {
        for &edge in slots.iter().flatten() {
            let Some(count) = occurrences.get_mut(edge) else {
                return Err("packet edge range".into());
            };
            *count += 1;
        }
    }
    if occurrences.iter().any(|&count| count != 1) {
        return Err("packet coverage".into());
    }
    for mode in &certificate.modes {
        if mode.packet >= certificate.packets.len()
            || mode.source >= certificate.radix
            || mode.target >= certificate.radix
            || mode.source == mode.target
            || !(1..certificate.precision).contains(&mode.weight)
            || mode.rows.is_empty()
            || mode.columns.is_empty()
        {
            return Err("mode header".into());
        }
        let slots = &certificate.packets[mode.packet];
        for &(slot, ref value) in &mode.rows {
            let Some(Some(edge)) = slots.get(slot) else {
                return Err("mode row slot".into());
            };
            if value.is_zero() || certificate.left_groups[*edge] != mode.target {
                return Err("mode row placement".into());
            }
        }
        for &(slot, ref value) in &mode.columns {
            let Some(Some(edge)) = slots.get(slot) else {
                return Err("mode column slot".into());
            };
            if value.is_zero() || certificate.right_groups[*edge] != mode.source {
                return Err("mode column placement".into());
            }
        }
    }
    for (packet, slots) in certificate.packets.iter().enumerate() {
        if certificate.weights[packet].len() != slots.len()
            || certificate.weights[packet]
                .iter()
                .any(|row| row.len() != slots.len())
        {
            return Err("packet weight shape".into());
        }
        for (row, &row_edge) in slots
            .iter()
            .enumerate()
            .filter_map(|(slot, edge)| edge.as_ref().map(|edge| (slot, edge)))
        {
            for (column, &column_edge) in slots
                .iter()
                .enumerate()
                .filter_map(|(slot, edge)| edge.as_ref().map(|edge| (slot, edge)))
            {
                let target = certificate.left_groups[row_edge];
                let source = certificate.right_groups[column_edge];
                if target == source {
                    continue;
                }
                let mut expanded = ring.zero();
                for mode in certificate.modes.iter().filter(|mode| {
                    mode.packet == packet && mode.target == target && mode.source == source
                }) {
                    let Some((_, row_value)) = mode.rows.iter().find(|&&(slot, _)| slot == row)
                    else {
                        continue;
                    };
                    let Some((_, column_value)) =
                        mode.columns.iter().find(|&&(slot, _)| slot == column)
                    else {
                        continue;
                    };
                    let term = ring
                        .mul(row_value, &ring.integer(column_value), meter)
                        .map_err(|error| error.to_string())?;
                    expanded = ring
                        .add(&expanded, &term, meter)
                        .map_err(|error| error.to_string())?;
                }
                if expanded != certificate.weights[packet][row][column] {
                    return Err("crossing mode series".into());
                }
            }
        }
    }
    let actual_tuples: BTreeSet<_> = certificate.tuples.iter().cloned().collect();
    if actual_tuples.len() != certificate.tuples.len()
        || actual_tuples != independent_tuples(certificate)
    {
        return Err("weighted tuple coverage".into());
    }
    let expected_tables = certificate.tuples.len() * certificate.radix * certificate.packets.len();
    if certificate.entries.len() != expected_tables {
        return Err("entry table coverage".into());
    }
    let tuple_set: BTreeSet<_> = certificate.tuples.iter().cloned().collect();
    let mut table_keys = BTreeSet::new();
    for table in &certificate.entries {
        if !tuple_set.contains(&table.beta)
            || table.group >= certificate.radix
            || table.packet >= certificate.packets.len()
            || !table_keys.insert((table.beta.clone(), table.group, table.packet))
        {
            return Err("entry table key".into());
        }
        let slots = &certificate.packets[table.packet];
        let arity = slots
            .iter()
            .flatten()
            .filter(|&&edge| certificate.left_groups[edge] == table.group)
            .count()
            + slots
                .iter()
                .flatten()
                .filter(|&&edge| certificate.right_groups[edge] == table.group)
                .count();
        if table.values.len() != 1usize << arity {
            return Err("entry table shape".into());
        }
        for (mask, value) in table.values.iter().enumerate() {
            let expected = expected_entry(
                certificate,
                &table.beta,
                table.group,
                table.packet,
                mask,
                ring,
                meter,
            )
            .map_err(|error| error.to_string())?;
            if &expected != value {
                return Err(format!(
                    "packet coefficient table: beta={:?}, group={}, packet={}, mask={}, slots={:?}, left={:?}, right={:?}, expected={expected:?}, actual={value:?}",
                    table.beta,
                    table.group,
                    table.packet,
                    mask,
                    certificate.packets[table.packet],
                    certificate.left_groups,
                    certificate.right_groups,
                ));
            }
        }
    }
    Ok(())
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
    let certificate = produce(
        &graph,
        &packing,
        &parameters,
        coordinate,
        &kernel,
        &modes,
        &mut meter,
    )
    .unwrap();
    check(&certificate, &ring, &mut meter).unwrap();

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

    let mut bad_tuples = certificate.clone();
    bad_tuples.tuples.pop();
    assert_eq!(
        check(&bad_tuples, &ring, &mut meter).unwrap_err(),
        "weighted tuple coverage"
    );

    let mut bad_entry = certificate.clone();
    bad_entry.entries[0].values[0] = ring
        .add(&bad_entry.entries[0].values[0], &ring.one(), &mut meter)
        .unwrap();
    assert!(check(&bad_entry, &ring, &mut meter)
        .unwrap_err()
        .starts_with("packet coefficient table:"));
}
