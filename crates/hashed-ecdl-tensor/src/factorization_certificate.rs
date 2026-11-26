//! Standalone verification of finite recursive-factorization certificates.
//!
//! This module consumes plain serialized data and the crate's exact ring
//! arithmetic. It deliberately does not import recursive contraction,
//! deformation, packing, child construction, or packet tensors.

use crate::ring::{Element, Ring};
use crate::{integrity, overflow, BigUint, Meter, Result};
use num_traits::Zero;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) const SCHEMA: u32 = 1;
const MAX_MODE_DEPTH: usize = 1_024;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct CertifiedElement(pub(crate) Vec<BigUint>);

impl CertifiedElement {
    #[cfg(test)]
    pub(crate) fn encode(value: Element) -> Self {
        Self(value.coordinates().to_vec())
    }

    fn validate(&self, ring: &Ring) -> std::result::Result<(), String> {
        if self.0.len() != ring.degree() || self.0.iter().any(|value| value >= ring.modulus()) {
            return Err("noncanonical certificate element".into());
        }
        Ok(())
    }

    pub(crate) fn decode(&self, ring: &Ring) -> std::result::Result<Element, String> {
        self.validate(ring)?;
        Ok(Element(self.0.clone()))
    }

    fn is_zero(&self) -> bool {
        self.0.iter().all(BigUint::is_zero)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct CertifiedMode {
    pub(crate) packet: usize,
    pub(crate) source: usize,
    pub(crate) target: usize,
    pub(crate) weight: usize,
    pub(crate) rows: Vec<(usize, CertifiedElement)>,
    pub(crate) columns: Vec<(usize, BigUint)>,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct EntryTable {
    pub(crate) beta: Vec<usize>,
    pub(crate) group: usize,
    pub(crate) packet: usize,
    pub(crate) values: Vec<CertifiedElement>,
}

/// Schema-versioned finite data checked by [`verify`].
#[derive(Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub(crate) schema: u32,
    pub(crate) degree: usize,
    pub(crate) radix: usize,
    pub(crate) precision: usize,
    pub(crate) edge_count: usize,
    pub(crate) packets: Vec<Vec<Option<usize>>>,
    pub(crate) left_groups: Vec<usize>,
    pub(crate) right_groups: Vec<usize>,
    pub(crate) weights: Vec<Vec<Vec<CertifiedElement>>>,
    pub(crate) modes: Vec<CertifiedMode>,
    pub(crate) tuples: Vec<Vec<usize>>,
    pub(crate) entries: Vec<EntryTable>,
}

fn preflight(certificate: &Certificate, meter: &mut Meter) -> std::result::Result<(), String> {
    if certificate.schema != SCHEMA
        || certificate.degree == 0
        || certificate.precision == 0
        || certificate.radix == 0
    {
        return Err("certificate context".into());
    }
    if certificate.modes.len() > MAX_MODE_DEPTH {
        return Err("certificate mode-depth limit".into());
    }
    for (length, phase) in [
        (certificate.edge_count, "certificate edges"),
        (certificate.packets.len(), "certificate packets"),
        (certificate.modes.len(), "certificate modes"),
        (certificate.tuples.len(), "certificate tuples"),
        (certificate.entries.len(), "certificate entry tables"),
    ] {
        meter
            .vector(length, phase)
            .map_err(|error| error.to_string())?;
    }
    for slots in &certificate.packets {
        meter
            .vector(slots.len(), "certificate packet slots")
            .map_err(|error| error.to_string())?;
    }
    for tuple in &certificate.tuples {
        meter
            .vector(tuple.len(), "certificate tuple width")
            .map_err(|error| error.to_string())?;
    }
    for table in &certificate.entries {
        meter
            .vector(table.values.len(), "certificate entry values")
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn independent_tuples(
    certificate: &Certificate,
    meter: &mut Meter,
) -> std::result::Result<BTreeSet<Vec<usize>>, String> {
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
        meter: &mut Meter,
    ) -> std::result::Result<(), String> {
        meter
            .charge(1, "certificate tuple enumeration")
            .map_err(|error| error.to_string())?;
        if index == certificate.modes.len() {
            if out.len() >= meter.limits.vector {
                return Err("resource limit: certificate tuple set".into());
            }
            out.insert(beta.to_vec());
            return Ok(());
        }
        let mode = &certificate.modes[index];
        let group = (mode.packet, mode.target);
        let group_capacity =
            bounds.get(&group).copied().unwrap_or(0) - usage.get(&group).copied().unwrap_or(0);
        let weight_capacity = (certificate.precision - 1 - weight) / mode.weight;
        for exponent in 0..=group_capacity.min(weight_capacity) {
            beta[index] = exponent;
            *usage.entry(group).or_insert(0) += exponent;
            let next_weight = exponent
                .checked_mul(mode.weight)
                .and_then(|extra| weight.checked_add(extra))
                .ok_or_else(|| "weighted tuple arithmetic overflow".to_string())?;
            visit(
                index + 1,
                certificate,
                bounds,
                beta,
                usage,
                next_weight,
                out,
                meter,
            )?;
            *usage.get_mut(&group).unwrap() -= exponent;
        }
        beta[index] = 0;
        Ok(())
    }

    let mut beta = vec![0usize; certificate.modes.len()];
    let mut usage = BTreeMap::new();
    let mut out = BTreeSet::new();
    visit(
        0,
        certificate,
        &bounds,
        &mut beta,
        &mut usage,
        0,
        &mut out,
        meter,
    )?;
    Ok(out)
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

pub(crate) fn verify_with_ring(
    certificate: &Certificate,
    ring: &Ring,
    meter: &mut Meter,
) -> std::result::Result<(), String> {
    preflight(certificate, meter)?;
    if certificate.degree != ring.degree() || certificate.precision != ring.precision() {
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
        value.validate(ring)?;
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
                if expanded.coordinates() != certificate.weights[packet][row][column].0.as_slice() {
                    return Err("crossing mode series".into());
                }
            }
        }
    }
    if certificate
        .tuples
        .iter()
        .any(|tuple| tuple.len() != certificate.modes.len())
    {
        return Err("weighted tuple shape".into());
    }
    let actual_tuples: BTreeSet<_> = certificate.tuples.iter().cloned().collect();
    if actual_tuples.len() != certificate.tuples.len()
        || actual_tuples != independent_tuples(certificate, meter)?
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
            if expected.coordinates() != value.0.as_slice() {
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

/// Verify a parsed certificate under explicit caller-supplied resource limits.
pub fn verify(certificate: &Certificate, meter: &mut Meter) -> std::result::Result<(), String> {
    preflight(certificate, meter)?;
    let ring = Ring::new(certificate.degree, certificate.precision, meter)
        .map_err(|error| error.to_string())?;
    verify_with_ring(certificate, &ring, meter)
}
