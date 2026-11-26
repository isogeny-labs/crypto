//! Independent finite checker for a produced factorization certificate.

use super::*;
use crate::contraction::{Edge, Tensor, TensorKind, TensorVertex};
use crate::network::FnPredicate;
use crate::Limits;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

const SCHEMA: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct CertifiedElement(Vec<BigUint>);

impl CertifiedElement {
    fn encode(value: Element) -> Self {
        Self(value.coordinates().to_vec())
    }

    fn decode(&self, ring: &Ring) -> std::result::Result<Element, String> {
        if self.0.len() != ring.degree() || self.0.iter().any(|value| value >= ring.modulus()) {
            return Err("noncanonical certificate element".into());
        }
        Ok(Element(self.0.clone()))
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(BigUint::is_zero)
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct CertifiedMode {
    packet: usize,
    source: usize,
    target: usize,
    weight: usize,
    rows: Vec<(usize, CertifiedElement)>,
    columns: Vec<(usize, BigUint)>,
}

#[derive(Clone, Serialize, Deserialize)]
struct EntryTable {
    beta: Vec<usize>,
    group: usize,
    packet: usize,
    values: Vec<CertifiedElement>,
}

#[derive(Clone, Serialize, Deserialize)]
struct Certificate {
    schema: u32,
    degree: usize,
    radix: usize,
    precision: usize,
    edge_count: usize,
    packets: Vec<Vec<Option<usize>>>,
    left_groups: Vec<usize>,
    right_groups: Vec<usize>,
    weights: Vec<Vec<Vec<CertifiedElement>>>,
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

fn producer_bounds(certificate: &Certificate) -> Vec<usize> {
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
            return Ok(certificate);
        }
    }
}

fn independent_tuples(certificate: &Certificate) -> BTreeSet<Vec<usize>> {
    let mut bounds = BTreeMap::new();
    for (packet, slots) in certificate.packets.iter().enumerate() {
        for &edge in slots.iter().flatten() {
            *bounds
                .entry((packet, certificate.left_groups[edge]))
                .or_insert(0usize) += 1;
        }
    }

    fn visit(
        index: usize,
        certificate: &Certificate,
        bounds: &BTreeMap<(usize, usize), usize>,
        beta: &mut [usize],
        usage: &mut BTreeMap<(usize, usize), usize>,
        weight: usize,
        out: &mut BTreeSet<Vec<usize>>,
    ) {
        if index == certificate.modes.len() {
            out.insert(beta.to_vec());
            return;
        }
        let mode = &certificate.modes[index];
        let group = (mode.packet, mode.target);
        let group_capacity =
            bounds.get(&group).copied().unwrap_or(0) - usage.get(&group).copied().unwrap_or(0);
        let weight_capacity = (certificate.precision - 1 - weight) / mode.weight;
        for exponent in 0..=group_capacity.min(weight_capacity) {
            beta[index] = exponent;
            *usage.entry(group).or_insert(0) += exponent;
            visit(
                index + 1,
                certificate,
                bounds,
                beta,
                usage,
                weight + exponent * mode.weight,
                out,
            );
            *usage.get_mut(&group).unwrap() -= exponent;
        }
        beta[index] = 0;
    }

    let mut beta = vec![0usize; certificate.modes.len()];
    let mut usage = BTreeMap::new();
    let mut out = BTreeSet::new();
    visit(0, certificate, &bounds, &mut beta, &mut usage, 0, &mut out);
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
    let arity = rows
        .len()
        .checked_add(columns.len())
        .ok_or_else(|| overflow("certificate packet arity"))?;
    let bits: Vec<_> = (0..arity).map(|bit| mask & (1usize << bit) != 0).collect();
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
                local = ring.add(
                    &local,
                    &certificate.weights[packet][row][column]
                        .decode(ring)
                        .map_err(|error| integrity(&error))?,
                    meter,
                )?;
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
                .map(|(_, value)| value.decode(ring).map_err(|error| integrity(&error)))
                .transpose()?
                .unwrap_or_else(|| ring.zero());
            incoming[active].push(if bits[row_index] {
                coefficient
            } else {
                ring.sub(&ring.zero(), &coefficient, meter)?
            });
        }
    }
    let choice_radix = active_incoming
        .len()
        .checked_add(1)
        .ok_or_else(|| overflow("certificate row-choice radix"))?;
    let assignments = choice_radix
        .checked_pow(
            u32::try_from(rows.len()).map_err(|_| overflow("certificate row-choice arity"))?,
        )
        .ok_or_else(|| overflow("certificate row-choice assignments"))?;
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
    if certificate.schema != SCHEMA
        || certificate.degree != ring.degree()
        || certificate.precision != ring.precision()
        || certificate.radix == 0
    {
        return Err("certificate context".into());
    }
    for value in certificate
        .weights
        .iter()
        .flatten()
        .flatten()
        .chain(
            certificate
                .modes
                .iter()
                .flat_map(|mode| mode.rows.iter().map(|(_, value)| value)),
        )
        .chain(certificate.entries.iter().flat_map(|table| &table.values))
    {
        value.decode(ring)?;
    }
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
            if value.is_zero()
                || value >= ring.modulus()
                || certificate.right_groups[*edge] != mode.source
            {
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
                        .mul(&row_value.decode(ring)?, &ring.integer(column_value), meter)
                        .map_err(|error| error.to_string())?;
                    expanded = ring
                        .add(&expanded, &term, meter)
                        .map_err(|error| error.to_string())?;
                }
                if expanded != certificate.weights[packet][row][column].decode(ring)? {
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
    let expected_tables = certificate
        .tuples
        .len()
        .checked_mul(certificate.radix)
        .and_then(|count| count.checked_mul(certificate.packets.len()))
        .ok_or_else(|| "entry table count overflow".to_string())?;
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
            .checked_add(
                slots
                    .iter()
                    .flatten()
                    .filter(|&&edge| certificate.right_groups[edge] == table.group)
                    .count(),
            )
            .ok_or_else(|| "entry table arity overflow".to_string())?;
        let assignments = 1usize
            .checked_shl(
                u32::try_from(arity).map_err(|_| "entry table arity overflow".to_string())?,
            )
            .ok_or_else(|| "entry table arity overflow".to_string())?;
        if table.values.len() != assignments {
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
            if expected != value.decode(ring)? {
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
    let produced = produce(
        &graph,
        &packing,
        &parameters,
        coordinate,
        &kernel,
        &modes,
        &mut meter,
    )
    .unwrap();
    let serialized = serde_json::to_vec(&produced).unwrap();
    assert_eq!(serialized.len(), 2_162);
    let certificate: Certificate = serde_json::from_slice(&serialized).unwrap();
    assert_eq!(serde_json::to_vec(&certificate).unwrap(), serialized);
    check(&certificate, &ring, &mut meter).unwrap();

    let mut bad_context = certificate.clone();
    bad_context.schema += 1;
    assert_eq!(
        check(&bad_context, &ring, &mut meter).unwrap_err(),
        "certificate context"
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
