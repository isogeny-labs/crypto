//! Independent replay of finite packet-packing provenance.
//!
//! This module consumes a labeled bipartite edge skeleton and an already
//! accepted factorization certificate. It does not import the source packing,
//! deformation, contraction, or graph implementations.

use crate::factorization_certificate::Certificate as FactorizationCertificate;
use crate::{BigUint, Meter};
use num_traits::Zero;
use serde::{Deserialize, Serialize};

/// Current packing-provenance wire schema.
pub const SCHEMA: u32 = 2;

/// Endpoint vertex indices for one edge in source order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EdgeEndpoints {
    /// Index of the left endpoint vertex.
    pub left: usize,
    /// Port index at the left endpoint vertex.
    pub left_port: usize,
    /// Index of the right endpoint vertex.
    pub right: usize,
    /// Port index at the right endpoint vertex.
    pub right_port: usize,
}

/// Finite data needed to replay packet construction independently.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Certificate {
    /// Wire schema version.
    pub schema: u32,
    /// Packet label radix.
    pub radix: usize,
    /// Packet-length exponent `a`, so `L=2^a`.
    pub packet_exponent: u32,
    /// Active valuation coordinate used for factorization groups.
    pub coordinate: u32,
    /// Whether labels are claimed to be the exact conditional-expectation output.
    pub conditional_labels: bool,
    /// Source degree cap when variance, capacity, and group balance are claimed.
    pub source_degree_cap: Option<usize>,
    /// Edge endpoints in the factorization certificate's edge order.
    pub edges: Vec<EdgeEndpoints>,
    /// Source labels for every left vertex, including isolates.
    pub row_labels: Vec<usize>,
    /// Source labels for every right vertex, including isolates.
    pub column_labels: Vec<usize>,
    /// Unordered radix-digit pair pattern attached to every emitted packet.
    pub packet_pairs: Vec<Vec<(usize, usize)>>,
}

/// Checked dimensions of one accepted packing-provenance replay.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VerificationSummary {
    /// Number of replayed edges.
    pub edges: usize,
    /// Number of left vertices, including isolates.
    pub left_vertices: usize,
    /// Number of right vertices, including isolates.
    pub right_vertices: usize,
    /// Number of reconstructed nonempty packets.
    pub packets: usize,
    /// Slots in every packet.
    pub packet_length: usize,
    /// Number of ordered row/column label classes.
    pub label_classes: usize,
    /// Number of unordered digit patterns replayed before empty suppression.
    pub orientation_patterns: usize,
    /// Nonnegative slack in `K L<=3M`, when that inequality holds.
    pub packing_slack: Option<BigUint>,
    /// Whether the replayed packet array satisfies source capacity.
    pub source_capacity_admitted: bool,
    /// Whether conditional-expectation label selection was independently replayed.
    pub conditional_labels_verified: bool,
    /// Whether variance, capacity, and all-coordinate group balance were checked.
    pub source_bounds_verified: bool,
}

fn digit(label: usize, coordinate: u32, radix: usize) -> Option<usize> {
    radix
        .checked_pow(coordinate)
        .map(|place| (label / place) % radix)
}

fn conditional_objective(
    edges: &[EdgeEndpoints],
    rows: &[Option<usize>],
    columns: &[Option<usize>],
    labels: usize,
    meter: &mut Meter,
) -> Result<BigUint, String> {
    let label = BigUint::from(labels);
    let square = &label * &label;
    let mut total = BigUint::zero();
    for first in edges {
        for second in edges {
            meter
                .charge(1, "packing provenance conditional edge pair")
                .map_err(|error| error.to_string())?;
            let mut unknown = 0usize;
            let mut impossible = false;
            for (left, right, assignment) in [
                (first.left, second.left, rows),
                (first.right, second.right, columns),
            ] {
                if left == right {
                    continue;
                }
                match (assignment[left], assignment[right]) {
                    (Some(x), Some(y)) if x != y => {
                        impossible = true;
                        break;
                    }
                    (Some(_), Some(_)) => {}
                    _ => unknown += 1,
                }
            }
            if !impossible {
                total += match unknown {
                    0 => square.clone(),
                    1 => label.clone(),
                    _ => BigUint::from(1u8),
                };
            }
        }
    }
    Ok(total)
}

fn verify_conditional_labels(
    provenance: &Certificate,
    labels: usize,
    meter: &mut Meter,
) -> Result<(), String> {
    let mut rows = vec![None; provenance.row_labels.len()];
    let mut columns = vec![None; provenance.column_labels.len()];
    for right_side in [false, true] {
        let count = if right_side {
            columns.len()
        } else {
            rows.len()
        };
        for vertex in 0..count {
            let isolated = provenance.edges.iter().all(|edge| {
                if right_side {
                    edge.right != vertex
                } else {
                    edge.left != vertex
                }
            });
            if isolated {
                if right_side {
                    columns[vertex] = Some(0);
                } else {
                    rows[vertex] = Some(0);
                }
                continue;
            }
            let mut best: Option<(BigUint, usize)> = None;
            for label in 0..labels {
                meter
                    .charge(1, "packing provenance conditional candidate")
                    .map_err(|error| error.to_string())?;
                if right_side {
                    columns[vertex] = Some(label);
                } else {
                    rows[vertex] = Some(label);
                }
                let score =
                    conditional_objective(&provenance.edges, &rows, &columns, labels, meter)?;
                if best.as_ref().map_or(true, |(old, _)| score < *old) {
                    best = Some((score, label));
                }
            }
            let selected = best
                .ok_or_else(|| "packing provenance conditional alphabet".to_string())?
                .1;
            if right_side {
                columns[vertex] = Some(selected);
            } else {
                rows[vertex] = Some(selected);
            }
        }
    }
    let rows: Vec<_> = rows.into_iter().map(Option::unwrap).collect();
    let columns: Vec<_> = columns.into_iter().map(Option::unwrap).collect();
    if rows != provenance.row_labels || columns != provenance.column_labels {
        return Err("packing provenance conditional labels".into());
    }
    Ok(())
}

/// Reconstruct packet ordering, slots, orientations, and active groups.
///
/// The factorization certificate must be verified separately. Acceptance here
/// proves that its packet and group arrays are exactly reproduced by the
/// serialized labeled edge skeleton under the source packet-construction
/// schedule. It does not prove how the vertex labels were selected or that the
/// external contraction theorem applies.
pub fn verify(
    factorization: &FactorizationCertificate,
    provenance: &Certificate,
    meter: &mut Meter,
) -> Result<VerificationSummary, String> {
    if provenance.schema != SCHEMA
        || provenance.radix == 0
        || provenance.coordinate >= provenance.packet_exponent
        || factorization.radix != provenance.radix
        || provenance.edges.len() != factorization.edge_count
        || (provenance.source_degree_cap.is_some() && !provenance.conditional_labels)
    {
        return Err("packing provenance context".into());
    }
    let packet_length = 1usize
        .checked_shl(provenance.packet_exponent)
        .ok_or_else(|| "packing provenance packet width".to_string())?;
    let labels = provenance
        .radix
        .checked_pow(provenance.packet_exponent)
        .ok_or_else(|| "packing provenance label width".to_string())?;
    let label_classes = labels
        .checked_mul(labels)
        .ok_or_else(|| "packing provenance class width".to_string())?;
    if factorization
        .packets
        .iter()
        .any(|packet| packet.len() != packet_length)
        || provenance
            .row_labels
            .iter()
            .chain(&provenance.column_labels)
            .any(|&label| label >= labels)
        || provenance.edges.iter().any(|edge| {
            edge.left >= provenance.row_labels.len() || edge.right >= provenance.column_labels.len()
        })
    {
        return Err("packing provenance shape".into());
    }
    if provenance.conditional_labels {
        verify_conditional_labels(provenance, labels, meter)?;
    }
    meter
        .vector(label_classes, "packing provenance classes")
        .map_err(|error| error.to_string())?;
    let mut classes = vec![Vec::new(); label_classes];
    for (edge_index, edge) in provenance.edges.iter().enumerate() {
        let class = provenance.row_labels[edge.left]
            .checked_mul(labels)
            .and_then(|value| value.checked_add(provenance.column_labels[edge.right]))
            .ok_or_else(|| "packing provenance class index".to_string())?;
        classes[class].push(edge_index);
        let left_group = digit(
            provenance.row_labels[edge.left],
            provenance.coordinate,
            provenance.radix,
        )
        .ok_or_else(|| "packing provenance group width".to_string())?;
        let right_group = digit(
            provenance.column_labels[edge.right],
            provenance.coordinate,
            provenance.radix,
        )
        .ok_or_else(|| "packing provenance group width".to_string())?;
        if factorization.left_groups.get(edge_index) != Some(&left_group)
            || factorization.right_groups.get(edge_index) != Some(&right_group)
        {
            return Err("packing provenance groups".into());
        }
    }
    if let Some(delta) = provenance.source_degree_cap {
        let sum_squares: BigUint = classes
            .iter()
            .map(|class| BigUint::from(class.len()) * BigUint::from(class.len()))
            .sum();
        let edges = BigUint::from(provenance.edges.len());
        let array = BigUint::from(label_classes);
        let lhs = &array * sum_squares;
        let base = &edges * &edges;
        let rhs = BigUint::from(2u8) * &edges * BigUint::from(delta) * &array;
        if lhs < base || lhs - base > rhs {
            return Err("packing provenance variance bound".into());
        }
    }

    let maximum = classes.iter().map(Vec::len).max().unwrap_or(0);
    let height = maximum
        .div_ceil(packet_length)
        .checked_mul(packet_length)
        .ok_or_else(|| "packing provenance height".to_string())?;
    let pairs: Vec<_> = (0..provenance.radix)
        .flat_map(|low| (low..provenance.radix).map(move |high| (low, high)))
        .collect();
    let orientation_patterns = pairs
        .len()
        .checked_pow(provenance.packet_exponent)
        .ok_or_else(|| "packing provenance pattern width".to_string())?;
    meter
        .vector(orientation_patterns, "packing provenance patterns")
        .map_err(|error| error.to_string())?;

    let mut expected_packets = Vec::new();
    let mut expected_pairs = Vec::new();
    for mut pattern in 0..orientation_patterns {
        meter
            .charge(1, "packing provenance pattern")
            .map_err(|error| error.to_string())?;
        let mut chosen = Vec::with_capacity(provenance.packet_exponent as usize);
        for _ in 0..provenance.packet_exponent {
            chosen.push(pairs[pattern % pairs.len()]);
            pattern /= pairs.len();
        }
        let unequal = chosen.iter().filter(|&&(low, high)| low != high).count();
        let copies = 1usize
            .checked_shl(provenance.packet_exponent - unequal as u32)
            .ok_or_else(|| "packing provenance copy width".to_string())?;
        for start in (0..height).step_by(copies) {
            meter
                .vector(packet_length, "packing provenance packet slots")
                .map_err(|error| error.to_string())?;
            let mut slots = Vec::with_capacity(packet_length);
            for slot in 0..packet_length {
                meter
                    .charge(1, "packing provenance slot")
                    .map_err(|error| error.to_string())?;
                let (mut row, mut column, mut place, mut copy, mut copy_bit) =
                    (0usize, 0usize, 1usize, 0usize, 0usize);
                for (coordinate, &(low, high)) in chosen.iter().enumerate() {
                    let bit = (slot >> coordinate) & 1;
                    let (row_digit, column_digit) = if low == high {
                        copy |= bit << copy_bit;
                        copy_bit += 1;
                        (low, high)
                    } else if bit == 0 {
                        (low, high)
                    } else {
                        (high, low)
                    };
                    row = row
                        .checked_add(
                            row_digit
                                .checked_mul(place)
                                .ok_or_else(|| "packing provenance row label".to_string())?,
                        )
                        .ok_or_else(|| "packing provenance row label".to_string())?;
                    column = column
                        .checked_add(
                            column_digit
                                .checked_mul(place)
                                .ok_or_else(|| "packing provenance column label".to_string())?,
                        )
                        .ok_or_else(|| "packing provenance column label".to_string())?;
                    place = place
                        .checked_mul(provenance.radix)
                        .ok_or_else(|| "packing provenance label place".to_string())?;
                }
                let class = row
                    .checked_mul(labels)
                    .and_then(|value| value.checked_add(column))
                    .ok_or_else(|| "packing provenance slot class".to_string())?;
                slots.push(classes[class].get(start + copy).copied());
            }
            if slots.iter().any(Option::is_some) {
                meter
                    .vector(expected_packets.len() + 1, "packing provenance packets")
                    .map_err(|error| error.to_string())?;
                expected_packets.push(slots);
                expected_pairs.push(chosen.clone());
            }
        }
    }
    if expected_packets != factorization.packets || expected_pairs != provenance.packet_pairs {
        return Err("packing provenance replay".into());
    }
    let packing_lhs = BigUint::from(expected_packets.len()) * BigUint::from(packet_length);
    let packing_rhs = BigUint::from(3u8) * BigUint::from(provenance.edges.len());
    let source_capacity_admitted = packing_lhs <= packing_rhs;
    if provenance.source_degree_cap.is_some() && !source_capacity_admitted {
        return Err("packing provenance source capacity".into());
    }
    if provenance.source_degree_cap.is_some() {
        for coordinate in 0..provenance.packet_exponent {
            let mut row_load = vec![0usize; provenance.radix];
            let mut column_load = vec![0usize; provenance.radix];
            for edge in &provenance.edges {
                let row = digit(
                    provenance.row_labels[edge.left],
                    coordinate,
                    provenance.radix,
                )
                .ok_or_else(|| "packing provenance balance width".to_string())?;
                let column = digit(
                    provenance.column_labels[edge.right],
                    coordinate,
                    provenance.radix,
                )
                .ok_or_else(|| "packing provenance balance width".to_string())?;
                row_load[row] = row_load[row]
                    .checked_add(1)
                    .ok_or_else(|| "packing provenance row load".to_string())?;
                column_load[column] = column_load[column]
                    .checked_add(1)
                    .ok_or_else(|| "packing provenance column load".to_string())?;
            }
            let maximum = BigUint::from(3u8) * BigUint::from(provenance.edges.len());
            if row_load
                .iter()
                .chain(&column_load)
                .any(|&load| BigUint::from(load) * BigUint::from(provenance.radix) > maximum)
            {
                return Err("packing provenance group balance".into());
            }
        }
    }
    Ok(VerificationSummary {
        edges: provenance.edges.len(),
        left_vertices: provenance.row_labels.len(),
        right_vertices: provenance.column_labels.len(),
        packets: expected_packets.len(),
        packet_length,
        label_classes,
        orientation_patterns,
        packing_slack: source_capacity_admitted.then(|| packing_rhs - packing_lhs),
        source_capacity_admitted,
        conditional_labels_verified: provenance.conditional_labels,
        source_bounds_verified: provenance.source_degree_cap.is_some(),
    })
}
